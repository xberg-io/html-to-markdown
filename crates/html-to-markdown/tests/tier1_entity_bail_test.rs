//! Tests that Tier-1 bails (with `BailReason::UnknownEntity`) when it encounters
//! a named HTML entity not in its decode table, rather than silently passing the
//! raw `&name;` text through and diverging from Tier-2 output.
//!
//! The task description named `&mdash;`, `&laquo;`, and `&copy;` as examples,
//! but those entities ARE in Tier-1's table.  The tests below use entities that
//! are genuinely absent: `&eacute;`, `&agrave;`, and `&ouml;`.

#![cfg(feature = "testkit")]

use html_to_markdown_rs::prescan::{self, PrescanReport};
use html_to_markdown_rs::tier1::{self, BailReason};
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Prescan `html` then run the Tier-1 scanner directly.
fn tier1_run(html: &str) -> Result<String, BailReason> {
    let (cleaned, report) = prescan::run(html);
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::ForceTier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    tier1::run(cleaned.as_ref(), &report, &opts)
}

/// Run `tier1::run` WITHOUT prescanning so entities are not rewritten.
fn tier1_raw(html: &str) -> Result<String, BailReason> {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::ForceTier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    tier1::run(html, &PrescanReport::default(), &opts)
}

/// `convert()` via the Tier-2-only path.
fn tier2(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2Only,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

/// `convert()` with `ForceTier1` — bails silently and falls back to Tier-2.
fn force_tier1(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::ForceTier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

// ── UnknownEntity tripwire tests ──────────────────────────────────────────────

/// `&eacute;` (é, U+00E9) is not in Tier-1's decode table.
///
/// We bypass the prescan (`tier1_raw`) because the prescan rewrites named
/// entities to numeric form before Tier-1 sees them.
#[test]
fn bails_on_unknown_named_entity_eacute() {
    let html = "<p>caf&eacute;</p>";
    let result = tier1_raw(html);
    let err = result.unwrap_err();
    assert!(
        matches!(err, BailReason::UnknownEntity { ref name, .. } if name == "eacute"),
        "expected UnknownEntity {{ name: \"eacute\", .. }}, got: {err}"
    );
}

/// `&agrave;` (à, U+00E0) is not in Tier-1's decode table.
#[test]
fn bails_on_unknown_named_entity_agrave() {
    let html = "<p>voil&agrave;</p>";
    let result = tier1_raw(html);
    let err = result.unwrap_err();
    assert!(
        matches!(err, BailReason::UnknownEntity { ref name, .. } if name == "agrave"),
        "expected UnknownEntity {{ name: \"agrave\", .. }}, got: {err}"
    );
}

/// `&ouml;` (ö, U+00F6) is not in Tier-1's decode table.
#[test]
fn bails_on_unknown_named_entity_ouml() {
    let html = "<p>M&ouml;bius</p>";
    let result = tier1_raw(html);
    let err = result.unwrap_err();
    assert!(
        matches!(err, BailReason::UnknownEntity { ref name, .. } if name == "ouml"),
        "expected UnknownEntity {{ name: \"ouml\", .. }}, got: {err}"
    );
}

/// Numeric hex reference `&#x1F600;` (emoji) should be decoded by the numeric
/// path and NOT trigger a bail.
#[test]
fn does_not_bail_on_numeric_hex_reference() {
    let html = "<p>emoji &#x1F600; end</p>";
    let result = tier1_raw(html);
    let md = result.expect("numeric hex reference should decode without bailing");
    assert!(md.contains('\u{1F600}'), "expected emoji in output, got: {md:?}");
}

/// `&amp;` and `&lt;` are in the decode table and must not bail.
#[test]
fn does_not_bail_on_amp_and_lt() {
    let html = "<p>a &amp; b &lt; c</p>";
    let result = tier1_raw(html);
    let md = result.expect("&amp; and &lt; are known entities — should not bail");
    assert!(md.contains('&'), "expected '&' in output, got: {md:?}");
    assert!(md.contains('<'), "expected '<' in output, got: {md:?}");
}

/// `convert(html_with_eacute, Some(Auto))` returns `Ok(_)`: the dispatcher
/// falls back to Tier-2 after the scanner bails, so the overall conversion
/// succeeds and the é character appears in the result.
#[test]
fn convert_auto_falls_back_on_unknown_entity_and_succeeds() {
    let html = "<p>caf&eacute;</p>";
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    let result = convert(html, Some(opts));
    let output = result.expect("convert must not return Err").content.unwrap_or_default();
    // Tier-2 decodes &eacute; → U+00E9 (é).
    assert!(
        output.contains('\u{00E9}'),
        "expected é character in Tier-2 fallback output, got: {output:?}"
    );
}

/// `force_tier1(html)` produces the same output as `tier2(html)` for content
/// with `&eacute;`.  The bail causes the dispatcher to fall back to Tier-2.
#[test]
fn force_tier1_matches_tier2_for_unknown_entity() {
    let html = "<p>caf&eacute;</p>";
    assert_eq!(
        force_tier1(html),
        tier2(html),
        "ForceTier1 fallback must produce the same output as Tier-2"
    );
}

/// The prescan does NOT rewrite named entities.  `tier1_run` (which prescans
/// first) still bails with `UnknownEntity` for `&eacute;` because the prescan
/// passes the entity through unchanged.  The `convert()` dispatcher then falls
/// back to Tier-2, which produces the correct output.
#[test]
fn prescan_does_not_rewrite_named_entity_so_tier1_still_bails() {
    let html = "<p>caf&eacute;</p>";
    // The prescan does not rewrite named entities to numeric form.
    // Tier-1 therefore bails with UnknownEntity even after prescanning.
    let result = tier1_run(html);
    assert!(
        matches!(result.unwrap_err(), BailReason::UnknownEntity { ref name, .. } if name == "eacute"),
        "Tier-1 should bail with UnknownEntity after prescanning since prescan does not rewrite named entities"
    );
}
