//! Tier-1 single-pass byte scanner.
//!
//! Walks `html.as_bytes()` once and emits Markdown directly to a pre-sized
//! output buffer.  On any construct it cannot handle exactly, returns a
//! [`BailReason`] so the dispatcher can fall back to Tier-2.
//!
//! # Supported subset (M9)
//!
//! Paragraph, Heading(1-6), Strong, Emphasis, Code (inline), Pre, Hr,
//! LineBreak, Link, Image, List(Unordered), List(Ordered), ListItem,
//! Blockquote, Block (div/section/article/etc.), Inline (span/etc.),
//! Table (GFM — conservative bail set, inline-only cell content).
//!
//! Bails on: RawText(script/style/textarea/etc.), DefinitionTerm,
//! DefinitionDescription, List(Definition), Ignored (head/meta/link),
//! nested tables, colspan/rowspan != 1, block children in cells,
//! `<caption>`, section-order violations, multi-line cell content,
//! and any HTML construct with in-text whitespace complexity or unclosed tags.

use crate::converter::prescan::PrescanReport;
use crate::converter::tier1::bail::BailReason;
use crate::converter::tier1::parse;
use crate::converter::tier1::spec_rules;
use crate::converter::tier1::state::{EscapeCtx, OpenTag, Tier1State};
use crate::converter::tier1::tags::{ListKind, RawKind, TagKind, TagSpec};
use crate::converter::tier1::{self};
use crate::options::ConversionOptions;

/// Entry point for the Tier-1 scanner.
pub fn scan(html: &str, _report: &PrescanReport, options: &ConversionOptions) -> Result<String, BailReason> {
    let bytes = html.as_bytes();
    let mut state = Tier1State::new(html.len());
    let mut pos = 0usize;
    let mut text_start = 0usize;

    while pos < bytes.len() {
        match bytes[pos] {
            b'<' => {
                // Flush pending text before we process this tag.
                if text_start < pos {
                    flush_text(&mut state, &html[text_start..pos], text_start)?;
                }

                let next = bytes.get(pos + 1).copied().unwrap_or(0);

                // `<!` — comment, DOCTYPE, or CDATA
                if next == b'!' {
                    if html[pos..].starts_with("<![CDATA[") {
                        return Err(BailReason::Cdata { offset: pos });
                    }
                    // Skip `<!-- ... -->` or `<!DOCTYPE ...>` etc.
                    pos = skip_bang(bytes, pos)?;
                    text_start = pos;
                    continue;
                }

                // `</` — closing tag
                if next == b'/' {
                    let name_start = pos + 2;
                    let name_end = parse::scan_tag_name(bytes, name_start);
                    if name_end == name_start {
                        // `</>` or similar — bail
                        return Err(BailReason::LiteralLt { offset: pos });
                    }
                    let close_bracket =
                        parse::find_tag_close(bytes, name_end).ok_or(BailReason::LiteralLt { offset: pos })?;

                    let tag_name_bytes = &bytes[name_start..name_end];
                    emit_close(&mut state, tag_name_bytes)?;

                    pos = close_bracket.0 + 1;
                    text_start = pos;
                    continue;
                }

                // Not a tag-name-start byte → literal `<`
                if !parse::is_tag_name_start(next) {
                    return Err(BailReason::LiteralLt { offset: pos });
                }

                // Opening tag
                let name_start = pos + 1;
                let name_end = parse::scan_tag_name(bytes, name_start);
                let tag_name_bytes = &bytes[name_start..name_end];

                // Lowercase the tag name into a stack buffer (max 32 bytes)
                let mut name_buf = [0u8; 32];
                let name_lower = lowercase_into(tag_name_bytes, &mut name_buf);

                // Custom elements (contain `-`) → bail
                if name_lower.contains(&b'-') {
                    return Err(BailReason::UnknownCustomElement {
                        name: bytes_to_string(tag_name_bytes),
                        offset: pos,
                    });
                }

                // Unknown tag → bail
                let spec = tier1::lookup(name_lower).ok_or_else(|| BailReason::UnknownCustomElement {
                    name: bytes_to_string(tag_name_bytes),
                    offset: pos,
                })?;

                // Bail on unsupported tag kinds for M3c
                bail_unsupported(spec, pos)?;

                // Bail on <pre> when code_block_style is not Indented.
                // Tier-1 only implements 4-space indented code blocks; other styles
                // (Backticks, Tildes) require Tier-2's fenced-block logic.
                if matches!(spec.kind, TagKind::Pre)
                    && options.code_block_style != crate::options::CodeBlockStyle::Indented
                {
                    return Err(BailReason::Classifier);
                }

                // Bail on nested lists: Tier-2 cycles bullet characters by depth
                // (-, *, +) but Tier-1 always uses "-". Nesting requires Tier-2.
                if matches!(spec.kind, TagKind::List(ListKind::Unordered | ListKind::Ordered))
                    && state.list_depth > 0
                {
                    return Err(BailReason::Classifier);
                }

                // Find end of tag (handles quoted attribute values)
                let close = parse::find_tag_close(bytes, name_end).ok_or(BailReason::LiteralLt { offset: pos })?;

                // Collect attributes (from after name_end to before `>` / `/>`)
                let attrs_end = if close.1 {
                    // Self-closing `/>` — back up one past the `/`
                    close.0.saturating_sub(1)
                } else {
                    close.0
                };
                // Most tag kinds (headings, paragraphs, emphasis, code, etc.) do
                // not read attributes during emit.  Skip the allocation in the
                // common case; only collect for the kinds whose emit paths
                // actually consult attributes.
                let attrs: Vec<(&[u8], Option<&[u8]>)> = match spec.kind {
                    TagKind::Link
                    | TagKind::Image
                    | TagKind::List(ListKind::Ordered)
                    | TagKind::TableCell { .. } => parse::collect_attrs(bytes, name_end, attrs_end),
                    _ => Vec::new(),
                };

                pos = close.0 + 1;

                // Void or self-closing: emit immediately, don't push stack
                if spec.is_void || close.1 {
                    emit_void(&mut state, spec, &attrs, html, options)?;
                    text_start = pos;
                    continue;
                }

                // Non-void open tag: emit opening markdown + push stack frame

                // M9: Nested-table bail — must come before the block-in-cell
                // check because <table> is a block element, and TableNestedTable
                // is a more specific reason than TableBlockChildInCell.
                if matches!(spec.kind, TagKind::Table) && !state.table_stack.is_empty() {
                    return Err(BailReason::TableNestedTable);
                }

                // M9: Block-in-cell bail.
                // If we are inside a table cell and the new tag is a block-level
                // element, bail immediately.  This check must come before the
                // implicit-close loop so that the bail fires before we try to pop
                // the cell frame.
                if state.in_table_cell() && spec.is_block {
                    return Err(BailReason::TableBlockChildInCell);
                }

                // M4: HTML5 implicit-close transitions.
                // Before pushing the new tag, check whether any open tag at the
                // top of the stack should be implicitly closed per the HTML5
                // optional-tag rules.  Repeat until no more implicit closes fire
                // (handles e.g. <li><li><li> or <p><p>).
                while let Some(top) = state.stack.last() {
                    if !spec_rules::should_close_for_new_tag(top.spec, spec) {
                        break;
                    }
                    emit_close_for_implicit(&mut state)?;
                }

                let prev_ctx = state.escape_ctx;
                let ol_start = if matches!(spec.kind, TagKind::List(ListKind::Ordered)) {
                    extract_ol_start(&attrs)
                } else {
                    1
                };
                let (link_href, link_title) = if matches!(spec.kind, TagKind::Link) {
                    extract_link_attrs(&attrs)?
                } else {
                    (None, None)
                };

                emit_open(&mut state, spec, &attrs, ol_start)?;

                // Record the content-start position AFTER emit_open so that
                // close-side post-processing operates on the correct slice.
                // When inside a table cell the position is in the cell buffer;
                // otherwise it is in the main output buffer.
                let output_content_start = state.cell_or_output_mut().len();

                // list_index is initialised to 0 for lists (counter starts at 0,
                // incremented on each <li>).  For non-lists, unused.
                let list_index = 0u16;

                state.stack.push(OpenTag {
                    spec,
                    content_start: output_content_start,
                    prev_escape_ctx: prev_ctx,
                    list_index,
                    link_href,
                    link_title,
                    ol_start,
                    name_range: name_start..name_end,
                });

                // Update escape context after pushing so the frame records the
                // pre-tag ctx correctly.
                update_escape_ctx(&mut state, spec, true);

                text_start = pos;
            }
            _ => pos += 1,
        }
    }

    // Flush any trailing text after the last tag.
    if text_start < pos {
        flush_text(&mut state, &html[text_start..pos], text_start)?;
    }

    // M4: Implicitly close any optional-close elements still open at EOF.
    // HTML5 allows omitting end tags for elements like <p>, <li>, <dt>, <dd>.
    while let Some(top) = state.stack.last() {
        if top.spec.optional_close.is_some() {
            emit_close_for_implicit(&mut state)?;
        } else {
            break;
        }
    }

    // Remaining unclosed block elements at EOF → bail
    if !state.stack.is_empty() {
        return Err(BailReason::EofWithOpenBlock {
            open_count: state.stack.len(),
        });
    }

    // Collapse runs of 3+ consecutive newlines to exactly 2, matching Tier-2's
    // `collapse_excess_blank_lines` post-processing step.
    if state.output.contains("\n\n\n") {
        collapse_excess_blank_lines(&mut state.output);
    }

    // Normalise trailing newlines to match Tier-2's final-output contract:
    //   `format!("{}\n", output.trim_end_matches('\n'))`
    // Tier-2 strips all trailing newlines and appends exactly one.  We mirror
    // that here so paragraphs (which emit "\n\n") don't leave an extra blank
    // line at the end.
    if !state.output.is_empty() {
        let trimmed_end = state.output.trim_end_matches('\n');
        if trimmed_end.is_empty() {
            state.output.clear();
        } else {
            let trimmed_len = trimmed_end.len();
            state.output.truncate(trimmed_len);
            state.output.push('\n');
        }
    }

    Ok(state.output)
}

