#![allow(missing_docs)]

//! Regression test for issue #396: `CodeBlockStyle::Backticks` emitted a
//! trailing `\n` inside the fence and omitted the blank line after the
//! closing fence, while `CodeBlockStyle::Indented` produced correct
//! separation. Both fenced styles (backticks and tildes) must now strip
//! trailing newlines from inner content and append a blank line after the
//! closing fence so following block content is separated correctly.

use html_to_markdown_rs::options::CodeBlockStyle;
use html_to_markdown_rs::{ConversionOptions, convert};

#[test]
fn backticks_no_trailing_newline_inside_fence_and_blank_line_after() {
    let html = "<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>";
    let mut options = ConversionOptions::default();
    options.code_block_style = CodeBlockStyle::Backticks;
    let result = convert(html, Some(options)).expect("conversion should succeed");
    let content = result.content.unwrap_or_default();
    assert_eq!(content, "Foo\n\n```\n1\n2\n```\n\nBar\n");
}

#[test]
fn tildes_no_trailing_newline_inside_fence_and_blank_line_after() {
    let html = "<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>";
    let mut options = ConversionOptions::default();
    options.code_block_style = CodeBlockStyle::Tildes;
    let result = convert(html, Some(options)).expect("conversion should succeed");
    let content = result.content.unwrap_or_default();
    assert_eq!(content, "Foo\n\n~~~\n1\n2\n~~~\n\nBar\n");
}

#[test]
fn indented_style_unchanged() {
    let html = "<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>";
    let mut options = ConversionOptions::default();
    options.code_block_style = CodeBlockStyle::Indented;
    let result = convert(html, Some(options)).expect("conversion should succeed");
    let content = result.content.unwrap_or_default();
    assert_eq!(content, "Foo\n\n    1\n    2\n\nBar\n");
}
