//! MCP prompts, resources, and argument completions.
//!
//! These back the `prompts`, `resources`, and `completions` capabilities the
//! server advertises in `get_info`. They are pure data + lookup helpers; the
//! `ServerHandler` methods in [`super::server`] delegate here.

use rmcp::ErrorData as McpError;
use rmcp::model::{
    ArgumentInfo, CacheScope, CompleteResult, CompletionInfo, GetPromptResult, JsonObject, ListPromptsResult,
    ListResourcesResult, Prompt, PromptArgument, PromptMessage, ReadResourceResult, Reference, Resource,
    ResourceContents, Role,
};
use rmcp::schemars;

use crate::options::validation::normalize_token;

/// Cache TTL (SEP-2549) for the prompt/resource catalogs. They are derived
/// entirely from compile-time constants and the options schema, so they are
/// safe to cache publicly for a long window (one hour).
const CATALOG_TTL_MS: u64 = 60 * 60 * 1000;

/// Prompt that drives `convert_html`.
const PROMPT_CONVERT: &str = "convert_to_markdown";
/// Prompt that extracts main article content.
const PROMPT_EXTRACT: &str = "extract_main_content";
/// Prompt that drives `extract_metadata`.
const PROMPT_METADATA: &str = "inspect_metadata";

/// Resource URI: JSON Schema of the conversion options.
const RES_OPTIONS: &str = "htmltomarkdown://options-schema";
/// Resource URI: output-formats guide.
const RES_FORMATS: &str = "htmltomarkdown://output-formats";

/// Valid `output_format` values, used for both prompts and completions.
const OUTPUT_FORMATS: [&str; 3] = ["markdown", "djot", "plain"];

/// Markdown guide returned by the output-formats resource.
const OUTPUT_FORMATS_GUIDE: &str = "\
# html-to-markdown output formats

Set `config.output_format` on the `convert_html` tool:

