//! Text processing utilities for Markdown conversion.

use std::borrow::Cow;

/// Returns true when the byte is one of the misc-escape characters:
/// `\` `&` `<` `` ` `` `[` `]` `>` `~` `#` `=` `+` `|` `-`.
#[inline]
const fn is_misc_escape(b: u8) -> bool {
    matches!(
        b,
        b'\\' | b'&' | b'<' | b'`' | b'[' | b']' | b'>' | b'~' | b'#' | b'=' | b'+' | b'|' | b'-'
    )
}

/// Returns true when a literal backslash at `bytes[i]` must be doubled so a
/// `CommonMark` parser reading the output back sees a literal backslash rather than
/// an escape-sequence trigger.
///
/// `CommonMark` only assigns meaning to a backslash when it precedes ASCII punctuation
/// (spec §2.4, "Backslash escapes") — there the backslash is *consumed* by the parser,
/// so leaving it bare silently loses a character from the source text. A backslash
/// before anything else is already literal and is left alone, with two exceptions
/// where the run boundary itself gives the byte a meaning it did not have in the
/// source:
///
/// - immediately before a line ending, which is `CommonMark`'s hard-line-break syntax
///   (and this crate's own `\\\n` hard-break marker under
///   [`NewlineStyle::Backslash`](crate::options::NewlineStyle));
/// - at the very end of the run, where whatever the emitter appends next (a closing
///   delimiter, a table cell separator, a line ending) would otherwise become the
///   backslash's escape target.
///
/// The rule is deliberately independent of `escape_misc`/`escape_asterisks`/
/// `escape_underscores`/`escape_ascii`: those flags choose how aggressively to
/// neutralise Markdown syntax, whereas this one preserves a byte that was present in
/// the source. `escape_markdown_title` in `converter/inline/link.rs` already escapes
/// backslashes unconditionally for the same reason. ~keep
#[inline]
pub const fn backslash_needs_escape(bytes: &[u8], i: usize) -> bool {
    if i + 1 >= bytes.len() {
        return true;
    }
    let next = bytes[i + 1];
    matches!(next, b'\n' | b'\r') || is_ascii_punct(next)
}

/// Returns true when the byte is one of the CommonMark ASCII-punctuation
/// characters that `escape_ascii` requests backslash-escaping for.
#[inline]
const fn is_ascii_punct(b: u8) -> bool {
    matches!(
        b,
        b'!' | b'"'
            | b'#'
            | b'$'
            | b'%'
            | b'&'
            | b'\''
            | b'('
            | b')'
            | b'*'
            | b'+'
            | b','
            | b'-'
            | b'.'
            | b'/'
            | b':'
            | b';'
            | b'<'
            | b'='
            | b'>'
            | b'?'
            | b'@'
            | b'['
            | b'\\'
            | b']'
            | b'^'
            | b'_'
            | b'`'
            | b'{'
            | b'|'
            | b'}'
            | b'~'
    )
}

/// Append the escaped form of `text` to `dest` in a single pass.
///
/// Replaces the previous regex-based pipeline (three sequential `regex::replace_all`
/// calls plus two `String::replace` calls).  All escape flags are honoured in one
/// byte walk; runs of non-special bytes are bulk-copied via `push_str` so multi-byte
/// UTF-8 codepoints flow through unchanged without per-byte char conversion.
///
/// Callers that need a `Cow` return type should use `escape` instead.
#[allow(clippy::fn_params_excessive_bools)]
pub fn escape_into(
    dest: &mut String,
    text: &str,
    escape_misc: bool,
    escape_asterisks: bool,
    escape_underscores: bool,
    escape_ascii: bool,
) {
    if text.is_empty() {
        return;
    }
    if escape_ascii {
        escape_ascii_into(dest, text);
        return;
    }
    let bytes = text.as_bytes();
    let mut run_start = 0;
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        let needs_misc = escape_misc && is_misc_escape(b);
        let needs_numbered = escape_misc && (b == b'.' || b == b')') && i > 0 && bytes[i - 1].is_ascii_digit();
        let needs_star = escape_asterisks && b == b'*';
        let needs_under = escape_underscores && b == b'_';
        let needs_backslash = b == b'\\' && backslash_needs_escape(bytes, i);
        if needs_misc || needs_numbered || needs_star || needs_under || needs_backslash {
            if i > run_start {
                dest.push_str(&text[run_start..i]);
            }
            dest.push('\\');
            dest.push(b as char);
            i += 1;
            run_start = i;
        } else {
            i += 1;
        }
    }
    if i > run_start {
        dest.push_str(&text[run_start..]);
    }
}

