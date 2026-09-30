//! List element handlers for HTML to Markdown conversion.
//!
//! This module provides specialized handling for various list types:
//! - **Ordered lists**: `<ol>` with counter management and formatting options
//! - **Unordered lists**: `<ul>` with bullet cycling based on nesting depth
//! - **List items**: `<li>` with task list and block-level detection
//! - **Definition lists**: `<dl>`, `<dt>`, `<dd>` elements
//! - **List utilities**: Indentation, loose/tight list detection, nesting depth calculation

pub mod definition;
pub mod item;
pub mod ordered;
pub mod unordered;
pub mod utils;

/// Dispatches list element handling to the appropriate handler.
///
/// Returns `true` if the element was handled, `false` otherwise.
///
/// # Supported Elements
///
/// - `ol`: Ordered list - routed to `ordered::handle`
/// - `ul`: Unordered list - routed to `unordered::handle`
/// - `li`: List item - routed to `item::handle_li`
/// - `dl`: Definition list - routed to `definition::handle_dl`
/// - `dt`: Definition term - routed to `definition::handle_dt`
/// - `dd`: Definition description - routed to `definition::handle_dd`
pub fn dispatch_list_handler(
    tag_name: &str,
    node_handle: &tl::NodeHandle,
    tag: &tl::HTMLTag,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::Context,
    depth: usize,
    dom_ctx: &super::DomContext,
) -> bool {
    match tag_name {
        "ol" => {
            ordered::handle(node_handle, parser, output, options, ctx, depth, dom_ctx);
            true
        }
        // ~keep A menu is an unordered list in HTML, so it takes the `<ul>` path: the same bullet,
        // ~keep blank lines and nesting (issue #657).
        "ul" | "menu" => {
            unordered::handle(node_handle, parser, output, options, ctx, depth, dom_ctx);
            true
        }
        "li" => {
            item::handle_li(node_handle, tag, parser, output, options, ctx, depth, dom_ctx);
            true
        }
        "dl" => {
            definition::handle_dl(node_handle, parser, output, options, ctx, depth, dom_ctx);
            true
        }
        "dt" => {
            definition::handle_dt(node_handle, parser, output, options, ctx, depth, dom_ctx);
            true
        }
        "dd" => {
            definition::handle_dd(node_handle, parser, output, options, ctx, depth, dom_ctx);
            true
        }
        _ => false,
    }
}
