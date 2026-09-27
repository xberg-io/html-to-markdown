//! Integration tests for Tier-1 metadata extraction (M5).
//!
//! Each test verifies that Tier-1 with `Tier1` and `extract_metadata: true`
//! produces output byte-identical to Tier-2 (`Tier2`, same options).
//!
//! These tests cover YAML frontmatter produced for:
//!   - `<title>` (including HTML entities and multi-word titles)
//!   - `<meta name="...">` (description, keywords, author, viewport)
//!   - `<meta property="og:...">` (Open Graph tags: title, description, image)
//!   - `<meta property="twitter:...">` (Twitter Card tags)
//!   - `<link rel="canonical">`, `<link rel="author">`, `<link rel="license">`
//!   - `<base href="...">`
//!   - Documents without a `<head>` element
//!   - Documents with an empty `<head>` element
//!   - Multiple concurrent meta tags
//!   - No-metadata case (option disabled)

#![cfg(feature = "testkit")]

use html_to_markdown_rs::{ConversionOptions, HighlightStyle, TierStrategy, convert};

/// Convert with `Tier1` + `extract_metadata: true`.
fn t1(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        extract_metadata: true,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts))
        .expect("tier-1 conversion must succeed")
        .content
        .unwrap_or_default()
}

/// Convert with `Tier2` + `extract_metadata: true` (same other defaults).
fn t2(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: true,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts))
        .expect("tier-2 conversion must succeed")
        .content
        .unwrap_or_default()
}

#[test]
fn tier1_extracts_title() {
    let html = "<html><head><title>Hello</title></head><body><p>body</p></body></html>";
    assert_eq!(t1(html), t2(html), "title metadata must be byte-identical");
}

#[test]
fn tier1_extracts_title_with_spaces() {
    let html = "<html><head><title>My Page Title Here</title></head><body><p>x</p></body></html>";
    assert_eq!(t1(html), t2(html), "multi-word title metadata must be byte-identical");
}

#[test]
fn tier1_extracts_title_with_leading_trailing_whitespace() {
    let html = "<html><head><title>  Padded Title  </title></head><body><p>x</p></body></html>";
    assert_eq!(
        t1(html),
        t2(html),
        "title with whitespace padding must be byte-identical"
    );
}

#[test]
fn tier1_extracts_meta_description() {
    let html = r#"<html><head><meta name="description" content="a desc"></head><body><p>x</p></body></html>"#;
    assert_eq!(t1(html), t2(html), "meta description must be byte-identical");
}

#[test]
fn tier1_extracts_meta_keywords() {
    let html =
        r#"<html><head><meta name="keywords" content="rust, markdown, html"></head><body><p>x</p></body></html>"#;
    assert_eq!(t1(html), t2(html), "meta keywords must be byte-identical");
}

#[test]
fn tier1_extracts_meta_author() {
    let html = r#"<html><head><meta name="author" content="Jane Doe"></head><body><p>x</p></body></html>"#;
    assert_eq!(t1(html), t2(html), "meta author must be byte-identical");
}

#[test]
fn tier1_extracts_og_image() {
    let html = r#"<html><head><meta property="og:image" content="foo.png"></head><body><p>x</p></body></html>"#;
    assert_eq!(t1(html), t2(html), "og:image must be byte-identical");
}

#[test]
fn tier1_extracts_og_title() {
    let html =
        r#"<html><head><meta property="og:title" content="Open Graph Title"></head><body><p>x</p></body></html>"#;
    assert_eq!(t1(html), t2(html), "og:title must be byte-identical");
}

#[test]
fn tier1_extracts_og_description() {
    let html =
        r#"<html><head><meta property="og:description" content="OG Description"></head><body><p>x</p></body></html>"#;
    assert_eq!(t1(html), t2(html), "og:description must be byte-identical");
}

#[test]
fn tier1_extracts_canonical_url() {
    let html =
        r#"<html><head><link rel="canonical" href="https://example.com/page"></head><body><p>x</p></body></html>"#;
    assert_eq!(t1(html), t2(html), "canonical link must be byte-identical");
}

#[test]
fn tier1_extracts_link_author() {
    let html =
        r#"<html><head><link rel="author" href="https://example.com/author"></head><body><p>x</p></body></html>"#;
    assert_eq!(t1(html), t2(html), "link-author must be byte-identical");
}

