//! Visitor callback hooks for custom HTML traversal during conversion.
//!
//! This module contains the visitor pattern implementation hooks that are called
//! before and after element processing during the HTML to Markdown conversion tree walk.
//! These hooks enable custom processing, analysis, or modification of elements during conversion.

use std::borrow::Cow;

use crate::converter::utility::content::is_block_level_element;
use crate::visitor::{NodeContext, NodeType, VisitResult};

/// State captured at element_start that is reused at element_end.
///
/// Holds the parent tag, sibling index, and inline classification so the
/// matching `handle_visitor_element_end` call does not re-walk the DOM.
/// Attributes are NOT collected here — they are built lazily inside
/// `NodeContext` when (and only when) a visitor reads `ctx.attributes()`.
#[derive(Debug)]
pub struct VisitorElementState {
    parent_tag: Option<String>,
    index_in_parent: usize,
    is_inline: bool,
}

impl VisitorElementState {
    fn build_node_ctx<'a>(&'a self, tag_name: &'a str, tag: &'a tl::HTMLTag<'a>, depth: usize) -> NodeContext<'a> {
        NodeContext::with_lazy_attributes(
            NodeType::Element,
            Cow::Borrowed(tag_name),
            tag,
            depth,
            self.index_in_parent,
            self.parent_tag.as_deref().map(Cow::Borrowed),
            self.is_inline,
        )
    }
}

/// Handles visitor callback for element start (before processing).
///
/// Returns the action to take **and** the state needed to call
/// `handle_visitor_element_end` without recomputing attributes / parent.
/// When the action is `Skip`, `Custom`, or `Error` the state is `None`
/// because no matching end callback will fire.
pub fn handle_visitor_element_start(
    visitor_handle: &crate::visitor::VisitorHandle,
    tag_name: &str,
    node_handle: &tl::NodeHandle,
    tag: &tl::HTMLTag,
    parser: &tl::Parser<'_>,
    output: &mut String,
    _ctx: &crate::converter::Context,
    depth: usize,
    dom_ctx: &crate::converter::DomContext,
) -> (VisitAction, Option<VisitorElementState>) {
    let state = VisitorElementState {
        parent_tag: dom_ctx
            .parent_tag_name(node_handle.get_inner(), parser)
            .map(str::to_owned),
        index_in_parent: dom_ctx.get_sibling_index(node_handle.get_inner()).unwrap_or(0),
        is_inline: !is_block_level_element(tag_name),
    };

    let node_ctx = state.build_node_ctx(tag_name, tag, depth);

    let visitor_start_result = {
        let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
        visitor.visit_element_start(&node_ctx)
    };

    match visitor_start_result {
        crate::visitor::VisitResult::Continue => (VisitAction::Continue, Some(state)),
        crate::visitor::VisitResult::Skip => (VisitAction::Skip, None),
        crate::visitor::VisitResult::Custom(custom_output) => {
            output.push_str(&custom_output);

            // For custom output, still call visit_element_end (except for tables)
            if !matches!(tag_name, "table") {
                let element_content = &custom_output;
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
                let _ = visitor.visit_element_end(&node_ctx, element_content);
            }

            (VisitAction::Custom, None)
        }
        crate::visitor::VisitResult::Error(_msg) => (VisitAction::Error, None),
        crate::visitor::VisitResult::PreserveHtml => (VisitAction::Continue, Some(state)),
    }
}

/// Handles visitor callback for element end (after processing).
///
/// Reuses the [`VisitorElementState`] captured at element_start so the
/// parent tag and sibling index are computed exactly once per element.
/// Attributes are built lazily inside `NodeContext` on first access.
pub fn handle_visitor_element_end(
    visitor_handle: &crate::visitor::VisitorHandle,
    tag_name: &str,
    state: &VisitorElementState,
    tag: &tl::HTMLTag,
    output: &mut String,
    element_output_start: usize,
    ctx: &crate::converter::Context,
    depth: usize,
) {
    // Skip visitor callback for table elements
    if matches!(tag_name, "table") {
        return;
    }

    let node_ctx = state.build_node_ctx(tag_name, tag, depth);

    // The saved `element_output_start` can become stale in two ways:
    // 1. A child visitor returning Custom/Skip truncates `output`, making the
    //    saved position point past the end of the now-shorter string.
    // 2. Element handlers (e.g. div) trim trailing whitespace that was present
    //    when the position was captured, then append new multi-byte content.
    //    The old position can land inside a multi-byte character.
    // Clamp to output length, then retreat to a valid char boundary.
    let safe_start = element_output_start.min(output.len());
    let safe_start = crate::converter::utility::content::floor_char_boundary(output, safe_start);
    let element_content = &output[safe_start..];

    let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
    match visitor.visit_element_end(&node_ctx, element_content) {
        VisitResult::Continue => {}
        VisitResult::Custom(custom) => {
            output.truncate(safe_start);
            output.push_str(&custom);
        }
        VisitResult::Skip => {
            output.truncate(safe_start);
        }
        VisitResult::Error(err) => {
            if ctx.visitor_error.borrow().is_none() {
                *ctx.visitor_error.borrow_mut() = Some(err);
            }
        }
        VisitResult::PreserveHtml => {}
    }
}

/// Result of visitor element start callback indicating what should happen next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitAction {
    /// Continue with normal element processing
    Continue,
    /// Skip the element entirely (don't process children or call visit_element_end)
    Skip,
    /// Custom output was provided, skip normal processing
    Custom,
    /// Error occurred during visitor callback
    Error,
}
