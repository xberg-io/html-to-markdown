//! Blockquote element handler for HTML to Markdown conversion.
//!
//! Handles `<blockquote>` elements including:
//! - Basic blockquote markdown output with `> ` prefix
//! - Nested blockquotes
//! - Citation URLs via `cite` attribute
//! - Visitor callback integration

use crate::converter::Context;
use crate::converter::dom_context::DomContext;
use crate::converter::main::walk_node;
use crate::converter::main_helpers::strip_trailing_backslash_breaks_from_fresh_buffer;
use crate::options::ConversionOptions;

#[cfg(feature = "visitor")]
use crate::converter::utility::serialization::serialize_node_to_html;
#[cfg(feature = "visitor")]
use std::borrow::Cow;

/// Handle a `<blockquote>` element and convert to Markdown.
///
/// This handler processes blockquote elements including:
/// - Converting inline blockquotes by processing children as inline
/// - Handling nested blockquotes via `blockquote_depth` tracking
/// - Processing citation URLs from cite attribute
/// - Invoking visitor callbacks when the visitor feature is enabled
/// - Adding proper spacing and blockquote prefix formatting
#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle_blockquote(
    node_handle: &tl::NodeHandle,
    tag: &tl::HTMLTag,
    parser: &tl::Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    if ctx.convert_as_inline {
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
            }
        }
        return;
    }

    // ~keep Resolved like every other destination the converter renders: a relative `cite`
    // is useless once the markdown leaves the page it came from. `resolve_url` returns
    // `None` for an already-absolute or unresolvable reference, which then passes through.
    let cite = crate::converter::utility::attributes::decoded_attribute(tag, "cite")
        .map(std::borrow::Cow::into_owned)
        .map(|value| ctx.resolve_url(&value).unwrap_or(value));

    // ~keep The quote writes the indent of the list items around it on each of its lines, so its
    // ~keep children start at column 0 of a container of their own, outside the item: a list in
    // ~keep the quote counts only its own markers, and every line of an item in the quote gets
    // ~keep the same column (issue #654). Bold or italic around the item holding the quote does
    // ~keep not make a list in the quote text: its items open. Under a caption's or summary's
    // ~keep markers, a list in the quote is still judged by where its markers fall, as outside it.
    // ~keep A quote right after an opening inline marker starts on that marker's line, so its
    // ~keep first line is text between the markers.
    let first_line_follows_markers = output.is_empty() && ctx.in_marker_text();
    let blockquote_ctx = Context {
        blockquote_depth: ctx.blockquote_depth + 1,
        in_list_item: false,
        in_list: false,
        list_indent_columns: 0,
        real_item_columns: 0,
        inline_depth: if first_line_follows_markers {
            ctx.inline_depth
        } else {
            0
        },
        quote_starts_after_markers: first_line_follows_markers,
        item_lines: crate::converter::list::utils::ItemLineScan::new_item(),
        ..ctx.clone()
    };

    let mut content = String::with_capacity(256);
    let children = tag.children();
    {
        for child_handle in children.top().iter() {
            walk_node(
                child_handle,
                parser,
                &mut content,
                options,
                &blockquote_ctx,
                depth + 1,
                dom_ctx,
            );
        }
    }

    // ~keep A trailing <br> run with no following sibling has no next dispatch to catch it
    // in `walk_node`'s pre-block-dispatch strip, since the blockquote's content is
    // simply finished here — so this closes its own trailing run the same way
    // `paragraph.rs` closes its own (issue #464 follow-up).
    strip_trailing_backslash_breaks_from_fresh_buffer(&mut content, options.newline_style);

    let trimmed_content = content.trim();

    #[cfg(feature = "visitor")]
    if let Some(ref visitor) = ctx.visitor {
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

        let node_ctx = NodeContext::with_lazy_attributes(
            NodeType::Blockquote,
            Cow::Borrowed("blockquote"),
            tag,
            depth,
            index_in_parent,
            parent_tag.map(Cow::Borrowed),
            false,
        );

        let mut visitor_ref = visitor.lock().expect("visitor mutex poisoned");
        match visitor_ref.visit_blockquote(&node_ctx, trimmed_content, ctx.blockquote_depth) {
            VisitResult::Continue => {}
            VisitResult::Custom(custom) => {
                output.push_str(&custom);
                return;
            }
            VisitResult::Skip => return,
            VisitResult::PreserveHtml => {
                let mut html_output = String::new();
                serialize_node_to_html(node_handle, parser, &mut html_output);
                output.push_str(&html_output);
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

    if !trimmed_content.is_empty() {
        let list_indent = if ctx.in_list_item {
            crate::converter::list::utils::continuation_indent_string(
                crate::converter::list::utils::block_columns(ctx, options),
                options,
            )
        } else {
            None
        };

        // ~keep A blockquote that continues already-started list item content needs its
        // first quoted line indented too; one that is the item's first content
        // instead sits right after the marker, which already provides that column
        // (see `block/paragraph.rs`'s `is_list_continuation` for the identical
        // first-line distinction, applied there to paragraphs only).
        // A plain suffix check like `output.ends_with("* ")` also matches the closing
        // "**"/"*" of `<strong>`/`<em>` immediately followed by a migrated trailing
        // space (e.g. `<strong>bold</strong> <blockquote>` leaves output ending in
        // "**bold** "), which is indistinguishable from a real bare bullet by suffix
        // alone. That false positive misclassified this blockquote as sitting right
        // after the marker (skipping the continuation indent) when real inline
        // content actually preceded it, leaving the quoted line unindented and
        // dropping it (and the rest of the list) out of the item on reparse. See
        // `list::utils::line_is_bare_list_marker`'s doc comment for the full
        // rationale; it decomposes the WHOLE line instead of checking a fixed suffix.
        let is_list_continuation = list_indent.is_some()
            && !output.is_empty()
            && !crate::converter::list::utils::line_is_bare_list_marker(output);

        // ~keep The quote is the item's first content: it starts on the marker line. A line
        // ~keep break after the marker left the item empty and the quote outside it, also in a
        // ~keep quote that holds the list (issue #617).
        let at_bare_marker =
            ctx.in_list_item && crate::converter::list::utils::trim_whitespace_after_bare_marker(output);
        if at_bare_marker {
            // ~keep Nothing to separate: the marker line is the quote's first line.
        } else if ctx.blockquote_depth > 0 && !ctx.in_list_item {
            if !output.is_empty() {
                while output.ends_with('\n') {
                    output.truncate(output.len() - 1);
                }
                output.push_str("\n\n");
            }
        } else if !output.is_empty() {
            // ~keep The quote writes its own list indent below, so the one `walk_node` put at
            // ~keep the start of this line inside a list item goes first.
            if ctx.in_list_item {
                crate::converter::trim_trailing_whitespace(output);
            }
            if output.ends_with("\n\n") {
                output.truncate(output.len() - 1);
            } else if ctx.in_list_item {
                // ~keep A blockquote directly following this item's own leading text (no
                // explicit <p>, e.g. `<li>a<blockquote>`, which the preceding text
                // handler ends with a single '\n' since it looks ahead to the next
                // block-level sibling) still legally interrupts that text per
                // CommonMark's "blockquote can interrupt a paragraph" rule -- no blank
                // line is required for the reparse to recover the same two-block split.
                // Forcing one here anyway (as the two branches below still do for the
                // top-level, non-list case, and for `output` already ending in a full
                // blank line) instead makes THIS specific text parse back as its own
                // `<p>` on reparse, which flips the whole list loose and desyncs the
                // next conversion pass from this one (spec examples 320, 321).
                if !output.ends_with('\n') {
                    output.push('\n');
                }
            } else if !output.ends_with('\n') {
                output.push_str("\n\n");
            } else if !output.ends_with("\n\n") {
                output.push('\n');
            }
        }

        let prefix = "> ";

        // ~keep Only blank-out whitespace-only lines; preserve leading whitespace on
        // real content lines (code block indentation, nested list markers) so
        // quoted block children keep their structural meaning (issue #13).
        //
        // ~keep Every physical line also needs the list item's own continuation indent
        // when this blockquote is inside a list item — CommonMark's list container
        // match is per physical line, so an unindented "> " line drops the rest of
        // the quote (and the item) out of the list on re-parse (spec example 263).
        for (index, line) in trimmed_content.lines().enumerate() {
            if let Some(ref indent) = list_indent {
                if index > 0 || is_list_continuation {
                    output.push_str(indent);
                }
            }
            output.push_str(prefix);
            if !line.trim().is_empty() {
                output.push_str(line);
            }
            output.push('\n');
        }

        if let Some(url) = cite {
            output.push('\n');
            if let Some(ref indent) = list_indent {
                output.push_str(indent);
            }
            output.push_str("— <");
            output.push_str(&url);
            output.push_str(">\n\n");
        }

        // ~keep Add trailing newlines only when appropriate for proper spacing
        // (matching paragraph conditional logic for CommonMark compliance)
        if !ctx.convert_as_inline && !ctx.in_table_cell && !ctx.in_list_item {
            while output.ends_with('\n') {
                output.truncate(output.len() - 1);
            }
            output.push_str("\n\n");
        }
    }
}
