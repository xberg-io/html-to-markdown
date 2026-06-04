//! Handler for line break elements (br).
//!
//! Converts HTML line break tags to Markdown line breaks using the configured
//! newline style (spaces, backslash, or plain newline).

use crate::converter::main_helpers::trim_trailing_whitespace;
use crate::options::{ConversionOptions, NewlineStyle};
#[cfg(feature = "visitor")]
use std::borrow::Cow;
use tl::{NodeHandle, Parser};

// Type aliases for Context and DomContext to avoid circular imports
type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Handle line break elements (br).
///
/// Converts to appropriate Markdown line break syntax based on the configured
/// newline style and current context (e.g., in headings).
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
    #[cfg(feature = "visitor")]
    if let Some(ref visitor_handle) = ctx.visitor {
        use crate::visitor::EMPTY_ATTRS;
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);
        let node_ctx = if let Some(tl::Node::Tag(t)) = node_handle.get(parser) {
            NodeContext::with_lazy_attributes(
                NodeType::Br,
                Cow::Borrowed("br"),
                t,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                true,
            )
        } else {
            NodeContext::with_borrowed_attributes(
                NodeType::Br,
                Cow::Borrowed("br"),
                &EMPTY_ATTRS,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                true,
            )
        };
        let visit_result = {
            let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
            visitor.visit_line_break(&node_ctx)
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

    if ctx.in_heading {
        trim_trailing_whitespace(output);
        output.push_str("  ");
    } else {
        if output.is_empty() || output.ends_with('\n') {
            output.push('\n');
        } else {
            match options.newline_style {
                NewlineStyle::Spaces => output.push_str("  \n"),
                NewlineStyle::Backslash => output.push_str("\\\n"),
            }
        }
    }
}
