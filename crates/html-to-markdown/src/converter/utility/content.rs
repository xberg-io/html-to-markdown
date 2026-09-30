//! Content extraction and manipulation utilities.
//!
//! Functions for extracting and processing element content, including text collection
//! and empty element detection.

use crate::converter::utility::escaping::is_block_level_name;
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

    let (trimmed, suffix) = drop_trailing_break_backslashes(text, trimmed, suffix);
    (prefix, suffix, trimmed)
}

/// Move each backslash break at the end of `trimmed`, a slice of `text`, into the suffix.
///
/// ~keep A break followed by a space or a second break (`<b>a<br> </b>`) leaves its backslash
/// ~keep at the end of the trimmed body once the whitespace after it is gone, and a backslash
/// ~keep right before the closing marker escapes it: `*a\*` shows the asterisks. A backslash
/// ~keep is a break only when a line end follows it in `text`.
fn drop_trailing_break_backslashes<'a>(text: &'a str, trimmed: &'a str, suffix: &'a str) -> (&'a str, &'a str) {
    if trimmed.is_empty() {
        return (trimmed, suffix);
    }
    // ~keep A non-empty trimmed body starts at the first non-space byte of `text`.
    let start = text.len() - text.trim_start().len();
    let mut end = start + trimmed.len();
    let mut suffix = suffix;
    while text[..end].ends_with('\\') && text[end..].starts_with('\n') {
        let body_end = start + text[start..end - 1].trim_end().len();
        if body_end == start {
            break;
        }
        end = body_end;
        suffix = "\\\n";
    }
    (&text[start..end], suffix)
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
        segments.push(label_segment(&rest[..marker_pos]));
        markers.push(marker);
        rest = &rest[marker_pos + marker.len()..];
    }
    segments.push(label_segment(rest));

    assemble_label(segments, markers)
}

/// Collapse the whitespace of one marker-free segment of a label, keeping its leading spaces
/// and tabs.
///
/// ~keep After a hard break they are the indent of the list item the label sits in, which the
/// ~keep text node wrote there; collapsed to one space, the line left the item and the link
/// ~keep broke (issue #678). The label's own two ends are trimmed later, in `assemble_label`.
fn label_segment(text: &str) -> String {
    let body = text.trim_start_matches([' ', '\t']);
    let mut segment = text[..text.len() - body.len()].to_string();
    collapse_whitespace_into(&mut segment, body);
    segment
}

/// Escape what would end the paragraph or start a block at `buffer[from..]`, text that starts a
/// line: a line of only spaces above it, then a block opener. In a link label only the block
/// opener, since the label escape (`escape_link_label`) ends a line of only spaces itself.
pub fn escape_text_line_start(buffer: &mut String, from: usize, in_link: bool) {
    let from = if in_link {
        from
    } else {
        from + end_break_lines_with_a_backslash(buffer, from)
    };
    crate::converter::utility::escaping::escape_continuation_line_start(buffer, from);
}

/// Whether `buffer[from..]` starts the line after a hard break: only indent before it on its
/// line, and a break marker at the end of the line above.
pub fn follows_a_hard_break(buffer: &str, from: usize) -> bool {
    let before = buffer[..from].trim_end_matches([' ', '\t']);
    before
        .strip_suffix('\n')
        .is_some_and(|above| above.ends_with("  ") || above.ends_with('\\'))
}

