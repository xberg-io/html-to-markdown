// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #505: a newline-only text node inside two nested transparent
//! inline wrappers was discarded, welding the words on either side together --
//! `<p><i>Alpha</i><span><span>\n</span></span>Beta</p>` rendered `*Alpha*Beta`.
//!
//! Issues #430 and #491 taught the text-node fallback that a lone newline inside an inline
//! wrapper still separates words when the wrapper is followed by inline content, but the check
//! looked one level up only: with a second wrapper the inner `<span>` is the last child of the
//! outer one, has no following sibling, and the newline was dropped. The check now climbs
//! through every transparent inline ancestor that has no following sibling of its own and stops
//! at the first block.
//!
//! Tier 1 already emitted the space, so this was a live cross-tier divergence; parity is
//! asserted by calling `tier1::run` directly so a silent bail cannot make it vacuous.

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, HighlightStyle, TierStrategy, convert, tier1};

const ISSUE_505: &str = "<p><i>Alpha</i><span><span>\n</span></span>Beta</p>";
const ISSUE_505_BARE: &str = "<p>Alpha<span><span>\n</span></span>Beta</p>";

fn content(html: &str) -> String {
    convert(html, Some(ConversionOptions::default()))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

fn base_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        ..ConversionOptions::default()
    }
}

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
fn should_keep_a_separator_for_a_newline_only_span_nested_in_a_span_after_emphasis() {
    let out = content(ISSUE_505);
    assert_eq!(out, "*Alpha* Beta\n", "actual: {out:?}");
}

#[test]
fn should_keep_a_separator_for_a_newline_only_span_nested_in_a_span_between_words() {
    let out = content(ISSUE_505_BARE);
    assert_eq!(out, "Alpha Beta\n", "actual: {out:?}");
}

#[test]
fn should_agree_between_tiers_on_nested_newline_only_spans() {
    let output = assert_tier1_matches_tier2(ISSUE_505_BARE);
    assert_eq!(output, "Alpha Beta\n", "actual: {output:?}");
    let output = assert_tier1_matches_tier2(ISSUE_505);
    assert_eq!(output, "*Alpha* Beta\n", "actual: {output:?}");
}

#[test]
fn should_climb_through_three_transparent_wrappers() {
    let out = content("<p>Alpha<span><span><span>\n</span></span></span>Beta</p>");
    assert_eq!(out, "Alpha Beta\n", "actual: {out:?}");
}

#[test]
fn should_emit_nothing_for_nested_newline_only_spans_at_the_end_of_their_block() {
    let out = content("<p>Alpha<span><span>\n</span></span></p>");
    assert_eq!(out, "Alpha\n", "actual: {out:?}");
}

#[test]
fn should_not_double_a_separator_when_the_following_text_starts_with_whitespace() {
    let out = content("<p><i>Alpha</i><span><span>\n</span></span> Beta</p>");
    assert_eq!(out, "*Alpha* Beta\n", "actual: {out:?}");
}

#[test]
fn should_not_double_a_separator_when_the_outer_span_ends_with_whitespace() {
    let out = content("<p>Alpha<span><span>\n</span>   </span>Beta</p>");
    assert_eq!(out, "Alpha Beta\n", "actual: {out:?}");
}

#[test]
fn should_stop_at_a_block_ancestor() {
    let out = content("<div><span><span>\n</span></span></div>Beta");
    assert_eq!(out, "Beta\n", "actual: {out:?}");
}
