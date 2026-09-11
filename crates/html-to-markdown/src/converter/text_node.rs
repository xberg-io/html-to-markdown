//! Text node processing for HTML to Markdown conversion.
//!
//! Handles raw text nodes with:
//! - HTML entity decoding
//! - Whitespace normalization and stripping
//! - Text escaping with configurable escape modes
//! - Visitor callbacks (when feature enabled)
//! - List item indentation

use std::borrow::Cow;

use crate::converter::dom_context::DomContext;
use crate::converter::main_helpers::{has_more_than_one_char, is_ascii_whitespace_only, is_inline_element};
use crate::converter::utility::siblings::{
    get_next_sibling_tag, next_sibling_is_inline_tag, previous_sibling_is_inline_tag,
};
use crate::options::ConversionOptions;
use crate::text;
#[cfg(feature = "visitor")]
use crate::visitor::EMPTY_ATTRS;

type Context = crate::converter::Context;

/// Process a raw text node during HTML to Markdown conversion.
///
/// Handles:
/// - HTML entity decoding
/// - Whitespace normalization and stripping
/// - Text escaping with configurable escape modes
/// - Visitor callbacks (when feature enabled)
/// - List item indentation
#[allow(clippy::too_many_lines)]
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn process_text_node(
    raw: &str,
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    let mut text = text::decode_html_entities_cow(raw);

    if text.is_empty() {
        return;
    }

    let text_ref = text.as_ref();
    let had_newlines = text_ref.contains('\n');
    let has_double_newline = text_ref.contains("\n\n") || text_ref.contains("\r\n\r\n");

    if options.strip_newlines && (text.contains('\r') || text.contains('\n')) {
        text = Cow::Owned(text.replace(['\r', '\n'], " "));
    }

    // ~keep Captured before any write below flips it: this is the one true "does real
    // ~keep content already precede this text node" signal (see `Context::at_fresh_block_start`).
    // ~keep Unlike comparing `output.len()` to `ctx.block_content_start`, it stays correct
    // ~keep even when `output` is a fresh local `String` an inline wrapper (sub/sup/em/...)
    // ~keep is building its content into, because it is shared via `Rc<Cell<bool>>` rather
    // ~keep than inferred from whichever buffer happens to be passed in.
    let was_fresh_block_start = ctx.at_fresh_block_start.get();

    if text.trim().is_empty() {
        if ctx.in_code {
            output.push_str(text.as_ref());
            return;
        }

        if options.whitespace_mode == crate::options::WhitespaceMode::Strict {
            if ctx.convert_as_inline || ctx.in_table_cell || ctx.in_list_item {
                output.push_str(text.as_ref());
                return;
            }
            if has_double_newline {
                if !output.ends_with("\n\n") {
                    output.push('\n');
                }
                return;
            }
            output.push_str(text.as_ref());
            return;
        }

        // ~keep The `block_output_ptr` address check guards against a false match on a
        // ~keep nested inline wrapper's (em/strong/link) fresh, empty SCRATCH buffer:
        // ~keep `walk_node` builds each such wrapper's content into its own local `String`
        // ~keep before splicing it into the real output (see `Context::block_output_ptr`'s
        // ~keep and `Context::at_fresh_block_start`'s doc comments for the general shape of
        // ~keep this hazard), so that buffer's length can coincidentally equal the ANCESTOR
        // ~keep paragraph's `block_content_start` even when real content already precedes
        // ~keep this point in the actual document -- e.g. `<p>A<i> </i>B</p>`, where the
        // ~keep space inside `<i>` sees its own buffer at length 0, identical to the
        // ~keep paragraph's block_content_start of 0, and was wrongly dropped entirely
        // ~keep rather than surviving as the one space `<i>`'s own whitespace-only handling
        // ~keep (`chomp_inline`) expects to receive (issue #481). An `inline_depth == 0`
        // ~keep guard was tried and reverted: it also blocked the legitimate case of a
        // ~keep genuinely block-level, buffer-sharing paragraph sitting at non-zero
        // ~keep `inline_depth` because a stray tag-soup `<b>`/`<a>` wraps WHOLE paragraphs
        // ~keep (Google Docs export artifact) -- `inline_depth` conflates "wrapped by an
        // ~keep inline ancestor" with "writing into that ancestor's detached scratch
        // ~keep buffer", and only the latter is what this check needs to exclude.
        if ctx.in_paragraph
            && std::ptr::from_ref::<String>(output) as usize == ctx.block_output_ptr
            && output.len() == ctx.block_content_start
        {
            return;
        }

        // ~keep CommonMark 4.8: leading whitespace at the very start of a block is
        // ~keep insignificant, with or without a newline in it. `<div>`/unknown tags,
        // ~keep `<style>`/`<textarea>`, and a stray closing tag never set `in_paragraph`,
        // ~keep so the check above never protected them — a lone leading space (no
        // ~keep newline) before e.g. `<div>` fell straight through to the verbatim-push
        // ~keep fallback below and survived into the output, breaking round-trip
        // ~keep stability. `in_table_cell`/`in_list_item`/`convert_as_inline` keep their
        // ~keep existing dedicated handling further down, untouched.
        if was_fresh_block_start && !ctx.convert_as_inline && !ctx.in_table_cell && !ctx.in_list_item {
            return;
        }

        if had_newlines {
            if output.is_empty() {
                return;
            }
            if !output.ends_with("\n\n") {
                // ~keep A run that mixes a newline with a more "significant" Unicode
                // ~keep whitespace character (a decoded `&nbsp;`, thin space, etc.) is
                // ~keep still whitespace-only by `str::trim`'s definition, but a compliant
                // ~keep HTML renderer's own pretty-printing can insert a leading `\n` in
                // ~keep front of this exact same content at any time (a rendered `<br>` is
                // ~keep always followed by a literal newline before its next text node).
                // ~keep Every branch below used to collapse a lone significant character to
                // ~keep a bare separating space -- or drop it outright when no separator was
                // ~keep needed -- which meant identical logical content survived or vanished
                // ~keep depending only on whether the HTML happened to be pretty-printed,
                // ~keep breaking round-trip stability. A lone significant character now
                // ~keep survives verbatim in every branch; more than one still collapses to
                // ~keep a single space.
                let significant: String = text
                    .as_ref()
                    .chars()
                    .filter(|c| !matches!(c, ' ' | '\t' | '\n' | '\r'))
                    .collect();
                let lone_significant_char = (!has_more_than_one_char(&significant))
                    .then(|| significant.chars().next())
                    .flatten();

                let next_tag = get_next_sibling_tag(node_handle, parser, dom_ctx);
                if let Some(next_tag) = next_tag {
                    if is_inline_element(next_tag) {
                        if let Some(ch) = lone_significant_char {
                            output.push(ch);
                        } else if !output.ends_with(' ') && !output.ends_with('\n') {
                            output.push(' ');
                        }
                        return;
                    }
                } else if let Some(ch) = lone_significant_char {
                    // ~keep This text node has no next sibling at all -- it is the tail of
                    // ~keep its parent's content -- so there is no "needs a separating
                    // ~keep space before the next word" question to ask; the lone
                    // ~keep significant character is preserved unconditionally.
                    output.push(ch);
                    return;
                } else if newline_span_needs_separating_space(node_handle, parser, dom_ctx)
                    && !output.ends_with(' ')
                    && !output.ends_with('\n')
                {
                    // ~keep issue #430: a lone "\n" inside an inline wrapper (e.g. a
                    // ~keep <span>) has no in-parent next sibling, but when that wrapper
                    // ~keep is itself followed by inline content the newline still
                    // ~keep separates words — collapse it to a single space.
                    output.push(' ');
                    return;
                } else if !significant.is_empty() && !output.ends_with(' ') && !output.ends_with('\n') {
                    output.push(' ');
                    return;
                }
            }
            return;
        }

        // ~keep A single-whitespace-character text node (typically the sole survivor of a
        // ~keep `<script>`/`<style>` removal that had to insert its own separating space --
        // ~keep see `preprocessing.rs`'s "neither side already has whitespace" guard --
        // ~keep landing next to a real, already-emitted trailing space) must still check
        // ~keep `output.ends_with(' ')` before pushing, exactly like the multi-char run just
        // ~keep above: otherwise two independently-legitimate single spaces stack into a
        // ~keep literal double space that only the first Markdown->HTML->Markdown hop
        // ~keep collapses back down, breaking round-trip stability.
        if previous_sibling_is_inline_tag(node_handle, parser, dom_ctx)
            && next_sibling_is_inline_tag(node_handle, parser, dom_ctx)
        {
            if has_more_than_one_char(text.as_ref()) {
                // ~keep A run collapses to one plain space only when it is genuinely ASCII
                // ~keep formatting whitespace. A run that is -- or contains -- a decoded
                // ~keep `&nbsp;`/other significant Unicode whitespace trims to empty under
                // ~keep `str::trim`'s definition (which is why this whole node reached the
                // ~keep "whitespace-only" branch), but collapsing it the same way discards
                // ~keep real, visible content: an `<img>`...`&nbsp;&nbsp;&nbsp;`...`<a>` run
                // ~keep between two inline siblings must survive verbatim, the same as it
                // ~keep already does one branch below when the two siblings are not both
                // ~keep directly adjacent.
                if is_ascii_whitespace_only(text.as_ref()) {
                    if !output.ends_with(' ') {
                        output.push(' ');
                    }
                } else {
                    output.push_str(text.as_ref());
                }
            } else if !output.ends_with(' ') {
                output.push_str(text.as_ref());
            }
        } else if !output.ends_with(' ') {
            output.push_str(text.as_ref());
        }
        return;
    }

    // ~keep From here on `text` has real, non-whitespace content, so anything still
    // ~keep downstream of this point in the document is no longer at a fresh block
    // ~keep start — flip the shared flag before it can leak "is fresh" to a later
    // ~keep sibling, whichever buffer this particular call happened to write into.
    ctx.at_fresh_block_start.set(false);

    let processed_text = if (ctx.in_code || ctx.in_ruby) && ctx.in_table_cell {
        // ~keep Code/ruby content is verbatim by design, but a GFM table cell cannot
        // ~keep contain a raw newline: fold line breaks to a space without touching any
        // ~keep other whitespace, and regardless of whitespace_mode — this is a structural
        // ~keep constraint of the cell, not a stylistic normalization (issue #455).
        text::fold_cell_line_breaks_verbatim_cow(text.as_ref()).into_owned()
    } else if ctx.in_code || ctx.in_ruby {
        text.into_owned()
    } else if ctx.in_table_cell {
        // ~keep Always escape * and _ in table cells to prevent unintended emphasis.
        // ~keep When escape_misc is false the previous implementation appended a
        // ~keep post-pass `String::replace('|', "\\|")`.  We fold the pipe escape
        // ~keep into the misc set so the byte-loop handles it in the same walk,
        // ~keep avoiding a second allocation.
        let normalized_text = if options.whitespace_mode == crate::options::WhitespaceMode::Normalized {
            text::normalize_cell_whitespace_cow(text.as_ref())
        } else {
            // ~keep Strict still preserves every other whitespace byte, but a raw newline in a
            // ~keep GFM cell splits the row across physical lines — a structural impossibility
            // ~keep rather than a formatting preference, so it folds in every mode (issue #457,
            // ~keep same reasoning as the verbatim fold above for #455).
            text::fold_cell_line_breaks_verbatim_cow(text.as_ref())
        };
        let src = normalized_text.as_ref();
        let mut out = String::with_capacity(src.len());
        text::escape_into(&mut out, src, options.escape_misc, true, true, options.escape_ascii);
        if !options.escape_misc {
            if out.contains('|') {
                out = out.replace('|', r"\|");
            }
        }
        out
    } else if options.whitespace_mode == crate::options::WhitespaceMode::Strict {
        text::escape(
            text.as_ref(),
            options.escape_misc,
            options.escape_asterisks,
            options.escape_underscores,
            options.escape_ascii,
        )
        .into_owned()
    } else {
        let has_double_newline = text.contains("\n\n") || text.contains("\r\n\r\n");
        let has_trailing_single_newline =
            text.ends_with('\n') && !text.ends_with("\n\n") && !text.ends_with("\r\n\r\n");

        // ~keep `prefix`/`suffix` presence and the trailing-`"\n\n"` special case are
        // ~keep unaffected by whitespace collapsing (both only ask "is there any
        // ~keep whitespace here", not "how much"), so deriving them from the collapsed
        // ~keep text is safe and keeps `chomp`'s Unicode-aware boundary detection intact.
        // ~keep The *content* fed to `normalize_block_whitespace_cow` below must be the
        // ~keep raw, pre-collapse core, though: only once its own leading/trailing
        // ~keep whitespace is already gone (by trimming the same boundaries `chomp` just
        // ~keep found) does every remaining `\n`-adjacent run inside it sit strictly
        // ~keep between two pieces of real content -- never at the text node's own edge,
        // ~keep where a *different* rule applies (see that function's doc comment).
        let normalized_text = text::normalize_whitespace_cow(text.as_ref());
        let (prefix, suffix, _) = text::chomp(normalized_text.as_ref());
        let core = text::normalize_block_whitespace_cow(text.trim());

        let skip_prefix = (was_fresh_block_start && !ctx.convert_as_inline && !ctx.in_table_cell && !ctx.in_list_item)
            || output.ends_with("\n\n")
            || output.ends_with("* ")
            || output.ends_with("- ")
            || output.ends_with(". ")
            || output.ends_with("] ")
            || (output.ends_with('\n') && prefix == " ")
            || (output.ends_with(' ')
                && prefix == " "
                && !previous_sibling_is_inline_tag(node_handle, parser, dom_ctx));

        let mut final_text = String::with_capacity(prefix.len() + core.len() + suffix.len() + 2);
        if !skip_prefix && !prefix.is_empty() {
            final_text.push_str(prefix);
        }

        let escaped_core = text::escape(
            core.as_ref(),
            options.escape_misc,
            options.escape_asterisks,
            options.escape_underscores,
            options.escape_ascii,
        );
        final_text.push_str(&escaped_core);

        if !suffix.is_empty() {
            final_text.push_str(suffix);
        } else if has_trailing_single_newline {
            let safe_start = ctx.block_content_start.min(output.len());
            let safe_start = crate::converter::utility::content::floor_char_boundary(output, safe_start);
            let current_block_output = &output[safe_start..];
            let at_paragraph_break = current_block_output.ends_with("\n\n");
            if !at_paragraph_break {
                if has_double_newline {
                    final_text.push('\n');
                } else if let Some(next_tag) = get_next_sibling_tag(node_handle, parser, dom_ctx) {
                    if matches!(next_tag, "span") {
                    } else if ctx.inline_depth > 0 || ctx.convert_as_inline || ctx.in_paragraph {
                        final_text.push(' ');
                    } else {
                        final_text.push('\n');
                    }
                } else if ctx.inline_depth > 0 || ctx.convert_as_inline || ctx.in_paragraph {
                    final_text.push(' ');
                } else {
                    final_text.push('\n');
                }
            }
        }

        final_text
    };

    #[cfg(feature = "visitor")]
    let final_text = if let Some(ref visitor_handle) = ctx.visitor {
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

        let node_ctx = NodeContext::with_borrowed_attributes(
            NodeType::Text,
            Cow::Borrowed(""),
            &EMPTY_ATTRS,
            depth,
            index_in_parent,
            parent_tag.map(Cow::Borrowed),
            true,
        );

        let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
        match visitor.visit_text(&node_ctx, &processed_text) {
            VisitResult::Continue => processed_text,
            VisitResult::Custom(custom) => {
                if ctx.inline_depth > 0 || ctx.in_heading {
                    processed_text
                } else {
                    custom
                }
            }
            VisitResult::Skip => return,
            VisitResult::Error(err) => {
                if ctx.visitor_error.borrow().is_none() {
                    *ctx.visitor_error.borrow_mut() = Some(err);
                }
                return;
            }
            VisitResult::PreserveHtml => processed_text,
        }
    } else {
        processed_text
    };

    #[cfg(not(feature = "visitor"))]
    let final_text = processed_text;

    // ~keep A text node that starts a fresh, still-unindented physical line inside a list
    // ~keep item (e.g. sibling text right after a heading, which only emits a single
    // ~keep trailing newline rather than a blank line) needs the same continuation indent
    // ~keep every block handler adds before its own first line, or it lands flush left and
    // ~keep the item derails on re-parse (CommonMark spec example 300). Excluded from verbatim
    // ~keep contexts (`in_code`/`in_ruby`) and from contexts that build into a detached
    // ~keep scratch buffer rather than the real document (`in_table_cell`, `convert_as_inline`),
    // ~keep where `output` is not the list item's own accumulating text and indenting it would
    // ~keep corrupt literal content instead.
    if ctx.in_list_item
        && !ctx.in_code
        && !ctx.in_ruby
        && !ctx.in_table_cell
        && !ctx.convert_as_inline
        && output.ends_with('\n')
        && !output.ends_with("\n\n")
    {
        if let Some(indent) =
            crate::converter::list::utils::continuation_indent_string(ctx.list_depth, ctx.list_indent_columns, options)
        {
            output.push_str(&indent);
        }
    }

    if ctx.in_list_item && final_text.contains("\n\n") {
        let indent = " ".repeat(4 * ctx.list_depth);
        let mut first = true;
        for part in final_text.split("\n\n") {
            if !first {
                output.push_str("\n\n");
                output.push_str(&indent);
            }
            first = false;
            output.push_str(part.trim());
        }
    } else {
        output.push_str(&final_text);
    }
}

/// Whether a whitespace-only newline text node with no in-parent next sibling
/// still separates inline content (issue #430).
///
/// Returns true when the node's parent is an inline-like element that is itself
/// followed by inline content — e.g. the lone `"\n"` inside the middle `<span>`
/// of `<span>a</span><span>\n</span><span>b</span>`.
fn newline_span_needs_separating_space(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    dom_ctx: &DomContext,
) -> bool {
    let Some(parent_id) = dom_ctx.parent_of(node_handle.get_inner()) else {
        return false;
    };
    let parent_is_inline = dom_ctx
        .tag_info(parent_id, parser)
        .is_some_and(|info| info.is_inline_like);
    if !parent_is_inline {
        return false;
    }
    let parent_handle = tl::NodeHandle::new(parent_id);
    next_sibling_is_inline_tag(&parent_handle, parser, dom_ctx)
}
