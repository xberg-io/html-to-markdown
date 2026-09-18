// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #501: a whitespace-only `<span>` at the start of a `<div>` leaked
//! its spaces into the output at a line start, so `<span>    </span><img ...>` rendered as
//! `    ![A](S)` -- four columns of indentation, which `CommonMark` reads as an indented code block.
//!
//! Issue #460 dropped leading whitespace at the start of a *paragraph*; `<div>` never sets
//! `in_paragraph`, so that guard did not fire. The rule is now the general one: an ASCII-only
//! whitespace run at the start of a line is insignificant in Markdown and is never emitted.
//! Tier 1 already got this right, so this was a live cross-tier divergence.

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, HighlightStyle, TierStrategy, convert, tier1};

const ISSUE_501: &str = "<p>P</p>\n<div>\n<span>    </span><img alt=\"A\" src=\"S\"/>\n</div>";

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
fn should_not_indent_an_image_after_a_whitespace_only_span_at_a_div_start() {
    let out = content(ISSUE_501);
    assert_eq!(out, "P\n\n![A](S)\n", "actual: {out:?}");
}

#[test]
fn should_agree_across_tiers_for_a_whitespace_only_span_at_a_div_start() {
    let out = assert_tier1_matches_tier2(ISSUE_501);
    assert_eq!(out, "P\n\n![A](S)\n", "actual: {out:?}");
}

#[test]
fn should_drop_line_start_whitespace_before_text_in_a_div_after_a_list() {
    let out = content("<ul><li>x</li></ul><div><span>    </span>after</div>");
    assert_eq!(out, "- x\n\nafter\n", "actual: {out:?}");
}

#[test]
fn should_drop_a_whitespace_only_wrapper_at_the_start_of_a_second_paragraph() {
    let out = content("<p>A</p><p><i> </i>B</p>");
    assert_eq!(out, "A\n\nB\n", "actual: {out:?}");
}

#[test]
fn should_keep_a_separating_space_between_inline_siblings_mid_line() {
    let html = "<div><b>A</b><span>    </span><img alt=\"I\" src=\"S\"/></div>";
    let out = content(html);
    assert_eq!(out, "**A** ![I](S)\n", "actual: {out:?}");
    assert_eq!(assert_tier1_matches_tier2(html), "**A** ![I](S)\n");
}

#[test]
fn should_keep_a_non_breaking_space_only_span_at_a_line_start() {
    // ~keep A decoded `&nbsp;` is significant content, not formatting whitespace: the rule is
    // ~keep ASCII-only, so this run survives exactly as it did before.
    let out = content("<p>P</p><div><span>\u{a0}</span>after</div>");
    assert_eq!(out, "P\n\n\u{a0}after\n", "actual: {out:?}");
}
