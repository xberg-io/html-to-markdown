//! Handler for div element.
//!
//! Converts HTML div elements to Markdown by processing children while maintaining
//! appropriate spacing and context awareness for:
//! - Table continuations: Uses table-specific line breaks
//! - List continuations: Uses list indentation
//! - Block context: Adds surrounding newlines for proper block separation

use crate::converter::main_helpers::{
    emit_table_cell_break, strip_trailing_backslash_breaks, trim_trailing_whitespace,
};
use crate::options::{ConversionOptions, NewlineStyle};
use tl::{NodeHandle, Parser};

type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Handles div elements.
///
/// Divs are generic container elements that need special handling based on context:
/// - When inline context: passes through children without separators
/// - When in table cell: uses table-specific line breaks (<br> or backslash)
/// - When in list item: uses list continuation indentation
/// - When in block context: adds appropriate newlines before/after content
///
/// # Note
/// This function references `walk_node` and helper functions from converter.rs
/// which must be accessible (pub(crate)) for this module to work correctly.
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

    let is_table_continuation = (ctx.in_table_cell || ctx.in_layout_cell)
        && !output.is_empty()
        && !output.ends_with('|')
        && !output.ends_with("<br>")
        // ~keep A layout cell, unlike a real one, may already hold a newline: its row renders as a
        // ~keep list item, not a pipe row. Separating there opens the next line with a stray space.
        // ~keep Inert for `in_table_cell`, whose buffer never holds a newline by construction.
        && !output.ends_with('\n');

    if ctx.convert_as_inline {
        // ~keep A layout-table cell converts as inline but is still a cell, so its sibling
        // ~keep boundary follows the settled cell rule (issues #453/#454) instead of
        // ~keep disappearing, which is what glued adjacent <div>s together (issue #470).
        // ~keep `in_table_cell` never reaches this branch: a real cell does not set
        // ~keep `convert_as_inline`, so only `in_layout_cell` can make this fire.
        if is_table_continuation {
            emit_table_cell_break(output, options.br_in_tables);
        }
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
            }
        }
        return;
    }

    let content_start_pos = output.len();

    // ~keep A plain suffix check like `output.ends_with("* ")` also matches the closing
    // ~keep "**"/"*" of `<strong>`/`<em>` immediately followed by a migrated trailing
    // ~keep space, indistinguishable from a real bare bullet by suffix alone -- and,
    // ~keep being hardcoded to `-`/`*`, never matched the third bullet `+` at all. The
    // ~keep false positive misclassified this div as sitting right after the marker,
    // ~keep which skips BOTH branches below (neither `is_list_continuation` nor
    // ~keep `needs_leading_sep` fires), so the div's content got glued directly onto the
    // ~keep preceding inline text with no separator at all. See
    // ~keep `list::utils::line_is_bare_list_marker`'s doc comment for the full rationale.
    let is_list_continuation =
        ctx.in_list_item && !output.is_empty() && !crate::converter::list::utils::line_is_bare_list_marker(output);

    let needs_leading_sep = !ctx.in_table_cell
        && !ctx.in_list_item
        && !ctx.convert_as_inline
        && !output.is_empty()
        && !output.ends_with("\n\n");

    if is_table_continuation {
        emit_table_cell_break(output, options.br_in_tables);
    } else if is_list_continuation {
        add_list_continuation_indent(output, ctx.list_depth, false, options);
    } else if needs_leading_sep {
        trim_trailing_whitespace(output);
        output.push_str("\n\n");
    }

    // ~keep Measured the same way `block/paragraph.rs` does, so a text node can tell "at the
    // ~keep start of this div's line, in this div's buffer" from an inline wrapper's empty
    // ~keep scratch buffer. Without it a whitespace-only `<span>` opening a `<div>` was pushed
    // ~keep verbatim and `<span>    </span><img>` became an indented code block (issue #501).
    let div_ctx = Context {
        block_content_start: output.len(),
        block_output_ptr: std::ptr::from_ref::<String>(output) as usize,
        ..ctx.clone()
    };

    let children = tag.children();
    {
        for child_handle in children.top().iter() {
            walk_node(child_handle, parser, output, options, &div_ctx, depth + 1, dom_ctx);
        }
    }

    if options.newline_style == NewlineStyle::Backslash {
        // ~keep A trailing <br> run with no following sibling has no next dispatch to catch
        // ~keep it in `walk_node`'s pre-block-dispatch strip, since the div is simply
        // ~keep finishing here — so this closes its own trailing run the same way
        // ~keep `paragraph.rs` closes its own (issue #464 follow-up).
        strip_trailing_backslash_breaks(output, content_start_pos);
    }

    let has_content = output.len() > content_start_pos;

    if has_content {
        if content_start_pos == 0 && output.starts_with('\n') && !output.starts_with("\n\n") {
            output.remove(0);
        }
        trim_trailing_whitespace(output);

        if ctx.in_table_cell {
            // ~keep No trailing separator in table cells
        } else if ctx.in_list_item {
            if is_list_continuation {
                if !output.ends_with('\n') {
                    output.push('\n');
                }
            } else if !output.ends_with("\n\n") {
                if output.ends_with('\n') {
                    output.push('\n');
                } else {
                    output.push_str("\n\n");
                }
            }
        } else if !ctx.in_list_item && !ctx.convert_as_inline {
            if output.ends_with("\n\n") {
            } else if output.ends_with('\n') {
                output.push('\n');
            } else {
                output.push_str("\n\n");
            }
        }
    }
}

/// Helper function to add list continuation indentation
fn add_list_continuation_indent(
    output: &mut String,
    list_depth: usize,
    _block_level: bool,
    _options: &ConversionOptions,
) {
    if !output.ends_with('\n') {
        output.push('\n');
    }

    for _ in 0..list_depth {
        output.push_str("  ");
    }
}