- **markdown** (default) — CommonMark-compatible Markdown with GFM tables.
- **djot** — Djot lightweight markup (https://djot.net).
- **plain** — visible text only, with all markup stripped.

All three are produced from the same parse; switching format never changes which\n\
content is captured, only how it is rendered.
";

/// List the available prompts.
pub fn list_prompts() -> ListPromptsResult {
    let prompts = vec![
        Prompt::new(
            PROMPT_CONVERT,
            Some("Convert an HTML document to clean Markdown (or Djot/plain) using the convert_html tool."),
            Some(vec![
                PromptArgument::new("html")
                    .with_description("The HTML document to convert.")
                    .with_required(true),
                PromptArgument::new("output_format")
                    .with_description("Target format: markdown (default), djot, or plain."),
            ]),
        ),
        Prompt::new(
            PROMPT_EXTRACT,
            Some(
                "Extract the main article content from an HTML page as clean Markdown, \
                 stripping navigation, forms, and boilerplate.",
            ),
            Some(vec![
                PromptArgument::new("html")
                    .with_description("The HTML page to extract the article from.")
                    .with_required(true),
            ]),
        ),
        Prompt::new(
            PROMPT_METADATA,
            Some(
                "Summarise an HTML page's metadata (title, Open Graph, Twitter Card, JSON-LD, \
                 links, images) using the extract_metadata tool.",
            ),
            Some(vec![
                PromptArgument::new("html")
                    .with_description("The HTML page to inspect.")
                    .with_required(true),
            ]),
        ),
    ];
    ListPromptsResult::with_all_items(prompts)
        .with_ttl_ms(CATALOG_TTL_MS)
        .with_cache_scope(CacheScope::Public)
}

/// Resolve a prompt by name into its rendered messages.
///
/// # Errors
///
/// Returns `invalid_params` if the prompt name is unknown or a required
/// argument (`html`) is missing.
pub fn get_prompt(name: &str, arguments: Option<&JsonObject>) -> Result<GetPromptResult, McpError> {
    let html = required_arg(arguments, "html")?;
    match name {
        PROMPT_CONVERT => {
            let format = optional_arg(arguments, "output_format").unwrap_or_else(|| "markdown".to_string());
            let text = format!(
                "Convert the following HTML to {format} using the `convert_html` tool \
                 (set `config.output_format` to \"{format}\"). Return only the converted output.\n\n\
                 ```html\n{html}\n```"
            );
            Ok(prompt_result(format!("Convert HTML to {format}."), text))
        }
        PROMPT_EXTRACT => {
            let text = format!(
                "Use the `convert_html` tool on the following HTML with aggressive preprocessing \
                 (`config` = {{\"preprocessing\":{{\"preset\":\"aggressive\"}}}}) to strip navigation, \
                 forms, and boilerplate, then return the cleaned article as Markdown.\n\n\
                 ```html\n{html}\n```"
            );
            Ok(prompt_result(
                "Extract main article content as Markdown.".to_string(),
                text,
            ))
        }
        PROMPT_METADATA => {
            let text = format!(
                "Call the `extract_metadata` tool on the following HTML and summarise the key \
                 metadata (title, description, Open Graph, Twitter Card, structured data).\n\n\
                 ```html\n{html}\n```"
            );
            Ok(prompt_result("Summarise HTML metadata.".to_string(), text))
        }
        other => Err(McpError::invalid_params(format!("unknown prompt: {other}"), None)),
    }
}

/// List the readable resources.
pub fn list_resources() -> ListResourcesResult {
    let resources = vec![
        Resource::new(RES_OPTIONS, "Conversion options schema")
            .with_description("JSON Schema of every convert_html config option, with descriptions and defaults.")
            .with_mime_type("application/json"),
        Resource::new(RES_FORMATS, "Output formats guide")
            .with_description("How the markdown, djot, and plain output formats differ.")
            .with_mime_type("text/markdown"),
    ];
    ListResourcesResult::with_all_items(resources)
        .with_ttl_ms(CATALOG_TTL_MS)
        .with_cache_scope(CacheScope::Public)
}

/// Read a resource by URI.
///
/// # Errors
///
/// Returns `invalid_params` if the URI is not a known resource.
pub fn read_resource(uri: &str) -> Result<ReadResourceResult, McpError> {
    match uri {
        RES_OPTIONS => {
            let schema = schemars::schema_for!(super::params::ConvertConfig);
            let json = serde_json::to_string_pretty(&schema).unwrap_or_default();
            Ok(ReadResourceResult::new(vec![
                ResourceContents::text(json, uri).with_mime_type("application/json"),
            ])
            .with_ttl_ms(CATALOG_TTL_MS)
            .with_cache_scope(CacheScope::Public))
        }
        RES_FORMATS => Ok(ReadResourceResult::new(vec![
            ResourceContents::text(OUTPUT_FORMATS_GUIDE, uri).with_mime_type("text/markdown"),
        ])
        .with_ttl_ms(CATALOG_TTL_MS)
        .with_cache_scope(CacheScope::Public)),
        other => Err(McpError::invalid_params(format!("unknown resource: {other}"), None)),
    }
}

/// Complete an argument value for a prompt or resource reference.
pub fn complete(reference: &Reference, argument: &ArgumentInfo) -> CompleteResult {
    let values = match reference {
        Reference::Prompt(prompt) if prompt.name == PROMPT_CONVERT && argument.name == "output_format" => {
            prefix_filter(&OUTPUT_FORMATS, &argument.value)
        }
        _ => Vec::new(),
    };
    CompleteResult::new(CompletionInfo::with_all_values(values).unwrap_or_default())
}

/// Build a single-message user prompt result.
fn prompt_result(description: String, text: String) -> GetPromptResult {
    GetPromptResult::new(vec![PromptMessage::new_text(Role::User, text)]).with_description(description)
}

/// Read a required string argument, erroring if absent or empty.
fn required_arg(arguments: Option<&JsonObject>, key: &str) -> Result<String, McpError> {
    optional_arg(arguments, key)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| McpError::invalid_params(format!("missing required argument: {key}"), None))
}

/// Read an optional string argument.
fn optional_arg(arguments: Option<&JsonObject>, key: &str) -> Option<String> {
    arguments?.get(key)?.as_str().map(ToString::to_string)
}

