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
        // Numbered-list escape: `.` / `)` are escaped iff the previous input
        // byte is an ASCII digit.  Read from `bytes`, not from `dest` — at
        // this point in the loop the digit is still inside the pending run
        // `bytes[run_start..i]` and has not been flushed.
        let needs_numbered = escape_misc && (b == b'.' || b == b')') && i > 0 && bytes[i - 1].is_ascii_digit();
        let needs_star = escape_asterisks && b == b'*';
        let needs_under = escape_underscores && b == b'_';
        if needs_misc || needs_numbered || needs_star || needs_under {
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

    if !escape_misc && !escape_asterisks && !escape_underscores && !escape_ascii {
        return Cow::Borrowed(text);
    }

    // Single-pass scan to determine whether any byte needs escaping.  Returns
    // `Cow::Borrowed` immediately when the answer is no, avoiding the
    // destination String allocation entirely for the common "clean" case.
    let needs_any = text.as_bytes().iter().any(|&b| {
        if escape_ascii {
            return is_ascii_punct(b);
        }
        (escape_misc && (is_misc_escape(b) || b == b'.' || b == b')'))
            || (escape_asterisks && b == b'*')
            || (escape_underscores && b == b'_')
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
    // ASCII fast path: most real-world text is ASCII-only.  Walking bytes
    // avoids `char` decoding and lets the loop be tight.  Any non-ASCII byte
    // (>= 0x80) is a continuation byte for a multi-byte codepoint which
    // could be a Unicode space; fall back to the char-aware path in that
    // case.
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
    html_escape::decode_html_entities(text).into_owned()
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

    html_escape::decode_html_entities(text)
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
    fn test_chomp() {
        assert_eq!(chomp("  text  "), (" ", " ", "text"));
        assert_eq!(chomp("text"), ("", "", "text"));
        assert_eq!(chomp(" text"), (" ", "", "text"));
        assert_eq!(chomp("text "), ("", " ", "text"));
        assert_eq!(chomp(""), ("", "", ""));
    }
}
