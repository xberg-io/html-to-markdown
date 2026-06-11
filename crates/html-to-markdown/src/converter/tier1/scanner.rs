//! Tier-1 single-pass byte scanner.
//!
//! Walks `html.as_bytes()` once and emits Markdown directly to a pre-sized
//! output buffer.  On any construct it cannot handle exactly, returns a
//! [`BailReason`] so the dispatcher can fall back to Tier-2.
//!
//! # Supported subset (M9 + Phase E + Phase I)
//!
//! Paragraph, Heading(1-6), Strong, Emphasis, Code (inline), Pre, Hr,
//! `LineBreak`, Link, Image, List(Unordered), List(Ordered), `ListItem`,
//! Blockquote, Block (div/section/article/center/etc.), Inline (span/etc.),
//! Table (GFM — conservative bail set, inline-only cell content),
//! SVG (emitted as base64 data URI — Phase I), and custom elements (tag names
//! containing `-`, treated as Block containers).
//!
//! Bails on: RawText(script/style/textarea/etc.), `DefinitionTerm`,
//! `DefinitionDescription`, List(Definition), Ignored (head/meta/link),
//! nested tables, non-inlineable block children in cells (heading/list/blockquote/pre),
//! section-order violations, and any HTML construct with in-text whitespace
//! complexity or unclosed tags.

use crate::converter::tier1::bail::BailReason;
use crate::converter::tier1::parse;
use crate::converter::tier1::spec_rules;
use crate::converter::tier1::state::{EscapeCtx, OpenTag, Tier1State};
use crate::converter::tier1::tags::{ListKind, TagKind, TagSpec};
use crate::converter::tier1::{self};
use crate::converter::utility::attributes::NAV_KEYWORDS;
use crate::options::ConversionOptions;

use memchr::{memchr2, memchr3};

/// Maximum byte length of a tag name lowercased into a stack buffer.
///
/// Names longer than this are silently truncated and will not match any
/// entry in the spec table, causing an `UnknownCustomElement` bail.
const MAX_TAG_NAME_BYTES: usize = 32;

/// Maximum byte length scanned when looking for a `;` to close an entity.
///
/// Entities longer than this are treated as bare `&` literals.
const MAX_ENTITY_NAME_BYTES: usize = 32;

/// Minimum number of dashes in a GFM separator cell.
///
/// Matches Tier-2's `col_widths.get(i).unwrap_or(0).max(MIN_SEPARATOR_DASHES)`.
const MIN_SEPARATOR_DASHES: usize = 3;

/// Static `TagSpec` used for all unknown custom elements (tag names containing
/// `-`, e.g. `<x-foo>`, `<my-component>`).
///
/// Tier-2 treats unknown custom elements as generic block containers and emits
/// their inner content as plain text.  Using a `Block` spec here produces
/// byte-identical output to Tier-2 for the common cases where custom-element
/// content is plain text or standard HTML children.
///
/// The static reference `&CUSTOM_ELEMENT_BLOCK_SPEC` is used anywhere the
/// scanner needs a `&'static TagSpec` for a custom element open/close tag.
static CUSTOM_ELEMENT_BLOCK_SPEC: TagSpec = TagSpec {
    kind: TagKind::Block,
    is_void: false,
    is_block: true,
    optional_close: None,
    is_rawtext: false,
};

/// ATX heading prefixes indexed by level − 1 (0 = `h1`, 5 = `h6`).
const HEADING_PREFIXES: [&str; 6] = ["# ", "## ", "### ", "#### ", "##### ", "###### "];

/// List-item indentation strings indexed by depth (0 = top-level, no indent).
///
/// Depths beyond the table size fall back to a runtime allocation.
const LIST_ITEM_INDENTS: [&str; 8] = [
    "",
    "  ",
    "    ",
    "      ",
    "        ",
    "          ",
    "            ",
    "              ",
];

/// Successful output of the Tier-1 scanner.
#[derive(Debug, Clone, Default)]
pub struct ScanOutput {
    /// Accumulated Markdown body.
    pub body: String,
    /// Byte range of `<head>…</head>` content (if a `<head>` was seen) in
    /// the input the scanner walked.  Forwarded by `tier1::run` to
    /// `head_metadata::extract_frontmatter` so the YAML frontmatter step
    /// works without a `PrescanReport`.
    pub head_range: Option<std::ops::Range<usize>>,
}