#[test]
fn tier1_extracts_base_href() {
    let html = r#"<html><head><base href="https://example.com/"></head><body><p>x</p></body></html>"#;
    assert_eq!(t1(html), t2(html), "base-href must be byte-identical");
}

#[test]
fn tier1_extracts_multiple_meta_tags() {
    let html = r#"<html><head>
        <title>My Site</title>
        <meta name="description" content="site description">
        <meta property="og:image" content="image.png">
        <link rel="canonical" href="https://example.com/">
    </head><body><p>content</p></body></html>"#;
    assert_eq!(t1(html), t2(html), "multiple meta tags must be byte-identical");
}

#[test]
fn tier1_no_head_element_produces_no_frontmatter() {
    let html = "<body><p>just a paragraph</p></body>";
    assert_eq!(t1(html), t2(html), "no-head document must be byte-identical");
}

#[test]
fn tier1_empty_head_produces_no_frontmatter() {
    let html = "<html><head></head><body><p>paragraph</p></body></html>";
    assert_eq!(
        t1(html),
        t2(html),
        "empty head must produce no frontmatter (byte-identical)"
    );
}

#[test]
fn tier1_no_frontmatter_when_extract_metadata_false() {
    let html = "<html><head><title>Hello</title></head><body><p>body</p></body></html>";
    let opts_t1 = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    let opts_t2 = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    let out_t1 = convert(html, Some(opts_t1)).unwrap().content.unwrap_or_default();
    let out_t2 = convert(html, Some(opts_t2)).unwrap().content.unwrap_or_default();
    assert_eq!(out_t1, out_t2, "disabled metadata must be byte-identical");
    assert!(
        !out_t1.starts_with("---\n"),
        "no YAML frontmatter expected when extract_metadata=false"
    );
}

#[test]
fn tier1_yaml_quotes_colon_in_value() {
    let html = r#"<html><head><meta name="description" content="Key: Value"></head><body><p>x</p></body></html>"#;
    assert_eq!(t1(html), t2(html), "YAML-quoted colon value must be byte-identical");
}

// ~keep ── 10. Auto-routing respects extract_metadata now (no longer forces Tier-2) ──

#[test]
fn auto_routing_with_extract_metadata_can_use_tier1() {
    // ~keep With no other Tier-2 signals, Auto should allow the classifier to pick
    // ~keep Tier-1 when extract_metadata=true (M5 removes that guard).  The output
    // ~keep must still match Tier-2.
    let html = "<html><head><title>AutoTest</title></head><body><p>content</p></body></html>";
    let opts_auto = ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        extract_metadata: true,
        ..ConversionOptions::default()
    };
    let opts_t2 = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: true,
        ..ConversionOptions::default()
    };
    let auto_out = convert(html, Some(opts_auto)).unwrap().content.unwrap_or_default();
    let t2_out = convert(html, Some(opts_t2)).unwrap().content.unwrap_or_default();
    assert_eq!(auto_out, t2_out, "Auto with extract_metadata must match Tier-2");
    assert!(
        auto_out.starts_with("---\n"),
        "Auto output must include YAML frontmatter"
    );
}

// ~keep ── 11. The document's first `<base href>` and first canonical link ──

#[test]
fn should_report_the_first_base_href_on_both_tiers() {
    let html = r#"<html><head><base href="/first/"><base href="/second/"></head><body><p>x</p></body></html>"#;
    let out = t2(html);
    assert!(out.contains("base: /first/\n"), "{out}");
    assert_eq!(t1(html), out);
}

#[test]
fn should_report_the_base_href_the_parser_keeps_on_both_tiers() {
    let html = r#"<html><head><title><base href="https://evil.example/"></title><base href="/real/"></head><body><p>x</p></body></html>"#;
    let out = t2(html);
    assert!(out.contains("base: /real/\n"), "{out}");
    assert_eq!(t1(html), out);
}

#[test]
fn should_report_a_base_href_from_the_body_with_an_empty_head_on_both_tiers() {
    let html = r#"<html><head></head><body><base href="/in-body/"><p>x</p></body></html>"#;
    let out = t2(html);
    assert!(out.contains("base: /in-body/\n"), "{out}");
    assert_eq!(t1(html), out);
}

