// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #499: an unmatched `[` in a link label or image alt text was
//! left unescaped, so `<img alt="[A;B)" src="S">` rendered `![[A;B)](S)`. `CommonMark` 0.31.2
//! (link text / images) requires brackets in a label to be either backslash-escaped or a
//! matched pair; on reparse the inner `[A;B)](S)` forms a real link and the leading `![`
//! dangles, silently dropping the image. An unmatched `]` was already escaped
//! (`![(A;B\]](S)`); only the mirror case -- an unmatched `[` -- was missed.
//!
//! Brackets inside a link *destination* are governed by a different rule entirely (angle-bracket
//! wrapping / percent-encoding of the URL), so `<a href="?[A;B)">` is deliberately unaffected by
//! this fix -- those cases are pinned here as controls, not as coverage of the fix itself.

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, HighlightStyle, TierStrategy, convert, tier1};

fn content(html: &str, escape_ascii: bool) -> String {
    convert(
        html,
        Some(ConversionOptions {
            escape_ascii,
            ..ConversionOptions::default()
        }),
    )
    .expect("conversion should succeed")
    .content
    .unwrap_or_default()
}

/// Options that clear every classifier gate so Tier 1 is genuinely reachable, mirroring
/// `tier1_scanner_parity_test.rs`'s `base_options`.
fn base_options(escape_ascii: bool) -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        escape_ascii,
        ..ConversionOptions::default()
    }
}

/// Asserts Tier-1 and Tier-2 agree and returns the shared output. Calls `tier1::run` directly so
/// a silent bail-to-Tier-2 fallback cannot make the assertion vacuous.
fn assert_tier1_matches_tier2(html: &str, escape_ascii: bool) -> String {
    let report = PrescanReport::default();
    let options = base_options(escape_ascii);
    let tier1_output = tier1::run(html, &report, &options).expect("tier1 scanner should not bail on this input");

    let tier2_options = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        ..options
    };
    let tier2_output = convert(html, Some(tier2_options))
        .expect("tier2 conversion must succeed")
        .content
        .unwrap_or_default();

    assert_eq!(
        tier1_output, tier2_output,
        "Tier-1 and Tier-2 must agree on {html:?} (escape_ascii={escape_ascii})"
    );
    tier1_output
}

// ~keep ── The fix: an unmatched `[` in an image alt is now escaped ──────────────────────

#[test]
fn should_escape_an_unmatched_open_bracket_in_image_alt() {
    let out = content(r#"<img alt="[A;B)" src="S">"#, false);
    assert_eq!(out, "![\\[A;B)](S)\n", "actual: {out:?}");
}

#[test]
fn should_escape_an_unmatched_open_bracket_in_image_alt_with_escape_ascii() {
    let out = content(r#"<img alt="[A;B)" src="S">"#, true);
    assert_eq!(out, "![\\[A;B)](S)\n", "actual: {out:?}");
}

#[test]
fn should_agree_between_tiers_on_an_unmatched_open_bracket_in_image_alt() {
    let out = assert_tier1_matches_tier2(r#"<img alt="[A;B)" src="S">"#, false);
    assert_eq!(out, "![\\[A;B)](S)\n", "actual: {out:?}");
}

#[test]
fn should_agree_between_tiers_on_an_unmatched_open_bracket_in_image_alt_with_escape_ascii() {
    let out = assert_tier1_matches_tier2(r#"<img alt="[A;B)" src="S">"#, true);
    assert_eq!(out, "![\\[A;B)](S)\n", "actual: {out:?}");
}

// ~keep The same hazard in link *text* (not the image-alt shape the issue reported, but the
// ~keep same `escape_link_label` helper serves both, per Tier-1/Tier-2 parity documented in
// ~keep `content.rs`).
#[test]
fn should_escape_an_unmatched_open_bracket_in_link_text() {
    let out = content(r#"<a href="S">[A;B)</a>"#, false);
    assert_eq!(out, "[\\[A;B)](S)\n", "actual: {out:?}");
}

#[test]
fn should_agree_between_tiers_on_an_unmatched_open_bracket_in_link_text() {
    let out = assert_tier1_matches_tier2(r#"<a href="S">[A;B)</a>"#, false);
    assert_eq!(out, "[\\[A;B)](S)\n", "actual: {out:?}");
}

// ~keep ── Controls: shapes the fix must NOT change ──────────────────────────────────────

// ~keep An unmatched `]` in an image alt was already escaped before this fix; the new
// ~keep unmatched-`[` handling must not disturb it.
#[test]
fn should_not_regress_an_unmatched_close_bracket_in_image_alt() {
    let out = content(r#"<img alt="(A;B]" src="S">"#, false);
    assert_eq!(out, "![(A;B\\]](S)\n", "actual: {out:?}");
}

#[test]
fn should_not_regress_an_unmatched_close_bracket_in_image_alt_with_escape_ascii() {
    let out = content(r#"<img alt="(A;B]" src="S">"#, true);
    assert_eq!(out, "![(A;B\\]](S)\n", "actual: {out:?}");
}

// ~keep Brackets in a link *destination* are a different rule entirely (URL escaping, not
// ~keep label escaping) and must be completely unaffected by this fix.
#[test]
fn should_not_regress_an_unmatched_open_bracket_in_a_link_destination() {
    let out = content(r#"<a href="?[A;B)">T</a>"#, false);
    assert_eq!(out, "[T](?[A;B\\))\n", "actual: {out:?}");
}

#[test]
fn should_not_regress_an_unmatched_open_bracket_in_a_link_destination_with_escape_ascii() {
    let out = content(r#"<a href="?[A;B)">T</a>"#, true);
    assert_eq!(out, "[T](?[A;B\\))\n", "actual: {out:?}");
}

#[test]
fn should_not_regress_an_unmatched_close_bracket_in_a_link_destination() {
    let out = content(r#"<a href="?(A;B]">T</a>"#, false);
    assert_eq!(out, "[T](?\\(A;B])\n", "actual: {out:?}");
}

#[test]
fn should_not_regress_an_unmatched_close_bracket_in_a_link_destination_with_escape_ascii() {
    let out = content(r#"<a href="?(A;B]">T</a>"#, true);
    assert_eq!(out, "[T](?\\(A;B])\n", "actual: {out:?}");
}
