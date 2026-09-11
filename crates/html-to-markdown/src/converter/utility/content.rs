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
        // ~keep plain space.
        let prefix = if text.contains([' ', '\t']) { " " } else { "" };
        return (prefix, "", "");
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
                    let is_block = dom_ctx.tag_info(handle.get_inner(), parser).map_or_else(
                        || {
                            let tag_name = normalized_tag_name(tag.name().as_utf8_str());
                            is_block_level_element(tag_name.as_ref())
                        },
                        |info| info.is_block,
                    );
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
    let mut result = String::with_capacity(label.len());
    let mut rest = label;

    while let Some((marker_pos, marker)) = find_earliest_hard_break_marker(rest) {
        collapse_whitespace_into(&mut result, &rest[..marker_pos]);
        result.push_str(marker);
        rest = &rest[marker_pos + marker.len()..];
    }
    collapse_whitespace_into(&mut result, rest);

    drop_boundary_hard_breaks(result.trim()).to_string()
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

/// Drop a hard-break marker that ends up at the label's very start or end -- it has no
/// preceding/following line to break to/from, so (matching the pre-existing behaviour of
/// collapsing such a break down to nothing) it is removed rather than kept.
fn drop_boundary_hard_breaks(mut text: &str) -> &str {
    loop {
        let without_leading = HARD_BREAK_MARKERS
            .iter()
            .find_map(|marker| text.strip_prefix(marker))
            .map(str::trim_start);
        let without_trailing = HARD_BREAK_MARKERS
            .iter()
            .find_map(|marker| text.strip_suffix(marker))
            .map(str::trim_end);

        let next = without_leading.or(without_trailing);
        match next {
            Some(stripped) if stripped != text => text = stripped,
            _ => return text,
        }
    }
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

/// Escape special Markdown characters in a link label or image alt text.
///
/// Handles bracket escaping to prevent unintended link label termination.
/// Tracks matched bracket pairs and escapes a closing bracket that has no local opener
/// (it would otherwise close the caller's own wrapping `[`/`![` early), and escapes a
/// matched pair outright when it is itself link- or reference-link-shaped.
///
/// # Examples
/// ```text
/// Input:  "]"
/// Output: "\\]"
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
pub fn escape_link_label(text: &str) -> Cow<'_, str> {
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

    let mut result = String::with_capacity(text.len() + 2);
    for (byte_pos, ch) in text.char_indices() {
        if escape_at[byte_pos] {
            result.push('\\');
        }
        result.push(ch);
    }

    Cow::Owned(result)
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

    // ~keep A hard break with nothing before/after it has no line to break to or
    // ~keep from, so it is dropped entirely -- matching the pre-existing behaviour of
    // ~keep trimming a leading/trailing break down to nothing, not just collapsing it
    // ~keep to a space.
    #[test]
    fn normalize_link_label_drops_a_leading_hard_break() {
        assert_eq!(normalize_link_label("  \nbar"), "bar");
    }

    #[test]
    fn normalize_link_label_drops_a_trailing_hard_break() {
        assert_eq!(normalize_link_label("foo  \n"), "foo");
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
