#![allow(missing_docs)]
fn convert(
    html: &str,
    opts: Option<html_to_markdown_rs::ConversionOptions>,
) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

use html_to_markdown_rs::{ConversionOptions, OutputFormat};

fn djot_options() -> ConversionOptions {
    ConversionOptions {
        output_format: OutputFormat::Djot,
        ..Default::default()
    }
}

#[test]
fn test_djot_emphasis() {
    let html = "<p>Text with <em>emphasis</em> word</p>";
    let result = convert(html, Some(djot_options())).unwrap();
    assert!(result.contains("_emphasis_"), "Expected _emphasis_, got: {result}");
}

#[test]
fn test_djot_italic() {
    let html = "<p>Text with <i>italic</i> word</p>";
    let result = convert(html, Some(djot_options())).unwrap();
    assert!(result.contains("_italic_"), "Expected _italic_, got: {result}");
}

#[test]
fn test_djot_strong() {
    let html = "<p>Text with <strong>strong</strong> word</p>";
    let result = convert(html, Some(djot_options())).unwrap();
    assert!(result.contains("*strong*"), "Expected *strong*, got: {result}");
    // Should NOT have double asterisks
    assert!(
        !result.contains("**strong**"),
        "Should not have **strong**, got: {result}"
    );
}

#[test]
fn test_djot_bold() {
    let html = "<p>Text with <b>bold</b> word</p>";
    let result = convert(html, Some(djot_options())).unwrap();
    assert!(result.contains("*bold*"), "Expected *bold*, got: {result}");
    // Should NOT have double asterisks
    assert!(
        !result.contains("**bold**"),
        "Should not have double asterisks, got: {result}"
    );
}

#[test]
fn test_djot_options_debug() {
    // Debug test to verify options are set correctly
    let options = djot_options();
    println!("Output format: {:?}", options.output_format);
    assert_eq!(options.output_format, OutputFormat::Djot);
}

#[test]
fn test_djot_strikethrough() {
    let html = "<p>Text with <del>deleted</del> word</p>";
    let result = convert(html, Some(djot_options())).unwrap();
    assert!(result.contains("{-deleted-}"), "Expected {{-deleted-}}, got: {result}");
}

#[test]
fn test_djot_strikethrough_s_tag() {
    let html = "<p>Text with <s>strikethrough</s> word</p>";
    let result = convert(html, Some(djot_options())).unwrap();
    assert!(
        result.contains("{-strikethrough-}"),
        "Expected {{-strikethrough-}}, got: {result}"
    );
}

#[test]
fn test_djot_inserted() {
    let html = "<p>Text with <ins>inserted</ins> word</p>";
    let result = convert(html, Some(djot_options())).unwrap();
    assert!(
        result.contains("{+inserted+}"),
        "Expected {{+inserted+}}, got: {result}"
    );
}

#[test]
fn test_djot_highlight() {
    let html = "<p>Text with <mark>highlighted</mark> word</p>";
    let result = convert(html, Some(djot_options())).unwrap();
    assert!(
        result.contains("{=highlighted=}"),
        "Expected {{=highlighted=}}, got: {result}"
    );
}

#[test]
fn test_djot_subscript() {
    let html = "<p>H<sub>2</sub>O</p>";
    let result = convert(html, Some(djot_options())).unwrap();
    assert!(result.contains("~2~"), "Expected ~2~, got: {result}");
}

#[test]
fn test_djot_superscript() {
    let html = "<p>x<sup>2</sup></p>";
    let result = convert(html, Some(djot_options())).unwrap();
    assert!(result.contains("^2^"), "Expected ^2^, got: {result}");
}

#[test]
fn test_djot_combined_formatting() {
    let html = "<p><strong>Bold</strong> and <em>italic</em> and <del>deleted</del></p>";
    let result = convert(html, Some(djot_options())).unwrap();
    assert!(result.contains("*Bold*"), "Expected *Bold*, got: {result}");
    assert!(result.contains("_italic_"), "Expected _italic_, got: {result}");
    assert!(result.contains("{-deleted-}"), "Expected {{-deleted-}}, got: {result}");
}

#[test]
fn test_markdown_output_unchanged() {
    // Verify Markdown output still works as expected
    let html = "<p><strong>Bold</strong> and <em>italic</em></p>";
    let result = convert(html, None).unwrap();
    assert!(
        result.contains("**Bold**"),
        "Expected **Bold** for Markdown, got: {result}"
    );
    assert!(
        result.contains("*italic*"),
        "Expected *italic* for Markdown, got: {result}"
    );
}

#[test]
fn test_markdown_strikethrough_unchanged() {
    let html = "<p><del>deleted</del></p>";
    let result = convert(html, None).unwrap();
    assert!(
        result.contains("~~deleted~~"),
        "Expected ~~deleted~~ for Markdown, got: {result}"
    );
}

#[test]
fn test_output_format_default_is_markdown() {
    let options = ConversionOptions::default();
    assert_eq!(options.output_format, OutputFormat::Markdown);
}
