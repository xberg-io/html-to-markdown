//! HTML preprocessing and normalization.
//!
//! Functions for preprocessing HTML before conversion, including script/style stripping,
//! tag repair, and malformed HTML handling.

use std::borrow::Cow;
use std::str;

use super::markup::{find_tag_end, is_json_ld_script_open_tag, matches_end_tag_start, matches_tag_start};

/// If `idx` starts an `<svg`/`</svg` tag, adjust `svg_depth` accordingly and return the index
/// just past the tag. Returns `None` when `idx` does not start an svg tag (or the tag is
/// unterminated), in which case the caller's scan is unaffected.
///
/// Extracted from `strip_script_and_style_tags` — identical `<svg`/`</svg` detection and depth
/// bookkeeping, unchanged.
pub(super) fn track_svg_tag(bytes: &[u8], idx: usize, svg_depth: &mut usize) -> Option<usize> {
    if matches_tag_start(bytes, idx + 1, b"svg") {
        if let Some(open_end) = find_tag_end(bytes, idx + 1 + b"svg".len()) {
            *svg_depth += 1;
            return Some(open_end);
        }
    } else if matches_end_tag_start(bytes, idx + 1, b"svg") {
        if let Some(close_end) = find_tag_end(bytes, idx + 2 + b"svg".len()) {
            if *svg_depth > 0 {
                *svg_depth = svg_depth.saturating_sub(1);
            }
            return Some(close_end);
        }
    }
    None
}

/// What the first pass does with the content of a raw-text element.
#[derive(Clone, Copy, PartialEq, Eq)]
enum RawTextContent {
    /// The element is removed (`style`).
    Removed,
    /// The element is removed, but a structured data script is kept for the metadata.
    RemovedUnlessStructuredData,
    /// The element is kept and its content is text (`textarea`).
    Kept,
}

/// The page that the first pass reads.
///
/// ~keep The escape of a kept script is not idempotent (`&` becomes `&amp;`), so it runs on the
/// ~keep source page only. The tree builder writes a script as raw text and escapes the text of
/// ~keep a `textarea` itself, so a kept element of a repaired page is already in the form the
/// ~keep parser reads, however many times the page is repaired.
#[derive(Clone, Copy, PartialEq, Eq)]
enum RawTextPage {
    /// The page as its author wrote it.
    Source,
    /// The page that the HTML tree builder wrote from a page this pass has read.
    Repaired,
}

/// A candidate raw-text element: `idx` is its `<`, `last` is the end of the copied input and
/// `name` is the tag name in lower case.
struct RawTextElement<'a> {
    input: &'a str,
    idx: usize,
    last: usize,
    name: &'a str,
    content: RawTextContent,
    page: RawTextPage,
}

/// Remove or rewrite the raw-text element that starts at `element.idx`, appending the input
/// before it to `*output` (lazily allocated).
///
/// A removed element leaves one space when the two characters around it are not white space.
/// A kept element is written by `push_kept_element` and adds no space: the walker reads the
/// white space on its two sides as if the element were not there.
///
/// Returns `Some(new_pos)` when the element was consumed — the caller sets both `last` and `idx`
/// to it — or `None` when `idx` does not start a closed element of this name, or when a kept
/// element needs no change (a `textarea` with no `<`, any kept element of a repaired page).
fn strip_raw_text_element(element: &RawTextElement<'_>, output: &mut Option<String>) -> Option<usize> {
    let bytes = element.input.as_bytes();
    let name = element.name.as_bytes();
    let tag_end = raw_text_tag_end(bytes, element.idx, name, TagSide::Start)?;
    let (close_start, close_idx) = find_raw_text_end_tag(bytes, tag_end, name)?;
    let body = &element.input[tag_end..close_start];

    let kept = match element.content {
        RawTextContent::Removed => false,
        RawTextContent::RemovedUnlessStructuredData => is_json_ld_script_open_tag(&element.input[element.idx..tag_end]),
        // ~keep A `textarea` with no `<` in it is already one text for the parser.
        RawTextContent::Kept if !body.contains('<') => return None,
        RawTextContent::Kept => true,
    };
    if kept && element.page == RawTextPage::Repaired {
        return None;
    }

    let out = output.get_or_insert_with(|| String::with_capacity(element.input.len()));
    out.push_str(&element.input[element.last..element.idx]);
    if kept {
        let attributes = &element.input[element.idx + 1 + name.len()..tag_end];
        push_kept_element(out, element.name, attributes, body, element.content);
    }
    // ~keep A kept element adds no space of its own: the white space around it is the page's.
    if !kept
        && element.idx > 0
        && close_idx < bytes.len()
        && !bytes[element.idx - 1].is_ascii_whitespace()
        && !bytes[close_idx].is_ascii_whitespace()
    {
        out.push(' ');
    }
    Some(close_idx)
}

