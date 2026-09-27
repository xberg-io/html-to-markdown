#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #545: legacy named character references without a `;`.
//!
//! The WHATWG named character reference table lets about a hundred names (`copy`, `amp`, `not`,
//! `eacute`, ...) close without a `;`, and a browser decodes them in text. In an attribute value
//! the spec leaves one alone when the next character is `=` or ASCII alphanumeric, which keeps
//! query strings such as `?a=1&copy=2` intact. Every case runs on both tiers.

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn convert_with(html: &str, tier: TierStrategy) -> String {
    let options = ConversionOptions {
        tier_strategy: tier,
        ..ConversionOptions::default()
    };
    convert(html, Some(options))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

fn assert_converts(html: &str, expected: &str) {
    for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
        assert_eq!(convert_with(html, tier).trim_end(), expected, "{tier:?} for {html:?}");
    }
}

/// `(label, paragraph text, expected Markdown)` for references in text.
const TEXT_CASES: &[(&str, &str, &str)] = &[
    ("before a space", "&copy 2024 Example", "\u{a9} 2024 Example"),
    ("at the end", "Example &copy", "Example \u{a9}"),
    ("before a letter", "&copyright", "\u{a9}right"),
    ("before an equals sign", "a=1&copy=2", "a=1\u{a9}=2"),
    (
        "longest legacy prefix",
        "I'm &notit; I tell you",
        "I'm \u{ac}it; I tell you",
    ),
    ("accented letter", "caf&eacute au lait", "caf\u{e9} au lait"),
    ("ampersand", "Tom &amp Jerry", "Tom & Jerry"),
    ("decoded once", "&amp;copy 2024", "&copy 2024"),
    ("semicolon form still decodes", "&notin; set", "\u{2209} set"),
    ("not a legacy name", "&hellip and more", "&hellip and more"),
];

#[test]
fn legacy_references_in_text_are_decoded() {
    for (label, text, expected) in TEXT_CASES {
        for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
            let html = format!("<p>{text}</p>");
            assert_eq!(convert_with(&html, tier).trim_end(), *expected, "{label} on {tier:?}");
        }
    }
}

#[test]
fn legacy_reference_before_equals_or_alphanumeric_stays_in_an_href() {
    assert_converts(
        r#"<p><a href="https://example.com/?a=1&copy=2&notx=3">link</a></p>"#,
        "[link](https://example.com/?a=1&copy=2&notx=3)",
    );
}

#[test]
fn legacy_reference_before_other_characters_decodes_in_an_attribute() {
    assert_converts(
        r#"<p><img src="a.png" alt="&copy 2024 Example"></p>"#,
        "![\u{a9} 2024 Example](a.png)",
    );
    assert_converts(
        r#"<p><a href="https://example.com/?a=1&amp;b=2" title="Tom &amp Jerry">x</a></p>"#,
        "[x](https://example.com/?a=1&b=2 \"Tom & Jerry\")",
    );
}

#[test]
fn legacy_references_in_head_metadata_follow_the_same_rules() {
    let html = r#"<html><head><title>&copy 2024</title><meta name="description" content="Tom &amp Jerry"><base href="https://example.com/?a=1&copy=2"></head><body><p>b</p></body></html>"#;
    for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
        let options = ConversionOptions {
            tier_strategy: tier,
            extract_metadata: true,
            ..ConversionOptions::default()
        };
        let out = convert(html, Some(options)).unwrap().content.unwrap_or_default();
        for line in [
            "title: \u{a9} 2024",
            "meta-description: Tom & Jerry",
            "base: https://example.com/?a=1&copy=2",
        ] {
            assert!(out.lines().any(|l| l == line), "{tier:?}: expected {line:?} in {out:?}");
        }
    }
}

#[test]
fn code_block_language_class_is_read_as_an_attribute() {
    assert_converts(
        r#"<pre><code class="language-a&notb">x</code></pre>"#,
        "```a&notb\nx\n```",
    );
}

/// Tier 1 reads attribute values itself. A legacy name before `=` in an `href` is not a
/// reference there, so Tier 1 keeps it and does not bail.
#[test]
fn tier1_keeps_a_legacy_name_in_an_href_without_bailing() {
    let options = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        ..ConversionOptions::default()
    };
    let html = r#"<p><a href="https://example.com/?a=1&copy=2">link</a></p>"#;
    let out = html_to_markdown_rs::tier1::run(html, &html_to_markdown_rs::prescan::PrescanReport::default(), &options)
        .expect("an href query needs no Tier 2");
    assert_eq!(out.trim_end(), "[link](https://example.com/?a=1&copy=2)");
}

/// JSON-LD is script text, which a browser never decodes. It keeps the attribute rule, so a
/// query string inside it survives.
#[cfg(feature = "metadata")]
#[test]
fn json_ld_keeps_a_legacy_name_in_a_query_string() {
    let html = r#"<html><head><script type="application/ld+json">{"@type":"Article","url":"https://example.com/?a=1&copy=2"}</script></head><body><p>b</p></body></html>"#;
    let metadata = convert(html, None).expect("convert failed").metadata;
    assert_eq!(metadata.structured_data.len(), 1);
    assert!(
        metadata.structured_data[0]
            .raw_json
            .contains("https://example.com/?a=1&copy=2"),
        "{:?}",
        metadata.structured_data[0].raw_json
    );
}
