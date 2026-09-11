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

// TODO(xberg-io/html-to-markdown#465): 5 cyclomatic-complexity and 38 size/complexity findings
// in this file, currently excluded via the quality-debt baseline in alef.toml. Splitting
// these needs compiler-in-the-loop verification, not a mechanical pass. Delete this
// note and the file's baseline entry together once it goes green. Help wanted.

use crate::converter::inline::link::has_uri_scheme;
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

/// Minimum length of a Markdown code fence (```` ``` ````) per `CommonMark`.
///
/// ~keep Mirrors `converter::handlers::code_block::MIN_FENCE_LENGTH`. Not imported
/// ~keep from there: that module is owned by another concurrent edit lane and this
/// ~keep crate's `tier1/` ownership boundary forbids editing it to add a `pub(crate)`
/// ~keep re-export. Reported as a proposed shared-helper extraction (see module docs).
const MIN_FENCE_LENGTH: usize = 3;

/// Static `TagSpec` used for all unknown custom elements (tag names containing
/// `-`, e.g. `<x-foo>`, `<my-component>`).
///
/// ~keep Unknown/custom elements are inline by default in HTML (there is no
/// ~keep such thing as a block-level custom element absent a UA stylesheet
/// ~keep rule or CSS `display` override, neither of which this converter
/// ~keep applies) and Tier-2's DOM walk treats them exactly that way: an
/// ~keep `<x-widget>` inside flowing text stays inline in the surrounding
/// ~keep paragraph/list-item/blockquote instead of splitting it. A `Block`
/// ~keep spec here previously matched Tier-2 only for the common case of a
/// ~keep custom element as a whole top-level document/child; it diverged the
/// ~keep moment one appeared inline.
///
/// The static reference `&CUSTOM_ELEMENT_INLINE_SPEC` is used anywhere the
/// scanner needs a `&'static TagSpec` for a custom element open/close tag.
static CUSTOM_ELEMENT_INLINE_SPEC: TagSpec = TagSpec {
    kind: TagKind::Inline,
    is_void: false,
    is_block: false,
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
    // ~keep Phase DD: Tier-2 runs an html5ever roundtrip when custom-element
    // ~keep tags are present in the source, which canonicalizes attribute
    // ~keep entities.  Mirror that for byte-equality.
    state.canonicalize_attr_entities = crate::converter::main_helpers::has_custom_element_tags(html);
    let mut table_probes: Vec<TableLayoutProbe> = Vec::new();
    let mut pos = 0usize;
    let mut text_start = 0usize;

    while pos < bytes.len() {
        match bytes[pos] {
            b'<' => {
                if text_start < pos {
                    // ~keep Peek the upcoming tag BEFORE flushing the preceding text: a
                    // ~keep purely-whitespace run immediately after an inline-close marker
                    // ~keep (`**`/`*`/etc.) is collapsed to one space by default, but
                    // ~keep Tier-2's `text_node.rs` pushes it verbatim when the next
                    // ~keep sibling isn't inline — see `flush_text`'s use of this flag for
                    // ~keep why that only matters (byte-for-byte) when the next sibling is
                    // ~keep specifically a nested `<ul>`/`<ol>`.
                    let next_tag_is_list = upcoming_tag_is_list_open(bytes, pos);
                    // ~keep Same idea, narrower target: whether the whitespace about to be
                    // ~keep flushed sits directly in front of a following `<img>` — see
                    // ~keep `flush_text`'s use of `next_tag_is_img` alongside
                    // ~keep `Tier1State::last_emitted_was_img`.
                    let next_tag_is_img = upcoming_tag_is_named(bytes, pos, b"img");
                    // ~keep A trailing bare `\n` (no accompanying space/tab) on an
                    // ~keep otherwise non-whitespace text node needs to know whether the
                    // ~keep upcoming sibling is specifically `<span>` — see
                    // ~keep `trailing_single_newline_join`'s doc comment for why that one
                    // ~keep tag is special-cased.
                    let next_tag_is_span = upcoming_tag_is_named(bytes, pos, b"span");
                    flush_text(
                        &mut state,
                        &html[text_start..pos],
                        text_start,
                        next_tag_is_list,
                        next_tag_is_img,
                        next_tag_is_span,
                    )?;
                }

                let next = bytes.get(pos + 1).copied().unwrap_or(0);

                if next == b'!' {
                    if html[pos..].starts_with("<![CDATA[") {
                        return Err(BailReason::Cdata { offset: pos });
                    }
                    pos = skip_bang(bytes, pos)?;
                    text_start = pos;
                    continue;
                }

                // ~keep `<?` — processing instruction.  Tier-2 handles these
                // ~keep inconsistently depending on whether html5ever-repair
                // ~keep ran (it rewrites bogus comments) and how tl chooses
                // ~keep to parse the run.  Either way the byte shape
                // ~keep downstream differs from the simple skip Tier-1 could
                // ~keep perform, so bail and let the Tier-2 fallback produce
                // ~keep the authoritative output.
                if next == b'?' {
                    return Err(BailReason::Classifier);
                }

                if next == b'/' {
                    let name_start = pos + 2;
                    let name_end = parse::scan_tag_name(bytes, name_start);
                    if name_end == name_start {
                        // ~keep `</>` or similar — bail
                        return Err(BailReason::LiteralLt { offset: pos });
                    }
                    let close_bracket =
                        parse::find_tag_close(bytes, name_end).ok_or(BailReason::LiteralLt { offset: pos })?;

                    let tag_name_bytes = &bytes[name_start..name_end];
                    emit_close(&mut state, tag_name_bytes, options, &mut table_probes)?;

                    pos = close_bracket.0 + 1;
                    text_start = pos;
                    continue;
                }

                // ~keep Not a tag-name-start byte → literal `<` in text. Tier-2
                // ~keep emits these verbatim (html5ever/astral-tl both parse a
                // ~keep bare `<x` as a text node). Emit the `<` and continue so
                // ~keep we don't bail on commonly-unescaped source like `x < 5`.
                if !parse::is_tag_name_start(next) {
                    flush_text(&mut state, "<", pos, false, false, false)?;
                    pos += 1;
                    text_start = pos;
                    continue;
                }

                let name_start = pos + 1;
                let name_end = parse::scan_tag_name(bytes, name_start);
                let tag_name_bytes = &bytes[name_start..name_end];

                let mut name_buf = [0u8; MAX_TAG_NAME_BYTES];
                let name_lower = lowercase_into(tag_name_bytes, &mut name_buf);

                // ~keep Audit #12 follow-up: `strip_hidden_elements`
                // ~keep (converter/utility/preprocessing.rs, outside tier1/) removes any
                // ~keep element carrying `hidden` or a `style="display:none"` /
                // ~keep `style="visibility:hidden"` / `style="font-size:0"` declaration — tag
                // ~keep and all descendant content — as a raw-string pass Tier-2 always runs
                // ~keep before parsing,
                // ~keep before either tier is even aware of tag identity. Checked here,
                // ~keep before the `<svg>`/`<template>` special cases below, so it is as
                // ~keep tag-agnostic as the pass it mirrors (a hidden `<svg>` must bail
                // ~keep too — its dedicated branch below has no hidden-element awareness
                // ~keep of its own). Reuses that pass's exact helpers (widened to
                // ~keep `pub(crate)`) rather than re-implementing the `hidden`/style
                // ~keep declaration scan a third time — see the code-fence/code-span
                // ~keep duplication this lane already fixed above.
                let tag_open_end =
                    crate::converter::utility::preprocessing::find_tag_end(bytes, pos + 1).unwrap_or(bytes.len());
                let tag_slice = &html[pos..tag_open_end];
                if crate::converter::utility::preprocessing::tag_has_hidden_attribute(tag_slice)
                    || crate::converter::utility::preprocessing::tag_has_hidden_style(tag_slice)
                {
                    return Err(BailReason::HiddenElement { offset: pos });
                }

                // ~keep Phase I: `<svg>` — emit as base64 data URI matching Tier-2's
                // ~keep `handle_svg` output.  The entire subtree (open tag through
                // ~keep `</svg>`) is consumed here; the scanner skips past it without
                // ~keep pushing anything on the open-tag stack.
                // ~keep
                // ~keep `tl::parse` is called on just the SVG fragment to normalize
                // ~keep attribute order via `serialize_element` (which sorts attrs
                // ~keep alphabetically — raw source bytes differ, so slicing alone is
                // ~keep not byte-identical with Tier-2 output).
                if name_lower == b"svg" {
                    let tag_open_start = pos;
                    let Some((close_pos, is_self_closing)) = parse::find_tag_close(bytes, name_end) else {
                        // ~keep Unclosed SVG open tag — skip to end; Tier-2 handles it.
                        pos = bytes.len();
                        text_start = pos;
                        continue;
                    };
                    let open_tag_end = close_pos + 1;

                    let svg_end = if is_self_closing {
                        // ~keep `<svg ... />` — self-closing, no children.
                        open_tag_end
                    } else {
                        // ~keep Find matching `</svg>` with depth tracking.
                        find_svg_close(bytes, open_tag_end).unwrap_or(bytes.len())
                    };

                    let svg_slice = &html[tag_open_start..svg_end];

                    emit_svg_from_slice(svg_slice, tag_open_start, &mut state, options)?;

                    pos = svg_end;
                    text_start = pos;
                    continue;
                }

                // ~keep Phase N: `<template>` — inert script container; Tier-2 drops
                // ~keep its content. Skip the entire subtree without emitting anything.
                // ~keep Self-closing form is rare but handled.
                // ~keep
                // ~keep Audit #12 follow-up: this comment previously cited
                // ~keep plain_text.rs SKIP_TAGS as the reason, which was true only for
                // ~keep the plain-text output path — the two tiers were silently
                // ~keep mismatched for Markdown output until main.rs's `"template" |
                // ~keep "noscript" => {}` arm landed (converter/main.rs, outside
                // ~keep tier1/). Both tiers now agree `<template>` is inert.
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

                // ~keep Resolve the tag spec.  Custom elements (names containing `-`)
                // ~keep are not in the static TAGS table but are treated as generic
                // ~keep inline passthroughs, matching `TagKind::Inline` behaviour and
                // ~keep HTML's default inline rendering for unknown elements.  All
                // ~keep other unknown tags are still bailed immediately.
                let spec: &'static TagSpec = if name_lower.contains(&b'-') {
                    &CUSTOM_ELEMENT_INLINE_SPEC
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

                // ~keep Raw-text "ignored" tags (`<script>`, `<style>`): their
                // ~keep spec is `TagKind::Ignored` with `is_rawtext = true` (see
                // ~keep tags.rs `rawtext_ignored`).  Prescan also strips their
                // ~keep content (STRIP_CONTENT_TAGS); Tier-2 does the same.  Skip
                // ~keep them inline so we don't bail to Tier-2 just because a page
                // ~keep contains an empty `<script></script>` left over from
                // ~keep prescan.  Other `RawText` kinds (textarea / title / xmp /
                // ~keep iframe / noscript / noembed / noframes) keep their text
                // ~keep content in Tier-2 and must continue to bail until Tier-1
                // ~keep learns to emit it correctly.
                if matches!(spec.kind, TagKind::Ignored) && spec.is_rawtext {
                    let open_end = match parse::find_tag_close(bytes, name_end) {
                        Some(close) => close.0 + 1,
                        None => bytes.len(),
                    };
                    pos = find_raw_text_close(bytes, open_end, name_lower).unwrap_or(bytes.len());

                    // ~keep Tier-2's `strip_script_and_style_tags` preprocessing pass
                    // ~keep (converter/utility/preprocessing.rs, outside tier1/) inserts a
                    // ~keep boundary space *per removed element* when its source-adjacent
                    // ~keep bytes are non-whitespace.  Two `<script>`/`<style>` tags sitting
                    // ~keep back-to-back with zero separating whitespace therefore each
                    // ~keep contribute a boundary space, and — because they collapse to a
                    // ~keep single whitespace-only DOM text node — Tier-2's downstream
                    // ~keep whitespace-mode handling of that node produces an idiosyncratic
                    // ~keep byte pattern (observed: a stray `\n\n  \n` at the nuxt-example
                    // ~keep fixture's trailing `<script><script></body>`) that is specific
                    // ~keep to whitespace-only-node handling, not reproducible by mirroring
                    // ~keep the boundary-space rule alone.  Bail so Tier-2 (authoritative)
                    // ~keep handles this rare, adjacency-only case; the single-tag word-glue
                    // ~keep mirror below still covers the common case.
                    if is_adjacent_rawtext_ignored_open(bytes, pos) {
                        return Err(BailReason::AdjacentRawTextTags { offset: pos });
                    }

                    text_start = pos;
                    // ~keep Mirror the single-element boundary-space rule: a space is
                    // ~keep inserted only when the removed tag would otherwise glue two
                    // ~keep word characters together.  The "before" check uses the emitted
                    // ~keep output tail (not the raw source byte) so that a space already
                    // ~keep produced by a preceding sibling is never doubled up; the "after"
                    // ~keep check peeks the next source byte, matching Tier-2's boundary
                    // ~keep condition exactly.
                    if name_lower == b"script" || name_lower == b"style" {
                        let after_is_word = pos < bytes.len() && !bytes[pos].is_ascii_whitespace();
                        if after_is_word {
                            let dest = state.cell_or_output_mut();
                            let ends_with_word = !dest.is_empty()
                                && !dest.ends_with(' ')
                                && !dest.ends_with('\t')
                                && !dest.ends_with('\n')
                                && !dest.ends_with('<')
                                && !dest.ends_with("<br>");
                            if ends_with_word {
                                dest.push(' ');
                            }
                        }
                    }
                    continue;
                }

                // ~keep Non-rawtext `Ignored` tags (`<head>`, `<meta>`, `<link>`):
                // ~keep Tier-2 does not emit any markdown from their bodies — head
                // ~keep is consumed by metadata extraction; meta/link are void.
                // ~keep Silent-skip them here so Tier-1 can be invoked on inputs
                // ~keep that contain a `<head>` (the common case for full HTML
                // ~keep documents) without bailing.  For non-void `<head>`, capture
                // ~keep the content range on `state.head_range` so `tier1::run` can
                // ~keep hand it to `head_metadata::extract_frontmatter` when
                // ~keep metadata extraction is enabled.
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

                // ~keep Bail on unsupported tag kinds for M3c
                bail_unsupported(spec, pos)?;

                // ~keep Phase D': mirror Tier-2's preprocessing pipeline <nav> /
                // ~keep nav-hinted <header> / <footer> / <aside> / <form> strip.
                // ~keep When the user's preprocessing options request the strip,
                // ~keep jump past the matching close tag without pushing any frame.
                // ~keep Matches Tier-2's should_drop_for_preprocessing
                // ~keep (preprocessing_helpers.rs).
                if is_preprocessing_skip_candidate(name_lower) {
                    let close = parse::find_tag_close(bytes, name_end).ok_or(BailReason::LiteralLt { offset: pos })?;
                    let attrs_end = if close.1 { close.0.saturating_sub(1) } else { close.0 };
                    let skip_attrs = parse::collect_attrs(bytes, name_end, attrs_end);
                    if should_skip_preprocessing(name_lower, &skip_attrs, options) {
                        let open_end = close.0 + 1;
                        if close.1 {
                            pos = open_end;
                        } else {
                            pos = find_balanced_close(bytes, open_end, name_lower).unwrap_or(bytes.len());
                        }
                        text_start = pos;
                        continue;
                    }
                }

                // ~keep Bail on <pre> when code_block_style is not Indented.
                // ~keep Phase Q.4: Tier-1 supports Indented (4-space) and
                // ~keep Backticks (`` ``` ``-fenced) code blocks via open_pre /
                // ~keep close_pre.  Tildes still require Tier-2's fence emitter.
                if matches!(spec.kind, TagKind::Pre)
                    && options.code_block_style == crate::options::CodeBlockStyle::Tildes
                {
                    return Err(BailReason::Classifier);
                }

                let close = parse::find_tag_close(bytes, name_end).ok_or(BailReason::LiteralLt { offset: pos })?;

                let attrs_end = if close.1 { close.0.saturating_sub(1) } else { close.0 };
                // ~keep Most tag kinds (headings, paragraphs, emphasis, code, etc.) do
                // ~keep not read attributes during emit.  Skip the allocation in the
                // ~keep common case; only collect for the kinds whose emit paths
                // ~keep actually consult attributes.  `<abbr>` is `TagKind::Inline`
                // ~keep but its `title` attribute is read at open time to mirror
                // ~keep Tier-2's `handle_abbr` — include it in the collect-set.
                // ~keep `Table` is in the collect-set for `border`, which feeds the
                // ~keep `has_span && border="0"` leg of Tier-2's `looks_like_layout`.
                // ~keep Omitting it silently disables that bail: `attrs` is empty, so the
                // ~keep lookup returns None, `border_zero` stays false, and Tier-1 emits a
                // ~keep GFM table where Tier-2 emits a bullet list. `TableCell` was already
                // ~keep here for colspan, which is why only the border half was affected.
                let needs_attrs = matches!(
                    spec.kind,
                    TagKind::Link
                        | TagKind::Image
                        | TagKind::List(ListKind::Ordered)
                        | TagKind::Table
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

                // ~keep Any child tag opening while a link is on the stack means that
                // ~keep link's label buffer is no longer flat text — Tier-2's autolink
                // ~keep predicate strips such tags before comparing to href, so
                // ~keep `close_link` cannot safely trust a literal buffer mismatch here.
                // ~keep Mark every currently open link (not just the innermost, in case
                // ~keep of malformed nested `<a>`) so `close_link` can bail via
                // ~keep `BailReason::LinkAutolinkNestedMarkup` instead of risking a false
                // ~keep negative against Tier-2's tag-stripped text.
                for entry in &mut state.link_stack {
                    entry.2 = true;
                }

                if spec.is_void || close.1 {
                    emit_void(&mut state, spec, &attrs, html, options)?;
                    text_start = pos;
                    continue;
                }

                // ~keep Phase HH: nested tables are NO LONGER bailed here.  An inner
                // ~keep table is opened with `inline_mode = true` (set inside
                // ~keep `open_table`), and on `</table>` the rendered GFM markdown
                // ~keep is written into the parent cell buffer rather than
                // ~keep `state.output`.  The parent cell's newline-collapse step
                // ~keep then flattens the inner table to a single inline run,
                // ~keep matching Tier-2's behaviour.

                // ~keep M4: HTML5 implicit-close transitions.
                // ~keep Run BEFORE the block-in-cell check so that structural table
                // ~keep elements like `<tr>` correctly close any open `<td>`/`<th>`
                // ~keep before the block check evaluates `in_table_cell()`.  Without
                // ~keep this ordering, `<th>h1<tr>` would fire the bail even though
                // ~keep `<tr>` is not a content element inside the cell.
                while let Some(top) = state.stack.last() {
                    if !spec_rules::should_close_for_new_tag(top.spec, spec) {
                        break;
                    }
                    emit_close_for_implicit(&mut state, options, &mut table_probes)?;
                }

                // ~keep M9: Block-in-cell bail.
                // ~keep Evaluated AFTER M4 implicit closes so that table-structural
                // ~keep elements (e.g. a `<tr>` following an unclosed `<th>`) correctly
                // ~keep collapse the cell state before the check runs.
                // ~keep
                // ~keep The `inlineable` set below is the source of truth for what may
                // ~keep appear in a cell; every kind in it has cell-aware open/close helpers
                // ~keep that redirect output to the cell accumulator and match Tier-2's
                // ~keep `cell_text_content` normalisation (`text.replace('\n', " ")` when
                // ~keep `br_in_tables` is false).  Blockquote and Pre are included: their
                // ~keep close helpers return early in a cell (Phase GG) rather than emitting
                // ~keep a `> ` prefix or a code fence, which is what Tier-2 does too.
                // ~keep A block kind bails only when it has no cell-aware helper, so adding
                // ~keep one here without adding the helper will silently diverge from Tier-2.
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

                // ~keep A nested list's indent must equal the cumulative width of every
                // ~keep ancestor marker ("- " = 2, "1. " = 3, "10. " = 4, ...); Tier-1's
                // ~keep `push_list_item_indent` hardcodes a uniform 2-space-per-depth
                // ~keep scheme. That is correct for ul-in-ul (all markers are 2 wide) but
                // ~keep wrong the moment an ordered list is anywhere in the ancestor chain
                // ~keep of a nested list. Bail so Tier-2's cumulative-width logic wins.
                if let TagKind::List(kind) = spec.kind {
                    if kind != ListKind::Definition
                        && state.list_depth > 0
                        && (kind == ListKind::Ordered || find_parent_list_kind(&state.stack) == Some(ListKind::Ordered))
                    {
                        return Err(BailReason::ListNestedOrdered);
                    }
                }

                // ~keep See `BailReason::ListItemUnsupportedBlockChild`'s doc comment for the
                // ~keep full root-cause writeup. `<blockquote>`/`<div>`/`<table>`/`<dl>` bail
                // ~keep unconditionally inside a list item (any position); `<p>` bails only as
                // ~keep a continuation of already-started text (its bare-marker/first-content
                // ~keep shape is already correct); `<pre>` bails only as bare-marker/first
                // ~keep content (its continuation shape is already correct).
                if !state.in_table_cell() && state.list_continuation_indent_width() > 0 {
                    let bare_marker_line = line_is_bare_list_marker(&state.output);
                    let bails = match spec.kind {
                        TagKind::Blockquote | TagKind::Table => true,
                        TagKind::Block => name_lower == b"div",
                        TagKind::List(ListKind::Definition) => true,
                        TagKind::Paragraph => !bare_marker_line,
                        TagKind::Pre => bare_marker_line,
                        _ => false,
                    };
                    if bails {
                        return Err(BailReason::ListItemUnsupportedBlockChild);
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
                    state.link_stack.push((href, title, false));
                }
                // ~keep Mirror Tier-2's `semantic/attributes.rs::handle_abbr`:
                // ~keep capture the abbreviation's `title` attribute and emit
                // ~keep `" (title)"` after the abbr's text content at close time.
                if name_lower == b"abbr" {
                    let title = find_attr(&attrs, b"title")
                        .and_then(|b| std::str::from_utf8(b).ok())
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(str::to_owned);
                    state.abbr_titles.push(title);
                }

                // ~keep TIER1-57: `state.stack` is an explicit `Vec`, not native
                // ~keep recursion — the scanner itself has no stack-overflow risk.
                // ~keep But Tier-2's recursive `walk_node` silently truncates once
                // ~keep `depth >= effective_max_depth` (main.rs), and this scanner
                // ~keep had no equivalent ceiling: a pathologically deep DOM would
                // ~keep scan to completion here while Tier-2 truncates, diverging
                // ~keep from the byte-equality contract. Bail so Tier-2's (truncated,
                // ~keep authoritative) output wins instead of silently accepting
                // ~keep input Tier-2 would reject part of. `state.stack.len()` (open
                // ~keep tags below this one) is the same `depth` Tier-2 checks before
                // ~keep dispatching this node.
                let max_depth = crate::converter::main_helpers::effective_max_depth(options);
                if state.stack.len() >= max_depth {
                    return Err(BailReason::DepthLimitExceeded {
                        depth: state.stack.len(),
                        max_depth,
                    });
                }

                emit_open(&mut state, spec, name_lower, &attrs, &mut table_probes, options)?;

                // ~keep Record the content-start position AFTER emit_open so that
                // ~keep close-side post-processing operates on the correct slice.
                // ~keep When inside a table cell the position is in the cell buffer;
                // ~keep otherwise it is in the main output buffer.
                let output_content_start = state.cell_or_output_mut().len();

                let list_index = 0u16;

                state.stack.push(OpenTag {
                    spec,
                    content_start: output_content_start,
                    prev_escape_ctx: prev_ctx,
                    list_index,
                    ol_start,
                    name_range: name_start..name_end,
                    dropped_whitespace_only_text: false,
                });

                apply_open_escape_ctx(&mut state, spec);

                text_start = pos;
            }
            _ => {
                // ~keep Batch ASCII fast-path: skip forward to the next `<` or `&`
                // ~keep (the only two bytes that require special handling) in one
                // ~keep memchr2 call instead of advancing one byte at a time.
                // ~keep flush_text handles entity decoding and whitespace collapsing
                // ~keep for whatever raw slice [text_start..pos] we hand it, so it
                // ~keep is correct to jump pos all the way to the next special byte.
                // ~keep This is safe across every context (<pre>, table cells, etc.)
                // ~keep because:
                // ~keep   • `<` still triggers the tag-dispatch path above.
                // ~keep   • `&` is preserved in the slice passed to flush_text, which
                // ~keep     entity-decodes it correctly regardless of context.
                // ~keep   • Raw-text elements (script/style/textarea/…) bail before
                // ~keep     reaching this arm, so we never skip inside them.
                match memchr2(b'<', b'&', &bytes[pos..]) {
                    Some(offset) if offset > 0 => pos += offset,
                    Some(_) => pos += 1,
                    None => pos = bytes.len(),
                }
            }
        }
    }

    if text_start < pos {
        flush_text(&mut state, &html[text_start..pos], text_start, false, false, false)?;
    }

    // ~keep Phase N2: implicitly close all remaining open elements at EOF.
    // ~keep HTML5 parsers (html5ever and tl) close every open element when input
    // ~keep ends, so Tier-2 produces output even for malformed input like
    // ~keep `<p>hello <b>world` (no `</b>`, no `</p>`).  Mirror that here by
    // ~keep running emit_close_for_implicit on every remaining frame, regardless
    // ~keep of whether it has an OptionalCloseRule.
    // ~keep
    // ~keep Before closing, trim trailing inline whitespace (spaces, tabs, newlines)
    // ~keep from the output buffer.  In well-formed HTML the close tag arrives
    // ~keep before the file's trailing newline; the inline close-marker emission
    // ~keep (e.g. `**` for `</strong>`) lands flush against the content.  At EOF
    // ~keep any trailing newline is between the implicit close and the file end,
    // ~keep not inside the inline body, so we trim it before pushing the close
    // ~keep marker to match Tier-2's `world**` instead of `world\n**`.
    while !state.stack.is_empty() {
        let buf = &mut state.output;
        while matches!(buf.as_bytes().last(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            buf.pop();
        }
        emit_close_for_implicit(&mut state, options, &mut table_probes)?;
    }

    // ~keep Mirror Tier-2's final render-stage order exactly (main.rs):
    // ~keep `trim_line_end_whitespace` runs BEFORE `collapse_excess_blank_lines`.
    // ~keep Without this, a code span's delimiter padding space that lands right
    // ~keep before an embedded newline in `<code>` content (e.g. a leading
    // ~keep `\n`-starting body next to a trailing backtick run needing a pad)
    // ~keep survives in Tier-1 but is stripped by Tier-2 as ordinary
    // ~keep end-of-line trailing whitespace — this is a general per-line pass,
    // ~keep not something scoped to code spans specifically, so reuse the exact
    // ~keep Tier-2 helper rather than reimplementing it.
    crate::converter::main_helpers::trim_line_end_whitespace(&mut state.output);

    // ~keep Collapse runs of 3+ consecutive newlines to exactly 2, matching Tier-2's
    // ~keep `collapse_excess_blank_lines` post-processing step.
    if state.output.contains("\n\n\n") {
        collapse_excess_blank_lines(&mut state.output);
    }

    // ~keep Normalise trailing newlines to match Tier-2's final-output contract:
    // ~keep   `format!("{}\n", output.trim_end_matches('\n'))`
    // ~keep Tier-2 strips all trailing newlines and appends exactly one.  We mirror
    // ~keep that here so paragraphs (which emit "\n\n") don't leave an extra blank
    // ~keep line at the end.
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

// ~keep ── Bail guard ────────────────────────────────────────────────────────────────

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

// ~keep ── SVG helpers ───────────────────────────────────────────────────────────────

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
                        // ~keep A closing tag with no terminating `>` before EOF must
                        // ~keep return `None` (not `Some(len + 1)`, an out-of-bounds
                        // ~keep offset one past the last valid slice/index into `bytes`).
                        // ~keep Callers (`find_svg_close` via `.unwrap_or(bytes.len())`)
                        // ~keep already handle `None` by clamping to end-of-input.
                        return if j < len { Some(j + 1) } else { None };
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
    // ~keep CDATA inside SVG cannot be processed correctly without the prescan's
    // ~keep entity-escaping transformation.  Bail to Tier-2 so it sees the
    // ~keep prescan-normalized form (where `<![CDATA[` is escaped to `&lt;![CDATA[`).
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

    // ~keep Re-parse just the SVG fragment.  Wrap it in a minimal document so
    // ~keep tl has proper context — the same pattern used by head_metadata.rs.
    let wrapped = format!("<html><body>{svg_slice}</body></html>");
    let dom = match tl::parse(&wrapped, tl::ParserOptions::default()) {
        Ok(d) => d,
        Err(_) => {
            // ~keep Parse failure: emit nothing rather than bail — matches
            // ~keep Tier-2's silent skip on serialization failure.
            return Ok(());
        }
    };
    let parser = dom.parser();

    // ~keep Locate the first `<svg>` node in the parsed fragment.
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

    // ~keep Extract title from a direct `<title>` child, mirroring Tier-2.
    let title = if let Some(tl::Node::Tag(svg_tag)) = handle.get(parser) {
        let mut found = String::from("SVG Image");
        for child_handle in svg_tag.children().top().iter() {
            if let Some(tl::Node::Tag(child)) = child_handle.get(parser) {
                if child.name().as_utf8_str().as_ref().eq_ignore_ascii_case("title") {
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

    // ~keep Security fix mirror (media/svg.rs::handle_svg, outside tier1/): an
    // ~keep unescaped `<title>` here lets `x](https://evil.example)y` in the SVG
    // ~keep source close the image label early and open a second, attacker-
    // ~keep controlled Markdown image/link — the input is inert HTML, but the
    // ~keep unescaped label turns it into a live injection. `escape_link_label`
    // ~keep (utility/content.rs) is the shared helper Tier-2's `<a>` label path
    // ~keep already uses; call the same one here rather than a third
    // ~keep hand-written escaper.
    let escaped_title = crate::converter::utility::content::escape_link_label(&title);

    let dest = state.cell_or_output_mut();
    dest.push_str("![");
    dest.push_str(&escaped_title);
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
        // ~keep This arm IS load-bearing — do not delete it as unreachable.  The inline
        // ~keep raw-text handling above is gated on `TagKind::Ignored && is_rawtext`, which
        // ~keep is only `<script>`/`<style>`.  The seven genuine `TagKind::RawText` kinds
        // ~keep (title / xmp / textarea / iframe / noscript / noembed / noframes) fall
        // ~keep through to here, and this is the bail that keeps Tier-1 from emitting their
        // ~keep text content incorrectly.  See the sibling note above `find_raw_text_close`.
        TagKind::RawText(_) => Err(BailReason::Classifier),

        // ~keep `Ignored` tags (head/meta/link/script/style) are now handled inline
        // ~keep by the main scan loop (see the dispatch above `bail_unsupported`).
        // ~keep The match arm is kept for exhaustiveness — it cannot fire in
        // ~keep practice.
        TagKind::Ignored => Err(BailReason::Classifier),

        _ => Ok(()),
    }
}

// ~keep `TagKind::Block` is Tier-1's catch-all for HTML5 "generic block container"
// ~keep elements, but Tier-2's `main.rs` dispatch match does NOT give every one of
// ~keep them a dedicated separator-emitting handler. `<div>` (block/div.rs), the
// ~keep semantic/media/form-dispatched names (section, article, header, footer,
// ~keep aside, main, nav, details, dialog, figure, menu, audio, video, fieldset,
// ~keep legend, form), and -- as of the fix below -- `address`/`search`/`hgroup`/
// ~keep `center` (routed straight to `block::div::handle`, since they are
// ~keep content-bearing containers with no formatting beyond block separation) all
// ~keep render through a handler that pushes a leading and/or trailing `"\n\n"`.
// ~keep The names below are the ones that still fall through to Tier-2's `_ =>` arm
// ~keep (`block::unknown::handle`) or, for `html`/`body`,
// ~keep `block::container::handle_structural_container` -- both of which just walk
// ~keep children with NO separator of their own, and deliberately so:
// ~keep `colgroup`/`col` are table-internal metadata (never render visible content
// ~keep of their own), `base` is void metadata, and `html`/`body` are document
// ~keep wrappers whose children ARE the document -- giving them a separator would
// ~keep change the shape of every converted document, not fix a content-merging bug.
// ~keep `nav`/`form`/`header`/`footer`/`aside` are excluded from this list even
// ~keep though they can also hit a preprocessing-strip shortcut elsewhere
// ~keep (`is_preprocessing_skip_candidate`): when that shortcut does NOT fire, they
// ~keep get semantic-handler separator behaviour like `<div>`.
fn block_container_is_passthrough(name_lower: &[u8]) -> bool {
    matches!(name_lower, b"colgroup" | b"col" | b"base" | b"html" | b"body")
}

fn emit_open(
    state: &mut Tier1State,
    spec: &'static TagSpec,
    name_lower: &[u8],
    attrs: &[(&[u8], Option<&[u8]>)],
    table_probes: &mut Vec<TableLayoutProbe>,
    options: &ConversionOptions,
) -> Result<(), BailReason> {
    // ~keep Opening any tag ends the "just closed a custom element" boundary
    // ~keep window (see the field's doc comment on `Tier1State`).
    state.last_closed_custom_element = false;
    // ~keep Opening any tag ends the "just emitted an <img>" window too (see
    // ~keep `Tier1State::last_emitted_was_img`); the `TagKind::Image` arm below
    // ~keep re-sets it to true after this reset runs.
    state.last_emitted_was_img = false;

    // ~keep Phase V: when a block-level tag opens inside a link, bail.  Tier-2's
    // ~keep link handler collapses block children (img alt, paragraph text) into
    // ~keep an inline link label; replicating that in Tier-1 requires content
    // ~keep capture similar to Phase R's summary buffer.  Until that lands, bail
    // ~keep so Tier-2's fallback handles the collapse.
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
        TagKind::List(kind) => open_list(state, kind, options),
        TagKind::ListItem => open_list_item(state, options),
        TagKind::DefinitionTerm => open_dt(state),
        TagKind::DefinitionDescription => open_dd(state),
        TagKind::Strong => {
            // ~keep Inside a <summary> accumulation buffer, Tier-2 processes
            // ~keep children with `in_strong: true` which suppresses nested
            // ~keep strong markers.  Mirror that by not pushing `**` when inside
            // ~keep a summary, so `<strong>b</strong>` inside `<summary>` emits
            // ~keep just `b` instead of `**b**`.
            // ~keep Phase FF-2: figcaption uses the same buffer stack but
            // ~keep Tier-2 does NOT set in_strong for figcaption children, so
            // ~keep emit `**` normally when the topmost wrap-buf is a
            // ~keep figcaption (or there's no wrap-buf at all).
            //
            // ~keep `EscapeCtx::STRONG` here is `state.escape_ctx` BEFORE this
            // ~keep tag's own bit is applied (`apply_open_escape_ctx` runs after
            // ~keep `emit_open` returns) — i.e. "is there already a `<strong>`
            // ~keep ancestor" — matching Tier-2's `ctx.in_strong` check in
            // ~keep `handle_strong`, which likewise tests the context inherited
            // ~keep from ancestors before forcing it true for its own children.
            // ~keep Without this, `<strong><strong>x</strong></strong>` emits
            // ~keep `****x****`, which is not valid CommonMark strong emphasis.
            if !state.summary_at_top() && !state.escape_ctx.contains(EscapeCtx::STRONG) {
                // ~keep issue #483: opening `**` right after the buffer's own matching
                // ~keep `**` close (no intervening content) would form one longer
                // ~keep CommonMark delimiter run on reparse, not two independent strong
                // ~keep spans. Tier-2 merges this case (`merge_adjacent_emphasis`);
                // ~keep Tier-1 does not replicate the merge, so it bails instead.
                if state.cell_or_output_mut().ends_with("**") {
                    return Err(BailReason::AdjacentInlineEmphasis);
                }
                state.cell_or_output_mut().push_str("**");
            }
        }
        TagKind::Emphasis => {
            // ~keep issue #483: same reasoning as the `Strong` arm above, for the lone
            // ~keep `*` marker -- but a lone `*` must not match the second half of a `**`
            // ~keep (that is a `<strong>` close, an unrelated tag kind), hence excluding
            // ~keep `ends_with("**")` here.
            let buf = state.cell_or_output_mut();
            if buf.ends_with('*') && !buf.ends_with("**") {
                return Err(BailReason::AdjacentInlineEmphasis);
            }
            buf.push('*');
        }
        TagKind::Strikethrough => {
            // ~keep Tier-2's handle_strikethrough suppresses the `~~` wrapping
            // ~keep when inside `<code>`/`<pre>` (in_code).  Mirror via EscapeCtx.
            if !state.escape_ctx.contains(EscapeCtx::CODE) && !state.escape_ctx.contains(EscapeCtx::PRE) {
                state.cell_or_output_mut().push_str("~~");
            }
        }
        TagKind::Inserted => {
            // ~keep Tier-2's handle_inserted emits `==` markers unconditionally for
            // ~keep <ins>.  Mirror Strikethrough's in-code/pre suppression for
            // ~keep consistency (no `==` inside backtick spans / fenced blocks).
            if !state.escape_ctx.contains(EscapeCtx::CODE) && !state.escape_ctx.contains(EscapeCtx::PRE) {
                state.cell_or_output_mut().push_str("==");
            }
        }
        // ~keep Phase CC: defer the open backtick marker — close_code does
        // ~keep smart escaping based on the content (mirrors Tier-2's
        // ~keep render_code_with_escaping at inline/code.rs:260).  Inside an
        // ~keep outer <code> or <pre>, the inner code is transparent.
        TagKind::Code if !state.escape_ctx.contains(EscapeCtx::PRE) && !state.escape_ctx.contains(EscapeCtx::CODE) => {}
        TagKind::Code if state.pre_lang.is_none() && state.escape_ctx.contains(EscapeCtx::PRE) => {
            if let Some(lang) = extract_language_from_class(attrs) {
                state.pre_lang = Some(lang);
            }
        }
        TagKind::Link => open_link(state),
        TagKind::Table => open_table(state, attrs, table_probes),
        TagKind::TableCaption => open_table_caption(state),
        TagKind::TableHead => open_table_head(state)?,
        TagKind::TableBody => open_table_body(state)?,
        TagKind::TableFoot => open_table_foot(state),
        TagKind::TableRow => open_table_row(state),
        TagKind::TableCell { is_header } => open_table_cell(state, attrs, is_header, table_probes)?,
        // ~keep Block containers: emit a leading blank-line separator when there's
        // ~keep already preceding content.  Mirrors Tier-2's div/sectioning handlers
        // ~keep (`block/div.rs`'s `needs_leading_sep` branch and the separator push in
        // ~keep `semantic/sectioning.rs`) which prefix block content with `\n\n` to
        // ~keep separate it from siblings.
        // ~keep
        // ~keep Inside a table cell, Tier-2's `is_table_continuation` in `div.rs` treats
        // ~keep a sibling-div as a "table continuation" and emits `"  \n"` when
        // ~keep the cell already has non-`|`/non-`<br>` content.  After
        // ~keep `close_table_cell`'s `replace('\n', ' ')` step, this becomes a 3-space
        // ~keep run between sibling divs — matching Tier-2's lists_timeline cell
        // ~keep layout `[link]   [other-link]`.  Without this, Tier-1 emits 1 space.
        TagKind::Block => {
            if block_container_is_passthrough(name_lower) {
                // ~keep No separator, in or out of a table cell: Tier-2's catch-all
                // ~keep handler for these names never emits one. See
                // ~keep `block_container_is_passthrough`'s doc comment.
            } else if state.in_table_cell() {
                let br_in_tables = options.br_in_tables;
                let cell_buf = state.cell_or_output_mut();
                if !cell_buf.is_empty()
                    && !cell_buf.ends_with('|')
                    && !cell_buf.ends_with("<br>")
                    && !cell_buf.ends_with("  \n")
                {
                    // ~keep Routed through the same helper Tier-2's `is_table_continuation`
                    // ~keep branch uses (`block/div.rs` -> `emit_table_cell_break`) rather
                    // ~keep than pushing `"  \n"` unconditionally. The hardcoded form ignored
                    // ~keep `br_in_tables`, which defaults to false: Tier-2 emits a single
                    // ~keep space, while `"  \n"` survives `close_table_cell`'s
                    // ~keep `replace('\n', ' ')` as a three-space run, so two sibling block
                    // ~keep containers in one cell rendered `a   b` here against Tier-2's
                    // ~keep `a b`. The list path was already moved onto this helper; the
                    // ~keep block path was missed.
                    crate::converter::main_helpers::emit_table_cell_break(cell_buf, br_in_tables);
                }
            } else {
                // ~keep Tier-2's `needs_leading_sep` (block/div.rs) appends "\n\n" BLINDLY
                // ~keep whenever the output doesn't already end with a blank line — it does
                // ~keep NOT special-case a lone trailing newline into pushing just one more.
                // ~keep That produces a transient run of 3 newlines when a
                // ~keep single-newline-terminated sibling (a list/table/`<hr>`) precedes this
                // ~keep div; ordinarily harmless, since the final `collapse_excess_blank_lines`
                // ~keep pass folds any 3+ run back down to exactly 2 — EXCEPT when the div's
                // ~keep own first child is a `<blockquote>`, whose entry logic
                // ~keep (`close_blockquote`) inspects the exact trailing state before that
                // ~keep collapse runs and treats "ends with \n\n" (true for the transient
                // ~keep 3-run too) as "pop one", landing on the blank line Tier-2 actually
                // ~keep keeps. `ensure_blank_line`'s normalized (never-3+) output would make
                // ~keep that pop collapse a lone newline straight back down to one, losing the
                // ~keep separator — hence the blind push here instead of `ensure_blank_line`.
                let dest = &mut state.output;
                if !dest.is_empty() && !dest.ends_with("\n\n") {
                    crate::converter::tier1::state::trim_trailing_horizontal(dest);
                    dest.push_str("\n\n");
                }
            }
        }
        // ~keep Summary: push accumulation buffer so children redirect into it (Phase R).
        TagKind::Summary => open_summary(state),
        // ~keep Figcaption: same buffer mechanism as summary (Phase FF-2); the
        // ~keep wrap delimiter differs (`*…*` vs `**…**`) and is emitted by
        // ~keep close_figcaption.
        TagKind::Figcaption => open_figcaption(state),
        // ~keep Button: no leading separator (matches Tier-2 handle_button which
        // ~keep does nothing on open).  Close-side `\n\n` is emitted by close_button.
        TagKind::Button => {}
        TagKind::Inline => {}
        _ => {}
    }

    Ok(())
}

fn open_paragraph(state: &mut Tier1State) {
    // ~keep When inside a table cell, treat `<p>` as a transparent container.
    // ~keep Tier-2's paragraph.rs emits `<br>` when `in_table_cell` and there is
    // ~keep already cell content; we mirror that behaviour so the cell buffer stays
    // ~keep on one logical line (no `\n` in cell output to collapse later).
    if state.in_table_cell() {
        let cell_buf = state.cell_or_output_mut();
        if !cell_buf.is_empty() && !cell_buf.ends_with("<br>") {
            cell_buf.push_str("<br>");
        }
        return;
    }
    // ~keep Mirrors Tier-2: when output is non-empty and doesn't already end
    // ~keep with "\n\n", push "\n\n" (may produce three newlines total when
    // ~keep output ends with a single "\n", e.g. right after a table row or
    // ~keep an `<hr>`).
    // ~keep Phase EE: when the paragraph is the first child of a list-item
    // ~keep (output ends with a freshly-emitted bullet like `- ` or `1. `),
    // ~keep the paragraph content joins the bullet inline.  Tier-2's
    // ~keep paragraph.rs (`is_list_continuation`) only applies this special case
    // ~keep when `ctx.in_list_item` is true -- gate on the same condition here.
    // ~keep Without it, ordinary top-level text that happens to end in "- "/"* "/
    // ~keep "+ "/"N. " (a real bullet-looking suffix, OR the `<strong>`/`<em>`
    // ~keep ambiguity `line_is_bare_list_marker` exists to rule out) wrongly
    // ~keep skipped the "\n\n" separator before a following `<p>` and glued the
    // ~keep two blocks onto a single line, even with no list anywhere in sight.
    // ~keep Check BEFORE `trim_trailing_horizontal`, which would strip the
    // ~keep trailing space from the bullet.
    let in_list_item = state
        .stack
        .iter()
        .any(|frame| matches!(frame.spec.kind, TagKind::ListItem));
    if in_list_item {
        let dest = state.cell_or_output_mut();
        if dest.ends_with("- ") || dest.ends_with("* ") || dest.ends_with("+ ") || ends_with_ordered_marker(dest) {
            return;
        }
    }
    // ~keep Drop trailing horizontal whitespace from inter-tag preservation
    // ~keep (Phase U-2) before the block separator.
    let dest = state.cell_or_output_mut();
    crate::converter::tier1::state::trim_trailing_horizontal(dest);
    if !dest.is_empty() && !dest.ends_with("\n\n") {
        dest.push_str("\n\n");
    }
}

fn open_heading(state: &mut Tier1State) {
    // ~keep When inside a table cell, Tier-2 does NOT add a leading separator before
    // ~keep the heading (`needs_leading_sep = false` when `in_table_cell`).  The
    // ~keep heading text is emitted directly into the cell accumulator with no `#`
    // ~keep prefix and no surrounding newlines.
    if state.in_table_cell() {
        return;
    }
    // ~keep A heading inside `<summary>`/`<figcaption>` is not in a table cell but
    // ~keep also must not touch `state.output`: Tier-2's `handle_summary` walks
    // ~keep children with a fresh LOCAL `content` buffer as `output`, so
    // ~keep `heading.rs`'s leading-separator step runs against that buffer, not
    // ~keep the real document output. `cell_or_output_mut` already resolves to the
    // ~keep active summary/figcaption buffer here (it takes priority over table
    // ~keep cells), so routing through it — instead of the hardcoded
    // ~keep `state.ensure_blank_line()` — keeps the separator (and, in
    // ~keep `close_heading`, the `#` prefix) inside the same buffer the heading's
    // ~keep own text lands in. Without this, `content_start` (captured right after
    // ~keep this call, from that same buffer's length) gets treated as an offset
    // ~keep into `state.output` instead — an unrelated, much larger buffer — and
    // ~keep `close_heading` splices its `#` prefix into the middle of whatever
    // ~keep text happens to sit at that byte offset in the real output.
    ensure_blank_line_buf(state.cell_or_output_mut());
}

/// Buffer-generic equivalent of [`Tier1State::ensure_blank_line`].
///
/// Operates on whichever accumulator buffer the caller passes in — `state.output`,
/// a table cell, or a `<summary>`/`<figcaption>` wrap buffer — rather than assuming
/// `state.output`. See `Tier1State::cell_or_output_mut`'s buffer-selection priority.
fn ensure_blank_line_buf(buf: &mut String) {
    if buf.is_empty() {
        return;
    }
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

fn open_blockquote(state: &mut Tier1State) {
    // ~keep Tier-2's `handle_blockquote` (blockquote.rs) branches on nesting: a
    // ~keep NESTED blockquote (already inside an outer one, per `EscapeCtx::BLOCKQUOTE`)
    // ~keep unconditionally gets a blank-line separator on open. A TOP-LEVEL
    // ~keep blockquote instead computes its separator from whatever the
    // ~keep immediately preceding sibling already left in `output` — see
    // ~keep `close_blockquote`, which needs that untouched pre-open tail, so
    // ~keep nothing is done here for the top-level case.
    if state.escape_ctx.contains(EscapeCtx::BLOCKQUOTE) {
        state.ensure_blank_line();
    }
}

fn open_pre(state: &mut Tier1State, attrs: &[(&[u8], Option<&[u8]>)]) {
    state.ensure_blank_line();
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

/// Strip one bare list marker -- a single bullet char (`-`, `*`, `+`) followed by a space,
/// or one-or-more ASCII digits followed by `". "` -- from the front of `text`, returning
/// what remains after it. Returns `None` when `text` does not start with a marker. Mirrors
/// Tier-2's `strip_leading_bare_marker` (`list/utils.rs`).
fn strip_leading_bare_marker(text: &str) -> Option<&str> {
    let digit_count = text.bytes().take_while(u8::is_ascii_digit).count();
    if digit_count > 0 {
        if let Some(rest) = text[digit_count..].strip_prefix(". ") {
            return Some(rest);
        }
    }
    let mut chars = text.chars();
    let first = chars.next()?;
    if matches!(first, '-' | '*' | '+') {
        return chars.as_str().strip_prefix(' ');
    }
    None
}

/// Whether the current line of `output` (from the last `\n`, or the very start of the
/// buffer) is nothing but one or more bare list markers -- concatenated bullets
/// (`"- "`, `"* "`, `"+ "`) and/or ordered markers (`"N. "`) -- with optional leading
/// indentation and no other content. Mirrors Tier-2's `line_is_bare_list_marker`
/// (`list/utils.rs`).
///
/// ~keep A plain suffix check like `output.ends_with("* ")` also matches the closing
/// ~keep `"**"` of `<strong>` (or the closing `"*"` of `<em>`) immediately followed by a
/// ~keep migrated trailing space, e.g. `"**b** "`: its last two bytes are literally `'*'`
/// ~keep and `' '`, indistinguishable by suffix alone from a real bare `"* "` bullet. That
/// ~keep false positive suppressed the newline before a nested list, flattening it onto
/// ~keep the parent line and destroying it on reparse. Requiring the WHOLE line (after
/// ~keep stripping only leading indentation) to decompose into nothing but marker tokens
/// ~keep rules that out, and also handles several single-child lists nested directly
/// ~keep inside each other, whose bare markers stack on one physical line with nothing
/// ~keep else between them.
fn line_is_bare_list_marker(output: &str) -> bool {
    let line_start = output.rfind('\n').map_or(0, |pos| pos + 1);
    let mut rest = output[line_start..].trim_start_matches([' ', '\t']);
    if rest.is_empty() {
        return false;
    }
    while let Some(next) = strip_leading_bare_marker(rest) {
        if next.is_empty() {
            return true;
        }
        rest = next;
    }
    false
}

fn open_list(state: &mut Tier1State, kind: ListKind, options: &ConversionOptions) {
    // ~keep When inside a table cell, mirror Tier-2's `add_list_leading_separator`:
    // ~keep emit a line-break separator if there is already cell content (but not if it
    // ~keep already ends with `|`, ` `, or `<br>`) -- a literal `<br>` under
    // ~keep `br_in_tables`, otherwise a single space, exactly like
    // ~keep `main_helpers::emit_table_cell_break`.  Do not touch `state.output`.
    if state.in_table_cell() {
        let cell_buf = state.cell_or_output_mut();
        if !cell_buf.is_empty() && !cell_buf.ends_with('|') && !cell_buf.ends_with(' ') && !cell_buf.ends_with("<br>") {
            if options.br_in_tables {
                cell_buf.push_str("<br>");
            } else {
                cell_buf.push(' ');
            }
        }
        state.list_depth = state.list_depth.saturating_add(1);
        if matches!(kind, ListKind::Unordered) {
            state.ul_depth = state.ul_depth.saturating_add(1);
        }
        return;
    }
    let current_list_depth = state.list_depth;
    {
        let dest = state.cell_or_output_mut();
        if !dest.is_empty() {
            if current_list_depth == 0 {
                // ~keep Mirror Tier-2's top-level `add_list_leading_separator` branch
                // ~keep (`!ctx.in_list`, list/utils.rs): append "\n\n" unless the tail is
                // ~keep already a blank line or the current line is nothing but a bare
                // ~keep list marker (see `line_is_bare_list_marker`'s doc comment for why
                // ~keep a plain suffix check on "* "/"- "/". " is not enough here).
                let needs_newline = !dest.ends_with("\n\n") && !line_is_bare_list_marker(dest);
                if needs_newline {
                    dest.push_str("\n\n");
                }
            } else {
                // ~keep Mirror Tier-2's `ctx.in_list_item` branch the same way: the same
                // ~keep whole-line bare-marker check (against a bare newline instead of a
                // ~keep blank line), trimming ONLY once it actually decides to insert the
                // ~keep separator — not eagerly beforehand.
                let needs_newline = !dest.ends_with('\n') && !line_is_bare_list_marker(dest);
                if needs_newline {
                    crate::converter::tier1::state::trim_trailing_horizontal(dest);
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

fn open_list_item(state: &mut Tier1State, options: &ConversionOptions) {
    // ~keep When inside a table cell, Tier-2 does NOT emit bullet/number prefixes
    // ~keep for list items (see list/item.rs: `if !ctx.in_table_cell { ... bullet ... }`).
    // ~keep Sibling <li>s still need a boundary though — mirror Tier-2's reuse of
    // ~keep `add_list_leading_separator` per item (list/item.rs's `else if
    // ~keep ctx.in_table_cell` arm) with the same continuation condition already used by
    // ~keep `open_list` above: a literal `<br>` under `br_in_tables`, otherwise a single
    // ~keep space (mirroring `main_helpers::emit_table_cell_break`), so
    // ~keep `<li>a</li><li>b</li>` in a cell becomes `a<br>b` (or `a b` with
    // ~keep `br_in_tables: false`) instead of the two items' raw text running together.
    if state.in_table_cell() {
        if find_parent_list_kind(&state.stack) == Some(ListKind::Ordered) {
            increment_ol_counter(&mut state.stack);
        }
        let cell_buf = state.cell_or_output_mut();
        if !cell_buf.is_empty() && !cell_buf.ends_with('|') && !cell_buf.ends_with(' ') && !cell_buf.ends_with("<br>") {
            if options.br_in_tables {
                cell_buf.push_str("<br>");
            } else {
                cell_buf.push(' ');
            }
        }
        return;
    }
    let parent_kind = find_parent_list_kind(&state.stack);
    let indent_depth = state.list_depth.saturating_sub(1);
    // ~keep Mirror Tier-2's fresh-line-only indent (list/item.rs): a nested list that is
    // ~keep the sole/first content of its enclosing <li> renders directly after that
    // ~keep parent's own bare marker on the SAME physical line -- the parent marker's own
    // ~keep printed width already reaches this item's target column, so indenting here
    // ~keep too would double-count it. The indent is only needed when this item genuinely
    // ~keep starts a fresh physical line.
    if indent_depth > 0 && (state.output.is_empty() || state.output.ends_with('\n')) {
        push_list_item_indent(&mut state.output, indent_depth);
    }
    if parent_kind == Some(ListKind::Ordered) {
        let counter = increment_ol_counter(&mut state.stack);
        let start = find_ol_start(&state.stack);
        let index = start.saturating_sub(1) + counter;
        let marker = format!("{index}. ");
        state.list_item_marker_widths.push(marker.len());
        state.output.push_str(&marker);
    } else {
        let bullet_idx = state.ul_depth.saturating_sub(1) as usize % TIER1_BULLETS.len();
        state.list_item_marker_widths.push(2);
        state.output.push(TIER1_BULLETS[bullet_idx] as char);
        state.output.push(' ');
    }
}

fn open_link(state: &mut Tier1State) {
    // ~keep Track link count inside tables for layout-table detection.
    if let Some(ts) = state.table_stack.last_mut() {
        ts.link_count += 1;
    }
    state.cell_or_output_mut().push('[');
}

/// Per-table inputs to Tier-2's layout-table heuristic that `TableState` does not
/// already carry.
///
/// Mirrors the three `TableScan` fields (`block/table/scanner.rs`) that Tier-2's
/// `looks_like_layout` reads in `block/table/builder.rs`:
///
/// ```text
/// looks_like_layout = nested_table_count > 1 || distinct_counts.len() > 1
///                                            || (has_span && has_border_zero)
/// ```
///
/// Only the middle term is derivable from `TableState` (as `inconsistent_cols` in
/// [`close_table`]); the other two are collected here.  One entry is pushed by
/// [`open_table`] and popped by [`close_table`], in lockstep with
/// `Tier1State::table_stack`, so `last_mut()` is always the innermost open table.
#[derive(Debug, Clone, Copy, Default)]
struct TableLayoutProbe {
    /// Number of directly-nested `<table>` elements closed inside this table.
    ///
    /// Counts one level only: a table nested inside a nested table increments its
    /// immediate parent, never this frame — matching Tier-2's `scan_own_structure`,
    /// which stops its walk at each nested `<table>` boundary.
    nested_table_count: usize,
    /// True once any cell in this table carried a `colspan`/`rowspan` attribute.
    has_span: bool,
    /// True when the `<table>` tag carried `border="0"` exactly.
    border_zero: bool,
}

fn open_table(state: &mut Tier1State, attrs: &[(&[u8], Option<&[u8]>)], table_probes: &mut Vec<TableLayoutProbe>) {
    // ~keep Phase HH: nested tables are accumulated natively; an inner table inherits
    // ~keep `inline_mode = true` so its final GFM rendering writes into the parent
    // ~keep cell buffer rather than `state.output`.  The parent cell's newline
    // ~keep collapse then flattens the inner table to a single inline run.
    let inline_mode = !state.table_stack.is_empty();
    state.table_stack.push(crate::converter::tier1::state::TableState {
        inline_mode,
        ..Default::default()
    });
    // ~keep Tier-2 compares the raw attribute value against the literal string "0"
    // ~keep (`builder.rs`: `b.as_utf8_str() == "0"`), with no trimming and no numeric
    // ~keep parse: `border="00"`, `border=" 0"` and a valueless `border` are all NOT
    // ~keep border-zero there, so they must not be here either.
    table_probes.push(TableLayoutProbe {
        border_zero: find_attr(attrs, b"border").is_some_and(|value| value == b"0".as_slice()),
        ..TableLayoutProbe::default()
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
        if ts.seen_tbody_close || ts.seen_tfoot {
            return Err(BailReason::TableSectionOrder);
        }
        ts.in_thead = true;
    }
    Ok(())
}

fn open_table_body(state: &mut Tier1State) -> Result<(), BailReason> {
    if let Some(ts) = state.table_stack.last_mut() {
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
    if let Some(ts) = state.table_stack.last_mut() {
        ts.current_row.clear();
    }
}

fn open_table_cell(
    state: &mut Tier1State,
    attrs: &[(&[u8], Option<&[u8]>)],
    is_header: bool,
    table_probes: &mut [TableLayoutProbe],
) -> Result<(), BailReason> {
    // ~keep Tier-2's `has_span` (block/table/scanner.rs::scan_row_cells) is set by the
    // ~keep mere *presence* of a `colspan`/`rowspan` attribute — `attrs.get(k).is_some()`
    // ~keep — so `colspan="1"` and a valueless `colspan` both count.  Deliberately NOT
    // ~keep `value > 1`: this feeds `looks_like_layout` in close_table and a tighter
    // ~keep predicate would leave the byte-equality divergence in place for exactly the
    // ~keep tables it excluded.
    let spanning = has_attr(attrs, b"colspan") || has_attr(attrs, b"rowspan");
    if let Some(probe) = table_probes.last_mut() {
        probe.has_span |= spanning;
    }
    // ~keep rowspan: accepted but not expanded (lossy — a spanned cell renders once,
    // ~keep matching mdream).  colspan: expanded by `close_table_cell` adding
    // ~keep `(colspan - 1)` empty cells so Tier-2's column-count expectations are
    // ~keep met (without this, infobox-style `<th colspan="2">` rows trigger Tier-2's
    // ~keep layout-table fallback in close_table on what should be a normal GFM table).
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
    // ~keep A void element closes the "just closed a custom element" boundary
    // ~keep window too (see the field's doc comment on `Tier1State`).
    state.last_closed_custom_element = false;
    // ~keep Closes the "just emitted an <img>" window too (see
    // ~keep `Tier1State::last_emitted_was_img`); the `TagKind::Image` arm below
    // ~keep re-sets it to true after this reset runs.
    state.last_emitted_was_img = false;

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
            // ~keep `<br>` outside any block context emits nothing (Tier-2 behaviour).
            // ~keep Three context-dependent emissions:
            // ~keep   - Inside a link (anywhere): `"  \n"`, unmodified, UNLESS the link's
            // ~keep     body is still empty (nothing emitted since the `<a>`/wrapper
            // ~keep     opened), in which case nothing is emitted at all. Tier-2's
            // ~keep     `normalize_link_label` (utility/content.rs) now preserves this
            // ~keep     exact marker mid-label instead of collapsing it (CommonMark
            // ~keep     spec examples 642/643 — a hard break inside link text is legal
            // ~keep     and must survive a convert/render/convert round trip), trimming
            // ~keep     it away only if it ends up at the label's start/end — a break
            // ~keep     with nothing before it has no preceding line to break, so a
            // ~keep     leading `<br>` (`<a><br>bar</a>`) must produce `[bar]`, not
            // ~keep     `[  \nbar]`. Router (`router.rs`) bails Tier-1 whenever
            // ~keep     `newline_style` is not `Spaces`, so this literal is the only
            // ~keep     marker Tier-1 ever needs to match. No trim beforehand otherwise,
            // ~keep     matching the non-heading branch below and Tier-2's own
            // ~keep     `line_break.rs`, which does not trim here either — only the
            // ~keep     label's own start/end trimming (in `normalize_link_label` /
            // ~keep     `close_link` below) cleans up the ends.
            // ~keep   - Inside a table cell (not in a link): mirrors Tier-2's
            // ~keep     `emit_table_cell_break` (main_helpers.rs) — trim trailing
            // ~keep     spaces/tabs, then emit a literal `<br>` when `br_in_tables`
            // ~keep     is true, or collapse to a single space (guarded against a
            // ~keep     leading space on an empty cell) otherwise.  `newline_style`
            // ~keep     is never consulted inside a cell (issue #453, issue #454).
            // ~keep   - Inside a regular block (paragraph, div, etc.): `"  \n"`, with any
            // ~keep     trailing whitespace already in the buffer (e.g. a decoded
            // ~keep     `&nbsp;` folded to a plain space) trimmed first so the hard-break
            // ~keep     prefix is exactly two spaces, not two-plus-N.  Mirrors the
            // ~keep     table-cell branch's own `trim_trailing_whitespace` call below.
            let link_frame_content_start = state
                .stack
                .iter()
                .rev()
                .find_map(|f| matches!(f.spec.kind, TagKind::Link).then_some(f.content_start));
            if let Some(link_content_start) = link_frame_content_start {
                if link_content_start < state.cell_or_output_mut().len() {
                    state.cell_or_output_mut().push_str("  \n");
                }
            } else if state.in_table_cell() {
                let dest = state.cell_or_output_mut();
                crate::converter::main_helpers::trim_trailing_whitespace(dest);
                if options.br_in_tables {
                    dest.push_str("<br>");
                } else if !dest.is_empty() {
                    dest.push(' ');
                }
            } else if state.stack.is_empty() {
                // ~keep bare `<br>` at top level — Tier-2 emits nothing
            } else {
                let dest = state.cell_or_output_mut();
                crate::converter::main_helpers::trim_trailing_whitespace(dest);
                dest.push_str("  \n");
            }
        }

        TagKind::Image => {
            let src = find_attr(attrs, b"src").unwrap_or_default();

            // ~keep Tier-2's `handle_img` (`handlers/image.rs`'s `resolve_effective_src`,
            // ~keep landed in commit 55699777d5, after this scanner's pinned baseline) falls
            // ~keep back to `data-src` / `data-lazy-src` / `data-original` / `data-srcset` /
            // ~keep `srcset` -- in that precedence order -- whenever `src` is empty/whitespace
            // ~keep or a `data:` URI (the standard lazy-loading placeholder pattern: the real
            // ~keep image URL sits in a `data-*` attribute until the element scrolls into
            // ~keep view). See `BailReason::ImageLazyLoadSrc`'s doc comment for why this bails
            // ~keep rather than reimplementing that precedence (including `srcset`'s
            // ~keep width/density-descriptor comparison) here. A plain `<img src="x.png">`
            // ~keep with none of the fallback attributes is unaffected and stays on this path.
            let src_is_lazy_load_placeholder = {
                let trimmed = src.trim_ascii();
                trimmed.is_empty() || trimmed.starts_with(b"data:")
            };
            if src_is_lazy_load_placeholder
                && [
                    b"data-src".as_slice(),
                    b"data-lazy-src".as_slice(),
                    b"data-original".as_slice(),
                    b"data-srcset".as_slice(),
                    b"srcset".as_slice(),
                ]
                .iter()
                .any(|name| find_attr(attrs, name).is_some())
            {
                return Err(BailReason::ImageLazyLoadSrc);
            }

            let alt = find_attr(attrs, b"alt").unwrap_or_default();
            let title = find_attr(attrs, b"title");

            // ~keep Phase DD: src gets entity-decoding (URL semantics).
            // ~keep For alt/title:
            // ~keep   • With custom-element tags → T2 ran html5ever roundtrip
            // ~keep     and canonicalized entities; decode + re-encode the
            // ~keep     special set to match.
            // ~keep   • Without → T2 just yields tl's raw attribute bytes;
            // ~keep     keep entities verbatim.
            let src = decode_attr(src)?;
            let canonicalize = state.canonicalize_attr_entities;
            let alt_owned;
            let alt: &str = if canonicalize {
                alt_owned = canonicalize_attr_entities(&decode_attr(alt)?).into_owned();
                &alt_owned
            } else {
                let raw = std::str::from_utf8(alt).map_err(|_| BailReason::Classifier)?;
                bail_if_canonicalization_is_undecidable(raw, &decode_attr(alt)?)?;
                raw
            };

            let keep_as_markdown = should_keep_image_as_markdown(html, &state.stack, options);

            let dest = state.cell_or_output_mut();
            if keep_as_markdown {
                // ~keep Security fix mirror (`handlers/image.rs::format_image_markdown`,
                // ~keep outside tier1/ — same class of bug as the SVG title / link label
                // ~keep fixes above): an unescaped `alt` lets `x](https://evil.example`
                // ~keep close the image label early and open a second,
                // ~keep attacker-controlled Markdown image/link. Only applied on this
                // ~keep (markdown-emitting) branch — the `!keep_as_markdown` branch below
                // ~keep emits `alt` as plain text with no `![...]` wrapping at all,
                // ~keep matching `format_image_markdown`'s `use_alt_only` branch, which
                // ~keep also does not call `escape_link_label`.
                let escaped_alt = crate::converter::utility::content::escape_link_label(alt);
                if let Some(title_bytes) = title {
                    let title_owned;
                    let title_str: &str = if canonicalize {
                        title_owned = canonicalize_attr_entities(&decode_attr(title_bytes)?).into_owned();
                        &title_owned
                    } else {
                        let raw = std::str::from_utf8(title_bytes).map_err(|_| BailReason::Classifier)?;
                        bail_if_canonicalization_is_undecidable(raw, &decode_attr(title_bytes)?)?;
                        raw
                    };
                    #[allow(clippy::format_push_string)]
                    dest.push_str(&format!("![{escaped_alt}]({src} \"{title_str}\")"));
                } else {
                    #[allow(clippy::format_push_string)]
                    dest.push_str(&format!("![{escaped_alt}]({src})"));
                }
            } else {
                // ~keep Strip to alt-text only — mirrors Tier-2 behaviour when the image
                // ~keep is in a heading whose tag is not in `keep_inline_images_in`.
                dest.push_str(alt);
            }
            // ~keep Set regardless of `keep_as_markdown` — Tier-2's `is_empty_inline_element`
            // ~keep (paragraph.rs) checks the DOM tag name only, not how it renders.
            state.last_emitted_was_img = true;
        }

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
#[allow(clippy::missing_const_for_fn)]
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
        // ~keep No restriction — always emit markdown image (Tier-2 default).
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
            return false;
        }
    }
    // ~keep No heading ancestor at all: no restriction applies — emit markdown image.
    // ~keep This matches Tier-2 behaviour: the `keep_inline_images_in` guard only
    // ~keep fires when `ctx.in_heading` is true.
    true
}

/// Byte-level ASCII case-insensitive comparison — no allocation.
#[cfg(feature = "inline-images")]
fn eq_ascii_ignore_case(a: &[u8], b: &[u8]) -> bool {
    a.eq_ignore_ascii_case(b)
}

/// Returns `true` when `bytes[pos..]` opens a `<script` or `<style` tag with no
/// separating whitespace — i.e. a second raw-text-ignored element sitting directly
/// adjacent to the one the scanner just finished skipping.  See the bail site in
/// the `TagKind::Ignored`-and-`is_rawtext` branch above for why this forces a
/// Tier-2 fallback rather than being handled inline.
fn is_adjacent_rawtext_ignored_open(bytes: &[u8], pos: usize) -> bool {
    const CANDIDATES: [&[u8]; 2] = [b"script", b"style"];
    if bytes.get(pos) != Some(&b'<') {
        return false;
    }
    let name_start = pos + 1;
    for name in CANDIDATES {
        let name_end = name_start + name.len();
        if bytes.len() < name_end {
            continue;
        }
        if !bytes[name_start..name_end].eq_ignore_ascii_case(name) {
            continue;
        }
        // ~keep Require a valid tag-name terminator so `<scriptx>` doesn't match.
        if let Some(b' ' | b'\t' | b'\n' | b'\r' | b'>' | b'/') = bytes.get(name_end) {
            return true;
        }
    }
    false
}

fn emit_close(
    state: &mut Tier1State,
    tag_name_bytes: &[u8],
    options: &ConversionOptions,
    table_probes: &mut Vec<TableLayoutProbe>,
) -> Result<(), BailReason> {
    let mut name_buf = [0u8; MAX_TAG_NAME_BYTES];
    let name_lower = lowercase_into(tag_name_bytes, &mut name_buf);

    // ~keep Custom element close tags (e.g. `</x-foo>`) use the same static Inline
    // ~keep spec as their corresponding open tag.  All other unknown close tags bail.
    let spec: &'static TagSpec = if name_lower.contains(&b'-') {
        &CUSTOM_ELEMENT_INLINE_SPEC
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

    // ~keep Closing any tag ends the "just emitted an <img>" window too (see
    // ~keep `Tier1State::last_emitted_was_img`) — an intervening close means the
    // ~keep two images are not both unwrapped direct siblings any more.
    state.last_emitted_was_img = false;

    while let Some(top) = state.stack.last() {
        if kinds_match(&top.spec.kind, &spec.kind) {
            break;
        }
        if top.spec.optional_close.is_some() {
            emit_close_for_implicit(state, options, table_probes)?;
        } else {
            break;
        }
    }

    // ~keep Pop the matching frame from the open-tag stack.
    // ~keep Tier-2 is lenient about mismatched tags; for M3c we bail.
    let actual_depth = state.stack.len() as u8;
    let frame = pop_matching_frame(&mut state.stack, spec).ok_or_else(|| BailReason::DepthMismatch {
        tag: bytes_to_string(name_lower),
        expected: 1,
        actual: actual_depth,
    })?;

    state.escape_ctx = frame.prev_escape_ctx;
    state.last_closed_custom_element = std::ptr::eq(spec, &raw const CUSTOM_ELEMENT_INLINE_SPEC);

    match spec.kind {
        TagKind::Paragraph => close_paragraph(state),
        TagKind::Heading(n) => close_heading(state, &frame, n, false)?,
        TagKind::Blockquote => close_blockquote(state, &frame),
        TagKind::Pre => close_pre(state, &frame, options),
        // ~keep Strong: suppress close marker when inside summary, or when this
        // ~keep frame nested inside another `<strong>` and so never emitted an
        // ~keep open marker either (see open-side guard) — `state.escape_ctx` was
        // ~keep just restored to `frame.prev_escape_ctx` above.
        TagKind::Strong if state.summary_at_top() || state.escape_ctx.contains(EscapeCtx::STRONG) => {}
        TagKind::Strong => close_inline_marker(state, &frame, "**")?,
        TagKind::Emphasis => close_inline_marker(state, &frame, "*")?,
        TagKind::Strikethrough
            if state.escape_ctx.contains(EscapeCtx::CODE) || state.escape_ctx.contains(EscapeCtx::PRE) => {}
        TagKind::Strikethrough => close_inline_marker(state, &frame, "~~")?,
        TagKind::Inserted
            if state.escape_ctx.contains(EscapeCtx::CODE) || state.escape_ctx.contains(EscapeCtx::PRE) => {}
        TagKind::Inserted => close_inline_marker(state, &frame, "==")?,
        TagKind::Code => close_code(state, &frame),
        TagKind::Link => close_link(state, &frame, options)?,
        TagKind::List(ListKind::Definition) => close_dl(state, &frame),
        TagKind::List(kind) => close_list(state, kind),
        TagKind::ListItem => close_list_item(state, &frame),
        TagKind::DefinitionTerm => close_dt(state),
        TagKind::DefinitionDescription => close_dd(state),
        TagKind::Hr => {}
        TagKind::Table => close_table(state, options, table_probes)?,
        TagKind::TableHead => close_table_head(state),
        TagKind::TableBody => close_table_body(state),
        TagKind::TableFoot => {}
        TagKind::TableRow => close_table_row(state),
        TagKind::TableCell { .. } => close_table_cell(state, false)?,
        TagKind::TableCaption => close_table_caption(state),
        // ~keep Generic block container close: when it produced visible content,
        // ~keep ensure a paragraph-break separator follows so the next sibling
        // ~keep doesn't run together with this div's last byte.  Mirrors Tier-2's
        // ~keep `div::handle` post-children block: `output.push_str("\n\n")` when
        // ~keep `has_content` (see block/div.rs around line 124-130).
        TagKind::Block => close_block_container(state, &frame, name_lower),
        // ~keep Summary: pop accumulation buffer, trim, emit `**…**\n\n` (Phase R).
        TagKind::Summary => close_summary(state, &frame),
        // ~keep Figcaption: pop accumulation buffer, trim, emit `*…*\n\n` (Phase FF-2).
        TagKind::Figcaption => close_figcaption(state, &frame),
        // ~keep Button (Phase T): emit `\n\n` when content was produced — mirrors
        // ~keep Tier-2 `form/elements.rs`'s `handle_button`.  No leading separator on open.
        TagKind::Button => close_button(state, &frame),
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
        TagKind::LineBreak | TagKind::Image => {}
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
fn close_block_container(state: &mut Tier1State, frame: &OpenTag, name_lower: &[u8]) {
    if block_container_is_passthrough(name_lower) {
        // ~keep Mirrors the open-side skip in `emit_open`'s `TagKind::Block` arm:
        // ~keep Tier-2's catch-all handler for these names never emits a trailing
        // ~keep separator either.
        return;
    }
    if state.in_table_cell() {
        return;
    }
    let buf = state.cell_or_output_mut();
    if buf.len() <= frame.content_start {
        return;
    }
    // ~keep Drop trailing horizontal whitespace (left over from inter-tag whitespace
    // ~keep preservation) before emitting the block separator.  Same rationale as
    // ~keep `ensure_blank_line` (Phase U-2).
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

// ~keep ── Summary strong-wrap (Phase R) ────────────────────────────────────────────

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
    // ~keep Pop the buffer we pushed in open_summary.
    let buf = match state.pop_summary_buf() {
        Some(b) => b,
        None => return,
    };
    let trimmed = buf.trim();
    if trimmed.is_empty() {
        return;
    }
    // ~keep Acquire the parent destination.  Because we already popped the buffer
    // ~keep above, cell_or_output_mut now returns the next-outer target — which may
    // ~keep be the table cell buffer (when the summary was inside a <td>), an outer
    // ~keep summary buffer, or the main output.
    // ~keep
    // ~keep Check whether we're emitting into a table cell BEFORE borrowing `dest`,
    // ~keep so we can decide whether to add a leading separator without conflicting
    // ~keep with the mutable borrow.
    let writing_to_cell = state.in_table_cell();
    let dest = state.cell_or_output_mut();
    // ~keep Ensure a blank-line separator before the summary block when there is
    // ~keep preceding content and we're NOT writing to a table cell (cells are
    // ~keep rendered to a single line; block separators would be collapsed anyway).
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

// ~keep ── Figcaption italic-wrap (Phase FF-2) ──────────────────────────────────────

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
    // ~keep Phase FF-2: trim trailing horizontal whitespace introduced by
    // ~keep Phase U-2's inter-tag-whitespace preservation, so the block
    // ~keep separator (\n\n) doesn't sit after a stray space.  Tier-2 does
    // ~keep not emit that space when the figcaption follows inline content.
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
/// Mirrors the block-separator tail of Tier-2 `form/elements.rs`'s `handle_button`:
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
    // ~keep Drop trailing horizontal whitespace from the inter-tag fix before the
    // ~keep block separator (Phase U-2).
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

/// Clamp a stored byte offset (e.g. `OpenTag::content_start`, captured as
/// `buf.len()` when the tag opened) to a valid, in-bounds char boundary of
/// `buf` as it stands *now*.
///
/// `content_start` is read back at close time, sometimes against a different
/// buffer than the one it was captured against (`state.output` vs. the
/// current table-cell accumulator — see the `state.in_table_cell()` branches
/// throughout this file) or after other frames' close handlers have mutated
/// the buffer. On the correct path `content_start` is already valid, so this
/// is a no-op there (500k-case fuzzing under `TierStrategy::Auto` never hit a
/// clamp); it exists so a stale offset degrades to a clamped position instead
/// of an out-of-bounds or not-a-char-boundary panic in `&buf[start..]`,
/// `buf.truncate(start)`, or `buf.insert_str(start, …)`.
fn clamp_to_char_boundary(buf: &str, at: usize) -> usize {
    let mut at = at.min(buf.len());
    while at > 0 && !buf.is_char_boundary(at) {
        at -= 1;
    }
    at
}

/// Close an inline emphasis-style element (`<strong>`, `<em>`, `<b>`, `<i>`).
///
/// When the element produced no visible content (the source had `<strong></strong>`
/// or `<i>   </i>`), erase the open marker too instead of emitting an empty
/// `**` / `*` pair.  Tier-2's DOM walker reaches the same result by emitting
/// nothing for an empty inline node; the byte-equality oracle requires us to
/// match that.
fn close_inline_marker(state: &mut Tier1State, frame: &OpenTag, marker: &str) -> Result<(), BailReason> {
    let buf = state.cell_or_output_mut();
    let content_start = clamp_to_char_boundary(buf, frame.content_start);
    let content_is_absent = buf.len() <= content_start;
    let is_whitespace_only_not_empty = !content_is_absent
        && buf[content_start..]
            .bytes()
            .all(|b| matches!(b, b' ' | b'\t' | b'\n' | b'\r'));
    let body_is_empty = content_is_absent || is_whitespace_only_not_empty;
    if body_is_empty {
        // ~keep issue #481: a body that is (or, per `dropped_whitespace_only_text`, WAS
        // ~keep before `flush_text` dropped it) non-empty but entirely whitespace (e.g.
        // ~keep `<i> </i>`) is not the same as a genuinely empty one (`<i></i>`) --
        // ~keep Tier-2's `chomp_inline` preserves it as a single space OUTSIDE the
        // ~keep markers, while erasing the markers here (as the genuinely-empty case
        // ~keep correctly does) would silently drop that space. Only bail for
        // ~keep strong/emphasis markers (`**`/`*`) -- Strikethrough (`~~`) and
        // ~keep Inserted (`==`) share this function but are not part of issue #481.
        let was_whitespace_only =
            is_whitespace_only_not_empty || (content_is_absent && frame.dropped_whitespace_only_text);
        if was_whitespace_only && matches!(marker, "**" | "*") {
            return Err(BailReason::WhitespaceOnlyInlineEmphasis);
        }
        let open_marker_start = clamp_to_char_boundary(buf, content_start.saturating_sub(marker.len()));
        buf.truncate(open_marker_start);
        return Ok(());
    }

    // ~keep Mirror Tier-2's `chomp_inline` (utility/content.rs:31): leading/trailing
    // ~keep whitespace (including Unicode whitespace like NBSP `\u{a0}`) inside the
    // ~keep strong/emphasis markers gets pushed OUTSIDE them so `**\u{a0}X**` becomes
    // ~keep `\u{a0}**X**`.  Required for byte-equality on Wikipedia fixtures with
    // ~keep `<b><span>&nbsp;</span>X</b>` patterns.
    let content_str = &buf[content_start..];
    let leading_len = content_str.len() - content_str.trim_start().len();
    // ~keep `content_start` is re-bound (not just conditionally mutated in place)
    // ~keep because the trailing check below indexes `buf` at this offset again:
    // ~keep moving `leading_len` bytes from right after the open marker to right
    // ~keep before it shifts every later byte's position forward by `leading_len`
    // ~keep without changing the buffer's total length, so the ORIGINAL
    // ~keep `content_start` value no longer points at the start of the real
    // ~keep content — it now lands `marker.len()` bytes into whatever sits at
    // ~keep the old marker position, which is a char boundary only by
    // ~keep coincidence. Reusing the stale value panicked
    // ~keep ("byte index is not a char boundary") on `<em>\u{2003}x</em>`-shaped
    // ~keep input, where the 1-byte `*` marker and the 3-byte migrated space
    // ~keep don't line up.
    let content_start = if leading_len > 0 {
        let leading: String = content_str[..leading_len].to_owned();
        buf.replace_range(content_start..content_start + leading_len, "");
        let marker_start = clamp_to_char_boundary(buf, content_start.saturating_sub(marker.len()));
        buf.insert_str(marker_start, &leading);
        content_start + leading_len
    } else {
        content_start
    };

    // ~keep Trailing counterpart of the leading-whitespace migration above: a
    // ~keep trailing whitespace run (e.g. a decoded `&nbsp;` folded to a plain
    // ~keep space) is pushed OUTSIDE the closing marker instead of staying
    // ~keep inside it, matching Tier-2's `chomp_inline` suffix handling.
    let content_str = &buf[content_start..];
    let trailing_len = content_str.len() - content_str.trim_end().len();
    if trailing_len > 0 {
        let trailing_start = buf.len() - trailing_len;
        let trailing: String = buf[trailing_start..].to_owned();
        buf.truncate(trailing_start);
        buf.push_str(marker);
        buf.push_str(&trailing);
        return Ok(());
    }

    buf.push_str(marker);
    Ok(())
}

/// Implicitly close the top-of-stack frame without a matching `</tag>` in the
/// input.  Called by the M4 implicit-close loop when HTML5 optional-tag rules
/// require an open element to be closed before the next tag is pushed.
///
/// Mirrors `emit_close` but skips the stack-pop search (we always close the
/// literal top frame) and skips the tag-name lookup (we use the frame's spec
/// directly).
fn emit_close_for_implicit(
    state: &mut Tier1State,
    options: &ConversionOptions,
    table_probes: &mut Vec<TableLayoutProbe>,
) -> Result<(), BailReason> {
    let frame = state.stack.pop().ok_or_else(|| BailReason::DepthMismatch {
        tag: String::from("(implicit)"),
        expected: 1,
        actual: 0,
    })?;
    let spec = frame.spec;

    state.escape_ctx = frame.prev_escape_ctx;
    state.last_closed_custom_element = std::ptr::eq(spec, &raw const CUSTOM_ELEMENT_INLINE_SPEC);

    match spec.kind {
        TagKind::Paragraph => close_paragraph(state),
        TagKind::Heading(n) => close_heading(state, &frame, n, true)?,
        TagKind::Blockquote => close_blockquote(state, &frame),
        TagKind::Pre => close_pre(state, &frame, options),
        // ~keep Strong: suppress close marker when inside summary, or when this
        // ~keep frame nested inside another `<strong>` and so never emitted an
        // ~keep open marker either (see open-side guard) — `state.escape_ctx` was
        // ~keep just restored to `frame.prev_escape_ctx` above.
        TagKind::Strong if state.summary_at_top() || state.escape_ctx.contains(EscapeCtx::STRONG) => {}
        TagKind::Strong => close_inline_marker(state, &frame, "**")?,
        TagKind::Emphasis => close_inline_marker(state, &frame, "*")?,
        TagKind::Strikethrough
            if state.escape_ctx.contains(EscapeCtx::CODE) || state.escape_ctx.contains(EscapeCtx::PRE) => {}
        TagKind::Strikethrough => close_inline_marker(state, &frame, "~~")?,
        TagKind::Inserted
            if state.escape_ctx.contains(EscapeCtx::CODE) || state.escape_ctx.contains(EscapeCtx::PRE) => {}
        TagKind::Inserted => close_inline_marker(state, &frame, "==")?,
        TagKind::Code => close_code(state, &frame),
        TagKind::Link => close_link(state, &frame, options)?,
        TagKind::List(ListKind::Definition) => close_dl(state, &frame),
        TagKind::List(kind) => close_list(state, kind),
        TagKind::ListItem => close_list_item(state, &frame),
        TagKind::DefinitionTerm => close_dt(state),
        TagKind::DefinitionDescription => close_dd(state),
        TagKind::TableCell { .. } => close_table_cell(state, true)?,
        TagKind::TableRow => close_table_row(state),
        // ~keep Summary: pop accumulation buffer, trim, emit `**…**\n\n` (Phase R).
        TagKind::Summary => close_summary(state, &frame),
        // ~keep Figcaption: pop accumulation buffer, trim, emit `*…*\n\n` (Phase FF-2).
        TagKind::Figcaption => close_figcaption(state, &frame),
        // ~keep Button (Phase T): emit `\n\n` on EOF close just like explicit close.
        TagKind::Button => close_button(state, &frame),
        // ~keep An unclosed `<table>` at EOF (html5ever/tl both implicitly close every
        // ~keep open element there, per the loop's own doc comment) used to hit the
        // ~keep do-nothing arm below, discarding the WHOLE accumulated table --
        // ~keep including fully-formed rows -- instead of rendering it. `close_table`
        // ~keep already tolerates an incomplete/empty table (its `is_blank` check
        // ~keep bails rather than emitting), so it is exactly as safe to call here as
        // ~keep it is from `emit_close`'s explicit `</table>` arm.
        TagKind::Table => close_table(state, options, table_probes)?,
        TagKind::Block | TagKind::Inline => {}
        TagKind::LineBreak
        | TagKind::Image
        | TagKind::Hr
        | TagKind::TableHead
        | TagKind::TableBody
        | TagKind::TableFoot
        | TagKind::TableCaption
        | TagKind::RawText(_)
        | TagKind::Ignored => {}
    }

    Ok(())
}

fn close_paragraph(state: &mut Tier1State) {
    // ~keep When inside a table cell, `<p>` is transparent — no block separators.
    // ~keep Any inter-paragraph separators were already added as `<br>` at open time
    // ~keep by `open_paragraph`; `close_paragraph` does nothing in this context.
    if state.in_table_cell() {
        return;
    }
    // ~keep Tier-2 appends "\n\n" after paragraph content (always two newlines).
    // ~keep Matching this precisely is required for byte-equal output.
    trim_trailing_inline_whitespace(state);
    state.cell_or_output_mut().push_str("\n\n");
}

/// Close a heading element.
///
/// When `is_implicit` is true the empty-heading guard is skipped: implicitly
/// closed headings have already had their content flushed through the normal
/// path, so we just prepend the prefix unconditionally.
fn close_heading(state: &mut Tier1State, frame: &OpenTag, n: u8, is_implicit: bool) -> Result<(), BailReason> {
    // ~keep When inside a table cell, Tier-2 emits the heading text directly into
    // ~keep the cell accumulator — no `#` prefix, no block separators.  The
    // ~keep `frame.content_start` is a position in the CELL buffer (set by
    // ~keep `cell_or_output_mut().len()` at emit_open time), so all position
    // ~keep arithmetic must use the cell buffer, not `state.output`.
    if state.in_table_cell() {
        let cell_buf = state.cell_or_output_mut();
        while cell_buf.ends_with(' ') || cell_buf.ends_with('\t') {
            cell_buf.pop();
        }
        if !is_implicit {
            let cell_buf = state.cell_or_output_mut();
            let content_start = clamp_to_char_boundary(cell_buf, frame.content_start);
            let content = &cell_buf[content_start..];
            if content.trim().is_empty() {
                state.cell_or_output_mut().truncate(content_start);
            }
        }
        return Ok(());
    }

    trim_trailing_inline_whitespace(state);
    // ~keep All buffer touches below go through `cell_or_output_mut` rather than
    // ~keep `state.output` directly. A heading inside `<summary>`/`<figcaption>`
    // ~keep is not a table cell (the early return above doesn't catch it) but its
    // ~keep `frame.content_start` was captured from the active wrap buffer's
    // ~keep length (see the open-tag push site), not `state.output`'s — using
    // ~keep `state.output` here would insert the `#` prefix at that same small
    // ~keep offset into the real, much longer document output instead, splicing
    // ~keep it into the middle of unrelated already-emitted text. See
    // ~keep `open_heading`'s matching note.
    let buf = state.cell_or_output_mut();
    let content_start = clamp_to_char_boundary(buf, frame.content_start);

    if !is_implicit {
        let content = &buf[content_start..];
        if content.trim().is_empty() {
            // ~keep Empty heading: Tier-2 emits nothing. Roll back to before
            // ~keep the heading's block separator was added.
            buf.truncate(content_start);
            let trimmed_len = buf.trim_end_matches('\n').len();
            if trimmed_len > 0 {
                buf.truncate(trimmed_len);
                buf.push('\n');
            } else {
                buf.clear();
            }
            return Ok(());
        }
    }

    // ~keep Normalize whitespace in the heading body: Tier-2's heading.rs walks
    // ~keep children with `convert_as_inline: true` which routes text through
    // ~keep text-node normalization, folding `\n + indent` runs to a single space.
    // ~keep Mirror that here so `<h3>Mozilla\n   sponsorship</h3>` emits
    // ~keep `### Mozilla sponsorship` rather than `### Mozilla\n  sponsorship`.
    if buf[content_start..].contains('\n') {
        let content = buf[content_start..].to_owned();
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
        buf.truncate(content_start);
        buf.push_str(normalized.trim_end());
    }

    // ~keep Tier-2's `text.trim()` (heading.rs) trims the ENTIRE rendered heading
    // ~keep body — not just internal whitespace runs — before prefixing. A leading
    // ~keep whitespace character (a decoded `&nbsp;`, or one migrated out of an
    // ~keep `<em>`/`<strong>` open marker by `close_inline_marker`'s leading-migration
    // ~keep step) would otherwise double up against the "# " prefix's own trailing
    // ~keep space (`<h3>&nbsp;x</h3>` -> "###  x" instead of Tier-2's "### x").
    let leading_ws_len = buf[content_start..]
        .char_indices()
        .find(|&(_, c)| !c.is_whitespace())
        .map_or(buf.len() - content_start, |(i, _)| i);
    if leading_ws_len > 0 {
        buf.replace_range(content_start..content_start + leading_ws_len, "");
    }

    let prefix = heading_prefix(n);
    buf.insert_str(content_start, prefix);
    // ~keep Tier-2 leaves a blank line ("\n\n") after a heading. A
    // ~keep following paragraph's "\n\n" guard then finds it already and appends
    // ~keep nothing, yielding the expected single blank line.
    ensure_blank_line_buf(state.cell_or_output_mut());
    Ok(())
}

fn close_blockquote(state: &mut Tier1State, frame: &OpenTag) {
    // ~keep Phase GG follow-up: inside a table cell `frame.content_start` indexes
    // ~keep into the cell buffer, not `state.output`.  Don't prefix `> ` — Tier-2
    // ~keep also collapses blockquote inside cells to plain inline text.
    if state.in_table_cell() {
        return;
    }
    let content_start = clamp_to_char_boundary(&state.output, frame.content_start);
    let mut content = state.output[content_start..].to_owned();
    // ~keep Trailing horizontal whitespace (e.g. a decoded `&nbsp;` folded to a
    // ~keep plain space) sitting right before `</blockquote>` is trimmed here,
    // ~keep mirroring `close_inline_marker`'s and `<br>`'s own trailing-whitespace
    // ~keep trim — Tier-2 does not carry it into the quoted line.
    crate::converter::main_helpers::trim_trailing_whitespace(&mut content);
    // ~keep Tier-2's `content.trim()` (blockquote.rs) trims the ENTIRE accumulated
    // ~keep child content — not just internal whitespace runs — before the "> "
    // ~keep per-line prefixing. A leading whitespace character (a decoded
    // ~keep `&nbsp;`, or one migrated out of an `<em>`/`<strong>` open marker by
    // ~keep `close_inline_marker`'s leading-migration step) would otherwise double
    // ~keep up against the prefix's own trailing space (`<blockquote>&nbsp;x</blockquote>`
    // ~keep -> ">  x" instead of Tier-2's "> x").
    let leading_ws_len = content
        .char_indices()
        .find(|&(_, c)| !c.is_whitespace())
        .map_or(content.len(), |(i, _)| i);
    if leading_ws_len > 0 {
        content.replace_range(0..leading_ws_len, "");
    }
    let prefixed = prefix_blockquote_lines(&content);
    state.output.truncate(content_start);
    if frame.prev_escape_ctx.contains(EscapeCtx::BLOCKQUOTE) {
        // ~keep Nested blockquote: `open_blockquote` already normalized the tail to
        // ~keep a clean blank line above; collapse it back to one newline here,
        // ~keep mirroring Tier-2's nested-blockquote separator handling.
        if state.output.ends_with("\n\n") {
            state.output.pop();
        }
    } else if !state.output.is_empty() {
        // ~keep Mirror Tier-2's top-level branch exactly (blockquote.rs:135-143): an
        // ~keep existing blank-line separator collapses to a single newline, but a
        // ~keep lone trailing newline (or none at all) is PROMOTED up to a blank
        // ~keep line. This is a literal function of the untouched pre-open tail —
        // ~keep see `open_blockquote` for why nothing runs there in this branch.
        if state.output.ends_with("\n\n") {
            state.output.pop();
        } else if !state.output.ends_with('\n') {
            state.output.push_str("\n\n");
        } else {
            state.output.push('\n');
        }
    }
    push_list_item_continuation_lines(state, &prefixed);

    // ~keep Tier-2's `handle_blockquote` (blockquote.rs:225-232) unconditionally trims
    // ~keep every trailing newline and re-pushes exactly "\n\n" after EVERY close --
    // ~keep top-level or nested, and regardless of what follows (a sibling, or nothing
    // ~keep at all; a trailing "\n\n" at document end is trimmed back down by the
    // ~keep shared end-of-document normalization both tiers already share). Existing
    // ~keep tests only covered "nothing follows the blockquote", where a missing
    // ~keep trailing separator here is invisible. `<blockquote>a</blockquote>after`
    // ~keep (top-level) and `<blockquote><blockquote>a</blockquote>b</blockquote>`
    // ~keep (nested) both need it: Tier-2 emits a blank line before `after` / a bare
    // ~keep `>` line before `b`. Gated on `!in_list_item` to mirror Tier-2's own
    // ~keep `!ctx.in_table_cell && !ctx.in_list_item` guard -- `in_table_cell` already
    // ~keep returned early above. In practice every list-item blockquote already bails
    // ~keep via `BailReason::ListItemUnsupportedBlockChild` before reaching here; the
    // ~keep guard is kept so this stays correct if that bail is ever narrowed.
    let in_list_item = state
        .stack
        .iter()
        .any(|open_frame| matches!(open_frame.spec.kind, TagKind::ListItem));
    if !in_list_item {
        state.ensure_blank_line();
    }
}

fn close_pre(state: &mut Tier1State, frame: &OpenTag, options: &ConversionOptions) {
    use crate::options::CodeBlockStyle;
    // ~keep Phase GG follow-up: when `<pre>` opened inside a table cell, its content
    // ~keep was accumulated into `current_cell` (the cell buffer), not `state.output`.
    // ~keep The frame's `content_start` indexes into the cell buffer.  Don't emit a
    // ~keep code fence — Tier-2 also collapses pre inside cells to plain inline text
    // ~keep (the cell's `replace('\n', ' ')` step does the rest).
    if state.in_table_cell() {
        return;
    }
    let content_start = clamp_to_char_boundary(&state.output, frame.content_start);
    let raw = state.output[content_start..].to_owned();
    state.output.truncate(content_start);
    // ~keep Render into a scratch buffer first, then (when inside a list item)
    // ~keep indent every physical line to the item's continuation column
    // ~keep before appending to `state.output` — see `push_list_item_continuation_lines`.
    let mut rendered = String::new();
    match options.code_block_style {
        CodeBlockStyle::Indented => {
            rendered.push_str(&indent_pre_lines(&raw));
        }
        CodeBlockStyle::Backticks => {
            // ~keep the fence must be strictly longer than the longest run of `` ` ``
            // ~keep inside the content, otherwise the fence terminates early and
            // ~keep corrupts the rest of the document (CommonMark 4.5). Mirrors
            // ~keep `handlers::code_block::format_code_block`'s Backticks branch.
            let fence_length = (longest_consecutive_backtick_run(&raw) + 1).max(MIN_FENCE_LENGTH);
            let fence: String = std::iter::repeat_n('`', fence_length).collect();

            rendered.push_str(&fence);
            if let Some(lang) = state.pre_lang.take() {
                rendered.push_str(&lang);
            } else if !options.code_language.is_empty() {
                rendered.push_str(&options.code_language);
            }
            rendered.push('\n');
            // ~keep Strip a single leading newline (Tier-2 emits `\ncontent...`) but
            // ~keep ALL trailing newlines, not just one: Tier-2's `format_code_block`
            // ~keep (handlers/code_block.rs) closes the fenced branch with
            // ~keep `content.trim_end_matches('\n')`, unconditionally, regardless of
            // ~keep how many trailing newlines `handle_pre` reconstructed upstream.
            // ~keep A nested block element that itself forces a full blank-line
            // ~keep separator on close (e.g. `<blockquote>`, since its fix) leaves 2
            // ~keep trailing newlines here when it is the last child of `<pre>` --
            // ~keep stripping only one left a spurious blank line before the closing
            // ~keep fence that Tier-2 never emits.
            let raw = raw.strip_prefix('\n').unwrap_or(&raw);
            let raw = raw.trim_end_matches('\n');
            rendered.push_str(raw);
            rendered.push('\n');
            rendered.push_str(&fence);
            // ~keep Tier-2's `format_code_block` (handlers/code_block.rs) ends the
            // ~keep Backticks/Tildes branch with `output.push_str("\n\n")` — a clean
            // ~keep blank-line terminator, not a single newline.  Matching that
            // ~keep precisely matters beyond the general case (where a following
            // ~keep sibling's own leading-separator logic papers over a single-vs-
            // ~keep double difference either way): `close_blockquote`'s top-level
            // ~keep branch inspects this exact trailing state to decide whether to
            // ~keep collapse or promote the separator, so leaving only one newline
            // ~keep here made a `<pre>` directly followed by a `<blockquote>`
            // ~keep diverge from Tier-2 (discovered via that fix; see tests).
            rendered.push_str("\n\n");
        }
        CodeBlockStyle::Tildes => {
            rendered.push_str(&indent_pre_lines(&raw));
        }
    }
    push_list_item_continuation_lines(state, &rendered);
    state.pre_lang = None;
}

/// Indent a text node that starts a fresh, still-unindented physical line
/// inside a list item (e.g. sibling text right after a heading, which only
/// emits a single trailing newline rather than a blank line) to the item's
/// continuation column — the same indent every block handler
/// (`push_list_item_continuation_lines`) adds before its own first line.
/// Without it the line lands flush left and the item (and the rest of the
/// list) falls out of the list on reparse (CommonMark spec example 300).
///
/// Excluded from verbatim contexts (checked by the caller: `in_pre`/`in_code`
/// text never reaches this point) and from contexts that accumulate into a
/// detached scratch buffer rather than the real document (`in_table_cell`,
/// `in_summary`, `in_table_caption`), where `cell_or_output_mut()` is not the
/// list item's own accumulating text and indenting it would corrupt literal
/// or already-wrapped content instead. Mirrors Tier-2's `text_node.rs`
/// (`ctx.in_list_item && output.ends_with('\n') && !output.ends_with("\n\n")`).
fn indent_fresh_list_item_text_line(state: &mut Tier1State) {
    if state.in_table_cell() || state.in_summary() || state.in_table_caption() {
        return;
    }
    let in_list_item = state
        .stack
        .iter()
        .any(|frame| matches!(frame.spec.kind, TagKind::ListItem));
    if !in_list_item {
        return;
    }
    let indent_width = state.list_continuation_indent_width();
    if indent_width == 0 {
        return;
    }
    let dest = state.cell_or_output_mut();
    if dest.ends_with('\n') && !dest.ends_with("\n\n") {
        let indent: String = std::iter::repeat_n(' ', indent_width).collect();
        dest.push_str(&indent);
    }
}

/// Append `rendered` (a fully-formatted block's text, possibly spanning
/// several physical lines) to `state.output`, indenting every line to the
/// innermost open list item's continuation column when inside one.
///
/// CommonMark matches list containment per physical line (spec examples
/// 263, 273, 274, 318, 324): a non-blank line that isn't indented to the
/// item's continuation width falls out of the item — and the list — on
/// reparse. Mirrors Tier-2's `format_code_block_in_list_item`
/// (handlers/code_block.rs): the very first line skips the indent when it
/// is NOT a continuation (i.e. it sits directly after the item's own
/// marker, like `- ` + the block's first line, and already starts at the
/// right column); every other non-blank line always gets indented. Blank
/// lines are left bare — an indented blank line would just be trailing
/// whitespace.
fn push_list_item_continuation_lines(state: &mut Tier1State, rendered: &str) {
    let indent_width = state.list_continuation_indent_width();
    if indent_width == 0 {
        state.output.push_str(rendered);
        return;
    }
    // ~keep A plain suffix check like `ends_with("* ")` also matches the closing
    // ~keep "**"/"*" of `<strong>`/`<em>` immediately followed by a migrated trailing
    // ~keep space, indistinguishable from a real bare bullet by suffix alone. Reuse
    // ~keep `line_is_bare_list_marker` (this file, mirrors Tier-2's
    // ~keep `list::utils::line_is_bare_list_marker`) instead of repeating that ambiguity.
    let is_continuation = !state.output.is_empty() && !line_is_bare_list_marker(&state.output);
    let indent: String = std::iter::repeat_n(' ', indent_width).collect();
    for (index, segment) in rendered.split_inclusive('\n').enumerate() {
        let line = segment.strip_suffix('\n').unwrap_or(segment);
        if line.is_empty() || (index == 0 && !is_continuation) {
            state.output.push_str(segment);
        } else {
            state.output.push_str(&indent);
            state.output.push_str(segment);
        }
    }
}

fn close_code(state: &mut Tier1State, frame: &OpenTag) {
    if state.escape_ctx.contains(EscapeCtx::PRE) || state.escape_ctx.contains(EscapeCtx::CODE) {
        return;
    }
    // ~keep Phase CC: smart backtick escaping (mirrors inline/code.rs:260).
    // ~keep Open emitted nothing; content from `frame.content_start` to buf
    // ~keep end is the raw code content.  Choose num_backticks + delimiter
    // ~keep spaces from that slice, then truncate and re-emit wrapped.
    let buf = state.cell_or_output_mut();
    let content_start = clamp_to_char_boundary(buf, frame.content_start);
    if content_start >= buf.len() {
        // ~keep No content emitted between open and close — Tier-2 emits
        // ~keep nothing for empty <code></code>.
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
            min_safe_code_span_delimiter_length(content)
        } else {
            1
        };
        (needs_delimiter_spaces, num_backticks)
    };

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

/// Compute the length of the longest consecutive run of `` ` `` in `content`.
///
/// ~keep Mirrors `converter::handlers::code_block::longest_consecutive_run`
/// ~keep (see the shared-helper note on `MIN_FENCE_LENGTH` above).
fn longest_consecutive_backtick_run(content: &str) -> usize {
    content
        .chars()
        .fold((0usize, 0usize), |(max, current), c| {
            if c == '`' {
                let next = current + 1;
                (max.max(next), next)
            } else {
                (max, 0)
            }
        })
        .0
}

/// Smallest backtick-run length (starting at 1) that does not occur as a run inside `content`.
///
/// ~keep Mirrors `converter::handlers::code_block::min_safe_code_span_delimiter_length`
/// ~keep byte-for-byte (see the shared-helper note on `MIN_FENCE_LENGTH` above). CommonMark
/// ~keep closes an inline code span at the next backtick string of the *same* length as the
/// ~keep opener (6.1), so `longest_run + 1` unconditionally over-escapes: content `` `` `` (a
/// ~keep single length-2 run, no length-1 run) is valid with a single backtick delimiter.
fn min_safe_code_span_delimiter_length(content: &str) -> usize {
    let mut run_lengths = std::collections::HashSet::new();
    let mut current = 0usize;
    for c in content.chars() {
        if c == '`' {
            current += 1;
        } else {
            if current > 0 {
                run_lengths.insert(current);
            }
            current = 0;
        }
    }
    if current > 0 {
        run_lengths.insert(current);
    }

    let mut candidate = 1usize;
    while run_lengths.contains(&candidate) {
        candidate += 1;
    }
    candidate
}

/// Returns `true` if every byte of `needle` appears in `haystack`, in order, not
/// necessarily contiguously (a byte-level subsequence test). O(haystack.len()), no
/// allocation.
///
/// ~keep Used by `close_link`'s nested-markup autolink guard: greedy left-to-right
/// ~keep matching is correct for subsequence testing regardless of UTF-8 char
/// ~keep boundaries — if a char sequence is a subsequence of another, each char's
/// ~keep byte run appears intact and in order, so the greedy byte scan below always
/// ~keep finds it. A byte-level "match" that happens to straddle char boundaries can
/// ~keep only make this return `true` MORE often than a char-aware version would,
/// ~keep never less — which only makes the caller's bail more conservative, never
/// ~keep less safe.
fn is_byte_subsequence(needle: &str, haystack: &str) -> bool {
    let mut needle_bytes = needle.bytes();
    let Some(mut want) = needle_bytes.next() else {
        return true;
    };
    for byte in haystack.bytes() {
        if byte == want {
            match needle_bytes.next() {
                Some(next_want) => want = next_want,
                None => return true,
            }
        }
    }
    false
}

/// Rewrites the link the scanner has already opened in `dest` into GFM autolink form
/// (`<href>`) when Tier-2 would do the same, returning `Ok(true)` when it did.
///
/// Mirrors Tier-2's predicate at `handlers/link.rs:91-101` exactly: `options.autolinks &&
/// !options.default_title && href non-empty && has_uri_scheme(href) && (label == href ||
/// (mailto: && label == href[7..]))`. `dest[trim_start..]` is Tier-2's `raw_text` for the
/// common flat-text case -- both sides have already collapsed whitespace/NBSP the same way,
/// and neither has run `escape_link_label` or bracket-wrapping yet.
fn try_emit_autolink(
    dest: &mut String,
    trim_start: usize,
    frame: &OpenTag,
    href_str: &str,
    has_nested_tag: bool,
    options: &ConversionOptions,
) -> Result<bool, BailReason> {
    if !options.autolinks || options.default_title || href_str.is_empty() || !has_uri_scheme(href_str) {
        return Ok(false);
    }
    let mailto_suffix = href_str.strip_prefix("mailto:");
    let label = &dest[trim_start..];
    if has_nested_tag {
        // ~keep A nested tag means the buffer is no longer flat text, so a literal
        // ~keep `label == href` compare is untrustworthy: `<b>u</b>` renders as `**u**`,
        // ~keep which never equals bare `u`, yet Tier-2 -- which compares against the
        // ~keep TAG-STRIPPED text -- would autolink it. Bailing on every nested tag is not
        // ~keep the fix either: measured against `tools/benchmark-harness/fixtures`,
        // ~keep 85-100% of scheme-href `<a>` elements on every fixture that has any are
        // ~keep wrapped in at least a `<span>`, so a blanket bail would drop nearly every
        // ~keep real page off the Tier-1 fast path.
        // ~keep
        // ~keep Instead, a cheap PROVABLY CONSERVATIVE precheck. Every transformation
        // ~keep Tier-1 applies while rendering a label (emphasis/strong/strikethrough and
        // ~keep inserted markers, code-span backtick fences, label-bracket escaping,
        // ~keep `![alt](src)` for a nested `<img>`) only INSERTS characters; none delete or
        // ~keep substitute the underlying text. Tier-2's `raw_text` is that same underlying
        // ~keep text with tags stripped and whitespace collapsed, both of which only REMOVE
        // ~keep characters. So `raw_text` is always a subsequence of this buffer, and if
        // ~keep `raw_text == target` then `target` is a subsequence of the buffer too.
        // ~keep Contrapositive: a `target` that is NOT a subsequence proves Tier-2 cannot
        // ~keep autolink either, so falling through to the normal link form is safe. Bail
        // ~keep only when the precheck cannot rule it out.
        let could_be_autolink = is_byte_subsequence(href_str, label)
            || mailto_suffix.is_some_and(|suffix| is_byte_subsequence(suffix, label));
        if could_be_autolink {
            return Err(BailReason::LinkAutolinkNestedMarkup);
        }
        // ~keep Ruled out above, so the exact `label == target` match below cannot fire
        // ~keep either -- equality would imply the subsequence that was just disproved.
    }
    let rendered = if label == href_str {
        href_str
    } else if mailto_suffix == Some(label) {
        // ~keep `mailto_suffix` is `Some` on this arm by construction.
        mailto_suffix.unwrap_or(href_str)
    } else {
        return Ok(false);
    };
    // ~keep An autolink has no bracket wrapping at all, but the scanner already emitted
    // ~keep the opening `[` at link-open time, before it could know the label. Remove it,
    // ~keep mirroring the href-less branch in `close_link` that does the same cleanup.
    let bracket_search_end = clamp_to_char_boundary(dest, frame.content_start);
    let label_start = if let Some(bracket_pos) = dest[..bracket_search_end].rfind('[') {
        dest.remove(bracket_pos);
        trim_start - 1
    } else {
        trim_start
    };
    let rendered = rendered.to_owned();
    dest.truncate(label_start);
    dest.push('<');
    dest.push_str(&rendered);
    dest.push('>');
    Ok(true)
}

fn close_link(state: &mut Tier1State, frame: &OpenTag, options: &ConversionOptions) -> Result<(), BailReason> {
    // ~keep Close the link: `](href "title")` or `](href)`
    // ~keep If no href, just emit the text as-is (Tier-2 behaviour: no link markup).
    // ~keep Link state was pushed to state.link_stack at open; pop it now.
    let (href, title, has_nested_tag) = state.link_stack.pop().unwrap_or((None, None, false));
    let dest = state.cell_or_output_mut();
    // ~keep Trim trailing whitespace inside the link label so `[text  ](url)`
    // ~keep collapses to `[text](url)` — matches Tier-2's normalize_link_label
    // ~keep at utility/content.rs:145 (kimbrain.html and similar source HTML
    // ~keep with whitespace before </a>).
    let trim_start = clamp_to_char_boundary(dest, frame.content_start);
    let trimmed_end = dest[trim_start..].trim_end_matches(|c: char| c.is_whitespace()).len();
    dest.truncate(trim_start + trimmed_end);
    // ~keep Mirror Tier-2's `normalize_whitespace_cow` step inside
    // ~keep `normalize_link_label` (utility/content.rs:144): any Unicode whitespace
    // ~keep in the link label (notably NBSP `\u{00a0}`) collapses to a single ASCII
    // ~keep space.  Tier-1 otherwise emits `[Designed\u{a0}by](url)` where Tier-2
    // ~keep emits `[Designed by](url)`.
    if dest[trim_start..].contains('\u{00a0}') {
        let normalised: String = dest[trim_start..]
            .chars()
            .map(|c| if c == '\u{00a0}' { ' ' } else { c })
            .collect();
        dest.truncate(trim_start);
        dest.push_str(&normalised);
    }
    if let Some(href_str) = href.as_deref() {
        if try_emit_autolink(dest, trim_start, frame, href_str, has_nested_tag, options)? {
            return Ok(());
        }
    }
    // ~keep Wikipedia back-reference normalisation (Tier-2 `handlers/link.rs:205`):
    // ~keep a label of exactly `^` paired with an `#anchor` href is rewritten to
    // ~keep `↑` so it does not look like Markdown's footnote syntax.
    if let Some(href_str) = href.as_deref() {
        if href_str.starts_with('#') && dest.len() == trim_start + 1 && dest.as_bytes()[trim_start] == b'^' {
            dest.truncate(trim_start);
            dest.push('↑');
        }
    }
    if let Some(href) = href {
        // ~keep Security fix mirror (`handlers/link.rs:208`, outside tier1/, via
        // ~keep the shared `escape_link_label` helper — same class of bug just
        // ~keep fixed for the SVG `<title>` label above): an unescaped `]` in
        // ~keep link text lets `<a href="/x">a] (https://evil.example) b</a>`
        // ~keep close the label early and open a second, attacker-controlled
        // ~keep Markdown link — inert HTML source turned into a live injection
        // ~keep by the unescaped label. Only applies when a `[...]` label is
        // ~keep actually being emitted (the `href`-less branch below emits the
        // ~keep text with no bracket wrapping at all, matching Tier-2's separate
        // ~keep no-`escape_link_label` code path for that case). Applied after
        // ~keep the caret rewrite above (matching Tier-2's order); `↑` contains
        // ~keep no bracket so the rewrite is a no-op for `escape_link_label`
        // ~keep either way.
        // ~keep `escaped_label` borrows from `dest` when unescaped (the common case), so it
        // ~keep cannot outlive the `truncate`/`push_str` below that mutably borrows `dest` --
        // ~keep only rewrite `dest` when escaping actually produced an owned copy.
        match crate::converter::utility::content::escape_link_label(&dest[trim_start..]) {
            std::borrow::Cow::Borrowed(_) => {}
            std::borrow::Cow::Owned(escaped_label) => {
                dest.truncate(trim_start);
                dest.push_str(&escaped_label);
            }
        }
        if let Some(title) = title {
            // ~keep Tier-2 in production HTML fixtures HTML-encodes a literal `"`
            // ~keep in the title attribute to `&quot;` (rather than the
            // ~keep `replace('"', "\\\"")` in `escape_markdown_title` in `inline/link.rs`).  The
            // ~keep backslash-escape branch of link.rs appears unreachable in
            // ~keep practice for the title attribute path on these fixtures.
            // ~keep Mirror the observed fixture behaviour to match expected output.
            let escaped_title;
            let title_out: &str = if title.contains('"') {
                escaped_title = title.replace('"', "&quot;");
                &escaped_title
            } else {
                &title
            };
            #[allow(clippy::format_push_string)]
            dest.push_str(&format!("]({href} \"{title_out}\")"));
        } else {
            #[allow(clippy::format_push_string)]
            dest.push_str(&format!("]({href})"));
        }
    } else {
        let bracket_search_end = clamp_to_char_boundary(dest, frame.content_start);
        if let Some(bracket_pos) = dest[..bracket_search_end].rfind('[') {
            dest.remove(bracket_pos);
        }
    }
    Ok(())
}

fn close_list(state: &mut Tier1State, kind: ListKind) {
    state.list_depth = state.list_depth.saturating_sub(1);
    if matches!(kind, ListKind::Unordered) {
        state.ul_depth = state.ul_depth.saturating_sub(1);
    }
    // ~keep When inside a table cell, Tier-2 does NOT add a trailing newline after
    // ~keep the list — the cell accumulator handles any separators via the
    // ~keep `\n → space` replacement at cell-close time.
    if state.in_table_cell() {
        return;
    }
    let dest = state.cell_or_output_mut();
    if !dest.ends_with('\n') {
        dest.push('\n');
    }
}

fn close_list_item(state: &mut Tier1State, frame: &OpenTag) {
    // ~keep When inside a table cell, Tier-2 does NOT add a trailing newline after
    // ~keep each list item (see list/item.rs: `if !ctx.in_table_cell { ... \n ... }`).
    // ~keep Items are concatenated directly in the cell accumulator.
    if state.in_table_cell() {
        let cell_buf = state.cell_or_output_mut();
        while cell_buf.ends_with(' ') || cell_buf.ends_with('\t') {
            cell_buf.pop();
        }
        return;
    }
    state.list_item_marker_widths.pop();
    trim_trailing_inline_whitespace(state);
    let dest = state.cell_or_output_mut();
    // ~keep Phase EE: loose-list separator.  When this item had block-level
    // ~keep children (its content range contains a `\n\n` block separator),
    // ~keep mirror Tier-2's `handle_li` ensure_trailing_blank_line behaviour
    // ~keep so the next sibling `<li>` starts after a blank line.  Plain text
    // ~keep items still get the tight `\n` terminator.
    let had_block_children = {
        let start = clamp_to_char_boundary(dest, frame.content_start);
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

// ~keep ── Definition-list helpers ───────────────────────────────────────────────────
// ~keep
// ~keep Tier-2 reference: crates/html-to-markdown/src/converter/list/definition.rs.
// ~keep Tier-2 builds the full <dl> content in a buffer, trims it, then emits with
// ~keep "\n\n" boundaries. <dt> emits trimmed term + "\n"; <dd> emits trimmed
// ~keep description + "\n\n". Tier-1 streams the same shape by:
// ~keep   - open_dl: ensure blank line; record content_start on the frame
// ~keep   - close_dt: trim trailing whitespace, push "\n"
// ~keep   - close_dd: trim trailing whitespace, push "\n\n"
// ~keep   - close_dl: trim leading/trailing whitespace inside the dl range, then
// ~keep               normalise the trailing separator to "\n\n"
// ~keep
// ~keep Bails on dl/dt/dd are removed (see bail_unsupported). Implicit close of an
// ~keep open dt/dd when a sibling dt/dd opens is wired via OptionalCloseRule::
// ~keep CloseSiblingDtDd in spec_rules.rs and runs the same close_dt/close_dd path
// ~keep through emit_close_for_implicit.

fn open_dl(state: &mut Tier1State) {
    if state.in_table_cell() {
        return;
    }
    state.ensure_blank_line();
}

const fn open_dt(_state: &mut Tier1State) {}

const fn open_dd(_state: &mut Tier1State) {}

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
    // ~keep Empty dl: emit nothing (matches Tier-2 which skips when trimmed content
    // ~keep is empty).
    if buf.len() <= frame.content_start {
        return;
    }
    // ~keep Tier-2 trims the dl's accumulated content, so any trailing whitespace
    // ~keep from the last dt/dd close should collapse to a single "\n\n" separator.
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

fn close_table(
    state: &mut Tier1State,
    options: &ConversionOptions,
    table_probes: &mut Vec<TableLayoutProbe>,
) -> Result<(), BailReason> {
    // ~keep Pop the table state and (if safe) emit the GFM table to main output.
    let Some(ts) = state.table_stack.pop() else {
        return Ok(());
    };
    // ~keep Popped together with the `TableState` it belongs to; the truncate re-syncs
    // ~keep the two stacks if a malformed document ever pushed one without the other.
    let probe = table_probes.pop().unwrap_or_default();
    table_probes.truncate(state.table_stack.len());

    // ~keep Safety checks: ensure Tier-2 would also use the GFM path.
    // ~keep
    // ~keep Tier-2 uses the layout (non-GFM) path when ALL of these hold:
    // ~keep   (a) no <th> anywhere in the table, AND
    // ~keep   (b) no <caption>, AND
    // ~keep   (c) looks_like_layout || is_blank || (row_count<=2 && link_count>=3)
    // ~keep
    // ~keep where (block/table/builder.rs)
    // ~keep   looks_like_layout = nested_table_count > 1
    // ~keep                    || distinct_counts.len() > 1
    // ~keep                    || (has_span && has_border_zero)
    // ~keep
    // ~keep All three disjuncts are checked below — none of them is unreachable here.
    // ~keep An earlier revision of this comment claimed nested tables and
    // ~keep colspan/rowspan had "already bailed"; both claims were false (Phase HH
    // ~keep renders a nested table inline into the parent cell, and open_table_cell
    // ~keep expands colspan instead of bailing), and the resulting gap let Tier-1
    // ~keep emit a GFM table for input Tier-2 renders as a bullet list.
    // ~keep
    // ~keep If those conditions could apply to this table, we bail rather than
    // ~keep emit a GFM table that Tier-2 would have rendered differently.
    // ~keep
    // ~keep When a <caption> is present, Tier-2 always takes the GFM path
    // ~keep regardless of <th> presence (has_caption short-circuits the layout check).
    let has_caption = ts.caption_text.is_some();
    // One-cell wrappers are unwrapped by Tier-2 to preserve the nested table's
    // structure. Its DOM walker also handles the traversal-depth boundary.
    if !ts.has_th
        && !has_caption
        && !probe.has_span
        && probe.nested_table_count > 0
        && ts.rows.len() == 1
        && ts.first_row_col_count == Some(1)
    {
        return Err(BailReason::TableNestedTable);
    }
    if !ts.has_th && !has_caption {
        // ~keep No <th> and no <caption>: check if Tier-2 would take the layout path.
        let row_count = ts.rows.len();

        // ~keep Inconsistent column counts → layout table in Tier-2.
        // ~keep Compare colspan-expanded column counts (sum of cell colspans per row)
        // ~keep because Tier-2 computes column counts post-colspan expansion.
        let expanded_cols = |row: &Vec<(String, u16)>| -> usize { row.iter().map(|(_, c)| usize::from(*c)).sum() };
        let inconsistent_cols = {
            let first = ts.first_row_col_count.unwrap_or(0);
            ts.rows.iter().any(|r| expanded_cols(r) != first)
        };

        // ~keep Link-heavy with few rows → layout table in Tier-2.
        let link_heavy = row_count <= 2 && ts.link_count >= 3;

        // ~keep Blank table → Tier-2 emits nothing (not a bail case).
        let is_blank = ts.rows.is_empty() || ts.rows.iter().all(|r| r.iter().all(|(c, _)| c.trim().is_empty()));

        // ~keep Two or more directly-nested tables → layout table in Tier-2.  Tier-2
        // ~keep counts one nesting level only, which is what the probe accumulates.
        let multiple_nested_tables = probe.nested_table_count > 1;

        // ~keep A spanning cell in a `border="0"` table → layout table in Tier-2.  Both
        // ~keep halves are mirrored exactly: span = attribute presence, border = the
        // ~keep literal value "0" (see open_table_cell / open_table).
        let spanning_borderless = probe.has_span && probe.border_zero;

        if inconsistent_cols || link_heavy || is_blank || multiple_nested_tables || spanning_borderless {
            // ~keep Tier-2 would not emit a GFM table here.  Bail so the fallback
            // ~keep produces the correct layout output.  Phase L's full layout
            // ~keep emit deferred — needs more careful per-cell content tracking
            // ~keep to mirror Tier-2's walker exactly.
            return Err(BailReason::Classifier);
        }
    }
    // ~keep Phase HH: a nested table writes its GFM rendering into the parent
    // ~keep cell buffer; the parent's `close_table_cell` then collapses the
    // ~keep resulting newlines to spaces.  An outer table writes to the main
    // ~keep output buffer as before.
    if ts.inline_mode {
        if let Some(outer) = state.table_stack.last_mut() {
            outer.had_nested_table = true;
        }
        // ~keep Mirrors Tier-2's `nested_table_count`: the enclosing table counts this
        // ~keep one, and stops there — tables nested deeper already counted against
        // ~keep their own immediate parent when they closed.
        if let Some(outer_probe) = table_probes.last_mut() {
            outer_probe.nested_table_count += 1;
        }
        // ~keep Tier-2's `render_cell_text` (block/table/cell.rs, commit ee77eb2a18)
        // ~keep now escapes the bare `|` a nested table's own row/separator syntax
        // ~keep leaves behind, rendering the nested table into a scratch buffer and
        // ~keep escaping just that fragment before appending it to the outer cell.
        // ~keep Left unescaped, those pipes read as *outer-row* cell boundaries on
        // ~keep reparse and GFM truncates the row, silently dropping the inner
        // ~keep cells — genuine content loss, not a cosmetic difference. Mirror the
        // ~keep same scratch-buffer-then-escape shape here rather than escaping the
        // ~keep whole outer cell buffer, so any literal text already accumulated
        // ~keep alongside the nested table in the same cell is left untouched.
        let mut nested = String::new();
        emit_gfm_table(&mut nested, ts);
        if nested.contains('|') {
            nested = crate::converter::utility::content::escape_bare_pipes_outside_code_spans(&nested);
        }
        // ~keep Mirrors Tier-2's `fold_nested_table_rows` (block/table/cell.rs, issue #469):
        // ~keep the inner rows are joined with `<br>` under `br_in_tables` and a space
        // ~keep otherwise, so `close_table_cell`'s unconditional newline fold has nothing left
        // ~keep to flatten and both tiers spell the boundary the same way.
        let nested = crate::converter::block::table::cell::fold_nested_table_rows(&nested, options.br_in_tables);
        let dest = state.cell_or_output_mut();
        if !nested.is_empty() && !dest.trim_end().is_empty() {
            crate::converter::main_helpers::emit_table_cell_break(dest, options.br_in_tables);
        }
        dest.push_str(&nested);
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
    let Some(ts) = state.table_stack.last_mut() else {
        return;
    };
    if ts.current_row.is_empty() {
        return;
    }
    // ~keep Track first-row column count for consistency checking — use the
    // ~keep colspan-expanded count so Tier-2's heuristic compares the same numbers.
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
    // ~keep Trim the accumulated cell text (matches Tier-2 `text.trim()`).
    let cell_text_raw = ts.current_cell.trim().to_owned();
    // ~keep Replace newlines with spaces — mirrors Tier-2's `cell_text_content`
    // ~keep which calls `text.replace('\n', " ")` when `br_in_tables` is false.
    // ~keep `<br>` itself is handled at emission time (see `TagKind::LineBreak`
    // ~keep in `emit_void`), so no sentinel expansion is needed here.
    let cell_text = if cell_text_raw.contains('\n') {
        cell_text_raw.replace('\n', " ")
    } else {
        cell_text_raw
    };
    let cell_text = cell_text.trim().to_owned();
    // ~keep Bail if the cell contains a pipe: Tier-2 escapes `|` → `\|`
    // ~keep which changes the cell width computation; Tier-1 does not
    // ~keep implement pipe escaping.  Implicit closes skip this check because
    // ~keep they are triggered during structural teardown, not fresh cell data.
    // ~keep
    // ~keep Phase HH exception: when a nested table emitted GFM markdown into this
    // ~keep cell, the pipes were already escaped (`|` -> `\|`) by the nested-table
    // ~keep close path above -- see `escape_bare_pipes_outside_code_spans` -- so
    // ~keep `cell_text` still literally contains `|` bytes (an escaped pipe is still
    // ~keep two bytes, one of them `|`) even though Tier-2 no longer emits a bare
    // ~keep one here either (commit ee77eb2a18). `had_nested_table` gates the skip;
    // ~keep reset it so subsequent cells in the same row are still pipe-checked.
    let allow_pipes = ts.had_nested_table;
    ts.had_nested_table = false;
    if !is_implicit && !allow_pipes && cell_text.contains('|') {
        return Err(BailReason::TableBlockChildInCell);
    }
    // ~keep Phase L-prep: store (text, colspan) so emit_gfm_table can mirror
    // ~keep Tier-2's `for _ in 0..colspan { output.push_str(" |") }` (cell.rs:248)
    // ~keep and the layout-heuristic uses the colspan-expanded column count.
    let colspan = ts.current_cell_colspan;
    ts.current_row.push((cell_text, colspan));
    ts.current_cell.clear();
    ts.current_cell_colspan = 1;
    Ok(())
}

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
    let mut i = len - 2;
    while i > 0 && bytes[i - 1].is_ascii_digit() {
        i -= 1;
    }
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

fn flush_text(
    state: &mut Tier1State,
    raw: &str,
    base_offset: usize,
    next_tag_is_list: bool,
    next_tag_is_img: bool,
    next_tag_is_span: bool,
) -> Result<(), BailReason> {
    if raw.is_empty() {
        return Ok(());
    }

    // ~keep Read-then-clear: this text flush is the only event that can be the
    // ~keep "next scanner event" after a custom-element close for the purposes
    // ~keep of the boundary check below (see the field's doc comment).
    let after_custom_element_close = state.last_closed_custom_element;
    state.last_closed_custom_element = false;
    // ~keep Same read-then-clear convention, for the "previous direct sibling
    // ~keep was an <img>" half of the adjacent-images check below.
    let after_img = state.last_emitted_was_img;
    state.last_emitted_was_img = false;

    // ~keep Inside a table but outside a cell or caption: whitespace between
    // ~keep structural tags (`<table>...<tr>`, `<tr>...<td>`) is insignificant and
    // ~keep silently dropped here, matching Tier-2 (which processes only tag
    // ~keep children of a table and never walks a stray whitespace text node).
    // ~keep Caption content is the exception — Tier-2 walks caption children and
    // ~keep accumulates their text into the caption output.
    //
    // ~keep Non-whitespace text at this position is HTML5 "foster parented"
    // ~keep content: the real parsing algorithm relocates it to just before the
    // ~keep `<table>` element rather than dropping it. Tier-2's own DOM (built by
    // ~keep `tl`, not a full HTML5 tree-construction implementation) only
    // ~keep sometimes surfaces it there — empirically, it depends on adjacency to
    // ~keep a `<!--comment-->` in a way this single-pass byte scanner cannot
    // ~keep reproduce byte-for-byte without a second pass over the table's
    // ~keep contents. Bail instead of silently discarding real content (the
    // ~keep original defect here): Tier-2's fallback renders whatever the true
    // ~keep answer is. `raw.trim().is_empty()` is the same "is this
    // ~keep insignificant" test already used everywhere else in this function,
    // ~keep so a decoded whitespace-only entity run (e.g. `&nbsp;` alone) is
    // ~keep treated as non-whitespace and bails too — overly conservative, but
    // ~keep never wrong, and rare enough in practice to not matter.
    if !state.table_stack.is_empty() && !state.in_table_cell() && !state.in_table_caption() {
        if raw.trim().is_empty() {
            return Ok(());
        }
        return Err(BailReason::Classifier);
    }

    let in_pre = state.escape_ctx.contains(EscapeCtx::PRE);
    // ~keep Phase EE: inside `<code>` text is verbatim — Tier-2's handle_code
    // ~keep walks children and pushes their text without normalize_whitespace,
    // ~keep so `\n` and runs of spaces inside `<code>` survive into the
    // ~keep wrapped span.  Treat as `in_pre` for the no-collapse path.
    let in_code = state.escape_ctx.contains(EscapeCtx::CODE);

    // ~keep Phase NN: text containing Unicode whitespace (NBSP `\u{00A0}`, hair
    // ~keep space `\u{200A}`, etc., or their entity forms) folds those to ASCII
    // ~keep space — but only when the chunk has non-whitespace content.
    // ~keep Mirrors Tier-2 `text_node.rs:124` and `:154` which run
    // ~keep `normalize_whitespace_cow` on text outside `<code>`/`<pre>` (folding
    // ~keep Unicode space chars).  The whitespace-only branch at `:80-112`
    // ~keep preserves a pure-NBSP text node between inline siblings as-is (e.g.
    // ~keep `<a>X</a>&nbsp;<a>Y</a>` keeps the NBSP).  Without this rule,
    // ~keep `First<NBSP>appeared` reaches the buffer verbatim where Tier-2 outputs
    // ~keep `First appeared`.
    // ~keep Common Unicode-whitespace entity forms: named + numeric (decimal +
    // ~keep hex).  Tier-2's `normalize_whitespace_cow` folds the decoded chars;
    // ~keep Tier-1's flush_text runs BEFORE entity decode, so the patterns must
    // ~keep be listed explicitly.
    const UNICODE_WS_ENTITIES: &[&str] = &[
        "&nbsp;", "&#160;", "&#xa0;", "&#xA0;", "&ensp;", "&#8194;", "&#x2002;", "&emsp;", "&#8195;", "&#x2003;",
        "&thinsp;", "&#8201;", "&#x2009;", "&hairsp;", "&#8202;", "&#x200a;", "&#x200A;",
    ];
    let raw_owned_nbsp;
    let raw: &str = if !in_pre && !in_code {
        let has_ws_entity = UNICODE_WS_ENTITIES.iter().any(|p| raw.contains(p));
        let has_unicode_ws_literal = raw.bytes().any(|b| b >= 0x80)
            && raw
                .chars()
                .any(|c| c.is_whitespace() && c != ' ' && c != '\t' && c != '\n' && c != '\r');
        if has_ws_entity || has_unicode_ws_literal {
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

    // ~keep Inter-block whitespace strip: in a block-edge context (output empty,
    // ~keep ends with a newline, or ends with a list-item marker like "- " /
    // ~keep "1. "), whitespace-only text between adjacent elements (the
    // ~keep indentation in pretty-printed HTML) is not meaningful and must be
    // ~keep discarded.  Tier-2's DOM walker gets this for free because the
    // ~keep parser yields text nodes separately from tag nodes and the walker
    // ~keep skips whitespace-only text at block-level boundaries.  Skipped when
    // ~keep inside `<pre>` (verbatim) or inside a table cell (caller is
    // ~keep accumulating cell text).
    // ~keep
    // ~keep We also treat "the current open frame is a link/emphasis frame whose
    // ~keep body is still empty" as a block-edge: text appearing immediately
    // ~keep after `<a>` → `[`, `<strong>` → `**`, etc. inherits leading
    // ~keep whitespace from the source HTML's indentation and Tier-2 trims it
    // ~keep when building the inline label.  This catches cases like
    // ~keep `<a href>\n   <span>EN</span>\n</a>` where the whitespace after
    // ~keep `<a>` would otherwise leak into the link label as `[ EN]`.
    // ~keep
    // ~keep Plain `<p>`/`<div>`/`<h1>` frames are NOT in this set — Tier-2 keeps
    // ~keep the leading whitespace inside the very first paragraph of a document
    // ~keep (it becomes the single space after `normalize_whitespace`).  Only
    // ~keep post-content paragraphs see "\n\n" before them, which the
    // ~keep `output.ends_with('\n')` check above already handles.
    // ~keep Phase R-3: inside `<summary>`, any tag's body-start is also an inline
    // ~keep frame edge.  Tier-2's handle_summary collects all children with
    // ~keep text-normalization in effect; leading whitespace inside `<span>`,
    // ~keep `<div>`, `<p>` (etc.) bodies gets stripped just like inside `<a>`.
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
    // ~keep Determine whether the current active output position is at a "block
    // ~keep edge" (empty or after a newline / list marker).  When inside a summary
    // ~keep accumulation buffer, consult that buffer rather than state.output so
    // ~keep that inter-element spaces inside the summary are preserved correctly.
    // ~keep Snap the relevant properties to local booleans before releasing the
    // ~keep borrow to avoid conflicts with subsequent state reads.
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
    // ~keep Tier-2's `was_fresh_block_start` check (text_node.rs:99) runs BEFORE
    // ~keep any other whitespace-only-node disposition and takes absolute
    // ~keep priority: at document start, a whitespace-only text node is DROPPED
    // ~keep outright, even when the surrounding context (e.g. between two
    // ~keep `<img>` tags' emitted Markdown and a following inline element) would
    // ~keep otherwise call for a single separating space. This matters because
    // ~keep `<img>` — like any non-text content — never flips
    // ~keep `Tier1State::at_document_start`, so it stays true across leading
    // ~keep images the way Tier-2's flag stays true across a leading `<img>`
    // ~keep (image handling never touches `at_fresh_block_start` either).
    // ~keep Excluded from table cells and list items, matching Tier-2's
    // ~keep `!ctx.in_table_cell && !ctx.in_list_item` guard — those have their
    // ~keep own dedicated whitespace handling below. Also excluded inside a
    // ~keep heading: Tier-2's heading handler is a separate, allow-listed
    // ~keep divergence (flattens a heading's `<img>` to bare alt text instead
    // ~keep of Markdown image syntax) that assembles its line outside the
    // ~keep generic text-node/`at_fresh_block_start` pipeline this mirrors, so
    // ~keep it always keeps a real separating space after that alt text —
    // ~keep applying this rule there would additionally drop that space and
    // ~keep compound one known divergence with a new one.
    let in_list_item_frame_for_ws = state
        .stack
        .iter()
        .any(|frame| matches!(frame.spec.kind, TagKind::ListItem));
    let document_start_drops_ws = state.at_document_start
        && !state.in_table_cell()
        && !in_list_item_frame_for_ws
        && !state.escape_ctx.contains(EscapeCtx::HEADING);
    if !in_pre && (is_block_edge || document_start_drops_ws) && raw_is_whitespace {
        // ~keep issue #481: record that a whitespace-only text node was dropped right at
        // ~keep this Strong/Emphasis frame's own content start, BEFORE it disappears —
        // ~keep `close_inline_marker` cannot tell a genuinely empty `<em></em>` apart from
        // ~keep a whitespace-only `<em> </em>` from buffer bytes alone once this drop has
        // ~keep already erased them (see `OpenTag::dropped_whitespace_only_text`'s doc
        // ~keep comment). Harmless when more real content follows this same drop (e.g.
        // ~keep `<em> <b>x</b></em>`): the frame's body ends up non-empty regardless, so
        // ~keep `close_inline_marker` never consults the flag in that case.
        if at_inline_frame_start {
            if let Some(frame) = state.stack.last_mut() {
                if matches!(frame.spec.kind, TagKind::Strong | TagKind::Emphasis) {
                    frame.dropped_whitespace_only_text = true;
                }
            }
        }
        // ~keep Drop block-edge whitespace anywhere — including inside table cells.
        // ~keep A cell-open `<td>`/`<th>` produces a fresh empty buffer; the
        // ~keep pretty-printer's inter-tag whitespace before the first child would
        // ~keep otherwise leak as a leading space into the cell, breaking the
        // ~keep 3-space gap heuristic (`  \n` from `<div>` open becomes 4 spaces
        // ~keep instead of 3 after `replace('\n', ' ')`).
        return Ok(());
    }
    // ~keep Tier-2 text_node.rs:100-113 collapses whitespace-only text nodes
    // ~keep between adjacent inline siblings to a single space — including
    // ~keep inside table cells where the surrounding `<a>`/`<span>` siblings are
    // ~keep inline.  Mirror that here so `<a>x</a>\n  <a>y</a>` inside a `<td>`
    // ~keep emits `[x] [y]` (single space) instead of `[x]\n [y]` which the
    // ~keep cell-close `replace('\n', ' ')` would turn into two spaces.  Skip
    // ~keep when at a block edge (cell just opened) so the cell doesn't start
    // ~keep with a stray space.
    if !in_pre && state.in_table_cell() && raw_is_whitespace && !is_block_edge {
        // ~keep Tier-2's text_node.rs:80-98 drops whitespace text between non-inline
        // ~keep siblings: when the parent is a list (`<ul>`/`<ol>`/`<dl>`), the
        // ~keep inter-`<li>` whitespace returns without pushing because the next
        // ~keep sibling `<li>` is a block, not inline.  Mirror that here so adjacent
        // ~keep `<li>` siblings in a cell concatenate without separation
        // ~keep (`[v](u1)[t](u2)` not `[v](u1) [t](u2)`).  For inline parents
        // ~keep (`<span>`/`<a>`/`<td>` direct inline-sibling case), keep the
        // ~keep single-space fold.
        if matches!(state.stack.last().map(|f| f.spec.kind), Some(TagKind::List(_))) {
            return Ok(());
        }
        // ~keep `after_custom_element_close` overrides the usual "already ends
        // ~keep with a space, skip" dedup — see `Tier1State::last_closed_custom_element`.
        let dest = state.cell_or_output_mut();
        if !dest.is_empty() && !dest.ends_with('\n') && (after_custom_element_close || !dest.ends_with(' ')) {
            dest.push(' ');
        }
        return Ok(());
    }
    // ~keep Whitespace-only text outside any inline element (link / strong / em /
    // ~keep code) and outside `<pre>` / table cells is structural indentation
    // ~keep between block siblings (e.g. between `</div>` and the next `<div>`).
    // ~keep Tier-2 emits a single ASCII space here when the surrounding context
    // ~keep is inline, but otherwise the DOM walker treats it as a no-op.  For
    // ~keep Tier-1's heuristic we collapse it to nothing — matches Tier-2 for
    // ~keep the common block-between-blocks case and the inline cases are caught
    // ~keep by the inline-frame check above.
    // ~keep
    // ~keep Exception (Phase U + U-2): when the output tail is inline content
    // ~keep (text or `**`/`*`/`` ` ``/`)` close markers) AND we're NOT at a
    // ~keep block edge, a whitespace-only text node between siblings must
    // ~keep become a single space.  Without this `</strong> <em>` would emit
    // ~keep `**a***b*` and `<span>Open Search Bar</span>\n<button>` would lose
    // ~keep the space before the button's content.
    // ~keep
    // ~keep Phase U-2 dropped the original "horizontal whitespace only" guard:
    // ~keep a newline-bearing whitespace run between two inline siblings still
    // ~keep collapses to a single space in Tier-2.  The "what if next tag is a
    // ~keep block?" regression is now handled later in `ensure_blank_line` and
    // ~keep `close_block_container`, which trim trailing horizontal whitespace
    // ~keep before emitting `\n\n`.
    if !in_pre && !state.in_table_cell() && raw_is_whitespace {
        // ~keep When inside a <summary> accumulation buffer, treat the context as
        // ~keep inline (like strong/emphasis): inter-element spaces must be
        // ~keep preserved so `<span>a</span> <span>b</span>` collects "a b" not "ab".
        let inside_inline = state.in_summary()
            || state.stack.iter().any(|frame| {
                matches!(
                    frame.spec.kind,
                    TagKind::Link | TagKind::Strong | TagKind::Emphasis | TagKind::Code
                )
            });
        if !inside_inline {
            // ~keep Tier-2's `paragraph.rs` skips a whitespace-only text node
            // ~keep OUTRIGHT (not deduped to a space, DROPPED) when it sits directly
            // ~keep between two "empty inline" siblings that are both direct
            // ~keep children of the same `<p>` — scoped here to the `<img>`/`<img>`
            // ~keep case the allow-listed divergence was keyed on (`is_empty_inline_element`
            // ~keep also covers `br`/`hr`/`input`, but those combinations are
            // ~keep unverified and out of scope). `state.stack.last()` being
            // ~keep `Paragraph` mirrors walking the `<p>`'s OWN direct children —
            // ~keep an intervening wrapper tag would put something else on top.
            let in_direct_paragraph = matches!(
                state.stack.last().map(|frame| frame.spec.kind),
                Some(TagKind::Paragraph)
            );
            if in_direct_paragraph && after_img && next_tag_is_img {
                return Ok(());
            }
            // ~keep Use the active buffer (summary buf or main output) for the
            // ~keep tail check so spaces between adjacent inline elements inside
            // ~keep a summary are preserved correctly.
            let active_tail: &str = state.cell_or_output_mut();
            // ~keep `after_custom_element_close` overrides the tail-already-has-
            // ~keep content requirement below — see
            // ~keep `Tier1State::last_closed_custom_element`.
            if output_ends_with_inline_close_marker(active_tail)
                || output_ends_with_inline_text(active_tail)
                || (after_custom_element_close && !active_tail.is_empty() && !active_tail.ends_with('\n'))
            {
                let dest = state.cell_or_output_mut();
                if next_tag_is_list && raw.contains('\n') {
                    // ~keep Tier-2's text_node.rs branches BEFORE the previous/next-
                    // ~keep sibling check above when the whitespace run itself
                    // ~keep contains a newline (`had_newlines`): with a non-inline
                    // ~keep next sibling (`<ul>`/`<ol>`), that branch drops the run
                    // ~keep entirely rather than collapsing or preserving it. Push
                    // ~keep nothing here to match.
                } else if next_tag_is_list {
                    // ~keep Tier-2's text_node.rs pushes a whitespace-only text node
                    // ~keep VERBATIM (not collapsed) whenever the next sibling isn't
                    // ~keep inline — see `previous_sibling_is_inline_tag(...) &&
                    // ~keep next_sibling_is_inline_tag(...)`, false here since `<ul>`/
                    // ~keep `<ol>` are block. That raw tail then feeds
                    // ~keep `add_list_leading_separator`'s literal `ends_with("* "|
                    // ~keep "- "|". ")` check (list/utils.rs), which only
                    // ~keep false-negatives (collides with a bullet marker, skipping
                    // ~keep the separator) when the run is EXACTLY one space —
                    // ~keep collapsing here to a single space unconditionally would
                    // ~keep manufacture that collision for runs that never had it.
                    dest.push_str(raw);
                } else {
                    dest.push(' ');
                }
            }
            return Ok(());
        }
        // ~keep Inside an inline frame (`<a>`/`<strong>`/`<em>`/`<code>`) or summary
        // ~keep accumulation: a whitespace-only text node (often the indent run
        // ~keep between two inline siblings like `</span>\n  <a>`) must collapse to
        // ~keep a single ASCII space — Tier-2's text-node normalize_whitespace folds
        // ~keep any `\n` + spaces run into one space.  Without this, Tier-1 falls
        // ~keep through to `decode_and_collapse_into` which preserves the `\n` and
        // ~keep emits `*[a](x)\n [b](y)*` where Tier-2 has `*[a](x) [b](y)*`.
        let active_tail: &str = state.cell_or_output_mut();
        if !active_tail.is_empty() && !active_tail.ends_with(' ') && !active_tail.ends_with('\n') {
            let dest = state.cell_or_output_mut();
            dest.push(' ');
        }
        return Ok(());
    }
    // ~keep Snapshot-then-flip: reached only once `raw` is known to carry real
    // ~keep (non-whitespace) content — every earlier branch above either
    // ~keep returns early or is gated on `raw_is_whitespace`. This is the one
    // ~keep true "does real content already precede this text node, anywhere
    // ~keep in the document" signal, mirroring Tier-2's
    // ~keep `Context::at_fresh_block_start` (see `Tier1State::at_document_start`'s
    // ~keep doc comment for why buffer emptiness can't stand in for it).
    let was_at_document_start = state.at_document_start;
    state.at_document_start = false;
    // ~keep CommonMark 4.8: leading whitespace at the very start of the
    // ~keep document is insignificant — mirrors Tier-2's `was_fresh_block_start`
    // ~keep exclusion of table cells and list items (which have their own
    // ~keep dedicated whitespace handling reached via `block_separator_after`
    // ~keep and the table-cell gate below). Also excluded inside a heading:
    // ~keep Tier-2's heading handler is a separate, allow-listed divergence
    // ~keep (flattens a heading's `<img>` to bare alt text instead of
    // ~keep Markdown image syntax) that assembles its line outside the
    // ~keep generic text-node/`at_fresh_block_start` pipeline this mirrors, so
    // ~keep it always keeps the real space between that alt text and
    // ~keep following prose — stripping it here would additionally drop that
    // ~keep space and compound one known divergence with a new one.
    let in_list_item_frame = state
        .stack
        .iter()
        .any(|frame| matches!(frame.spec.kind, TagKind::ListItem));
    let document_start_strip = was_at_document_start
        && !state.in_table_cell()
        && !in_list_item_frame
        && !state.escape_ctx.contains(EscapeCtx::HEADING);
    // ~keep Even when the text is not entirely whitespace, strip its LEADING
    // ~keep whitespace when:
    // ~keep   - we're at the start of an open inline element's body (`<a>`,
    // ~keep     `<strong>`, etc.), OR
    // ~keep   - the output ends with a block separator (`\n\n`) or a list-item
    // ~keep     marker — Tier-2's text-node `skip_prefix` logic does the same, OR
    // ~keep   - we're at the very start of the document (see
    // ~keep     `document_start_strip` above).
    let block_separator_after = {
        let active: &str = state.cell_or_output_mut();
        active.ends_with("\n\n")
            || active.ends_with("- ")
            || active.ends_with("* ")
            || active.ends_with("+ ")
            || ends_with_ordered_marker(active)
    };
    // ~keep When the leading run being stripped below sits at the very start of a
    // ~keep `<strong>`/`<em>` body specifically (not `<a>`/`<code>`, and not the
    // ~keep summary-accumulation kinds handled by `at_inline_frame_start`'s other
    // ~keep arm), Tier-2's `chomp_inline` (utility/content.rs) does not delete
    // ~keep that whitespace: it collapses the run to a single ASCII space that
    // ~keep `close_inline_marker` then migrates outside the opening marker.
    // ~keep Deleting it outright — correct for `<a>` (`normalize_link_label`
    // ~keep really does trim) and left as-is for `<code>` (fully verbatim,
    // ~keep handled separately) — would make `<em>&nbsp;x</em>` render `*x*`
    // ~keep instead of Tier-2's ` *x*`. Push one space into the buffer here so
    // ~keep `close_inline_marker`'s existing leading-migration block (added
    // ~keep alongside its trailing counterpart) has something to move.
    // ~keep At document start there is nothing to migrate the space onto —
    // ~keep Tier-2's `skip_prefix` drops the prefix outright rather than
    // ~keep collapsing it to a space (`<em>&nbsp;x</em>` alone renders `*x*`,
    // ~keep not ` *x*`), so `document_start_strip` suppresses the migration.
    let leading_ws_migrates_out = at_inline_frame_start
        && !document_start_strip
        && matches!(
            state.stack.last().map(|frame| frame.spec.kind),
            Some(TagKind::Strong | TagKind::Emphasis)
        );
    // ~keep The `!state.in_table_cell()` gate below exists for the general
    // ~keep block_separator_after case (a cell has no block separators to
    // ~keep speak of). But Tier-2's `normalize_link_label` trims a link label's
    // ~keep leading whitespace unconditionally, in a cell or not — so an `<a>`
    // ~keep frame must bypass the gate, or `<td><a> x</a></td>` keeps the space
    // ~keep Tier-2 trims (`[ x](...)` instead of `[x](...)`).
    let in_link_frame = matches!(state.stack.last().map(|frame| frame.spec.kind), Some(TagKind::Link));
    // ~keep Tier-2's `process_text_node` (`text_node.rs`) drops a text node's leading
    // ~keep whitespace run whenever `output.ends_with('\n') && prefix == " "` — one of
    // ~keep several `skip_prefix` conditions, and unlike the others it is not limited
    // ~keep to a double newline. A hard break inside a link (`<a>foo<br> bar</a>`) is
    // ~keep the only way link content ever ends in a bare `\n` (see `normalize_link_label`
    // ~keep and the `TagKind::LineBreak` `in_link` arm above, which now preserve/emit it
    // ~keep rather than folding it to a space), so this mirrors that one `skip_prefix`
    // ~keep arm scoped to exactly the case Tier-1 can produce it in: right after such a
    // ~keep break, `bar`'s leading space must not survive, or Tier-1 emits
    // ~keep `[foo  \n bar]` where Tier-2 emits `[foo  \nbar]`.
    let after_link_hard_break = in_link_frame && state.cell_or_output_mut().ends_with('\n');
    // ~keep Distinct from `at_inline_frame_start` above (whose unconditional strip is reserved
    // ~keep for Link/Strong/Emphasis/Code -- kinds with their own always-on trim wrapper in
    // ~keep Tier-2: link-label normalization, `chomp_inline`'s marker migration, code's verbatim
    // ~keep path). A bare inline element (`<span>`, `<u>`, `<mark>`, ... -- `TagKind::Inline`) has
    // ~keep none of those; its children flow through the exact same generic per-text-node path as
    // ~keep top-level prose, so Tier-2's `skip_prefix` (text_node.rs) only drops THIS text node's
    // ~keep leading whitespace when its own specific condition holds: `output.ends_with(' ') &&
    // ~keep prefix == " " && !previous_sibling_is_inline_tag(...)`. This text node is the very
    // ~keep first content written into the current frame (the same "nothing emitted yet" proxy
    // ~keep `at_inline_frame_start` uses for "no previous sibling"), so that last clause is
    // ~keep trivially true here; only `output.ends_with(' ')` needs checking. `prefix == " "` is
    // ~keep implicit in the trim below only firing when `raw` actually starts with ASCII
    // ~keep whitespace, which is exactly when Tier-2's `chomp` produces `prefix == " "`.
    // ~keep Confirmed empirically against Tier-2: `<span>with </span>\n  <span> more</span>` (a
    // ~keep whitespace-only sibling between the spans already pushed a separating space) collapses
    // ~keep the second span's own leading space away, but `<span>a</span><span> baz</span>` (no
    // ~keep separating whitespace, so `output` does not already end in a space) keeps it verbatim
    // ~keep (Google Docs' pretty-printed multi-`<span>` export hits the first shape: issue found
    // ~keep via `test_documents/html/office-gdocs/gdocs-web-page-export.html`). `<del>`/`<ins>`
    // ~keep (Strikethrough/Inserted) do NOT get this treatment -- Tier-2's dedicated strike/ins
    // ~keep wrapper leaves a genuine double space in the equivalent shape (verified) -- so they
    // ~keep are deliberately excluded here.
    let bare_inline_frame_start_after_space = {
        let buf_len = state.cell_or_output_mut().len();
        let is_bare_inline_start = matches!(
            state.stack.last(),
            Some(frame) if frame.content_start >= buf_len && matches!(frame.spec.kind, TagKind::Inline)
        );
        is_bare_inline_start && state.cell_or_output_mut().ends_with(' ')
    };
    // ~keep Tier-2's `handle_code` renders `<code>` content fully verbatim — no
    // ~keep trimming, no whitespace normalization at all. `in_code` (already
    // ~keep computed above) covers both bare `<code>` and `<pre><code>`; the
    // ~keep `<pre>` case is already excluded via `!in_pre`, so this only adds
    // ~keep the bare-`<code>` exclusion needed to stop a leading plain ASCII
    // ~keep space/tab/newline run from being deleted (`<code> x</code>` must
    // ~keep stay "` x`", not become "`x`").
    let raw = if !in_pre
        && !in_code
        && (!state.in_table_cell() || in_link_frame)
        && (at_inline_frame_start
            || block_separator_after
            || document_start_strip
            || after_link_hard_break
            || bare_inline_frame_start_after_space)
    {
        let trimmed = raw.trim_start_matches([' ', '\t', '\n', '\r']);
        if leading_ws_migrates_out && trimmed.len() < raw.len() {
            state.cell_or_output_mut().push(' ');
        }
        trimmed
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

    // ~keep Inside an `<a>` link frame, Tier-2's `normalize_link_label` replaces
    // ~keep newlines with spaces before whitespace collapsing.  Mirror that here so
    // ~keep text spanning `\n` inside an `<a>` (e.g. `<a>Skip to main\n  content</a>`)
    // ~keep collapses to `[Skip to main content]` instead of leaking the newline.
    // ~keep `<strong>`/`<em>` do NOT normalize newlines in Tier-2 — only links do.
    // ~keep
    // ~keep `<summary>` is treated the same as `<a>` here (Phase R-3): Tier-2's
    // ~keep handle_summary collects children into a local content buffer and
    // ~keep wraps in `**...**\n\n`; the surrounding text-normalization layer
    // ~keep collapses internal newline runs to single spaces before emission.
    // ~keep Without this, summary content with multi-line inline children leaks
    // ~keep `\n  \n  ` between text runs.
    // ~keep Table cells fold newlines the same way: Tier-2's `process_text_node`
    // ~keep has a dedicated `ctx.in_table_cell` branch (text_node.rs) that runs
    // ~keep `normalize_cell_whitespace_cow` per text node — folding `\n`/`\r` into
    // ~keep the whitespace run before collapsing to a single space — rather than
    // ~keep the generic non-cell chomp path.  Without this, a text node whose
    // ~keep trailing run is `\n` + indentation (pretty-printed HTML, e.g. a
    // ~keep multi-line `<td>` around an `<ins>`/`<span>`) leaves the `\n` byte
    // ~keep un-collapsed here; `close_table_cell`'s later blanket
    // ~keep `replace('\n', " ")` then turns that leftover `\n` into a SECOND
    // ~keep space next to the one already collapsed from the trailing run,
    // ~keep double-spacing the cell relative to Tier-2.
    let inside_inline = state.in_table_cell()
        || state.in_summary()
        || state.stack.iter().any(|frame| matches!(frame.spec.kind, TagKind::Link));

    // ~keep Phase Y: text-node chomp.  Tier-2's text_node.rs runs `chomp()` on
    // ~keep every text node and substitutes the leading and trailing whitespace
    // ~keep runs with simpler stand-ins:
    // ~keep   prefix → `" "` if the run had any leading whitespace, else `""`
    // ~keep   suffix → `"\n\n"` if trailing run contained `\n\n`,
    // ~keep          → `" "`   if trailing run had space/tab (folding any `\n`),
    // ~keep          → `trailing_single_newline_join(...)` if trailing run was `\n` only.
    // ~keep Without this, Tier-1 keeps the literal `\n  ` in text like
    // ~keep "The number of\n  " and emits `of\n ` while Tier-2 emits `of `,
    // ~keep and likewise the leading whitespace case `</em>\n  baz` produces
    // ~keep `*bar*\n baz` instead of `*bar* baz`.
    // ~keep
    // ~keep Applied only outside inline frames (which call
    // ~keep `decode_and_collapse_into_inline` and handle `\n` collapse already),
    // ~keep outside `<pre>` (verbatim), and outside table cells (which run
    // ~keep `normalize_whitespace_cow` directly).
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
            let core_start = leading_len;
            let core_end = trimmed_len;
            if core_start >= core_end {
                // ~keep Whitespace-only text node — already handled by the
                // ~keep earlier whitespace-only branches; skip Phase Y here.
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
                    trailing_single_newline_join(state, next_tag_is_span)
                } else {
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

    // ~keep Issue #458: Tier-2 escapes a literal `\` in prose whether or not any
    // ~keep `escape_*` flag is set (`text::backslash_needs_escape`), so Tier-1 has to
    // ~keep apply the same rule or the two tiers disagree on every document containing
    // ~keep one.  The rule runs over the bytes this call appends rather than over `raw`,
    // ~keep because its target must be the *decoded* text with whitespace already
    // ~keep collapsed — exactly what Tier-2 hands to `text::escape`.
    let in_cell = state.in_table_cell();

    indent_fresh_list_item_text_line(state);

    // ~keep A link label and a `<summary>` body are the exception: Tier-1 folds their
    // ~keep newlines into spaces while collapsing, but Tier-2 escapes the text node
    // ~keep first and only then folds (`normalize_link_label` / the summary
    // ~keep accumulator).  A `\` that sat before a newline in the source is escaped by
    // ~keep Tier-2 and would not be by a pass reading the already-folded text, so escape
    // ~keep the decoded-but-unfolded text and let the collapse run over the result.
    // ~keep Entities are resolved up front, hence `has_entities: false` below — the
    // ~keep collapse pass must not decode a second time.
    if inside_inline && !in_cell && raw.contains('\\') {
        let mut staged = String::with_capacity(raw.len() + 8);
        if has_entities {
            decode_entities_into(&mut staged, raw, base_offset)?;
        } else {
            staged.push_str(raw);
        }
        escape_backslash_run(&mut staged, 0, false);
        let dest = state.cell_or_output_mut();
        return decode_and_collapse_into_inline(dest, &staged, false, base_offset);
    }

    let dest = state.cell_or_output_mut();
    let emitted_from = dest.len();

    if !has_entities {
        let needle_present = if inside_inline {
            memchr3(b' ', b'\t', b'\n', raw.as_bytes()).is_some()
        } else {
            memchr::memchr2(b' ', b'\t', raw.as_bytes()).is_some()
        };
        if !needle_present {
            dest.push_str(raw);
        } else if inside_inline {
            decode_and_collapse_into_inline(dest, raw, false, base_offset)?;
        } else {
            decode_and_collapse_into(dest, raw, false, base_offset)?;
        }
    } else if inside_inline {
        decode_and_collapse_into_inline(dest, raw, has_entities, base_offset)?;
    } else {
        decode_and_collapse_into(dest, raw, has_entities, base_offset)?;
    }

    escape_backslash_run(dest, emitted_from, in_cell);
    Ok(())
}

/// Apply `text::backslash_needs_escape` to the run `buffer[from..]`.
///
/// `run_ends_at_last_byte` selects where the run ends, which genuinely differs between
/// the two Tier-2 call sites this mirrors:
///
/// - Prose (`text_node.rs`'s normalized branch) escapes `chomp()`'s *core*, the text
///   node with its boundary whitespace stripped — so a trailing `\` counts as
///   end-of-run even when spaces or a newline follow it in the emitted bytes. Pass
///   `false`.
/// - A table cell (`text_node.rs`'s `in_table_cell` branch) escapes the whole
///   normalized text node with no chomp, so the run ends at the last byte whatever it
///   is. Pass `true`.
///
/// Getting this backwards flips `<p>a\ </p>` or `<td>a\ </td>` by one byte against
/// Tier-2. ~keep
fn escape_backslash_run(buffer: &mut String, from: usize, run_ends_at_last_byte: bool) {
    if memchr::memchr(b'\\', &buffer.as_bytes()[from..]).is_none() {
        return;
    }
    let run = buffer[from..].to_owned();
    let run_end = if run_ends_at_last_byte {
        run.len()
    } else {
        run.trim_end().len()
    };
    let bytes = &run.as_bytes()[..run_end];

    let mut rewritten = String::with_capacity(run.len() + 4);
    let mut copied_to = 0usize;
    for i in memchr::memchr_iter(b'\\', bytes) {
        if crate::text::backslash_needs_escape(bytes, i) {
            rewritten.push_str(&run[copied_to..i]);
            rewritten.push_str(r"\\");
            copied_to = i + 1;
        }
    }
    rewritten.push_str(&run[copied_to..]);

    buffer.truncate(from);
    buffer.push_str(&rewritten);
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
    // ~keep Mirrors Tier-2's `normalize_block_whitespace_cow` (text.rs): when a literal
    // ~keep `\n` survives into the output (only possible here when `collapse_newlines`
    // ~keep is false -- the `true` variant folds `\n` straight into a space below and
    // ~keep never leaves one in `out`), a run of spaces/tabs immediately after it is a
    // ~keep Markdown continuation line's leading indentation. A compliant parser drops
    // ~keep that entirely on reparse regardless of width (CommonMark spec 4.9), so
    // ~keep collapsing it to one space here was not a fixed point -- see the CommonMark
    // ~keep spec fixpoint oracle, example 182. Zero is. `s` has already had its own
    // ~keep leading/trailing whitespace resolved into a synthetic prefix/suffix by the
    // ~keep caller's Phase Y step, so every `\n` this sees with more content after it is
    // ~keep a genuine mid-text line break, never the text node's own edge.
    let mut at_line_start = false;
    while i < bytes.len() {
        let next_special = match (has_entities, collapse_newlines) {
            (true, true) => {
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
                at_line_start = !collapse_newlines && bytes[pos - 1] == b'\n';
            }
            match bytes[pos] {
                b' ' | b'\t' if at_line_start => {
                    i = pos + 1;
                }
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
                    at_line_start = false;
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
        // ~keep Phase N3: entity name (`&name;`) not in Tier-1's decode table.
        // ~keep Tier-2 and mdream pass these through verbatim instead of decoding.
        // ~keep Push the raw `&name;` and advance past it.
        out.push_str(&s[amp..=end]);
        return Ok(end + 1);
    }
    out.push('&');
    Ok(amp + 1)
}

/// Apply the escape-context bits for an opening tag.
///
/// The close path restores `state.escape_ctx` directly from `frame.prev_escape_ctx`
/// so a symmetric `remove_open_escape_ctx` is not needed.
#[inline]
fn apply_open_escape_ctx(state: &mut Tier1State, spec: &TagSpec) {
    if spec.kind == TagKind::Pre {
        state.escape_ctx |= EscapeCtx::PRE | EscapeCtx::CODE;
        return;
    }

    let bit = match spec.kind {
        TagKind::Code => EscapeCtx::CODE,
        TagKind::Link => EscapeCtx::LINK,
        TagKind::Blockquote => EscapeCtx::BLOCKQUOTE,
        TagKind::Heading(_) => EscapeCtx::HEADING,
        TagKind::Strong => EscapeCtx::STRONG,
        _ => return,
    };

    state.escape_ctx |= bit;
}

/// Report whether an attribute is present, with or without a value.
///
/// [`find_attr`] returns the attribute's *value* and so cannot tell an absent
/// attribute from a valueless one (`<td colspan>`).  Tier-2's spanning-cell test
/// (`block/table/scanner.rs`: `attrs.get("colspan").is_some()`) keys off presence
/// alone, so mirroring it needs this distinction.
fn has_attr(attrs: &[(&[u8], Option<&[u8]>)], key: &[u8]) -> bool {
    attrs.iter().any(|(k, _)| k.eq_ignore_ascii_case(key))
}

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

/// Mirrors `should_drop_for_preprocessing` (preprocessing_helpers.rs) for
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

    if options.preprocessing.remove_forms && name_lower == b"form" {
        return true;
    }

    if !options.preprocessing.remove_navigation {
        return false;
    }

    if name_lower == b"nav" {
        return true;
    }

    // ~keep <header> / <footer> / <aside> — drop only when navigation hints present.
    // ~keep (Aggressive would drop footer/aside unconditionally, but Aggressive routes
    // ~keep through Tier-2 via the existing router gate so Tier-1 only needs the
    // ~keep Standard-preset behaviour: nav-hint check.)
    if matches!(name_lower, b"header" | b"footer" | b"aside") {
        return byte_attrs_have_navigation_hint(attrs);
    }

    false
}

/// Byte-level equivalent of `element_has_navigation_hint` for use in the
/// Tier-1 scanner where attributes are raw `&[u8]` slices rather than a
/// parsed `tl::HTMLTag`.
fn byte_attrs_have_navigation_hint(attrs: &[(&[u8], Option<&[u8]>)]) -> bool {
    if let Some(role) = find_attr(attrs, b"role") {
        let role_lc = role.to_ascii_lowercase();
        if matches!(role_lc.as_slice(), b"navigation" | b"menubar" | b"tablist" | b"toolbar") {
            return true;
        }
    }

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
    let mut start = 0;
    let len = value.len();
    loop {
        while start < len && value[start].is_ascii_whitespace() {
            start += 1;
        }
        if start >= len {
            break;
        }
        let mut end = start;
        while end < len && !value[end].is_ascii_whitespace() {
            end += 1;
        }
        let token_bytes = &value[start..end];
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
            start = end;
            continue;
        };

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
    // ~keep Mirror Tier-2's `inline/link.rs:82` which captures the title attribute
    // ~keep via tl::parse's `as_utf8_str()` — tl decodes numeric entities
    // ~keep (`&#039;` → `'`) but preserves named entities (`&amp;`, `&quot;`,
    // ~keep `&lt;`).  Use a partial-decode pass for titles to match.
    let title = find_attr(attrs, b"title").map(decode_title_attr).transpose()?;
    Ok((href, title))
}

/// Decode a link-title attribute: numeric entities (`&#NNN;`, `&#xNNN;`)
/// resolve to characters, named entities (`&amp;`, `&quot;`, etc.) survive
/// as-is.  Mirrors tl::parse's `as_utf8_str()` behaviour on attribute values.
/// Decode a link-title attribute: numeric entities (`&#NNN;`, `&#xNNN;`)
/// resolve to characters, named entities (`&amp;`, `&quot;`, etc.) survive
/// as-is.  Mirrors Tier-2's observed behaviour on link titles: it decodes
/// `&#039;` → `'` but preserves `&amp;`/`&quot;` literally.
fn decode_title_attr(bytes: &[u8]) -> Result<String, BailReason> {
    let s = std::str::from_utf8(bytes).map_err(|_| BailReason::Classifier)?;
    if !s.contains("&#") {
        return Ok(s.to_owned());
    }
    let mut out = String::with_capacity(s.len());
    let bytes_s = s.as_bytes();
    let mut i = 0;
    while i < bytes_s.len() {
        let Some(rel) = memchr::memchr(b'&', &bytes_s[i..]) else {
            out.push_str(&s[i..]);
            break;
        };
        let amp_pos = i + rel;
        if amp_pos > i {
            out.push_str(&s[i..amp_pos]);
        }
        if amp_pos + 1 >= bytes_s.len() || bytes_s[amp_pos + 1] != b'#' {
            out.push('&');
            i = amp_pos + 1;
            continue;
        }
        let mut j = amp_pos + 2;
        while j < bytes_s.len() && bytes_s[j] != b';' {
            j += 1;
        }
        if j >= bytes_s.len() {
            out.push_str(&s[amp_pos..]);
            break;
        }
        let body = &s[amp_pos + 2..j];
        let (digits, radix) = match body.strip_prefix(['x', 'X']) {
            Some(hex) => (hex, 16),
            None => (body, 10),
        };
        if let Ok(cp) = crate::text::parse_character_reference_number(digits, radix) {
            if let Some(replacement) = crate::text::numeric_character_reference_override(cp) {
                out.push(replacement);
                i = j + 1;
                continue;
            }
            if let Some(ch) = u32::try_from(cp).ok().and_then(char::from_u32) {
                out.push(ch);
                i = j + 1;
                continue;
            }
        }
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
    decode_entities_into(&mut out, s, 0)?;
    Ok(out)
}

/// Bail when this scanner cannot know whether Tier-2 will canonicalize an image's
/// `alt`/`title` entities, and the answer would be visible in the output.
///
/// ~keep `canonicalize_attr_entities` is set from `has_custom_element_tags` alone,
/// ~keep because that is the one repair trigger a byte scanner can evaluate. It is not
/// ~keep the only one: `has_inline_block_misnest` routes a document through the same
/// ~keep html5ever roundtrip, and that check needs a parsed DOM, which Tier-1 runs
/// ~keep before anything has been parsed and exists precisely to avoid building.
///
/// ~keep So when the flag is false the scanner has not established that Tier-2 will
/// ~keep leave entities alone -- only that ONE of the reasons to rewrite them does not
/// ~keep apply. Emitting the raw form on that basis is a guess. It was previously right
/// ~keep by accident: `has_custom_element_tags` treated any `<!--comment-->` as a custom
/// ~keep element, so nearly every real document set the flag and the gap stayed hidden
/// ~keep until that false positive was fixed.
///
/// ~keep Bailing is safe in both directions -- Tier-2's fallback produces Tier-2's answer
/// ~keep whether or not it repairs -- so the only cost is losing the fast path. Restricting
/// ~keep it to values where the two branches genuinely disagree keeps that cost off the
/// ~keep common cases. The comparison is against what the canonicalizing branch would have
/// ~keep emitted -- `canonicalize_attr_entities(decode_attr(raw))` -- not against `raw`
/// ~keep itself: an `alt` written `&amp;` decodes to `&` and canonicalizes straight back to
/// ~keep `&amp;`, so both branches agree and there is nothing to decide. Only a spelling the
/// ~keep roundtrip would rewrite, such as `&#x22;` becoming `&quot;`, actually forks.
fn bail_if_canonicalization_is_undecidable(raw: &str, decoded: &str) -> Result<(), BailReason> {
    if canonicalize_attr_entities(decoded) != raw {
        return Err(BailReason::Classifier);
    }
    Ok(())
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
    let content = content.trim_end_matches('\n');
    if content.is_empty() {
        return String::new();
    }

    let lines: Vec<&str> = content.split('\n').collect();
    let mut result = String::with_capacity(content.len() + lines.len() * 2);

    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
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
    let raw = raw.strip_prefix('\n').unwrap_or(raw);
    let raw = raw.trim_end_matches('\n');
    if raw.is_empty() {
        return String::new();
    }

    let min_indent = raw
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
        .min()
        .unwrap_or(0);

    let mut result = String::with_capacity(raw.len() + raw.lines().count() * 4);
    for line in raw.lines() {
        if line.trim().is_empty() {
            // ~keep Empty / whitespace-only line: emit as a bare `\n` (no 4-space
            // ~keep indent prefix).  Tier-2's `block/code.rs` also skips the indent
            // ~keep for blank lines inside indented code blocks — without this,
            // ~keep round-tripped CommonMark `    code\n    \n    code` would
            // ~keep render with stray trailing spaces in the blank gap.
        } else {
            result.push_str("    ");
            // ~keep Convert char-count `min_indent` into a byte offset by walking
            // ~keep `char_indices`.  Indexing `line[min_indent..]` directly panics
            // ~keep when the leading whitespace contains multibyte characters such
            // ~keep as `\u{a0}` (NBSP).  Mirrors Tier-2's `dedent_code_block`
            // ~keep (text/processing.rs:38-50).
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

// ~keep ── GFM table emission ────────────────────────────────────────────────────────

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
    // ~keep Emit caption (if any) BEFORE the table body.
    // ~keep
    // ~keep Mirrors Tier-2 builder.rs caption handling: `*escaped_text*\n\n`.
    // ~keep Tier-2 emits the caption as part of the table child loop, which runs
    // ~keep before the rows are rendered, so the caption appears even when there
    // ~keep are no table rows.  The caption text has already been trimmed and
    // ~keep hyphen-escaped when `</caption>` was processed.
    if let Some(ref caption) = ts.caption_text {
        if !caption.is_empty() {
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

    // ~keep Pre-table separator: mirrors Tier-2's `convert_table` logic exactly.
    // ~keep Tier-2 (block/table/mod.rs): `if !output.is_empty() && !output.ends_with("\n\n")`
    // ~keep — only adds separator when there is existing output (no leading blank lines).
    if !target.is_empty() && !target.ends_with("\n\n") {
        if target.ends_with('\n') {
            target.push('\n');
        } else {
            target.push_str("\n\n");
        }
    }

    // ~keep Pre-compute max column widths across ALL rows (mirrors Tier-2's pre-pass).
    // ~keep Tier-2: separator dashes = max(col_content_char_count_across_all_rows, 3).
    // ~keep col_count is the colspan-expanded column count (sum of colspans per row).
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
            // ~keep Only the cell's anchor column owns the width — spanned columns
            // ~keep contribute zero (matches Tier-2's per-cell pad calculation).
            if col < col_widths.len() && w > col_widths[col] {
                col_widths[col] = w;
            }
            col += usize::from(*span);
        }
    }

    for (row_index, row) in ts.rows.iter().enumerate() {
        // ~keep Row: `|` then each cell as ` text |` (padded to col_width like Tier-2).
        target.push('|');
        let mut col = 0usize;
        for (cell, span) in row {
            target.push(' ');
            target.push_str(cell);
            // ~keep Pad to column width (mirrors Tier-2 cell.rs padding logic).
            let cell_len = cell.chars().count();
            let col_w = col_widths.get(col).copied().unwrap_or(0);
            for _ in cell_len..col_w {
                target.push(' ');
            }
            // ~keep Tier-2 (cell.rs:248): `for _ in 0..colspan { output.push_str(" |") }`.
            // ~keep colspan trailing ` |` separators per cell — produces `| Header | | |`
            // ~keep for `<th colspan="3">Header</th>` instead of `| Header |  |  |`.
            for _ in 0..*span {
                target.push_str(" |");
            }
            col += usize::from(*span);
        }
        target.push('\n');

        // ~keep After row 0 (the header row), emit the separator row.
        // ~keep Tier-2: col_widths.get(i).unwrap_or(0).max(MIN_SEPARATOR_DASHES).
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
        // ~keep Latin-1 Supplement (U+00A0–U+00FF) — html5ever's Tier-2 backend
        // ~keep decodes the full HTML5 named entity table; mirror the Latin-1
        // ~keep block here so Tier-1 byte-equality holds for accented Western
        // ~keep text without bailing.  Entries already defined above (`nbsp`,
        // ~keep `copy`, `reg`, `laquo`, `raquo`, `frac12`, etc.) are not repeated.
        "iexcl" => "\u{00A1}",
        "brvbar" => "\u{00A6}",
        "sect" => "\u{00A7}",
        "uml" => "\u{00A8}",
        "ordf" => "\u{00AA}",
        "not" => "\u{00AC}",
        "shy" => "\u{00AD}",
        "macr" => "\u{00AF}",
        "sup2" => "\u{00B2}",
        "sup3" => "\u{00B3}",
        "acute" => "\u{00B4}",
        "micro" => "\u{00B5}",
        "para" => "\u{00B6}",
        "cedil" => "\u{00B8}",
        "sup1" => "\u{00B9}",
        "ordm" => "º",
        "iquest" => "\u{00BF}",
        "Agrave" => "\u{00C0}",
        "Aacute" => "\u{00C1}",
        "Acirc" => "\u{00C2}",
        "Atilde" => "\u{00C3}",
        "Auml" => "\u{00C4}",
        "Aring" => "\u{00C5}",
        "AElig" => "\u{00C6}",
        "Ccedil" => "\u{00C7}",
        "Egrave" => "\u{00C8}",
        "Eacute" => "\u{00C9}",
        "Ecirc" => "\u{00CA}",
        "Euml" => "\u{00CB}",
        "Igrave" => "\u{00CC}",
        "Iacute" => "\u{00CD}",
        "Icirc" => "\u{00CE}",
        "Iuml" => "\u{00CF}",
        "ETH" => "\u{00D0}",
        "Ntilde" => "\u{00D1}",
        "Ograve" => "\u{00D2}",
        "Oacute" => "\u{00D3}",
        "Ocirc" => "\u{00D4}",
        "Otilde" => "\u{00D5}",
        "Ouml" => "\u{00D6}",
        "Oslash" => "\u{00D8}",
        "Ugrave" => "\u{00D9}",
        "Uacute" => "\u{00DA}",
        "Ucirc" => "\u{00DB}",
        "Uuml" => "\u{00DC}",
        "Yacute" => "\u{00DD}",
        "THORN" => "\u{00DE}",
        "szlig" => "\u{00DF}",
        "agrave" => "\u{00E0}",
        "aacute" => "\u{00E1}",
        "acirc" => "\u{00E2}",
        "atilde" => "\u{00E3}",
        "auml" => "\u{00E4}",
        "aring" => "\u{00E5}",
        "aelig" => "\u{00E6}",
        "ccedil" => "\u{00E7}",
        "egrave" => "\u{00E8}",
        "eacute" => "\u{00E9}",
        "ecirc" => "\u{00EA}",
        "euml" => "\u{00EB}",
        "igrave" => "\u{00EC}",
        "iacute" => "\u{00ED}",
        "icirc" => "\u{00EE}",
        "iuml" => "\u{00EF}",
        "eth" => "\u{00F0}",
        "ntilde" => "\u{00F1}",
        "ograve" => "\u{00F2}",
        "oacute" => "\u{00F3}",
        "ocirc" => "\u{00F4}",
        "otilde" => "\u{00F5}",
        "ouml" => "\u{00F6}",
        "oslash" => "\u{00F8}",
        "ugrave" => "\u{00F9}",
        "uacute" => "\u{00FA}",
        "ucirc" => "\u{00FB}",
        "uuml" => "\u{00FC}",
        "yacute" => "\u{00FD}",
        "thorn" => "\u{00FE}",
        "yuml" => "\u{00FF}",
        _ => return decode_named_entity_fallback(out, name),
    };
    out.push_str(s);
    true
}

/// Falls back to the full WHATWG named-character-reference table for names
/// outside the hot subset above.
///
/// ~keep `html_escape::NAMED_ENTITIES` is the exact table Tier-2 decodes
/// ~keep against (see `text::decode_html_entities_cow`, which calls
/// ~keep `html_escape::decode_html_entities`), so looking it up here — rather
/// ~keep than hand-copying a second ~2000-entry table into Tier-1 — is what
/// ~keep makes Tier-1 byte-identical to Tier-2 for names like `&notin;`
/// ~keep instead of merely covering a hand-picked subset.
fn decode_named_entity_fallback(out: &mut String, name: &str) -> bool {
    let name_bytes = name.as_bytes();
    if let Ok(index) = html_escape::NAMED_ENTITIES.binary_search_by(|(entity_name, _)| entity_name.cmp(&name_bytes)) {
        out.push_str(html_escape::NAMED_ENTITIES[index].1);
        return true;
    }
    decode_numeric_entity_into(out, name)
}

fn decode_numeric_entity_into(out: &mut String, name: &str) -> bool {
    let Some(rest) = name.strip_prefix('#') else {
        return false;
    };
    let (digits, radix) = match rest.strip_prefix(['x', 'X']) {
        Some(hex) => (hex, 16),
        None => (rest, 10),
    };
    let Ok(code_point) = crate::text::parse_character_reference_number(digits, radix) else {
        return false;
    };
    if let Some(replacement) = crate::text::numeric_character_reference_override(code_point) {
        out.push(replacement);
        return true;
    }
    match u32::try_from(code_point).ok().and_then(char::from_u32) {
        Some(ch) => {
            out.push(ch);
            true
        }
        None => false,
    }
}

/// Skip `<!--...-->`, `<!DOCTYPE...>`, or any `<!...>` construct.
/// Returns the position immediately after the closing `>`.
///
/// On failure returns `Err(BailReason::LiteralLt)`.
fn skip_bang(bytes: &[u8], pos: usize) -> Result<usize, BailReason> {
    let start = pos + 2;

    if bytes.get(start) == Some(&b'-') && bytes.get(start + 1) == Some(&b'-') {
        let comment_start = start + 2;
        let mut i = comment_start;
        while i + 2 < bytes.len() {
            if bytes[i] == b'-' && bytes[i + 1] == b'-' && bytes[i + 2] == b'>' {
                return Ok(i + 3);
            }
            i += 1;
        }
        // ~keep Unclosed comment — bail
        return Err(BailReason::LiteralLt { offset: pos });
    }

    let mut i = start;
    while i < bytes.len() {
        if bytes[i] == b'>' {
            return Ok(i + 1);
        }
        i += 1;
    }
    Err(BailReason::LiteralLt { offset: pos })
}

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

/// Peek the lowercased tag name of the upcoming OPEN tag at `bytes[lt_pos]`
/// (expected to be `<`), if there is one — `None` for a close tag (`</...`),
/// a non-tag `<` (comment, bang, literal), or EOF. Shared by
/// `upcoming_tag_is_list_open` and `upcoming_tag_is_named` so the main scan
/// loop can peek ahead, BEFORE the tag itself is parsed, to tell `flush_text`
/// what kind of tag the text about to be flushed sits directly in front of.
fn upcoming_open_tag_name<'b>(bytes: &[u8], lt_pos: usize, buf: &'b mut [u8; MAX_TAG_NAME_BYTES]) -> Option<&'b [u8]> {
    if bytes.get(lt_pos) != Some(&b'<') {
        return None;
    }
    let &next = bytes.get(lt_pos + 1)?;
    if !parse::is_tag_name_start(next) {
        return None;
    }
    let name_start = lt_pos + 1;
    let name_end = parse::scan_tag_name(bytes, name_start);
    Some(lowercase_into(&bytes[name_start..name_end], buf))
}

/// Decide what a text node's trailing *bare* `\n` (no accompanying space/tab,
/// not part of a `\n\n` run) collapses to.
///
/// Mirrors Tier-2's `has_trailing_single_newline` follow-up step
/// (`text_node.rs`, the `else if has_trailing_single_newline` arm): `chomp()`
/// itself reduces that trailing run to nothing, but a supplementary check then
/// puts a joining character back unless the block already ends in a blank
/// line. Only reachable from the non-inline, non-table-cell Phase Y branch in
/// `flush_text`, so `state.output` — not `cell_or_output_mut()` — is always
/// the right buffer to inspect here (mirrors Tier-2's `ctx.block_content_start`
/// slice of the real `output`, which the same non-inline/non-cell precondition
/// guarantees is `state.output` too).
///
/// - `<span>` is a hardcoded exception in Tier-2's source: no join at all.
/// - Otherwise: a blank-line break already in place needs nothing either. The
///   "already" is scoped to the enclosing `<p>`/`<div>`'s OWN content (Tier-2's
///   `ctx.block_content_start`, i.e. `nearest_block_content_start` here) —
///   never the whole document buffer. A paragraph that just opened right
///   after a preceding one leaves the DOCUMENT ending in "\n\n" (its own
///   leading separator) while its OWN content is still empty; scoping the
///   check avoids reading that separator as "this text node already touches a
///   blank line" and wrongly swallowing the join.
/// - Otherwise: a paragraph ancestor, or a `<strong>`/`<em>` (Tier-2's
///   `inline_depth`-incrementing wrappers) ancestor, joins with a single
///   space; anything else (e.g. a bare `<div>`) joins with a literal newline.
fn trailing_single_newline_join(state: &Tier1State, next_tag_is_span: bool) -> &'static str {
    if next_tag_is_span {
        return "";
    }
    let block_start = clamp_to_char_boundary(&state.output, nearest_block_content_start(state));
    if state.output[block_start..].ends_with("\n\n") {
        return "";
    }
    let in_paragraph_or_inline_wrapper = state.stack.iter().any(|frame| {
        matches!(
            frame.spec.kind,
            TagKind::Paragraph | TagKind::Strong | TagKind::Emphasis
        )
    });
    if in_paragraph_or_inline_wrapper { " " } else { "\n" }
}

/// Position where the innermost enclosing `<p>`/`<div>` frame's OWN content
/// starts in `state.output` — Tier-1's equivalent of Tier-2's
/// `ctx.block_content_start` (set in `block/paragraph.rs`, which handles both
/// tags). Falls back to `0` (the whole buffer) when no such ancestor is open,
/// matching `Context::default`'s `block_content_start: 0`.
fn nearest_block_content_start(state: &Tier1State) -> usize {
    state
        .stack
        .iter()
        .rev()
        .find(|frame| matches!(frame.spec.kind, TagKind::Paragraph | TagKind::Block))
        .map_or(0, |frame| frame.content_start)
}

/// Peek whether the upcoming tag at `bytes[lt_pos]` is an opening `<ul>`/`<ol>`.
/// See `flush_text`'s `next_tag_is_list` parameter for why that distinction
/// matters.
fn upcoming_tag_is_list_open(bytes: &[u8], lt_pos: usize) -> bool {
    let mut name_buf = [0u8; MAX_TAG_NAME_BYTES];
    matches!(
        upcoming_open_tag_name(bytes, lt_pos, &mut name_buf),
        Some(b"ul" | b"ol")
    )
}

/// Peek whether the upcoming tag at `bytes[lt_pos]` is an opening tag named
/// exactly `name` (already lowercase). See `flush_text`'s `next_tag_is_img`
/// parameter for why that distinction matters.
fn upcoming_tag_is_named(bytes: &[u8], lt_pos: usize, name: &[u8]) -> bool {
    let mut name_buf = [0u8; MAX_TAG_NAME_BYTES];
    upcoming_open_tag_name(bytes, lt_pos, &mut name_buf) == Some(name)
}
