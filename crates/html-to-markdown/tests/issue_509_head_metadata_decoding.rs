#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #509: head metadata values kept their character references encoded.
//!
//! `<base href="https://example.com/it&#x27;s/">` produced `base: https://example.com/it&#x27;s/`
//! in the frontmatter. The canonical link and the title had the same defect. Every case runs on
//! both tiers, because Tier 1 and Tier 2 reach the head through different entry points.

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn frontmatter(html: &str, tier: TierStrategy) -> String {
    let options = ConversionOptions {
        tier_strategy: tier,
        extract_metadata: true,
        ..ConversionOptions::default()
    };
    convert(html, Some(options))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

fn assert_frontmatter_line(html: &str, expected_line: &str) {
    for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
        let out = frontmatter(html, tier);
        assert!(
            out.lines().any(|line| line == expected_line),
            "{tier:?}: expected the line {expected_line:?}; actual: {out:?}"
        );
    }
}

fn base(href: &str) -> String {
    format!("<html><head><base href=\"{href}\"></head><body><p>body</p></body></html>")
}

/// `(label, raw href, decoded href)` for every character-reference form an attribute can carry.
const BASE_HREF_CASES: &[(&str, &str, &str)] = &[
    ("hex", "https://example.com/it&#x27;s/", "https://example.com/it's/"),
    ("decimal", "https://example.com/it&#39;s/", "https://example.com/it's/"),
    ("named", "https://example.com/caf&eacute;/", "https://example.com/café/"),
    (
        "amp in query",
        "https://example.com/?a=1&amp;b=2",
        "https://example.com/?a=1&b=2",
    ),
    // ~keep 150 is 0x96, which the WHATWG table maps to an en dash, not the C1 control.
    (
        "windows-1252",
        "https://example.com/a&#150;b/",
        "https://example.com/a\u{2013}b/",
    ),
    (
        "surrogate",
        "https://example.com/a&#xD800;b/",
        "https://example.com/a\u{FFFD}b/",
    ),
    (
        "out of range",
        "https://example.com/a&#x110000;b/",
        "https://example.com/a\u{FFFD}b/",
    ),
    // ~keep In an attribute, a named reference without `;` followed by `=` stays literal.
    (
        "ambiguous ampersand",
        "https://example.com/?a=1&copy=2",
        "https://example.com/?a=1&copy=2",
    ),
    // ~keep Decoding runs once: an escaped reference comes out as the reference text.
    (
        "double encoded",
        "https://example.com/it&amp;#x27;s/",
        "https://example.com/it&#x27;s/",
    ),
];

#[test]
fn base_href_character_references_are_decoded() {
    let mut mismatches = Vec::new();
    for (label, raw, decoded) in BASE_HREF_CASES {
        let html = base(raw);
        for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
            let out = frontmatter(&html, tier);
            let expected = format!("base: {decoded}");
            if !out.lines().any(|line| line == expected) {
                mismatches.push(format!("{label} on {tier:?}: expected {expected:?}; actual: {out:?}"));
            }
        }
    }
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
}

#[test]
fn canonical_href_character_references_are_decoded() {
    let html = r#"<html><head><link rel="canonical" href="https://example.com/it&#x27;s?a=1&amp;b=2"></head><body><p>body</p></body></html>"#;
    assert_frontmatter_line(html, "canonical: https://example.com/it's?a=1&b=2");
}

#[test]
fn title_character_references_are_decoded() {
    let html =
        "<html><head><title>Tom &amp; Jerry&#8217;s &quot;Page&quot;</title></head><body><p>body</p></body></html>";
    assert_frontmatter_line(html, "title: Tom & Jerry\u{2019}s \"Page\"");
}

#[test]
fn meta_content_character_references_are_decoded() {
    let html = r#"<html><head><meta name="description" content="Tom &amp; Jerry&#x27;s"><meta property="og:url" content="https://example.com/?a=1&amp;b=2"></head><body><p>body</p></body></html>"#;
    assert_frontmatter_line(html, "meta-description: Tom & Jerry's");
    assert_frontmatter_line(html, "meta-og:url: https://example.com/?a=1&b=2");
}

#[cfg(feature = "metadata")]
#[test]
fn document_metadata_values_are_decoded() {
    let html = concat!(
        "<html><head><title>Tom &amp; Jerry</title>",
        r#"<base href="https://example.com/it&#x27;s/">"#,
        r#"<link rel="canonical" href="https://example.com/?a=1&amp;b=2">"#,
        "</head><body><p>body</p></body></html>",
    );
    // ~keep Tier 2 alone builds the structured metadata; Tier 1 only emits frontmatter text.
    let options = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: true,
        ..ConversionOptions::default()
    };
    let document = convert(html, Some(options))
        .expect("conversion should succeed")
        .metadata
        .document;
    assert_eq!(document.title.as_deref(), Some("Tom & Jerry"));
    assert_eq!(document.base_href.as_deref(), Some("https://example.com/it's/"));
    assert_eq!(document.canonical_url.as_deref(), Some("https://example.com/?a=1&b=2"));
}

/// A custom element sends Tier 2 through the html5ever repair, which re-serializes the head
/// before `tl` reads it. The values must come out the same as on the direct path.
#[test]
fn values_are_decoded_once_after_html5ever_repair() {
    let html = concat!(
        "<html><head><title>Tom &amp; Jerry &amp;lt;3</title>",
        r#"<base href="https://example.com/it&#x27;s/?a=1&copy=2">"#,
        "</head><body><my-widget>x</my-widget></body></html>",
    );
    let out = frontmatter(html, TierStrategy::Tier2);
    for expected in ["title: Tom & Jerry &lt;3", "base: https://example.com/it's/?a=1&copy=2"] {
        assert!(
            out.lines().any(|line| line == expected),
            "expected the line {expected:?}; actual: {out:?}"
        );
    }
}

/// The public document-structure builder reads the head on its own. Its `<meta content>` values
/// must be decoded like the frontmatter's.
#[test]
fn document_structure_meta_content_is_decoded() {
    let html = concat!(
        r#"<html><head><title>Tom &amp; Jerry</title><meta name="description" content="Tom &amp; Jerry&#x27;s"></head>"#,
        "<body><p>body</p></body></html>",
    );
    let dom = tl::parse(html, tl::ParserOptions::default()).expect("parse html");
    let document = html_to_markdown_rs::types::build_document_structure(&dom);
    let entries = document
        .nodes
        .iter()
        .find_map(|node| match &node.content {
            html_to_markdown_rs::types::NodeContent::MetadataBlock { entries } => Some(entries.clone()),
            _ => None,
        })
        .expect("the head produces a metadata block");
    let value = |key: &str| entries.iter().find(|e| e.key == key).map(|e| e.value.clone());
    assert_eq!(value("title").as_deref(), Some("Tom & Jerry"));
    assert_eq!(value("description").as_deref(), Some("Tom & Jerry's"));
}
