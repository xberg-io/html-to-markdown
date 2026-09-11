//! Positive-trigger coverage for the two emphasis-related `BailReason` variants added for
//! issues #481/#483:
//!
//!   - `AdjacentInlineEmphasis`        — NEW (this file)
//!   - `WhitespaceOnlyInlineEmphasis`  — NEW (this file)
//!
//! Each `tier1_run` call below goes straight through `tier1::run`, bypassing `convert()`'s
//! Auto-strategy classifier entirely, so a bail is a hard `Err` return here rather than a
//! silent fall-through to Tier-2 that would trivially match itself.

#![cfg(feature = "testkit")]

use html_to_markdown_rs::prescan;
use html_to_markdown_rs::tier1::{self, BailReason};
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn tier1_run(html: &str) -> Result<String, BailReason> {
    let (cleaned, report) = prescan::run(html);
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    tier1::run(cleaned.as_ref(), &report, &opts)
}

fn tier2(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

/// Runs through `convert()`'s default (Auto) dispatch, proving the bail actually routes to
/// Tier-2 end-to-end rather than merely returning `Err` from a direct `tier1::run` call.
fn auto(html: &str) -> String {
    let opts = ConversionOptions {
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

// ~keep ── AdjacentInlineEmphasis ────────────────────────────────────────────────────

#[test]
fn should_bail_when_strong_opens_right_after_a_matching_strong_close() {
    let err = tier1_run("<strong>A</strong><strong>B</strong>").unwrap_err();
    assert!(
        matches!(err, BailReason::AdjacentInlineEmphasis),
        "expected AdjacentInlineEmphasis, got {err:?}"
    );
}

#[test]
fn should_bail_when_emphasis_opens_right_after_a_matching_emphasis_close() {
    let err = tier1_run("<i>A</i><i>B</i>").unwrap_err();
    assert!(
        matches!(err, BailReason::AdjacentInlineEmphasis),
        "expected AdjacentInlineEmphasis, got {err:?}"
    );
}

#[test]
fn should_bail_for_three_adjacent_emphasis_elements() {
    let err = tier1_run("<i>A</i><i>B</i><i>C</i>").unwrap_err();
    assert!(
        matches!(err, BailReason::AdjacentInlineEmphasis),
        "expected AdjacentInlineEmphasis, got {err:?}"
    );
}

#[test]
fn should_not_bail_when_emphasis_elements_are_separated_by_a_real_space() {
    // ~keep Control: a genuine space between the elements means the buffer does not end
    // ~keep with the matching close marker when the second opens.
    tier1_run("<i>A</i> <i>B</i>").expect("must not bail on the control case");
}

#[test]
fn should_not_bail_across_different_emphasis_kinds() {
    // ~keep `<em>` closing (`*`) into `<strong>` opening (`**`) is not the same delimiter
    // ~keep kind -- Tier-1's own byte-for-byte output for this shape is correct without a
    // ~keep merge (verified against the comrak oracle: `*A***B**` parses as
    // ~keep `<em>A</em><strong>B</strong>`).
    tier1_run("<i>A</i><b>B</b>").expect("must not bail across different emphasis kinds");
}

#[test]
fn adjacent_inline_emphasis_bail_falls_back_to_tier2_end_to_end() {
    let html = "<strong>A</strong><strong>B</strong>";
    assert_eq!(auto(html), "**AB**\n");
    assert_eq!(auto(html), tier2(html));
}

// ~keep ── WhitespaceOnlyInlineEmphasis ──────────────────────────────────────────────

#[test]
fn should_bail_when_an_emphasis_body_is_whitespace_only() {
    let err = tier1_run("A<i> </i>B").unwrap_err();
    assert!(
        matches!(err, BailReason::WhitespaceOnlyInlineEmphasis),
        "expected WhitespaceOnlyInlineEmphasis, got {err:?}"
    );
}

#[test]
fn should_bail_when_a_strong_body_is_whitespace_only() {
    let err = tier1_run("A<strong> </strong>B").unwrap_err();
    assert!(
        matches!(err, BailReason::WhitespaceOnlyInlineEmphasis),
        "expected WhitespaceOnlyInlineEmphasis, got {err:?}"
    );
}

#[test]
fn should_not_bail_on_a_genuinely_empty_emphasis_body() {
    // ~keep Control: `<i></i>` (no whitespace at all) is a DIFFERENT case from
    // ~keep `<i> </i>` -- both tiers agree it contributes nothing, so no bail is needed.
    let markdown = tier1_run("A<i></i>B").expect("must not bail on a genuinely empty body");
    assert_eq!(markdown, "AB\n");
    assert_eq!(markdown, tier2("A<i></i>B"));
}

#[test]
fn should_not_bail_when_leading_whitespace_is_followed_by_more_content() {
    // ~keep Regression guard for `tier1_leading_inline_whitespace_test.rs`: leading
    // ~keep whitespace followed by a NESTED TAG (not just more text) must still take the
    // ~keep existing migration path, not the new whitespace-only-body bail -- the body
    // ~keep ends up non-empty once the nested tag's own content is written.
    let html = "<p><em> <b>x</b></em></p>";
    let markdown = tier1_run(html).expect("must not bail when real content follows the leading whitespace");
    assert_eq!(markdown, tier2(html));
}

#[test]
fn whitespace_only_inline_emphasis_bail_falls_back_to_tier2_end_to_end() {
    let html = "A<i> </i>B";
    assert_eq!(auto(html), "A B\n");
    assert_eq!(auto(html), tier2(html));
}

// ~keep ── Display strings ───────────────────────────────────────────────────────────

#[test]
fn bail_reason_display_strings_are_non_empty() {
    for reason in [
        BailReason::AdjacentInlineEmphasis,
        BailReason::WhitespaceOnlyInlineEmphasis,
    ] {
        let s = reason.to_string();
        assert!(!s.is_empty(), "Display for {reason:?} produced empty string");
    }
}
