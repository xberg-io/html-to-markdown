//! Content extraction and manipulation utilities.
//!
//! Functions for extracting and processing element content, including text collection
//! and empty element detection.

use crate::text;
use std::borrow::Cow;
#[cfg(feature = "visitor")]
use std::collections::BTreeMap;

pub use crate::converter::DomContext;

/// Collect all attributes from an HTML tag as a `BTreeMap<String, String>`.
///
/// Boolean attributes (those with `None` as the value) are skipped; only
/// attributes that carry an explicit value are included.
#[cfg(feature = "visitor")]
pub fn collect_tag_attributes(tag: &tl::HTMLTag) -> BTreeMap<String, String> {
    tag.attributes()
        .iter()
        .filter_map(|(k, v)| v.as_ref().map(|val| (k.to_string(), val.to_string())))
        .collect()
}

/// Chomp whitespace from inline element content, preserving line breaks.
///
/// Similar to `text::chomp` but handles line breaks from `<br>` tags specially.
/// Line breaks are extracted as suffix to be placed outside formatting.
/// Returns (prefix, suffix, `trimmed_text`).
pub fn chomp_inline(text: &str) -> (&str, &str, &str) {
    if text.is_empty() {
        return ("", "", "");
    }

    let has_trailing_linebreak = text.ends_with("  \n") || text.ends_with("\\\n");

    if text.trim().is_empty() && !has_trailing_linebreak {
        // ~keep A whitespace-only body (e.g. `<i> </i>`) is ONE space in the source, not
        // ~keep two: the starts_with/ends_with checks below would treat the very same run
        // ~keep as both prefix AND suffix, and every caller's "push prefix, then
        // ~keep append_inline_suffix(suffix)" shape then emits it twice (issue #481).
        // ~keep Represented once, as a prefix (with an empty suffix so
        // ~keep `append_inline_suffix` is a no-op), every existing caller naturally
        // ~keep collapses back to a single space. A hard trailing linebreak is excluded
        // ~keep above and keeps the full logic below -- it is not interchangeable with a
        // ~keep plain space. A body that is only a newline counts too: that is what a
        // ~keep `<br>` with nothing before it leaves in a wrapper's scratch buffer
        // ~keep (`line_break.rs`'s block-start arm), and `<b>Alpha</b><b><br></b><b>Beta</b>`
        // ~keep joined its words when it was worth nothing (issue #502).
        return (" ", "", "");
    }

    let prefix = if text.starts_with(&[' ', '\t'][..]) { " " } else { "" };

    let suffix = if has_trailing_linebreak {
        if text.ends_with("  \n") { "  \n" } else { "\\\n" }
    } else if text.ends_with(&[' ', '\t'][..]) {
        " "
    } else {
        ""
    };

    let trimmed = if has_trailing_linebreak {
        text.strip_suffix("  \n").map_or_else(
            || text.strip_suffix("\\\n").map_or_else(|| text.trim(), |s| s.trim()),
            |s| s.trim(),
        )
    } else {
        text.trim()
    };

    (prefix, suffix, trimmed)
}

/// Merge a newly-opening emphasis delimiter into the matching close marker `output` already
/// ends with, instead of emitting a second, textually-adjacent delimiter run.
///
/// `CommonMark` parses `*A**B*` as `A` in emphasis followed by a literal `**B*` — NOT as two
/// consecutive emphasis runs — because a closing delimiter run immediately followed by an
/// opening one of the same character forms a single, longer run (issue #483). Sibling
/// `<i>`/`<b>` elements that each independently wrap their own content in `*…*`/`**…**`
/// therefore corrupt on reparse unless the second element's open marker is suppressed and the
/// previous element's close marker is removed, letting the merged run share one pair of
/// delimiters (`<i>A</i><i>B</i>` -> `*AB*`, not `*A**B*`).
///
/// Pops exactly `count` trailing copies of `symbol` from `output` and returns `true` only when:
/// - `output` ends with a run of `symbol` whose length is EXACTLY `count` (not more, not
///   fewer) -- so `***x***` (a real 3-run) is left alone rather than half-eaten, and
/// - the character immediately preceding that run (if any) is neither `symbol` nor `\` -- so
///   a longer run one byte further back, or a backslash-escaped literal (`\*`), is left alone.
///
/// Returns `false` and leaves `output` untouched otherwise, including when `count == 0` (no
/// delimiter to merge, e.g. a `<strong>` nested inside another `<strong>`, which emits no
/// marker of its own).
pub fn merge_adjacent_emphasis(output: &mut String, symbol: char, count: usize) -> bool {
    if count == 0 {
        return false;
    }

    let mut rev = output.chars().rev();
    for _ in 0..count {
        match rev.next() {
            Some(c) if c == symbol => {}
            _ => return false,
        }
    }
    if let Some(preceding) = rev.next() {
        if preceding == symbol || preceding == '\\' {
            return false;
        }
    }

    let new_len = output.len() - symbol.len_utf8() * count;
    output.truncate(new_len);
    true
}

/// Get the text content of a node and its children.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn get_text_content(node_handle: &tl::NodeHandle, parser: &tl::Parser, dom_ctx: &DomContext) -> String {
    dom_ctx.text_content(*node_handle, parser)
}

/// Determine whether a node is block-level, preferring the DOM context's precomputed tag
/// info when available and falling back to a name-based check otherwise.
///
/// ~keep Shared between `collect_link_label_text` (topmost block *descendants*, for
/// ~keep skipping block content out of an inline label) and `handlers/link.rs`'s
/// ~keep direct-children partition (issue #490) -- both need the identical block/inline
/// ~keep classification, or a byte could end up assigned to neither half, or both.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn node_is_block_level(handle: &tl::NodeHandle, parser: &tl::Parser, dom_ctx: &DomContext) -> bool {
    let Some(tl::Node::Tag(tag)) = handle.get(parser) else {
        return false;
    };
    dom_ctx.tag_info(handle.get_inner(), parser).map_or_else(
        || {
            let tag_name = normalized_tag_name(tag.name().as_utf8_str());
            is_block_level_element(tag_name.as_ref())
        },
        |info| info.is_block,
    )
}