// ── Bail guard ────────────────────────────────────────────────────────────────

/// Return `Err(BailReason::Classifier)` for tag kinds not supported in M9.
///
/// Table-related tags are now handled by the scanner (M9); they are no longer
/// bailed here.  Table-specific bail reasons are emitted by the table-handling
/// code in `emit_open` and `emit_close`.
#[inline]
fn bail_unsupported(spec: &TagSpec, _offset: usize) -> Result<(), BailReason> {
    match spec.kind {
        TagKind::DefinitionTerm
        | TagKind::DefinitionDescription
        | TagKind::List(ListKind::Definition)
        | TagKind::RawText(RawKind::Textarea)
        | TagKind::RawText(RawKind::Title)
        | TagKind::RawText(RawKind::Xmp)
        | TagKind::RawText(RawKind::Iframe)
        | TagKind::RawText(RawKind::Noscript)
        | TagKind::RawText(RawKind::NoEmbed)
        | TagKind::RawText(RawKind::NoFrames) => Err(BailReason::Classifier),

        // script/style are `Ignored` with `is_rawtext=true`; handled as void-like
        // by the prescan (stripped). We bail here as a safety net — if they
        // appear in the scanner they weren't stripped by the prescan, which
        // means they have content we can't skip.
        TagKind::RawText(RawKind::Script) | TagKind::RawText(RawKind::Style) => Err(BailReason::Classifier),

        // head / meta / link are Ignored — we can silently skip them when void,
        // but if they have children (weird HTML) we bail.
        TagKind::Ignored => Err(BailReason::Classifier),

        _ => Ok(()),
    }
}

// ── Open-tag emission ─────────────────────────────────────────────────────────

fn emit_open(
    state: &mut Tier1State,
    spec: &'static TagSpec,
    attrs: &[(&[u8], Option<&[u8]>)],
    ol_start: u16,
) -> Result<(), BailReason> {
    let _ = ol_start;

    match spec.kind {
        TagKind::Paragraph => {
            // Mirrors Tier-2: when output is non-empty and doesn't already end
            // with "\n\n", push "\n\n" (may produce three newlines total when
            // output ends with a single "\n", e.g. right after a table row or
            // an `<hr>`).
            if !state.output.is_empty() && !state.output.ends_with("\n\n") {
                state.output.push_str("\n\n");
            }
        }

        TagKind::Heading(_) => {
            state.ensure_blank_line();
        }

        TagKind::Blockquote => {
            state.ensure_blank_line();
        }

        TagKind::Pre => {
            state.ensure_blank_line();
        }

        TagKind::List(_) => {
            // Lists at the top level need a blank line if there's preceding content.
            // Inside a list item (`list_depth > 0`) just a newline is enough.
            if !state.output.is_empty() {
                if state.list_depth == 0 {
                    // Top-level list: ensure blank line before
                    state.ensure_blank_line();
                } else {
                    // Nested list inside a list item: just newline
                    if !state.output.ends_with('\n') {
                        state.output.push('\n');
                    }
                }
            }
            state.list_depth = state.list_depth.saturating_add(1);
        }

        TagKind::ListItem => {
            // Emit the list item marker.
            let parent_kind = find_parent_list_kind(&state.stack);
            let indent = list_item_indent(state.list_depth.saturating_sub(1));
            match parent_kind {
                Some(ListKind::Ordered) => {
                    // Increment counter on parent ordered list frame
                    let counter = increment_ol_counter(&mut state.stack);
                    let start = find_ol_start(&state.stack);
                    let index = start.saturating_sub(1) + counter;
                    state.output.push_str(&indent);
                    state.output.push_str(&format!("{index}. "));
                }
                _ => {
                    state.output.push_str(&indent);
                    state.output.push_str("- ");
                }
            }
        }

        TagKind::Strong => {
            state.cell_or_output_mut().push_str("**");
        }

        TagKind::Emphasis => {
            state.cell_or_output_mut().push('*');
        }

        TagKind::Code => {
            // When inside <pre>, <code> is transparent — no backtick markers.
            if !state.escape_ctx.contains(EscapeCtx::PRE) {
                state.cell_or_output_mut().push('`');
            }
        }

        TagKind::Link => {
            // Track link count inside tables for layout-table detection.
            if let Some(ts) = state.table_stack.last_mut() {
                ts.link_count += 1;
            }
            state.cell_or_output_mut().push('[');
        }

        // ── Table handling ──────────────────────────────────────────────────────

        TagKind::Table => {
            // The nested-table check was already done in the main scanner loop
            // (before this emit_open call) to ensure TableNestedTable takes
            // priority over TableBlockChildInCell.
            state.table_stack.push(crate::converter::tier1::state::TableState::default());
        }

        TagKind::TableCaption => {
            return Err(BailReason::TableCaption);
        }

        TagKind::TableHead => {
            if let Some(ts) = state.table_stack.last_mut() {
                // thead after any body or foot section → section order violation.
                if ts.seen_tbody_close || ts.seen_tfoot {
                    return Err(BailReason::TableSectionOrder);
                }
                ts.in_thead = true;
            }
        }

        TagKind::TableBody => {
            if let Some(ts) = state.table_stack.last_mut() {
                // tbody after tfoot open → section order violation.
                if ts.seen_tfoot {
                    return Err(BailReason::TableSectionOrder);
                }
            }
        }

        TagKind::TableFoot => {
            if let Some(ts) = state.table_stack.last_mut() {
                ts.seen_tfoot = true;
            }
        }

        TagKind::TableRow => {
            // Clear the in-progress row.
            if let Some(ts) = state.table_stack.last_mut() {
                ts.current_row.clear();
            }
        }

        TagKind::TableCell { is_header } => {
            // Check rowspan / colspan — bail if either != 1.
            let span_val = |key: &[u8]| -> u32 {
                find_attr(attrs, key)
                    .and_then(|b| std::str::from_utf8(b).ok())
                    .and_then(|s| s.trim().parse::<u32>().ok())
                    .unwrap_or(1)
            };
            if span_val(b"rowspan") != 1 || span_val(b"colspan") != 1 {
                return Err(BailReason::TableRowspanColspan);
            }
            // Start accumulating cell text.
            if let Some(ts) = state.table_stack.last_mut() {
                ts.current_cell.clear();
                ts.in_cell = true;
                if is_header {
                    ts.has_th = true;
                }
            }
        }

        // Block containers: just track them on the stack (no inline marker).
        TagKind::Block | TagKind::Inline => {}

        _ => {}
    }

    Ok(())
}

