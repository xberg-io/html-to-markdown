//! Blockquote element handler for HTML to Markdown conversion.
//!
//! Handles `<blockquote>` elements including:
//! - Basic blockquote markdown output with `> ` prefix
//! - Nested blockquotes
//! - Citation URLs via `cite` attribute
//! - Visitor callback integration

use crate::converter::Context;
use crate::converter::dom_context::DomContext;
use crate::converter::main::walk_node;
use crate::options::ConversionOptions;

#[cfg(feature = "visitor")]
use crate::converter::utility::content::collect_tag_attributes;
#[cfg(feature = "visitor")]
use std::collections::BTreeMap;

#[cfg(feature = "visitor")]
use crate::converter::utility::serialization::serialize_node_to_html;

/// Handle a `<blockquote>` element and convert to Markdown.
///
/// This handler processes blockquote elements including:
/// - Converting inline blockquotes by processing children as inline
/// - Handling nested blockquotes via blockquote_depth tracking
/// - Processing citation URLs from cite attribute
/// - Invoking visitor callbacks when the visitor feature is enabled
/// - Adding proper spacing and blockquote prefix formatting
#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle_blockquote(
    node_handle: &tl::NodeHandle,
    tag: &tl::HTMLTag,
    parser: &tl::Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    // If in inline conversion mode, just process children inline
    if ctx.convert_as_inline {
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
            }
        }
        return;
    }

    // Extract cite attribute if present
    let cite = tag
        .attributes()
        .get("cite")
        .flatten()
        .map(|v| v.as_utf8_str().to_string());

    // Create context for nested blockquote processing
    let blockquote_ctx = Context {
        blockquote_depth: ctx.blockquote_depth + 1,
        ..ctx.clone()
    };

    // Process blockquote content
    let mut content = String::with_capacity(256);
    let children = tag.children();
    {
        for child_handle in children.top().iter() {
            walk_node(
                child_handle,
                parser,
                &mut content,
                options,
                &blockquote_ctx,
                depth + 1,
                dom_ctx,
            );
        }
    }

    let trimmed_content = content.trim();

    // Handle visitor integration
    #[cfg(feature = "visitor")]
    if let Some(ref visitor) = ctx.visitor {
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        let attributes: BTreeMap<String, String> = collect_tag_attributes(tag);

        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

        let node_ctx = NodeContext {
            node_type: NodeType::Blockquote,
            tag_name: "blockquote".to_string(),
            attributes,
            depth,
            index_in_parent,
            parent_tag,
            is_inline: false,
        };

        let mut visitor_ref = visitor.borrow_mut();
        match visitor_ref.visit_blockquote(&node_ctx, trimmed_content, ctx.blockquote_depth) {
            VisitResult::Continue => {}
            VisitResult::Custom(custom) => {
                output.push_str(&custom);
                return;
            }
            VisitResult::Skip => return,
            VisitResult::PreserveHtml => {
                let mut html_output = String::new();
                serialize_node_to_html(node_handle, parser, &mut html_output);
                output.push_str(&html_output);
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

    // Output blockquote if content is not empty
    if !trimmed_content.is_empty() {
        // Add proper spacing based on blockquote depth
        if ctx.blockquote_depth > 0 {
            output.push_str("\n\n\n");
        } else if !output.is_empty() {
            if output.ends_with("\n\n") {
                // Paragraph already added \n\n; blockquote needs just \n
                output.truncate(output.len() - 1);
            } else if !output.ends_with('\n') {
                output.push_str("\n\n");
            } else if !output.ends_with("\n\n") {
                output.push('\n');
            }
        }

        // Add blockquote prefix to each line
        let prefix = "> ";

        for line in trimmed_content.lines() {
            output.push_str(prefix);
            output.push_str(line.trim());
            output.push('\n');
        }

        // Add citation if present
        if let Some(url) = cite {
            output.push('\n');
            output.push_str("— <");
            output.push_str(&url);
            output.push_str(">\n\n");
        }

        // Add trailing newlines only when appropriate for proper spacing
        // (matching paragraph conditional logic for CommonMark compliance)
        if !ctx.convert_as_inline && !ctx.in_table_cell && !ctx.in_list_item {
            while output.ends_with('\n') {
                output.truncate(output.len() - 1);
            }
            output.push_str("\n\n");
        }
    }
}
