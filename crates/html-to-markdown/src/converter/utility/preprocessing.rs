//! HTML preprocessing and normalization.
//!
//! Functions for preprocessing HTML before conversion, including script/style stripping,
//! tag repair, and malformed HTML handling.

use std::borrow::Cow;
use std::str;

/// Strip script and style tags and their content from HTML.
pub fn strip_script_and_style_tags(input: &str) -> Cow<'_, str> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    if len == 0 {
        return Cow::Borrowed(input);
    }

    let mut idx = 0;
    let mut last = 0;
    let mut output: Option<String> = None;
    let mut svg_depth = 0usize;

    // Fast-path: check if there are any < characters at all
    if !bytes.contains(&b'<') {
        return Cow::Borrowed(input);
    }

    while idx < len {
        if bytes[idx] == b'<' && idx + 1 < len {
            if matches_tag_start(bytes, idx + 1, b"svg") {
                if let Some(open_end) = find_tag_end(bytes, idx + 1 + b"svg".len()) {
                    svg_depth += 1;
                    idx = open_end;
                    continue;
                }
            } else if matches_end_tag_start(bytes, idx + 1, b"svg") {
                if let Some(close_end) = find_tag_end(bytes, idx + 2 + b"svg".len()) {
                    if svg_depth > 0 {
                        svg_depth = svg_depth.saturating_sub(1);
                    }
                    idx = close_end;
                    continue;
                }
            }

            if svg_depth > 0 {
                idx += 1;
                continue;
            }

            // Check for </script or </style (closing tags first for safety)
            if bytes[idx + 1] == b'/' && idx + 2 < len {
                // Match </script>
                if idx + 9 <= len && eq_ascii_insensitive(&bytes[idx..idx + 9], b"</script>") {
                    idx += 9;
                    continue;
                }

                // Match </style>
                if idx + 8 <= len && eq_ascii_insensitive(&bytes[idx..idx + 8], b"</style>") {
                    idx += 8;
                    continue;
                }
            }

            // Check for <script or <style (opening tags)
            // Match <script (case insensitive)
            if idx + 7 < len && eq_ascii_insensitive(&bytes[idx..idx + 7], b"<script") {
                // Check if this is actually "<script" followed by whitespace, >, or attribute
                let after_tag = bytes[idx + 7];
                if after_tag == b'>'
                    || after_tag == b' '
                    || after_tag == b'\t'
                    || after_tag == b'\n'
                    || after_tag == b'\r'
                {
                    // Find the opening tag end
                    let mut tag_end = idx + 7;
                    while tag_end < len && bytes[tag_end] != b'>' {
                        tag_end += 1;
                    }

                    if tag_end < len {
                        tag_end += 1; // Include the '>'

                        // Check if this is a JSON-LD script tag
                        let tag_content = &input[idx..tag_end];
                        if !is_json_ld_script_open_tag(tag_content) {
                            // Find the closing </script> tag
                            let close_tag = find_closing_tag_bytes(bytes, tag_end, b"script");
                            if let Some(close_idx) = close_tag {
                                let out = output.get_or_insert_with(|| String::with_capacity(len));
                                out.push_str(&input[last..idx]);
                                if idx > 0
                                    && close_idx < len
                                    && !bytes[idx - 1].is_ascii_whitespace()
                                    && !bytes[close_idx].is_ascii_whitespace()
                                {
                                    out.push(' ');
                                }
                                last = close_idx;
                                idx = close_idx;
                                continue;
                            }
                        }
                    }
                }
            }
            // Match <style (case insensitive)
            else if idx + 6 < len && eq_ascii_insensitive(&bytes[idx..idx + 6], b"<style") {
                // Check if this is actually "<style" followed by whitespace, >, or attribute
                let after_tag = bytes[idx + 6];
                if after_tag == b'>'
                    || after_tag == b' '
                    || after_tag == b'\t'
                    || after_tag == b'\n'
                    || after_tag == b'\r'
                {
                    // Find the opening tag end
                    let mut tag_end = idx + 6;
                    while tag_end < len && bytes[tag_end] != b'>' {
                        tag_end += 1;
                    }

                    if tag_end < len {
                        tag_end += 1; // Include the '>'

                        // Find the closing </style> tag
                        let close_tag = find_closing_tag_bytes(bytes, tag_end, b"style");
                        if let Some(close_idx) = close_tag {
                            let out = output.get_or_insert_with(|| String::with_capacity(len));
                            out.push_str(&input[last..idx]);
                            if idx > 0
                                && close_idx < len
                                && !bytes[idx - 1].is_ascii_whitespace()
                                && !bytes[close_idx].is_ascii_whitespace()
                            {
                                out.push(' ');
                            }
                            last = close_idx;
                            idx = close_idx;
                            continue;
                        }
                    }
                }
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

/// Find the position of a closing tag in bytes.
/// Returns the position AFTER the closing tag (including the '>').
/// This is highly optimized for performance and uses a fast-path scan.
#[inline]
pub fn find_closing_tag_bytes(bytes: &[u8], start: usize, tag: &[u8]) -> Option<usize> {
    let len = bytes.len();
    let tag_len = tag.len();

    // Fast path: look for the closing tag pattern byte-by-byte
    // We use a simple byte scan to find '</' then validate the tag name
    let mut idx = start;

    // Limit search to prevent stack overflow on large files
    // Look for closing tag within reasonable bounds
    const MAX_SCAN: usize = 100_000_000; // 100MB limit per tag - prevents pathological cases

    while idx < len && (idx - start) < MAX_SCAN {
        // Optimization: skip forward to next '<' quickly using memchr
        if bytes[idx] != b'<' {
            if let Some(pos) = memchr::memchr(b'<', &bytes[idx..]) {
                idx += pos;
            } else {
                break;
            }
        }

        // Check for </ pattern
        if idx + 2 < len && bytes[idx + 1] == b'/' {
            // Check if tag name matches
            if idx + 2 + tag_len <= len && eq_ascii_insensitive(&bytes[idx + 2..idx + 2 + tag_len], tag) {
                // Ensure it's followed by > or whitespace
                let after_tag = idx + 2 + tag_len;
                if after_tag < len && (bytes[after_tag] == b'>' || bytes[after_tag].is_ascii_whitespace()) {
                    // Find the >
                    let mut close_idx = after_tag;
                    while close_idx < len && bytes[close_idx] != b'>' {
                        close_idx += 1;
                    }
                    if close_idx < len {
                        return Some(close_idx + 1); // Include the '>'
                    }
                }
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

/// Normalize HTML comment endings that would confuse the `tl` parser.
///
/// The `astral-tl` parser mishandles HTML comments whose closing sequence
/// contains more than two dashes before the `>` (e.g. `<!-- foo --->` or
/// `<!-- foo ---->`).  When it encounters such a comment it creates an empty
/// comment node and silently discards every byte that follows, so all document
/// content after the comment is lost.
///
/// This function rewrites those bogus closings: every `--[-]+>` sequence that
/// terminates an HTML comment is normalised to `-->`.  Regular `-->` closings
/// are left unchanged.
///
/// # Algorithm
///
/// Scans the input byte-by-byte looking for `<!--`.  For each comment found it
/// scans forward for `-->` using the HTML5 comment-end state machine:
///
/// - `--[` zero or more `-` `]>` ends the comment.
/// - Any other character after `--` resets back into the comment body.
///
/// If the actual number of leading dashes before `>` is more than two the
/// closing sequence is replaced with `-->`.
pub fn normalize_bogus_comment_endings(input: &str) -> Cow<'_, str> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    // Fast path: the input must contain at least "<!--" and "--->".
    // Without "<!--" there are no comments; without "---" there cannot be a
    // bogus closing.
    if len < 7 || !bytes.windows(4).any(|w| w == b"<!--") {
        return Cow::Borrowed(input);
    }

    let mut idx = 0;
    let mut last = 0;
    let mut output: Option<String> = None;

    while idx + 3 < len {
        // Find the next comment opening.
        if !(bytes[idx] == b'<' && bytes[idx + 1] == b'!' && bytes[idx + 2] == b'-' && bytes[idx + 3] == b'-') {
            idx += 1;
            continue;
        }

        // We are positioned at `<!--`.
        idx += 4; // advance past `<!--`

        // Walk the comment body looking for the closing sequence.
        // The HTML5 comment-end state machine:
        //   COMMENT state: most chars append to body; `-` → COMMENT_END_DASH
        //   COMMENT_END_DASH: `-` → COMMENT_END; other → COMMENT
        //   COMMENT_END: `>` → done; `-` → stay in COMMENT_END (extra dash);
        //                other → COMMENT
        // We track consecutive dashes at the current position.
        let mut consecutive_dashes: usize = 0;

        while idx < len {
            let b = bytes[idx];
            if b == b'-' {
                consecutive_dashes += 1;
                idx += 1;
            } else if b == b'>' && consecutive_dashes >= 2 {
                // We found a closing sequence.  `consecutive_dashes` is the
                // total number of dashes before this `>`.  A well-formed close
                // is exactly two (`-->`).  Any additional dashes are bogus.
                if consecutive_dashes > 2 {
                    // Rewrite: keep the comment body (without the extra dashes)
                    // and replace the closing sequence with `-->`.
                    let out = output.get_or_insert_with(|| String::with_capacity(len));
                    // Flush everything up to the start of the extra dashes.
                    // The comment body ends `consecutive_dashes` bytes before
                    // the current `idx` (which points at `>`).
                    let close_start = idx - consecutive_dashes;
                    out.push_str(&input[last..close_start]);
                    out.push_str("-->");
                    idx += 1; // consume `>`
                    last = idx;
                } else {
                    // Normal `-->` — no rewrite needed.
                    idx += 1; // consume `>`
                }
                break;
            } else {
                // Any non-dash non-`>` character resets the dash count and
                // returns us to the plain comment body state.
                consecutive_dashes = 0;
                idx += 1;
            }
        }
        // If we reached end-of-input without finding a close, the comment is
        // unclosed.  We leave the remainder as-is; the parser will handle it.
    }

    match output {
        Some(mut out) => {
            if last < len {
                out.push_str(&input[last..]);
            }
            Cow::Owned(out)
        }
        None => Cow::Borrowed(input),
    }
}

/// Normalize closing tags whose `>` appears on a subsequent line.
///
/// Some HTML formatters (JSX-style) write closing tags as:
///
/// ```html
/// </a
/// >
/// ```
///
/// The `tl` parser does not handle end-tags with a newline before the closing
/// `>`, leaving the element unclosed so all subsequent siblings become children
/// of the open element.  This pass collapses such patterns to a single-line
/// closing tag (`</a>`) before the document reaches `tl`.
///
/// Only the whitespace between the tag name and the closing `>` is normalised;
/// the rest of the document is untouched.
pub fn normalize_split_closing_tags(input: &str) -> Cow<'_, str> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    // Fast path: need both '</' and '\n' to have any candidates.
    if len < 4 || !bytes.contains(&b'\n') {
        return Cow::Borrowed(input);
    }

    let mut idx = 0;
    let mut last = 0;
    let mut output: Option<String> = None;

    while idx + 2 < len {
        // Look for `</`
        if bytes[idx] != b'<' || bytes[idx + 1] != b'/' {
            idx += 1;
            continue;
        }

        // Scan tag name: ASCII letters, digits, hyphens (HTML5 allows hyphens in custom elements)
        let name_start = idx + 2;
        let mut name_end = name_start;
        while name_end < len && (bytes[name_end].is_ascii_alphanumeric() || bytes[name_end] == b'-') {
            name_end += 1;
        }

        if name_end == name_start {
            // No tag name — not a closing tag we care about.
            idx += 1;
            continue;
        }

        // After the tag name, skip any whitespace.  If there is a newline in
        // that whitespace before the `>`, we need to rewrite.
        let ws_start = name_end;
        let mut ws_end = ws_start;
        let mut has_newline = false;
        while ws_end < len && bytes[ws_end].is_ascii_whitespace() {
            if bytes[ws_end] == b'\n' || bytes[ws_end] == b'\r' {
                has_newline = true;
            }
            ws_end += 1;
        }

        if !has_newline || ws_end >= len || bytes[ws_end] != b'>' {
            // Either no whitespace newline, or the `>` is not the next char.
            idx += 1;
            continue;
        }

        // We have `</tagname [whitespace-with-newline]>` — rewrite to `</tagname>`.
        let tag_name = &input[name_start..name_end];
        let out = output.get_or_insert_with(|| String::with_capacity(len));
        out.push_str(&input[last..idx]);
        out.push_str("</");
        out.push_str(tag_name);
        out.push('>');

        idx = ws_end + 1; // advance past the `>`
        last = idx;
    }

    match output {
        Some(mut out) => {
            if last < len {
                out.push_str(&input[last..]);
            }
            Cow::Owned(out)
        }
        None => Cow::Borrowed(input),
    }
}

/// Preprocess HTML to normalize tags and fix common issues.
pub fn preprocess_html(input: &str) -> Cow<'_, str> {
    const SELF_CLOSING: [(&[u8], &str); 3] = [(b"<br/>", "<br>"), (b"<hr/>", "<hr>"), (b"<img/>", "<img>")];
    const TAGS: [&[u8]; 2] = [b"script", b"style"];
    const SVG: &[u8] = b"svg";
    const DOCTYPE: &[u8] = b"doctype";
    const EMPTY_COMMENT: &[u8] = b"<!---->";

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
        if bytes[idx] == b'<' {
            if bytes[idx..].starts_with(EMPTY_COMMENT) {
                let out = output.get_or_insert_with(|| String::with_capacity(input.len()));
                out.push_str(&input[last..idx]);
                out.push_str("<!-- -->");
                idx += EMPTY_COMMENT.len();
                last = idx;
                continue;
            }

            let mut replaced = false;
            for (pattern, replacement) in &SELF_CLOSING {
                if bytes[idx..].starts_with(pattern) {
                    let out = output.get_or_insert_with(|| String::with_capacity(input.len()));
                    out.push_str(&input[last..idx]);
                    out.push_str(replacement);
                    idx += pattern.len();
                    last = idx;
                    replaced = true;
                    break;
                }
            }
            if replaced {
                continue;
            }

            if matches_tag_start(bytes, idx + 1, SVG) {
                if let Some(open_end) = find_tag_end(bytes, idx + 1 + SVG.len()) {
                    svg_depth += 1;
                    idx = open_end;
                    continue;
                }
            } else if matches_end_tag_start(bytes, idx + 1, SVG) {
                if let Some(close_end) = find_tag_end(bytes, idx + 2 + SVG.len()) {
                    if svg_depth > 0 {
                        svg_depth = svg_depth.saturating_sub(1);
                    }
                    idx = close_end;
                    continue;
                }
            }

            if svg_depth == 0 {
                let mut handled = false;
                for tag in TAGS {
                    if matches_tag_start(bytes, idx + 1, tag) {
                        if let Some(open_end) = find_tag_end(bytes, idx + 1 + tag.len()) {
                            if tag == b"script" && is_json_ld_script_open_tag(&input[idx..open_end]) {
                                continue;
                            }
                            let remove_end = find_closing_tag(bytes, open_end, tag).unwrap_or(open_end);
                            let out = output.get_or_insert_with(|| String::with_capacity(input.len()));
                            out.push_str(&input[last..idx]);
                            out.push_str(&input[idx..open_end]);
                            out.push_str("</");
                            // `TAGS` contains only ASCII byte literals (`b"script"`, `b"style"`),
                            // which are always valid UTF-8; `from_utf8` cannot fail here.
                            if let Ok(tag_str) = str::from_utf8(tag) {
                                out.push_str(tag_str);
                            }
                            out.push('>');

                            last = remove_end;
                            idx = remove_end;
                            handled = true;
                        }
                    }

                    if handled {
                        break;
                    }
                }

                if handled {
                    continue;
                }

                if idx + 2 < len && bytes[idx + 1] == b'!' {
                    let mut cursor = idx + 2;
                    while cursor < len && bytes[cursor].is_ascii_whitespace() {
                        cursor += 1;
                    }

                    if cursor + DOCTYPE.len() <= len
                        && bytes[cursor..cursor + DOCTYPE.len()].eq_ignore_ascii_case(DOCTYPE)
                    {
                        if let Some(end) = find_tag_end(bytes, cursor + DOCTYPE.len()) {
                            let out = output.get_or_insert_with(|| String::with_capacity(input.len()));
                            out.push_str(&input[last..idx]);
                            last = end;
                            idx = end;
                            continue;
                        }
                    }
                }
            }

            let is_valid_tag = if idx + 1 < len {
                match bytes[idx + 1] {
                    b'!' => {
                        idx + 2 < len
                            && (bytes[idx + 2] == b'-'
                                || bytes[idx + 2].is_ascii_alphabetic()
                                || bytes[idx + 2].is_ascii_uppercase())
                    }
                    b'/' => {
                        idx + 2 < len && (bytes[idx + 2].is_ascii_alphabetic() || bytes[idx + 2].is_ascii_uppercase())
                    }
                    b'?' => true,
                    c if c.is_ascii_alphabetic() || c.is_ascii_uppercase() => true,
                    _ => false,
                }
            } else {
                false
            };

            if !is_valid_tag {
                let out = output.get_or_insert_with(|| String::with_capacity(input.len() + 4));
                out.push_str(&input[last..idx]);
                out.push_str("&lt;");
                idx += 1;
                last = idx;
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

/// Check if a script tag is a JSON-LD script.
pub fn is_json_ld_script_open_tag(tag: &str) -> bool {
    let bytes = tag.as_bytes();
    let mut idx = 0;
    while idx + 4 <= bytes.len() {
        if eq_ascii_case_insensitive(&bytes[idx..], b"type") {
            let before_ok = idx == 0
                || bytes
                    .get(idx.saturating_sub(1))
                    .is_some_and(|b| b.is_ascii_whitespace() || *b == b'<' || *b == b'/');
            let after_ok = bytes
                .get(idx + 4)
                .is_some_and(|b| b.is_ascii_whitespace() || *b == b'=');
            if !before_ok || !after_ok {
                idx += 4;
                continue;
            }

            let mut i = idx + 4;
            while bytes.get(i).is_some_and(u8::is_ascii_whitespace) {
                i += 1;
            }
            if bytes.get(i) != Some(&b'=') {
                idx += 4;
                continue;
            }
            i += 1;
            while bytes.get(i).is_some_and(u8::is_ascii_whitespace) {
                i += 1;
            }
            if i >= bytes.len() {
                return false;
            }

            let (value_start, value_end) = match bytes[i] {
                b'"' | b'\'' => {
                    let quote = bytes[i];
                    let start = i + 1;
                    let mut end = start;
                    while end < bytes.len() && bytes[end] != quote {
                        end += 1;
                    }
                    (start, end)
                }
                _ => {
                    let start = i;
                    let mut end = start;
                    while end < bytes.len() && !bytes[end].is_ascii_whitespace() && bytes[end] != b'>' {
                        end += 1;
                    }
                    (start, end)
                }
            };

            let value = &tag[value_start..value_end];
            let media_type = value.split(';').next().unwrap_or(value).trim();
            return eq_ascii_case_insensitive(media_type.as_bytes(), b"application/ld+json");
        }
        idx += 1;
    }
    false
}

/// Case-insensitive byte comparison for ASCII.
#[inline]
pub fn eq_ascii_case_insensitive(haystack: &[u8], needle: &[u8]) -> bool {
    if haystack.len() < needle.len() {
        return false;
    }
    haystack
        .iter()
        .zip(needle.iter())
        .all(|(a, b)| a.eq_ignore_ascii_case(b))
}

/// Check if bytes match a tag start pattern.
pub fn matches_tag_start(bytes: &[u8], mut start: usize, tag: &[u8]) -> bool {
    if start >= bytes.len() {
        return false;
    }

    if start + tag.len() > bytes.len() {
        return false;
    }

    if !bytes[start..start + tag.len()].eq_ignore_ascii_case(tag) {
        return false;
    }

    start += tag.len();

    match bytes.get(start) {
        Some(b'>' | b'/' | b' ' | b'\t' | b'\n' | b'\r') => true,
        Some(_) => false,
        None => true,
    }
}

/// Find the end of an HTML tag (the position of '>').
pub fn find_tag_end(bytes: &[u8], mut idx: usize) -> Option<usize> {
    let len = bytes.len();
    let mut in_quote: Option<u8> = None;

    while idx < len {
        match bytes[idx] {
            b'"' | b'\'' => {
                if let Some(current) = in_quote {
                    if current == bytes[idx] {
                        in_quote = None;
                    }
                } else {
                    in_quote = Some(bytes[idx]);
                }
            }
            b'>' if in_quote.is_none() => return Some(idx + 1),
            _ => {}
        }
        idx += 1;
    }

    None
}

/// Find the closing tag for a given tag name.
pub fn find_closing_tag(bytes: &[u8], mut idx: usize, tag: &[u8]) -> Option<usize> {
    let len = bytes.len();
    let mut depth = 1usize;

    while idx < len {
        if bytes[idx] == b'<' {
            if matches_tag_start(bytes, idx + 1, tag) {
                if let Some(next) = find_tag_end(bytes, idx + 1 + tag.len()) {
                    depth += 1;
                    idx = next;
                    continue;
                }
            } else if matches_end_tag_start(bytes, idx + 1, tag) {
                if let Some(close) = find_tag_end(bytes, idx + 2 + tag.len()) {
                    depth -= 1;
                    if depth == 0 {
                        return Some(close);
                    }
                    idx = close;
                    continue;
                }
            }
        }

        idx += 1;
    }

    None
}

/// Check if bytes match an end tag pattern.
pub fn matches_end_tag_start(bytes: &[u8], start: usize, tag: &[u8]) -> bool {
    if start >= bytes.len() || bytes[start] != b'/' {
        return false;
    }
    matches_tag_start(bytes, start + 1, tag)
}

/// Close implicitly-terminated list-item elements that `tl` would otherwise
/// absorb as deep children, causing stack overflows on large documents.
///
/// The HTML5 parsing spec (§13.2.6.4.7 "in body" insertion mode) states that
/// an open `<li>`, `<dt>`, or `<dd>` tag is *implicitly closed* when:
///
/// - Another `<li>` / `<dt>` / `<dd>` open tag is encountered, or
/// - The closing `</ul>`, `</ol>`, or `</dl>` tag is reached.
///
/// The `tl` parser is not a full HTML5 parser; it does not apply implicit
/// closure rules.  When it encounters `<li>content<li>more` it treats the
/// second `<li>` as a child of the first, producing a linear chain of depth
/// equal to the number of items.  A list with 400 unclosed `<li>` items
/// causes `walk_node` to recurse 400 levels deep; with 467 such lists in a
/// single document (curl.se/changes.html) the cumulative stack depth triggers
/// an OS-level stack overflow.
///
/// This function rewrites the source HTML in one linear pass before `tl` sees
/// it, inserting the missing `</li>`, `</dt>`, and `</dd>` close tags exactly
/// where the HTML5 spec says they belong.
///
/// # Scope
///
/// Only the three list-item element types are handled here; `<p>` and other
/// auto-closing block elements are intentionally left to the existing
/// `has_inline_block_misnest` → `repair_with_html5ever` path.
pub fn normalize_unclosed_list_items(input: &str) -> Cow<'_, str> {
    // Fast-path: skip the scan entirely when there are no list-item tags.
    // `<li`, `<dt`, and `<dd` share the first two bytes `<l`, `<d` — if
    // neither byte sequence can appear there is nothing to do.
    let bytes = input.as_bytes();
    let len = bytes.len();
    if len < 4
        || (!bytes.windows(3).any(|w| {
            w.eq_ignore_ascii_case(b"<li") || w.eq_ignore_ascii_case(b"<dt") || w.eq_ignore_ascii_case(b"<dd")
        }))
    {
        return Cow::Borrowed(input);
    }

    // `open_item` tracks the tag name of the currently-open list-item element
    // (one of "li", "dt", "dd"), or `None` when no such element is open.
    // We use a simple single-element stack because `<li>` cannot contain
    // another `<li>` without an intervening `</li>` in well-formed HTML —
    // the only nesting that is legal is inside a child `<ul>`/`<ol>`/`<dl>`.
    // We therefore reset the stack on every `</ul>`, `</ol>`, `</dl>`.
    let mut open_item: Option<&'static str> = None;
    // Depth of `<ul>` / `<ol>` / `<dl>` nesting.  We only track open
    // list-item state for the innermost list (depth == 1 at the item level).
    // When a nested list opens we push a "saved" state; when it closes we
    // restore it.  We use a `Vec` because nesting can be arbitrary.
    let mut list_stack: Vec<Option<&'static str>> = Vec::new();

    // Tracks whether the current scan position is inside a `<pre>` or
    // `<code>` block, an HTML comment, or an attribute value — all of which
    // must be passed through verbatim.
    let mut in_pre_or_code: usize = 0; // nesting depth counter
    let mut in_comment = false;

    let mut idx = 0usize;
    let mut last_flush = 0usize;
    let mut output: Option<String> = None;

    // Helper: emit `close_tag` just before position `pos` by flushing the
    // bytes up to `pos` and then appending the close tag.
    macro_rules! emit_close_before {
        ($pos:expr, $close_tag:expr) => {{
            let out = output.get_or_insert_with(|| String::with_capacity(len + 64));
            out.push_str(&input[last_flush..$pos]);
            out.push_str($close_tag);
            last_flush = $pos;
        }};
    }

    while idx < len {
        let b = bytes[idx];

        // ── HTML comment handling ─────────────────────────────────────────────
        if in_comment {
            if b == b'-' && idx + 2 < len && bytes[idx + 1] == b'-' && bytes[idx + 2] == b'>' {
                in_comment = false;
                idx += 3;
            } else {
                idx += 1;
            }
            continue;
        }

        if b == b'<' && idx + 3 < len && bytes[idx + 1] == b'!' && bytes[idx + 2] == b'-' && bytes[idx + 3] == b'-' {
            in_comment = true;
            idx += 4;
            continue;
        }

        // ── Non-tag character ─────────────────────────────────────────────────
        if b != b'<' {
            idx += 1;
            continue;
        }

        // ── We are at `<` ─────────────────────────────────────────────────────
        let tag_start = idx;
        idx += 1;
        if idx >= len {
            break;
        }

        let is_close = bytes[idx] == b'/';
        if is_close {
            idx += 1;
            if idx >= len {
                break;
            }
        }

        // Skip whitespace after `<` or `</`
        while idx < len && bytes[idx].is_ascii_whitespace() {
            idx += 1;
        }

        // Collect tag name (ASCII alphanumeric + hyphens)
        let name_start = idx;
        while idx < len {
            let ch = bytes[idx];
            if ch == b'>' || ch == b'/' || ch.is_ascii_whitespace() {
                break;
            }
            idx += 1;
        }
        let name_bytes = &bytes[name_start..idx];
        if name_bytes.is_empty() {
            continue;
        }

        // Advance to the closing `>` of this tag (skip quoted attribute values)
        {
            let mut in_single_quote = false;
            let mut in_double_quote = false;
            while idx < len {
                match bytes[idx] {
                    b'\'' if !in_double_quote => {
                        in_single_quote = !in_single_quote;
                        idx += 1;
                    }
                    b'"' if !in_single_quote => {
                        in_double_quote = !in_double_quote;
                        idx += 1;
                    }
                    b'>' if !in_single_quote && !in_double_quote => {
                        idx += 1;
                        break;
                    }
                    _ => {
                        idx += 1;
                    }
                }
            }
        }

        // ── Classify the tag ─────────────────────────────────────────────────
        let tag_is_verbatim = name_bytes.eq_ignore_ascii_case(b"pre")
            || name_bytes.eq_ignore_ascii_case(b"code")
            || name_bytes.eq_ignore_ascii_case(b"script")
            || name_bytes.eq_ignore_ascii_case(b"style");

        if tag_is_verbatim {
            if is_close {
                in_pre_or_code = in_pre_or_code.saturating_sub(1);
            } else {
                in_pre_or_code += 1;
            }
            continue;
        }

        // Inside verbatim blocks: pass through unchanged.
        if in_pre_or_code > 0 {
            continue;
        }

        let is_list_container = name_bytes.eq_ignore_ascii_case(b"ul")
            || name_bytes.eq_ignore_ascii_case(b"ol")
            || name_bytes.eq_ignore_ascii_case(b"dl");

        let is_li = name_bytes.eq_ignore_ascii_case(b"li");
        let is_def_term = name_bytes.eq_ignore_ascii_case(b"dt");
        let is_def_desc = name_bytes.eq_ignore_ascii_case(b"dd");
        let is_list_item = is_li || is_def_term || is_def_desc;

        if is_close {
            if is_list_container {
                // Closing a list: implicitly close any open list-item inside it.
                if let Some(item) = open_item.take() {
                    let close_tag = match item {
                        "li" => "</li>",
                        "dt" => "</dt>",
                        "dd" => "</dd>",
                        _ => unreachable!(),
                    };
                    emit_close_before!(tag_start, close_tag);
                }
                // Restore the outer list's open-item state from the stack.
                open_item = list_stack.pop().unwrap_or(None);
            } else if is_list_item {
                // Explicit close: clear the open-item state.
                open_item = None;
            }
        } else {
            // Opening tag
            if is_list_container {
                // Save the current open-item state and start fresh for the nested list.
                list_stack.push(open_item.take());
            } else if is_list_item {
                // Determine the tag name as a static str so we can store it.
                let item_name: &'static str = if is_li {
                    "li"
                } else if is_def_term {
                    "dt"
                } else {
                    "dd"
                };

                if let Some(prev_item) = open_item.replace(item_name) {
                    // The previous list-item was not explicitly closed; insert its
                    // closing tag immediately before this new opening tag.
                    let close_tag = match prev_item {
                        "li" => "</li>",
                        "dt" => "</dt>",
                        "dd" => "</dd>",
                        _ => unreachable!(),
                    };
                    emit_close_before!(tag_start, close_tag);
                }
            }
        }
    }

    // If a list-item was still open at the very end, close it.
    if let Some(item) = open_item.take() {
        let close_tag = match item {
            "li" => "</li>",
            "dt" => "</dt>",
            "dd" => "</dd>",
            _ => unreachable!(),
        };
        let out = output.get_or_insert_with(|| String::with_capacity(len + 16));
        out.push_str(&input[last_flush..]);
        out.push_str(close_tag);
        last_flush = len;
    }

    match output {
        Some(mut out) => {
            if last_flush < len {
                out.push_str(&input[last_flush..]);
            }
            Cow::Owned(out)
        }
        None => Cow::Borrowed(input),
    }
}

/// Sanitize malformed markdown-like URLs in HTML attributes.
///
/// Handles cases like: `//[domain.com/path](http://domain.com/path)`
/// Extracts the actual URL from parentheses.
///
/// This is an internal function used during preprocessing to extract valid URLs
/// from malformed HTML that contains markdown-like syntax.
///
/// # Arguments
/// * `url` - The URL string to sanitize
///
/// # Returns
/// * `Cow<str>` - Either the borrowed original URL or an owned sanitized version
pub fn sanitize_markdown_url(url: &str) -> Cow<'_, str> {
    // Pattern: ...[text](actual_url) or similar markdown-like syntax
    // This handles malformed HTML where markdown syntax wasn't properly converted
    // and prevents downstream URL parsing errors (e.g., bracketed "IPv6" hosts).

    // Fast-path: we only care about markdown-like link syntax.
    let Some(mid) = url.find("](") else {
        return Cow::Borrowed(url);
    };

    // Ensure there is an opening '[' before the "](..." sequence.
    if !url[..mid].contains('[') {
        return Cow::Borrowed(url);
    }

    let paren_start = mid + 2;
    let Some(rel_end) = url[paren_start..].find(')') else {
        return Cow::Borrowed(url);
    };
    let paren_end = paren_start + rel_end;
    if paren_start >= paren_end {
        return Cow::Borrowed(url);
    }

    Cow::Owned(url[paren_start..paren_end].to_string())
}

/// Strip elements with the `hidden` attribute from HTML.
///
/// Scans for opening tags containing the `hidden` attribute, finds their
/// matching closing tag, and removes the entire element (tag + content).
/// Self-closing tags with `hidden` are also removed.
pub fn strip_hidden_elements(input: &str) -> Cow<'_, str> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    if len == 0 || !bytes.contains(&b'<') {
        return Cow::Borrowed(input);
    }

    let mut idx = 0;
    let mut last = 0;
    let mut output: Option<String> = None;

    while idx < len {
        if bytes[idx] == b'<' && idx + 1 < len && bytes[idx + 1] != b'/' && bytes[idx + 1] != b'!' {
            // Find the end of this opening tag
            if let Some(tag_end) = find_tag_end(bytes, idx + 1) {
                let tag_slice = &input[idx..tag_end];
                if tag_has_hidden_attribute(tag_slice) {
                    // Extract the tag name
                    let name_start = idx + 1;
                    let mut name_end = name_start;
                    while name_end < len
                        && !bytes[name_end].is_ascii_whitespace()
                        && bytes[name_end] != b'>'
                        && bytes[name_end] != b'/'
                    {
                        name_end += 1;
                    }
                    let tag_name = &bytes[name_start..name_end];

                    // Check if it's a self-closing tag (e.g., <br hidden> or <br hidden/>)
                    let is_self_closing = tag_slice.ends_with("/>")
                        || tag_name.eq_ignore_ascii_case(b"br")
                        || tag_name.eq_ignore_ascii_case(b"hr")
                        || tag_name.eq_ignore_ascii_case(b"img")
                        || tag_name.eq_ignore_ascii_case(b"input");

                    let remove_end = if is_self_closing {
                        tag_end
                    } else {
                        // Find the closing tag
                        find_closing_tag_bytes(bytes, tag_end, tag_name).unwrap_or(tag_end)
                    };

                    let out = output.get_or_insert_with(|| String::with_capacity(len));
                    out.push_str(&input[last..idx]);
                    last = remove_end;
                    idx = remove_end;
                    continue;
                }
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

/// Check if an opening tag string contains the `hidden` attribute.
///
/// Handles: `hidden`, `hidden=""`, `hidden="hidden"`, `hidden="true"`.
/// Does NOT match attributes like `data-hidden` or `aria-hidden`.
fn tag_has_hidden_attribute(tag: &str) -> bool {
    let bytes = tag.as_bytes();
    let len = bytes.len();
    let needle = b"hidden";
    let nlen = needle.len();

    let mut i = 0;
    // Skip past the tag name
    while i < len && bytes[i] != b' ' && bytes[i] != b'\t' && bytes[i] != b'\n' && bytes[i] != b'>' {
        i += 1;
    }

    while i + nlen <= len {
        if bytes[i..i + nlen].eq_ignore_ascii_case(needle) {
            // Check that the character before is whitespace (attribute boundary)
            let before_ok = i == 0 || bytes[i - 1].is_ascii_whitespace();
            // Check that the character after is whitespace, '>', '=', or '/'
            let after = bytes.get(i + nlen).copied();
            let after_ok = matches!(after, None | Some(b' ' | b'\t' | b'\n' | b'\r' | b'>' | b'=' | b'/'));
            if before_ok && after_ok {
                return true;
            }
        }
        i += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::{
        normalize_bogus_comment_endings, normalize_split_closing_tags, normalize_unclosed_list_items,
        sanitize_markdown_url,
    };

    // ── normalize_bogus_comment_endings ───────────────────────────────────────

    #[test]
    fn normalize_bogus_comment_endings_leaves_well_formed_comment_unchanged() {
        let input = "<p>A</p><!-- foo --><p>B</p>";
        let result = normalize_bogus_comment_endings(input);
        // Borrowed means unchanged
        assert_eq!(result.as_ref(), input);
    }

    #[test]
    fn normalize_bogus_comment_endings_rewrites_triple_dash_close() {
        let input = "<!-- foo --->";
        let result = normalize_bogus_comment_endings(input);
        assert_eq!(result.as_ref(), "<!-- foo -->");
    }

    #[test]
    fn normalize_bogus_comment_endings_rewrites_four_dash_close() {
        let input = "<!-- foo ---->";
        let result = normalize_bogus_comment_endings(input);
        assert_eq!(result.as_ref(), "<!-- foo -->");
    }

    #[test]
    fn normalize_bogus_comment_endings_preserves_content_after_comment() {
        let input = "<h1>One</h1><!-- /// ---><p>Two</p>";
        let result = normalize_bogus_comment_endings(input);
        assert_eq!(result.as_ref(), "<h1>One</h1><!-- /// --><p>Two</p>");
    }

    #[test]
    fn normalize_bogus_comment_endings_handles_multiple_bogus_comments() {
        let input = "<p>A</p><!-- x ---><p>B</p><!-- y ----><p>C</p>";
        let result = normalize_bogus_comment_endings(input);
        assert_eq!(result.as_ref(), "<p>A</p><!-- x --><p>B</p><!-- y --><p>C</p>");
    }

    #[test]
    fn normalize_bogus_comment_endings_handles_no_comments() {
        let input = "<p>Just a paragraph</p>";
        let result = normalize_bogus_comment_endings(input);
        assert_eq!(result.as_ref(), input);
    }

    #[test]
    fn normalize_bogus_comment_endings_empty_input() {
        let result = normalize_bogus_comment_endings("");
        assert_eq!(result.as_ref(), "");
    }

    // ── normalize_split_closing_tags ──────────────────────────────────────────

    #[test]
    fn normalize_split_closing_tags_collapses_newline_before_close_bracket() {
        let input = "<a href=\"#x\">text</a\n>";
        let result = normalize_split_closing_tags(input);
        assert_eq!(result.as_ref(), "<a href=\"#x\">text</a>");
    }

    #[test]
    fn normalize_split_closing_tags_collapses_indented_newline_before_close_bracket() {
        let input = "<a href=\"#x\">text</a\n  >";
        let result = normalize_split_closing_tags(input);
        assert_eq!(result.as_ref(), "<a href=\"#x\">text</a>");
    }

    #[test]
    fn normalize_split_closing_tags_leaves_well_formed_closing_tags_unchanged() {
        let input = "<a href=\"#x\">text</a>";
        let result = normalize_split_closing_tags(input);
        assert_eq!(result.as_ref(), input);
    }

    #[test]
    fn normalize_split_closing_tags_handles_multiple_split_closing_tags() {
        let input = "<li><a href=\"#a\">A</a\n  >\n<a href=\"#b\">B</a\n>";
        let result = normalize_split_closing_tags(input);
        assert_eq!(result.as_ref(), "<li><a href=\"#a\">A</a>\n<a href=\"#b\">B</a>");
    }

    #[test]
    fn normalize_split_closing_tags_does_not_collapse_inline_whitespace() {
        // Only newlines trigger the normalisation; spaces alone must not.
        let input = "<a href=\"#x\">text</a >";
        let result = normalize_split_closing_tags(input);
        // A space before > is actually valid HTML and tl handles it fine.
        // We must not touch it to avoid over-normalising.
        assert_eq!(result.as_ref(), input);
    }

    #[test]
    fn normalize_split_closing_tags_empty_input() {
        let result = normalize_split_closing_tags("");
        assert_eq!(result.as_ref(), "");
    }

    // ── sanitize_markdown_url ─────────────────────────────────────────────────

    #[test]
    fn sanitize_markdown_url_extracts_scheme_relative_markdown_like_url() {
        let input = "//[p1.zemanta.com/v2/p/ns/45625/PAGE\\_VIEW/](http://p1.zemanta.com/v2/p/ns/45625/PAGE_VIEW/)";
        let sanitized = sanitize_markdown_url(input);
        assert_eq!(sanitized, "http://p1.zemanta.com/v2/p/ns/45625/PAGE_VIEW/");
    }

    #[test]
    fn sanitize_markdown_url_extracts_standard_markdown_like_url() {
        let input = "[label](https://example.com/path?q=1)";
        let sanitized = sanitize_markdown_url(input);
        assert_eq!(sanitized, "https://example.com/path?q=1");
    }

    #[test]
    fn sanitize_markdown_url_leaves_normal_urls_unchanged() {
        let input = "https://example.com/normal";
        let sanitized = sanitize_markdown_url(input);
        assert_eq!(sanitized, input);
    }

    // ── normalize_unclosed_list_items ─────────────────────────────────────────

    #[test]
    fn normalize_unclosed_list_items_leaves_well_formed_list_unchanged() {
        let input = "<ul><li>A</li><li>B</li></ul>";
        let result = normalize_unclosed_list_items(input);
        assert_eq!(result.as_ref(), input);
    }

    #[test]
    fn normalize_unclosed_list_items_closes_unclosed_li_before_next_li() {
        let input = "<ul><li>A<li>B</ul>";
        let result = normalize_unclosed_list_items(input);
        assert_eq!(result.as_ref(), "<ul><li>A</li><li>B</li></ul>");
    }

    #[test]
    fn normalize_unclosed_list_items_closes_chain_of_unclosed_li() {
        let input = "<ul><li>A<li>B<li>C</ul>";
        let result = normalize_unclosed_list_items(input);
        assert_eq!(result.as_ref(), "<ul><li>A</li><li>B</li><li>C</li></ul>");
    }

    #[test]
    fn normalize_unclosed_list_items_does_not_modify_input_without_list_items() {
        let input = "<p>Hello</p><div>World</div>";
        let result = normalize_unclosed_list_items(input);
        // Fast-path: no list-item tags → Borrowed
        assert!(matches!(result, std::borrow::Cow::Borrowed(_)));
    }

    #[test]
    fn normalize_unclosed_list_items_handles_nested_list_correctly() {
        // Nested ul: the inner li items should not close prematurely via the outer li
        let input = "<ul><li>Outer<ul><li>Inner A<li>Inner B</ul><li>Outer B</ul>";
        let result = normalize_unclosed_list_items(input);
        assert_eq!(
            result.as_ref(),
            "<ul><li>Outer<ul><li>Inner A</li><li>Inner B</li></ul></li><li>Outer B</li></ul>"
        );
    }

    #[test]
    fn normalize_unclosed_list_items_handles_dt_and_dd() {
        let input = "<dl><dt>Term A<dd>Def A<dt>Term B<dd>Def B</dl>";
        let result = normalize_unclosed_list_items(input);
        assert_eq!(
            result.as_ref(),
            "<dl><dt>Term A</dt><dd>Def A</dd><dt>Term B</dt><dd>Def B</dd></dl>"
        );
    }

    #[test]
    fn normalize_unclosed_list_items_does_not_touch_content_in_pre() {
        // Content inside <pre> must be passed through verbatim even if it looks like a tag.
        let input = "<ul><li>A<pre><li>not-a-list-item</pre><li>B</ul>";
        let result = normalize_unclosed_list_items(input);
        assert_eq!(
            result.as_ref(),
            "<ul><li>A<pre><li>not-a-list-item</pre></li><li>B</li></ul>"
        );
    }

    #[test]
    fn normalize_unclosed_list_items_skips_html_comments() {
        let input = "<ul><li>A<!-- <li>comment --><li>B</ul>";
        let result = normalize_unclosed_list_items(input);
        assert_eq!(result.as_ref(), "<ul><li>A<!-- <li>comment --></li><li>B</li></ul>");
    }

    #[test]
    fn normalize_unclosed_list_items_empty_input() {
        let result = normalize_unclosed_list_items("");
        assert_eq!(result.as_ref(), "");
    }
}
