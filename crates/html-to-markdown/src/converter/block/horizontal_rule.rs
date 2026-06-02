//! Handler for horizontal rule elements (hr).
//!
//! Converts HTML horizontal rule tags to Markdown horizontal rules (---)
//! with appropriate spacing handling based on context.

use crate::converter::main_helpers::trim_trailing_whitespace;
use crate::converter::utility::siblings::get_previous_sibling_tag;
use std::borrow::Cow;
use tl::{NodeHandle, Parser};

// Type aliases for Context and DomContext to avoid circular imports
type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Handle horizontal rule elements (hr).
///
/// Converts to Markdown horizontal rule (---) with appropriate blank line
/// spacing based on context and previous siblings.
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle(
    node_handle: &NodeHandle,
    parser: &Parser,
    output: &mut String,
    _options: &crate::options::ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    #[cfg(feature = "visitor")]
    if let Some(ref visitor_handle) = ctx.visitor {
        use crate::converter::utility::content::collect_tag_attributes;
        use crate::visitor::{NodeContext, NodeType, VisitResult};
        use std::collections::BTreeMap;

        let tag = match node_handle.get(parser) {
            Some(tl::Node::Tag(t)) => t,
            _ => return,
        };
        let attributes: BTreeMap<String, String> = collect_tag_attributes(tag);
        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);
        let node_ctx = NodeContext {
            node_type: NodeType::Hr,
            tag_name: Cow::Borrowed("hr"),
            attributes: Cow::Owned(attributes),
            depth,
            index_in_parent,
            parent_tag: parent_tag.map(Cow::Owned),
            is_inline: false,
        };
        let visit_result = {
            let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
            visitor.visit_horizontal_rule(&node_ctx)
        };
        match visit_result {
            VisitResult::Continue => {}
            VisitResult::Skip => return,
            VisitResult::Custom(custom) => {
                output.push_str(&custom);
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

    if !output.is_empty() {
        let prev_tag = get_previous_sibling_tag(node_handle, parser, dom_ctx);
        let last_line_is_blockquote = output
            .rsplit('\n')
            .find(|line| !line.trim().is_empty())
            .is_some_and(|line| line.trim_start().starts_with('>'));
        let needs_blank_line = !ctx.in_paragraph && !matches!(prev_tag, Some("blockquote")) && !last_line_is_blockquote;

        // If previous element was a blockquote, it added \n\n; reduce to \n
        if matches!(prev_tag, Some("blockquote")) && output.ends_with("\n\n") {
            output.truncate(output.len() - 1);
        } else if ctx.in_paragraph || !needs_blank_line {
            if !output.ends_with('\n') {
                output.push('\n');
            }
        } else {
            trim_trailing_whitespace(output);
            if output.ends_with('\n') {
                if !output.ends_with("\n\n") {
                    output.push('\n');
                }
            } else {
                output.push_str("\n\n");
            }
        }
    }
    output.push_str("---\n");
}