/// Write a kept raw-text element in a form the parser reads as one element with one text.
/// `attributes` is the open tag after its name, with the `>`; `body` is the content.
///
/// ~keep The parser has no raw-text rule: a `<p>` in the content would open an element that
/// ~keep holds the rest of the page, so each `<` of the body is written as `&lt;`; the reader
/// ~keep of the text decodes the reference. A script is raw text: a reference in it is not
/// ~keep decoded, so each `&` is written as `&amp;` and the one decode of the reader gives the
/// ~keep text as written. A `textarea` is escapable raw text: its references are decoded, so
/// ~keep its `&` stays. The parser also closes an element only with an end tag of the same
/// ~keep spelling, so the two tag names are written in one spelling, and it reads only white
/// ~keep space after the name, so a `/` or a form feed there is written as a space.
fn push_kept_element(out: &mut String, name: &str, attributes: &str, body: &str, content: RawTextContent) {
    let references_are_text = content == RawTextContent::RemovedUnlessStructuredData;
    out.push('<');
    out.push_str(name);
    if matches!(attributes.as_bytes().first(), Some(b'/' | b'\x0C')) {
        out.push(' ');
        out.push_str(&attributes[1..]);
    } else {
        out.push_str(attributes);
    }
    let mut rest = body;
    while let Some(at) = memchr::memchr2(b'<', b'&', rest.as_bytes()) {
        out.push_str(&rest[..at]);
        out.push_str(match rest.as_bytes()[at] {
            b'<' => "&lt;",
            _ if references_are_text => "&amp;",
            _ => "&",
        });
        rest = &rest[at + 1..];
    }
    out.push_str(rest);
    out.push_str("</");
    out.push_str(name);
    out.push('>');
}

/// The raw-text elements the first pass reads, each with what happens to its content.
const FIRST_PASS_ELEMENTS: [(&str, RawTextContent); 3] = [
    ("script", RawTextContent::RemovedUnlessStructuredData),
    ("style", RawTextContent::Removed),
    ("textarea", RawTextContent::Kept),
];

/// Strip script and style tags and their content from HTML, and write the content of a
/// structured data script and of a `textarea` as text.
pub fn strip_script_and_style_tags(input: &str) -> Cow<'_, str> {
    strip_raw_text_elements(input, RawTextPage::Source)
}

/// Strip script and style tags and their content from a page that the HTML tree builder wrote
/// from the output of [`strip_script_and_style_tags`]. A kept element stays as it is.
pub fn strip_script_and_style_tags_of_repaired_page(input: &str) -> Cow<'_, str> {
    strip_raw_text_elements(input, RawTextPage::Repaired)
}

