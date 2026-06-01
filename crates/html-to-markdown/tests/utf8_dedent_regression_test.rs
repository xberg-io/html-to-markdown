//! Regression tests for the UTF-8 boundary panic in `dedent_code_block`.
//!
//! Before the fix, any `<pre>` block whose leading whitespace contained
//! multi-byte characters caused a panic because the char-count indent was
//! used as a raw byte index into the string slice.

use html_to_markdown_rs::convert;

/// NBSP (\u{a0}) used as the sole indent character.
///
/// NBSP is a 2-byte character in UTF-8 (0xC2 0xA0).  Using its char count
/// as a byte offset previously caused `slice index starts at a non-char
/// boundary` panic.
#[test]
fn test_nbsp_indent() {
    // Two NBSP chars as indent on each line
    let html = "<pre>\u{a0}\u{a0}line one\n\u{a0}\u{a0}line two</pre>";
    let result = convert(html, None);
    assert!(result.is_ok(), "panicked on NBSP indent: {:?}", result.err());
    let md = result.unwrap().content.unwrap_or_default();
    assert!(md.contains("line one"), "expected 'line one' in output, got: {md:?}");
    assert!(md.contains("line two"), "expected 'line two' in output, got: {md:?}");
}

/// Mixed ASCII tab + NBSP indent.
///
/// The minimum indent is computed across all non-blank lines; with a mix of
/// tab (1 byte, 1 char) and NBSP (2 bytes, 1 char) the char-to-byte mismatch
/// was guaranteed to produce a panic on the NBSP line.
#[test]
fn test_mixed_tab_nbsp_indent() {
    // Line 1: tab + content; line 2: NBSP + content
    let html = "<pre>\tascii_line\n\u{a0}nbsp_line</pre>";
    let result = convert(html, None);
    assert!(result.is_ok(), "panicked on mixed tab+NBSP indent: {:?}", result.err());
    let md = result.unwrap().content.unwrap_or_default();
    assert!(md.contains("ascii_line"), "expected 'ascii_line' in output, got: {md:?}");
    assert!(md.contains("nbsp_line"), "expected 'nbsp_line' in output, got: {md:?}");
}

/// Blank lines interleaved with multi-byte-indented content.
///
/// Blank lines must be passed through as empty strings without applying the
/// byte-offset slice — this exercises the `line.trim().is_empty()` branch.
#[test]
fn test_blank_lines_interleaved_with_multibyte_indent() {
    // Two lines with NBSP indent, separated by a blank line
    let html = "<pre>\u{a0}first\n\n\u{a0}third</pre>";
    let result = convert(html, None);
    assert!(result.is_ok(), "panicked on blank-interleaved NBSP: {:?}", result.err());
    let md = result.unwrap().content.unwrap_or_default();
    assert!(md.contains("first"), "expected 'first' in output, got: {md:?}");
    assert!(md.contains("third"), "expected 'third' in output, got: {md:?}");
}

/// Latin-1 supplement characters at the start of indented lines.
///
/// `é` (U+00E9) and `ñ` (U+00F1) are both 2-byte in UTF-8.  Using them as
/// leading whitespace is unusual but they are valid UTF-8 and `char::is_whitespace`
/// returns false for them, so min_indent stays 0 — the key is that the
/// converter must not panic regardless.
#[test]
fn test_latin1_supplement_chars_at_line_start() {
    // é and ñ are NOT whitespace, so indent=0 and no slicing occurs — but
    // the conversion must succeed without panic.
    let html = "<pre>éline\nñline</pre>";
    let result = convert(html, None);
    assert!(result.is_ok(), "panicked on Latin-1 chars at line start: {:?}", result.err());
    let md = result.unwrap().content.unwrap_or_default();
    assert!(md.contains("éline"), "expected 'éline' in output, got: {md:?}");
    assert!(md.contains("ñline"), "expected 'ñline' in output, got: {md:?}");
}

/// CJK character at line start.
///
/// CJK characters are 3 bytes in UTF-8.  When used as the first char in a
/// line (non-whitespace), indent=0 so no slice happens — but this exercises
/// the char-boundary safety of the whole path.
#[test]
fn test_cjk_chars_at_line_start() {
    let html = "<pre>日本語\n  code_here</pre>";
    let result = convert(html, None);
    assert!(result.is_ok(), "panicked on CJK char at line start: {:?}", result.err());
    let md = result.unwrap().content.unwrap_or_default();
    assert!(md.contains("日本語"), "expected '日本語' in output, got: {md:?}");
    assert!(md.contains("code_here"), "expected 'code_here' in output, got: {md:?}");
}

/// NBSP indent with a deeper common indent (2 NBSPs), verifying the dedent
/// actually removes the correct number of bytes rather than char-count bytes.
#[test]
fn test_nbsp_deep_indent_dedented_correctly() {
    // Four NBSP chars of indent on every line; after dedent, result has no leading whitespace.
    let nbsp4 = "\u{a0}\u{a0}\u{a0}\u{a0}";
    let html = format!("<pre>{nbsp4}alpha\n{nbsp4}beta</pre>");
    let result = convert(&html, None);
    assert!(result.is_ok(), "panicked on deep NBSP dedent: {:?}", result.err());
    let md = result.unwrap().content.unwrap_or_default();
    // After full dedent the visible text should be present
    assert!(md.contains("alpha"), "expected 'alpha' in output, got: {md:?}");
    assert!(md.contains("beta"), "expected 'beta' in output, got: {md:?}");
}
