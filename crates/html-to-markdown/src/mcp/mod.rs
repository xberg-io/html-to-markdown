//! Model Context Protocol (MCP) server implementation.
//!
//! Provides an MCP server that exposes html-to-markdown conversion as an MCP tool.
//!
//! # Features
//!
//! - **convert_html**: Convert HTML to Markdown (or JSON) with optional `ConversionOptions`
//!
//! # Example
//!
//! ```rust,no_run
//! use html_to_markdown_rs::mcp::start_mcp_server;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     start_mcp_server().await?;
//!     Ok(())
//! }
//! ```

mod errors;
mod format;
mod params;
mod server;

pub use server::start_mcp_server;
#[cfg(feature = "mcp-http")]
pub use server::start_mcp_server_http;