/// Collect inline text for link labels, skipping block-level descendants.
#[allow(clippy::match_wildcard_for_single_variants)]
pub fn collect_link_label_text(
    children: &[tl::NodeHandle],
    parser: &tl::Parser,
    dom_ctx: &DomContext,
) -> (String, Vec<tl::NodeHandle>, bool) {
    let mut text = String::new();
    let mut saw_block = false;
    let mut block_nodes = Vec::new();
    let mut stack: Vec<_> = children.iter().rev().copied().collect();

    while let Some(handle) = stack.pop() {
        if let Some(node) = handle.get(parser) {
            match node {
                tl::Node::Raw(bytes) => {
                    let raw = bytes.as_utf8_str();
                    let decoded = text::decode_html_entities_cow(raw.as_ref());
                    text.push_str(decoded.as_ref());
                }
                tl::Node::Tag(tag) => {
                    let is_block = node_is_block_level(&handle, parser, dom_ctx);
                    if is_block {
                        saw_block = true;
                        block_nodes.push(handle);
                        continue;
                    }

                    if let Some(children) = dom_ctx.children_of(handle.get_inner()) {
                        for child in children.iter().rev() {
                            stack.push(*child);
                        }
                    } else {
                        let tag_children = tag.children();
                        let mut child_nodes: Vec<_> = tag_children.top().iter().copied().collect();
                        child_nodes.reverse();
                        stack.extend(child_nodes);
                    }
                }
                _ => {}
            }
        }
    }

    (text, block_nodes, saw_block)
}

/// The two hard-line-break markers `block/line_break.rs` can emit for a real `<br>`:
/// `"  \n"` for `NewlineStyle::Spaces`, `"\\\n"` for `NewlineStyle::Backslash`.
const HARD_BREAK_MARKERS: [&str; 2] = ["  \n", "\\\n"];

/// Normalize a link label by collapsing incidental newlines/whitespace while preserving an
/// explicit hard line break (`<br>`) that appears mid-label.
///
/// A hard line break inside a link's visible text is legal `CommonMark` (`[foo  \nbar](url)`),
/// so collapsing it unconditionally is lossy: convert to Markdown, render that back to HTML,
/// and convert again, and the `<br>` that survived the round trip disappears on the second
/// pass. Only the two exact marker shapes `block/line_break.rs` emits for a real `<br>` are
/// preserved; every other newline (soft line breaks from wrapped source text, `\r`) still
/// collapses to a single space, matching the previous behaviour.
///
/// ~keep This scans for the two marker substrings directly and copies everything else through
/// ~keep the ordinary whitespace-collapsing rules, rather than swapping the markers out for a
/// ~keep placeholder character and restoring them afterward. A placeholder scheme is unsound
/// ~keep here: it assumes an injective mapping over a character set the label cannot contain,
/// ~keep and that is false for arbitrary HTML input. An earlier version used Private Use Area
/// ~keep code points as placeholders on the reasoning that "no producer this crate parses
/// ~keep assigns them" -- but icon fonts do exactly that (Bootstrap 3's Glyphicons start at
/// ~keep U+E001), so a label already containing that literal character collided with the
/// ~keep placeholder and reappeared as a spurious hard break after "restoration". Because the
/// ~keep three marker bytes (space, backslash, `\n`) are pure ASCII, they can never occur as
/// ~keep part of a multi-byte UTF-8 sequence, so matching/splitting on them with plain byte
/// ~keep offsets (via `str::find`) is always on a char boundary -- no placeholder needed.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn normalize_link_label(label: &str) -> String {
    let mut segments: Vec<String> = Vec::new();
    let mut markers: Vec<&'static str> = Vec::new();
    let mut rest = label;

    while let Some((marker_pos, marker)) = find_earliest_hard_break_marker(rest) {
        let mut segment = String::new();
        collapse_whitespace_into(&mut segment, &rest[..marker_pos]);
        segments.push(segment);
        markers.push(marker);
        rest = &rest[marker_pos + marker.len()..];
    }
    let mut segment = String::new();
    collapse_whitespace_into(&mut segment, rest);
    segments.push(segment);

    assemble_label(segments, markers)
}

/// Re-join a label's whitespace-collapsed segments and the hard-break markers between them,
/// trimming the label's own outer whitespace without eating a break that sits at either end.
///
/// A `<br>` against the `</a>` is real content: `<a href="H">A<br></a>B` renders as A, a line
/// break, then B, and `[A  \n](H)B` re-parses to exactly that `<a href="H">A<br/></a>B`
/// (issue #497). It was previously dropped because the whole label was `str::trim`-ed, and a
/// `"  \n"` marker is indistinguishable from incidental trailing whitespace once flattened.
///
/// A run of breaks at either end still collapses to one. Two adjacent markers put a blank line
/// in the label, and a blank line ends the paragraph -- so `[  \n  \nA](H)` would destroy the
/// link rather than preserve a second break nobody can see anyway.
fn assemble_label(mut segments: Vec<String>, mut markers: Vec<&'static str>) -> String {
    if let Some(first) = segments.first_mut() {
        *first = first.trim_start().to_string();
    }
    if let Some(last) = segments.last_mut() {
        *last = last.trim_end().to_string();
    }

    let mut leading = "";
    while segments.len() > 1 && segments[0].is_empty() {
        segments.remove(0);
        leading = markers.remove(0);
    }
    let mut trailing = "";
    while segments.len() > 1 && segments.last().is_some_and(String::is_empty) {
        segments.pop();
        trailing = markers.pop().unwrap_or("");
    }

    // ~keep Nothing but breaks: a break needs a line on both sides to mean anything, so a
    // ~keep label of only `<br>` collapses to empty exactly as it did before.
    if segments.iter().all(String::is_empty) {
        return String::new();
    }

    let mut result = String::with_capacity(label_capacity(&segments, &markers, leading, trailing));
    result.push_str(leading);
    for (index, segment) in segments.iter().enumerate() {
        if index > 0 {
            result.push_str(markers.get(index - 1).copied().unwrap_or(""));
        }
        result.push_str(segment);
    }
    result.push_str(trailing);
    result
}

