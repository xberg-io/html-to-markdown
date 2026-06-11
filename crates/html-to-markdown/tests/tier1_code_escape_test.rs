//! Tier-1 smart code-span backtick escaping (Phase CC).
//!
//! Tier-2's `render_code_with_escaping` (inline/code.rs:260) chooses the
//! number of outer backticks and delimiter spaces based on the content.
//! Tier-1 used to emit a single backtick at open and close — when the
//! content itself contained backticks, the adjacent markers fused with
//! the content and the inner backticks visually disappeared.

#![cfg(feature = "testkit")]

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn tier1(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

fn tier2(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

fn assert_matches(html: &str) {
    let t1 = tier1(html);
    let t2 = tier2(html);
    assert_eq!(
        t1, t2,
        "tier1 diverged from tier2\ninput: {html:?}\ntier1: {t1:?}\ntier2: {t2:?}"
    );
}

#[test]
fn plain_code_uses_single_backticks() {
    assert_matches("<p><code>foo</code></p>");
}

#[test]
fn code_with_inner_backtick_uses_double_with_spaces() {
    assert_matches("<p><code>`#RRGGBB`</code></p>");
}

#[test]
fn code_starting_with_backtick_pads_with_space() {
    assert_matches("<p><code>`x</code></p>");
}

#[test]
fn code_ending_with_backtick_pads_with_space() {
    assert_matches("<p><code>x`</code></p>");
}

#[test]
fn code_with_two_consecutive_backticks_uses_single_backtick() {
    assert_matches("<p><code>x``y</code></p>");
}

#[test]
fn code_inside_table_cell_matches_tier2() {
    assert_matches(
        "<table><thead><tr><th>K</th></tr></thead><tbody><tr><td><code>`a`</code></td></tr></tbody></table>",
    );
}

#[test]
fn empty_code_emits_nothing() {
    assert_matches("<p>before<code></code>after</p>");
}