/// Entry point for the Tier-1 scanner.
pub fn scan(html: &str, options: &ConversionOptions) -> Result<ScanOutput, BailReason> {
    let bytes = html.as_bytes();
    let mut state = Tier1State::new(html.len());
    // Phase DD: Tier-2 runs an html5ever roundtrip when custom-element
    // tags are present in the source, which canonicalizes attribute
    // entities.  Mirror that for byte-equality.
    state.canonicalize_attr_entities = crate::converter::main_helpers::has_custom_element_tags(html);
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

                // `<?` — processing instruction.  Tier-2 handles these
                // inconsistently depending on whether html5ever-repair
                // ran (it rewrites bogus comments) and how tl chooses
                // to parse the run.  Either way the byte shape
                // downstream differs from the simple skip Tier-1 could
                // perform, so bail and let the Tier-2 fallback produce
                // the authoritative output.
                if next == b'?' {
                    return Err(BailReason::Classifier);
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
                    emit_close(&mut state, tag_name_bytes, options)?;

                    pos = close_bracket.0 + 1;
                    text_start = pos;
                    continue;
                }

                // Not a tag-name-start byte → literal `<` in text. Tier-2
                // emits these verbatim (html5ever/astral-tl both parse a
                // bare `<x` as a text node). Emit the `<` and continue so
                // we don't bail on commonly-unescaped source like `x < 5`.
                if !parse::is_tag_name_start(next) {
                    flush_text(&mut state, "<", pos)?;
                    pos += 1;
                    text_start = pos;
                    continue;
                }

                // Opening tag
                let name_start = pos + 1;
                let name_end = parse::scan_tag_name(bytes, name_start);
                let tag_name_bytes = &bytes[name_start..name_end];

                // Lowercase the tag name into a stack buffer (max 32 bytes)
                let mut name_buf = [0u8; MAX_TAG_NAME_BYTES];
                let name_lower = lowercase_into(tag_name_bytes, &mut name_buf);

                // Phase I: `<svg>` — emit as base64 data URI matching Tier-2's
                // `handle_svg` output.  The entire subtree (open tag through
                // `</svg>`) is consumed here; the scanner skips past it without
                // pushing anything on the open-tag stack.
                //
                // `tl::parse` is called on just the SVG fragment to normalize
                // attribute order via `serialize_element` (which sorts attrs
                // alphabetically — raw source bytes differ, so slicing alone is
                // not byte-identical with Tier-2 output).
                if name_lower == b"svg" {
                    let tag_open_start = pos;
                    let Some((close_pos, is_self_closing)) = parse::find_tag_close(bytes, name_end) else {
                        // Unclosed SVG open tag — skip to end; Tier-2 handles it.
                        pos = bytes.len();
                        text_start = pos;
                        continue;
                    };
                    let open_tag_end = close_pos + 1;

                    let svg_end = if is_self_closing {
                        // `<svg ... />` — self-closing, no children.
                        open_tag_end
                    } else {
                        // Find matching `</svg>` with depth tracking.
                        find_svg_close(bytes, open_tag_end).unwrap_or(bytes.len())
                    };

                    let svg_slice = &html[tag_open_start..svg_end];

                    emit_svg_from_slice(svg_slice, tag_open_start, &mut state, options)?;

                    pos = svg_end;
                    text_start = pos;
                    continue;
                }

                // Phase N: `<template>` — inert script container; Tier-2 drops
                // its content (see plain_text.rs SKIP_TAGS).  Skip the entire
                // subtree without emitting anything.  Self-closing form is rare
                // but handled.
                if name_lower == b"template" {
                    let Some((close_pos, is_self_closing)) = parse::find_tag_close(bytes, name_end) else {
                        pos = bytes.len();
                        text_start = pos;
                        continue;
                    };
                    let open_tag_end = close_pos + 1;
                    pos = if is_self_closing {
                        open_tag_end
                    } else {
                        find_balanced_close(bytes, open_tag_end, b"template").unwrap_or(bytes.len())
                    };
                    text_start = pos;
                    continue;
                }

                // Resolve the tag spec.  Custom elements (names containing `-`)
                // are not in the static TAGS table but are treated as generic
                // block containers — Tier-2 emits their inner content as plain
                // block text, which matches `TagKind::Block` behaviour.  All
                // other unknown tags are still bailed immediately.
                let spec: &'static TagSpec = if name_lower.contains(&b'-') {
                    &CUSTOM_ELEMENT_BLOCK_SPEC
                } else {
                    match tier1::lookup(name_lower) {
                        Some(s) => s,
                        None => {
                            return Err(BailReason::UnknownCustomElement {
                                name: bytes_to_string(tag_name_bytes).into(),
                                offset: pos,
                            });
                        }
                    }
                };

                // Raw-text "ignored" tags (`<script>`, `<style>`): their
                // spec is `TagKind::Ignored` with `is_rawtext = true` (see
                // tags.rs `rawtext_ignored`).  Prescan also strips their
                // content (STRIP_CONTENT_TAGS); Tier-2 does the same.  Skip
                // them inline so we don't bail to Tier-2 just because a page
                // contains an empty `<script></script>` left over from
                // prescan.  Other `RawText` kinds (textarea / title / xmp /
                // iframe / noscript / noembed / noframes) keep their text
                // content in Tier-2 and must continue to bail until Tier-1
                // learns to emit it correctly.
                if matches!(spec.kind, TagKind::Ignored) && spec.is_rawtext {
                    let open_end = match parse::find_tag_close(bytes, name_end) {
                        Some(close) => close.0 + 1,
                        None => bytes.len(),
                    };
                    pos = find_raw_text_close(bytes, open_end, name_lower).unwrap_or(bytes.len());
                    text_start = pos;
                    continue;
                }

                // Non-rawtext `Ignored` tags (`<head>`, `<meta>`, `<link>`):
                // Tier-2 does not emit any markdown from their bodies — head
                // is consumed by metadata extraction; meta/link are void.
                // Silent-skip them here so Tier-1 can be invoked on inputs
                // that contain a `<head>` (the common case for full HTML
                // documents) without bailing.  For non-void `<head>`, capture
                // the content range on `state.head_range` so `tier1::run` can
                // hand it to `head_metadata::extract_frontmatter` when
                // metadata extraction is enabled.
                if matches!(spec.kind, TagKind::Ignored) {
                    let open_end = match parse::find_tag_close(bytes, name_end) {
                        Some(close) => close.0 + 1,
                        None => bytes.len(),
                    };
                    if spec.is_void {
                        pos = open_end;
                        text_start = pos;
                        continue;
                    }
                    // Non-void: scan for matching close tag, record the
                    // [open_end .. close_start] range as head content.
                    let (close_start, close_end) = match find_close_tag_range(bytes, open_end, name_lower) {
                        Some(pair) => pair,
                        None => (bytes.len(), bytes.len()),
                    };
                    if state.head_range.is_none() {
                        state.head_range = Some(open_end..close_start);
                    }
                    pos = close_end;
                    text_start = pos;
                    continue;
                }

                // Bail on unsupported tag kinds for M3c
                bail_unsupported(spec, pos)?;

                // Phase D': mirror Tier-2's preprocessing pipeline <nav> /
                // nav-hinted <header> / <footer> / <aside> / <form> strip.
                // When the user's preprocessing options request the strip,
                // jump past the matching close tag without pushing any frame.
                // Matches Tier-2's should_drop_for_preprocessing
                // (preprocessing_helpers.rs:115).
                if is_preprocessing_skip_candidate(name_lower) {
                    let close = parse::find_tag_close(bytes, name_end).ok_or(BailReason::LiteralLt { offset: pos })?;
                    let attrs_end = if close.1 { close.0.saturating_sub(1) } else { close.0 };
                    let skip_attrs = parse::collect_attrs(bytes, name_end, attrs_end);
                    if should_skip_preprocessing(name_lower, &skip_attrs, options) {
                        let open_end = close.0 + 1;
                        if close.1 {
                            // Self-closing void — nothing to skip over.
                            pos = open_end;
                        } else {
                            pos = find_balanced_close(bytes, open_end, name_lower).unwrap_or(bytes.len());
                        }
                        text_start = pos;
                        continue;
                    }
                    // Not skipped — fall through to the regular open path.
                    // The regular path will call find_tag_close again (cheap)
                    // and re-collect attrs only for kinds that need them;
                    // nav/header/footer/aside/form are Block kind so attrs
                    // won't be re-collected.
                }

                // Bail on <pre> when code_block_style is not Indented.
                // Phase Q.4: Tier-1 supports Indented (4-space) and
                // Backticks (`` ``` ``-fenced) code blocks via open_pre /
                // close_pre.  Tildes still require Tier-2's fence emitter.
                if matches!(spec.kind, TagKind::Pre)
                    && options.code_block_style == crate::options::CodeBlockStyle::Tildes
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
                // actually consult attributes.  `<abbr>` is `TagKind::Inline`
                // but its `title` attribute is read at open time to mirror
                // Tier-2's `handle_abbr` — include it in the collect-set.
                let needs_attrs = matches!(
                    spec.kind,
                    TagKind::Link
                        | TagKind::Image
                        | TagKind::List(ListKind::Ordered)
                        | TagKind::TableCell { .. }
                        | TagKind::Pre
                        | TagKind::Code
                ) || name_lower == b"abbr";
                let attrs: Vec<(&[u8], Option<&[u8]>)> = if needs_attrs {
                    parse::collect_attrs(bytes, name_end, attrs_end)
                } else {
                    Vec::new()
                };

                pos = close.0 + 1;

                // Void or self-closing: emit immediately, don't push stack
                if spec.is_void || close.1 {
                    emit_void(&mut state, spec, &attrs, html, options)?;
                    text_start = pos;
                    continue;
                }

                // Non-void open tag: emit opening markdown + push stack frame

                // Phase HH: nested tables are NO LONGER bailed here.  An inner
                // table is opened with `inline_mode = true` (set inside
                // `open_table`), and on `</table>` the rendered GFM markdown
                // is written into the parent cell buffer rather than
                // `state.output`.  The parent cell's newline-collapse step
                // then flattens the inner table to a single inline run,
                // matching Tier-2's behaviour.

                // M4: HTML5 implicit-close transitions.
                // Run BEFORE the block-in-cell check so that structural table
                // elements like `<tr>` correctly close any open `<td>`/`<th>`
                // before the block check evaluates `in_table_cell()`.  Without
                // this ordering, `<th>h1<tr>` would fire the bail even though
                // `<tr>` is not a content element inside the cell.
                while let Some(top) = state.stack.last() {
                    if !spec_rules::should_close_for_new_tag(top.spec, spec) {
                        break;
                    }
                    emit_close_for_implicit(&mut state, options)?;
                }

                // M9: Block-in-cell bail.
                // Evaluated AFTER M4 implicit closes so that table-structural
                // elements (e.g. a `<tr>` following an unclosed `<th>`) correctly
                // collapse the cell state before the check runs.
                //
                // Allow `<p>`, `<div>/<section>/…` (TagKind::Block), `<ul>/<ol>`,
                // and `<h1>-<h6>` inside cells — each has cell-aware open/close
                // helpers that redirect their output to the cell accumulator and
                // match Tier-2's `cell_text_content` normalisation
                // (`text.replace('\n', " ")` when `br_in_tables` is false).
                //
                // All other block kinds (blockquote, pre, etc.) still bail because
                // they produce multi-line content that would diverge from Tier-2's
                // cell normalisation.
                if state.in_table_cell() && spec.is_block {
                    let inlineable = matches!(
                        spec.kind,
                        TagKind::Paragraph
                            | TagKind::Block
                            | TagKind::Summary
                            | TagKind::Figcaption
                            | TagKind::Blockquote
                            | TagKind::Pre
                            | TagKind::List(_)
                            | TagKind::ListItem
                            | TagKind::Heading(_)
                            | TagKind::DefinitionTerm
                            | TagKind::DefinitionDescription
                            | TagKind::Table
                    );
                    if !inlineable {
                        return Err(BailReason::TableBlockChildInCell);
                    }
                }

                let prev_ctx = state.escape_ctx;
                let ol_start = if matches!(spec.kind, TagKind::List(ListKind::Ordered)) {
                    extract_ol_start(&attrs)
                } else {
                    1
                };
                if matches!(spec.kind, TagKind::Link) {
                    let (href, title) = extract_link_attrs(&attrs)?;
                    state.link_stack.push((href, title));
                }
                // Mirror Tier-2's `semantic/attributes.rs::handle_abbr`:
                // capture the abbreviation's `title` attribute and emit
                // `" (title)"` after the abbr's text content at close time.
                if name_lower == b"abbr" {
                    let title = find_attr(&attrs, b"title")
                        .and_then(|b| std::str::from_utf8(b).ok())
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(str::to_owned);
                    state.abbr_titles.push(title);
                }

                emit_open(&mut state, spec, &attrs)?;

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
                    ol_start,
                    name_range: name_start..name_end,
                });

                // Update escape context after pushing so the frame records the
                // pre-tag ctx correctly.
                apply_open_escape_ctx(&mut state, spec);

                text_start = pos;
            }
            _ => {
                // Batch ASCII fast-path: skip forward to the next `<` or `&`
                // (the only two bytes that require special handling) in one
                // memchr2 call instead of advancing one byte at a time.
                // flush_text handles entity decoding and whitespace collapsing
                // for whatever raw slice [text_start..pos] we hand it, so it
                // is correct to jump pos all the way to the next special byte.
                // This is safe across every context (<pre>, table cells, etc.)
                // because:
                //   • `<` still triggers the tag-dispatch path above.
                //   • `&` is preserved in the slice passed to flush_text, which
                //     entity-decodes it correctly regardless of context.
                //   • Raw-text elements (script/style/textarea/…) bail before
                //     reaching this arm, so we never skip inside them.
                match memchr2(b'<', b'&', &bytes[pos..]) {
                    Some(offset) if offset > 0 => pos += offset,
                    Some(_) => pos += 1,       // next byte is already special; advance past current
                    None => pos = bytes.len(), // no more special bytes; jump to end
                }
            }
        }
    }

    // Flush any trailing text after the last tag.
    if text_start < pos {
        flush_text(&mut state, &html[text_start..pos], text_start)?;
    }

    // Phase N2: implicitly close all remaining open elements at EOF.
    // HTML5 parsers (html5ever and tl) close every open element when input
    // ends, so Tier-2 produces output even for malformed input like
    // `<p>hello <b>world` (no `</b>`, no `</p>`).  Mirror that here by
    // running emit_close_for_implicit on every remaining frame, regardless
    // of whether it has an OptionalCloseRule.
    //
    // Before closing, trim trailing inline whitespace (spaces, tabs, newlines)
    // from the output buffer.  In well-formed HTML the close tag arrives
    // before the file's trailing newline; the inline close-marker emission
    // (e.g. `**` for `</strong>`) lands flush against the content.  At EOF
    // any trailing newline is between the implicit close and the file end,
    // not inside the inline body, so we trim it before pushing the close
    // marker to match Tier-2's `world**` instead of `world\n**`.
    while !state.stack.is_empty() {
        let buf = &mut state.output;
        while matches!(buf.as_bytes().last(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            buf.pop();
        }
        emit_close_for_implicit(&mut state, options)?;
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

    Ok(ScanOutput {
        body: state.output,
        head_range: state.head_range,
    })
}

// ── Bail guard ────────────────────────────────────────────────────────────────

/// Return `Err(BailReason::Classifier)` for tag kinds not supported in M9.
///
/// Table-related tags are now handled by the scanner (M9); they are no longer
/// bailed here.  Table-specific bail reasons are emitted by the table-handling
/// code in `emit_open` and `emit_close`.
/// Locate the matching close tag for `tag_name` starting at `open_end`.
///
/// Returns `Some((close_start, close_end))` where `close_start` is the byte
/// index of the `<` opening the `</tag>` close and `close_end` is the byte
/// index immediately after its `>`.  `None` when no matching close exists.
///
/// Used by `<head>` silent-skip to record the content slice
/// (`open_end..close_start`) for metadata extraction while advancing past the
/// entire `<head>…</head>` block.
fn find_close_tag_range(bytes: &[u8], open_end: usize, tag_name: &[u8]) -> Option<(usize, usize)> {
    let len = bytes.len();
    let mut idx = open_end;
    while idx < len {
        match memchr3(b'<', b'<', b'<', &bytes[idx..]) {
            Some(off) => idx += off,
            None => return None,
        }
        if idx + 2 < len && bytes[idx + 1] == b'/' {
            let after_slash = idx + 2;
            if after_slash + tag_name.len() <= len
                && bytes[after_slash..after_slash + tag_name.len()].eq_ignore_ascii_case(tag_name)
            {
                let post_name = after_slash + tag_name.len();
                if matches!(bytes.get(post_name), Some(b'>' | b'/' | b' ' | b'\t' | b'\n' | b'\r')) {
                    let mut j = post_name;
                    while j < len && bytes[j] != b'>' {
                        j += 1;
                    }
                    if j < len {
                        return Some((idx, j + 1));
                    }
                    return None;
                }
            }
        }
        idx += 1;
    }
    None
}

// ── SVG helpers ───────────────────────────────────────────────────────────────

/// Find the byte offset immediately after the matching `</svg>` close tag,
/// starting from `open_end` (the byte after the `>` of the opening `<svg ...>`).
///
/// Tracks nesting depth so nested `<svg>` elements (valid in SVG 1.1) are
/// handled correctly.  Returns `None` when no matching close is found.
fn find_svg_close(bytes: &[u8], open_end: usize) -> Option<usize> {
    find_balanced_close(bytes, open_end, b"svg")
}

/// Find the byte offset immediately after the matching close tag for
/// `tag_name`, starting from `open_end` (the byte after the `>` of the
/// opening tag).  Tracks nesting depth so nested same-name elements are
/// handled correctly.  Returns `None` when no matching close is found.
fn find_balanced_close(bytes: &[u8], open_end: usize, tag_name: &[u8]) -> Option<usize> {
    let len = bytes.len();
    let mut idx = open_end;
    let mut depth = 1usize;
    while idx < len {
        match memchr::memchr(b'<', &bytes[idx..]) {
            Some(off) => idx += off,
            None => return None,
        }
        if idx + 1 < len && bytes[idx + 1] == b'/' {
            let name_start = idx + 2;
            if name_start + tag_name.len() <= len
                && bytes[name_start..name_start + tag_name.len()].eq_ignore_ascii_case(tag_name)
            {
                let after = name_start + tag_name.len();
                if matches!(
                    bytes.get(after),
                    Some(b'>' | b'/' | b' ' | b'\t' | b'\n' | b'\r') | None
                ) {
                    depth -= 1;
                    if depth == 0 {
                        let mut j = after;
                        while j < len && bytes[j] != b'>' {
                            j += 1;
                        }
                        return Some(j + 1);
                    }
                }
            }
        } else if idx + 1 < len {
            let name_start = idx + 1;
            if name_start + tag_name.len() <= len
                && bytes[name_start..name_start + tag_name.len()].eq_ignore_ascii_case(tag_name)
            {
                let after = name_start + tag_name.len();
                if matches!(
                    bytes.get(after),
                    Some(b'>' | b'/' | b' ' | b'\t' | b'\n' | b'\r') | None
                ) {
                    // Walk to end of the open tag to determine self-closing.
                    let mut j = after;
                    let mut in_q: Option<u8> = None;
                    let tag_end = loop {
                        if j >= len {
                            break len;
                        }
                        match bytes[j] {
                            b'"' | b'\'' => {
                                if let Some(q) = in_q {
                                    if q == bytes[j] {
                                        in_q = None;
                                    }
                                } else {
                                    in_q = Some(bytes[j]);
                                }
                            }
                            b'>' if in_q.is_none() => {
                                break j + 1;
                            }
                            _ => {}
                        }
                        j += 1;
                    };
                    // Self-closing (`/>`) does not increment depth.
                    let is_self_closing = tag_end >= 2 && bytes[tag_end - 2] == b'/';
                    if !is_self_closing {
                        depth += 1;
                    }
                }
            }
        }
        idx += 1;
    }
    None
}

/// Emit a `<svg>` element as a Markdown base64 data URI, matching Tier-2's
/// `handle_svg` output byte-for-byte.
///
/// `svg_slice` is the raw HTML source bytes for the entire `<svg…>…</svg>`
/// element.  We re-parse it with `tl::parse` to get the canonical attribute
/// order that `serialize_element` produces (it sorts attributes alphabetically,
/// so raw-source slicing would diverge from Tier-2).
///
/// Mirrors Tier-2's `media/svg.rs::handle_svg`:
/// - Walks children for a `<title>` tag → alt text.  Default: "SVG Image".
/// - Calls `serialize_element` on the root SVG node.
/// - Base64-encodes (STANDARD engine) the serialized bytes.
/// - Emits `![{title}](data:image/svg+xml;base64,{b64})`.
/// - When `options.skip_images` → emits nothing (matches Tier-2 skip).
fn emit_svg_from_slice(
    svg_slice: &str,
    svg_start_offset: usize,
    state: &mut Tier1State,
    options: &ConversionOptions,
) -> Result<(), BailReason> {
    // CDATA inside SVG cannot be processed correctly without the prescan's
    // entity-escaping transformation.  Bail to Tier-2 so it sees the
    // prescan-normalized form (where `<![CDATA[` is escaped to `&lt;![CDATA[`).
    if svg_slice.contains("<![CDATA[") {
        return Err(BailReason::Cdata {
            offset: svg_start_offset,
        });
    }

    if options.skip_images {
        return Ok(());
    }

    use crate::converter::media::svg::serialize_element;
    use base64::{Engine as _, engine::general_purpose::STANDARD};

    // Re-parse just the SVG fragment.  Wrap it in a minimal document so
    // tl has proper context — the same pattern used by head_metadata.rs.
    let wrapped = format!("<html><body>{svg_slice}</body></html>");
    let dom = match tl::parse(&wrapped, tl::ParserOptions::default()) {
        Ok(d) => d,
        Err(_) => {
            // Parse failure: emit nothing rather than bail — matches
            // Tier-2's silent skip on serialization failure.
            return Ok(());
        }
    };
    let parser = dom.parser();

    // Locate the first `<svg>` node in the parsed fragment.
    let svg_handle = dom.nodes().iter().enumerate().find_map(|(i, node)| {
        if let tl::Node::Tag(tag) = node {
            if tag.name().as_utf8_str().as_ref().eq_ignore_ascii_case("svg") {
                Some(tl::NodeHandle::new(i as u32))
            } else {
                None
            }
        } else {
            None
        }
    });

    let Some(handle) = svg_handle else {
        return Ok(());
    };

    // Extract title from a direct `<title>` child, mirroring Tier-2.
    let title = if let Some(tl::Node::Tag(svg_tag)) = handle.get(parser) {
        let mut found = String::from("SVG Image");
        for child_handle in svg_tag.children().top().iter() {
            if let Some(tl::Node::Tag(child)) = child_handle.get(parser) {
                if child.name().as_utf8_str().as_ref().eq_ignore_ascii_case("title") {
                    // Collect text content of the <title> tag.
                    let mut text = String::new();
                    for grandchild in child.children().top().iter() {
                        if let Some(tl::Node::Raw(raw)) = grandchild.get(parser) {
                            text.push_str(&raw.as_utf8_str());
                        }
                    }
                    let trimmed = text.trim().to_owned();
                    if !trimmed.is_empty() {
                        found = trimmed;
                    }
                    break;
                }
            }
        }
        found
    } else {
        String::from("SVG Image")
    };

    let svg_html = serialize_element(&handle, parser);
    let base64_svg = STANDARD.encode(svg_html.as_bytes());

    let dest = state.cell_or_output_mut();
    dest.push_str("![");
    dest.push_str(&title);
    dest.push_str("](data:image/svg+xml;base64,");
    dest.push_str(&base64_svg);
    dest.push(')');

    Ok(())
}

/// Skip the body of a raw-text element (script/style/textarea/iframe/…).
///
/// `open_end` is the byte index immediately after the tag's `>`.  `tag_name`
/// is the lowercased open-tag name.  Returns the byte index after the
/// matching `</tag>` close, or `None` if no matching close tag exists in the
/// remainder of the input.
///
/// Mirrors the prescan's STRIP_CONTENT_TAGS handling: content is discarded,
/// only the position advances.  Matches Tier-2's behaviour byte-for-byte
/// because Tier-2 sees this content already stripped by the prescan.
fn find_raw_text_close(bytes: &[u8], open_end: usize, tag_name: &[u8]) -> Option<usize> {
    let len = bytes.len();
    let mut idx = open_end;
    while idx < len {
        // memchr to skip ahead to the next `<`.
        match memchr3(b'<', b'<', b'<', &bytes[idx..]) {
            Some(off) => idx += off,
            None => return None,
        }
        // Need `</tag` to start a closing tag.
        if idx + 2 < len && bytes[idx + 1] == b'/' {
            let after_slash = idx + 2;
            if after_slash + tag_name.len() <= len
                && bytes[after_slash..after_slash + tag_name.len()].eq_ignore_ascii_case(tag_name)
            {
                let post_name = after_slash + tag_name.len();
                // Tag name must be followed by `>` / whitespace / `/`.
                if matches!(bytes.get(post_name), Some(b'>' | b'/' | b' ' | b'\t' | b'\n' | b'\r')) {
                    // Walk to the closing `>`.
                    let mut j = post_name;
                    while j < len && bytes[j] != b'>' {
                        j += 1;
                    }
                    if j < len {
                        return Some(j + 1);
                    }
                    return None;
                }
            }
        }
        idx += 1;
    }
    None
}

#[inline]
const fn bail_unsupported(spec: &TagSpec, _offset: usize) -> Result<(), BailReason> {
    match spec.kind {
        // Raw-text content tags are handled inline by the main scan loop
        // (see find_raw_text_close).  They never reach this point in practice;
        // listed here only to make the match exhaustive over TagKind::RawText.
        TagKind::RawText(_) => Err(BailReason::Classifier),

        // `Ignored` tags (head/meta/link/script/style) are now handled inline
        // by the main scan loop (see the dispatch above `bail_unsupported`).
        // The match arm is kept for exhaustiveness — it cannot fire in
        // practice.
        TagKind::Ignored => Err(BailReason::Classifier),

        _ => Ok(()),
    }
}

// ── Open-tag emission ─────────────────────────────────────────────────────────

fn emit_open(
    state: &mut Tier1State,
    spec: &'static TagSpec,
    attrs: &[(&[u8], Option<&[u8]>)],
) -> Result<(), BailReason> {
    // Phase V: when a block-level tag opens inside a link, bail.  Tier-2's
    // link handler collapses block children (img alt, paragraph text) into
    // an inline link label; replicating that in Tier-1 requires content
    // capture similar to Phase R's summary buffer.  Until that lands, bail
    // so Tier-2's fallback handles the collapse.
    if matches!(
        spec.kind,
        TagKind::Block
            | TagKind::Paragraph
            | TagKind::Heading(_)
            | TagKind::Blockquote
            | TagKind::Pre
            | TagKind::List(_)
            | TagKind::Table
    ) && state.stack.iter().any(|f| matches!(f.spec.kind, TagKind::Link))
    {
        return Err(BailReason::Classifier);
    }
    match spec.kind {
        TagKind::Paragraph => open_paragraph(state),
        TagKind::Heading(_) => open_heading(state),
        TagKind::Blockquote => open_blockquote(state),
        TagKind::Pre => open_pre(state, attrs),
        TagKind::List(ListKind::Definition) => open_dl(state),
        TagKind::List(kind) => open_list(state, kind),
        TagKind::ListItem => open_list_item(state),
        TagKind::DefinitionTerm => open_dt(state),
        TagKind::DefinitionDescription => open_dd(state),
        TagKind::Strong => {
            // Inside a <summary> accumulation buffer, Tier-2 processes
            // children with `in_strong: true` which suppresses nested
            // strong markers.  Mirror that by not pushing `**` when inside
            // a summary, so `<strong>b</strong>` inside `<summary>` emits
            // just `b` instead of `**b**`.
            // Phase FF-2: figcaption uses the same buffer stack but
            // Tier-2 does NOT set in_strong for figcaption children, so
            // emit `**` normally when the topmost wrap-buf is a
            // figcaption (or there's no wrap-buf at all).
            if !state.summary_at_top() {
                state.cell_or_output_mut().push_str("**");
            }
        }
        TagKind::Emphasis => {
            state.cell_or_output_mut().push('*');
        }
        TagKind::Strikethrough => {
            // Tier-2's handle_strikethrough suppresses the `~~` wrapping
            // when inside `<code>`/`<pre>` (in_code).  Mirror via EscapeCtx.
            if !state.escape_ctx.contains(EscapeCtx::CODE) && !state.escape_ctx.contains(EscapeCtx::PRE) {
                state.cell_or_output_mut().push_str("~~");
            }
        }
        TagKind::Inserted => {
            // Tier-2's handle_inserted emits `==` markers unconditionally for
            // <ins>.  Mirror Strikethrough's in-code/pre suppression for
            // consistency (no `==` inside backtick spans / fenced blocks).
            if !state.escape_ctx.contains(EscapeCtx::CODE) && !state.escape_ctx.contains(EscapeCtx::PRE) {
                state.cell_or_output_mut().push_str("==");
            }
        }
        // Phase CC: defer the open backtick marker — close_code does
        // smart escaping based on the content (mirrors Tier-2's
        // render_code_with_escaping at inline/code.rs:260).  Inside an
        // outer <code> or <pre>, the inner code is transparent.
        TagKind::Code if !state.escape_ctx.contains(EscapeCtx::PRE) && !state.escape_ctx.contains(EscapeCtx::CODE) => {}
        // When inside <pre>, a nested <code class="language-X"> contributes
        // the language tag for the enclosing fence.
        TagKind::Code if state.pre_lang.is_none() && state.escape_ctx.contains(EscapeCtx::PRE) => {
            if let Some(lang) = extract_language_from_class(attrs) {
                state.pre_lang = Some(lang);
            }
        }
        TagKind::Link => open_link(state),
        TagKind::Table => open_table(state),
        TagKind::TableCaption => open_table_caption(state),
        TagKind::TableHead => open_table_head(state)?,
        TagKind::TableBody => open_table_body(state)?,
        TagKind::TableFoot => open_table_foot(state),
        TagKind::TableRow => open_table_row(state),
        TagKind::TableCell { is_header } => open_table_cell(state, attrs, is_header)?,
        // Block containers: emit a leading blank-line separator when there's
        // already preceding content.  Mirrors Tier-2's div/sectioning handlers
        // (block/div.rs:88, semantic/sectioning.rs:71) which prefix block
        // content with `\n\n` to separate from siblings.
        //
        // Inside a table cell, Tier-2's div.rs:60 treats a sibling-div as a
        // "table continuation" and emits `"  \n"` (two-space + newline) when
        // the cell already has non-`|`/non-`<br>` content.  After
        // `close_table_cell`'s `replace('\n', ' ')` step, this becomes a 3-space
        // run between sibling divs — matching Tier-2's lists_timeline cell
        // layout `[link]   [other-link]`.  Without this, Tier-1 emits 1 space.
        TagKind::Block => {
            if state.in_table_cell() {
                let cell_buf = state.cell_or_output_mut();
                if !cell_buf.is_empty()
                    && !cell_buf.ends_with('|')
                    && !cell_buf.ends_with("<br>")
                    && !cell_buf.ends_with("  \n")
                {
                    // Trim trailing horizontal whitespace, then append the
                    // table-continuation line break.  Mirrors div.rs:75-85.
                    while cell_buf.ends_with(' ') || cell_buf.ends_with('\t') {
                        cell_buf.pop();
                    }
                    cell_buf.push_str("  \n");
                }
            } else {
                state.ensure_blank_line();
            }
        }
        // Summary: push accumulation buffer so children redirect into it (Phase R).
        TagKind::Summary => open_summary(state),
        // Figcaption: same buffer mechanism as summary (Phase FF-2); the
        // wrap delimiter differs (`*…*` vs `**…**`) and is emitted by
        // close_figcaption.
        TagKind::Figcaption => open_figcaption(state),
        // Button: no leading separator (matches Tier-2 handle_button which
        // does nothing on open).  Close-side `\n\n` is emitted by close_button.
        TagKind::Button => {}
        TagKind::Inline => {}
        // All other kinds (LineBreak, Hr, Image, etc.) are void — they never
        // reach emit_open because the void/self-closing branch fires first.
        _ => {}
    }

    Ok(())
}

// ── Per-TagKind open helpers ──────────────────────────────────────────────────

fn open_paragraph(state: &mut Tier1State) {
    // When inside a table cell, treat `<p>` as a transparent container.
    // Tier-2's paragraph.rs emits `<br>` when `in_table_cell` and there is
    // already cell content; we mirror that behaviour so the cell buffer stays
    // on one logical line (no `\n` in cell output to collapse later).
    if state.in_table_cell() {
        let cell_buf = state.cell_or_output_mut();
        if !cell_buf.is_empty() && !cell_buf.ends_with("<br>") {
            cell_buf.push_str("<br>");
        }
        return;
    }
    // Mirrors Tier-2: when output is non-empty and doesn't already end
    // with "\n\n", push "\n\n" (may produce three newlines total when
    // output ends with a single "\n", e.g. right after a table row or
    // an `<hr>`).
    let dest = state.cell_or_output_mut();
    // Phase EE: when the paragraph is the first child of a list-item
    // (output ends with a freshly-emitted bullet like `- ` or `1. `),
    // the paragraph content joins the bullet inline.  Tier-2's
    // paragraph.rs achieves this by checking the parent and skipping
    // the leading `\n\n` for the first block child.  Check BEFORE
    // `trim_trailing_horizontal`, which would strip the trailing
    // space from the bullet.
    if dest.ends_with("- ") || dest.ends_with("* ") || dest.ends_with("+ ") || ends_with_ordered_marker(dest) {
        return;
    }
    // Drop trailing horizontal whitespace from inter-tag preservation
    // (Phase U-2) before the block separator.
    crate::converter::tier1::state::trim_trailing_horizontal(dest);
    if !dest.is_empty() && !dest.ends_with("\n\n") {
        dest.push_str("\n\n");
    }
}

fn open_heading(state: &mut Tier1State) {
    // When inside a table cell, Tier-2 does NOT add a leading separator before
    // the heading (`needs_leading_sep = false` when `in_table_cell`).  The
    // heading text is emitted directly into the cell accumulator with no `#`
    // prefix and no surrounding newlines.
    if state.in_table_cell() {
        return;
    }
    state.ensure_blank_line();
}

fn open_blockquote(state: &mut Tier1State) {
    state.ensure_blank_line();
}

fn open_pre(state: &mut Tier1State, attrs: &[(&[u8], Option<&[u8]>)]) {
    state.ensure_blank_line();
    // Capture language from <pre class="language-X">; nested <code class="...">
    // can still override later if pre had no language hint.
    if let Some(lang) = extract_language_from_class(attrs) {
        state.pre_lang = Some(lang);
    }
}

/// Extract the language tag from a `class` attribute matching `language-X`
/// or `lang-X`.  Mirrors Tier-2's `extract_language_from_pre`.
fn extract_language_from_class(attrs: &[(&[u8], Option<&[u8]>)]) -> Option<String> {
    let class_bytes = find_attr(attrs, b"class")?;
    let class = std::str::from_utf8(class_bytes).ok()?;
    for cls in class.split_ascii_whitespace() {
        if let Some(rest) = cls.strip_prefix("language-") {
            return Some(rest.to_owned());
        }
        if let Some(rest) = cls.strip_prefix("lang-") {
            return Some(rest.to_owned());
        }
    }
    None
}

fn open_list(state: &mut Tier1State, kind: ListKind) {
    // When inside a table cell, mirror Tier-2's `add_list_leading_separator`:
    // push `<br>` if there is already cell content (but not if it already ends
    // with `|`, ` `, or `<br>`).  Do not touch `state.output`.
    if state.in_table_cell() {
        let cell_buf = state.cell_or_output_mut();
        if !cell_buf.is_empty() && !cell_buf.ends_with('|') && !cell_buf.ends_with(' ') && !cell_buf.ends_with("<br>") {
            cell_buf.push_str("<br>");
        }
        state.list_depth = state.list_depth.saturating_add(1);
        if matches!(kind, ListKind::Unordered) {
            state.ul_depth = state.ul_depth.saturating_add(1);
        }
        return;
    }
    // Lists at the top level need a blank line if there's preceding content.
    // Inside a list item (`list_depth > 0`) just a newline is enough.
    let current_list_depth = state.list_depth;
    {
        let dest = state.cell_or_output_mut();
        // Drop trailing horizontal whitespace from inter-tag preservation
        // (Phase U-2) before the block separator.
        crate::converter::tier1::state::trim_trailing_horizontal(dest);
        if !dest.is_empty() {
            if current_list_depth == 0 {
                // Top-level list: ensure blank line before
                if !dest.ends_with("\n\n") {
                    if dest.ends_with('\n') {
                        dest.push('\n');
                    } else {
                        dest.push_str("\n\n");
                    }
                }
            } else {
                // Nested list inside a list item: just newline
                if !dest.ends_with('\n') {
                    dest.push('\n');
                }
            }
        }
    }
    state.list_depth = state.list_depth.saturating_add(1);
    if matches!(kind, ListKind::Unordered) {
        state.ul_depth = state.ul_depth.saturating_add(1);
    }
}

/// Cycle through the canonical default `options.bullets` value (`"-*+"`) by
/// `<ul>` nesting depth.  The router (`router.rs::classify`) gates Tier-1 to
/// the literal default, so this hardcoded cycle reproduces Tier-2 byte-for-byte.
const TIER1_BULLETS: [u8; 3] = [b'-', b'*', b'+'];

fn open_list_item(state: &mut Tier1State) {
    // When inside a table cell, Tier-2 does NOT emit bullet/number prefixes
    // for list items (see list/item.rs: `if !ctx.in_table_cell { ... bullet ... }`).
    // The cell accumulator already receives the raw text; separators are handled
    // by the `\n` → ` ` replacement at cell-close time.
    if state.in_table_cell() {
        // For ordered lists we still need to increment the counter so that
        // subsequent items get the right index if we ever exit the cell context.
        if find_parent_list_kind(&state.stack) == Some(ListKind::Ordered) {
            increment_ol_counter(&mut state.stack);
        }
        return;
    }
    // Emit the list item marker.
    let parent_kind = find_parent_list_kind(&state.stack);
    let indent_depth = state.list_depth.saturating_sub(1);
    if parent_kind == Some(ListKind::Ordered) {
        // Increment counter on parent ordered list frame
        let counter = increment_ol_counter(&mut state.stack);
        let start = find_ol_start(&state.stack);
        let index = start.saturating_sub(1) + counter;
        push_list_item_indent(&mut state.output, indent_depth);
        // measured: write! is slower on this workload (Stage 5c)
        #[allow(clippy::format_push_string)]
        state.output.push_str(&format!("{index}. "));
    } else {
        push_list_item_indent(&mut state.output, indent_depth);
        let bullet_idx = state.ul_depth.saturating_sub(1) as usize % TIER1_BULLETS.len();
        state.output.push(TIER1_BULLETS[bullet_idx] as char);
        state.output.push(' ');
    }
}

fn open_link(state: &mut Tier1State) {
    // Track link count inside tables for layout-table detection.
    if let Some(ts) = state.table_stack.last_mut() {
        ts.link_count += 1;
    }
    state.cell_or_output_mut().push('[');
}

fn open_table(state: &mut Tier1State) {
    // Phase HH: nested tables are no longer a bail; an inner table inherits
    // `inline_mode = true` so its final GFM rendering writes into the parent
    // cell buffer rather than `state.output`.  The parent cell's newline
    // collapse then flattens the inner table to a single inline run.
    let inline_mode = !state.table_stack.is_empty();
    state.table_stack.push(crate::converter::tier1::state::TableState {
        inline_mode,
        ..Default::default()
    });
}

fn open_table_caption(state: &mut Tier1State) {
    if let Some(ts) = state.table_stack.last_mut() {
        ts.caption_buf.clear();
        ts.in_caption = true;
    }
}

fn open_table_head(state: &mut Tier1State) -> Result<(), BailReason> {
    if let Some(ts) = state.table_stack.last_mut() {
        // thead after any body or foot section → section order violation.
        if ts.seen_tbody_close || ts.seen_tfoot {
            return Err(BailReason::TableSectionOrder);
        }
        ts.in_thead = true;
    }
    Ok(())
}

fn open_table_body(state: &mut Tier1State) -> Result<(), BailReason> {
    if let Some(ts) = state.table_stack.last_mut() {
        // tbody after tfoot open → section order violation.
        if ts.seen_tfoot {
            return Err(BailReason::TableSectionOrder);
        }
    }
    Ok(())
}

fn open_table_foot(state: &mut Tier1State) {
    if let Some(ts) = state.table_stack.last_mut() {
        ts.seen_tfoot = true;
    }
}

fn open_table_row(state: &mut Tier1State) {
    // Clear the in-progress row.
    if let Some(ts) = state.table_stack.last_mut() {
        ts.current_row.clear();
    }
}

fn open_table_cell(
    state: &mut Tier1State,
    attrs: &[(&[u8], Option<&[u8]>)],
    is_header: bool,
) -> Result<(), BailReason> {
    // rowspan: accepted but not expanded (lossy — a spanned cell renders once,
    // matching mdream).  colspan: expanded by `close_table_cell` adding
    // `(colspan - 1)` empty cells so Tier-2's column-count expectations are
    // met (without this, infobox-style `<th colspan="2">` rows trigger Tier-2's
    // layout-table fallback in close_table on what should be a normal GFM table).
    let colspan = find_attr(attrs, b"colspan")
        .and_then(|b| std::str::from_utf8(b).ok())
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(1)
        .max(1);
    if let Some(ts) = state.table_stack.last_mut() {
        ts.current_cell.clear();
        ts.in_cell = true;
        ts.current_cell_colspan = colspan;
        if is_header {
            ts.has_th = true;
        }
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
            {
                let dest = state.cell_or_output_mut();
                if !dest.is_empty() && !dest.ends_with("\n\n") {
                    if dest.ends_with('\n') {
                        dest.push('\n');
                    } else {
                        dest.push_str("\n\n");
                    }
                }
            }
            state.cell_or_output_mut().push_str("---\n");
        }

        TagKind::LineBreak => {
            // `<br>` outside any block context emits nothing (Tier-2 behaviour).
            // Three context-dependent emissions:
            //   - Inside a link (anywhere): one space.  Tier-2's
            //     `normalize_link_label` (utility/content.rs ~145) collapses
            //     whitespace runs in link labels, so multiple spaces or
            //     `  \n` would normalize back to one space.
            //   - Inside a table cell (not in a link): emit a single sentinel
            //     `\u{0001}`.  `close_table_cell` collapses whitespace runs
            //     before normalising the sentinel to three spaces — matches
            //     Tier-2 walking `<br>` to `"  \n"` and `replace('\n', ' ')`
            //     producing `"   "` (three spaces).
            //   - Inside a regular block (paragraph, div, etc.): `"  \n"`.
            let in_link = state.stack.iter().any(|f| matches!(f.spec.kind, TagKind::Link));
            if in_link {
                state.cell_or_output_mut().push(' ');
            } else if state.in_table_cell() {
                state.cell_or_output_mut().push('\u{0001}');
            } else if state.stack.is_empty() {
                // bare `<br>` at top level — Tier-2 emits nothing
            } else {
                state.cell_or_output_mut().push_str("  \n");
            }
        }

        TagKind::Image => {
            let src = find_attr(attrs, b"src").unwrap_or_default();
            let alt = find_attr(attrs, b"alt").unwrap_or_default();
            let title = find_attr(attrs, b"title");

            // Phase DD: src gets entity-decoding (URL semantics).
            // For alt/title:
            //   • With custom-element tags → T2 ran html5ever roundtrip
            //     and canonicalized entities; decode + re-encode the
            //     special set to match.
            //   • Without → T2 just yields tl's raw attribute bytes;
            //     keep entities verbatim.
            let src = decode_attr(src)?;
            let canonicalize = state.canonicalize_attr_entities;
            let alt_owned;
            let alt: &str = if canonicalize {
                alt_owned = canonicalize_attr_entities(&decode_attr(alt)?).into_owned();
                &alt_owned
            } else {
                std::str::from_utf8(alt).map_err(|_| BailReason::Classifier)?
            };

            let keep_as_markdown = should_keep_image_as_markdown(html, &state.stack, options);

            let dest = state.cell_or_output_mut();
            if keep_as_markdown {
                if let Some(title_bytes) = title {
                    let title_owned;
                    let title_str: &str = if canonicalize {
                        title_owned = canonicalize_attr_entities(&decode_attr(title_bytes)?).into_owned();
                        &title_owned
                    } else {
                        std::str::from_utf8(title_bytes).map_err(|_| BailReason::Classifier)?
                    };
                    // measured: write! is slower on this workload (Stage 5c)
                    #[allow(clippy::format_push_string)]
                    dest.push_str(&format!("![{alt}]({src} \"{title_str}\")"));
                } else {
                    // measured: write! is slower on this workload (Stage 5c)
                    #[allow(clippy::format_push_string)]
                    dest.push_str(&format!("![{alt}]({src})"));
                }
            } else {
                // Strip to alt-text only — mirrors Tier-2 behaviour when the image
                // is in a heading whose tag is not in `keep_inline_images_in`.
                dest.push_str(alt);
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

fn emit_close(state: &mut Tier1State, tag_name_bytes: &[u8], options: &ConversionOptions) -> Result<(), BailReason> {
    // Lowercase the tag name to look it up in the spec table.
    let mut name_buf = [0u8; MAX_TAG_NAME_BYTES];
    let name_lower = lowercase_into(tag_name_bytes, &mut name_buf);

    // Custom element close tags (e.g. `</x-foo>`) use the same static Block
    // spec as their corresponding open tag.  All other unknown close tags bail.
    let spec: &'static TagSpec = if name_lower.contains(&b'-') {
        &CUSTOM_ELEMENT_BLOCK_SPEC
    } else {
        match tier1::lookup(name_lower) {
            Some(s) => s,
            None => {
                return Err(BailReason::UnknownCustomElement {
                    name: bytes_to_string(tag_name_bytes).into(),
                    offset: 0,
                });
            }
        }
    };

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
            emit_close_for_implicit(state, options)?;
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
        TagKind::Paragraph => close_paragraph(state),
        TagKind::Heading(n) => close_heading(state, &frame, n, false)?,
        TagKind::Blockquote => close_blockquote(state, &frame),
        TagKind::Pre => close_pre(state, &frame, options),
        // Strong: suppress close marker when inside summary (see open strong guard).
        TagKind::Strong if state.summary_at_top() => {}
        TagKind::Strong => close_inline_marker(state, &frame, "**"),
        TagKind::Emphasis => close_inline_marker(state, &frame, "*"),
        // Strikethrough: also transparent inside <code>/<pre>; mirrors open guard.
        TagKind::Strikethrough
            if state.escape_ctx.contains(EscapeCtx::CODE) || state.escape_ctx.contains(EscapeCtx::PRE) => {}
        TagKind::Strikethrough => close_inline_marker(state, &frame, "~~"),
        TagKind::Inserted
            if state.escape_ctx.contains(EscapeCtx::CODE) || state.escape_ctx.contains(EscapeCtx::PRE) => {}
        TagKind::Inserted => close_inline_marker(state, &frame, "=="),
        TagKind::Code => close_code(state, &frame),
        TagKind::Link => close_link(state, &frame),
        TagKind::List(ListKind::Definition) => close_dl(state, &frame),
        TagKind::List(kind) => close_list(state, kind),
        TagKind::ListItem => close_list_item(state, &frame),
        TagKind::DefinitionTerm => close_dt(state),
        TagKind::DefinitionDescription => close_dd(state),
        TagKind::Hr => {
            // Should not happen (hr is void), but handle gracefully.
        }
        TagKind::Table => close_table(state)?,
        TagKind::TableHead => close_table_head(state),
        TagKind::TableBody => close_table_body(state),
        TagKind::TableFoot => {
            // tfoot close: no action needed beyond restoring the frame (done above).
        }
        TagKind::TableRow => close_table_row(state),
        TagKind::TableCell { .. } => close_table_cell(state, false)?,
        TagKind::TableCaption => close_table_caption(state),
        // Generic block container close: when it produced visible content,
        // ensure a paragraph-break separator follows so the next sibling
        // doesn't run together with this div's last byte.  Mirrors Tier-2's
        // `div::handle` post-children block: `output.push_str("\n\n")` when
        // `has_content` (see block/div.rs around line 124-130).
        TagKind::Block => close_block_container(state, &frame),
        // Summary: pop accumulation buffer, trim, emit `**…**\n\n` (Phase R).
        TagKind::Summary => close_summary(state, &frame),
        // Figcaption: pop accumulation buffer, trim, emit `*…*\n\n` (Phase FF-2).
        TagKind::Figcaption => close_figcaption(state, &frame),
        // Button (Phase T): emit `\n\n` when content was produced — mirrors
        // Tier-2 `form/elements.rs:592-594`.  No leading separator on open.
        TagKind::Button => close_button(state, &frame),
        // Inline containers (span/etc.): no separator.  For `<abbr>` pop the
        // title side-stack and emit ` (title)` after the abbr's text content.
        TagKind::Inline => {
            if name_lower == b"abbr" {
                if let Some(Some(title)) = state.abbr_titles.pop() {
                    let dest = state.cell_or_output_mut();
                    dest.push_str(" (");
                    dest.push_str(&title);
                    dest.push(')');
                }
            }
        }
        // Void-only kinds that never have open frames:
        TagKind::LineBreak | TagKind::Image => {}
        // Explicitly no-op: all remaining known kinds not listed above.
        TagKind::RawText(_) | TagKind::Ignored => {}
    }

    Ok(())
}

/// Append a paragraph-break separator after a generic block container close
/// (`<div>`, `<section>`, etc.) when it produced visible content.
///
/// Without this Tier-1 emits adjacent block content with no separator
/// (e.g. `[image-link](href)EN` instead of `[image-link](href)\n\nEN`),
/// diverging from Tier-2 which always emits `\n\n` after a block-with-content
/// close (see Tier-2 `block/div.rs`).  Skipped inside table cells and inline
/// contexts where the surrounding code already handles spacing.
fn close_block_container(state: &mut Tier1State, frame: &OpenTag) {
    if state.in_table_cell() {
        return;
    }
    let buf = state.cell_or_output_mut();
    if buf.len() <= frame.content_start {
        // Empty block — no content emitted, no separator needed.
        return;
    }
    // Drop trailing horizontal whitespace (left over from inter-tag whitespace
    // preservation) before emitting the block separator.  Same rationale as
    // `ensure_blank_line` (Phase U-2).
    while buf.ends_with(' ') || buf.ends_with('\t') {
        buf.pop();
    }
    if buf.ends_with("\n\n") {
        return;
    }
    if buf.ends_with('\n') {
        buf.push('\n');
    } else {
        buf.push_str("\n\n");
    }
}

// ── Summary strong-wrap (Phase R) ────────────────────────────────────────────

/// Open a `<summary>` element.
///
/// Push a fresh accumulation buffer so all child text collects here instead
/// of in the outer destination (main output, table cell, or caption).
/// The summary buffer has the highest priority in `cell_or_output_mut`, so
/// even when inside a table cell the children write to this buffer rather
/// than the cell buffer.  This matches Tier-2's `handle_summary` which
/// always processes children into a local `content` buffer then wraps with
/// `**…**\n\n` before writing to the outer output.
///
/// No leading separator is emitted on open; deferred to `close_summary`
/// once we know whether the content is non-empty.
fn open_summary(state: &mut Tier1State) {
    state.push_summary_buf(crate::converter::tier1::state::WrapKind::Summary);
}

/// Close a `<summary>` element.
///
/// Pops the accumulation buffer (if any), trims it, and emits
/// `**{trimmed}**\n\n` into the parent destination (main output, an outer
/// summary buffer, a table cell, or a caption).
///
/// Mirrors Tier-2's `handle_summary` (semantic/summary.rs:138–249):
/// - collect children with `in_strong: true` (block children render inline)
/// - trim
/// - emit `**…**\n\n`
fn close_summary(state: &mut Tier1State, _frame: &OpenTag) {
    // Pop the buffer we pushed in open_summary.
    let buf = match state.pop_summary_buf() {
        Some(b) => b,
        None => return, // malformed nesting — nothing pushed
    };
    let trimmed = buf.trim();
    if trimmed.is_empty() {
        return;
    }
    // Acquire the parent destination.  Because we already popped the buffer
    // above, cell_or_output_mut now returns the next-outer target — which may
    // be the table cell buffer (when the summary was inside a <td>), an outer
    // summary buffer, or the main output.
    //
    // Check whether we're emitting into a table cell BEFORE borrowing `dest`,
    // so we can decide whether to add a leading separator without conflicting
    // with the mutable borrow.
    let writing_to_cell = state.in_table_cell();
    let dest = state.cell_or_output_mut();
    // Ensure a blank-line separator before the summary block when there is
    // preceding content and we're NOT writing to a table cell (cells are
    // rendered to a single line; block separators would be collapsed anyway).
    if !writing_to_cell && !dest.is_empty() && !dest.ends_with("\n\n") {
        if dest.ends_with('\n') {
            dest.push('\n');
        } else {
            dest.push_str("\n\n");
        }
    }
    dest.push_str("**");
    dest.push_str(trimmed);
    dest.push_str("**\n\n");
}

// ── Figcaption italic-wrap (Phase FF-2) ──────────────────────────────────────

/// Open a `<figcaption>` element.
///
/// Reuses the summary accumulation buffer stack — children write into it,
/// `close_figcaption` pops + wraps with `*…*\n\n` (vs Summary's `**…**`).
fn open_figcaption(state: &mut Tier1State) {
    state.push_summary_buf(crate::converter::tier1::state::WrapKind::Figcaption);
}

/// Close a `<figcaption>` element.
///
/// Mirrors Tier-2's `semantic/figure.rs::handle_figcaption`:
/// - collect children into a local buffer
/// - trim
/// - prepend single-space-or-blank-line separator
/// - emit `*{trimmed}*\n\n`
///
/// An empty/whitespace-only caption emits nothing (Tier-2 returns early).
fn close_figcaption(state: &mut Tier1State, _frame: &OpenTag) {
    let buf = match state.pop_summary_buf() {
        Some(b) => b,
        None => return,
    };
    let trimmed = buf.trim();
    if trimmed.is_empty() {
        return;
    }
    let writing_to_cell = state.in_table_cell();
    let dest = state.cell_or_output_mut();
    // Phase FF-2: trim trailing horizontal whitespace introduced by
    // Phase U-2's inter-tag-whitespace preservation, so the block
    // separator (\n\n) doesn't sit after a stray space.  Tier-2 does
    // not emit that space when the figcaption follows inline content.
    while dest.ends_with(' ') || dest.ends_with('\t') {
        dest.pop();
    }
    if !writing_to_cell && !dest.is_empty() && !dest.ends_with("\n\n") {
        if dest.ends_with('\n') {
            dest.push('\n');
        } else {
            dest.push_str("\n\n");
        }
    }
    dest.push('*');
    dest.push_str(trimmed);
    dest.push_str("*\n\n");
}

/// Close a `<button>` (Phase T).  When the button produced visible content,
/// emit `\n\n` after.  Skipped in table cells (cells stay one logical line).
///
/// Mirrors Tier-2 `form/elements.rs:592-594`:
/// ```text
/// if !ctx.convert_as_inline && output.len() > start_len {
///     output.push_str("\n\n");
/// }
/// ```
fn close_button(state: &mut Tier1State, frame: &OpenTag) {
    if state.in_table_cell() {
        return;
    }
    let dest = state.cell_or_output_mut();
    if dest.len() <= frame.content_start {
        return;
    }
    // Drop trailing horizontal whitespace from the inter-tag fix before the
    // block separator (Phase U-2).
    while dest.ends_with(' ') || dest.ends_with('\t') {
        dest.pop();
    }
    if dest.ends_with("\n\n") {
        return;
    }
    if dest.ends_with('\n') {
        dest.push('\n');
    } else {
        dest.push_str("\n\n");
    }
}

/// Close an inline emphasis-style element (`<strong>`, `<em>`, `<b>`, `<i>`).
///
/// When the element produced no visible content (the source had `<strong></strong>`
/// or `<i>   </i>`), erase the open marker too instead of emitting an empty
/// `**` / `*` pair.  Tier-2's DOM walker reaches the same result by emitting
/// nothing for an empty inline node; the byte-equality oracle requires us to
/// match that.
fn close_inline_marker(state: &mut Tier1State, frame: &OpenTag, marker: &str) {
    let buf = state.cell_or_output_mut();
    let body_is_empty = buf.len() <= frame.content_start
        || buf[frame.content_start..]
            .bytes()
            .all(|b| matches!(b, b' ' | b'\t' | b'\n' | b'\r'));
    if body_is_empty {
        // Truncate back to the position before the open marker was pushed.
        let open_marker_start = frame.content_start.saturating_sub(marker.len());
        buf.truncate(open_marker_start);
        return;
    }

    // Mirror Tier-2's `chomp_inline` (utility/content.rs:31): leading/trailing
    // whitespace (including Unicode whitespace like NBSP `\u{a0}`) inside the
    // strong/emphasis markers gets pushed OUTSIDE them so `**\u{a0}X**` becomes
    // `\u{a0}**X**`.  Required for byte-equality on Wikipedia fixtures with
    // `<b><span>&nbsp;</span>X</b>` patterns.
    let content_str = &buf[frame.content_start..];
    let leading_len = content_str.len() - content_str.trim_start().len();
    if leading_len > 0 {
        let leading: String = content_str[..leading_len].to_owned();
        buf.replace_range(frame.content_start..frame.content_start + leading_len, "");
        let marker_start = frame.content_start.saturating_sub(marker.len());
        buf.insert_str(marker_start, &leading);
    }

    buf.push_str(marker);
}

// ── Implicit close-tag emission ───────────────────────────────────────────────

/// Implicitly close the top-of-stack frame without a matching `</tag>` in the
/// input.  Called by the M4 implicit-close loop when HTML5 optional-tag rules
/// require an open element to be closed before the next tag is pushed.
///
/// Mirrors `emit_close` but skips the stack-pop search (we always close the
/// literal top frame) and skips the tag-name lookup (we use the frame's spec
/// directly).
fn emit_close_for_implicit(state: &mut Tier1State, options: &ConversionOptions) -> Result<(), BailReason> {
    let frame = state.stack.pop().ok_or_else(|| BailReason::DepthMismatch {
        tag: String::from("(implicit)"),
        expected: 1,
        actual: 0,
    })?;
    let spec = frame.spec;

    // Restore escape context.
    state.escape_ctx = frame.prev_escape_ctx;

    match spec.kind {
        TagKind::Paragraph => close_paragraph(state),
        TagKind::Heading(n) => close_heading(state, &frame, n, true)?,
        TagKind::Blockquote => close_blockquote(state, &frame),
        TagKind::Pre => close_pre(state, &frame, options),
        // Strong: suppress close marker when inside summary (see open strong guard).
        TagKind::Strong if state.summary_at_top() => {}
        TagKind::Strong => close_inline_marker(state, &frame, "**"),
        TagKind::Emphasis => close_inline_marker(state, &frame, "*"),
        // Strikethrough: also transparent inside <code>/<pre>; mirrors open guard.
        TagKind::Strikethrough
            if state.escape_ctx.contains(EscapeCtx::CODE) || state.escape_ctx.contains(EscapeCtx::PRE) => {}
        TagKind::Strikethrough => close_inline_marker(state, &frame, "~~"),
        TagKind::Inserted
            if state.escape_ctx.contains(EscapeCtx::CODE) || state.escape_ctx.contains(EscapeCtx::PRE) => {}
        TagKind::Inserted => close_inline_marker(state, &frame, "=="),
        TagKind::Code => close_code(state, &frame),
        TagKind::Link => close_link(state, &frame),
        TagKind::List(ListKind::Definition) => close_dl(state, &frame),
        TagKind::List(kind) => close_list(state, kind),
        TagKind::ListItem => close_list_item(state, &frame),
        TagKind::DefinitionTerm => close_dt(state),
        TagKind::DefinitionDescription => close_dd(state),
        TagKind::TableCell { .. } => close_table_cell(state, true)?,
        TagKind::TableRow => close_table_row(state),
        // Summary: pop accumulation buffer, trim, emit `**…**\n\n` (Phase R).
        TagKind::Summary => close_summary(state, &frame),
        // Figcaption: pop accumulation buffer, trim, emit `*…*\n\n` (Phase FF-2).
        TagKind::Figcaption => close_figcaption(state, &frame),
        // Button (Phase T): emit `\n\n` on EOF close just like explicit close.
        TagKind::Button => close_button(state, &frame),
        // Generic block/inline: no closing marker.
        TagKind::Block | TagKind::Inline => {}
        // Void-only kinds and other no-op kinds:
        TagKind::LineBreak
        | TagKind::Image
        | TagKind::Hr
        | TagKind::Table
        | TagKind::TableHead
        | TagKind::TableBody
        | TagKind::TableFoot
        | TagKind::TableCaption
        | TagKind::RawText(_)
        | TagKind::Ignored => {}
    }

    Ok(())
}

// ── Per-TagKind close helpers ─────────────────────────────────────────────────

fn close_paragraph(state: &mut Tier1State) {
    // When inside a table cell, `<p>` is transparent — no block separators.
    // Any inter-paragraph separators were already added as `<br>` at open time
    // by `open_paragraph`; `close_paragraph` does nothing in this context.
    if state.in_table_cell() {
        return;
    }
    // Tier-2 appends "\n\n" after paragraph content (always two newlines).
    // Matching this precisely is required for byte-equal output.
    trim_trailing_inline_whitespace(state);
    state.cell_or_output_mut().push_str("\n\n");
}

/// Close a heading element.
///
/// When `is_implicit` is true the empty-heading guard is skipped: implicitly
/// closed headings have already had their content flushed through the normal
/// path, so we just prepend the prefix unconditionally.
fn close_heading(state: &mut Tier1State, frame: &OpenTag, n: u8, is_implicit: bool) -> Result<(), BailReason> {
    // When inside a table cell, Tier-2 emits the heading text directly into
    // the cell accumulator — no `#` prefix, no block separators.  The
    // `frame.content_start` is a position in the CELL buffer (set by
    // `cell_or_output_mut().len()` at emit_open time), so all position
    // arithmetic must use the cell buffer, not `state.output`.
    if state.in_table_cell() {
        // Trim trailing inline whitespace from the cell buffer.
        let cell_buf = state.cell_or_output_mut();
        while cell_buf.ends_with(' ') || cell_buf.ends_with('\t') {
            cell_buf.pop();
        }
        if !is_implicit {
            let content = &state.cell_or_output_mut()[frame.content_start..];
            if content.trim().is_empty() {
                // Empty heading in a cell: roll back to content_start.
                let len = frame.content_start;
                state.cell_or_output_mut().truncate(len);
            }
        }
        // No prefix, no trailing separator — just the raw text in the cell.
        return Ok(());
    }

    trim_trailing_inline_whitespace(state);

    if !is_implicit {
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
            return Ok(());
        }
    }

    // Normalize whitespace in the heading body: Tier-2's heading.rs walks
    // children with `convert_as_inline: true` which routes text through
    // text-node normalization, folding `\n + indent` runs to a single space.
    // Mirror that here so `<h3>Mozilla\n   sponsorship</h3>` emits
    // `### Mozilla sponsorship` rather than `### Mozilla\n  sponsorship`.
    if state.output[frame.content_start..].contains('\n') {
        let content = state.output[frame.content_start..].to_owned();
        let mut normalized = String::with_capacity(content.len());
        let mut prev_was_space = false;
        for ch in content.chars() {
            let is_ws = ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r';
            if is_ws {
                if !prev_was_space {
                    normalized.push(' ');
                    prev_was_space = true;
                }
            } else {
                normalized.push(ch);
                prev_was_space = false;
            }
        }
        state.output.truncate(frame.content_start);
        state.output.push_str(normalized.trim_end());
    }

    let prefix = heading_prefix(n);
    state.output.insert_str(frame.content_start, prefix);
    // Tier-2 leaves a blank line ("\n\n") after a heading. A
    // following paragraph's "\n\n" guard then finds it already and appends
    // nothing, yielding the expected single blank line.
    state.ensure_blank_line();
    Ok(())
}

fn close_blockquote(state: &mut Tier1State, frame: &OpenTag) {
    // Phase GG follow-up: inside a table cell `frame.content_start` indexes
    // into the cell buffer, not `state.output`.  Don't prefix `> ` — Tier-2
    // also collapses blockquote inside cells to plain inline text.
    if state.in_table_cell() {
        return;
    }
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

fn close_pre(state: &mut Tier1State, frame: &OpenTag, options: &ConversionOptions) {
    use crate::options::CodeBlockStyle;
    // Phase GG follow-up: when `<pre>` opened inside a table cell, its content
    // was accumulated into `current_cell` (the cell buffer), not `state.output`.
    // The frame's `content_start` indexes into the cell buffer.  Don't emit a
    // code fence — Tier-2 also collapses pre inside cells to plain inline text
    // (the cell's `replace('\n', ' ')` step does the rest).
    if state.in_table_cell() {
        return;
    }
    let raw = state.output[frame.content_start..].to_owned();
    state.output.truncate(frame.content_start);
    match options.code_block_style {
        CodeBlockStyle::Indented => {
            let indented = indent_pre_lines(&raw);
            state.output.push_str(&indented);
        }
        CodeBlockStyle::Backticks => {
            state.output.push_str("```");
            if let Some(lang) = state.pre_lang.take() {
                state.output.push_str(&lang);
            } else if !options.code_language.is_empty() {
                state.output.push_str(&options.code_language);
            }
            state.output.push('\n');
            // Strip a single trailing newline from raw so the closing
            // fence doesn't end up after a blank line.  Tier-2 emits
            // `content\n```\n` (single newline).
            let raw = raw.strip_suffix('\n').unwrap_or(&raw);
            state.output.push_str(raw);
            state.output.push('\n');
            state.output.push_str("```\n");
        }
        // Tildes are router-gated; this arm is unreachable in practice but
        // kept exhaustive.
        CodeBlockStyle::Tildes => {
            let indented = indent_pre_lines(&raw);
            state.output.push_str(&indented);
        }
    }
    // Reset for the next <pre>.
    state.pre_lang = None;
}

fn close_code(state: &mut Tier1State, frame: &OpenTag) {
    // escape_ctx was already restored to prev (outer) context above.
    // When inside <pre> or an outer <code>, this <code> is transparent —
    // no backtick markers, content already emitted in place.
    if state.escape_ctx.contains(EscapeCtx::PRE) || state.escape_ctx.contains(EscapeCtx::CODE) {
        return;
    }
    // Phase CC: smart backtick escaping (mirrors inline/code.rs:260).
    // Open emitted nothing; content from `frame.content_start` to buf
    // end is the raw code content.  Choose num_backticks + delimiter
    // spaces from that slice, then truncate and re-emit wrapped.
    let buf = state.cell_or_output_mut();
    let content_start = frame.content_start.min(buf.len());
    if content_start >= buf.len() {
        // No content emitted between open and close — Tier-2 emits
        // nothing for empty <code></code>.
        return;
    }

    let contains_backtick = buf[content_start..].contains('`');

    let (needs_spaces, num_backticks) = {
        let content = &buf[content_start..];
        let first_char = content.chars().next();
        let last_char = content.chars().last();
        let starts_with_space = first_char == Some(' ');
        let ends_with_space = last_char == Some(' ');
        let starts_with_backtick = first_char == Some('`');
        let ends_with_backtick = last_char == Some('`');
        let all_spaces = content.chars().all(|c| c == ' ');

        let needs_delimiter_spaces = all_spaces
            || starts_with_backtick
            || ends_with_backtick
            || (starts_with_space && ends_with_space && contains_backtick);

        let num_backticks = if contains_backtick {
            let max_consecutive = content
                .chars()
                .fold((0usize, 0usize), |(max, current), c| {
                    if c == '`' {
                        let new_current = current + 1;
                        (max.max(new_current), new_current)
                    } else {
                        (max, 0)
                    }
                })
                .0;
            if max_consecutive == 1 { 2 } else { 1 }
        } else {
            1
        };
        (needs_delimiter_spaces, num_backticks)
    };

    // Splice in the open marker before the content.  Cheap path: build
    // the wrap prefix/suffix as &str pieces and use String::insert_str.
    // (We push the suffix at the end without an extra allocation.)
    let mut prefix = String::with_capacity(num_backticks + 1);
    for _ in 0..num_backticks {
        prefix.push('`');
    }
    if needs_spaces {
        prefix.push(' ');
    }
    buf.insert_str(content_start, &prefix);
    if needs_spaces {
        buf.push(' ');
    }
    for _ in 0..num_backticks {
        buf.push('`');
    }
}

fn close_link(state: &mut Tier1State, frame: &OpenTag) {
    // Close the link: `](href "title")` or `](href)`
    // If no href, just emit the text as-is (Tier-2 behaviour: no link markup).
    // Link state was pushed to state.link_stack at open; pop it now.
    let (href, title) = state.link_stack.pop().unwrap_or((None, None));
    let dest = state.cell_or_output_mut();
    // Trim trailing whitespace inside the link label so `[text  ](url)`
    // collapses to `[text](url)` — matches Tier-2's normalize_link_label
    // at utility/content.rs:145 (kimbrain.html and similar source HTML
    // with whitespace before </a>).
    let trim_start = frame.content_start.min(dest.len());
    let trimmed_end = dest[trim_start..].trim_end_matches(|c: char| c.is_whitespace()).len();
    dest.truncate(trim_start + trimmed_end);
    // Mirror Tier-2's `normalize_whitespace_cow` step inside
    // `normalize_link_label` (utility/content.rs:144): any Unicode whitespace
    // in the link label (notably NBSP `\u{00a0}`) collapses to a single ASCII
    // space.  Tier-1 otherwise emits `[Designed\u{a0}by](url)` where Tier-2
    // emits `[Designed by](url)`.
    if dest[trim_start..].contains('\u{00a0}') {
        let normalised: String = dest[trim_start..]
            .chars()
            .map(|c| if c == '\u{00a0}' { ' ' } else { c })
            .collect();
        dest.truncate(trim_start);
        dest.push_str(&normalised);
    }
    // Wikipedia back-reference normalisation (Tier-2 `handlers/link.rs:208`):
    // a label of exactly `^` paired with an `#anchor` href is rewritten to
    // `↑` so it does not look like Markdown's footnote syntax.
    if let Some(href_str) = href.as_deref() {
        if href_str.starts_with('#') && dest.len() == trim_start + 1 && dest.as_bytes()[trim_start] == b'^' {
            dest.truncate(trim_start);
            dest.push('↑');
        }
    }
    if let Some(href) = href {
        if let Some(title) = title {
            // measured: write! is slower on this workload (Stage 5c)
            #[allow(clippy::format_push_string)]
            dest.push_str(&format!("]({href} \"{title}\")"));
        } else {
            // measured: write! is slower on this workload (Stage 5c)
            #[allow(clippy::format_push_string)]
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

fn close_list(state: &mut Tier1State, kind: ListKind) {
    state.list_depth = state.list_depth.saturating_sub(1);
    if matches!(kind, ListKind::Unordered) {
        state.ul_depth = state.ul_depth.saturating_sub(1);
    }
    // When inside a table cell, Tier-2 does NOT add a trailing newline after
    // the list — the cell accumulator handles any separators via the
    // `\n → space` replacement at cell-close time.
    if state.in_table_cell() {
        return;
    }
    // Ensure the list ends with exactly one newline before any following content.
    let dest = state.cell_or_output_mut();
    if !dest.ends_with('\n') {
        dest.push('\n');
    }
}

fn close_list_item(state: &mut Tier1State, frame: &OpenTag) {
    // When inside a table cell, Tier-2 does NOT add a trailing newline after
    // each list item (see list/item.rs: `if !ctx.in_table_cell { ... \n ... }`).
    // Items are concatenated directly in the cell accumulator.
    if state.in_table_cell() {
        // Trim trailing inline whitespace from the cell buffer.
        let cell_buf = state.cell_or_output_mut();
        while cell_buf.ends_with(' ') || cell_buf.ends_with('\t') {
            cell_buf.pop();
        }
        return;
    }
    trim_trailing_inline_whitespace(state);
    let dest = state.cell_or_output_mut();
    // Phase EE: loose-list separator.  When this item had block-level
    // children (its content range contains a `\n\n` block separator),
    // mirror Tier-2's `handle_li` ensure_trailing_blank_line behaviour
    // so the next sibling `<li>` starts after a blank line.  Plain text
    // items still get the tight `\n` terminator.
    let had_block_children = {
        let start = frame.content_start.min(dest.len());
        dest[start..].contains("\n\n")
    };
    if had_block_children {
        if !dest.ends_with("\n\n") {
            if dest.ends_with('\n') {
                dest.push('\n');
            } else {
                dest.push_str("\n\n");
            }
        }
    } else if !dest.is_empty() && !dest.ends_with('\n') {
        dest.push('\n');
    }
}

// ── Definition-list helpers ───────────────────────────────────────────────────
//
// Tier-2 reference: crates/html-to-markdown/src/converter/list/definition.rs.
// Tier-2 builds the full <dl> content in a buffer, trims it, then emits with
// "\n\n" boundaries. <dt> emits trimmed term + "\n"; <dd> emits trimmed
// description + "\n\n". Tier-1 streams the same shape by:
//   - open_dl: ensure blank line; record content_start on the frame
//   - close_dt: trim trailing whitespace, push "\n"
//   - close_dd: trim trailing whitespace, push "\n\n"
//   - close_dl: trim leading/trailing whitespace inside the dl range, then
//               normalise the trailing separator to "\n\n"
//
// Bails on dl/dt/dd are removed (see bail_unsupported). Implicit close of an
// open dt/dd when a sibling dt/dd opens is wired via OptionalCloseRule::
// CloseSiblingDtDd in spec_rules.rs and runs the same close_dt/close_dd path
// through emit_close_for_implicit.

fn open_dl(state: &mut Tier1State) {
    if state.in_table_cell() {
        return;
    }
    state.ensure_blank_line();
}

const fn open_dt(_state: &mut Tier1State) {
    // No marker; content streams into the current buffer. close_dt adds the
    // trailing newline.
}

const fn open_dd(_state: &mut Tier1State) {
    // No marker; content streams into the current buffer. close_dd adds the
    // trailing paragraph separator.
}

fn close_dt(state: &mut Tier1State) {
    if state.in_table_cell() {
        return;
    }
    trim_trailing_inline_whitespace(state);
    let buf = state.cell_or_output_mut();
    if buf.is_empty() || buf.ends_with('\n') {
        return;
    }
    buf.push('\n');
}

fn close_dd(state: &mut Tier1State) {
    if state.in_table_cell() {
        return;
    }
    trim_trailing_inline_whitespace(state);
    let buf = state.cell_or_output_mut();
    if buf.is_empty() {
        return;
    }
    // Normalise to exactly "\n\n" at the end.
    if buf.ends_with("\n\n") {
        return;
    }
    if buf.ends_with('\n') {
        buf.push('\n');
    } else {
        buf.push_str("\n\n");
    }
}

fn close_dl(state: &mut Tier1State, frame: &OpenTag) {
    if state.in_table_cell() {
        return;
    }
    let buf = state.cell_or_output_mut();
    // Empty dl: emit nothing (matches Tier-2 which skips when trimmed content
    // is empty).
    if buf.len() <= frame.content_start {
        return;
    }
    // Tier-2 trims the dl's accumulated content, so any trailing whitespace
    // from the last dt/dd close should collapse to a single "\n\n" separator.
    while buf.len() > frame.content_start {
        let last = buf.as_bytes()[buf.len() - 1];
        if matches!(last, b' ' | b'\t' | b'\n' | b'\r') {
            buf.pop();
        } else {
            break;
        }
    }
    if buf.len() == frame.content_start {
        return;
    }
    buf.push_str("\n\n");
}

fn close_table(state: &mut Tier1State) -> Result<(), BailReason> {
    // Pop the table state and (if safe) emit the GFM table to main output.
    let Some(ts) = state.table_stack.pop() else {
        return Ok(());
    };

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
    //
    // When a <caption> is present, Tier-2 always takes the GFM path
    // regardless of <th> presence (has_caption short-circuits the layout check).
    let has_caption = ts.caption_text.is_some();
    if !ts.has_th && !has_caption {
        // No <th> and no <caption>: check if Tier-2 would take the layout path.
        let row_count = ts.rows.len();

        // Inconsistent column counts → layout table in Tier-2.
        // Compare colspan-expanded column counts (sum of cell colspans per row)
        // because Tier-2 computes column counts post-colspan expansion.
        let expanded_cols = |row: &Vec<(String, u16)>| -> usize { row.iter().map(|(_, c)| usize::from(*c)).sum() };
        let inconsistent_cols = {
            let first = ts.first_row_col_count.unwrap_or(0);
            ts.rows.iter().any(|r| expanded_cols(r) != first)
        };

        // Link-heavy with few rows → layout table in Tier-2.
        let link_heavy = row_count <= 2 && ts.link_count >= 3;

        // Blank table → Tier-2 emits nothing (not a bail case).
        let is_blank = ts.rows.is_empty() || ts.rows.iter().all(|r| r.iter().all(|(c, _)| c.trim().is_empty()));

        if inconsistent_cols || link_heavy || is_blank {
            // Tier-2 would not emit a GFM table here.  Bail so the fallback
            // produces the correct layout output.  Phase L's full layout
            // emit deferred — needs more careful per-cell content tracking
            // to mirror Tier-2's walker exactly.
            return Err(BailReason::Classifier);
        }
    }
    // Phase HH: a nested table writes its GFM rendering into the parent
    // cell buffer; the parent's `close_table_cell` then collapses the
    // resulting newlines to spaces.  An outer table writes to the main
    // output buffer as before.
    if ts.inline_mode {
        if let Some(outer) = state.table_stack.last_mut() {
            outer.had_nested_table = true;
        }
        let target = state.cell_or_output_mut();
        emit_gfm_table(target, ts);
    } else {
        emit_gfm_table(&mut state.output, ts);
    }
    Ok(())
}

fn close_table_head(state: &mut Tier1State) {
    if let Some(ts) = state.table_stack.last_mut() {
        ts.in_thead = false;
    }
}

fn close_table_body(state: &mut Tier1State) {
    if let Some(ts) = state.table_stack.last_mut() {
        ts.seen_tbody_close = true;
    }
}

/// Finalise a `<caption>` element.
///
/// Mirrors Tier-2's `builder.rs` caption handling: trim the collected text,
/// replace `-` with `\-` to prevent Markdown table-separator interpretation,
/// and store the result in `ts.caption_text` for emission before the table body.
fn close_table_caption(state: &mut Tier1State) {
    let Some(ts) = state.table_stack.last_mut() else {
        return;
    };
    ts.in_caption = false;
    let raw = std::mem::take(&mut ts.caption_buf);
    let trimmed = raw.trim();
    if !trimmed.is_empty() {
        ts.caption_text = Some(trimmed.replace('-', r"\-"));
    }
}

fn close_table_row(state: &mut Tier1State) {
    // Commit current_row to rows (skip empty rows).
    let Some(ts) = state.table_stack.last_mut() else {
        return;
    };
    if ts.current_row.is_empty() {
        return;
    }
    // Track first-row column count for consistency checking — use the
    // colspan-expanded count so Tier-2's heuristic compares the same numbers.
    let col_count: usize = ts.current_row.iter().map(|(_, c)| usize::from(*c)).sum();
    if ts.first_row_col_count.is_none() {
        ts.first_row_col_count = Some(col_count);
    }
    let row = std::mem::take(&mut ts.current_row);
    ts.rows.push(row);
}

/// Close a table cell (`<td>` or `<th>`).
///
/// `is_implicit` skips the pipe-escape bail that only applies when the cell
/// was explicitly closed (implicit closes happen during row/table teardown
/// where we've already committed to the data we have).
fn close_table_cell(state: &mut Tier1State, is_implicit: bool) -> Result<(), BailReason> {
    let Some(ts) = state.table_stack.last_mut() else {
        return Ok(());
    };
    ts.in_cell = false;
    // Trim the accumulated cell text (matches Tier-2 `text.trim()`).
    let cell_text_raw = ts.current_cell.trim().to_owned();
    // Replace newlines with spaces — mirrors Tier-2's `cell_text_content`
    // which calls `text.replace('\n', " ")` when `br_in_tables` is false.
    let cell_text = if cell_text_raw.contains('\n') {
        cell_text_raw.replace('\n', " ")
    } else {
        cell_text_raw
    };
    // Expand the `<br>` sentinel `\u{0001}` to three literal spaces — Tier-2
    // emits `<br>` as `"  \n"` and the cell-level `replace('\n', ' ')` yields
    // `"   "` (three spaces).  Using a sentinel keeps multi-space runs from
    // inter-tag whitespace distinguishable from `<br>`-derived padding.
    let cell_text = if cell_text.contains('\u{0001}') {
        cell_text.replace('\u{0001}', "   ")
    } else {
        cell_text
    };
    // Trim again after newline replacement (drops stray edge whitespace).
    let cell_text = cell_text.trim().to_owned();
    // Bail if the cell contains a pipe: Tier-2 escapes `|` → `\|`
    // which changes the cell width computation; Tier-1 does not
    // implement pipe escaping.  Implicit closes skip this check because
    // they are triggered during structural teardown, not fresh cell data.
    //
    // Phase HH exception: when a nested table emitted GFM markdown into
    // this cell, the literal pipes are part of the inner table's
    // rendering — Tier-2 does NOT escape them either.  `had_nested_table`
    // gates the skip; reset it so subsequent cells in the same row are
    // still pipe-checked.
    let allow_pipes = ts.had_nested_table;
    ts.had_nested_table = false;
    if !is_implicit && !allow_pipes && cell_text.contains('|') {
        return Err(BailReason::TableBlockChildInCell);
    }
    // Phase L-prep: store (text, colspan) so emit_gfm_table can mirror
    // Tier-2's `for _ in 0..colspan { output.push_str(" |") }` (cell.rs:248)
    // and the layout-heuristic uses the colspan-expanded column count.
    let colspan = ts.current_cell_colspan;
    ts.current_row.push((cell_text, colspan));
    ts.current_cell.clear();
    ts.current_cell_colspan = 1;
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
/// True when `s` ends with an ordered-list marker (`<digit(s)>. ` or `<digit(s)>) `).
///
/// Used by the inter-block whitespace strip to recognise that the scanner just
/// emitted a list-item marker and the next text would be the item content;
/// leading whitespace from the source HTML indentation should be dropped.
fn ends_with_ordered_marker(s: &str) -> bool {
    let bytes = s.as_bytes();
    let len = bytes.len();
    if len < 3 || bytes[len - 1] != b' ' {
        return false;
    }
    let punct = bytes[len - 2];
    if punct != b'.' && punct != b')' {
        return false;
    }
    // Walk back over digits.
    let mut i = len - 2;
    while i > 0 && bytes[i - 1].is_ascii_digit() {
        i -= 1;
    }
    // At least one digit and the start of digits is followed by a non-digit
    // (or start of buffer).
    i < len - 2 && (i == 0 || !bytes[i - 1].is_ascii_digit())
}

/// Returns `true` when the output tail is an explicit inline-element close
/// marker emitted by Tier-1.  These markers signal that the next whitespace
/// text node is between two inline siblings and should collapse to a single
/// space — even when the whitespace run contains a newline (Phase U-2).
///
/// Recognised markers:
/// - `**` — `</strong>` / `</b>` close
/// - `*` — `</em>` / `</i>` close (only a lone `*`, not part of `**`)
/// - `` ` `` — `</code>` close
/// - `)` — `</a>` (link) close, e.g. `](href)`
///
/// Block edges (`\n`, empty output, trailing space) are explicitly excluded.
fn output_ends_with_inline_close_marker(output: &str) -> bool {
    if output.is_empty() || output.ends_with('\n') || output.ends_with(' ') || output.ends_with('\t') {
        return false;
    }
    if output.ends_with("**") || output.ends_with('`') || output.ends_with(')') {
        return true;
    }
    output.ends_with('*') && !output.ends_with("**")
}

/// Returns `true` when the output tail is a non-marker text character —
/// e.g. ending in a letter, digit, or punctuation other than the inline-
/// close markers.  Text-tail preservation only fires for *horizontal*
/// whitespace runs (no `\n`/`\r`) because we cannot tell at flush time
/// whether the next tag is inline or block; preserving a space across a
/// newline-bearing run risks `text \n\n<list>` regressions.
fn output_ends_with_inline_text(output: &str) -> bool {
    if output.is_empty() || output.ends_with('\n') || output.ends_with(' ') || output.ends_with('\t') {
        return false;
    }
    !output_ends_with_inline_close_marker(output)
}

fn flush_text(state: &mut Tier1State, raw: &str, base_offset: usize) -> Result<(), BailReason> {
    if raw.is_empty() {
        return Ok(());
    }

    // Inside a table but outside a cell or caption: discard text (whitespace
    // between structural tags like <table>...<tr> or <tr>...<td>).
    // Tier-2 processes only tag children explicitly, ignoring text nodes at
    // this level.  Caption content is the exception — Tier-2 walks caption
    // children and accumulates their text into the caption output.
    if !state.table_stack.is_empty() && !state.in_table_cell() && !state.in_table_caption() {
        return Ok(());
    }

    let in_pre = state.escape_ctx.contains(EscapeCtx::PRE);
    // Phase EE: inside `<code>` text is verbatim — Tier-2's handle_code
    // walks children and pushes their text without normalize_whitespace,
    // so `\n` and runs of spaces inside `<code>` survive into the
    // wrapped span.  Treat as `in_pre` for the no-collapse path.
    let in_code = state.escape_ctx.contains(EscapeCtx::CODE);

    // Phase NN: text containing Unicode whitespace (NBSP `\u{00A0}`, hair
    // space `\u{200A}`, etc., or their entity forms) folds those to ASCII
    // space — but only when the chunk has non-whitespace content.
    // Mirrors Tier-2 `text_node.rs:124` and `:154` which run
    // `normalize_whitespace_cow` on text outside `<code>`/`<pre>` (folding
    // Unicode space chars).  The whitespace-only branch at `:80-112`
    // preserves a pure-NBSP text node between inline siblings as-is (e.g.
    // `<a>X</a>&nbsp;<a>Y</a>` keeps the NBSP).  Without this rule,
    // `First<NBSP>appeared` reaches the buffer verbatim where Tier-2 outputs
    // `First appeared`.
    // Common Unicode-whitespace entity forms: named + numeric (decimal +
    // hex).  Tier-2's `normalize_whitespace_cow` folds the decoded chars;
    // Tier-1's flush_text runs BEFORE entity decode, so the patterns must
    // be listed explicitly.
    const UNICODE_WS_ENTITIES: &[&str] = &[
        "&nbsp;", // U+00A0
        "&#160;", "&#xa0;", "&#xA0;", "&ensp;", // U+2002
        "&#8194;", "&#x2002;", "&emsp;", // U+2003
        "&#8195;", "&#x2003;", "&thinsp;", // U+2009
        "&#8201;", "&#x2009;", "&hairsp;", // U+200A
        "&#8202;", "&#x200a;", "&#x200A;",
    ];
    let raw_owned_nbsp;
    let raw: &str = if !in_pre && !in_code {
        let has_ws_entity = UNICODE_WS_ENTITIES.iter().any(|p| raw.contains(p));
        let has_unicode_ws_literal = raw.bytes().any(|b| b >= 0x80)
            && raw
                .chars()
                .any(|c| c.is_whitespace() && c != ' ' && c != '\t' && c != '\n' && c != '\r');
        if has_ws_entity || has_unicode_ws_literal {
            // Treat the chunk as logically whitespace-only when stripping
            // the Unicode-ws entities leaves only whitespace characters.
            let mut stripped = raw.to_owned();
            for p in UNICODE_WS_ENTITIES {
                if stripped.contains(p) {
                    stripped = stripped.replace(p, "");
                }
            }
            let is_logically_whitespace = stripped.chars().all(char::is_whitespace);
            if is_logically_whitespace {
                raw
            } else {
                // Fold all non-ASCII Unicode whitespace and known entities
                // to ASCII space, leaving ASCII whitespace untouched (those
                // are handled by the downstream collapse).
                let mut after_entities = raw.to_owned();
                for p in UNICODE_WS_ENTITIES {
                    if after_entities.contains(p) {
                        after_entities = after_entities.replace(p, " ");
                    }
                }
                let mut tmp = String::with_capacity(after_entities.len());
                for c in after_entities.chars() {
                    if c.is_whitespace() && c != ' ' && c != '\t' && c != '\n' && c != '\r' {
                        tmp.push(' ');
                    } else {
                        tmp.push(c);
                    }
                }
                raw_owned_nbsp = tmp;
                raw_owned_nbsp.as_str()
            }
        } else {
            raw
        }
    } else {
        raw
    };

    // Inter-block whitespace strip: in a block-edge context (output empty,
    // ends with a newline, or ends with a list-item marker like "- " /
    // "1. "), whitespace-only text between adjacent elements (the
    // indentation in pretty-printed HTML) is not meaningful and must be
    // discarded.  Tier-2's DOM walker gets this for free because the
    // parser yields text nodes separately from tag nodes and the walker
    // skips whitespace-only text at block-level boundaries.  Skipped when
    // inside `<pre>` (verbatim) or inside a table cell (caller is
    // accumulating cell text).
    //
    // We also treat "the current open frame is a link/emphasis frame whose
    // body is still empty" as a block-edge: text appearing immediately
    // after `<a>` → `[`, `<strong>` → `**`, etc. inherits leading
    // whitespace from the source HTML's indentation and Tier-2 trims it
    // when building the inline label.  This catches cases like
    // `<a href>\n   <span>EN</span>\n</a>` where the whitespace after
    // `<a>` would otherwise leak into the link label as `[ EN]`.
    //
    // Plain `<p>`/`<div>`/`<h1>` frames are NOT in this set — Tier-2 keeps
    // the leading whitespace inside the very first paragraph of a document
    // (it becomes the single space after `normalize_whitespace`).  Only
    // post-content paragraphs see "\n\n" before them, which the
    // `output.ends_with('\n')` check above already handles.
    // Phase R-3: inside `<summary>`, any tag's body-start is also an inline
    // frame edge.  Tier-2's handle_summary collects all children with
    // text-normalization in effect; leading whitespace inside `<span>`,
    // `<div>`, `<p>` (etc.) bodies gets stripped just like inside `<a>`.
    let in_summary_snapshot = state.in_summary();
    let at_inline_frame_start = match state.stack.last() {
        Some(frame) => {
            let cs = frame.content_start;
            let kind = frame.spec.kind;
            let buf_len = state.cell_or_output_mut().len();
            cs >= buf_len
                && (matches!(
                    kind,
                    TagKind::Link | TagKind::Strong | TagKind::Emphasis | TagKind::Code
                ) || (in_summary_snapshot
                    && matches!(
                        kind,
                        TagKind::Inline | TagKind::Block | TagKind::Paragraph | TagKind::Heading(_)
                    )))
        }
        None => false,
    };
    // Determine whether the current active output position is at a "block
    // edge" (empty or after a newline / list marker).  When inside a summary
    // accumulation buffer, consult that buffer rather than state.output so
    // that inter-element spaces inside the summary are preserved correctly.
    // Snap the relevant properties to local booleans before releasing the
    // borrow to avoid conflicts with subsequent state reads.
    let (active_empty, active_ends_newline, active_ends_list_marker, active_ends_ordered) = {
        let buf: &str = state.cell_or_output_mut();
        (
            buf.is_empty(),
            buf.ends_with('\n'),
            buf.ends_with("- ") || buf.ends_with("* ") || buf.ends_with("+ "),
            ends_with_ordered_marker(buf),
        )
    };
    let is_block_edge =
        active_empty || active_ends_newline || active_ends_list_marker || active_ends_ordered || at_inline_frame_start;
    let raw_is_whitespace = raw.bytes().all(|b| b == b' ' || b == b'\t' || b == b'\n' || b == b'\r');
    if !in_pre && is_block_edge && raw_is_whitespace {
        // Drop block-edge whitespace anywhere — including inside table cells.
        // A cell-open `<td>`/`<th>` produces a fresh empty buffer; the
        // pretty-printer's inter-tag whitespace before the first child would
        // otherwise leak as a leading space into the cell, breaking the
        // 3-space gap heuristic (`  \n` from `<div>` open becomes 4 spaces
        // instead of 3 after `replace('\n', ' ')`).
        return Ok(());
    }
    // Tier-2 text_node.rs:100-113 collapses whitespace-only text nodes
    // between adjacent inline siblings to a single space — including
    // inside table cells where the surrounding `<a>`/`<span>` siblings are
    // inline.  Mirror that here so `<a>x</a>\n  <a>y</a>` inside a `<td>`
    // emits `[x] [y]` (single space) instead of `[x]\n [y]` which the
    // cell-close `replace('\n', ' ')` would turn into two spaces.  Skip
    // when at a block edge (cell just opened) so the cell doesn't start
    // with a stray space.
    if !in_pre && state.in_table_cell() && raw_is_whitespace && !is_block_edge {
        // Tier-2's text_node.rs:80-98 drops whitespace text between non-inline
        // siblings: when the parent is a list (`<ul>`/`<ol>`/`<dl>`), the
        // inter-`<li>` whitespace returns without pushing because the next
        // sibling `<li>` is a block, not inline.  Mirror that here so adjacent
        // `<li>` siblings in a cell concatenate without separation
        // (`[v](u1)[t](u2)` not `[v](u1) [t](u2)`).  For inline parents
        // (`<span>`/`<a>`/`<td>` direct inline-sibling case), keep the
        // single-space fold.
        if matches!(state.stack.last().map(|f| f.spec.kind), Some(TagKind::List(_))) {
            return Ok(());
        }
        let dest = state.cell_or_output_mut();
        if !dest.is_empty() && !dest.ends_with(' ') && !dest.ends_with('\n') {
            dest.push(' ');
        }
        return Ok(());
    }
    // Whitespace-only text outside any inline element (link / strong / em /
    // code) and outside `<pre>` / table cells is structural indentation
    // between block siblings (e.g. between `</div>` and the next `<div>`).
    // Tier-2 emits a single ASCII space here when the surrounding context
    // is inline, but otherwise the DOM walker treats it as a no-op.  For
    // Tier-1's heuristic we collapse it to nothing — matches Tier-2 for
    // the common block-between-blocks case and the inline cases are caught
    // by the inline-frame check above.
    //
    // Exception (Phase U + U-2): when the output tail is inline content
    // (text or `**`/`*`/`` ` ``/`)` close markers) AND we're NOT at a
    // block edge, a whitespace-only text node between siblings must
    // become a single space.  Without this `</strong> <em>` would emit
    // `**a***b*` and `<span>Open Search Bar</span>\n<button>` would lose
    // the space before the button's content.
    //
    // Phase U-2 dropped the original "horizontal whitespace only" guard:
    // a newline-bearing whitespace run between two inline siblings still
    // collapses to a single space in Tier-2.  The "what if next tag is a
    // block?" regression is now handled later in `ensure_blank_line` and
    // `close_block_container`, which trim trailing horizontal whitespace
    // before emitting `\n\n`.
    if !in_pre && !state.in_table_cell() && raw_is_whitespace {
        // When inside a <summary> accumulation buffer, treat the context as
        // inline (like strong/emphasis): inter-element spaces must be
        // preserved so `<span>a</span> <span>b</span>` collects "a b" not "ab".
        let inside_inline = state.in_summary()
            || state.stack.iter().any(|frame| {
                matches!(
                    frame.spec.kind,
                    TagKind::Link | TagKind::Strong | TagKind::Emphasis | TagKind::Code
                )
            });
        if !inside_inline {
            // Use the active buffer (summary buf or main output) for the
            // tail check so spaces between adjacent inline elements inside
            // a summary are preserved correctly.
            let active_tail: &str = state.cell_or_output_mut();
            // Preserve a single space optimistically whenever the output
            // tail is inline content (text or `**`/`*`/`` ` ``/`)`) even
            // across newline-bearing whitespace.  Block-tag-open paths
            // (`ensure_blank_line`, `close_block_container`, `close_button`,
            // `open_paragraph`, `open_list`) all trim the trailing space
            // before emitting their separator — so this is safe.
            if output_ends_with_inline_close_marker(active_tail) || output_ends_with_inline_text(active_tail) {
                let dest = state.cell_or_output_mut();
                dest.push(' ');
            }
            return Ok(());
        }
        // Inside an inline frame (`<a>`/`<strong>`/`<em>`/`<code>`) or summary
        // accumulation: a whitespace-only text node (often the indent run
        // between two inline siblings like `</span>\n  <a>`) must collapse to
        // a single ASCII space — Tier-2's text-node normalize_whitespace folds
        // any `\n` + spaces run into one space.  Without this, Tier-1 falls
        // through to `decode_and_collapse_into` which preserves the `\n` and
        // emits `*[a](x)\n [b](y)*` where Tier-2 has `*[a](x) [b](y)*`.
        let active_tail: &str = state.cell_or_output_mut();
        if !active_tail.is_empty() && !active_tail.ends_with(' ') && !active_tail.ends_with('\n') {
            let dest = state.cell_or_output_mut();
            dest.push(' ');
        }
        return Ok(());
    }
    // Even when the text is not entirely whitespace, strip its LEADING
    // whitespace when:
    //   - we're at the start of an open inline element's body (`<a>`,
    //     `<strong>`, etc.), OR
    //   - the output ends with a block separator (`\n\n`) or a list-item
    //     marker — Tier-2's text-node `skip_prefix` logic does the same.
    //
    // Not when output is empty (first paragraph of a document keeps its
    // leading whitespace per Tier-2's behaviour).
    let block_separator_after = {
        let active: &str = state.cell_or_output_mut();
        active.ends_with("\n\n")
            || active.ends_with("- ")
            || active.ends_with("* ")
            || active.ends_with("+ ")
            || ends_with_ordered_marker(active)
    };
    let raw = if !in_pre && !state.in_table_cell() && (at_inline_frame_start || block_separator_after) {
        raw.trim_start_matches([' ', '\t', '\n', '\r'])
    } else {
        raw
    };
    if raw.is_empty() {
        return Ok(());
    }

    let has_entities = raw.contains('&');

    if in_pre || in_code {
        if has_entities {
            let dest = state.cell_or_output_mut();
            decode_entities_into(dest, raw, base_offset)?;
        } else {
            state.cell_or_output_mut().push_str(raw);
        }
        return Ok(());
    }

    // Inside an `<a>` link frame, Tier-2's `normalize_link_label` replaces
    // newlines with spaces before whitespace collapsing.  Mirror that here so
    // text spanning `\n` inside an `<a>` (e.g. `<a>Skip to main\n  content</a>`)
    // collapses to `[Skip to main content]` instead of leaking the newline.
    // `<strong>`/`<em>` do NOT normalize newlines in Tier-2 — only links do.
    //
    // `<summary>` is treated the same as `<a>` here (Phase R-3): Tier-2's
    // handle_summary collects children into a local content buffer and
    // wraps in `**...**\n\n`; the surrounding text-normalization layer
    // collapses internal newline runs to single spaces before emission.
    // Without this, summary content with multi-line inline children leaks
    // `\n  \n  ` between text runs.
    let inside_inline = state.in_summary() || state.stack.iter().any(|frame| matches!(frame.spec.kind, TagKind::Link));

    // Phase Y: text-node chomp.  Tier-2's text_node.rs runs `chomp()` on
    // every text node and substitutes the leading and trailing whitespace
    // runs with simpler stand-ins:
    //   prefix → `" "` if the run had any leading whitespace, else `""`
    //   suffix → `"\n\n"` if trailing run contained `\n\n`,
    //          → `" "`   if trailing run had space/tab (folding any `\n`),
    //          → `""`    if trailing run was `\n` only.
    // Without this, Tier-1 keeps the literal `\n  ` in text like
    // "The number of\n  " and emits `of\n ` while Tier-2 emits `of `,
    // and likewise the leading whitespace case `</em>\n  baz` produces
    // `*bar*\n baz` instead of `*bar* baz`.
    //
    // Applied only outside inline frames (which call
    // `decode_and_collapse_into_inline` and handle `\n` collapse already),
    // outside `<pre>` (verbatim), and outside table cells (which run
    // `normalize_whitespace_cow` directly).
    let raw_owned;
    let raw = if !inside_inline && !state.in_table_cell() {
        let trim_chars: &[char] = &['\n', '\r', ' ', '\t'];
        let after_lead = raw.trim_start_matches(trim_chars);
        let leading_len = raw.len() - after_lead.len();
        let lead_has_nl = leading_len > 0 && raw.as_bytes()[..leading_len].iter().any(|&b| b == b'\n' || b == b'\r');
        let trimmed_len = raw.trim_end_matches(trim_chars).len();
        let trailing_len = raw.len() - trimmed_len;
        let trail_has_nl = trailing_len > 0 && raw.as_bytes()[trimmed_len..].iter().any(|&b| b == b'\n' || b == b'\r');
        if lead_has_nl || trail_has_nl {
            // Slice safely: leading_len and trimmed_len are byte offsets
            // produced by `trim_*_matches` on the same `raw`.
            let core_start = leading_len;
            let core_end = trimmed_len;
            if core_start >= core_end {
                // Whitespace-only text node — already handled by the
                // earlier whitespace-only branches; skip Phase Y here.
                raw
            } else {
                let core = &raw[core_start..core_end];
                let trailing = &raw[core_end..];
                let prefix = if leading_len > 0 { " " } else { "" };
                let suffix = if trailing.contains("\n\n") {
                    "\n\n"
                } else if trailing.bytes().any(|b| b == b' ' || b == b'\t') {
                    " "
                } else if trail_has_nl {
                    ""
                } else {
                    // Pure space/tab trailing run; let the downstream
                    // collapse handle it as before.
                    trailing
                };
                raw_owned = format!("{prefix}{core}{suffix}");
                raw_owned.as_str()
            }
        } else {
            raw
        }
    } else {
        raw
    };
    if raw.is_empty() {
        return Ok(());
    }
    let has_entities = raw.contains('&');

    // Outside `<pre>`: collapse runs of space/tab into a single space, decode
    // entities, write directly into the output (or cell) buffer.  Newlines preserved
    // unless inside an inline frame (see above).
    if !has_entities {
        // No entities. Quick check: does this text have collapsible whitespace?
        // Use a fast memchr2/3 to find the first space/tab(/newline); if not
        // found, we can take the hot path.
        let needle_present = if inside_inline {
            memchr3(b' ', b'\t', b'\n', raw.as_bytes()).is_some()
        } else {
            memchr::memchr2(b' ', b'\t', raw.as_bytes()).is_some()
        };
        if !needle_present {
            state.cell_or_output_mut().push_str(raw);
            return Ok(());
        }
        let dest = state.cell_or_output_mut();
        return if inside_inline {
            decode_and_collapse_into_inline(dest, raw, false, base_offset)
        } else {
            decode_and_collapse_into(dest, raw, false, base_offset)
        };
    }

    let dest = state.cell_or_output_mut();
    if inside_inline {
        decode_and_collapse_into_inline(dest, raw, has_entities, base_offset)
    } else {
        decode_and_collapse_into(dest, raw, has_entities, base_offset)
    }
}

/// Decode HTML entities directly into `out` (no intermediate allocation).
///
/// `base_offset` is the byte offset of `s` within the original HTML input and
/// is used to report the position of any unrecognised entity in the bail reason.
///
/// Uses memchr to quickly find the next `&` and bulk-copies non-entity runs.
///
/// Returns `Err(BailReason::UnknownEntity)` when an entity cannot be decoded.
fn decode_entities_into(out: &mut String, s: &str, base_offset: usize) -> Result<(), BailReason> {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        // Fast path: use memchr to find next `&`.
        if let Some(pos) = memchr::memchr(b'&', &bytes[i..]) {
            let amp_pos = i + pos;
            if amp_pos > i {
                out.push_str(&s[i..amp_pos]);
            }
            i = decode_entity_at(bytes, s, amp_pos, out, base_offset)?;
        } else {
            if i < bytes.len() {
                out.push_str(&s[i..]);
            }
            break;
        }
    }
    Ok(())
}

/// Decode entities AND collapse spaces/tabs in one pass, directly into `out`.
///
/// `base_offset` is the byte offset of `s` within the original HTML input and
/// is used to report the position of any unrecognised entity in the bail reason.
///
/// Uses memchr3 to quickly find the next special byte (space/tab/&), then
/// bulk-copies the run in one `push_str` to avoid per-byte overhead.
///
/// Returns `Err(BailReason::UnknownEntity)` when an entity cannot be decoded.
fn decode_and_collapse_into(
    out: &mut String,
    s: &str,
    has_entities: bool,
    base_offset: usize,
) -> Result<(), BailReason> {
    decode_and_collapse_into_inner(out, s, has_entities, base_offset, false)
}

/// Collapse like `decode_and_collapse_into` but treat `\n`/`\r` as collapsible
/// whitespace too.  Used for text inside `<a>`/`<strong>`/`<em>` frames where
/// Tier-2's `normalize_link_label` first replaces newlines with spaces, then
/// runs whitespace normalization.
fn decode_and_collapse_into_inline(
    out: &mut String,
    s: &str,
    has_entities: bool,
    base_offset: usize,
) -> Result<(), BailReason> {
    decode_and_collapse_into_inner(out, s, has_entities, base_offset, true)
}

fn decode_and_collapse_into_inner(
    out: &mut String,
    s: &str,
    has_entities: bool,
    base_offset: usize,
    collapse_newlines: bool,
) -> Result<(), BailReason> {
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut prev_was_space = false;
    while i < bytes.len() {
        let next_special = match (has_entities, collapse_newlines) {
            (true, true) => {
                // Cold path: inline frame with entities. Find min of two memchr3 calls.
                let s_pos = memchr3(b' ', b'\t', b'\n', &bytes[i..]).map(|pos| i + pos);
                let e_pos = memchr::memchr(b'&', &bytes[i..]).map(|pos| i + pos);
                match (s_pos, e_pos) {
                    (Some(a), Some(b)) => Some(a.min(b)),
                    (Some(a), None) | (None, Some(a)) => Some(a),
                    (None, None) => None,
                }
            }
            (true, false) => memchr3(b' ', b'\t', b'&', &bytes[i..]).map(|pos| i + pos),
            (false, true) => memchr3(b' ', b'\t', b'\n', &bytes[i..]).map(|pos| i + pos),
            (false, false) => memchr::memchr2(b' ', b'\t', &bytes[i..]).map(|pos| i + pos),
        };

        if let Some(pos) = next_special {
            if pos > i {
                out.push_str(&s[i..pos]);
                prev_was_space = false;
            }
            match bytes[pos] {
                b' ' | b'\t' => {
                    if !prev_was_space {
                        out.push(' ');
                    }
                    prev_was_space = true;
                    i = pos + 1;
                }
                b'\n' if collapse_newlines => {
                    if !prev_was_space {
                        out.push(' ');
                    }
                    prev_was_space = true;
                    i = pos + 1;
                }
                b'&' => {
                    prev_was_space = false;
                    i = decode_entity_at(bytes, s, pos, out, base_offset)?;
                }
                _ => unreachable!(),
            }
        } else {
            if i < bytes.len() {
                out.push_str(&s[i..]);
            }
            break;
        }
    }
    Ok(())
}

/// Scan and decode a single HTML entity starting at `amp_pos` (the `&` byte).
///
/// Looks for a matching `;` within 32 bytes, then dispatches to
/// `decode_entity_into` or `decode_numeric_entity_into`.
///
/// Returns the position immediately after the entity (i.e. after the `;`), or
/// after the bare `&` when no valid entity boundary is found.
///
/// Emits `Err(BailReason::UnknownEntity)` when an `&name;` sequence is found
/// but the name is not in the decode table.
fn decode_entity_at(
    bytes: &[u8],
    s: &str,
    amp_pos: usize,
    out: &mut String,
    _base_offset: usize,
) -> Result<usize, BailReason> {
    let amp = amp_pos;
    let mut end = amp + 1;
    while end < bytes.len() && end - amp <= MAX_ENTITY_NAME_BYTES && bytes[end] != b';' {
        end += 1;
    }
    if end < bytes.len() && bytes[end] == b';' && end > amp + 1 {
        let entity = &s[amp + 1..end];
        if decode_entity_into(out, entity) {
            return Ok(end + 1);
        }
        // Phase N3: entity name (`&name;`) not in Tier-1's decode table.
        // Tier-2 and mdream pass these through verbatim instead of decoding.
        // Push the raw `&name;` and advance past it.
        out.push_str(&s[amp..=end]);
        return Ok(end + 1);
    }
    out.push('&');
    Ok(amp + 1)
}

// ── Escape context management ─────────────────────────────────────────────────

/// Apply the escape-context bits for an opening tag.
///
/// The close path restores `state.escape_ctx` directly from `frame.prev_escape_ctx`
/// so a symmetric `remove_open_escape_ctx` is not needed.
#[inline]
fn apply_open_escape_ctx(state: &mut Tier1State, spec: &TagSpec) {
    // Handle <pre> specially: it sets both PRE and CODE bits.
    if spec.kind == TagKind::Pre {
        state.escape_ctx |= EscapeCtx::PRE | EscapeCtx::CODE;
        return;
    }

    let bit = match spec.kind {
        TagKind::Code => EscapeCtx::CODE,
        TagKind::Link => EscapeCtx::LINK,
        TagKind::Blockquote => EscapeCtx::BLOCKQUOTE,
        TagKind::Heading(_) => EscapeCtx::HEADING,
        _ => return,
    };

    state.escape_ctx |= bit;
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

/// Returns true when `name_lower` is a tag that *may* need preprocessing-skip
/// evaluation.  All other tags skip the more expensive `should_skip_preprocessing`
/// check entirely.
fn is_preprocessing_skip_candidate(name_lower: &[u8]) -> bool {
    matches!(name_lower, b"nav" | b"header" | b"footer" | b"aside" | b"form")
}

/// Mirrors `should_drop_for_preprocessing` (preprocessing_helpers.rs:115) for
/// the Tier-1 byte scanner.
///
/// Called only for tags that passed [`is_preprocessing_skip_candidate`].
/// Uses the raw attribute byte slices collected by [`parse::collect_attrs`]
/// instead of the Tier-2 `tl::HTMLTag` DOM node.
fn should_skip_preprocessing(name_lower: &[u8], attrs: &[(&[u8], Option<&[u8]>)], options: &ConversionOptions) -> bool {
    use crate::options::PreprocessingPreset;

    if !options.preprocessing.enabled {
        return false;
    }

    if options.preprocessing.preset == PreprocessingPreset::Minimal {
        return false;
    }

    // Form removal — Standard and Aggressive when the flag is set.
    if options.preprocessing.remove_forms && name_lower == b"form" {
        return true;
    }

    if !options.preprocessing.remove_navigation {
        return false;
    }

    // <nav> is unconditionally navigation.
    if name_lower == b"nav" {
        return true;
    }

    // <header> / <footer> / <aside> — drop only when navigation hints present.
    // (Aggressive would drop footer/aside unconditionally, but Aggressive routes
    // through Tier-2 via the existing router gate so Tier-1 only needs the
    // Standard-preset behaviour: nav-hint check.)
    if matches!(name_lower, b"header" | b"footer" | b"aside") {
        return byte_attrs_have_navigation_hint(attrs);
    }

    false
}

/// Byte-level equivalent of `element_has_navigation_hint` for use in the
/// Tier-1 scanner where attributes are raw `&[u8]` slices rather than a
/// parsed `tl::HTMLTag`.
fn byte_attrs_have_navigation_hint(attrs: &[(&[u8], Option<&[u8]>)]) -> bool {
    // role ∈ { "navigation", "menubar", "tablist", "toolbar" }
    if let Some(role) = find_attr(attrs, b"role") {
        let role_lc = role.to_ascii_lowercase();
        if matches!(role_lc.as_slice(), b"navigation" | b"menubar" | b"tablist" | b"toolbar") {
            return true;
        }
    }

    // aria-label contains "navigation", "menu", "contents", "table of contents", or "toc"
    if let Some(label) = find_attr(attrs, b"aria-label") {
        let label_lc = label.to_ascii_lowercase();
        const ARIA_SUBSTRINGS: &[&[u8]] = &[b"navigation", b"menu", b"contents", b"table of contents", b"toc"];
        if ARIA_SUBSTRINGS
            .iter()
            .any(|sub| label_lc.windows(sub.len()).any(|w| w == *sub))
        {
            return true;
        }
    }

    // class / id — tokenize (split on whitespace, normalise _:./→-, lowercase)
    // and match against NAV_KEYWORDS.
    for attr_name in [b"class".as_slice(), b"id".as_slice()] {
        if let Some(value) = find_attr(attrs, attr_name) {
            if byte_value_has_nav_keyword(value) {
                return true;
            }
        }
    }

    false
}

/// Tokenize a raw attribute byte value and return true when any token matches
/// a keyword in [`NAV_KEYWORDS`].
///
/// Tokens are split on ASCII whitespace.  Each token is normalised by
/// replacing `_`, `:`, `.`, `/` with `-` and lowercasing before comparison.
fn byte_value_has_nav_keyword(value: &[u8]) -> bool {
    // Iterate over whitespace-separated tokens without allocating a Vec.
    let mut start = 0;
    let len = value.len();
    loop {
        // Skip leading whitespace.
        while start < len && value[start].is_ascii_whitespace() {
            start += 1;
        }
        if start >= len {
            break;
        }
        // Find end of token.
        let mut end = start;
        while end < len && !value[end].is_ascii_whitespace() {
            end += 1;
        }
        let token_bytes = &value[start..end];
        // Normalise: replace separator chars with `-`, lowercase.
        // Use a stack buffer when small enough to avoid heap allocation.
        let mut buf = [0u8; 64];
        let normalised: &[u8] = if token_bytes.len() <= buf.len() {
            let n = token_bytes.len();
            for (i, &b) in token_bytes.iter().enumerate() {
                buf[i] = match b {
                    b'_' | b':' | b'.' | b'/' => b'-',
                    _ => b.to_ascii_lowercase(),
                };
            }
            &buf[..n]
        } else {
            // Token is longer than any NAV_KEYWORD — skip without heap alloc.
            start = end;
            continue;
        };

        // Compare against each NAV_KEYWORD.
        if NAV_KEYWORDS.iter().any(|kw| kw.as_bytes() == normalised) {
            return true;
        }

        start = end;
    }
    false
}

/// Extract `href` and `title` from the attribute list for a link.
fn extract_link_attrs(attrs: &[(&[u8], Option<&[u8]>)]) -> Result<(Option<String>, Option<String>), BailReason> {
    let href = find_attr(attrs, b"href").map(decode_attr).transpose()?;
    // Mirror Tier-2's `inline/link.rs:82` which captures the title attribute
    // via tl::parse's `as_utf8_str()` — tl decodes numeric entities
    // (`&#039;` → `'`) but preserves named entities (`&amp;`, `&quot;`,
    // `&lt;`).  Use a partial-decode pass for titles to match.
    let title = find_attr(attrs, b"title").map(decode_title_attr).transpose()?;
    Ok((href, title))
}

/// Decode a link-title attribute: numeric entities (`&#NNN;`, `&#xNNN;`)
/// resolve to characters, named entities (`&amp;`, `&quot;`, etc.) survive
/// as-is.  Mirrors tl::parse's `as_utf8_str()` behaviour on attribute values.
fn decode_title_attr(bytes: &[u8]) -> Result<String, BailReason> {
    let s = std::str::from_utf8(bytes).map_err(|_| BailReason::Classifier)?;
    if !s.contains("&#") {
        return Ok(s.to_owned());
    }
    let mut out = String::with_capacity(s.len());
    let bytes_s = s.as_bytes();
    let mut i = 0;
    while i < bytes_s.len() {
        // Find next `&` (entity start) via memchr.
        let Some(rel) = memchr::memchr(b'&', &bytes_s[i..]) else {
            // No more entities: bulk-copy the rest (preserves UTF-8 sequences).
            out.push_str(&s[i..]);
            break;
        };
        let amp_pos = i + rel;
        if amp_pos > i {
            out.push_str(&s[i..amp_pos]);
        }
        if amp_pos + 1 >= bytes_s.len() || bytes_s[amp_pos + 1] != b'#' {
            // Named entity (or stray `&`): preserve the `&` literal and
            // continue scanning after it; the named-entity name itself is
            // ASCII so per-byte advancement is safe.
            out.push('&');
            i = amp_pos + 1;
            continue;
        }
        // Numeric entity: find the terminating `;` (always within ASCII range).
        let mut j = amp_pos + 2;
        while j < bytes_s.len() && bytes_s[j] != b';' {
            j += 1;
        }
        if j >= bytes_s.len() {
            // Unterminated: preserve literal `&` and everything after.
            out.push_str(&s[amp_pos..]);
            break;
        }
        let body = &s[amp_pos + 2..j];
        let cp_opt = if let Some(hex) = body.strip_prefix(['x', 'X']) {
            u32::from_str_radix(hex, 16).ok()
        } else {
            body.parse::<u32>().ok()
        };
        if let Some(cp) = cp_opt {
            if let Some(ch) = char::from_u32(cp) {
                out.push(ch);
                i = j + 1;
                continue;
            }
        }
        // Failed to decode: preserve literal `&#…;`.
        out.push_str(&s[amp_pos..=j]);
        i = j + 1;
    }
    Ok(out)
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
/// Returns `Err(BailReason::Classifier)` when the value is not valid UTF-8
/// (malformed bytes in attributes cannot be decoded faithfully).
/// Returns `Err(BailReason::UnknownEntity)` when the value contains an entity
/// that Tier-1 cannot decode (Tier-2 would decode it differently).
fn decode_attr(bytes: &[u8]) -> Result<String, BailReason> {
    let s = std::str::from_utf8(bytes).map_err(|_| BailReason::Classifier)?;
    if !s.contains('&') {
        return Ok(s.to_owned());
    }
    let mut out = String::with_capacity(s.len());
    // Attribute values do not carry a meaningful offset into the HTML source;
    // use 0 as the base so the entity name is still reported.
    decode_entities_into(&mut out, s, 0)?;
    Ok(out)
}

/// Canonicalize the special-character set in an attribute value to match
/// the output produced by html5ever's serializer (which Tier-2 runs on
/// HTML containing custom elements).  Numeric forms like `&#x22;` decode
/// to `"` and re-encode to the canonical named form `&quot;`; literal
/// special chars are also escaped.  Matches the set in
/// `html5ever::serialize::escape_for_attribute`.
fn canonicalize_attr_entities(input: &str) -> std::borrow::Cow<'_, str> {
    let needs_escape = input
        .bytes()
        .any(|b| matches!(b, b'&' | b'<' | b'>' | b'"') || b == 0xC2);
    if !needs_escape {
        return std::borrow::Cow::Borrowed(input);
    }
    let mut out = String::with_capacity(input.len() + 8);
    for c in input.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\u{a0}' => out.push_str("&nbsp;"),
            _ => out.push(c),
        }
    }
    std::borrow::Cow::Owned(out)
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
        if frame.spec.kind == TagKind::List(ListKind::Ordered) {
            frame.list_index = frame.list_index.saturating_add(1);
            return frame.list_index;
        }
    }
    1
}

/// Get the `ol_start` value from the nearest `List(Ordered)` frame.
fn find_ol_start(stack: &[OpenTag]) -> u16 {
    for frame in stack.iter().rev() {
        if frame.spec.kind == TagKind::List(ListKind::Ordered) {
            return frame.ol_start;
        }
    }
    1
}

// ── Formatting helpers ────────────────────────────────────────────────────────

/// Return the ATX heading prefix for level `n` (1–6).
///
/// Uses the `HEADING_PREFIXES` table — no allocation.
fn heading_prefix(n: u8) -> &'static str {
    let idx = (n as usize).saturating_sub(1).min(5);
    HEADING_PREFIXES[idx]
}

/// Push the list-item indentation for `depth` into `out`.
///
/// Depth 0 → no indent; each level adds two spaces (matches the router's
/// `list_indent_width == 2` gate).  Depths 0–7 use the static `LIST_ITEM_INDENTS`
/// table; deeper nesting (rare) falls back to a runtime loop.
fn push_list_item_indent(out: &mut String, depth: u16) {
    let idx = depth as usize;
    if idx < LIST_ITEM_INDENTS.len() {
        out.push_str(LIST_ITEM_INDENTS[idx]);
    } else {
        out.reserve(idx * 2);
        for _ in 0..idx {
            out.push_str("  ");
        }
    }
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
        if line.trim().is_empty() {
            // Empty / whitespace-only line: emit as a bare `\n` (no 4-space
            // indent prefix).  Tier-2's `block/code.rs` also skips the indent
            // for blank lines inside indented code blocks — without this,
            // round-tripped CommonMark `    code\n    \n    code` would
            // render with stray trailing spaces in the blank gap.
        } else {
            result.push_str("    ");
            // Convert char-count `min_indent` into a byte offset by walking
            // `char_indices`.  Indexing `line[min_indent..]` directly panics
            // when the leading whitespace contains multibyte characters such
            // as `\u{a0}` (NBSP).  Mirrors Tier-2's `dedent_code_block`
            // (text/processing.rs:38-50).
            let mut remaining = min_indent;
            let mut cut = 0;
            for (idx, ch) in line.char_indices() {
                if remaining == 0 {
                    break;
                }
                if ch.is_whitespace() {
                    remaining -= 1;
                    cut = idx + ch.len_utf8();
                } else {
                    break;
                }
            }
            result.push_str(&line[cut..]);
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
fn emit_gfm_table(target: &mut String, ts: crate::converter::tier1::state::TableState) {
    // Emit caption (if any) BEFORE the table body.
    //
    // Mirrors Tier-2 builder.rs caption handling: `*escaped_text*\n\n`.
    // Tier-2 emits the caption as part of the table child loop, which runs
    // before the rows are rendered, so the caption appears even when there
    // are no table rows.  The caption text has already been trimmed and
    // hyphen-escaped when `</caption>` was processed.
    if let Some(ref caption) = ts.caption_text {
        if !caption.is_empty() {
            // Ensure the caption starts after any preceding content.
            if !target.is_empty() && !target.ends_with("\n\n") {
                if target.ends_with('\n') {
                    target.push('\n');
                } else {
                    target.push_str("\n\n");
                }
            }
            target.push('*');
            target.push_str(caption);
            target.push_str("*\n\n");
        }
    }

    if ts.rows.is_empty() {
        return;
    }

    // Pre-table separator: mirrors Tier-2's `convert_table` logic exactly.
    // Tier-2 (block/table/mod.rs): `if !output.is_empty() && !output.ends_with("\n\n")`
    // — only adds separator when there is existing output (no leading blank lines).
    if !target.is_empty() && !target.ends_with("\n\n") {
        if target.ends_with('\n') {
            target.push('\n');
        } else {
            target.push_str("\n\n");
        }
    }

    // Pre-compute max column widths across ALL rows (mirrors Tier-2's pre-pass).
    // Tier-2: separator dashes = max(col_content_char_count_across_all_rows, 3).
    // col_count is the colspan-expanded column count (sum of colspans per row).
    let col_count = ts
        .rows
        .iter()
        .map(|r| r.iter().map(|(_, c)| usize::from(*c)).sum::<usize>())
        .max()
        .unwrap_or(0);
    let mut col_widths: Vec<usize> = vec![0; col_count];
    for row in &ts.rows {
        let mut col = 0usize;
        for (cell, span) in row {
            let w = cell.chars().count();
            // Only the cell's anchor column owns the width — spanned columns
            // contribute zero (matches Tier-2's per-cell pad calculation).
            if col < col_widths.len() && w > col_widths[col] {
                col_widths[col] = w;
            }
            col += usize::from(*span);
        }
    }

    for (row_index, row) in ts.rows.iter().enumerate() {
        // Row: `|` then each cell as ` text |` (padded to col_width like Tier-2).
        target.push('|');
        let mut col = 0usize;
        for (cell, span) in row {
            target.push(' ');
            target.push_str(cell);
            // Pad to column width (mirrors Tier-2 cell.rs padding logic).
            let cell_len = cell.chars().count();
            let col_w = col_widths.get(col).copied().unwrap_or(0);
            for _ in cell_len..col_w {
                target.push(' ');
            }
            // Tier-2 (cell.rs:248): `for _ in 0..colspan { output.push_str(" |") }`.
            // colspan trailing ` |` separators per cell — produces `| Header | | |`
            // for `<th colspan="3">Header</th>` instead of `| Header |  |  |`.
            for _ in 0..*span {
                target.push_str(" |");
            }
            col += usize::from(*span);
        }
        target.push('\n');

        // After row 0 (the header row), emit the separator row.
        // Tier-2: col_widths.get(i).unwrap_or(0).max(MIN_SEPARATOR_DASHES).
        if row_index == 0 {
            target.push_str("| ");
            for i in 0..col_count.max(1) {
                if i > 0 {
                    target.push_str(" | ");
                }
                let dash_count = col_widths.get(i).copied().unwrap_or(0).max(MIN_SEPARATOR_DASHES);
                for _ in 0..dash_count {
                    target.push('-');
                }
            }
            target.push_str(" |\n");
        }
    }
}

/// Trim trailing spaces and tabs from the end of the output (used before
/// closing block elements that trim trailing whitespace in Tier-2).
fn trim_trailing_inline_whitespace(state: &mut Tier1State) {
    let buf = state.cell_or_output_mut();
    while buf.ends_with(' ') || buf.ends_with('\t') {
        buf.pop();
    }
}

/// Collapse runs of 3+ consecutive newlines down to 2, matching Tier-2's
/// `collapse_excess_blank_lines` post-processing step.
fn collapse_excess_blank_lines(output: &mut String) {
    let mut consecutive = 0usize;
    output.retain(|c| {
        if c == '\n' {
            consecutive += 1;
            consecutive <= 2
        } else {
            consecutive = 0;
            true
        }
    });
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
/// truncated (names > `MAX_TAG_NAME_BYTES` won't appear in the spec table and
/// will be rejected as unknown).
fn lowercase_into<'b>(bytes: &[u8], buf: &'b mut [u8; MAX_TAG_NAME_BYTES]) -> &'b [u8] {
    let len = bytes.len().min(MAX_TAG_NAME_BYTES);
    for (i, &b) in bytes[..len].iter().enumerate() {
        buf[i] = b.to_ascii_lowercase();
    }
    &buf[..len]
}

/// Convert a byte slice to an owned `String` (lossy UTF-8).
fn bytes_to_string(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}