/// Exact byte length [`assemble_label`] is about to write.
fn label_capacity(segments: &[String], markers: &[&str], leading: &str, trailing: &str) -> usize {
    segments.iter().map(String::len).sum::<usize>()
        + markers.iter().map(|marker| marker.len()).sum::<usize>()
        + leading.len()
        + trailing.len()
}

/// Find the earliest occurrence of either hard-break marker in `text`, if any.
fn find_earliest_hard_break_marker(text: &str) -> Option<(usize, &'static str)> {
    HARD_BREAK_MARKERS
        .iter()
        .filter_map(|marker| text.find(marker).map(|pos| (pos, *marker)))
        .min_by_key(|(pos, _)| *pos)
}

/// Fold newlines to a space and collapse whitespace runs in a marker-free segment, appending
/// the result to `out`. Mirrors the whitespace handling `normalize_link_label` has always
/// applied outside of a hard-break marker.
fn collapse_whitespace_into(out: &mut String, segment: &str) {
    if segment.is_empty() {
        return;
    }

    let folded: Cow<'_, str> = if segment.contains(['\n', '\r']) {
        Cow::Owned(segment.replace(['\n', '\r'], " "))
    } else {
        Cow::Borrowed(segment)
    };

    out.push_str(text::normalize_whitespace_cow(folded.as_ref()).as_ref());
}

/// Normalize a tag name to lowercase, preserving borrowed input when possible.
pub fn normalized_tag_name(raw: Cow<'_, str>) -> Cow<'_, str> {
    if raw.as_bytes().iter().any(u8::is_ascii_uppercase) {
        let mut owned = raw.into_owned();
        owned.make_ascii_lowercase();
        Cow::Owned(owned)
    } else {
        raw
    }
}

/// Check if an element is block-level (not inline).
pub fn is_block_level_element(tag_name: &str) -> bool {
    is_block_level_name(tag_name, crate::converter::main_helpers::is_inline_element(tag_name))
}

/// Returns the largest valid char boundary index at or before `index`.
///
/// If `index` is already a char boundary it is returned unchanged.
/// Otherwise it walks backwards to find one.  Returns 0 if no boundary
/// is found before `index`.
pub const fn floor_char_boundary(s: &str, index: usize) -> usize {
    if index >= s.len() {
        s.len()
    } else {
        let mut i = index;
        while i > 0 && !s.is_char_boundary(i) {
            i -= 1;
        }
        i
    }
}

/// Escape a link label or image alt text so it cannot break out of the `[...]` / `![...]`
/// it is about to be wrapped in.
///
/// Two independent escapes, in order:
///
/// 1. [`escape_label_brackets`] -- a `]` with no local opener, or a matched pair that is
///    itself link- or reference-link-shaped.
/// 2. [`escape_block_openers_on_continuation_lines`] -- a `CommonMark` block-structure
///    marker opening a line after the first (issue #496).
///
/// Both are unconditional, unlike the `escape_misc`/`escape_asterisks` family: those decide
/// whether text that merely *looks* like Markdown is emitted verbatim, whereas these two
/// decide whether the link or image survives at all.
pub fn escape_link_label(text: &str) -> Cow<'_, str> {
    match escape_label_brackets(text) {
        Cow::Borrowed(bracketed) => escape_block_openers_on_continuation_lines(bracketed),
        Cow::Owned(bracketed) => Cow::Owned(escape_block_openers_on_continuation_lines(&bracketed).into_owned()),
    }
}