/// Append the `escape_ascii` form of `text` to `dest` in a single pass.
///
/// Every byte in `is_ascii_punct` is prefixed with `\`.  Non-ASCII bytes
/// (UTF-8 continuation bytes for multi-byte codepoints) flow through
/// unchanged in bulk runs.
fn escape_ascii_into(dest: &mut String, text: &str) {
    let bytes = text.as_bytes();
    let mut run_start = 0;
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if is_ascii_punct(b) {
            if i > run_start {
                dest.push_str(&text[run_start..i]);
            }
            dest.push('\\');
            dest.push(b as char);
            i += 1;
            run_start = i;
        } else {
            i += 1;
        }
    }
    if i > run_start {
        dest.push_str(&text[run_start..]);
    }
}

/// Escape Markdown special characters in text.
///
/// A literal backslash is escaped regardless of every flag below whenever leaving it
/// bare would change its meaning once the output is re-parsed as `CommonMark` — see
/// [`backslash_needs_escape`] for the exact rule.
///
/// # Arguments
///
/// * `text` - Text to escape
/// * `escape_misc` - Escape miscellaneous characters (`\` `&` `<` `` ` `` `[` `>` `~` `#` `=` `+` `|` `-`)
/// * `escape_asterisks` - Escape asterisks (`*`)
/// * `escape_underscores` - Escape underscores (`_`)
/// * `escape_ascii` - Escape all ASCII punctuation (for `CommonMark` spec compliance)
///
/// # Returns
///
/// Escaped text — `Cow::Borrowed(text)` when no escaping was necessary, otherwise
/// `Cow::Owned` containing the escaped string.
#[allow(clippy::fn_params_excessive_bools)]
pub fn escape(
    text: &str,
    escape_misc: bool,
    escape_asterisks: bool,
    escape_underscores: bool,
    escape_ascii: bool,
) -> Cow<'_, str> {
    if text.is_empty() {
        return Cow::Borrowed("");
    }

    // ~keep Backslash escaping is not gated by any flag, so the all-flags-false
    // ~keep shortcut must first confirm there is no backslash to consider.
    if !escape_misc && !escape_asterisks && !escape_underscores && !escape_ascii && !text.contains('\\') {
        return Cow::Borrowed(text);
    }

    let bytes = text.as_bytes();
    let needs_any = bytes.iter().enumerate().any(|(i, &b)| {
        if escape_ascii {
            return is_ascii_punct(b);
        }
        (escape_misc && (is_misc_escape(b) || b == b'.' || b == b')'))
            || (escape_asterisks && b == b'*')
            || (escape_underscores && b == b'_')
            || (b == b'\\' && backslash_needs_escape(bytes, i))
    });
    if !needs_any {
        return Cow::Borrowed(text);
    }

    let mut dest = String::with_capacity(text.len() + 8);
    escape_into(
        &mut dest,
        text,
        escape_misc,
        escape_asterisks,
        escape_underscores,
        escape_ascii,
    );
    Cow::Owned(dest)
}

/// Extract boundary whitespace from text (chomp).
///
/// Returns (prefix, suffix, `trimmed_text`) tuple.
/// Prefix/suffix are " " if original text had leading/trailing whitespace.
/// However, suffix is "" if the trailing whitespace is only newlines (not spaces/tabs).
/// This prevents trailing newlines from becoming trailing spaces in the output.
/// The trimmed text has all leading/trailing whitespace removed.
#[must_use]
pub fn chomp(text: &str) -> (&str, &str, &str) {
    if text.is_empty() {
        return ("", "", "");
    }

    let prefix = if text.starts_with(|c: char| c.is_whitespace()) {
        " "
    } else {
        ""
    };

    let suffix = if text.ends_with("\n\n") || text.ends_with("\r\n\r\n") {
        "\n\n"
    } else if text.ends_with([' ', '\t']) {
        " "
    } else {
        ""
    };

    let trimmed = if suffix == "\n\n" {
        text.trim_end_matches("\n\n").trim_end_matches("\r\n\r\n").trim()
    } else {
        text.trim()
    };

    (prefix, suffix, trimmed)
}