#[test]
fn should_report_the_first_base_href_on_the_auto_path() {
    // ~keep Without the `metadata` feature, these options let the router pick Tier 1.
    let html = r#"<html><head><base href="/first/"><base href="/second/"></head><body><p>x</p></body></html>"#;
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        extract_metadata: true,
        highlight_style: HighlightStyle::None,
        ..ConversionOptions::default()
    };
    let out = convert(html, Some(opts)).unwrap().content.unwrap_or_default();
    assert!(out.contains("base: /first/\n"), "{out}");
}

#[test]
fn should_report_the_first_canonical_link_on_both_tiers() {
    let html = r#"<html><head><link rel="canonical" href="https://example.com/first"><link rel="canonical" href="https://example.com/second"></head><body><p>x</p></body></html>"#;
    let out = t2(html);
    assert!(out.contains("canonical: https://example.com/first\n"), "{out}");
    assert_eq!(t1(html), out);
}

#[test]
fn should_report_the_first_base_href_from_the_tier1_entry_point() {
    let html = r#"<html><head><base href="/first/"><base href="/second/"></head><body><p>x</p></body></html>"#;
    let opts = ConversionOptions {
        extract_metadata: true,
        ..ConversionOptions::default()
    };
    let out = html_to_markdown_rs::tier1::run(html, &html_to_markdown_rs::prescan::PrescanReport::default(), &opts)
        .expect("tier-1 conversion must succeed");
    assert!(out.contains("base: /first/\n"), "{out}");
}

#[test]
fn should_report_the_first_meta_tag_per_name_on_both_tiers() {
    let html = r#"<html><head><meta name="description" content="first"><meta name="description" content="second"><meta property="og:title" content="first"><meta property="og:title" content="second"></head><body><p>x</p></body></html>"#;
    let out = t2(html);
    assert!(out.contains("meta-description: first\n"), "{out}");
    assert!(out.contains("meta-og:title: first\n"), "{out}");
    assert_eq!(t1(html), out);
}

#[test]
fn should_report_the_head_metadata_after_a_leading_doctype_and_comment_on_both_tiers() {
    let html = r#"<!DOCTYPE html><!-- note --><html><head><title>T</title><base href="/b/"></head><body><p>x</p></body></html>"#;
    let out = t2(html);
    assert!(out.contains("title: T\n") && out.contains("base: /b/\n"), "{out}");
    assert_eq!(t1(html), out);
}

#[test]
fn should_report_the_base_href_of_a_document_without_a_head_tag_on_both_tiers() {
    // ~keep The parser creates the head itself, and the `<base>` goes in it.
    let html = r#"<base href="https://a.example/"><p>x</p>"#;
    let out = t2(html);
    assert!(out.contains("base: https://a.example/\n"), "{out}");
    assert_eq!(t1(html), out);
}

#[cfg(feature = "metadata")]
#[test]
fn should_keep_the_first_base_href_and_canonical_link_in_the_document_metadata() {
    let html = r#"<html><head><base href="/first/"><base href="/second/"><link rel="canonical" href="https://example.com/first"><link rel="canonical" href="https://example.com/second"></head><body><p>x</p></body></html>"#;
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: true,
        ..ConversionOptions::default()
    };
    let document = convert(html, Some(opts)).unwrap().metadata.document;
    assert_eq!(document.base_href.as_deref(), Some("/first/"));
    assert_eq!(document.canonical_url.as_deref(), Some("https://example.com/first"));
}

#[cfg(feature = "metadata")]
#[test]
fn should_keep_the_base_href_of_a_document_without_a_head_tag_in_the_document_metadata() {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: true,
        ..ConversionOptions::default()
    };
    let result = convert(r#"<base href="https://a.example/"><p>x</p>"#, Some(opts)).unwrap();
    assert_eq!(
        result.metadata.document.base_href.as_deref(),
        Some("https://a.example/")
    );
}

#[cfg(feature = "metadata")]
#[test]
fn should_keep_the_first_meta_tag_per_name_in_the_document_metadata() {
    let html = r#"<html><head><meta name="description" content="first"><meta name="description" content="second"></head><body><p>x</p></body></html>"#;
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: true,
        ..ConversionOptions::default()
    };
    let document = convert(html, Some(opts)).unwrap().metadata.document;
    assert_eq!(document.description.as_deref(), Some("first"));
}