/// Escape the brackets in a link label or image alt text that would otherwise terminate it.
///
/// One of the two halves of [`escape_link_label`]; see there for the other.
/// Tracks matched bracket pairs and escapes a closing bracket that has no local opener
/// (it would otherwise close the caller's own wrapping `[`/`![` early), an opening
/// bracket that is never matched by a later closer (it would otherwise open a
/// link/image span that swallows the caller's own closing `]`/`)` on reparse), and
/// escapes a matched pair outright when it is itself link- or reference-link-shaped.
///
/// # Examples
/// ```text
/// Input:  "]"
/// Output: "\\]"
///
/// Input:  "[A;B)"
/// Output: "\\[A;B)"
///
/// Input:  "[outer [inner]]"
/// Output: "[outer [inner]]"
///
/// Input:  "[foo](uri2)"
/// Output: "\\[foo\\](uri2)"
/// ```
///
/// Returns `Cow::Borrowed` when `text` contains neither `[` nor `]` (escaping is then
/// necessarily a no-op), or `Cow::Owned` with the escaped text otherwise.
fn escape_label_brackets(text: &str) -> Cow<'_, str> {
    if text.is_empty() {
        return Cow::Borrowed("");
    }

    // ~keep Escapes are only ever inserted at `[`/`]` byte positions (see the match below),
    // ~keep so when neither appears the two-pass logic further down is guaranteed to be a
    // ~keep no-op -- skip both its allocations (the `escape_at` vec and the output String).
    if memchr::memchr2(b'[', b']', text.as_bytes()).is_none() {
        return Cow::Borrowed(text);
    }

    // ~keep Two linear passes rather than one pass that mutates the output string as it
    // ~keep goes: escaping a matched bracket pair needs to touch the *opener*, which by
    // ~keep the time its `]` is found has already been written. Retroactively
    // ~keep `String::insert`-ing it back in is O(remaining length) per escape, so a label
    // ~keep built of many link-shaped pairs (`[a](b)[a](b)...`) would make the whole
    // ~keep function O(n^2) -- the same denial-of-service shape `bare_lt_complexity.rs`
    // ~keep exists to catch elsewhere in this crate. Precomputing which byte offsets need
    // ~keep an escape first keeps the second pass a single linear append.
    let mut escape_at = vec![false; text.len()];
    // ~keep Byte offsets (into `text`) of unescaped `[` openers not yet matched by a `]`,
    // ~keep innermost last. A bare depth counter cannot tell "is there a local opener"
    // ~keep from "which one", and the `](`/`][` check below needs the specific opener.
    let mut open_positions: Vec<usize> = Vec::new();
    let mut backslash_count = 0usize;
    let mut chars = text.char_indices().peekable();

    while let Some((byte_pos, ch)) = chars.next() {
        if ch == '\\' {
            backslash_count += 1;
            continue;
        }

        let is_escaped = backslash_count % 2 == 1;
        backslash_count = 0;

        match ch {
            '[' if !is_escaped => open_positions.push(byte_pos),
            ']' if !is_escaped => match open_positions.pop() {
                None => {
                    // ~keep No local opener: left alone, this `]` closes the caller's own
                    // ~keep wrapping `[`/`![` early, truncating the label.
                    escape_at[byte_pos] = true;
                }
                Some(open_pos) => {
                    // ~keep A `]` immediately followed by `(` or `[` completes an inline
                    // ~keep link/image (`](dest)`) or a reference link (`][ref]`) on
                    // ~keep reparse: `CommonMark` parses link/image label content as full
                    // ~keep inline markdown, so a literal `[foo](uri2)` inside it silently
                    // ~keep becomes a real nested link and the destination is lost --
                    // ~keep escaping only this closer is not enough and is actively worse:
                    // ~keep the still-open outer `[`/`![` is then free to be captured by a
                    // ~keep *later* unescaped `]` instead (verified against comrak), so the
                    // ~keep image/link fails to form at all. Escaping both ends of the
                    // ~keep matched pair together is the only shape that round-trips.
                    //
                    // ~keep Exempted when the opener is itself preceded by `!`: that is a
                    // ~keep real, intentional `![alt](src)` this converter already emitted
                    // ~keep while walking a nested `<img>` inside link text (CommonMark
                    // ~keep permits images, just not links, inside link text), not literal
                    // ~keep text that merely looks link-shaped -- escaping it would corrupt
                    // ~keep the nested image instead of protecting the label (caught by
                    // ~keep `test_commonmark_compliance`'s `[![moon](moon.jpg)](/uri)`).
                    let is_link_shaped = matches!(chars.peek(), Some((_, '(' | '[')));
                    let is_nested_image = open_pos > 0 && text.as_bytes()[open_pos - 1] == b'!';
                    if is_link_shaped && !is_nested_image {
                        escape_at[open_pos] = true;
                        escape_at[byte_pos] = true;
                    }
                }
            },
            _ => {}
        }
    }

    // ~keep Whatever is left in `open_positions` once the scan ends is a `[` with no
    // ~keep matching `]` anywhere in the label. Left alone, it starts a link/image span
    // ~keep that CommonMark's inline parser will happily extend into the *caller's own*
    // ~keep closing `]`/`)` on reparse -- the exact failure in issue #499, where
    // ~keep `![[A;B)](S)` reparses as dangling `!` text followed by a real
    // ~keep `[A;B)](S)` link, silently dropping the image. Escaping every such opener
    // ~keep is the mirror of the unmatched-`]` case above.
    for open_pos in open_positions {
        escape_at[open_pos] = true;
    }

    let mut result = String::with_capacity(text.len() + 2);
    for (byte_pos, ch) in text.char_indices() {
        if escape_at[byte_pos] {
            result.push('\\');
        }
        result.push(ch);
    }

    Cow::Owned(result)
}

/// `CommonMark` HTML-block start conditions of type 1: raw-text elements, whose *opening*
/// tag alone on a line starts a block. Their closing tags do not.
const HTML_BLOCK_RAW_TEXT_TAGS: [&str; 4] = ["pre", "script", "style", "textarea"];

/// `CommonMark` HTML-block start conditions of type 6 (spec 0.31.2): either an opening or a
/// closing tag with one of these names starts a block, and type 6 -- unlike type 7 -- may
/// interrupt a paragraph.
const HTML_BLOCK_TAGS: [&str; 62] = [
    "address",
    "article",
    "aside",
    "base",
    "basefont",
    "blockquote",
    "body",
    "caption",
    "center",
    "col",
    "colgroup",
    "dd",
    "details",
    "dialog",
    "dir",
    "div",
    "dl",
    "dt",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "frame",
    "frameset",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hr",
    "html",
    "iframe",
    "legend",
    "li",
    "link",
    "main",
    "menu",
    "menuitem",
    "nav",
    "noframes",
    "ol",
    "optgroup",
    "option",
    "p",
    "param",
    "search",
    "section",
    "summary",
    "table",
    "tbody",
    "td",
    "tfoot",
    "th",
    "thead",
    "title",
    "tr",
    "track",
    "ul",
];

/// Escape a `CommonMark` block-structure marker that opens a *continuation* line of a link
/// label or image alt text.
///
/// Block structure is parsed before inline structure (spec appendix A), so a line inside a
/// multi-line label that reads as a block opener terminates the paragraph the label lives in
/// and the `[` / `![` never pairs with its `]`. `![A\n-\n](S)` therefore produces no image at
/// all -- it produces `<h2>![A</h2><p>](S)</p>`, with the image simply gone (issue #496).
/// A hard line break does not protect the next line either: hard breaks are inline, and block
/// parsing has already finished by the time they are looked at.
///
/// Only continuation lines are examined. A label's first line is preceded on that same line by
/// the caller's `[` / `![`, so it cannot start a block however it begins.
///
/// Returns `Cow::Borrowed` when `text` is single-line or no continuation line opens a block.
fn escape_block_openers_on_continuation_lines(text: &str) -> Cow<'_, str> {
    if !text.contains('\n') {
        return Cow::Borrowed(text);
    }

    let mut escape_at: Vec<usize> = Vec::new();
    let mut line_start = 0usize;
    for (index, line) in text.split('\n').enumerate() {
        if index > 0 {
            if let Some(offset) = block_opener_escape_offset(line) {
                escape_at.push(line_start + offset);
            }
        }
        line_start += line.len() + 1;
    }

    if escape_at.is_empty() {
        return Cow::Borrowed(text);
    }

    let mut result = String::with_capacity(text.len() + escape_at.len());
    let mut copied = 0usize;
    for position in escape_at {
        result.push_str(&text[copied..position]);
        result.push('\\');
        copied = position;
    }
    result.push_str(&text[copied..]);
    Cow::Owned(result)
}