/// Emit a void element (no closing tag).
fn emit_void(
    state: &mut Tier1State,
    spec: &'static TagSpec,
    attrs: &[(&[u8], Option<&[u8]>)],
    html: &str,
    options: &ConversionOptions,
) -> Result<(), BailReason> {
    match spec.kind {
        TagKind::Hr => {
            state.ensure_blank_line();
            state.output.push_str("---\n");
        }

        TagKind::LineBreak => {
            // `<br>` outside any block context emits nothing (Tier-2 behaviour).
            // Inside a table cell, bail: Tier-1 does not implement br_in_tables.
            // Inside a regular block (paragraph, div, etc.) emit `  \n`.
            if state.in_table_cell() {
                // Bail: we can't handle <br> inside table cells reliably.
                return Err(BailReason::TableBlockChildInCell);
            } else if state.stack.is_empty() {
                // bare `<br>` at top level — Tier-2 emits nothing
            } else {
                state.output.push_str("  \n");
            }
        }

        TagKind::Image => {
            let src = find_attr(attrs, b"src").unwrap_or_default();
            let alt = find_attr(attrs, b"alt").unwrap_or_default();
            let title = find_attr(attrs, b"title");

            let src = decode_attr(src)?;
            let alt = decode_attr(alt)?;

            let keep_as_markdown = should_keep_image_as_markdown(html, &state.stack, options);

            let dest = state.cell_or_output_mut();
            if keep_as_markdown {
                if let Some(title_bytes) = title {
                    let title_str = decode_attr(title_bytes)?;
                    dest.push_str(&format!("![{alt}]({src} \"{title_str}\")"));
                } else {
                    dest.push_str(&format!("![{alt}]({src})"));
                }
            } else {
                // Strip to alt-text only — mirrors Tier-2 behaviour when the image
                // is in a heading whose tag is not in `keep_inline_images_in`.
                dest.push_str(&alt);
            }
        }

        // Ignored void elements (meta, link, area, wbr, etc.) — drop silently
        TagKind::Ignored | TagKind::Inline | TagKind::Block => {}

        _ => {}
    }
    Ok(())
}

/// Decide whether an `<img>` should be emitted as `![alt](src)` markdown.
///
/// When the `inline-images` feature is disabled, images are always kept as
/// markdown (original Tier-1 behaviour).
///
/// When the feature is enabled this mirrors the Tier-2 logic in
/// `converter.rs`:
/// - `keep_inline_images_in` empty → always emit markdown image.
/// - `keep_inline_images_in` non-empty → emit markdown only when the image
///   has a heading ancestor (`h1`–`h6`) whose (lowercased) tag name is in the
///   list; otherwise emit alt-text only.
///
/// Ancestor matching is ASCII-case-insensitive so callers may supply "H1" or
/// "h1" interchangeably.
#[inline]
fn should_keep_image_as_markdown(html: &str, stack: &[OpenTag], options: &ConversionOptions) -> bool {
    #[cfg(feature = "inline-images")]
    {
        keep_inline_image_for_ancestors(html.as_bytes(), stack, &options.keep_inline_images_in)
    }
    #[cfg(not(feature = "inline-images"))]
    {
        let _ = html;
        let _ = stack;
        let _ = options;
        true
    }
}

/// Return `true` when the `<img>` should be emitted as `![alt](src)` markdown.
///
/// Mirrors the Tier-2 logic in `converter.rs`: images are kept as markdown
/// unconditionally when `keep_inline_images_in` is empty.  When the list is
/// non-empty, an image is kept only when it has a heading ancestor (`h1`–`h6`)
/// whose (lowercased) tag name appears in the list; otherwise the caller should
/// emit alt-text only.
///
/// The comparison is ASCII-case-insensitive on both the stack name bytes and the
/// user-supplied strings, so callers may supply "H1" or "h1" interchangeably.
#[cfg(feature = "inline-images")]
fn keep_inline_image_for_ancestors(input: &[u8], stack: &[OpenTag], keep: &[String]) -> bool {
    if keep.is_empty() {
        // No restriction — always emit markdown image (Tier-2 default).
        return true;
    }
    for frame in stack.iter().rev() {
        if matches!(frame.spec.kind, TagKind::Heading(_)) {
            let name = &input[frame.name_range.clone()];
            for keep_name in keep {
                if eq_ascii_ignore_case(name, keep_name.as_bytes()) {
                    return true;
                }
            }
            // Found a heading ancestor but its name is not in the list.
            return false;
        }
    }
    // No heading ancestor at all: no restriction applies — emit markdown image.
    // This matches Tier-2 behaviour: the `keep_inline_images_in` guard only
    // fires when `ctx.in_heading` is true.
    true
}

/// Byte-level ASCII case-insensitive comparison — no allocation.
#[cfg(feature = "inline-images")]
fn eq_ascii_ignore_case(a: &[u8], b: &[u8]) -> bool {
    a.eq_ignore_ascii_case(b)
}

// ── Close-tag emission ────────────────────────────────────────────────────────

