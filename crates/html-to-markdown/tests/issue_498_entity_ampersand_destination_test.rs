// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #498: `CommonMark` decodes character references inside a link
//! destination or title, so emitting an already-decoded `&plus;` (or `&#43;`, `&#x2B;`) as a
//! bare `&plus;` changes the URL on re-parse (`&plus;` decodes to `+`). `img src` started
//! decoding entities in 3.14.0 (commit f5aec1dfec) without re-escaping them for the Markdown
//! destination it feeds; `<a href>`/`title` had the same defect since 3.12. The fix escapes an
//! entity-shaped `&` back to `&amp;` in a destination or title so it round-trips byte-for-byte
//! through a `CommonMark` parser.
//!
//! Routing Tier-1's `close_link` through the shared `append_url_destination` (rather than
//! interpolating the raw href) also closes a pre-existing Tier-1/Tier-2 divergence on
//! unbalanced-paren and whitespace-bearing destinations, covered by the parity cases below.

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, HighlightStyle, TierStrategy, convert, tier1};

fn content(html: &str) -> String {
    convert(html, Some(ConversionOptions::default()))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

/// Options that clear every classifier gate so Tier 1 is genuinely reachable, mirroring
/// `tier1_scanner_parity_test.rs`'s `base_options`.
fn base_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        ..ConversionOptions::default()
    }
}

/// Asserts Tier-1 and Tier-2 agree and returns the shared output. Calls `tier1::run` directly so
/// a silent bail-to-Tier-2 fallback cannot make the assertion vacuous.
fn assert_tier1_matches_tier2(html: &str) -> String {
    let report = PrescanReport::default();
    let tier1_output = tier1::run(html, &report, &base_options()).expect("tier1 scanner should not bail on this input");

    let tier2_options = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        ..base_options()
    };
    let tier2_output = convert(html, Some(tier2_options))
        .expect("tier2 conversion must succeed")
        .content
        .unwrap_or_default();

    assert_eq!(tier1_output, tier2_output, "Tier-1 and Tier-2 must agree on {html:?}");
    tier1_output
}

#[test]
fn should_keep_an_entity_shaped_ampersand_literal_in_an_image_destination() {
    let out = content(r#"<img src="?&amp;plus;">"#);
    assert_eq!(out, "![](?&amp;plus;)\n", "actual: {out:?}");
}

#[test]
fn should_agree_between_tiers_on_an_entity_shaped_ampersand_in_an_image_destination() {
    let out = assert_tier1_matches_tier2(r#"<img src="?&amp;plus;">"#);
    assert_eq!(out, "![](?&amp;plus;)\n", "actual: {out:?}");
}

#[test]
fn should_keep_an_entity_shaped_ampersand_literal_in_a_link_destination() {
    let out = content(r#"<a href="?&amp;plus;">T</a>"#);
    assert_eq!(out, "[T](?&amp;plus;)\n", "actual: {out:?}");
}

#[test]
fn should_agree_between_tiers_on_an_entity_shaped_ampersand_in_a_link_destination() {
    let out = assert_tier1_matches_tier2(r#"<a href="?&amp;plus;">T</a>"#);
    assert_eq!(out, "[T](?&amp;plus;)\n", "actual: {out:?}");
}

#[test]
fn should_keep_an_entity_shaped_ampersand_literal_in_a_numeric_decimal_reference_destination() {
    let out = content(r#"<a href="?&amp;#43;">T</a>"#);
    assert_eq!(out, "[T](?&amp;#43;)\n", "actual: {out:?}");
}

#[test]
fn should_agree_between_tiers_on_a_numeric_decimal_reference_destination() {
    let out = assert_tier1_matches_tier2(r#"<a href="?&amp;#43;">T</a>"#);
    assert_eq!(out, "[T](?&amp;#43;)\n", "actual: {out:?}");
}

#[test]
fn should_keep_an_entity_shaped_ampersand_literal_in_a_link_title() {
    let out = content(r#"<a href="/x" title="&amp;plus;">T</a>"#);
    assert_eq!(out, "[T](/x \"&amp;plus;\")\n", "actual: {out:?}");
}

#[test]
fn should_agree_between_tiers_on_an_entity_shaped_ampersand_in_a_link_title() {
    let out = assert_tier1_matches_tier2(r#"<a href="/x" title="&amp;plus;">T</a>"#);
    assert_eq!(out, "[T](/x \"&amp;plus;\")\n", "actual: {out:?}");
}

// ~keep ── Controls: a `&` that is not entity-shaped must not change ────────────────────────

#[test]
fn should_not_escape_a_bare_ampersand_in_a_link_destination() {
    let out = content(r#"<a href="?a&amp;b">T</a>"#);
    assert_eq!(out, "[T](?a&b)\n", "actual: {out:?}");
}

#[test]
fn should_agree_between_tiers_on_a_bare_ampersand_in_a_link_destination() {
    let out = assert_tier1_matches_tier2(r#"<a href="?a&amp;b">T</a>"#);
    assert_eq!(out, "[T](?a&b)\n", "actual: {out:?}");
}

// ~keep ── Parity: Tier-1's `close_link` now shares `append_url_destination` with Tier-2 ──────

#[test]
fn should_agree_between_tiers_on_a_destination_with_a_space_and_unbalanced_paren() {
    let out = assert_tier1_matches_tier2(r#"<a href="/a b(c">T</a>"#);
    assert_eq!(out, "[T](</a b(c>)\n", "actual: {out:?}");
}

#[test]
fn should_agree_between_tiers_on_a_destination_with_an_unbalanced_paren() {
    let out = assert_tier1_matches_tier2(r#"<a href="/a(b">T</a>"#);
    assert_eq!(out, "[T](/a\\(b)\n", "actual: {out:?}");
}
