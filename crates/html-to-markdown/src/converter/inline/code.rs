//! Handler for the code-like inline elements `<kbd>` and `<samp>`.
//!
//! Converts them to Markdown inline code formatting with support for:
//! - Keyboard input (<kbd>) rendered as code
//! - Sample output (<samp>) rendered as code
//! - Nested code context tracking (suppress formatting inside <code>/<pre>)
//! - Whitespace normalization for kbd/samp elements
//!
//! `<code>` itself is handled by `converter::handlers::code_block::handle_code`, which is
//! what `converter::main`'s walk dispatches to; this module's own `<code>` arm was a second,
//! silently divergent implementation that nothing ever reached. ~keep

use crate::converter::inline::wrapped::emit_code_span;
use crate::options::ConversionOptions;
use crate::text;
use tl::{NodeHandle, Parser};

type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Handler for code-related inline elements: code, kbd (keyboard), and samp (sample output).
///
/// Processes code content based on context:
/// - For <code> within <code>: passes content through without wrapping backticks (nested code detection)
/// - For <kbd> and <samp>: normalizes whitespace and wraps with backticks
/// - For standalone <code>: applies smart backtick escaping and delimiter spacing
/// - Handles visitor callbacks for custom behavior when feature is enabled
/// - Properly escapes backticks in content that contains them
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
        "kbd" | "samp" => {
            handle_kbd_samp(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx);
        }
        _ => {}
    }
}

/// Handle keyboard and sample output elements (<kbd> and <samp> tags).
///
/// These elements are rendered as inline code with:
/// - Whitespace normalization (via `text::normalize_whitespace`)
/// - Chomp inline handling for prefix/suffix spacing
/// - Simple single backtick wrapping (no smart escaping for keyboard/sample)
fn handle_kbd_samp(
    _tag_name: &str,
    node_handle: &NodeHandle,
    parser: &Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    use crate::converter::{append_inline_suffix, chomp_inline, walk_node};

    let Some(node) = node_handle.get(parser) else { return };

    let tag = match node {
        tl::Node::Tag(tag) => tag,
        _ => return,
    };

    let children = tag.children();
    if ctx.in_code {
        // ~keep A nested `<code>` renders transparently inside an outer code span
        // ~keep (`handlers::code_block::handle_code`); `<kbd>`/`<samp>` wrapped their own
        // ~keep backticks anyway, so the outer span grew a second, nested pair.
        for child_handle in children.top().iter() {
            walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
        }
        return;
    }

    let mut content = String::with_capacity(32);
    let code_ctx = Context {
        in_code: true,
        ..ctx.clone()
    };
    for child_handle in children.top().iter() {
        walk_node(
            child_handle,
            parser,
            &mut content,
            options,
            &code_ctx,
            depth + 1,
            dom_ctx,
        );
    }

    let normalized = text::normalize_whitespace(&content);
    let (prefix, suffix, trimmed) = chomp_inline(&normalized);

    if content.is_empty() {
        return;
    }

    // ~keep issue #481: an all-whitespace body is not an empty one -- `<kbd> </kbd>` is a key
    // ~keep whose label is a space. Render the span over the normalized body rather than over
    // ~keep `trimmed`, which is empty exactly when the whole body was whitespace.
    let body = if trimmed.is_empty() {
        normalized.as_str()
    } else {
        trimmed
    };
    let emit_prefix = if trimmed.is_empty() { "" } else { prefix };
    let emit_suffix = if trimmed.is_empty() { "" } else { suffix };

    output.push_str(emit_prefix);
    let mut span = String::with_capacity(body.len() + 4);
    render_code_span(body, &mut span);
    if emit_prefix.is_empty() {
        emit_code_span(&span, body, output, node_handle, parser, dom_ctx);
    } else {
        output.push_str(&span);
    }
    append_inline_suffix(output, emit_suffix, !body.is_empty(), node_handle, parser, dom_ctx);
}

/// Render `body` as an inline code span.
///
/// No delimiter-space padding: `CommonMark` strips one space from each end of a code span only
/// when the content is not entirely spaces, so an all-spaces body round-trips as written. ~keep
fn render_code_span(body: &str, span: &mut String) {
    span.push('`');
    span.push_str(body);
    span.push('`');
}