/// Byte offset within `line` of the character to backslash-escape so the line stops opening a
/// block, or `None` when the line opens no block that can interrupt a paragraph.
fn block_opener_escape_offset(line: &str) -> Option<usize> {
    let (indent, column) = leading_indent(line);
    // ~keep Four columns of indent is an indented code block, and an indented code block
    // ~keep cannot interrupt a paragraph -- such a line is already inert.
    if column >= 4 {
        return None;
    }
    let rest = line.get(indent..)?;
    let marker = *rest.as_bytes().first()?;
    let opens_block = match marker {
        b'>' => true,
        b'#' => is_atx_heading(rest),
        b'`' | b'~' => is_code_fence(rest, marker),
        b'=' => is_setext_underline(rest, b'='),
        b'-' => is_setext_underline(rest, b'-') || is_thematic_break(rest, b'-') || is_bullet_list_item(rest),
        b'*' => is_thematic_break(rest, b'*') || is_bullet_list_item(rest),
        b'+' => is_bullet_list_item(rest),
        b'_' => is_thematic_break(rest, b'_'),
        b'<' => is_html_block_opener(rest),
        // ~keep A digit cannot carry a backslash escape, so an ordered-list marker is
        // ~keep defused at its `.`/`)` delimiter instead of at its number.
        b'0'..=b'9' => return ordered_list_delimiter_offset(rest).map(|offset| indent + offset),
        _ => false,
    };
    opens_block.then_some(indent)
}

/// Split `line`'s leading indentation, returning `(byte length, column width)`.
fn leading_indent(line: &str) -> (usize, usize) {
    let mut length = 0usize;
    let mut column = 0usize;
    for &byte in line.as_bytes() {
        match byte {
            b' ' => column += 1,
            // ~keep A tab advances to the next multiple of four (spec section 2.2), so a
            // ~keep single leading tab is already four columns of indent.
            b'\t' => column += 4 - column % 4,
            _ => break,
        }
        length += 1;
    }
    (length, column)
}

/// An ATX heading: one to six `#`, then a space/tab or the end of the line.
fn is_atx_heading(rest: &str) -> bool {
    let hashes = rest.bytes().take_while(|&byte| byte == b'#').count();
    (1..=6).contains(&hashes) && matches!(rest.as_bytes().get(hashes), None | Some(b' ' | b'\t' | b'\r'))
}

/// An opening code fence: three or more of the same fence character.
fn is_code_fence(rest: &str, fence: u8) -> bool {
    let run = rest.bytes().take_while(|&byte| byte == fence).count();
    // ~keep A backtick fence's info string may not itself contain a backtick, so such a line
    // ~keep is ordinary text and needs no escape.
    run >= 3 && (fence != b'`' || !rest[run..].contains('`'))
}

/// A setext heading underline: nothing but `marker`, plus optional trailing whitespace.
fn is_setext_underline(rest: &str, marker: u8) -> bool {
    let trimmed = rest.trim_end();
    !trimmed.is_empty() && trimmed.bytes().all(|byte| byte == marker)
}

/// A thematic break: three or more `marker` characters and nothing else but spaces/tabs.
fn is_thematic_break(rest: &str, marker: u8) -> bool {
    let mut count = 0usize;
    for byte in rest.bytes() {
        if byte == marker {
            count += 1;
        } else if !matches!(byte, b' ' | b'\t' | b'\r') {
            return false;
        }
    }
    count >= 3
}

/// A bullet list item that can interrupt a paragraph: a marker, a space/tab, then content.
fn is_bullet_list_item(rest: &str) -> bool {
    // ~keep An empty list item cannot interrupt a paragraph (spec section 5.2), so both the
    // ~keep separating space/tab and non-blank content after it are required.
    matches!(rest.as_bytes().get(1), Some(b' ' | b'\t')) && rest.get(2..).is_some_and(|tail| !tail.trim().is_empty())
}

/// Byte offset of the `.`/`)` of an ordered list marker that can interrupt a paragraph.
fn ordered_list_delimiter_offset(rest: &str) -> Option<usize> {
    // ~keep Only a list starting at 1 can interrupt a paragraph (spec section 5.2).
    let bytes = rest.as_bytes();
    let starts_a_list = bytes.first() == Some(&b'1')
        && matches!(bytes.get(1), Some(b'.' | b')'))
        && matches!(bytes.get(2), Some(b' ' | b'\t'))
        && rest.get(3..).is_some_and(|tail| !tail.trim().is_empty());
    starts_a_list.then_some(1)
}

/// An HTML block of type 1 to 6 -- the types that may interrupt a paragraph.
///
/// Type 7 (any other complete open tag alone on its line) deliberately may not, so inline
/// markup such as a `<span>` opening a continuation line is left alone.
fn is_html_block_opener(rest: &str) -> bool {
    let after_bracket = &rest[1..];
    // ~keep Types 2-5: comment, processing instruction, declaration, CDATA.
    if after_bracket.starts_with("!--")
        || after_bracket.starts_with('?')
        || after_bracket.starts_with("![CDATA[")
        || (after_bracket.starts_with('!') && after_bracket.as_bytes().get(1).is_some_and(u8::is_ascii_alphabetic))
    {
        return true;
    }

    let is_closing = after_bracket.starts_with('/');
    let name_start = usize::from(is_closing);
    let name: String = after_bracket[name_start..]
        .bytes()
        .take_while(u8::is_ascii_alphanumeric)
        .map(|byte| byte.to_ascii_lowercase() as char)
        .collect();
    if name.is_empty() {
        return false;
    }

    // ~keep Both start conditions require the tag name to end at whitespace, `>`, `/>` or the
    // ~keep end of the line; anything else (`<divx`, `<div=`) is not a tag at all.
    let tail = &after_bracket[name_start + name.len()..];
    let is_terminated =
        tail.is_empty() || tail.starts_with('>') || tail.starts_with("/>") || tail.starts_with(char::is_whitespace);

    is_terminated
        && (HTML_BLOCK_TAGS.contains(&name.as_str())
            || (!is_closing && HTML_BLOCK_RAW_TEXT_TAGS.contains(&name.as_str())))
}

