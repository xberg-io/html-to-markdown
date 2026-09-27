#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #544: a head value could end its frontmatter line.
//!
//! The frontmatter wrote `key: value` verbatim, so a newline in a value (literal, or decoded
//! from `&#10;`) started a new line that YAML reads as a new key. Every key and value must stay
//! one YAML scalar. Every case runs on both tiers, which share the frontmatter writer but reach
//! the head through different entry points.

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn frontmatter(html: &str, tier: TierStrategy) -> String {
    let options = ConversionOptions {
        tier_strategy: tier,
        extract_metadata: true,
        ..ConversionOptions::default()
    };
    let out = convert(html, Some(options))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default();
    let body = out.strip_prefix("---\n").expect("output starts with frontmatter");
    let end = body.find("\n---\n").expect("frontmatter is closed");
    body[..=end].to_string()
}

fn description(content: &str) -> String {
    format!("<html><head><meta name=\"description\" content=\"{content}\"></head><body><p>body</p></body></html>")
}

fn assert_frontmatter(html: &str, expected: &str) {
    for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
        assert_eq!(frontmatter(html, tier), expected, "{tier:?} frontmatter for {html:?}");
    }
}

#[test]
fn newline_in_a_value_cannot_add_a_key() {
    // ~keep Decoded from `&#10;` and written literally: both must stay inside one scalar.
    for content in ["Hello&#10;injected: yes", "Hello\ninjected: yes"] {
        assert_frontmatter(&description(content), "meta-description: \"Hello\\ninjected: yes\"\n");
    }
}

#[test]
fn newline_in_the_title_cannot_add_a_key() {
    let html = "<html><head><title>A&#10;canonical: https://evil.example/</title></head><body><p>b</p></body></html>";
    assert_frontmatter(html, "title: \"A\\ncanonical: https://evil.example/\"\n");
}

#[test]
fn newline_in_a_key_cannot_add_a_key() {
    let html = "<html><head><meta name=\"a\nb: c\" content=\"v\"></head><body><p>b</p></body></html>";
    assert_frontmatter(html, "\"meta-a\\nb: c\": v\n");
}

/// `(label, raw content, expected frontmatter line)` for values that are not a plain YAML scalar.
const QUOTED_CASES: &[(&str, &str, &str)] = &[
    ("colon space", "Key: Value", "meta-description: \"Key: Value\"\n"),
    ("trailing colon", "Key:", "meta-description: \"Key:\"\n"),
    ("leading dash", "- item", "meta-description: \"- item\"\n"),
    ("comment", "#2b5797", "meta-description: \"#2b5797\"\n"),
    ("space hash", "a #b", "meta-description: \"a #b\"\n"),
    ("reserved at", "@reactjs", "meta-description: \"@reactjs\"\n"),
    ("flow start", "[a]", "meta-description: \"[a]\"\n"),
    (
        "leading quote, backslash",
        "&quot;a\\b&quot; said",
        "meta-description: \"\\\"a\\\\b\\\" said\"\n",
    ),
    ("carriage return", "a&#13;b", "meta-description: \"a\\rb\"\n"),
    ("tab", "a&#9;b", "meta-description: \"a\\tb\"\n"),
    ("line separator", "a\u{2028}b", "meta-description: \"a\\u2028b\"\n"),
    ("leading space", "&#32;a", "meta-description: \" a\"\n"),
    ("empty", "", "meta-description: \"\"\n"),
];

#[test]
fn values_that_are_not_plain_scalars_are_double_quoted() {
    for (label, content, expected) in QUOTED_CASES {
        for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
            assert_eq!(
                frontmatter(&description(content), tier),
                *expected,
                "{label} on {tier:?}"
            );
        }
    }
}

#[test]
fn plain_values_stay_plain() {
    // ~keep A colon not followed by a space, a comma and a dash inside a value are plain YAML.
    for content in [
        "https://react.dev/learn",
        "width=device-width, initial-scale=1",
        "Quick Start \u{2013} React",
        "a-b",
    ] {
        assert_frontmatter(&description(content), &format!("meta-description: {content}\n"));
    }
    // ~keep A plain scalar takes a quote or a backslash after its first character literally.
    assert_frontmatter(&description("say &quot;a\\b&quot;"), "meta-description: say \"a\\b\"\n");
    let html = "<html><head><meta property=\"og:title\" content=\"T\"></head><body><p>b</p></body></html>";
    assert_frontmatter(html, "meta-og:title: T\n");
}