fn emit_close(state: &mut Tier1State, tag_name_bytes: &[u8]) -> Result<(), BailReason> {
    // Lowercase the tag name to look it up in the spec table.
    let mut name_buf = [0u8; 32];
    let name_lower = lowercase_into(tag_name_bytes, &mut name_buf);

    let spec = tier1::lookup(name_lower).ok_or_else(|| BailReason::UnknownCustomElement {
        name: bytes_to_string(tag_name_bytes),
        offset: 0,
    })?;

    // M4: Before popping the matching frame, implicitly close any open optional-close
    // elements at the top of the stack that would be auto-closed by their parent's
    // close tag.  For example, `</ul>` with an open `<li>` on top → close the `<li>`
    // first (HTML5 spec: li/dt/dd are implicitly closed when their parent closes).
    while let Some(top) = state.stack.last() {
        if kinds_match(&top.spec.kind, &spec.kind) {
            // Found the matching frame — stop flushing optional-close children.
            break;
        }
        if top.spec.optional_close.is_some() {
            // This top-of-stack element has an optional close rule; implicitly close it.
            emit_close_for_implicit(state)?;
        } else {
            // Top element does not have optional close and doesn't match — genuine mismatch.
            break;
        }
    }

    // Pop the matching frame from the open-tag stack.
    // Tier-2 is lenient about mismatched tags; for M3c we bail.
    let actual_depth = state.stack.len() as u8;
    let frame = pop_matching_frame(&mut state.stack, spec).ok_or_else(|| BailReason::DepthMismatch {
        tag: bytes_to_string(name_lower),
        expected: 1,
        actual: actual_depth,
    })?;

    // Restore escape context
    state.escape_ctx = frame.prev_escape_ctx;

    match spec.kind {
        TagKind::Paragraph => {
            // Tier-2 appends "\n\n" after paragraph content (always two newlines).
            // Matching this precisely is required for byte-equal output.
            trim_trailing_inline_whitespace(state);
            state.output.push_str("\n\n");
        }

        TagKind::Heading(n) => {
            // The heading text was emitted as raw text; we need to prepend the
            // `# ` prefix.  We stored the output position before the heading
            // opened via `content_start` convention: we need to insert `# `
            // at the beginning of the heading content.
            //
            // Implementation: heading is on the stack, content_start is stored
            // in the frame.  We insert the `# ` prefix at `content_start`.
            trim_trailing_inline_whitespace(state);
            let content = &state.output[frame.content_start..];
            if content.trim().is_empty() {
                // Empty heading: Tier-2 emits nothing. Roll back to before
                // the heading's block separator was added.
                state.output.truncate(frame.content_start);
                // Also trim any blank-line separator that ensure_blank_line
                // added before the heading opened (frame.content_start is
                // after emit_open, which may have pushed "\n\n").
                let trimmed_len = state.output.trim_end_matches('\n').len();
                if trimmed_len > 0 {
                    state.output.truncate(trimmed_len);
                    state.output.push('\n');
                } else {
                    state.output.clear();
                }
            } else {
                let prefix = heading_prefix(n);
                state.output.insert_str(frame.content_start, &prefix);
                // Tier-2 leaves a blank line ("\n\n") after a heading. A
                // following paragraph's "\n\n" guard then finds it already
                // and appends nothing, yielding the expected single blank line.
                state.ensure_blank_line();
            }
        }

        TagKind::Blockquote => {
            // The blockquote content needs `> ` prefixed to every line.
            // We inserted nothing on open; now we need to post-process the
            // content from `frame.content_start` to the current output end.
            let content = state.output[frame.content_start..].to_owned();
            let prefixed = prefix_blockquote_lines(&content);
            state.output.truncate(frame.content_start);
            // Mirror Tier-2 blockquote.rs: when the output ends with "\n\n"
            // before the blockquote, remove one "\n" (heading-then-blockquote
            // produces only a single newline separator, not a blank line).
            if state.output.ends_with("\n\n") {
                state.output.pop();
            }
            state.output.push_str(&prefixed);
        }

        TagKind::Pre => {
            // Indent every line by 4 spaces.
            let raw = state.output[frame.content_start..].to_owned();
            let indented = indent_pre_lines(&raw);
            state.output.truncate(frame.content_start);
            state.output.push_str(&indented);
        }

        TagKind::Strong => {
            state.cell_or_output_mut().push_str("**");
        }

        TagKind::Emphasis => {
            state.cell_or_output_mut().push('*');
        }

        TagKind::Code => {
            // When inside <pre>, <code> is transparent — no backtick markers.
            if !state.escape_ctx.contains(EscapeCtx::PRE) {
                state.cell_or_output_mut().push('`');
            }
        }

        TagKind::Link => {
            // Close the link: `](href "title")` or `](href)`
            // If no href, just emit the text as-is (Tier-2 behaviour: no link markup).
            let dest = state.cell_or_output_mut();
            if let Some(href) = &frame.link_href {
                if let Some(title) = &frame.link_title {
                    dest.push_str(&format!("]({href} \"{title}\")"));
                } else {
                    dest.push_str(&format!("]({href})"));
                }
            } else {
                // No href: remove the `[` we emitted on open, keep just the text.
                // Find and remove the opening `[` that corresponds to this link.
                // content_start is relative to cell buffer when in a cell.
                if let Some(bracket_pos) = dest[..frame.content_start].rfind('[') {
                    dest.remove(bracket_pos);
                }
            }
        }

        TagKind::List(_) => {
            state.list_depth = state.list_depth.saturating_sub(1);
            // Ensure the list ends with exactly one newline before any following content.
            if !state.output.ends_with('\n') {
                state.output.push('\n');
            }
        }

        TagKind::ListItem => {
            trim_trailing_inline_whitespace(state);
            state.ensure_newline();
        }

        TagKind::Hr => {
            // Should not happen (hr is void), but handle gracefully.
        }

        // ── Table handling ──────────────────────────────────────────────────────

        TagKind::Table => {
            // Pop the table state and (if safe) emit the GFM table to main output.
            if let Some(ts) = state.table_stack.pop() {
                // Safety checks: ensure Tier-2 would also use the GFM path.
                //
                // Tier-2 uses the layout (non-GFM) path when ALL of these hold:
                //   (a) no <th> anywhere in the table, AND
                //   (b) no <caption>, AND
                //   (c) looks_like_layout || is_blank || (row_count<=2 && link_count>=3)
                //
                // Where looks_like_layout covers nested tables (already bailed),
                // colspan/rowspan (already bailed), and inconsistent column counts.
                //
                // If those conditions could apply to this table, we bail rather than
                // emit a GFM table that Tier-2 would have rendered differently.
                if !ts.has_th {
                    // No <th>: check if Tier-2 would take the layout path.
                    let row_count = ts.rows.len();

                    // Inconsistent column counts → layout table in Tier-2.
                    let inconsistent_cols = {
                        let first = ts.first_row_col_count.unwrap_or(0);
                        ts.rows.iter().any(|r| r.len() != first)
                    };

                    // Link-heavy with few rows → layout table in Tier-2.
                    let link_heavy = row_count <= 2 && ts.link_count >= 3;

                    // Blank table → Tier-2 emits nothing (not a bail case).
                    let is_blank = ts.rows.is_empty()
                        || ts.rows.iter().all(|r| r.iter().all(|c| c.trim().is_empty()));

                    if inconsistent_cols || link_heavy || is_blank {
                        // Tier-2 would not emit a GFM table here.
                        // Bail so the fallback produces the correct layout output.
                        return Err(BailReason::Classifier);
                    }
                }
                emit_gfm_table(state, ts);
            }
        }

        TagKind::TableHead => {
            if let Some(ts) = state.table_stack.last_mut() {
                ts.in_thead = false;
            }
        }

        TagKind::TableBody | TagKind::TableFoot => {
            // Body/foot close: mark seen_tbody_close only for tbody.
            if matches!(spec.kind, TagKind::TableBody) {
                if let Some(ts) = state.table_stack.last_mut() {
                    ts.seen_tbody_close = true;
                }
            }
        }

        TagKind::TableRow => {
            // Commit current_row to rows (skip empty rows).
            if let Some(ts) = state.table_stack.last_mut() {
                if !ts.current_row.is_empty() {
                    let col_count = ts.current_row.len();
                    // Track first-row column count for consistency checking.
                    if ts.first_row_col_count.is_none() {
                        ts.first_row_col_count = Some(col_count);
                    }
                    let row = std::mem::take(&mut ts.current_row);
                    ts.rows.push(row);
                }
            }
        }

        TagKind::TableCell { .. } => {
            if let Some(ts) = state.table_stack.last_mut() {
                ts.in_cell = false;
                // Trim the accumulated cell text (matches Tier-2 `text.trim()`).
                let cell_text = ts.current_cell.trim().to_owned();
                // Bail if the cell contains a newline (multi-line content).
                // Tier-2 replaces newlines with spaces, but Tier-1 only handles
                // single-line cells to guarantee byte-identical output.
                if cell_text.contains('\n') {
                    return Err(BailReason::TableBlockChildInCell);
                }
                // Bail if the cell contains a pipe: Tier-2 escapes `|` → `\|`
                // which changes the cell width computation; Tier-1 does not
                // implement pipe escaping.
                if cell_text.contains('|') {
                    return Err(BailReason::TableBlockChildInCell);
                }
                ts.current_row.push(cell_text);
                ts.current_cell.clear();
            }
        }

        TagKind::TableCaption => {
            // Should have been caught at open, but handle gracefully.
        }

        // Generic block and inline containers: no closing marker.
        TagKind::Block | TagKind::Inline => {}

        _ => {}
    }

    Ok(())
}

