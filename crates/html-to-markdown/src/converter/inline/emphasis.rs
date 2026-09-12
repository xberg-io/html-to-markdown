//! Handler for emphasis elements (strong, b, em, i).
//!
//! Converts HTML emphasis tags to Markdown formatting with support for:
//! - Bold/strong formatting using configurable symbols (** or __)
//! - Italic/emphasis formatting using configurable symbols (* or _)
//! - Nested emphasis context tracking
//! - Code context handling (suppress formatting in <code>)
//! - Visitor callbacks for custom emphasis processing
//! - Bootstrap caret detection (.caret class)

use crate::options::{ConversionOptions, OutputFormat};
#[cfg(feature = "visitor")]
use std::borrow::Cow;
use tl::{NodeHandle, Parser};

type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Handler for emphasis elements: strong, b (bold) and em, i (italic).
///
/// Processes emphasis content based on context:
/// - Suppresses formatting when already in strong/code context
/// - Applies configurable emphasis symbols (* or _)
/// - Handles nested emphasis with proper context tracking
/// - Supports visitor callbacks for custom behavior
/// - Detects Bootstrap caret elements (.caret class)
///
/// # Note
/// This function references helper functions and `walk_node` from converter.rs
/// which must be accessible (pub(crate)) for this module to work correctly.
pub fn handle(
    tag_name: &str,
    node_handle: &NodeHandle,
    parser: &Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    match tag_name {
        "strong" | "b" => {
            handle_strong(node_handle, parser, output, options, ctx, depth, dom_ctx);
        }
        "em" | "i" => {
            handle_emphasis(node_handle, parser, output, options, ctx, depth, dom_ctx);
        }
        _ => {}
    }
}

use crate::converter::inline::wrapped::{
    EMPHASIS_SIBLING_TAGS, InlineDelimiters, STRONG_SIBLING_TAGS, emit_wrapped_inline,
};

/// Resolve `<strong>`/`<b>`'s wrapping delimiters for the current context and options, then
/// emit via [`emit_wrapped_inline`].
fn emit_strong_wrapped(
    output: &mut String,
    content: &str,
    options: &ConversionOptions,
    ctx: &Context,
    node_handle: &NodeHandle,
    parser: &Parser,
    dom_ctx: &DomContext,
) {
    if ctx.in_strong {
        emit_wrapped_inline(
            output,
            content,
            &InlineDelimiters {
                open: "",
                close: "",
                merge_symbol: Some(options.strong_em_symbol),
                sibling_tag_names: &STRONG_SIBLING_TAGS,
            },
            node_handle,
            parser,
            dom_ctx,
        );
    } else if options.output_format == OutputFormat::Djot {
        // ~keep Djot strong always uses `*`, independent of `options.strong_em_symbol`
        // ~keep (pre-existing behaviour, unchanged by this refactor).
        emit_wrapped_inline(
            output,
            content,
            &InlineDelimiters {
                open: "*",
                close: "*",
                merge_symbol: Some('*'),
                sibling_tag_names: &STRONG_SIBLING_TAGS,
            },
            node_handle,
            parser,
            dom_ctx,
        );
    } else {
        let marker: String = [options.strong_em_symbol; 2].iter().collect();
        emit_wrapped_inline(
            output,
            content,
            &InlineDelimiters {
                open: &marker,
                close: &marker,
                merge_symbol: Some(options.strong_em_symbol),
                sibling_tag_names: &STRONG_SIBLING_TAGS,
            },
            node_handle,
            parser,
            dom_ctx,
        );
    }
}

/// Resolve `<em>`/`<i>`'s wrapping delimiters for the current context and options, then emit
/// via [`emit_wrapped_inline`].
fn emit_emphasis_wrapped(
    output: &mut String,
    content: &str,
    options: &ConversionOptions,
    node_handle: &NodeHandle,
    parser: &Parser,
    dom_ctx: &DomContext,
) {
    if options.output_format == OutputFormat::Djot {
        // ~keep Djot emphasis always uses `_`, independent of `options.strong_em_symbol`
        // ~keep (pre-existing behaviour, unchanged by this refactor).
        emit_wrapped_inline(
            output,
            content,
            &InlineDelimiters {
                open: "_",
                close: "_",
                merge_symbol: Some('_'),
                sibling_tag_names: &EMPHASIS_SIBLING_TAGS,
            },
            node_handle,
            parser,
            dom_ctx,
        );
    } else {
        let marker = options.strong_em_symbol.to_string();
        emit_wrapped_inline(
            output,
            content,
            &InlineDelimiters {
                open: &marker,
                close: &marker,
                merge_symbol: Some(options.strong_em_symbol),
                sibling_tag_names: &EMPHASIS_SIBLING_TAGS,
            },
            node_handle,
            parser,
            dom_ctx,
        );
    }
}