/// Normalize whitespace by collapsing consecutive spaces and tabs.
///
/// Multiple spaces and tabs are replaced with a single space.
/// Newlines are preserved.
/// Unicode spaces are normalized to ASCII spaces.
///
/// # Arguments
///
/// * `text` - The text to normalize
///
/// # Returns
///
/// Normalized text with collapsed spaces/tabs but preserved newlines
#[must_use]
pub fn normalize_whitespace(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut prev_was_space = false;

    for ch in text.chars() {
        let is_space = ch == ' ' || ch == '\t' || is_unicode_space(ch);

        if is_space {
            if !prev_was_space {
                result.push(' ');
                prev_was_space = true;
            }
        } else {
            result.push(ch);
            prev_was_space = false;
        }
    }

    result
}

/// Normalize whitespace in text, returning borrowed or owned result as needed.
///
/// This function optimizes memory by returning a borrowed reference when no normalization
/// is needed, and only allocating a new string when whitespace changes are necessary.
///
/// Multiple consecutive spaces, tabs, and Unicode space characters are replaced with
/// a single ASCII space. Newlines are preserved as-is.
///
/// # Arguments
///
/// * `text` - The text to normalize
///
/// # Returns
///
/// `Cow::Borrowed` if text is already normalized, or `Cow::Owned` with normalized text
#[must_use]
pub fn normalize_whitespace_cow(text: &str) -> Cow<'_, str> {
    let bytes = text.as_bytes();
    let mut prev_was_space = false;
    for &b in bytes {
        if b >= 0x80 {
            return normalize_whitespace_cow_slow(text);
        }
        let is_space = b == b' ' || b == b'\t';
        if is_space {
            if prev_was_space || b != b' ' {
                return Cow::Owned(normalize_whitespace(text));
            }
            prev_was_space = true;
        } else {
            prev_was_space = false;
        }
    }
    Cow::Borrowed(text)
}

/// Char-aware fallback path used when the input contains non-ASCII bytes.
/// Mirrors the previous behaviour exactly.
#[cold]
fn normalize_whitespace_cow_slow(text: &str) -> Cow<'_, str> {
    let mut prev_was_space = false;
    for ch in text.chars() {
        let is_space = ch == ' ' || ch == '\t' || is_unicode_space(ch);
        if is_space {
            if prev_was_space || ch != ' ' {
                return Cow::Owned(normalize_whitespace(text));
            }
            prev_was_space = true;
        } else {
            prev_was_space = false;
        }
    }
    Cow::Borrowed(text)
}

/// Normalize whitespace for a text node's already-trimmed core content, for the case
/// where an embedded `\n` is kept as a literal newline in the Markdown output.
///
/// Identical to [`normalize_whitespace_cow`] except for one case: a run of spaces/tabs
/// that immediately follows a newline collapses to nothing instead of to a single space.
///
/// ~keep Collapsing to one space is not a fixed point here. A CommonMark-compliant parser
/// ~keep forms a paragraph's raw content by removing each line's leading whitespace
/// ~keep entirely (spec 4.9) regardless of how many columns of indentation it had, so a
/// ~keep single space we chose to keep is silently dropped by the very first round trip
/// ~keep through a real renderer. That made `normalize_whitespace_cow` shrink such content
/// ~keep by one more space every pass until it reached zero (`CommonMark` spec example 182:
/// ~keep `<![CDATA[...]]>` whose body is indented source lines). Dropping straight to zero
/// ~keep matches what the round trip already forces, so it is stable on the very first pass.
/// ~keep Also sidesteps the four-space indented-code-block threshold: since no continuation
/// ~keep line is ever left with 1-4+ leading spaces, none can be reinterpreted as an
/// ~keep indented code block after a blank line splits the text into separate paragraphs on
/// ~keep re-parse.
///
/// ~keep Callers MUST pass text that already has its own leading/trailing whitespace
/// ~keep trimmed off (e.g. `chomp()`'s `core`, or `str::trim()`) rather than a raw,
/// ~keep untrimmed text node. The "collapse to nothing" rule applies only to a run that
/// ~keep sits strictly between two pieces of real content; at the text node's own edge, a
/// ~keep trailing `\n` + spaces is not an in-Markdown line break at all -- it is folded by
/// ~keep `text_node.rs`'s prefix/suffix handling into a plain word-separating space (or
/// ~keep dropped), so no literal newline byte survives there for a reparse to disagree
/// ~keep about. Running this function over that edge too would delete a space `chomp`'s
/// ~keep suffix computation still expects to find, concatenating words that must stay
/// ~keep separated (e.g. `<p>of\n  <kbd>#</kbd></p>` losing the space before `` `#` ``).
#[must_use]
pub fn normalize_block_whitespace_cow(text: &str) -> Cow<'_, str> {
    let bytes = text.as_bytes();
    let mut prev_was_space = false;
    let mut at_line_start = false;
    for &b in bytes {
        if b >= 0x80 {
            return Cow::Owned(normalize_block_whitespace(text));
        }
        if b == b'\n' {
            prev_was_space = false;
            at_line_start = true;
            continue;
        }
        let is_space = b == b' ' || b == b'\t';
        if is_space {
            if at_line_start || prev_was_space {
                return Cow::Owned(normalize_block_whitespace(text));
            }
            prev_was_space = true;
        } else {
            prev_was_space = false;
            at_line_start = false;
        }
    }
    Cow::Borrowed(text)
}

