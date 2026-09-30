//! Handlers for HTML5 definition list and heading group elements.
//!
//! Processes list and heading semantic elements:
//! - `<hgroup>` - Groups related headings together
//! - `<dl>` - Definition list container
//! - `<dt>` - Definition term
//! - `<dd>` - Definition description
//!
//! These elements have special formatting requirements for proper Markdown output.

use super::walk_node;

/// Handles the `<hgroup>` element.
///
/// An hgroup element groups related headings together (e.g., a title and subtitle).
/// In Markdown, we simply process all children sequentially, allowing nested
/// headings to maintain their individual formatting.
///
/// # Behavior
///
/// - Children are processed sequentially in the current context
/// - No special formatting is applied at the hgroup level
pub fn handle_hgroup(
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
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
            }
        }
    }
}

/// Handles the `<dl>` element.
///
/// A definition list contains terms and their definitions. Terms and definitions
/// are output as plain blocks without Pandoc-style colon syntax, since standard
/// Markdown and GFM do not support definition lists.
///
/// # Behavior
///
/// - **Inline mode**: Children are processed inline without block spacing
/// - **Block mode**: Content is collected and wrapped with proper spacing
pub fn handle_dl(
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
        if ctx.convert_as_inline {
            let children = tag.children();
            {
                for child_handle in children.top().iter() {
                    walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
                }
            }
            return;
        }

        let mut content = String::new();
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, &mut content, options, ctx, depth + 1, dom_ctx);
            }
        }

        // ~keep A trailing <br> run with no following sibling has no next dispatch to catch
        // ~keep it in `walk_node`'s pre-block-dispatch strip, since the dl's content is
        // ~keep simply finished here — so this closes its own trailing run the same way
        // ~keep `paragraph.rs` closes its own (issue #464 follow-up).
        crate::converter::main_helpers::strip_trailing_backslash_breaks_from_fresh_buffer(
            &mut content,
            options.newline_style,
        );

        let trimmed = content.trim();
        if !trimmed.is_empty() {
            if !output.is_empty() && !output.ends_with("\n\n") {
                output.push_str("\n\n");
            }
            output.push_str(trimmed);
            output.push_str("\n\n");
        }
    }
}

/// Handles the `<dt>` element.
///
/// A dt element contains a term being defined. Terms are output on their own line,
/// with definitions following on subsequent lines.
///
/// # Behavior
///
/// - **Inline mode**: Content is output as-is
/// - **Block mode**: Content is followed by a newline
pub fn handle_dt(
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
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, &mut content, options, ctx, depth + 1, dom_ctx);
            }
        }

        // ~keep A trailing <br> run with no following sibling has no next dispatch to catch
        // ~keep it in `walk_node`'s pre-block-dispatch strip, since the dt's content is
        // ~keep simply finished here — so this closes its own trailing run the same way
        // ~keep `paragraph.rs` closes its own (issue #464 follow-up).
        crate::converter::main_helpers::strip_trailing_backslash_breaks_from_fresh_buffer(
            &mut content,
            options.newline_style,
        );

        let trimmed = content.trim();
        if !trimmed.is_empty() {
            if ctx.convert_as_inline {
                output.push_str(trimmed);
            } else {
                output.push_str(trimmed);
                output.push('\n');
            }
        }
    }
}

/// Handles the `<dd>` element.
///
/// A dd element contains the definition for a term. It is output as a plain
/// block since standard Markdown and GFM do not support definition list syntax.
///
/// # Behavior
///
/// - **Inline mode**: Content is output as-is
/// - **Block mode**: Content is output as a block
pub fn handle_dd(
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
        let mut content = String::with_capacity(128);
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, &mut content, options, ctx, depth + 1, dom_ctx);
            }
        }

        // ~keep A trailing <br> run with no following sibling has no next dispatch to catch
        // ~keep it in `walk_node`'s pre-block-dispatch strip, since the dd's content is
        // ~keep simply finished here — so this closes its own trailing run the same way
        // ~keep `paragraph.rs` closes its own (issue #464 follow-up).
        crate::converter::main_helpers::strip_trailing_backslash_breaks_from_fresh_buffer(
            &mut content,
            options.newline_style,
        );

        let trimmed = content.trim();

        if ctx.convert_as_inline {
            if !trimmed.is_empty() {
                output.push_str(trimmed);
            }
        } else if !trimmed.is_empty() {
            output.push_str(trimmed);
            output.push_str("\n\n");
        }
    }
}

/// Dispatcher for definition list and related elements.
///
/// Routes `<hgroup>`, `<dl>`, `<dt>` and `<dd>` elements
/// to their respective handlers.
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
        "hgroup" => handle_hgroup(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx),
        "dl" => handle_dl(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx),
        "dt" => handle_dt(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx),
        "dd" => handle_dd(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx),
        _ => {}
    }
}
