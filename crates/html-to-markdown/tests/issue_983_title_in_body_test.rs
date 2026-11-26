//! Issue #983: a `<title>` in the body is the document title when no title comes before it, as
//! `document.title` reads it in a browser. Pages that stream their metadata put it there. A
//! browser does not render the element, so its text is not body text.
//!
//! Each input runs through the public entry with the default route, with the full converter
//! and with the fast converter forced. A forced fast run that hands the page over returns the
//! text of the full converter, so the three texts must be equal.

#![cfg(all(feature = "testkit", feature = "metadata"))]

use html_to_markdown_rs::{ConversionOptions, OutputFormat, TierStrategy, convert};

fn run(html: &str, extract_metadata: bool, tier_strategy: TierStrategy) -> String {
    let options = ConversionOptions {
        extract_metadata,
        tier_strategy,
        ..ConversionOptions::default()
    };
    convert(html, Some(options))
        .expect("the conversion succeeds")
        .content
        .unwrap_or_default()
}

/// The text of the default route, after a check that the full and the fast route give it too.
fn on_every_route(html: &str, extract_metadata: bool) -> String {
    let auto = run(html, extract_metadata, TierStrategy::Auto);
    let full = run(html, extract_metadata, TierStrategy::Tier2);
    let fast = run(html, extract_metadata, TierStrategy::Tier1);
    assert_eq!(full, auto, "full against default for {html:?}");
    assert_eq!(fast, auto, "fast against default for {html:?}");
    auto
}

fn document_title(html: &str) -> Option<String> {
    let options = ConversionOptions {
        extract_metadata: true,
        ..ConversionOptions::default()
    };
    convert(html, Some(options))
        .expect("the conversion succeeds")
        .metadata
        .document
        .title
}

const IN_BODY: &str = "<html><head></head><body><div><title>T</title><p>one</p></div></body></html>";

#[test]
fn a_title_in_the_body_is_the_document_title() {
    assert_eq!(on_every_route(IN_BODY, true), "---\ntitle: T\n---\n\none\n");
    assert_eq!(document_title(IN_BODY).as_deref(), Some("T"));
}

#[test]
fn a_title_in_the_body_is_not_body_text_when_metadata_is_off() {
    assert_eq!(on_every_route(IN_BODY, false), "one\n");
}

#[test]
fn a_title_after_body_content_is_the_document_title() {
    let html = "<p>one</p><title>T</title>";
    assert_eq!(on_every_route(html, true), "---\ntitle: T\n---\n\none\n");
    assert_eq!(on_every_route(html, false), "one\n");
}

#[test]
fn a_title_streamed_after_the_body_scripts_is_the_document_title() {
    let html = concat!(
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"/></head>"#,
        r#"<body><main><h1>Heading</h1><p>one</p></main>"#,
        r#"<script src="/app.js" async=""></script>"#,
        r#"<title>Page &amp; Co</title><meta name="description" content="d"/>"#,
        r#"</body></html>"#,
    );
    assert_eq!(
        on_every_route(html, true),
        "---\ntitle: Page & Co\n---\n\n# Heading\n\none\n"
    );
    assert_eq!(document_title(html).as_deref(), Some("Page & Co"));
}

#[test]
fn a_title_in_the_head_wins_over_one_in_the_body() {
    let html = "<html><head><title>A</title></head><body><title>B</title><p>one</p></body></html>";
    assert_eq!(on_every_route(html, true), "---\ntitle: A\n---\n\none\n");
    assert_eq!(document_title(html).as_deref(), Some("A"));
}

#[test]
fn an_empty_title_in_the_head_is_still_the_first_title() {
    // ~keep `document.title` reads the first title element, so an empty one in the head gives
    // ~keep an empty title even when the body holds another.
    let html = "<html><head><title></title></head><body><title>B</title><p>one</p></body></html>";
    assert_eq!(on_every_route(html, true), "one\n");
}

#[test]
fn the_first_of_two_titles_in_the_body_is_the_document_title() {
    let html = "<html><head></head><body><title>A</title><p>one</p><title>B</title></body></html>";
    assert_eq!(on_every_route(html, true), "---\ntitle: A\n---\n\none\n");
}

#[test]
fn a_title_in_a_drawing_or_template_in_the_body_is_not_the_document_title() {
    let template = "<html><head></head><body><template><title>T</title></template><p>one</p></body></html>";
    assert_eq!(document_title(template), None);
    let drawing = r#"<html><head></head><body><svg width="4" height="4"><title>T</title><circle r="1"/></svg><p>one</p></body></html>"#;
    assert_eq!(document_title(drawing), None);
}

#[test]
fn a_title_in_raw_text_content_in_the_body_is_not_the_document_title() {
    for element in ["noembed", "noframes"] {
        let html = format!(
            "<html><head></head><body><p>one</p><{element}><title>Fallback</title></{element}><title>Page</title></body></html>"
        );
        assert_eq!(document_title(&html), Some("Page".to_string()), "{element}");
    }
}

#[test]
fn a_title_in_the_body_is_not_plain_text() {
    let options = ConversionOptions {
        output_format: OutputFormat::Plain,
        ..ConversionOptions::default()
    };
    let text = convert(IN_BODY, Some(options))
        .expect("the conversion succeeds")
        .content
        .unwrap_or_default();
    assert_eq!(text.trim(), "one");
}

#[test]
fn the_text_of_a_title_in_the_body_is_the_document_title_as_written() {
    // ~keep HTML reads the content of a `title` element as text, so a tag in it is part of the
    // ~keep title and opens no element.
    for content in [
        "<div hidden>x</div>",
        "<template><p>x</p></template>",
        "<noscript><p>x</p></noscript>",
    ] {
        let html = format!("<p>see</p><title>{content}</title><p>after</p>");
        assert_eq!(
            on_every_route(&html, true),
            format!("---\ntitle: {content}\n---\n\nsee\n\nafter\n")
        );
        assert_eq!(on_every_route(&html, false), "see\n\nafter\n");
    }
}