/// Char-aware implementation backing [`normalize_block_whitespace_cow`]'s owned path.
///
/// Handles Unicode space characters the same way [`normalize_whitespace`] does, in addition
/// to dropping a post-newline leading run entirely.
#[cold]
fn normalize_block_whitespace(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut prev_was_space = false;
    let mut at_line_start = false;
    for ch in text.chars() {
        if ch == '\n' {
            result.push(ch);
            prev_was_space = false;
            at_line_start = true;
            continue;
        }
        let is_space = ch == ' ' || ch == '\t' || is_unicode_space(ch);
        if is_space {
            if at_line_start {
                continue;
            }
            if !prev_was_space {
                result.push(' ');
                prev_was_space = true;
            }
        } else {
            result.push(ch);
            prev_was_space = false;
            at_line_start = false;
        }
    }
    result
}

/// Normalize whitespace inside a Markdown table cell.
///
/// A table cell cannot contain a hard line break, so unlike
/// [`normalize_whitespace_cow`] — which preserves `\n` for block-level
/// rendering — this also folds `\n` and `\r` into the run before collapsing
/// consecutive whitespace to a single ASCII space (issue #453).
#[must_use]
pub fn normalize_cell_whitespace_cow(text: &str) -> Cow<'_, str> {
    if !text.contains('\n') && !text.contains('\r') {
        return normalize_whitespace_cow(text);
    }
    let folded = text.replace(['\n', '\r'], " ");
    Cow::Owned(normalize_whitespace(&folded))
}

/// Fold raw line breaks to a single space inside verbatim (code/ruby) table-cell content.
///
/// A GFM table cell cannot contain a literal newline, but code and ruby content must
/// otherwise stay byte-for-byte verbatim (issue #455): unlike
/// [`normalize_cell_whitespace_cow`], this does not collapse any other whitespace run —
/// only `"\r\n"`, `"\n"`, and `"\r"` are each replaced with one ASCII space.
#[must_use]
pub fn fold_cell_line_breaks_verbatim_cow(text: &str) -> Cow<'_, str> {
    if !text.contains('\n') && !text.contains('\r') {
        return Cow::Borrowed(text);
    }
    let mut folded = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '\r' => {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                folded.push(' ');
            }
            '\n' => folded.push(' '),
            other => folded.push(other),
        }
    }
    Cow::Owned(folded)
}

/// Decode common HTML entities.
///
/// Decodes the most common HTML entities to their character equivalents:
/// - `&quot;` → `"`
/// - `&apos;` → `'`
/// - `&lt;` → `<`
/// - `&gt;` → `>`
/// - `&amp;` → `&` (must be last to avoid double-decoding)
///
/// # Arguments
///
/// * `text` - Text containing HTML entities
///
/// # Returns
///
/// Text with entities decoded
#[must_use]
pub fn decode_html_entities(text: &str) -> String {
    let overridden = apply_numeric_character_reference_overrides(text);
    html_escape::decode_html_entities(overridden.as_ref()).into_owned()
}