// ── Implicit close-tag emission ───────────────────────────────────────────────

/// Implicitly close the top-of-stack frame without a matching `</tag>` in the
/// input.  Called by the M4 implicit-close loop when HTML5 optional-tag rules
/// require an open element to be closed before the next tag is pushed.
///
/// Mirrors `emit_close` but skips the stack-pop search (we always close the
/// literal top frame) and skips the tag-name lookup (we use the frame's spec
/// directly).
fn emit_close_for_implicit(state: &mut Tier1State) -> Result<(), BailReason> {
    let frame = state.stack.pop().ok_or(BailReason::DepthMismatch {
        tag: String::from("(implicit)"),
        expected: 1,
        actual: 0,
    })?;
    let spec = frame.spec;

    // Restore escape context.
    state.escape_ctx = frame.prev_escape_ctx;

    match spec.kind {
        TagKind::Paragraph => {
            // Tier-2 appends "\n\n" after paragraph content (always two newlines).
            // Matching this precisely is required for byte-equal output.
            trim_trailing_inline_whitespace(state);
            state.output.push_str("\n\n");
        }

        TagKind::Heading(n) => {
            let prefix = heading_prefix(n);
            trim_trailing_inline_whitespace(state);
            state.output.insert_str(frame.content_start, &prefix);
            // Tier-2 leaves a blank line ("\n\n") after a heading. A following
            // paragraph's "\n\n" guard then finds it already and appends
            // nothing, yielding the expected single blank line.
            state.ensure_blank_line();
        }

        TagKind::Blockquote => {
            let content = state.output[frame.content_start..].to_owned();
            let prefixed = prefix_blockquote_lines(&content);
            state.output.truncate(frame.content_start);
            state.output.push_str(&prefixed);
        }

        TagKind::Pre => {
            let raw = state.output[frame.content_start..].to_owned();
            let indented = indent_pre_lines(&raw);
            state.output.truncate(frame.content_start);
            state.output.push_str(&indented);
        }

        TagKind::Strong => {
            state.cell_or_output_mut().push_str("**");
        }

        TagKind::Emphasis => {
            state.cell_or_output_mut().push('*');
        }

        TagKind::Code => {
            // When inside <pre>, <code> is transparent — no backtick markers.
            // The escape_ctx was already restored to prev_escape_ctx above.
            if !state.escape_ctx.contains(EscapeCtx::PRE) {
                state.cell_or_output_mut().push('`');
            }
        }

        TagKind::Link => {
            let dest = state.cell_or_output_mut();
            if let Some(href) = &frame.link_href {
                if let Some(title) = &frame.link_title {
                    dest.push_str(&format!("]({href} \"{title}\")"));
                } else {
                    dest.push_str(&format!("]({href})"));
                }
            } else {
                if let Some(bracket_pos) = dest[..frame.content_start].rfind('[') {
                    dest.remove(bracket_pos);
                }
            }
        }

        TagKind::List(_) => {
            state.list_depth = state.list_depth.saturating_sub(1);
            if !state.output.ends_with('\n') {
                state.output.push('\n');
            }
        }

        TagKind::ListItem => {
            trim_trailing_inline_whitespace(state);
            state.ensure_newline();
        }

        // ── Table implicit closes ──────────────────────────────────────────────

        TagKind::TableCell { .. } => {
            // Implicitly close a <td>/<th> — same logic as the explicit close.
            if let Some(ts) = state.table_stack.last_mut() {
                ts.in_cell = false;
                let cell_text = ts.current_cell.trim().to_owned();
                if cell_text.contains('\n') {
                    return Err(BailReason::TableBlockChildInCell);
                }
                ts.current_row.push(cell_text);
                ts.current_cell.clear();
            }
        }

        TagKind::TableRow => {
            // Implicitly close a <tr> — commit current_row to rows.
            if let Some(ts) = state.table_stack.last_mut() {
                if !ts.current_row.is_empty() {
                    let col_count = ts.current_row.len();
                    if ts.first_row_col_count.is_none() {
                        ts.first_row_col_count = Some(col_count);
                    }
                    let row = std::mem::take(&mut ts.current_row);
                    ts.rows.push(row);
                }
            }
        }

        // Generic block/inline: no closing marker.
        TagKind::Block | TagKind::Inline => {}

        _ => {}
    }

    Ok(())
}

// ── Text flushing ─────────────────────────────────────────────────────────────

