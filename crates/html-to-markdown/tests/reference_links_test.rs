#![allow(missing_docs)]

use html_to_markdown_rs::{ConversionOptions, LinkStyle};

fn convert(html: &str, options: Option<ConversionOptions>) -> String {
    html_to_markdown_rs::convert(html, options)
        .unwrap()
        .content
        .unwrap_or_default()
}

fn ref_options() -> ConversionOptions {
    ConversionOptions {
        link_style: LinkStyle::Reference,
        ..Default::default()
    }
}

#[test]
fn basic_reference_link() {
    let html = r#"<a href="https://example.com">Click here</a>"#;
    let result = convert(html, Some(ref_options()));
    assert!(
        result.contains("[Click here][1]"),
        "Expected reference-style link, got: {result}"
    );
    assert!(
        result.contains("[1]: https://example.com"),
        "Expected reference definition, got: {result}"
    );
}

#[test]
fn reference_link_with_title() {
    let html = r#"<a href="https://example.com" title="Example">Click</a>"#;
    let result = convert(html, Some(ref_options()));
    assert!(
        result.contains("[Click][1]"),
        "Expected reference-style link, got: {result}"
    );
    assert!(
        result.contains(r#"[1]: https://example.com "Example""#),
        "Expected reference definition with title, got: {result}"
    );
}

#[test]
fn url_deduplication() {
    let html = r#"<a href="https://example.com">First</a> <a href="https://example.com">Second</a>"#;
    let result = convert(html, Some(ref_options()));
    assert!(
        result.contains("[First][1]"),
        "Expected first link with ref 1, got: {result}"
    );
    assert!(
        result.contains("[Second][1]"),
        "Expected second link reusing ref 1, got: {result}"
    );
    // Should only have one definition
    let count = result.matches("[1]: https://example.com").count();
    assert_eq!(count, 1, "Expected exactly one definition, got: {result}");
}

#[test]
fn different_titles_different_refs() {
    let html =
        r#"<a href="https://example.com" title="A">First</a> <a href="https://example.com" title="B">Second</a>"#;
    let result = convert(html, Some(ref_options()));
    assert!(
        result.contains("[First][1]"),
        "Expected first link ref 1, got: {result}"
    );
    assert!(
        result.contains("[Second][2]"),
        "Expected second link ref 2 (different title), got: {result}"
    );
}

#[test]
fn image_reference_style() {
    let html = r#"<img src="https://example.com/img.png" alt="A photo">"#;
    let result = convert(html, Some(ref_options()));
    assert!(
        result.contains("![A photo][1]"),
        "Expected reference-style image, got: {result}"
    );
    assert!(
        result.contains("[1]: https://example.com/img.png"),
        "Expected image reference definition, got: {result}"
    );
}

#[test]
fn mixed_links_and_images_share_numbering() {
    let html = r#"<a href="https://a.com">Link</a><img src="https://b.com/img.png" alt="Img">"#;
    let result = convert(html, Some(ref_options()));
    assert!(result.contains("[Link][1]"), "Expected link as ref 1, got: {result}");
    assert!(result.contains("![Img][2]"), "Expected image as ref 2, got: {result}");
}

#[test]
fn autolinks_unaffected() {
    let html = r#"<a href="https://example.com">https://example.com</a>"#;
    let options = ConversionOptions {
        link_style: LinkStyle::Reference,
        autolinks: true,
        ..Default::default()
    };
    let result = convert(html, Some(options));
    // Autolinks should still render as <url>
    assert!(
        result.contains("<https://example.com>"),
        "Autolinks should not be affected by reference style, got: {result}"
    );
}

#[test]
fn default_inline_unchanged() {
    let html = r#"<a href="https://example.com">Click</a>"#;
    let result = convert(html, None);
    assert!(
        result.contains("[Click](https://example.com)"),
        "Default should use inline style, got: {result}"
    );
}

#[test]
fn multiple_paragraphs_references_at_end() {
    let html = r#"<p><a href="https://a.com">A</a></p><p><a href="https://b.com">B</a></p>"#;
    let result = convert(html, Some(ref_options()));
    // References should be at the very end
    let ref_section_start = result.find("[1]:").expect("Should have ref section");
    let content_end = result.find("[A][1]").expect("Should have inline ref");
    assert!(
        ref_section_start > content_end,
        "Reference section should be after content"
    );
}

#[test]
fn empty_href_no_reference() {
    let html = r#"<a href="">Empty</a>"#;
    let result = convert(html, Some(ref_options()));
    // Empty href should not create a reference
    assert!(
        !result.contains("[1]:"),
        "Empty href should not create reference, got: {result}"
    );
}

#[test]
fn title_with_quotes_escaped() {
    let html = r#"<a href="https://example.com" title='Say "hello"'>Link</a>"#;
    let result = convert(html, Some(ref_options()));
    assert!(
        result.contains(r#"[1]: https://example.com "Say \"hello\"""#),
        "Quotes in title should be escaped, got: {result}"
    );
}

#[test]
fn media_elements_reference_style() {
    let html = r#"<video src="https://example.com/video.mp4"></video>"#;
    let result = convert(html, Some(ref_options()));
    assert!(
        result.contains("[1]: https://example.com/video.mp4"),
        "Video should use reference style, got: {result}"
    );
}