/// Candidate values whose normalised form starts with the normalised partial input.
fn prefix_filter(candidates: &[&str], partial: &str) -> Vec<String> {
    let needle = normalize_token(partial);
    candidates
        .iter()
        .filter(|c| normalize_token(c).starts_with(&needle))
        .map(|c| (*c).to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_prompts_exposes_three() {
        let names: Vec<String> = list_prompts().prompts.into_iter().map(|p| p.name).collect();
        assert_eq!(names, vec![PROMPT_CONVERT, PROMPT_EXTRACT, PROMPT_METADATA]);
    }

    #[test]
    fn test_convert_prompt_embeds_html_and_format() {
        let mut args = JsonObject::new();
        args.insert("html".into(), "<h1>Hi</h1>".into());
        args.insert("output_format".into(), "djot".into());
        let result = get_prompt(PROMPT_CONVERT, Some(&args)).expect("must resolve");
        let text = match &result.messages[0].content {
            rmcp::model::ContentBlock::Text(t) => t.text.clone(),
            _ => panic!("expected text content"),
        };
        assert!(text.contains("<h1>Hi</h1>"), "must embed the html");
        assert!(text.contains("djot"), "must reference the requested format");
    }

    #[test]
    fn test_convert_prompt_defaults_format_to_markdown() {
        let mut args = JsonObject::new();
        args.insert("html".into(), "<p>x</p>".into());
        let result = get_prompt(PROMPT_CONVERT, Some(&args)).expect("must resolve");
        let text = match &result.messages[0].content {
            rmcp::model::ContentBlock::Text(t) => t.text.clone(),
            _ => panic!("expected text content"),
        };
        assert!(text.contains("markdown"), "default format is markdown");
    }

    #[test]
    fn test_prompt_missing_html_errors() {
        let err = get_prompt(PROMPT_CONVERT, None).expect_err("missing html must error");
        assert_eq!(err.code.0, -32602, "must be invalid_params");
    }

    #[test]
    fn test_unknown_prompt_errors() {
        let mut args = JsonObject::new();
        args.insert("html".into(), "<p>x</p>".into());
        let err = get_prompt("does_not_exist", Some(&args)).expect_err("unknown prompt must error");
        assert_eq!(err.code.0, -32602);
    }

    #[test]
    fn test_list_resources_exposes_both_uris() {
        let uris: Vec<String> = list_resources().resources.into_iter().map(|r| r.uri).collect();
        assert!(uris.contains(&RES_OPTIONS.to_string()));
        assert!(uris.contains(&RES_FORMATS.to_string()));
    }

    #[test]
    fn test_read_options_resource_is_schema_json() {
        let result = read_resource(RES_OPTIONS).expect("must read");
        let text = match &result.contents[0] {
            ResourceContents::TextResourceContents { text, mime_type, .. } => {
                assert_eq!(mime_type.as_deref(), Some("application/json"));
                text.clone()
            }
            ResourceContents::BlobResourceContents { .. } => panic!("expected text"),
            _ => panic!("unexpected resource contents variant"),
        };
        let parsed: serde_json::Value = serde_json::from_str(&text).expect("schema must be valid JSON");
        assert!(
            parsed.to_string().contains("heading_style"),
            "options schema must list the config fields"
        );
    }

    #[test]
    fn test_read_formats_resource_is_markdown() {
        let result = read_resource(RES_FORMATS).expect("must read");
        let text = match &result.contents[0] {
            ResourceContents::TextResourceContents { text, .. } => text.clone(),
            ResourceContents::BlobResourceContents { .. } => panic!("expected text"),
            _ => panic!("unexpected resource contents variant"),
        };
        assert!(text.contains("Djot"), "guide must describe djot");
    }

    #[test]
    fn test_unknown_resource_errors() {
        let err = read_resource("htmltomarkdown://nope").expect_err("unknown resource must error");
        assert_eq!(err.code.0, -32602);
    }

    #[test]
    fn test_complete_output_format_prefix() {
        let reference = Reference::for_prompt(PROMPT_CONVERT);
        let arg = ArgumentInfo::new("output_format", "d");
        let result = complete(&reference, &arg);
        assert_eq!(result.completion.values, vec!["djot"]);
        assert_eq!(result.completion.total, Some(1));
    }

    #[test]
    fn test_complete_output_format_empty_returns_all() {
        let reference = Reference::for_prompt(PROMPT_CONVERT);
        let arg = ArgumentInfo::new("output_format", "");
        let result = complete(&reference, &arg);
        assert_eq!(result.completion.values, vec!["markdown", "djot", "plain"]);
    }

    #[test]
    fn test_complete_unknown_argument_is_empty() {
        let reference = Reference::for_prompt(PROMPT_CONVERT);
        let arg = ArgumentInfo::new("unknown", "");
        assert_eq!(
            complete(&reference, &arg).completion.values,
            [] as [std::string::String; 0]
        );
    }
}
