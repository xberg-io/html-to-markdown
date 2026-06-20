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
/// Exposes a single `convert_html` tool that converts HTML to Markdown
/// (or full JSON output) with optional `ConversionOptions`.
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
    /// Pass `options` as a JSON object to customise conversion behaviour.
    #[tool(
        description = "Convert HTML to Markdown. Pass json:true for full ConversionResult (content, tables, metadata, warnings). Pass options as a JSON object to control heading style, list indent, escaping, etc.",
        annotations(title = "Convert HTML", read_only_hint = true, idempotent_hint = true)
    )]
    async fn convert_html(
        &self,
        Parameters(params): Parameters<super::params::ConvertHtmlParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        use super::errors::map_conversion_error_to_mcp;
        use super::format::format_conversion_result;

        // Deserialize options from opaque JSON (kreuzberg pattern: opaque options).
        let opts: ConversionOptions = match params.options {
            Some(v) => serde_json::from_value(v)
                .map_err(|e| rmcp::ErrorData::invalid_params(format!("Invalid conversion options: {e}"), None))?,
            None => ConversionOptions::default(),
        };

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
                "Use the convert_html tool to convert HTML strings to Markdown. \
                 Pass json:true to receive the full ConversionResult (content, tables, \
                 document structure, metadata, warnings). \
                 Pass options as a JSON object to customise conversion (e.g., heading_style, \
                 escape_asterisks, extract_metadata).",
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
    use super::*;
    use rmcp::model::{ProtocolVersion, RawContent};

    #[test]
    fn test_tool_router_has_convert_html() {
        let router = HtmlToMarkdownMcp::tool_router();
        assert!(router.has_route("convert_html"), "convert_html tool must be registered");
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
        let params = super::super::params::ConvertHtmlParams {
            html: "<h1>Hello</h1>".into(),
            options: None,
            json: false,
        };
        let result = server
            .convert_html(Parameters(params))
            .await
            .expect("conversion must succeed");

        assert!(!result.content.is_empty(), "result must have content");
        let text = match &result.content[0].raw {
            RawContent::Text(t) => t.text.clone(),
            _ => panic!("expected text content"),
        };
        assert!(text.contains("# Hello"), "markdown must contain heading; got: {text}");
    }

    #[tokio::test]
    async fn test_convert_html_json_output() {
        let server = HtmlToMarkdownMcp::new();
        let params = super::super::params::ConvertHtmlParams {
            html: "<h1>World</h1>".into(),
            options: None,
            json: true,
        };
        let result = server
            .convert_html(Parameters(params))
            .await
            .expect("conversion must succeed");

        let text = match &result.content[0].raw {
            RawContent::Text(t) => t.text.clone(),
            _ => panic!("expected text content"),
        };
        let parsed: serde_json::Value = serde_json::from_str(&text).expect("json output must be valid JSON");
        assert!(parsed.get("content").is_some(), "JSON must have content field");
    }

    #[tokio::test]
    async fn test_convert_html_with_options() {
        let server = HtmlToMarkdownMcp::new();
        let opts = serde_json::json!({"escape_asterisks": true});
        let params = super::super::params::ConvertHtmlParams {
            html: "<p>*bold*</p>".into(),
            options: Some(opts),
            json: false,
        };
        let result = server
            .convert_html(Parameters(params))
            .await
            .expect("conversion must succeed");

        // Should succeed (options were valid)
        assert!(!result.content.is_empty());
    }

    #[tokio::test]
    async fn test_convert_html_invalid_options_returns_error() {
        let server = HtmlToMarkdownMcp::new();
        let opts = serde_json::json!({"unknown_field_xyz": true});
        let params = super::super::params::ConvertHtmlParams {
            html: "<p>test</p>".into(),
            options: Some(opts),
            json: false,
        };
        let result = server.convert_html(Parameters(params)).await;
        assert!(result.is_err(), "unknown option field must produce an error");
        let err = result.unwrap_err();
        assert_eq!(err.code.0, -32602, "must be invalid_params");
    }
}