/// Handle strong/bold emphasis (strong, b tags).
fn handle_strong(
    node_handle: &NodeHandle,
    parser: &Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    // ~keep reason: serialize_node is only used with the visitor feature; other imports depend
    // ~keep on feature-gated code paths in this function.
    #[allow(unused_imports)]
    use crate::converter::{get_text_content, serialize_node, walk_node};

    let Some(node) = node_handle.get(parser) else { return };

    let tag = match node {
        tl::Node::Tag(tag) => tag,
        _ => return,
    };

    if ctx.in_code {
        let children = tag.children();
        for child_handle in children.top().iter() {
            walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
        }
    } else {
        let mut content = String::with_capacity(64);
        let children = tag.children();
        {
            let strong_ctx = Context {
                inline_depth: ctx.inline_depth + 1,
                in_strong: true,
                ..ctx.clone()
            };
            for child_handle in children.top().iter() {
                walk_node(
                    child_handle,
                    parser,
                    &mut content,
                    options,
                    &strong_ctx,
                    depth + 1,
                    dom_ctx,
                );
            }
        }

        #[cfg(feature = "visitor")]
        let strong_output = if let Some(ref visitor_handle) = ctx.visitor {
            use crate::visitor::{NodeContext, NodeType, VisitResult};

            let text_content = get_text_content(node_handle, parser, dom_ctx);

            let node_id = node_handle.get_inner();
            let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
            let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

            let node_ctx = NodeContext::with_lazy_attributes(
                NodeType::Strong,
                tag.name().as_utf8_str(),
                tag,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                true,
            );

            let visit_result = {
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
                visitor.visit_strong(&node_ctx, &text_content)
            };
            match visit_result {
                VisitResult::Continue => None,
                VisitResult::Custom(custom) => Some(custom),
                VisitResult::Skip => Some(String::new()),
                VisitResult::PreserveHtml => Some(serialize_node(node_handle, parser)),
                VisitResult::Error(err) => {
                    if ctx.visitor_error.borrow().is_none() {
                        *ctx.visitor_error.borrow_mut() = Some(err);
                    }
                    None
                }
            }
        } else {
            None
        };

        #[cfg(feature = "visitor")]
        if let Some(custom_output) = strong_output {
            output.push_str(&custom_output);
        } else {
            emit_strong_wrapped(output, &content, options, ctx, node_handle, parser, dom_ctx);
        }

        #[cfg(not(feature = "visitor"))]
        emit_strong_wrapped(output, &content, options, ctx, node_handle, parser, dom_ctx);
    }
}

/// Handle emphasis/italic (em, i tags).
fn handle_emphasis(
    node_handle: &NodeHandle,
    parser: &Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    // ~keep reason: serialize_node is only used with the visitor feature; other imports depend
    // ~keep on feature-gated code paths in this function.
    #[allow(unused_imports)]
    use crate::converter::{get_text_content, serialize_node, walk_node};

    let Some(node) = node_handle.get(parser) else { return };

    let tag = match node {
        tl::Node::Tag(tag) => tag,
        _ => return,
    };

    if ctx.in_code {
        let children = tag.children();
        for child_handle in children.top().iter() {
            walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
        }
    } else {
        let mut content = String::with_capacity(64);
        let children = tag.children();
        {
            let em_ctx = Context {
                inline_depth: ctx.inline_depth + 1,
                ..ctx.clone()
            };
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, &mut content, options, &em_ctx, depth + 1, dom_ctx);
            }
        }

        #[cfg(feature = "visitor")]
        let em_output = if let Some(ref visitor_handle) = ctx.visitor {
            use crate::visitor::{NodeContext, NodeType, VisitResult};

            let text_content = get_text_content(node_handle, parser, dom_ctx);

            let node_id = node_handle.get_inner();
            let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
            let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

            let node_ctx = NodeContext::with_lazy_attributes(
                NodeType::Em,
                tag.name().as_utf8_str(),
                tag,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                true,
            );

            let visit_result = {
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
                visitor.visit_emphasis(&node_ctx, &text_content)
            };
            match visit_result {
                VisitResult::Continue => None,
                VisitResult::Custom(custom) => Some(custom),
                VisitResult::Skip => Some(String::new()),
                VisitResult::PreserveHtml => Some(serialize_node(node_handle, parser)),
                VisitResult::Error(err) => {
                    if ctx.visitor_error.borrow().is_none() {
                        *ctx.visitor_error.borrow_mut() = Some(err);
                    }
                    None
                }
            }
        } else {
            None
        };

        #[cfg(feature = "visitor")]
        if let Some(custom_output) = em_output {
            output.push_str(&custom_output);
        } else {
            emit_emphasis_wrapped(output, &content, options, node_handle, parser, dom_ctx);
            maybe_emit_caret(output, &content, tag);
        }

        #[cfg(not(feature = "visitor"))]
        {
            emit_emphasis_wrapped(output, &content, options, node_handle, parser, dom_ctx);
            maybe_emit_caret(output, &content, tag);
        }
    }
}

/// Detect a Bootstrap `.caret` marker (`<i class="caret"></i>` and similar) on a genuinely
/// empty (not merely whitespace-only) `<em>`/`<i>` body and render it as `" > "`.
///
/// Only reachable when `content` is empty: [`emit_wrapped_inline`] already handles the
/// non-empty and whitespace-only-but-non-empty cases and leaves `output` untouched otherwise.
fn maybe_emit_caret(output: &mut String, content: &str, tag: &tl::HTMLTag) {
    if !content.is_empty() {
        return;
    }
    if let Some(class_value) = tag
        .attributes()
        .get("class")
        .and_then(|v| v.as_ref().map(|val| val.as_utf8_str()))
    {
        if class_value.contains("caret") && !output.ends_with(' ') {
            output.push_str(" > ");
        }
    }
}