/// Parses a numeric character reference's digit run into its numeric value.
///
/// `digits` must be non-empty; `radix` is 10 (decimal `&#...;`) or 16 (hex `&#x...;`).
/// Returns `Err(())` when `digits` contains a character invalid for `radix` -- a
/// genuinely malformed reference, which callers treat as "leave the raw text alone".
/// A digit run that is syntactically valid but too large to fit `u64` returns
/// `Ok(u64::MAX)` rather than erroring: that value safely clears every threshold
/// [`numeric_character_reference_override`] checks (0x10FFFF), so an
/// arbitrarily-long digit run is handled exactly like any other clearly-out-of-range
/// value instead of silently wrapping.
pub fn parse_character_reference_number(digits: &str, radix: u32) -> Result<u64, ()> {
    match u64::from_str_radix(digits, radix) {
        Ok(number) => Ok(number),
        Err(err) if *err.kind() == std::num::IntErrorKind::PosOverflow => Ok(u64::MAX),
        Err(_) => Err(()),
    }
}

/// Maps a numeric character reference's value to the code point the WHATWG "numeric
/// character reference end state" algorithm requires, for the cases where that
/// differs from the naive `char::from_u32(value)` interpretation used everywhere else
/// in this crate: the null character, values outside the Unicode range, surrogates,
/// and the 0x80-0x9F Windows-1252 replacement table. Returns `None` when the naive
/// interpretation is already correct, which covers the overwhelming majority of
/// values -- callers fall back to `char::from_u32` in that case.
///
/// <https://html.spec.whatwg.org/multipage/parsing.html#numeric-character-reference-end-state>
#[must_use]
pub fn numeric_character_reference_override(value: u64) -> Option<char> {
    if value == 0 || value > 0x0010_FFFF || (0xD800..=0xDFFF).contains(&value) {
        return Some('\u{FFFD}');
    }

    let replacement = match value {
        0x80 => '\u{20AC}',
        0x82 => '\u{201A}',
        0x83 => '\u{0192}',
        0x84 => '\u{201E}',
        0x85 => '\u{2026}',
        0x86 => '\u{2020}',
        0x87 => '\u{2021}',
        0x88 => '\u{02C6}',
        0x89 => '\u{2030}',
        0x8A => '\u{0160}',
        0x8B => '\u{2039}',
        0x8C => '\u{0152}',
        0x8E => '\u{017D}',
        0x91 => '\u{2018}',
        0x92 => '\u{2019}',
        0x93 => '\u{201C}',
        0x94 => '\u{201D}',
        0x95 => '\u{2022}',
        0x96 => '\u{2013}',
        0x97 => '\u{2014}',
        0x98 => '\u{02DC}',
        0x99 => '\u{2122}',
        0x9A => '\u{0161}',
        0x9B => '\u{203A}',
        0x9C => '\u{0153}',
        0x9E => '\u{017E}',
        0x9F => '\u{0178}',
        _ => return None,
    };
    Some(replacement)
}

/// Recognizes a well-formed `&#...;` / `&#x...;` numeric character reference starting
/// at `bytes[amp]` (expected to be `&`), using exactly the syntax `html_escape`'s own
/// decoder accepts elsewhere in this module: a digit run terminated by `;`, with no
/// embedded `&` (which aborts the match so the caller retries from that `&` instead).
/// Returns the offset just past the terminating `;` and the parsed value, or `None`
/// if `bytes[amp]` does not begin such a reference.
fn scan_numeric_character_reference(bytes: &[u8], amp: usize) -> Option<(usize, u64)> {
    if bytes.get(amp + 1) != Some(&b'#') {
        return None;
    }

    let mut i = amp + 2;
    let is_hex = matches!(bytes.get(i), Some(b'x' | b'X'));
    if is_hex {
        i += 1;
    }

    let digits_start = i;
    loop {
        match bytes.get(i) {
            Some(b';') => break,
            Some(b'&') | None => return None,
            Some(_) => i += 1,
        }
    }

    let digits = &bytes[digits_start..i];
    let is_valid_digit: fn(&u8) -> bool = if is_hex {
        u8::is_ascii_hexdigit
    } else {
        u8::is_ascii_digit
    };
    if digits.is_empty() || !digits.iter().all(is_valid_digit) {
        return None;
    }

    // SAFETY-ish: `digits` was just validated as ASCII hex/decimal digits above. ~keep
    let digits_str = std::str::from_utf8(digits).unwrap_or_default();
    let radix = if is_hex { 16 } else { 10 };
    let value = parse_character_reference_number(digits_str, radix).ok()?;
    Some((i + 1, value))
}

