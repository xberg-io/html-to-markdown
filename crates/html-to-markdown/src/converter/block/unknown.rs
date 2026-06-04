//! Handler for unknown/unspecified HTML elements.
//!
//! Processes HTML elements that don't have specific handlers by passing through
//! their children while managing whitespace and content preservation:
//! - Processes all child nodes recursively
//! - Validates UTF-8 character boundaries when checking content
//! - Preserves code blocks (indented or fenced) while removing empty content
//! - Manages trailing whitespace intelligently

use crate::options::ConversionOptions;
#[cfg(feature = "visitor")]
use std::borrow::Cow;
use tl::{NodeHandle, Parser};

// Type aliases for Context and DomContext to avoid circular imports
type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Handles unknown/unspecified HTML elements.
///
/// For elements without specific handlers, this function:
/// 1. Records the output position before processing children
/// 2. Processes all children recursively
/// 3. Checks if any content was added
/// 4. Validates UTF-8 character boundaries for safe string slicing
/// 5. Detects if added content is a code block (indented or fenced)
/// 6. Removes empty content while preserving code blocks and spacing
///
/// # UTF-8 Safety
/// The function carefully handles UTF-8 boundaries to prevent panic when
/// slicing the output string. It finds the nearest valid boundary if the
/// position falls in the middle of a multi-byte character.
///
/// # Code Block Detection
/// Code blocks (identified by markdown formatting) are always preserved,
/// even if they appear "empty" according to trim().
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle(
    node_handle: &NodeHandle,
    parser: &Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    use crate::converter::walk_node;

    let Some(node) = node_handle.get(parser) else { return };

    let tag = match node {
        tl::Node::Tag(tag) => tag,
        _ => return,
    };

    #[cfg(feature = "visitor")]
    if let Some(ref visitor_handle) = ctx.visitor {
        use crate::converter::utility::serialization::serialize_tag_to_html;
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        let tag_name = tag.name().as_utf8_str().to_string();
        let raw_html = serialize_tag_to_html(node_handle, parser);
        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);
        let node_ctx = NodeContext::with_lazy_attributes(
            NodeType::Custom,
            Cow::Owned(tag_name.clone()),
            tag,
            depth,
            index_in_parent,
            parent_tag.map(Cow::Borrowed),
            false,
        );
        let visit_result = {
            let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
            visitor.visit_custom_element(&node_ctx, &tag_name, &raw_html)
        };
        match visit_result {
            VisitResult::Continue => {}
            VisitResult::Skip => return,
            VisitResult::Custom(custom) => {
                output.push_str(&custom);
                return;
            }
            VisitResult::PreserveHtml => {
                output.push_str(&raw_html);
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

    let len_before = output.len();
    let had_trailing_space = output.ends_with(' ');

    let children = tag.children();
    {
        for child_handle in children.top().iter() {
            walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
        }
    }

    let len_after = output.len();
    if len_after > len_before {
        let start_idx = if output.is_char_boundary(len_before) {
            len_before
        } else {
            // Find the nearest valid UTF-8 character boundary
            let capped = len_before.min(output.len());
            output
                .char_indices()
                .map(|(idx, _)| idx)
                .take_while(|idx| *idx <= capped)
                .last()
                .unwrap_or(capped)
        };

        let added_content = output[start_idx..].to_string();

        // Detect code blocks by markdown formatting
        let is_code_block =
            added_content.starts_with("    ") || added_content.starts_with("```") || added_content.starts_with("~~~");

        // Remove empty content while preserving code blocks
        if added_content.trim().is_empty() && !is_code_block {
            output.truncate(start_idx);
            if !had_trailing_space && added_content.contains(' ') {
                output.push(' ');
            }
        }
    }
}