/// End each line that holds only a spaces break, right above `buffer[from..]`, text that starts a
/// line, with a backslash when the line above that run ends with a hard break. Returns the bytes
/// inserted.
///
/// ~keep Two `<br>` in a row write a spaces break, `  `, on a line of its own, and a line of only
/// ~keep spaces is blank, which ends the paragraph (issue #690). Once text follows, a backslash
/// ~keep after the spaces makes the line a break, as in a link label. A run with nothing after it
/// ~keep stays trailing whitespace, which the block end trims. A break line is bare, or carries the
/// ~keep indent of the text line, the list item's content column (issue #681), and it gets that
/// ~keep indent: a bare line before the text of an ordered item (`1. ` is 3 columns) leaves the
/// ~keep item. A line with another indent belongs to a container that has ended the paragraph. Only the run is read,
/// ~keep and it is rewritten once: a backslash inserted per line moves the rest of the buffer each
/// ~keep time, quadratic in the length of the run.
fn end_break_lines_with_a_backslash(buffer: &mut String, from: usize) -> usize {
    let before = buffer[..from].trim_end_matches([' ', '\t']);
    let Some(mut line_end) = before.strip_suffix('\n').map(str::len) else {
        return 0;
    };
    let break_line = format!("{}  ", &buffer[line_end + 1..from]);
    let mut run_start = None;
    let mut run = 0;
    loop {
        let line_start = buffer[..line_end].rfind('\n').map_or(0, |pos| pos + 1);
        let line = &buffer[line_start..line_end];
        if line != break_line && line != "  " {
            if line.trim().is_empty() || !(line.ends_with("  ") || line.ends_with('\\')) {
                return 0;
            }
            let Some(run_start) = run_start else {
                return 0;
            };
            let run_end = before.len() - 1;
            let mut lines = format!("{break_line}\\\n").repeat(run);
            lines.pop();
            let inserted = lines.len() - (run_end - run_start);
            buffer.replace_range(run_start..run_end, &lines);
            return inserted;
        }
        run_start = Some(line_start);
        run += 1;
        let Some(above) = line_start.checked_sub(1) else {
            return 0;
        };
        line_end = above;
    }
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

#[cfg(test)]
mod tests {
    use super::{chomp_inline, end_break_lines_with_a_backslash, normalize_link_label};

    // ~keep A backslash break before a space or a second break at an element's end goes outside
    // ~keep the closing marker, never before it.
    #[test]
    fn chomp_inline_moves_a_trailing_backslash_break_into_the_suffix() {
        assert_eq!(chomp_inline("a\\\n "), ("", "\\\n", "a"));
        assert_eq!(chomp_inline("a\\\n\\\n"), ("", "\\\n", "a"));
        assert_eq!(chomp_inline(" a\\\n \\\n "), (" ", "\\\n", "a"));
        assert_eq!(chomp_inline("a\\\\ "), ("", " ", "a\\\\"));
        assert_eq!(chomp_inline("a\\\nb "), ("", " ", "a\\\nb"));
        assert_eq!(chomp_inline("\\\n "), ("", " ", "\\"));
    }

    // ~keep Issue #678: the spaces or tabs after a break are the list item's indent.
    #[test]
    fn normalize_link_label_keeps_the_indent_after_a_hard_break_but_not_at_the_label_ends() {
        assert_eq!(normalize_link_label("a  \n    2.  t"), "a  \n    2. t");
        assert_eq!(normalize_link_label("a\\\n\t2. t"), "a\\\n\t2. t");
        assert_eq!(normalize_link_label("  \n  2. t"), "  \n  2. t");
        assert_eq!(normalize_link_label("  a  \n  "), "a  \n");
    }

    // ~keep Issue #690: a line of only spaces between a hard break and text gets a backslash.
    #[test]
    fn end_break_lines_with_a_backslash_ends_only_a_run_after_a_hard_break() {
        let cases = [
            ("a  \n  \nb", "a  \n  \\\nb", 1),
            ("a  \n  \n  \n  b", "a  \n    \\\n    \\\n  b", 6),
            ("1.  \n     \n   x", "1.  \n     \\\n   x", 1),
            ("- a  \n    \n    \n  x", "- a  \n    \\\n    \\\n  x", 2),
            ("1.  \n  \n   x", "1.  \n     \\\n   x", 4),
            ("a\\\n  \nb", "a\\\n  \\\nb", 1),
            ("a\n  \nb", "a\n  \nb", 0),
            ("a  \n\nb", "a  \n\nb", 0),
            ("a  \n \nb", "a  \n \nb", 0),
            ("a  \n   \nb", "a  \n   \nb", 0),
            ("  \nb", "  \nb", 0),
            ("a  \nb", "a  \nb", 0),
        ];
        for (text, expected, inserted) in cases {
            let mut buffer = text.to_string();
            let from = buffer.len() - 1;
            let count = end_break_lines_with_a_backslash(&mut buffer, from);
            assert_eq!(count, inserted, "{text:?}");
            assert_eq!(buffer, expected, "{text:?}");
        }
    }
}
