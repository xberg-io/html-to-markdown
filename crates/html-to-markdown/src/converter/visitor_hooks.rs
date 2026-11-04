//! Visitor callback hooks for custom HTML traversal during conversion.
//!
//! This module contains the visitor pattern implementation hooks that are called
//! before and after element processing during the HTML to Markdown conversion tree walk.
//! These hooks enable custom processing, analysis, or modification of elements during conversion.

use std::borrow::Cow;

use crate::converter::utility::content::is_block_level_element;
use crate::visitor::{NodeContext, NodeType, VisitResult};

/// State captured at `element_start` that is reused at `element_end`.
///
/// Holds the parent tag, sibling index, and inline classification so the
/// matching `handle_visitor_element_end` call does not re-walk the DOM.
/// Attributes are NOT collected here — they are built lazily inside
/// `NodeContext` when (and only when) a visitor reads `ctx.attributes()`.
///
/// `parent_tag` borrows from the parser's source string to avoid allocating
/// per element_start call.
#[derive(Debug)]
pub struct VisitorElementState<'p> {
    parent_tag: Option<&'p str>,
    index_in_parent: usize,
    is_inline: bool,
}

/// Arguments needed to complete an element's visitor callback.
pub struct VisitorElementEndContext<'a> {
    /// Buffer containing the element's converted output.
    pub(crate) output: &'a mut String,
    /// Offset at which this element's output began.
    pub(crate) element_output_start: usize,
    /// Conversion state used to record a visitor error.
    pub(crate) ctx: &'a crate::converter::Context,
    /// Current traversal depth.
    pub(crate) depth: usize,
}

impl<'p> VisitorElementState<'p> {
    fn build_node_ctx<'a>(
        &'a self,
        tag_name: &'a str,
        tag: &'a crate::dom::HTMLTag<'a>,
        depth: usize,
    ) -> NodeContext<'a>
    where
        'p: 'a,
    {
        NodeContext::with_lazy_attributes(
            NodeType::Element,
            Cow::Borrowed(tag_name),
            tag,
            depth,
            self.index_in_parent,
            self.parent_tag.map(Cow::Borrowed),
            self.is_inline,
        )
    }
}

/// Captures the state shared by an element's start and end visitor callbacks.
pub fn build_visitor_element_state<'p>(
    tag_name: &str,
    node_handle: &crate::dom::NodeHandle,
    parser: &'p crate::dom::Parser<'p>,
    dom_ctx: &'p crate::converter::DomContext,
) -> VisitorElementState<'p> {
    VisitorElementState {
        parent_tag: dom_ctx.parent_tag_name(node_handle.get_inner(), parser),
        index_in_parent: dom_ctx.get_sibling_index(node_handle.get_inner()).unwrap_or(0),
        is_inline: !is_block_level_element(tag_name),
    }
}

/// Handles visitor callback for element start (before processing).
pub fn handle_visitor_element_start(
    visitor_handle: &crate::visitor::VisitorHandle,
    tag_name: &str,
    tag: &crate::dom::HTMLTag,
    state: &VisitorElementState<'_>,
    output: &mut String,
    depth: usize,
) -> VisitAction {
    let node_ctx = state.build_node_ctx(tag_name, tag, depth);

    let visitor_start_result = {
        let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
        visitor.visit_element_start(&node_ctx)
    };

    match resolve_media_source_preservation(visitor_start_result, tag_name, tag, state) {
        crate::visitor::VisitResult::Continue => VisitAction::Continue,
        crate::visitor::VisitResult::Skip => VisitAction::Skip,
        crate::visitor::VisitResult::Custom(custom_output) => {
            output.push_str(&custom_output);

            if !matches!(tag_name, "table") {
                let element_content = &custom_output;
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
                let _ = visitor.visit_element_end(&node_ctx, element_content);
            }

            VisitAction::Custom
        }
        crate::visitor::VisitResult::Error(_msg) => VisitAction::Error,
        crate::visitor::VisitResult::PreserveHtml => VisitAction::Continue,
    }
}

/// Handles visitor callback for element end (after processing).
///
/// Reuses the [`VisitorElementState`] captured at `element_start` so the
/// parent tag and sibling index are computed exactly once per element.
/// Attributes are built lazily inside `NodeContext` on first access.
pub fn handle_visitor_element_end(
    visitor_handle: &crate::visitor::VisitorHandle,
    tag_name: &str,
    state: &VisitorElementState<'_>,
    tag: &crate::dom::HTMLTag,
    end: VisitorElementEndContext<'_>,
) -> VisitAction {
    if matches!(tag_name, "table") {
        return VisitAction::Continue;
    }

    let node_ctx = state.build_node_ctx(tag_name, tag, end.depth);

    let safe_start = end.element_output_start.min(end.output.len());
    let safe_start = crate::converter::utility::content::floor_char_boundary(end.output, safe_start);
    let element_content = &end.output[safe_start..];

    let result = visitor_handle
        .lock()
        .expect("visitor mutex poisoned")
        .visit_element_end(&node_ctx, element_content);
    match resolve_media_source_preservation(result, tag_name, tag, state) {
        VisitResult::Continue | VisitResult::PreserveHtml => VisitAction::Continue,
        VisitResult::Custom(custom) => {
            end.output.truncate(safe_start);
            end.output.push_str(&custom);
            VisitAction::Custom
        }
        VisitResult::Skip => {
            end.output.truncate(safe_start);
            VisitAction::Skip
        }
        VisitResult::Error(err) => {
            if end.ctx.visitor_error.borrow().is_none() {
                *end.ctx.visitor_error.borrow_mut() = Some(err);
            }
            VisitAction::Error
        }
    }
}

fn resolve_media_source_preservation(
    result: VisitResult,
    name: &str,
    tag: &crate::dom::HTMLTag<'_>,
    state: &VisitorElementState<'_>,
) -> VisitResult {
    if matches!(result, VisitResult::PreserveHtml)
        && name == "source"
        && matches!(state.parent_tag, Some("audio" | "video"))
    {
        let mut html = String::new();
        crate::converter::utility::serialization::serialize_open_tag(tag, name, &mut html);
        VisitResult::Custom(html)
    } else {
        result
    }
}

/// Result of visitor element start callback indicating what should happen next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitAction {
    /// Continue with normal element processing
    Continue,
    /// Skip the element entirely (don't process children or call `visit_element_end`)
    Skip,
    /// Custom output was provided, skip normal processing
    Custom,
    /// Error occurred during visitor callback
    Error,
}