fn strip_raw_text_elements(input: &str, page: RawTextPage) -> Cow<'_, str> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    if len == 0 {
        return Cow::Borrowed(input);
    }

    let mut idx = 0;
    let mut last = 0;
    let mut output: Option<String> = None;
    let mut svg_depth = 0usize;

    while idx < len {
        let Some(offset) = memchr::memchr(b'<', &bytes[idx..]) else {
            break;
        };
        idx += offset;

        if idx + 1 < len {
            if let Some(new_idx) = track_svg_tag(bytes, idx, &mut svg_depth) {
                idx = new_idx;
                continue;
            }

            if svg_depth > 0 {
                idx += 1;
                continue;
            }

            // ~keep Check for </script or </style (closing tags first for safety)
            if bytes[idx + 1] == b'/' && idx + 2 < len {
                if idx + 9 <= len && eq_ascii_insensitive(&bytes[idx..idx + 9], b"</script>") {
                    idx += 9;
                    continue;
                }

                if idx + 8 <= len && eq_ascii_insensitive(&bytes[idx..idx + 8], b"</style>") {
                    idx += 8;
                    continue;
                }
            }

            // ~keep The first name that matches consumes the element; no name is a prefix of another.
            let consumed = FIRST_PASS_ELEMENTS.iter().find_map(|&(name, content)| {
                let element = RawTextElement {
                    input,
                    idx,
                    last,
                    name,
                    content,
                    page,
                };
                strip_raw_text_element(&element, &mut output)
            });
            if let Some(new_pos) = consumed {
                last = new_pos;
                idx = new_pos;
                continue;
            }
        }

        idx += 1;
    }

    if let Some(mut out) = output {
        if last < len {
            out.push_str(&input[last..]);
        }
        Cow::Owned(out)
    } else {
        Cow::Borrowed(input)
    }
}

/// Upper bound on how far a closing-tag scan may travel from its start offset.
///
/// ~keep DoS guard: hostile input can contain an unterminated element followed
/// ~keep by hundreds of megabytes of text; the scan gives up rather than walking
/// ~keep the whole document for every such tag.
const MAX_CLOSING_TAG_SCAN: usize = 100_000_000;

/// The side of an element that a tag is on.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TagSide {
    /// `<name ...>`
    Start,
    /// `</name ...>`
    End,
}

/// The one rule for the start tag and the end tag of a raw-text element. If the `<` at `lt`
/// starts a tag of `name` on `side`, return the index after the `>` of the tag.
///
/// ~keep The HTML rule: the name in any case, then tab, LF, FF, CR, space, `/` or `>`. The
/// ~keep first pass, the scan of hidden regions and the fast converter call this function; a
/// ~keep second copy of the rule is how they came to disagree on the form feed and the slash.
/// ~keep The second pass (`preprocess_html`) reads only what the first pass leaves: an element
/// ~keep with no end tag.
/// ~keep A `>` in a quoted attribute value does not end a start tag. A start tag with a quote
/// ~keep that never ends, and an end tag, end at the first `>`.
pub fn raw_text_tag_end(bytes: &[u8], lt: usize, name: &[u8], side: TagSide) -> Option<usize> {
    if bytes.get(lt) != Some(&b'<') {
        return None;
    }
    let name_start = match side {
        TagSide::Start => lt + 1,
        TagSide::End if bytes.get(lt + 1) == Some(&b'/') => lt + 2,
        TagSide::End => return None,
    };
    let after_name = name_start + name.len();
    if !bytes.get(name_start..after_name)?.eq_ignore_ascii_case(name) {
        return None;
    }
    if !matches!(
        bytes.get(after_name)?,
        b'\t' | b'\n' | b'\x0C' | b'\r' | b' ' | b'/' | b'>'
    ) {
        return None;
    }
    let first_bracket = || memchr::memchr(b'>', &bytes[after_name..]).map(|offset| after_name + offset + 1);
    match side {
        TagSide::Start => find_tag_end(bytes, after_name).or_else(first_bracket),
        TagSide::End => first_bracket(),
    }
}

/// Find the position of the FIRST closing tag in bytes, ignoring nesting.
/// Returns the position AFTER the closing tag (including the '>').
/// This is highly optimized for performance and uses a fast-path scan.
///
/// ~keep First-match (not depth-aware) is correct ONLY for the HTML raw-text
/// ~keep elements `<script>` and `<style>`: inside their bodies `<script` is
/// ~keep text, not a tag, so the first `</script>` is the spec terminator.
/// ~keep Counting depth here would treat `var s = "<script>"` as an opening tag
/// ~keep and run past the real close, swallowing the rest of the document.
/// ~keep Elements that can nest must use `find_closing_tag_bytes_nested`; do not
/// ~keep unify the two functions.
#[inline]
pub fn find_closing_tag_bytes(bytes: &[u8], start: usize, tag: &[u8]) -> Option<usize> {
    find_raw_text_end_tag(bytes, start, tag).map(|(_, end)| end)
}

