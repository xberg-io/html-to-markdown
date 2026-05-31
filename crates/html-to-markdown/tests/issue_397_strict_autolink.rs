#![allow(missing_docs)]

//! Regression test for issue #397: bare filenames such as
//! `<a href="foobar.png">foobar.png</a>` were being emitted as
//! `<foobar.png>` — invalid Markdown that looks like an HTML tag.
//! Per GFM §6.5, an autolink requires an absolute URI with a scheme.

use html_to_markdown_rs::{ConversionOptions, convert};

#[test]
fn filename_with_extension_is_not_autolinked() {
    let html = r#"<a href="foobar.png">foobar.png</a>"#;
    let result = convert(html, Some(ConversionOptions::default())).expect("conversion should succeed");
    let content = result.content.unwrap_or_default();
    assert_eq!(content.trim(), "[foobar.png](foobar.png)");
}

#[test]
fn relative_path_is_not_autolinked() {
    let html = r#"<a href="/docs/intro.html">/docs/intro.html</a>"#;
    let result = convert(html, Some(ConversionOptions::default())).expect("conversion should succeed");
    let content = result.content.unwrap_or_default();
    assert_eq!(content.trim(), "[/docs/intro.html](/docs/intro.html)");
}

#[test]
fn https_url_still_autolinked() {
    let html = r#"<a href="https://example.com">https://example.com</a>"#;
    let result = convert(html, Some(ConversionOptions::default())).expect("conversion should succeed");
    let content = result.content.unwrap_or_default();
    assert_eq!(content.trim(), "<https://example.com>");
}

#[test]
fn mailto_still_autolinked() {
    let html = r#"<a href="mailto:a@b.com">a@b.com</a>"#;
    let result = convert(html, Some(ConversionOptions::default())).expect("conversion should succeed");
    let content = result.content.unwrap_or_default();
    assert_eq!(content.trim(), "<a@b.com>");
}

#[test]
fn mixed_filename_and_url_produces_correct_output() {
    let html = r#"<a href="foobar.png">foobar.png</a> <a href="https://www.heise.de">https://www.heise.de</a>"#;
    let result = convert(html, Some(ConversionOptions::default())).expect("conversion should succeed");
    let content = result.content.unwrap_or_default();
    assert_eq!(content.trim(), "[foobar.png](foobar.png) <https://www.heise.de>");
}