/// Rewrites the numeric character references in `text` whose value
/// [`numeric_character_reference_override`] maps differently than `html_escape`'s own
/// decoder would (null, out-of-range, surrogates, and the 0x80-0x9F table), leaving
/// every other numeric reference and every named entity untouched for `html_escape`
/// to decode as before.
///
/// ~keep Safe to run as an isolated pre-pass rather than folding into a single scan:
/// ~keep every override character is outside the ASCII set that gives `&`, digits,
/// ~keep `x`/`X`, and `;` their meaning in entity syntax, so substituting one can never
/// ~keep create or extend a sequence for the later `html_escape` pass to (mis)decode.
fn apply_numeric_character_reference_overrides(text: &str) -> Cow<'_, str> {
    let bytes = text.as_bytes();
    if !bytes.contains(&b'&') {
        return Cow::Borrowed(text);
    }

    let mut out = String::new();
    let mut last = 0;
    let mut i = 0;
    let mut changed = false;
    while let Some(rel) = memchr::memchr(b'&', &bytes[i..]) {
        let amp = i + rel;
        match scan_numeric_character_reference(bytes, amp) {
            Some((end, value)) => {
                if let Some(replacement) = numeric_character_reference_override(value) {
                    out.push_str(&text[last..amp]);
                    out.push(replacement);
                    last = end;
                    changed = true;
                }
                i = end;
            }
            None => i = amp + 1,
        }
    }

    if !changed {
        return Cow::Borrowed(text);
    }
    out.push_str(&text[last..]);
    Cow::Owned(out)
}

/// Decode HTML entities in text, returning borrowed or owned result as needed.
///
/// This function optimizes memory by returning a borrowed reference when no HTML
/// entities are present, and only allocating a new string when entity decoding
/// is necessary.
///
/// Decodes common HTML entities like:
/// - `&quot;` → `"`
/// - `&apos;` → `'`
/// - `&lt;` → `<`
/// - `&gt;` → `>`
/// - `&amp;` → `&` (decoded last to avoid double-decoding)
///
/// # Arguments
///
/// * `text` - Text potentially containing HTML entities
///
/// # Returns
///
/// `Cow::Borrowed` if no entities found, or `Cow::Owned` with entities decoded
#[must_use]
pub fn decode_html_entities_cow(text: &str) -> Cow<'_, str> {
    if !text.contains('&') {
        return Cow::Borrowed(text);
    }

    match apply_numeric_character_reference_overrides(text) {
        Cow::Borrowed(unchanged) => html_escape::decode_html_entities(unchanged),
        Cow::Owned(overridden) => Cow::Owned(html_escape::decode_html_entities(&overridden).into_owned()),
    }
}