/// The first end tag of the raw-text element `name` at or after `start`: the position of its
/// `<` and the position after its `>`. Both converters end a raw-text element here.
pub fn find_raw_text_end_tag(bytes: &[u8], start: usize, name: &[u8]) -> Option<(usize, usize)> {
    let mut idx = start;
    while idx < bytes.len() && (idx - start) < MAX_CLOSING_TAG_SCAN {
        idx += memchr::memchr(b'<', &bytes[idx..])?;
        if let Some(end) = raw_text_tag_end(bytes, idx, name, TagSide::End) {
            return Some((idx, end));
        }
        idx += 1;
    }
    None
}

/// Void elements that never carry a closing tag, so an occurrence of one inside
/// a subtree must not raise the nesting depth of a closing-tag scan.
const VOID_ELEMENT_NAMES: [&[u8]; 4] = [b"br", b"hr", b"img", b"input"];

/// Whether an opening tag closes itself: XHTML-style `<tag/>` or a void element.
///
/// ~keep Shared by `strip_hidden_elements` and `find_closing_tag_bytes_nested`
/// ~keep so the element the stripper treats as self-closing and the element the
/// ~keep depth counter refuses to count can never disagree.
#[inline]
pub(super) fn is_self_closing_tag(tag_slice: &[u8], tag_name: &[u8]) -> bool {
    tag_slice.ends_with(b"/>")
        || VOID_ELEMENT_NAMES
            .iter()
            .any(|void_name| tag_name.eq_ignore_ascii_case(void_name))
}

/// Byte sequences delimiting the regions of an HTML document whose bytes are
/// character data rather than markup.
const COMMENT_OPEN: &[u8] = b"<!--";
const COMMENT_CLOSE: &[u8] = b"-->";
const CDATA_OPEN: &[u8] = b"<![CDATA[";
const CDATA_CLOSE: &[u8] = b"]]>";

/// Elements whose body is raw text: `<` inside them never starts a tag.
const RAW_TEXT_ELEMENT_NAMES: [&[u8]; 2] = [b"script", b"style"];

/// Index just past the first occurrence of `terminator` at or after `from`,
/// or the input length when the region is never terminated.
///
/// ~keep An unterminated comment/CDATA region runs to EOF per HTML5, so
/// ~keep returning `len` (rather than `None`) keeps the caller from resuming a
/// ~keep tag scan inside text that will never be markup.
#[inline]
fn find_sequence_end(bytes: &[u8], from: usize, terminator: &[u8]) -> usize {
    let len = bytes.len();
    if from >= len {
        return len;
    }
    match bytes[from..].windows(terminator.len()).position(|w| w == terminator) {
        Some(offset) => from + offset + terminator.len(),
        None => len,
    }
}

/// If a `<` at `idx` opens a region that is NOT markup — an HTML comment, a
/// CDATA section, or the body of a raw-text element — return the index just
/// past that region. Returns `None` when `idx` does not open such a region.
///
/// ~keep A depth-counting close-tag scan MUST consult this before interpreting
/// ~keep a `<`: `<!-- </div> -->` and `<script>{"a":"</div>"}</script>` contain
/// ~keep byte sequences that look like tags but are text. Counting them ends the
/// ~keep scan early (leaking the hidden element's tail into the output) or
/// ~keep inflates the depth (swallowing the following visible sibling).
#[inline]
pub fn skip_opaque_region(bytes: &[u8], idx: usize) -> Option<usize> {
    let len = bytes.len();
    if idx >= len || bytes[idx] != b'<' {
        return None;
    }

    if bytes[idx..].starts_with(COMMENT_OPEN) {
        return Some(find_sequence_end(bytes, idx + COMMENT_OPEN.len(), COMMENT_CLOSE));
    }

    if bytes[idx..].starts_with(CDATA_OPEN) {
        return Some(find_sequence_end(bytes, idx + CDATA_OPEN.len(), CDATA_CLOSE));
    }

    for raw_text_name in RAW_TEXT_ELEMENT_NAMES {
        if let Some(open_end) = raw_text_tag_end(bytes, idx, raw_text_name, TagSide::Start) {
            // ~keep First-match close is the spec terminator for raw text — see
            // ~keep the doc comment on `find_closing_tag_bytes`.
            return Some(find_closing_tag_bytes(bytes, open_end, raw_text_name).unwrap_or(len));
        }
    }

    None
}