/// Flush a raw HTML text segment into the output (or current cell buffer),
/// decoding entities and collapsing whitespace (unless inside `<pre>`).
///
/// `base_offset` is the byte offset of `raw` within the original HTML input;
/// it is forwarded to the entity decoder so that `BailReason::UnknownEntity`
/// carries an accurate position.
///
/// Returns `Err(BailReason::UnknownEntity)` if an unrecognised entity is found.
fn flush_text(state: &mut Tier1State, raw: &str, base_offset: usize) -> Result<(), BailReason> {
    if raw.is_empty() {
        return Ok(());
    }

    // Inside a table but outside a cell: discard text (whitespace between
    // structural tags like <table>...<tr> or <tr>...<td>).
    // Tier-2 processes only tag children explicitly, ignoring text nodes at
    // this level.
    if !state.table_stack.is_empty() && !state.in_table_cell() {
        return Ok(());
    }

    let in_pre = state.escape_ctx.contains(EscapeCtx::PRE);
    let has_entities = raw.contains('&');

    if in_pre {
        if has_entities {
            let dest = state.cell_or_output_mut();
            decode_entities_into(dest, raw, base_offset)?;
        } else {
            state.cell_or_output_mut().push_str(raw);
        }
        return Ok(());
    }

    // Outside `<pre>`: collapse runs of space/tab into a single space, decode
    // entities, write directly into the output (or cell) buffer.  Newlines preserved.
    let has_collapsible = has_double_ws(raw);
    if !has_entities && !has_collapsible {
        // Hot path: plain ASCII text with single-space whitespace already.
        state.cell_or_output_mut().push_str(raw);
        return Ok(());
    }

    let dest = state.cell_or_output_mut();
    decode_and_collapse_into(dest, raw, has_entities, base_offset)
}

/// True if `s` contains two consecutive whitespace bytes (space or tab) that
/// would collapse.
#[inline]
fn has_double_ws(s: &str) -> bool {
    let b = s.as_bytes();
    let mut i = 1;
    while i < b.len() {
        let a = b[i - 1];
        let c = b[i];
        if (a == b' ' || a == b'\t') && (c == b' ' || c == b'\t') {
            return true;
        }
        i += 1;
    }
    false
}

/// Decode HTML entities directly into `out` (no intermediate allocation).
///
/// `base_offset` is the byte offset of `s` within the original HTML input and
/// is used to report the position of any unrecognised entity in the bail reason.
///
/// Returns `Err(BailReason::UnknownEntity)` when an entity cannot be decoded.
fn decode_entities_into(out: &mut String, s: &str, base_offset: usize) -> Result<(), BailReason> {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'&' {
            // Push contiguous non-`&` run in one go using UTF-8 safety from `s`.
            let start = i;
            while i < bytes.len() && bytes[i] != b'&' {
                i += 1;
            }
            out.push_str(&s[start..i]);
            continue;
        }
        // Look for matching `;` within 32 bytes
        let amp = i;
        let mut end = i + 1;
        while end < bytes.len() && end - amp <= 32 && bytes[end] != b';' {
            end += 1;
        }
        if end < bytes.len() && bytes[end] == b';' && end > amp + 1 {
            let entity = &s[amp + 1..end];
            if decode_entity_into(out, entity) {
                i = end + 1;
                continue;
            }
            // Entity found (`&name;`) but not in the decode table or invalid
            // numeric reference — bail rather than silently passing it through.
            return Err(BailReason::UnknownEntity {
                name: entity.to_owned(),
                offset: base_offset + amp,
            });
        }
        out.push('&');
        i += 1;
    }
    Ok(())
}

/// Decode entities AND collapse spaces/tabs in one pass, directly into `out`.
///
/// `base_offset` is the byte offset of `s` within the original HTML input and
/// is used to report the position of any unrecognised entity in the bail reason.
///
/// Bulk-copies long runs of "ordinary" bytes (not space/tab/&) with a single
/// `push_str` to avoid the per-byte overhead of `push(char)`.
///
/// Returns `Err(BailReason::UnknownEntity)` when an entity cannot be decoded.
fn decode_and_collapse_into(
    out: &mut String,
    s: &str,
    has_entities: bool,
    base_offset: usize,
) -> Result<(), BailReason> {
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut prev_was_space = false;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b' ' || b == b'\t' {
            if !prev_was_space {
                out.push(' ');
            }
            prev_was_space = true;
            i += 1;
            continue;
        }
        if has_entities && b == b'&' {
            prev_was_space = false;
            let amp = i;
            let mut end = i + 1;
            while end < bytes.len() && end - amp <= 32 && bytes[end] != b';' {
                end += 1;
            }
            if end < bytes.len() && bytes[end] == b';' && end > amp + 1 {
                let entity = &s[amp + 1..end];
                if decode_entity_into(out, entity) {
                    i = end + 1;
                    continue;
                }
                // Entity found (`&name;`) but not in the decode table or invalid
                // numeric reference — bail rather than silently passing it through.
                return Err(BailReason::UnknownEntity {
                    name: entity.to_owned(),
                    offset: base_offset + amp,
                });
            }
            out.push('&');
            i += 1;
            continue;
        }
        // Bulk-copy a contiguous run of non-special bytes.
        let start = i;
        if has_entities {
            while i < bytes.len() {
                let bb = bytes[i];
                if bb == b' ' || bb == b'\t' || bb == b'&' {
                    break;
                }
                i += 1;
            }
        } else {
            while i < bytes.len() {
                let bb = bytes[i];
                if bb == b' ' || bb == b'\t' {
                    break;
                }
                i += 1;
            }
        }
        out.push_str(&s[start..i]);
        prev_was_space = false;
    }
    Ok(())
}

// ── Escape context management ─────────────────────────────────────────────────

fn update_escape_ctx(state: &mut Tier1State, spec: &TagSpec, is_open: bool) {
    // Handle <pre> specially: it sets both PRE and CODE bits.
    if spec.kind == TagKind::Pre {
        if is_open {
            state.escape_ctx |= EscapeCtx::PRE | EscapeCtx::CODE;
        } else {
            state.escape_ctx &= !(EscapeCtx::PRE | EscapeCtx::CODE);
        }
        return;
    }

    let bit = match spec.kind {
        TagKind::Code => EscapeCtx::CODE,
        TagKind::Link => EscapeCtx::LINK,
        TagKind::Blockquote => EscapeCtx::BLOCKQUOTE,
        TagKind::Heading(_) => EscapeCtx::HEADING,
        _ => return,
    };

    if is_open {
        state.escape_ctx |= bit;
    } else {
        state.escape_ctx &= !bit;
    }
}

// ── Attribute helpers ─────────────────────────────────────────────────────────

/// Find an attribute value by (lowercase) key name.
fn find_attr<'a>(attrs: &[(&'a [u8], Option<&'a [u8]>)], key: &[u8]) -> Option<&'a [u8]> {
    for (k, v) in attrs {
        if k.eq_ignore_ascii_case(key) {
            return *v;
        }
    }
    None
}

/// Extract `href` and `title` from the attribute list for a link.
fn extract_link_attrs(
    attrs: &[(&[u8], Option<&[u8]>)],
) -> Result<(Option<String>, Option<String>), BailReason> {
    let href = find_attr(attrs, b"href").map(decode_attr).transpose()?;
    let title = find_attr(attrs, b"title").map(decode_attr).transpose()?;
    Ok((href, title))
}

/// Extract `start` attribute from `<ol>` (defaults to 1).
fn extract_ol_start(attrs: &[(&[u8], Option<&[u8]>)]) -> u16 {
    find_attr(attrs, b"start")
        .and_then(|b| std::str::from_utf8(b).ok())
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(1)
}

