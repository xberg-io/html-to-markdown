#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #553: numeric character references without a closing `;`.
//!
//! The spec's numeric character reference states read the digit run, take a `;` when one follows,
//! and decode the reference either way, in text and in attribute values. `&#39` shows `'` in a
//! browser. Every case runs on both tiers.

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::tier1::{self, BailReason};
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
    ("decimal before a letter", "it&#39s", "it's"),
    ("hex before a space", "it&#x27 s", "it' s"),
    ("upper-case hex marker", "&#X41BC", "\u{41bc}"),
    ("at the end of input", "end&#65", "endA"),
    ("before another reference", "&#65&#66;", "AB"),
    ("windows-1252 override", "a&#150b", "a\u{2013}b"),
    ("semicolon form unchanged", "&#65;", "A"),
    ("no digits stays", "&#x; &#;", "&#x; &#;"),
];

#[test]
fn numeric_references_without_a_semicolon_decode_in_text() {
    for (label, text, expected) in TEXT_CASES {
        for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
            let html = format!("<p>{text}</p>");
            assert_eq!(convert_with(&html, tier).trim_end(), *expected, "{label} on {tier:?}");
        }
    }
}

#[test]
fn numeric_references_without_a_semicolon_decode_in_attributes() {
    // ~keep The attribute rule that keeps `&copy=` applies to named references only.
    assert_converts(
        "<p><a href=\"https://example.com/?a=1&#38b=2\">x</a></p>",
        "[x](https://example.com/?a=1&b=2)",
    );
    assert_converts("<p><img src=\"a.png\" alt=\"it&#39s\"></p>", "![it's](a.png)");
}

#[test]
fn tier1_hands_a_numeric_reference_without_a_semicolon_to_tier2() {
    let options = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    let result = tier1::run("<p>it&#39s</p>", &PrescanReport::default(), &options);
    assert!(
        matches!(&result, Err(BailReason::UnknownEntity { name, .. }) if &**name == "#39"),
        "expected an UnknownEntity bail naming #39, got {result:?}"
    );
}
