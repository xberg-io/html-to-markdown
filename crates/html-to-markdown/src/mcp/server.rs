//! HTML-to-Markdown MCP server implementation.

use crate::options::ConversionOptions;
use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, Content, Implementation, InitializeResult, ServerCapabilities, ServerInfo, ToolsCapability,
    },
    tool, tool_handler, tool_router,
    transport::stdio,
};

#[cfg(feature = "mcp-http")]
use rmcp::transport::streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager};

/// HTML-to-Markdown MCP server.
///
/// Exposes two tools:
/// - `convert_html` — convert HTML to Markdown (or full JSON output) with typed
///   `ConvertConfig` options.
/// - `extract_metadata` — extract structured `<head>`/`<meta>` metadata as JSON.
#[cfg_attr(alef, alef(skip))]
#[derive(Clone)]
pub struct HtmlToMarkdownMcp {
    // Consumed by the `#[tool_router]` macro-generated dispatch code; not
    // accessed directly in hand-written Rust, hence the allow.
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl HtmlToMarkdownMcp {
    /// Create a new server instance.
    pub(crate) fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Convert HTML to Markdown.
    ///
    /// Converts the provided HTML string to Markdown using the html-to-markdown engine.
    /// Pass `json: true` to receive the full `ConversionResult` as a JSON object
    /// (including content, tables, document structure, metadata, and warnings).
    /// Pass `config` to customise conversion behaviour with typed options.
    #[tool(
        description = "Convert HTML to Markdown (or Djot/plain via config.output_format). Pass json:true for the full ConversionResult (content, tables, document structure, metadata, warnings). Pass config to control heading style, list formatting, escaping, preprocessing, image extraction, and more — see the input schema for every option.",
        annotations(
            title = "Convert HTML",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn convert_html(
        &self,
        Parameters(params): Parameters<super::params::ConvertHtmlParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        use super::errors::map_conversion_error_to_mcp;
        use super::format::format_conversion_result;

        // Build typed options from the config mirror (defaults when omitted).
        let opts: ConversionOptions = params.config.map(Into::into).unwrap_or_default();

        let html = params.html;
        let want_json = params.json;

        // `convert` is synchronous and CPU-bound; run it on the blocking thread pool
        // to avoid blocking the async runtime.
        let result = tokio::task::spawn_blocking(move || crate::convert(&html, opts))
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(format!("Conversion task panicked: {e}"), None))?
            .map_err(map_conversion_error_to_mcp)?;

        let text = if want_json {
            format_conversion_result(&result)
        } else {
            result.content.unwrap_or_default()
        };

        Ok(CallToolResult::success(vec![Content::text(text)]))
    }

    /// Extract structured metadata from HTML.
    ///
    /// Runs the metadata extraction pass and returns only the `HtmlMetadata`
    /// (document title/description, Open Graph, Twitter Card, JSON-LD/microdata,
    /// headers, links, images) serialised as JSON.
    #[tool(
        description = "Extract structured metadata from HTML as JSON: document title/description/keywords/author, Open Graph and Twitter Card tags, JSON-LD and microdata, plus header, link, and image inventories. Convenience over convert_html for metadata-only use.",
        annotations(
            title = "Extract HTML Metadata",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn extract_metadata(
        &self,
        Parameters(params): Parameters<super::params::ExtractMetadataParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        use super::errors::map_conversion_error_to_mcp;
        use super::format::format_metadata;

        let opts = ConversionOptions {
            extract_metadata: true,
            ..ConversionOptions::default()
        };
        let html = params.html;

        let result = tokio::task::spawn_blocking(move || crate::convert(&html, opts))
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(format!("Conversion task panicked: {e}"), None))?
            .map_err(map_conversion_error_to_mcp)?;

        Ok(CallToolResult::success(vec![Content::text(format_metadata(
            &result.metadata,
        ))]))
    }
}

#[tool_handler]
impl ServerHandler for HtmlToMarkdownMcp {
    fn get_info(&self) -> ServerInfo {
        let mut capabilities = ServerCapabilities::default();
        capabilities.tools = Some(ToolsCapability::default());

        let server_info = Implementation::new("html-to-markdown-mcp", env!("CARGO_PKG_VERSION"))
            .with_title("HTML-to-Markdown MCP Server")
            .with_description(
                "Fast, lossless HTML to Markdown conversion. \
                 Supports optional ConversionOptions for heading style, list formatting, \
                 escaping, metadata extraction, and more.",
            )
            .with_website_url("https://github.com/kreuzberg-dev/html-to-markdown");

        InitializeResult::new(capabilities)
            .with_server_info(server_info)
            .with_instructions(
                "Two tools are available. convert_html converts an HTML string to Markdown \
                 (or Djot/plain via config.output_format); pass json:true for the full \
                 ConversionResult (content, tables, document structure, metadata, warnings), \
                 and pass config for typed options (heading_style, escape_asterisks, \
                 preprocessing, extract_images, …) — every option is described in the tool's \
                 input schema. extract_metadata returns only the structured metadata \
                 (title, Open Graph, Twitter Card, JSON-LD, headers, links, images) as JSON.",
            )
    }
}

/// Start the HTML-to-Markdown MCP server using stdio transport.
///
/// Blocks until the server shuts down.
///
/// # Errors
///
/// Returns an error if the server fails to start or encounters a fatal error.
///
/// # Example
///
/// ```rust,no_run
/// use html_to_markdown_rs::mcp::start_mcp_server;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     start_mcp_server().await?;
///     Ok(())
/// }
/// ```
#[cfg_attr(alef, alef(skip))]
pub async fn start_mcp_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let service = HtmlToMarkdownMcp::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}