/// Helper for block-level element detection.
pub fn is_block_level_name(tag_name: &str, is_inline: bool) -> bool {
    !is_inline
        && matches!(
            tag_name,
            "address"
                | "article"
                | "aside"
                | "blockquote"
                | "canvas"
                | "dd"
                | "div"
                | "dl"
                | "dt"
                | "fieldset"
                | "figcaption"
                | "figure"
                | "footer"
                | "form"
                | "h1"
                | "h2"
                | "h3"
                | "h4"
                | "h5"
                | "h6"
                | "header"
                | "hr"
                | "li"
                | "main"
                | "nav"
                | "ol"
                | "p"
                | "pre"
                | "section"
                | "table"
                | "tfoot"
                | "ul"
        )
}

/// Escape any bare pipe left in a nested table's rendered markdown: one that is neither
/// already backslash-escaped nor inside a matched backtick code span (a `CommonMark`-
/// compliant reparse does not treat either as a cell delimiter, so this must not touch
/// them either).
///
/// Scoped to a nested `<table>`'s own rendered text (see the call site in
/// [`render_cell_text`]) rather than applied to a whole cell's composed text: other block
/// content a cell may hold, such as `<pre>`, is deliberately left byte-for-byte alone by
/// its own handler (issues #455/#456) and must not be escaped here.
///
/// Walks backtick runs the same way a spec-compliant parser does: a run of N backticks
/// opens a code span only if a later run of exactly N backticks closes it; otherwise the
/// backticks are literal text and any pipes among them still need escaping.
pub fn escape_bare_pipes_outside_code_spans(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len() + 4);
    let mut i = 0usize;
    while i < chars.len() {
        let c = chars[i];
        if c == '\\' && i + 1 < chars.len() {
            out.push(c);
            out.push(chars[i + 1]);
            i += 2;
            continue;
        }
        if c == '`' {
            let run_start = i;
            while i < chars.len() && chars[i] == '`' {
                i += 1;
            }
            let run_len = i - run_start;
            if let Some(close_start) = find_matching_backtick_run(&chars, i, run_len) {
                out.extend(&chars[run_start..close_start + run_len]);
                i = close_start + run_len;
            } else {
                out.extend(&chars[run_start..i]);
            }
            continue;
        }
        if c == '|' {
            out.push('\\');
            out.push('|');
        } else {
            out.push(c);
        }
        i += 1;
    }
    out
}

// ~keep Lives here, shared, rather than beside either caller. Both tiers must escape a
// ~keep flattened nested table identically or the output forks: unescaped, these pipes are
// ~keep read as the OUTER row's cell delimiters on reparse, and GFM truncates the row to the
// ~keep header's column count, dropping the inner cells outright. That is content loss, and a
// ~keep silently drifting second copy would reintroduce it on whichever tier fell behind.

