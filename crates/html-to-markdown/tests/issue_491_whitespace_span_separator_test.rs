// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #491: a whitespace-only inline wrapper whose content is a newline
//! was discarded entirely when the wrapper was followed by a bare text node, welding the words on
//! either side together (`Alpha<span>\n</span>13` -> `Alpha13`).
//!
//! The reported input carries `style="white-space:pre"`, but that attribute is a red herring:
//! `white-space` is not implemented anywhere in this crate (the only inline CSS read is
//! `display`/`visibility`/`font-size`), and the defect reproduces without it. A browser renders
//! `Alpha<span>\n</span>13` as `Alpha 13` either way, so collapsing the newline to a separating
//! space is the correct HTML semantics independent of any CSS.
//!
//! Issue #430 already fixed the case where the wrapper is followed by another *element*
//! (`<span>\n</span><span>13</span>`). The predicate it added asks
//! `DomContext::next_inline_like`, whose scan returns `false` on the first non-whitespace raw
//! text sibling — so a bare `13` after the wrapper took the failing path and nothing was emitted.
//!
//! Tier 1 was already correct here, so this was a live cross-tier divergence, visible from the
//! CLI today via `--highlight-style none` (the last router gate for this input).

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, HighlightStyle, TierStrategy, convert, tier1};

const ISSUE_491_STYLED: &str = "Alpha<span style=\"white-space:pre\">\n</span>13";
const ISSUE_491_BARE: &str = "Alpha<span>\n</span>13";

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
fn should_keep_a_separator_when_a_newline_only_span_precedes_bare_text() {
    let out = content(ISSUE_491_STYLED);
    assert_eq!(out, "Alpha 13\n", "actual: {out:?}");
}

#[test]
fn should_keep_a_separator_regardless_of_the_white_space_style_attribute() {
    assert_eq!(
        content(ISSUE_491_STYLED),
        content(ISSUE_491_BARE),
        "`white-space: pre` is not implemented; it must not change the outcome"
    );
}

#[test]
fn should_agree_between_tiers_on_a_newline_only_span_before_bare_text() {
    let output = assert_tier1_matches_tier2(ISSUE_491_BARE);
    assert_eq!(output, "Alpha 13\n", "actual: {output:?}");
}

// ~keep ── Controls: the shapes that already worked must keep working ─────────────────

#[test]
fn should_not_regress_the_span_wrapped_sibling_case() {
    // ~keep Issue #430's original shape: the wrapper is followed by an element, not bare text.
    let out = content("<span>1 mezzo</span><span>\n</span><span>con carico</span>");
    assert_eq!(out, "1 mezzo con carico\n", "actual: {out:?}");
}

#[test]
fn should_emit_nothing_for_a_newline_only_span_at_the_end_of_its_parent() {
    // ~keep No following sibling at all means there is no "separate the next word" question to
    // ~keep ask, so the newline must still vanish rather than become a trailing space.
    let out = content("<p>Alpha<span>\n</span></p>");
    assert_eq!(out, "Alpha\n", "actual: {out:?}");
}

#[test]
fn should_not_double_a_separator_when_the_wrapper_follows_a_space() {
    let out = content("Alpha <span>\n</span>13");
    assert_eq!(out, "Alpha 13\n", "actual: {out:?}");
}