/// Start the HTML-to-Markdown MCP server with HTTP Stream transport.
///
/// # Arguments
///
/// * `host` - Host to bind to (e.g., `"127.0.0.1"` or `"0.0.0.0"`)
/// * `port` - Port number (e.g., `8001`)
///
/// # Example
///
/// ```no_run
/// use html_to_markdown_rs::mcp::start_mcp_server_http;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     start_mcp_server_http("127.0.0.1", 8001).await?;
///     Ok(())
/// }
/// ```
#[cfg(feature = "mcp-http")]
#[cfg_attr(alef, alef(skip))]
pub async fn start_mcp_server_http(
    host: impl AsRef<str>,
    port: u16,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use axum::Router;
    use std::net::SocketAddr;

    let http_service = StreamableHttpService::new(
        || Ok(HtmlToMarkdownMcp::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    let router = Router::new().nest_service("/mcp", http_service);

    let addr: SocketAddr = format!("{}:{}", host.as_ref(), port)
        .parse()
        .map_err(|e| format!("Invalid address: {e}"))?;

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::params::{ConvertConfig, ConvertHtmlParams, ExtractMetadataParams};
    use super::*;
    use rmcp::model::{ProtocolVersion, RawContent};

    fn text_of(result: &CallToolResult) -> String {
        match &result.content[0].raw {
            RawContent::Text(t) => t.text.clone(),
            _ => panic!("expected text content"),
        }
    }

    #[test]
    fn test_tool_router_has_both_tools() {
        let router = HtmlToMarkdownMcp::tool_router();
        assert!(router.has_route("convert_html"), "convert_html tool must be registered");
        assert!(
            router.has_route("extract_metadata"),
            "extract_metadata tool must be registered"
        );
    }

    #[test]
    fn test_tools_carry_read_only_annotations() {
        let router = HtmlToMarkdownMcp::tool_router();
        for name in ["convert_html", "extract_metadata"] {
            let tool = router.get(name).unwrap_or_else(|| panic!("{name} must exist"));
            let ann = tool
                .annotations
                .as_ref()
                .unwrap_or_else(|| panic!("{name} must carry annotations"));
            assert_eq!(ann.read_only_hint, Some(true), "{name} read_only_hint");
            assert_eq!(ann.idempotent_hint, Some(true), "{name} idempotent_hint");
            assert_eq!(ann.destructive_hint, Some(false), "{name} destructive_hint");
            assert_eq!(ann.open_world_hint, Some(false), "{name} open_world_hint");
            assert!(ann.title.is_some(), "{name} title");
        }
    }

    #[test]
    fn test_convert_html_input_schema_exposes_typed_config() {
        let router = HtmlToMarkdownMcp::tool_router();
        let tool = router.get("convert_html").expect("convert_html must exist");
        let schema = serde_json::to_string(&tool.input_schema).expect("schema serialises");
        assert!(
            schema.contains("heading_style"),
            "input schema must expose heading_style"
        );
        assert!(
            schema.contains("output_format"),
            "input schema must expose output_format"
        );
        assert!(
            schema.contains("preprocessing"),
            "input schema must expose preprocessing"
        );
    }

    #[test]
    fn test_server_info_fields() {
        let server = HtmlToMarkdownMcp::new();
        let info = server.get_info();

        assert_eq!(info.server_info.name, "html-to-markdown-mcp");
        assert_eq!(info.server_info.version, env!("CARGO_PKG_VERSION"));
        assert!(info.capabilities.tools.is_some());
        assert!(info.instructions.is_some());
    }

    #[test]
    fn test_server_info_has_description() {
        let server = HtmlToMarkdownMcp::new();
        let info = server.get_info();
        assert!(info.server_info.title.is_some());
        assert!(info.server_info.website_url.is_some());
    }

    #[test]
    fn test_server_info_protocol_version() {
        let server = HtmlToMarkdownMcp::new();
        let info = server.get_info();
        assert_eq!(info.protocol_version, ProtocolVersion::default());
    }

    #[tokio::test]
    async fn test_convert_html_basic() {
        let server = HtmlToMarkdownMcp::new();
        let params = ConvertHtmlParams {
            html: "<h1>Hello</h1>".into(),
            config: None,
            json: false,
        };
        let result = server
            .convert_html(Parameters(params))
            .await
            .expect("conversion must succeed");

        assert!(!result.content.is_empty(), "result must have content");
        let text = text_of(&result);
        assert!(text.contains("# Hello"), "markdown must contain heading; got: {text}");
    }

    #[tokio::test]
    async fn test_convert_html_json_output() {
        let server = HtmlToMarkdownMcp::new();
        let params = ConvertHtmlParams {
            html: "<h1>World</h1>".into(),
            config: None,
            json: true,
        };
        let result = server
            .convert_html(Parameters(params))
            .await
            .expect("conversion must succeed");

        let text = text_of(&result);
        let parsed: serde_json::Value = serde_json::from_str(&text).expect("json output must be valid JSON");
        assert!(parsed.get("content").is_some(), "JSON must have content field");
    }

    #[tokio::test]
    async fn test_convert_html_with_typed_config() {
        let server = HtmlToMarkdownMcp::new();
        let params = ConvertHtmlParams {
            html: "<p>*bold*</p>".into(),
            config: Some(ConvertConfig {
                escape_asterisks: Some(true),
                ..ConvertConfig::default()
            }),
            json: false,
        };
        let result = server
            .convert_html(Parameters(params))
            .await
            .expect("conversion must succeed");

        let text = text_of(&result);
        assert_eq!(text.trim(), r"\*bold\*", "escape_asterisks must escape both asterisks");
    }

    #[tokio::test]
    async fn test_convert_html_output_format_djot() {
        let server = HtmlToMarkdownMcp::new();
        let params = ConvertHtmlParams {
            html: "<h1>Hi</h1>".into(),
            config: Some(ConvertConfig {
                output_format: Some("djot".into()),
                ..ConvertConfig::default()
            }),
            json: false,
        };
        let result = server
            .convert_html(Parameters(params))
            .await
            .expect("conversion must succeed");
        assert!(
            text_of(&result).contains("Hi"),
            "djot output must carry the heading text"
        );
    }

    #[tokio::test]
    async fn test_extract_metadata_returns_metadata_json() {
        let server = HtmlToMarkdownMcp::new();
        let html = r#"<html><head><title>My Page</title>
            <meta property="og:title" content="OG Title"></head><body><p>hi</p></body></html>"#;
        let params = ExtractMetadataParams { html: html.into() };
        let result = server
            .extract_metadata(Parameters(params))
            .await
            .expect("metadata extraction must succeed");

        let parsed: serde_json::Value = serde_json::from_str(&text_of(&result)).expect("valid JSON");
        assert_eq!(parsed["document"]["title"], "My Page", "title must be extracted");
        assert!(parsed.get("structured_data").is_some(), "metadata JSON shape present");
    }
}
