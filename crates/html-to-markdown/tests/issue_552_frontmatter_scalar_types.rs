#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #552: frontmatter values YAML reads as something other than a string.
//!
//! A value such as `3`, `true`, `null` or `2024-01-01` is a valid plain scalar, and a YAML reader
//! resolves it to a number, a boolean, null or a date instead of the string the page wrote. Such a
//! value must be double quoted. The rows cover the YAML 1.2 core schema and the YAML 1.1 types that
//! common readers still apply. Every case runs on both tiers.

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

fn order(content: &str) -> String {
    format!("<html><head><meta name=\"order\" content=\"{content}\"></head><body><p>body</p></body></html>")
}

/// `(type, value)`: each value is a plain scalar that a YAML reader resolves to that type.
const NON_STRING_VALUES: &[(&str, &str)] = &[
    ("null", "null"),
    ("null", "Null"),
    ("null", "~"),
    ("bool", "true"),
    ("bool", "False"),
    ("bool", "TRUE"),
    ("bool 1.1", "yes"),
    ("bool 1.1", "No"),
    ("bool 1.1", "on"),
    ("bool 1.1", "OFF"),
    ("bool 1.1", "y"),
    ("bool 1.1", "N"),
    ("int", "3"),
    ("int", "+3"),
    ("int", "0x1F"),
    ("int", "0o17"),
    ("int 1.1", "017"),
    ("int 1.1", "0b101"),
    ("int 1.1", "1_000"),
    ("int 1.1 base 60", "12:30"),
    ("float", "3.14"),
    ("float", "3."),
    ("float", ".5"),
    ("float", "1e3"),
    ("float", "1.5E+3"),
    ("float", ".inf"),
    ("float", ".NaN"),
    ("float 1.1 base 60", "1:20.5"),
    ("date", "2024-01-01"),
    ("timestamp", "2024-1-1 10:00:00"),
    ("timestamp", "2024-01-01T10:00:00Z"),
    ("timestamp", "2001-12-14t21:59:43.10-05:00"),
    ("value key", "="),
    ("merge key", "<<"),
];

#[test]
fn values_yaml_reads_as_non_strings_are_double_quoted() {
    for (kind, value) in NON_STRING_VALUES {
        for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
            assert_eq!(
                frontmatter(&order(value), tier),
                format!("meta-order: \"{value}\"\n"),
                "{kind} {value:?} on {tier:?}"
            );
        }
    }
}

#[test]
fn strings_that_only_look_like_other_types_stay_plain() {
    for value in [
        "3D",
        "v1.2",
        "1.2.3",
        "127.0.0.1",
        "10 apples",
        "12:30 PM",
        "1:60",
        "1,000",
        "0xZZ",
        "truely",
        "Nope",
        "NULLS",
        "inf",
        "2024-01",
        "2024/01/01",
    ] {
        for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
            assert_eq!(
                frontmatter(&order(value), tier),
                format!("meta-order: {value}\n"),
                "{value:?} on {tier:?}"
            );
        }
    }
}