/// Check if a character is a unicode space character.
///
/// Includes: non-breaking space, various width spaces, etc.
const fn is_unicode_space(ch: char) -> bool {
    matches!(
        ch,
        '\u{00A0}'
            | '\u{1680}'
            | '\u{2000}'
            | '\u{2001}'
            | '\u{2002}'
            | '\u{2003}'
            | '\u{2004}'
            | '\u{2005}'
            | '\u{2006}'
            | '\u{2007}'
            | '\u{2008}'
            | '\u{2009}'
            | '\u{200A}'
            | '\u{202F}'
            | '\u{205F}'
            | '\u{3000}'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_misc() {
        assert_eq!(escape("foo & bar", true, false, false, false), r"foo \& bar");
        assert_eq!(escape("foo [bar]", true, false, false, false), r"foo \[bar\]");
        assert_eq!(escape("1. Item", true, false, false, false), r"1\. Item");
        assert_eq!(escape("1) Item", true, false, false, false), r"1\) Item");
    }

    #[test]
    fn test_escape_asterisks() {
        assert_eq!(escape("foo * bar", false, true, false, false), r"foo \* bar");
        assert_eq!(escape("**bold**", false, true, false, false), r"\*\*bold\*\*");
    }

    #[test]
    fn test_escape_underscores() {
        assert_eq!(escape("foo_bar", false, false, true, false), r"foo\_bar");
        assert_eq!(escape("__bold__", false, false, true, false), r"\_\_bold\_\_");
    }

    #[test]
    fn test_escape_ascii() {
        assert_eq!(escape(r##"!"#$%&"##, false, false, false, true), r#"\!\"\#\$\%\&"#);
        assert_eq!(escape("*+,-./", false, false, false, true), r"\*\+\,\-\.\/");
        assert_eq!(escape("<=>?@", false, false, false, true), r"\<\=\>\?\@");
        assert_eq!(escape(r"[\]^_`", false, false, false, true), r"\[\\\]\^\_\`");
        assert_eq!(escape("{|}~", false, false, false, true), r"\{\|\}\~");
    }

    #[test]
    fn should_escape_backslash_before_ascii_punctuation_even_with_all_flags_false() {
        // ~keep CommonMark example 15: a bare `\` before punctuation is consumed by the
        // ~keep parser on re-parse, so the source character is lost unless it is doubled.
        assert_eq!(escape(r"a\*b", false, false, false, false), r"a\\*b");
        assert_eq!(escape(r"3\.14", false, false, false, false), r"3\\.14");
        assert_eq!(escape(r"a\[b\]c", false, false, false, false), r"a\\[b\\]c");
    }

    #[test]
    fn should_not_escape_backslash_before_non_punctuation() {
        // ~keep CommonMark example 13: a `\` before a non-punctuation, non-line-ending
        // ~keep character is already literal, so doubling it would be pure noise.
        assert_eq!(escape(r"a\3b", false, false, false, false), r"a\3b");
        assert_eq!(escape(r"C:\Users\Alice", false, false, false, false), r"C:\Users\Alice");
        assert_eq!(escape("a\\ b", false, false, false, false), "a\\ b");
    }

    #[test]
    fn should_escape_backslash_at_end_of_text_run() {
        assert_eq!(escape(r"abc\", false, false, false, false), r"abc\\");
        assert_eq!(escape(r"\", false, false, false, false), r"\\");
    }

    #[test]
    fn should_escape_backslash_before_line_ending_to_avoid_hard_break_collision() {
        // ~keep A bare `\` immediately before a line ending is CommonMark's own
        // ~keep hard-line-break syntax, and is also this crate's `\\\n` hard-break marker
        // ~keep under NewlineStyle::Backslash.
        assert_eq!(escape("abc\\\ndef", false, false, false, false), "abc\\\\\ndef");
        assert_eq!(escape("abc\\\r\ndef", false, false, false, false), "abc\\\\\r\ndef");
    }

    #[test]
    fn should_escape_consecutive_backslashes_pairwise() {
        assert_eq!(escape(r"a\\b", false, false, false, false), r"a\\\b");
    }

    #[test]
    fn should_escape_backslash_independently_of_escape_misc() {
        // ~keep escape_misc does not gate `*`, so the result must equal the
        // ~keep all-flags-false one — proving the backslash rule is not folded into it.
        assert_eq!(escape(r"a\*b", true, false, false, false), r"a\\*b");
        // ~keep escape_misc still escapes every backslash on top of the rule; that
        // ~keep pre-existing behaviour is unchanged for callers that opt into it.
        assert_eq!(escape(r"a\3b", true, false, false, false), r"a\\3b");
    }

    #[test]
    fn should_borrow_unchanged_when_no_backslash_needs_escaping() {
        assert!(matches!(
            escape(r"a\3b", false, false, false, false),
            std::borrow::Cow::Borrowed(_)
        ));
        assert!(matches!(
            escape("plain", false, false, false, false),
            std::borrow::Cow::Borrowed(_)
        ));
    }

    #[test]
    fn test_chomp() {
        assert_eq!(chomp("  text  "), (" ", " ", "text"));
        assert_eq!(chomp("text"), ("", "", "text"));
        assert_eq!(chomp(" text"), (" ", "", "text"));
        assert_eq!(chomp("text "), ("", " ", "text"));
        assert_eq!(chomp(""), ("", "", ""));
    }
}
