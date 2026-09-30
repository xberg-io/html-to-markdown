//! Handlers for HTML5 interactive elements.
//!
//! Processes interactive disclosure semantic elements:
//! - `<details>` - Expandable/collapsible disclosure widget
//! - `<summary>` - Summary or caption for a details element
//!
//! These elements are treated as block-level content containers
//! with special formatting for the summary element.

use super::walk_node;
#[cfg(feature = "visitor")]
use std::borrow::Cow;

/// Handles the `<details>` element.
///
/// A details element represents a disclosure widget that can be toggled
/// to show/hide additional content. In Markdown, it's rendered as a block
/// with all content visible.
///
/// # Behavior
///
/// - **Inline mode**: Children are processed inline without block spacing
/// - **Block mode**: Content is collected and wrapped with proper blank-line spacing
/// - **Empty content**: Skipped entirely
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle_details(
    _tag_name: &str,
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::Context,
    depth: usize,
    dom_ctx: &super::DomContext,
) {
    if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
        #[cfg(feature = "visitor")]
        if let Some(ref visitor_handle) = ctx.visitor {
            use crate::visitor::{NodeContext, NodeType, VisitResult};

            let node_id = node_handle.get_inner();
            let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
            let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);
            let open = tag.attributes().get("open").is_some();
            let node_ctx = NodeContext::with_lazy_attributes(
                NodeType::Details,
                Cow::Borrowed("details"),
                tag,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                false,
            );
            let visit_result = {
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
                visitor.visit_details(&node_ctx, open)
            };
            match visit_result {
                VisitResult::Continue => {}
                VisitResult::Skip => return,
                VisitResult::Custom(custom) => {
                    if !output.is_empty() && !output.ends_with("\n\n") {
                        output.push_str("\n\n");
                    }
                    output.push_str(&custom);
                    output.push_str("\n\n");
                    return;
                }
                VisitResult::PreserveHtml => {
                    use crate::converter::utility::serialization::serialize_node;
                    output.push_str(&serialize_node(node_handle, parser));
                    return;
                }
                VisitResult::Error(err) => {
                    if ctx.visitor_error.borrow().is_none() {
                        *ctx.visitor_error.borrow_mut() = Some(err);
                    }
                    return;
                }
            }
        }

        if ctx.convert_as_inline {
            let children = tag.children();
            {
                for child_handle in children.top().iter() {
                    super::walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
                }
            }
            return;
        }

        let mut content = String::with_capacity(256);
        let children = tag.children();
        {
            let details_ctx = crate::converter::list::utils::nested_block_context(output, ctx, options);
            for child_handle in children.top().iter() {
                walk_node(
                    child_handle,
                    parser,
                    &mut content,
                    options,
                    &details_ctx,
                    depth + 1,
                    dom_ctx,
                );
            }
        }

        // ~keep A trailing <br> run with no following sibling has no next dispatch to catch
        // ~keep it in `walk_node`'s pre-block-dispatch strip, since the details content is
        // ~keep simply finished here — so this closes its own trailing run the same way
        // ~keep `paragraph.rs` closes its own (issue #464 follow-up).
        crate::converter::main_helpers::strip_trailing_backslash_breaks_from_fresh_buffer(
            &mut content,
            options.newline_style,
        );

        let trimmed = content.trim();
        if !trimmed.is_empty() {
            crate::converter::list::utils::start_container_block(output, ctx, options);
            output.push_str(trimmed);
            output.push_str("\n\n");
        }
    }
}

/// Handles the `<summary>` element.
///
/// A summary element contains a caption for a details element.
/// It is rendered as strong (bold) text to distinguish it from regular content.
///
/// # Behavior
///
/// - **Inline mode**: Content is rendered inline without emphasis
/// - **Block mode**: Content is wrapped in strong markers (e.g., `**text**`)
/// - Uses the configured strong/emphasis symbol from `ConversionOptions`
///
/// # Implementation Details
///
/// The handler:
/// 1. Creates a context with `in_strong: true` for nested formatting
/// 2. Collects content from all children
/// 3. Wraps non-empty content in strong markers (repeated twice per Markdown spec)
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle_summary(
    _tag_name: &str,
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::Context,
    depth: usize,
    dom_ctx: &super::DomContext,
) {
    if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
        let mut content = String::with_capacity(64);

        // ~keep Set strong context for nested content. Skip the clone entirely
        // ~keep when the resulting Context would be byte-identical to ctx —
        // ~keep either convert_as_inline (block path inactive) or already
        // ~keep in_strong upstream.
        let want_strong = !ctx.convert_as_inline;
        let summary_ctx_owned;
        let summary_ctx = if want_strong && !ctx.in_strong {
            summary_ctx_owned = super::Context {
                in_strong: true,
                text_in_markers: true,
                ..ctx.clone()
            };
            &summary_ctx_owned
        } else {
            ctx
        };

        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                super::walk_node(
                    child_handle,
                    parser,
                    &mut content,
                    options,
                    summary_ctx,
                    depth + 1,
                    dom_ctx,
                );
            }
        }

        // ~keep A trailing <br> run with no following sibling has no next dispatch to catch
        // ~keep it in `walk_node`'s pre-block-dispatch strip, since the summary content is
        // ~keep simply finished here — so this closes its own trailing run the same way
        // ~keep `paragraph.rs` closes its own (issue #464 follow-up).
        crate::converter::main_helpers::strip_trailing_backslash_breaks_from_fresh_buffer(
            &mut content,
            options.newline_style,
        );

        let trimmed = content.trim();
        if trimmed.is_empty() {
            return;
        }

        #[cfg(feature = "visitor")]
        if let Some(ref visitor_handle) = ctx.visitor {
            use crate::visitor::{NodeContext, NodeType, VisitResult};

            let node_id = node_handle.get_inner();
            let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
            let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);
            let node_ctx = NodeContext::with_lazy_attributes(
                NodeType::Summary,
                Cow::Borrowed("summary"),
                tag,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                false,
            );
            let visit_result = {
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
                visitor.visit_summary(&node_ctx, trimmed)
            };
            match visit_result {
                VisitResult::Continue => {}
                VisitResult::Skip => return,
                VisitResult::Custom(custom) => {
                    if ctx.convert_as_inline {
                        output.push_str(&custom);
                    } else {
                        output.push_str(&custom);
                        if !custom.ends_with('\n') {
                            output.push_str("\n\n");
                        }
                    }
                    return;
                }
                VisitResult::PreserveHtml => {
                    use crate::converter::utility::serialization::serialize_node;
                    output.push_str(&serialize_node(node_handle, parser));
                    return;
                }
                VisitResult::Error(err) => {
                    if ctx.visitor_error.borrow().is_none() {
                        *ctx.visitor_error.borrow_mut() = Some(err);
                    }
                    return;
                }
            }
        }

        if ctx.convert_as_inline {
            output.push_str(trimmed);
        } else {
            crate::converter::block::div::start_block(output, ctx, options);
            let mut symbol = String::with_capacity(2);
            symbol.push(options.strong_em_symbol);
            symbol.push(options.strong_em_symbol);
            output.push_str(&symbol);
            output.push_str(trimmed);
            output.push_str(&symbol);
            output.push_str("\n\n");
        }
    }
}

/// Dispatcher for interactive elements.
///
/// Routes `<details>` and `<summary>` elements to their handlers.
pub fn handle(
    tag_name: &str,
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::Context,
    depth: usize,
    dom_ctx: &super::DomContext,
) {
    match tag_name {
        "details" => handle_details(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx),
        "summary" => handle_summary(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx),
        _ => {}
    }
}