/// Find the start index of the next backtick run of exactly `run_len` backticks at or
/// after `start`, treating a longer or shorter run as not matching (mirroring `CommonMark`
/// code span matching, which requires an exact backtick-count match).
pub fn find_matching_backtick_run(chars: &[char], start: usize, run_len: usize) -> Option<usize> {
    let mut i = start;
    while i < chars.len() {
        if chars[i] == '`' {
            let candidate_start = i;
            while i < chars.len() && chars[i] == '`' {
                i += 1;
            }
            if i - candidate_start == run_len {
                return Some(candidate_start);
            }
        } else {
            i += 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_link_label_leaves_plain_text_unchanged() {
        assert_eq!(escape_link_label("plain text"), "plain text");
    }

    #[test]
    fn escape_link_label_escapes_an_unmatched_closing_bracket() {
        assert_eq!(escape_link_label("]"), "\\]");
    }

    // ~keep A `[...]` pair with a local opener and no trailing `(`/`[` is left unescaped
    // ~keep even though it superficially resembles the unmatched-bracket case above: the
    // ~keep opener/closer here match each other, so unlike a bare `]` they do not close
    // ~keep the caller's own wrapping bracket early.
    #[test]
    fn escape_link_label_leaves_a_matched_non_link_shaped_pair_unchanged() {
        assert_eq!(escape_link_label("[link]"), "[link]");
    }

    #[test]
    fn escape_link_label_leaves_balanced_nested_brackets_unchanged() {
        assert_eq!(escape_link_label("[outer [inner]]"), "[outer [inner]]");
    }

    // ~keep Regression for Cluster B (image alt text losing its nested destination):
    // ~keep `<img alt="[foo](uri2)">` must not let the alt text's own `[foo](uri2)` be
    // ~keep reparsed as a real nested link, or `uri2` is silently dropped on a second
    // ~keep conversion pass (CommonMark parses an image's alt as full inline content).
    #[test]
    fn escape_link_label_escapes_a_link_shaped_bracket_pair() {
        assert_eq!(escape_link_label("[foo](uri2)"), "\\[foo\\](uri2)");
    }

    // ~keep Same hazard, reference-link form: `[foo][ref]` is just as reparsable as
    // ~keep `[foo](uri2)`.
    #[test]
    fn escape_link_label_escapes_a_reference_link_shaped_bracket_pair() {
        assert_eq!(escape_link_label("[foo][ref]"), "\\[foo\\][ref]");
    }

    // ~keep A `[...]` NOT immediately followed by `(` or `[` cannot complete a link on
    // ~keep reparse, so it is left alone -- this is the exact shape the task's proposed
    // ~keep narrower rule ("escape `]` only before `(`") would also leave alone, but this
    // ~keep also confirms plain non-link-shaped bracket text is unaffected.
    #[test]
    fn escape_link_label_leaves_non_link_shaped_brackets_unchanged() {
        assert_eq!(escape_link_label("see [note] here"), "see [note] here");
    }

    // ~keep Regression: a link whose *label* is a real, intentionally-emitted nested
    // ~keep image (`![moon](moon.jpg)`, CommonMark permits images -- just not links --
    // ~keep inside link text) must not be escaped merely because it is link-shaped: it
    // ~keep is not literal text, it is markdown this converter already produced while
    // ~keep walking a nested `<img>`. Caught by `commonmark_compliance_test`'s example
    // ~keep 517 (`[![moon](moon.jpg)](/uri)`) before this exemption was added.
    #[test]
    fn escape_link_label_does_not_escape_a_nested_image() {
        assert_eq!(escape_link_label("![moon](moon.jpg)"), "![moon](moon.jpg)");
    }

    // ~keep A link-shaped bracket pair still nested inside an outer, non-link-shaped
    // ~keep bracket pair: only the inner (dangerous) pair is escaped, and the escape is
    // ~keep inserted at the correct byte offset for the *matching* opener, not just
    // ~keep prepended to the whole string.
    #[test]
    fn escape_link_label_escapes_only_the_link_shaped_inner_pair() {
        assert_eq!(escape_link_label("[a[b](c)]"), "[a\\[b\\](c)]");
    }

    // ~keep Regression for issue #499: an unmatched `[` is the mirror hazard of the
    // ~keep unmatched-`]` case above -- left alone, `![[A;B)](S)` reparses as dangling
    // ~keep `!` text followed by a real `[A;B)](S)` link, silently dropping the image.
    #[test]
    fn escape_link_label_escapes_an_unmatched_opening_bracket() {
        assert_eq!(escape_link_label("[A;B)"), "\\[A;B)");
    }

    #[test]
    fn escape_link_label_escapes_an_unmatched_opening_bracket_before_plain_text() {
        assert_eq!(escape_link_label("[a"), "\\[a");
    }

    // ~keep The outer `[` is unmatched and must be escaped; the inner `[a]` is a
    // ~keep matched, non-link-shaped pair and stays untouched -- the two rules apply
    // ~keep independently at their own byte offsets.
    #[test]
    fn escape_link_label_escapes_an_unmatched_outer_bracket_around_a_matched_inner_pair() {
        assert_eq!(escape_link_label("[[a]"), "\\[[a]");
    }

    #[test]
    fn escape_link_label_escapes_an_unmatched_opening_bracket_after_plain_text() {
        assert_eq!(escape_link_label("a[b"), "a\\[b");
    }

    #[test]
    fn escape_link_label_leaves_an_already_escaped_opening_bracket_unchanged() {
        assert_eq!(escape_link_label("\\[a"), "\\[a");
    }

    // ~keep Regression for CommonMark spec examples 642/643: a `<br>`-produced hard
    // ~keep line break (`"  \n"`, matching `NewlineStyle::Spaces`) mid-label must
    // ~keep survive, not collapse to a plain space.
    #[test]
    fn normalize_link_label_preserves_a_mid_label_spaces_style_hard_break() {
        assert_eq!(normalize_link_label("foo  \nbar"), "foo  \nbar");
    }

    #[test]
    fn normalize_link_label_preserves_a_mid_label_backslash_style_hard_break() {
        assert_eq!(normalize_link_label("foo\\\nbar"), "foo\\\nbar");
    }

    // ~keep Issue #497: a break at the label's edge is real content, not incidental
    // ~keep whitespace. `<a href="H">A<br></a>B` renders as A, a line break, then B, and
    // ~keep `[A  \n](H)B` re-parses to exactly that -- verified against comrak. These two
    // ~keep previously asserted the opposite (the break dropped), which is where the bug
    // ~keep lived: the whole label was `str::trim`-ed, and a flattened `"  \n"` is
    // ~keep indistinguishable from trailing source whitespace before a `</a>`.
    #[test]
    fn normalize_link_label_keeps_a_leading_hard_break() {
        assert_eq!(normalize_link_label("  \nbar"), "  \nbar");
    }

    #[test]
    fn normalize_link_label_keeps_a_trailing_hard_break() {
        assert_eq!(normalize_link_label("foo  \n"), "foo  \n");
    }

    #[test]
    fn normalize_link_label_keeps_a_boundary_hard_break_past_incidental_whitespace() {
        assert_eq!(normalize_link_label(" \u{a0}foo  \n "), "foo  \n");
    }

    // ~keep A run of breaks at one edge collapses to a single break: two adjacent markers
    // ~keep put a blank line in the label, and a blank line ends the paragraph the link
    // ~keep lives in -- destroying the link rather than preserving a second break nobody
    // ~keep can see.
    #[test]
    fn normalize_link_label_collapses_a_run_of_boundary_hard_breaks_to_one() {
        assert_eq!(normalize_link_label("  \n  \nbar"), "  \nbar");
        assert_eq!(normalize_link_label("foo  \n  \n"), "foo  \n");
    }

    // ~keep A break needs a line on both sides to mean anything, so a label of nothing but
    // ~keep breaks still collapses to empty and the caller's own href fallback takes over.
    #[test]
    fn normalize_link_label_drops_a_label_that_is_only_hard_breaks() {
        assert_eq!(normalize_link_label("  \n"), "");
        assert_eq!(normalize_link_label("  \n  \n"), "");
    }

    // ~keep An ordinary soft newline (no `<br>` behind it, e.g. wrapped source text)
    // ~keep still collapses to a single space -- only the two exact hard-break marker
    // ~keep shapes are preserved.
    #[test]
    fn normalize_link_label_still_collapses_an_incidental_newline_to_a_space() {
        assert_eq!(normalize_link_label("foo\nbar"), "foo bar");
        assert_eq!(normalize_link_label("foo \n bar"), "foo bar");
    }

    #[test]
    fn normalize_link_label_still_collapses_ordinary_whitespace_runs() {
        assert_eq!(normalize_link_label("foo   bar"), "foo bar");
        assert_eq!(normalize_link_label("  foo bar  "), "foo bar");
    }

    // ~keep Regression for a real collision in an earlier version of this function: it used
    // ~keep Private Use Area code points (U+E000/U+E001) as placeholders for the hard-break
    // ~keep markers, reasoning that no producer this crate parses assigns them. That is false
    // ~keep -- icon fonts live in the PUA (Bootstrap 3's Glyphicons start at U+E001) -- so a
    // ~keep label already containing that literal character collided with the placeholder and
    // ~keep reappeared as a spurious hard break once the placeholder was "restored". The
    // ~keep trigger needs both a literal PUA character AND a real hard-break marker in the
    // ~keep same label -- a PUA character alone never entered the placeholder-substitution
    // ~keep branch at all, which is why this was not caught by the other tests above.
    #[test]
    fn normalize_link_label_does_not_confuse_a_literal_pua_character_with_the_spaces_sentinel() {
        assert_eq!(normalize_link_label("a\u{E000}b  \nc"), "a\u{E000}b  \nc");
    }

    #[test]
    fn normalize_link_label_does_not_confuse_a_literal_pua_character_with_the_backslash_sentinel() {
        assert_eq!(normalize_link_label("a\u{E001}b\\\nc"), "a\u{E001}b\\\nc");
    }

    // ~keep The Glyphicon code point itself (U+E001) is exactly the second placeholder this
    // ~keep function used to use, so this pins the specific real-world icon-font byte, not
    // ~keep just "some" PUA character.
    #[test]
    fn normalize_link_label_preserves_a_glyphicon_code_point_alongside_a_spaces_style_hard_break() {
        assert_eq!(normalize_link_label("\u{E001} foo  \nbar"), "\u{E001} foo  \nbar");
    }

    #[test]
    fn normalize_link_label_preserves_a_glyphicon_code_point_alongside_a_backslash_style_hard_break() {
        assert_eq!(normalize_link_label("\u{E001} foo\\\nbar"), "\u{E001} foo\\\nbar");
    }

    // ~keep ── chomp_inline whitespace-only dedup (issue #481) ──────────────────────────

    #[test]
    fn chomp_inline_returns_a_single_space_prefix_for_whitespace_only_content() {
        assert_eq!(chomp_inline(" "), (" ", "", ""));
    }

    #[test]
    fn chomp_inline_returns_a_single_space_prefix_for_a_multi_char_whitespace_only_run() {
        assert_eq!(chomp_inline("   "), (" ", "", ""));
        assert_eq!(chomp_inline("\t "), (" ", "", ""));
    }

    #[test]
    fn chomp_inline_preserves_a_hard_trailing_linebreak_over_the_whitespace_only_collapse() {
        assert_eq!(chomp_inline("  \n"), (" ", "  \n", ""));
        assert_eq!(chomp_inline("\\\n"), ("", "\\\n", ""));
    }

    #[test]
    fn chomp_inline_leaves_non_whitespace_content_unaffected() {
        assert_eq!(chomp_inline(" a "), (" ", " ", "a"));
        assert_eq!(chomp_inline("a"), ("", "", "a"));
    }

    #[test]
    fn chomp_inline_returns_empty_for_empty_input() {
        assert_eq!(chomp_inline(""), ("", "", ""));
    }

    // ~keep ── merge_adjacent_emphasis (issue #483) ─────────────────────────────────────

    #[test]
    fn merge_adjacent_emphasis_pops_a_lone_matching_close_marker() {
        let mut output = String::from("*A*");
        assert!(merge_adjacent_emphasis(&mut output, '*', 1));
        assert_eq!(output, "*A");
    }

    #[test]
    fn merge_adjacent_emphasis_pops_a_double_matching_close_marker() {
        let mut output = String::from("**A**");
        assert!(merge_adjacent_emphasis(&mut output, '*', 2));
        assert_eq!(output, "**A");
    }

    #[test]
    fn merge_adjacent_emphasis_returns_false_when_output_does_not_end_with_the_marker() {
        let mut output = String::from("*A* ");
        assert!(!merge_adjacent_emphasis(&mut output, '*', 1));
        assert_eq!(output, "*A* ", "untouched on refusal");
    }

    #[test]
    fn merge_adjacent_emphasis_refuses_a_run_longer_than_count() {
        // ~keep `***x***` must not be half-eaten: a trailing run of 3 is not "exactly 2".
        let mut output = String::from("***x***");
        assert!(!merge_adjacent_emphasis(&mut output, '*', 2));
        assert_eq!(output, "***x***");
    }

    #[test]
    fn merge_adjacent_emphasis_refuses_when_the_run_is_shorter_than_count() {
        let mut output = String::from("*A*");
        assert!(!merge_adjacent_emphasis(&mut output, '*', 2));
        assert_eq!(output, "*A*");
    }

    #[test]
    fn merge_adjacent_emphasis_refuses_an_escaped_literal_asterisk() {
        // ~keep `\*` immediately before the run: the preceding character is a backslash,
        // ~keep so this is a literal escaped asterisk, not a mergeable close marker.
        let mut output = String::from(r"a\*");
        assert!(!merge_adjacent_emphasis(&mut output, '*', 1));
        assert_eq!(output, r"a\*");
    }

    #[test]
    fn merge_adjacent_emphasis_returns_false_for_zero_count() {
        let mut output = String::from("*A*");
        assert!(!merge_adjacent_emphasis(&mut output, '*', 0));
        assert_eq!(output, "*A*");
    }

    #[test]
    fn merge_adjacent_emphasis_respects_the_underscore_symbol_variant() {
        let mut output = String::from("__A__");
        assert!(merge_adjacent_emphasis(&mut output, '_', 2));
        assert_eq!(output, "__A");
    }

    #[test]
    fn merge_adjacent_emphasis_returns_false_on_empty_output() {
        let mut output = String::new();
        assert!(!merge_adjacent_emphasis(&mut output, '*', 1));
        assert_eq!(output, "");
    }
}