/// Decode an attribute value: entity-decode and convert to a String.
///
/// Returns `Err(BailReason::UnknownEntity)` when the value contains an entity
/// that Tier-1 cannot decode (Tier-2 would decode it differently).
fn decode_attr(bytes: &[u8]) -> Result<String, BailReason> {
    let s = std::str::from_utf8(bytes).unwrap_or("");
    if !s.contains('&') {
        return Ok(s.to_owned());
    }
    let mut out = String::with_capacity(s.len());
    // Attribute values do not carry a meaningful offset into the HTML source;
    // use 0 as the base so the entity name is still reported.
    decode_entities_into(&mut out, s, 0)?;
    Ok(out)
}

// ── Stack helpers ─────────────────────────────────────────────────────────────

/// Pop the topmost frame whose spec matches `spec`.
/// Tier-2 is lenient about close tags; we are strict in M3c: only pop the
/// Pop the topmost frame whose spec matches `spec`.
///
/// We compare by checking if the `TagKind` on the top frame maps to the same
/// "semantic group" as the spec being closed.  We are strict in M3c: only the
/// top frame is checked to avoid mismatched-close-tag complexity.
fn pop_matching_frame(stack: &mut Vec<OpenTag>, spec: &'static TagSpec) -> Option<OpenTag> {
    let top = stack.last()?;
    if kinds_match(&top.spec.kind, &spec.kind) {
        stack.pop()
    } else {
        None
    }
}

/// Return `true` if two `TagKind` values are the "same" for close-tag matching.
///
/// Uses pointer equality on the `&'static TagSpec` where possible for speed.
/// For kinds with inner data (`List`, `Heading`, `TableCell`) we use a
/// coarser match that still prevents cross-kind confusion:
/// - `List(Ordered)` only matches `List(Ordered)`, etc.
/// - `Heading(n)` matches `Heading(m)` for any n, m (HTML allows `</h3>` to
///   close `<h2>` in some parsers; we are lenient for headings since they
///   do not nest in practice).
fn kinds_match(a: &TagKind, b: &TagKind) -> bool {
    match (a, b) {
        (TagKind::List(la), TagKind::List(lb)) => la == lb,
        (TagKind::Heading(_), TagKind::Heading(_)) => true,
        (TagKind::TableCell { is_header: a_h }, TagKind::TableCell { is_header: b_h }) => a_h == b_h,
        _ => std::mem::discriminant(a) == std::mem::discriminant(b),
    }
}

/// Find the nearest enclosing list kind by walking the stack top-to-bottom.
fn find_parent_list_kind(stack: &[OpenTag]) -> Option<ListKind> {
    for frame in stack.iter().rev() {
        if let TagKind::List(kind) = frame.spec.kind {
            return Some(kind);
        }
    }
    None
}

/// Increment the ordered-list counter on the nearest `List(Ordered)` frame.
/// Returns the new counter value (1-based).
fn increment_ol_counter(stack: &mut [OpenTag]) -> u16 {
    for frame in stack.iter_mut().rev() {
        if let TagKind::List(ListKind::Ordered) = frame.spec.kind {
            frame.list_index = frame.list_index.saturating_add(1);
            return frame.list_index;
        }
    }
    1
}

/// Get the `ol_start` value from the nearest `List(Ordered)` frame.
fn find_ol_start(stack: &[OpenTag]) -> u16 {
    for frame in stack.iter().rev() {
        if let TagKind::List(ListKind::Ordered) = frame.spec.kind {
            return frame.ol_start;
        }
    }
    1
}

// ── Formatting helpers ────────────────────────────────────────────────────────

/// Return the ATX heading prefix for level `n` (1–6).
fn heading_prefix(n: u8) -> String {
    let n = n.min(6) as usize;
    let mut s = "#".repeat(n);
    s.push(' ');
    s
}

/// Compute the indentation string for a list item at the given depth.
/// Depth 0 → no indent; depth 1 → 2 spaces; etc.
fn list_item_indent(depth: u16) -> String {
    "  ".repeat(depth as usize)
}

/// Add `> ` prefix to every non-empty line of `content`, and `>` to empty
/// lines that are between non-empty ones (Tier-2 behaviour for multi-paragraph
/// blockquotes).
fn prefix_blockquote_lines(content: &str) -> String {
    // Trim trailing newline before splitting, so we don't produce a trailing `> `
    let content = content.trim_end_matches('\n');
    if content.is_empty() {
        return String::new();
    }

    let lines: Vec<&str> = content.split('\n').collect();
    let mut result = String::with_capacity(content.len() + lines.len() * 2);

    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            // Between paragraphs inside a blockquote: emit `>`
            result.push('>');
        } else {
            result.push_str("> ");
            result.push_str(line);
        }
        if i < lines.len() - 1 {
            result.push('\n');
        }
    }
    result.push('\n');
    result
}

/// Indent each line of a pre block by 4 spaces, after dedenting common leading whitespace.
///
/// Mirrors Tier-2's `dedent_code_block` + 4-space indent logic.
fn indent_pre_lines(raw: &str) -> String {
    // Strip leading newline (the newline right after `<pre>`)
    let raw = raw.strip_prefix('\n').unwrap_or(raw);
    // Strip trailing newline
    let raw = raw.trim_end_matches('\n');
    if raw.is_empty() {
        return String::new();
    }

    // Dedent: find minimum leading whitespace among non-empty lines.
    let min_indent = raw
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
        .min()
        .unwrap_or(0);

    let mut result = String::with_capacity(raw.len() + raw.lines().count() * 4);
    for line in raw.lines() {
        result.push_str("    ");
        if line.trim().is_empty() {
            // Preserve empty lines without adding trailing spaces.
        } else {
            result.push_str(&line[min_indent.min(line.len())..]);
        }
        result.push('\n');
    }
    result
}

// ── GFM table emission ────────────────────────────────────────────────────────

