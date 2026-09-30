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

    let is_table_continuation = continues_table_cell(output, ctx);

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
    // ~keep An empty div in a list item leaves the item as it found it (issue #583): its list
    // ~keep separator trims the line end, so the trimmed tail is kept to put back.
    let kept = ctx.in_list_item.then(|| {
        let kept_len = output.trim_end_matches([' ', '\t']).len();
        (kept_len, output[kept_len..].to_string())
    });

    // ~keep A div that opens at the start of another block's first text passes that start on.
    let opens_in_paragraph = (!ctx.in_table_cell
        && crate::converter::utility::escaping::ends_in_paragraph_text(output))
        || ctx.paragraph_start_at == Some((std::ptr::from_ref::<String>(output) as usize, output.len()));
    let is_list_continuation = start_block(output, ctx, options);

    // ~keep Measured the same way `block/paragraph.rs` does, so a text node can tell "at the
    // ~keep start of this div's line, in this div's buffer" from an inline wrapper's empty
    // ~keep scratch buffer. Without it a whitespace-only `<span>` opening a `<div>` was pushed
    // ~keep verbatim and `<span>    </span><img>` became an indented code block (issue #501).
    let children_start = output.len();
    let div_ctx = Context {
        block_content_start: output.len(),
        block_output_ptr: std::ptr::from_ref::<String>(output) as usize,
        paragraph_start_at: opens_in_paragraph.then(|| (std::ptr::from_ref::<String>(output) as usize, output.len())),
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
        strip_trailing_backslash_breaks(output, content_start_pos.min(children_start));
    }

    // ~keep In a list item the separator can take back the hard break the div follows, so the
    // ~keep output can end before the position measured on entry although the children wrote
    // ~keep content. Such a div looked empty and wrote no line end before the text after it.
    // ~keep The trailing-break strip above starts at the children for the same reason.
    let children_wrote = output.len() > children_start;
    if let Some((kept_len, kept_tail)) = kept.filter(|_| output.len() == children_start) {
        output.truncate(kept_len);
        output.push_str(&kept_tail);
    }
    let has_content = children_wrote || output.len() > content_start_pos;

    if has_content {
        if content_start_pos == 0 && output.starts_with('\n') && !output.starts_with("\n\n") {
            output.remove(0);
        }
        trim_trailing_whitespace(output);

        if ctx.in_table_cell {
            // ~keep Inline content after the div gets the cell break a block after it would get,
            // ~keep so its first word does not join the div's last one.
            if crate::converter::utility::siblings::following_sibling_content(node_handle.get_inner(), parser, dom_ctx)
                == crate::converter::utility::siblings::FollowingContent::Inline
            {
                emit_table_cell_break(output, options.br_in_tables);
            }
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

/// Whether the converter writes the element `name` with the `<div>` handler.
pub fn writes_like_div(name: &str) -> bool {
    matches!(name, "div" | "address" | "search" | "hgroup" | "center" | "dialog")
}

/// Whether a block written into a table cell's buffer continues content already in the cell.
fn continues_table_cell(output: &str, ctx: &Context) -> bool {
    (ctx.in_table_cell || ctx.in_layout_cell)
        && !output.is_empty()
        && !output.ends_with('|')
        && !output.ends_with("<br>")
        // ~keep A layout cell, unlike a real one, may already hold a newline: its row renders as a
        // ~keep list item, not a pipe row. Separating there opens the next line with a stray space.
        // ~keep Inert for `in_table_cell`, whose buffer never holds a newline by construction.
        && !output.ends_with('\n')
}

/// Separate a block from the content before it, as a `<div>` does: a cell break in a table cell,
/// the item's content column in a list item, and a blank line elsewhere. Returns whether the
/// block continues a list item after its content.
///
/// ~keep A plain suffix check like `output.ends_with("* ")` also matches the closing
/// ~keep "**"/"*" of `<strong>`/`<em>` immediately followed by a migrated trailing
/// ~keep space, indistinguishable from a real bare bullet by suffix alone -- and,
/// ~keep being hardcoded to `-`/`*`, never matched the third bullet `+` at all. The
/// ~keep false positive misclassified this div as sitting right after the marker,
/// ~keep which skips BOTH branches below (neither `is_list_continuation` nor
/// ~keep `needs_leading_sep` fires), so the div's content got glued directly onto the
/// ~keep preceding inline text with no separator at all. See
/// ~keep `list::utils::line_is_bare_list_marker`'s doc comment for the full rationale.
pub fn start_block(output: &mut String, ctx: &Context, options: &ConversionOptions) -> bool {
    let is_list_continuation =
        ctx.in_list_item && !output.is_empty() && !crate::converter::list::utils::line_is_bare_list_marker(output);
    let needs_leading_sep = !ctx.in_table_cell
        && !ctx.in_list_item
        && !ctx.convert_as_inline
        && !output.is_empty()
        && !output.ends_with("\n\n");
    if continues_table_cell(output, ctx) {
        emit_table_cell_break(output, options.br_in_tables);
    } else if is_list_continuation {
        crate::converter::list::utils::start_block_in_list_item(output, ctx, options);
    } else if needs_leading_sep {
        trim_trailing_whitespace(output);
        output.push_str("\n\n");
    }
    is_list_continuation
}
