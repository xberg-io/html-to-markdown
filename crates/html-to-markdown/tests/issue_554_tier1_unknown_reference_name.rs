#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #554: an unknown reference name before a nearby `;`.
//!
//! Tier 1 looked ahead for a `;` and, when the name before it was not a reference, wrote the whole
//! span `&name ... ;` as it was. A reference inside that span stayed encoded and its whitespace was
//! not collapsed, where Tier 2 writes the `&` and reads on from the next byte. Tier 1 runs here
//! without its fallback, so a bail cannot hide a difference.

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::tier1;
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn options(tier: TierStrategy) -> ConversionOptions {
    ConversionOptions {
        tier_strategy: tier,
        extract_metadata: false,
        ..ConversionOptions::default()
    }
}

/// `(label, html, expected Markdown)`.
const CASES: &[(&str, &str, &str)] = &[
    ("named reference inside", "<p>&foo &amp; bar;</p>", "&foo & bar;"),
    ("hot-table name inside", "<p>&foo &copy; x</p>", "&foo \u{a9} x"),
    ("numeric reference inside", "<p>&x &#65; y</p>", "&x A y"),
    ("whitespace inside", "<p>&foo    bar;</p>", "&foo bar;"),
    ("unknown name alone", "<p>&bogusname; x</p>", "&bogusname; x"),
    (
        "in an attribute",
        "<p><img src=\"a.png\" alt=\"&zz &amp; b;\"></p>",
        "![&zz & b;](a.png)",
    ),
];

#[test]
fn both_tiers_agree_on_a_reference_after_an_unknown_name() {
    for (label, html, expected) in CASES {
        let tier1 = tier1::run(html, &PrescanReport::default(), &options(TierStrategy::Tier1))
            .unwrap_or_else(|reason| panic!("{label}: Tier 1 bailed: {reason:?}"));
        let tier2 = convert(html, Some(options(TierStrategy::Tier2)))
            .expect("conversion should succeed")
            .content
            .unwrap_or_default();
        assert_eq!(tier2.trim_end(), *expected, "{label} on Tier2");
        assert_eq!(tier1.trim_end(), *expected, "{label} on Tier1");
    }
}