/// Whether the `<` at `idx` starts a tag rather than a stray `<` in text.
///
/// ~keep Mirrors the `is_valid_tag` test in `preprocess_html`: without it a
/// ~keep literal `<` in text (`a < b`) would make a scan jump to the next `>`,
/// ~keep stepping over the real closing tag in between.
#[inline]
pub(super) fn opens_a_tag(bytes: &[u8], idx: usize) -> bool {
    match bytes.get(idx + 1) {
        Some(b'/' | b'!') => bytes.get(idx + 2).is_some_and(u8::is_ascii_alphabetic),
        Some(byte) => byte.is_ascii_alphabetic(),
        None => false,
    }
}

/// Find the closing tag that matches an opening `<tag>`, counting nested
/// elements of the same name. Returns the position AFTER the closing tag
/// (including the '>'), or `None` if the element is never closed.
///
/// ~keep Required for elements that can nest (`div`, `span`, ...): the
/// ~keep first-match `find_closing_tag_bytes` stops at the INNER `</tag>`, which
/// ~keep leaves the hidden element's tail in the document. Kept separate from
/// ~keep that function because raw-text elements must NOT count depth — see its
/// ~keep doc comment.
///
/// ~keep Only real markup may move the depth counter, so every `<` is first
/// ~keep offered to `skip_opaque_region`, and a tag that is not the target is
/// ~keep jumped via the quote-aware `find_tag_end` rather than byte-by-byte:
/// ~keep otherwise `</tag>` written inside a comment, a quoted attribute value,
/// ~keep or a raw-text body is mistaken for the element's terminator.
#[inline]
pub fn find_closing_tag_bytes_nested(bytes: &[u8], start: usize, tag: &[u8]) -> Option<usize> {
    let len = bytes.len();
    if tag.is_empty() {
        return None;
    }

    let mut idx = start;
    let mut depth = 1usize;

    while idx < len && (idx - start) < MAX_CLOSING_TAG_SCAN {
        if bytes[idx] != b'<' {
            match memchr::memchr(b'<', &bytes[idx..]) {
                Some(pos) => idx += pos,
                None => break,
            }
        }

        if let Some(region_end) = skip_opaque_region(bytes, idx) {
            idx = region_end;
            continue;
        }

        if matches_end_tag_start(bytes, idx + 1, tag) {
            if let Some(close_end) = find_tag_end(bytes, idx + 2 + tag.len()) {
                depth -= 1;
                if depth == 0 {
                    return Some(close_end);
                }
                idx = close_end;
                continue;
            }
        } else if matches_tag_start(bytes, idx + 1, tag) {
            if let Some(open_end) = find_tag_end(bytes, idx + 1 + tag.len()) {
                if !is_self_closing_tag(&bytes[idx..open_end], tag) {
                    depth += 1;
                }
                idx = open_end;
                continue;
            }
        } else if opens_a_tag(bytes, idx) {
            if let Some(unrelated_tag_end) = find_tag_end(bytes, idx + 1) {
                idx = unrelated_tag_end;
                continue;
            }
        }

        idx += 1;
    }

    None
}

/// Compare bytes ignoring ASCII case.
#[inline]
pub fn eq_ascii_insensitive(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).all(|(x, y)| x.eq_ignore_ascii_case(y))
}
