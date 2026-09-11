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

/// Push the wrapped-content emission shared by `handle_strong` and `handle_emphasis`'s
/// non-custom (post-visitor) path.
///
/// `open`/`close` are the exact delimiter strings this element wraps its trimmed content in
/// (pass `("", "")` for a `<strong>` nested inside another `<strong>`, which Tier-2 renders
/// with no marker of its own). `merge_symbol` is the single delimiter character `open` is
/// built from, used to detect and merge a `CommonMark`-adjacency with the immediately
/// preceding sibling's close marker (issue #483, see `merge_adjacent_emphasis`'s doc
/// comment) instead of opening a second, textually-adjacent delimiter run.
///
/// `sibling_tag_names` lists the HTML tag names (e.g. `["strong", "b"]`) that make this
/// element's immediately preceding DOM sibling a genuine candidate for that merge. Buffer
/// content alone is ambiguous: ordinary prose can coincidentally end in a literal `*`/`**`
/// (CommonMark spec example 442, `<p>*<em>foo</em></p>` -> `**foo*`, NOT a merge into
/// `*foo*`) that is indistinguishable, byte-for-byte, from a just-emitted close marker.
/// Requiring the DOM to actually show a matching sibling element resolves the ambiguity in
/// favor of "no merge" whenever the preceding content is plain text rather than a real
/// emphasis/strong element.
///
/// ~keep This one function is the single point of truth for both `handle_strong` and
/// ~keep `handle_emphasis`'s emission logic, across all four (visitor x strong/emphasis)
/// ~keep call sites that used to inline a byte-for-byte copy of it -- see the emit block
/// ~keep duplication this replaces.
#[allow(clippy::too_many_arguments)]
fn emit_wrapped_inline(
    output: &mut String,
    content: &str,
    open: &str,
    close: &str,
    merge_symbol: char,
    sibling_tag_names: &[&str],
    node_handle: &NodeHandle,
    parser: &Parser,
    dom_ctx: &DomContext,
) {
    use crate::converter::utility::siblings::get_previous_sibling_tag;
    use crate::converter::{append_inline_suffix, chomp_inline, merge_adjacent_emphasis};

    let (prefix, suffix, trimmed) = chomp_inline(content);
    if !content.trim().is_empty() {
        output.push_str(prefix);
        let sibling_is_matching_tag = get_previous_sibling_tag(node_handle, parser, dom_ctx)
            .is_some_and(|name| sibling_tag_names.contains(&name));
        let merged = prefix.is_empty()
            && sibling_is_matching_tag
            && merge_adjacent_emphasis(output, merge_symbol, open.chars().count());
        if !merged {
            output.push_str(open);
        }
        output.push_str(trimmed);
        output.push_str(close);
        append_inline_suffix(output, suffix, !trimmed.is_empty(), node_handle, parser, dom_ctx);
    } else if !content.is_empty() {
        // ~keep issue #481: a whitespace-only body (e.g. `<i> </i>`) must contribute at
        // ~keep most one space -- `chomp_inline` above already collapsed prefix/suffix to
        // ~keep a single representation, but the buffer can already end with a real space
        // ~keep from a preceding sibling (e.g. `A <i> </i>B`), in which case even that one
        // ~keep copy must be suppressed. Mirrors `text_node.rs`'s `!output.ends_with(' ')`
        // ~keep guards, which this handler was the one outlier missing.
        if !output.ends_with(' ') {
            output.push_str(prefix);
        }
        append_inline_suffix(output, suffix, false, node_handle, parser, dom_ctx);
    }
}

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
    const SIBLING_TAGS: [&str; 2] = ["strong", "b"];
    if ctx.in_strong {
        emit_wrapped_inline(
            output,
            content,
            "",
            "",
            options.strong_em_symbol,
            &SIBLING_TAGS,
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
            "*",
            "*",
            '*',
            &SIBLING_TAGS,
            node_handle,
            parser,
            dom_ctx,
        );
    } else {
        let marker: String = [options.strong_em_symbol; 2].iter().collect();
        emit_wrapped_inline(
            output,
            content,
            &marker,
            &marker,
            options.strong_em_symbol,
            &SIBLING_TAGS,
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
    const SIBLING_TAGS: [&str; 2] = ["em", "i"];
    if options.output_format == OutputFormat::Djot {
        // ~keep Djot emphasis always uses `_`, independent of `options.strong_em_symbol`
        // ~keep (pre-existing behaviour, unchanged by this refactor).
        emit_wrapped_inline(
            output,
            content,
            "_",
            "_",
            '_',
            &SIBLING_TAGS,
            node_handle,
            parser,
            dom_ctx,
        );
    } else {
        let marker = options.strong_em_symbol.to_string();
        emit_wrapped_inline(
            output,
            content,
            &marker,
            &marker,
            options.strong_em_symbol,
            &SIBLING_TAGS,
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
