// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression for #380: panic "byte index N is not a char boundary" when
//! `include_document_structure = true` and an inline element contains a
//! multibyte character preceded by a block that leaves a single trailing `\n`
//! in the output buffer.

use html_to_markdown_rs::ConversionOptions;

fn options_with_structure() -> ConversionOptions {
    ConversionOptions {
        include_document_structure: true,
        ..Default::default()
    }
}

/// The exact HTML from the bug report: `<pre>` → `<p><span>■…</span></p>`.
///
/// With `include_document_structure = true` the paragraph handler slices
/// `output[content_start_pos..]` after conversion. A `<span>` used to pop a
/// trailing `\n` before its content, which moved the end of the string behind
/// that start and put the slice inside a multibyte char. The span no longer
/// removes anything (#546); this test keeps the multibyte case from panicking.
#[test]
fn pre_paragraph_multibyte_span_does_not_panic() {
    let html = "<pre>previous block</pre>\n\
                <p><span style=\"letter-spacing: 0.0px;\">■Example request</span></p>\n\
                <p>Plain follow-up item</p>";

    let result = html_to_markdown_rs::convert(html, Some(options_with_structure()));
    assert!(result.is_ok(), "conversion must not panic or error: {:?}", result.err());
    let content = result.unwrap().content.unwrap_or_default();
    assert!(
        content.contains("■Example request"),
        "multibyte char must survive: {content}"
    );
    assert!(
        content.contains("Plain follow-up item"),
        "follow-up paragraph must be present: {content}"
    );
}

/// Same panic path but with a heading (`<h2>`) as the preceding block.
/// Headings emit a single `\n` in some configurations, which is what the span popped.
#[test]
fn heading_paragraph_multibyte_span_does_not_panic() {
    let html = "<h2>Section header</h2>\n\
                <p><span style=\"letter-spacing: 0.0px;\">■Example request</span></p>\n\
                <p>Plain follow-up item</p>";

    let result = html_to_markdown_rs::convert(html, Some(options_with_structure()));
    assert!(result.is_ok(), "conversion must not panic or error: {:?}", result.err());
    let content = result.unwrap().content.unwrap_or_default();
    assert!(
        content.contains("■Example request"),
        "multibyte char must survive: {content}"
    );
}

/// Variant with `<div>` preceding block — often emits a single `\n`.
#[test]
fn div_paragraph_multibyte_span_does_not_panic() {
    let html = "<div>preceding text</div>\
                <p><span>■multibyte</span></p>";

    let result = html_to_markdown_rs::convert(html, Some(options_with_structure()));
    assert!(result.is_ok(), "conversion must not panic or error: {:?}", result.err());
}

/// Plain-text paragraph before — exercises the separator `\n\n` path.
#[test]
fn paragraph_paragraph_multibyte_span_does_not_panic() {
    let html = "<p>first paragraph</p>\
                <p><span style=\"letter-spacing: 0.0px;\">■Example request</span></p>";

    let result = html_to_markdown_rs::convert(html, Some(options_with_structure()));
    assert!(result.is_ok(), "conversion must not panic or error: {:?}", result.err());
    let content = result.unwrap().content.unwrap_or_default();
    assert!(
        content.contains("■Example request"),
        "multibyte char must survive: {content}"
    );
}

/// Exercises the `<figure>` path: figure with multibyte caption.
///
/// `figure.rs` also captures `output.len()` before processing children and
/// slices with that offset later; the same boundary bug can apply there.
#[test]
fn figure_multibyte_caption_does_not_panic() {
    let html = "<pre>code block</pre>\
                <figure>\
                  <img src=\"x.png\" alt=\"img\">\
                  <figcaption>■Caption text</figcaption>\
                </figure>";

    let result = html_to_markdown_rs::convert(html, Some(options_with_structure()));
    assert!(result.is_ok(), "conversion must not panic or error: {:?}", result.err());
}

/// Attempt to land `content_start_pos` exactly at byte 23 (mid-■ if not clamped).
///
/// "previous bl" = 11 bytes + "\n" = 12 bytes for the pre content,
/// plus the fenced code block wrappers "```\n" (4) + "\n```\n\n" (7) = 23 bytes total.
/// We craft the preceding content so that `content_start_pos` == 23 and the
/// pop (single-\n guard) would move the boundary inside ■ (E2 96 A0).
#[test]
fn crafted_23_byte_boundary_does_not_panic() {
    let html = "<p>12345678901</p>\
                <p><span>■X</span></p>";

    let result = html_to_markdown_rs::convert(html, Some(options_with_structure()));
    assert!(result.is_ok(), "conversion must not panic or error: {:?}", result.err());
}

/// Wide variety of multibyte characters to exercise the boundary check.
#[test]
fn various_multibyte_chars_in_structured_paragraphs_do_not_panic() {
    let chars = ["■", "→", "©", "€", "中", "🦀", "à", "ñ"];
    for ch in chars {
        let html = format!("<pre>block</pre><p><span>{ch}text</span></p>");
        let result = html_to_markdown_rs::convert(&html, Some(options_with_structure()));
        assert!(
            result.is_ok(),
            "conversion must not panic for char {ch:?}: {:?}",
            result.err()
        );
    }
}
