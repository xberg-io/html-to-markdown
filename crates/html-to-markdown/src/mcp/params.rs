//! MCP request parameter types.

use rmcp::schemars;

/// Parameters for the `convert_html` MCP tool.
///
/// `options` is an opaque JSON object so `ConversionOptions` does not need to
/// implement `JsonSchema` itself — the same pattern used by kreuzberg's MCP.
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ConvertHtmlParams {
    /// The HTML string to convert to Markdown.
    pub html: String,

    /// Optional conversion options as a JSON object.
    ///
    /// Any field from `ConversionOptions` may be set here. Unknown fields
    /// are rejected (the struct uses `deny_unknown_fields`).
    #[serde(default)]
    pub options: Option<serde_json::Value>,

    /// When `true`, return the full `ConversionResult` serialised as JSON
    /// instead of the bare Markdown string.
    #[serde(default)]
    pub json: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_html_params_minimal() {
        let json = r#"{"html": "<h1>Hi</h1>"}"#;
        let params: ConvertHtmlParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.html, "<h1>Hi</h1>");
        assert!(params.options.is_none());
        assert!(!params.json);
    }

    #[test]
    fn test_convert_html_params_with_options() {
        let json = r#"{"html": "<p>x</p>", "options": {"heading_style": "Atx"}, "json": true}"#;
        let params: ConvertHtmlParams = serde_json::from_str(json).unwrap();
        assert!(params.options.is_some());
        assert!(params.json);
    }

    #[test]
    fn test_convert_html_params_options_parsed_into_conversion_options() {
        use crate::options::ConversionOptions;
        let json = r#"{"html": "<b>x</b>", "options": {"escape_asterisks": true}}"#;
        let params: ConvertHtmlParams = serde_json::from_str(json).unwrap();
        let opts: ConversionOptions =
            serde_json::from_value(params.options.unwrap()).expect("options must deserialize");
        assert!(opts.escape_asterisks);
    }
}