/// Emit a completed table as GFM markdown, appending to `state.output`.
///
/// Format must match Tier-2 `convert_table_row` byte-for-byte:
/// - Each row: `|` + ` cell |` per cell → `| c1 | c2 |`
/// - After row 0: separator `| --- | --- |` (one `---` per column)
///
/// # Panics
///
/// Never — empty-table guard returns early.
fn emit_gfm_table(state: &mut Tier1State, ts: crate::converter::tier1::state::TableState) {
    if ts.rows.is_empty() {
        return;
    }

    // Pre-table separator: mirrors Tier-2's `convert_table` logic exactly.
    // Tier-2 (block/table/mod.rs): `if !output.is_empty() && !output.ends_with("\n\n")`
    // — only adds separator when there is existing output (no leading blank lines).
    if !state.output.is_empty() && !state.output.ends_with("\n\n") {
        if state.output.ends_with('\n') {
            state.output.push('\n');
        } else {
            state.output.push_str("\n\n");
        }
    }

    // Pre-compute max column widths across ALL rows (mirrors Tier-2's pre-pass).
    // Tier-2: separator dashes = max(col_content_char_count_across_all_rows, 3).
    let col_count = ts.rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let mut col_widths: Vec<usize> = vec![0; col_count];
    for row in &ts.rows {
        for (i, cell) in row.iter().enumerate() {
            let w = cell.chars().count();
            if w > col_widths[i] {
                col_widths[i] = w;
            }
        }
    }

    for (row_index, row) in ts.rows.iter().enumerate() {
        // Row: `|` then each cell as ` text |` (padded to col_width like Tier-2).
        state.output.push('|');
        for (i, cell) in row.iter().enumerate() {
            state.output.push(' ');
            state.output.push_str(cell);
            // Pad to column width (mirrors Tier-2 cell.rs padding logic).
            let cell_len = cell.chars().count();
            let col_w = col_widths.get(i).copied().unwrap_or(0);
            for _ in cell_len..col_w {
                state.output.push(' ');
            }
            state.output.push_str(" |");
        }
        state.output.push('\n');

        // After row 0 (the header row), emit the separator row.
        // Tier-2: col_widths.get(i).unwrap_or(0).max(MIN_SEPARATOR_DASHES)
        // where MIN_SEPARATOR_DASHES = 3.
        if row_index == 0 {
            state.output.push_str("| ");
            for i in 0..col_count.max(1) {
                if i > 0 {
                    state.output.push_str(" | ");
                }
                let dash_count = col_widths.get(i).copied().unwrap_or(0).max(3);
                for _ in 0..dash_count {
                    state.output.push('-');
                }
            }
            state.output.push_str(" |\n");
        }
    }
}

/// Trim trailing spaces and tabs from the end of the output (used before
/// closing block elements that trim trailing whitespace in Tier-2).
fn trim_trailing_inline_whitespace(state: &mut Tier1State) {
    while state.output.ends_with(' ') || state.output.ends_with('\t') {
        state.output.pop();
    }
}

/// Collapse runs of 3+ consecutive newlines down to 2, matching Tier-2's
/// `collapse_excess_blank_lines` post-processing step.
fn collapse_excess_blank_lines(output: &mut String) {
    let mut cleaned = String::with_capacity(output.len());
    let mut consecutive = 0usize;
    for ch in output.chars() {
        if ch == '\n' {
            consecutive += 1;
            if consecutive <= 2 {
                cleaned.push(ch);
            }
        } else {
            consecutive = 0;
            cleaned.push(ch);
        }
    }
    *output = cleaned;
}

// ── HTML entity decoding ──────────────────────────────────────────────────────

/// Decode a single HTML entity name (without `&` or `;`) directly into `out`.
///
/// Returns `true` when the entity was recognized and written; `false` when the
/// name didn't match any known entity (caller emits the literal `&...;`).
///
/// All named entities are static strings; numeric references emit a single
/// `char`. No `String` is allocated.
fn decode_entity_into(out: &mut String, name: &str) -> bool {
    let s: &str = match name {
        "amp" => "&",
        "lt" => "<",
        "gt" => ">",
        "quot" => "\"",
        "apos" => "'",
        "nbsp" => "\u{00A0}",
        "copy" => "\u{00A9}",
        "reg" => "\u{00AE}",
        "trade" => "\u{2122}",
        "mdash" => "\u{2014}",
        "ndash" => "\u{2013}",
        "hellip" => "\u{2026}",
        "laquo" => "\u{00AB}",
        "raquo" => "\u{00BB}",
        "lsquo" => "\u{2018}",
        "rsquo" => "\u{2019}",
        "ldquo" => "\u{201C}",
        "rdquo" => "\u{201D}",
        "prime" => "\u{2032}",
        "Prime" => "\u{2033}",
        "bull" => "\u{2022}",
        "middot" => "\u{00B7}",
        "deg" => "\u{00B0}",
        "plusmn" => "\u{00B1}",
        "times" => "\u{00D7}",
        "divide" => "\u{00F7}",
        "frac12" => "\u{00BD}",
        "frac14" => "\u{00BC}",
        "frac34" => "\u{00BE}",
        "euro" => "\u{20AC}",
        "pound" => "\u{00A3}",
        "yen" => "\u{00A5}",
        "cent" => "\u{00A2}",
        "larr" => "\u{2190}",
        "rarr" => "\u{2192}",
        "uarr" => "\u{2191}",
        "darr" => "\u{2193}",
        "harr" => "\u{2194}",
        "infin" => "\u{221E}",
        "alpha" => "\u{03B1}",
        "beta" => "\u{03B2}",
        "gamma" => "\u{03B3}",
        "delta" => "\u{03B4}",
        "pi" => "\u{03C0}",
        "sigma" => "\u{03C3}",
        "omega" => "\u{03C9}",
        _ => return decode_numeric_entity_into(out, name),
    };
    out.push_str(s);
    true
}

fn decode_numeric_entity_into(out: &mut String, name: &str) -> bool {
    let Some(rest) = name.strip_prefix('#') else {
        return false;
    };
    let code_point = if rest.starts_with('x') || rest.starts_with('X') {
        match u32::from_str_radix(&rest[1..], 16) {
            Ok(n) => n,
            Err(_) => return false,
        }
    } else {
        match rest.parse::<u32>() {
            Ok(n) => n,
            Err(_) => return false,
        }
    };
    match char::from_u32(code_point) {
        Some(ch) => {
            out.push(ch);
            true
        }
        None => false,
    }
}

// ── Comment / DOCTYPE skipping ────────────────────────────────────────────────

/// Skip `<!--...-->`, `<!DOCTYPE...>`, or any `<!...>` construct.
/// Returns the position immediately after the closing `>`.
///
/// On failure returns `Err(BailReason::LiteralLt)`.
fn skip_bang(bytes: &[u8], pos: usize) -> Result<usize, BailReason> {
    // pos points at `<`, pos+1 is `!`
    let start = pos + 2; // byte after `!`

    if bytes.get(start) == Some(&b'-') && bytes.get(start + 1) == Some(&b'-') {
        // HTML comment: `<!-- ... -->`
        let comment_start = start + 2;
        let mut i = comment_start;
        while i + 2 < bytes.len() {
            if bytes[i] == b'-' && bytes[i + 1] == b'-' && bytes[i + 2] == b'>' {
                return Ok(i + 3);
            }
            i += 1;
        }
        // Unclosed comment — bail
        return Err(BailReason::LiteralLt { offset: pos });
    }

    // DOCTYPE or similar `<!...>` — skip to `>`
    let mut i = start;
    while i < bytes.len() {
        if bytes[i] == b'>' {
            return Ok(i + 1);
        }
        i += 1;
    }
    Err(BailReason::LiteralLt { offset: pos })
}

// ── Misc helpers ──────────────────────────────────────────────────────────────

/// Convert tag name bytes to lowercase in a fixed-size stack buffer.
/// Returns a slice into `buf`.  If the name is longer than `buf`, it is
/// truncated (names > 32 bytes won't appear in the spec table anyway and
/// will be rejected as unknown).
fn lowercase_into<'b>(bytes: &[u8], buf: &'b mut [u8; 32]) -> &'b [u8] {
    let len = bytes.len().min(32);
    for (i, &b) in bytes[..len].iter().enumerate() {
        buf[i] = b.to_ascii_lowercase();
    }
    &buf[..len]
}

/// Convert a byte slice to an owned `String` (lossy UTF-8).
fn bytes_to_string(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}
