//! Tests Tier-1's named-HTML-entity handling.
//!
//! Phase N3 changed policy: Tier-1 no longer bails on unknown `&name;` —
//! it passes the raw entity through to match Tier-2/mdream, which also
//! preserve genuinely-unknown entities verbatim.  Latin-1 entities like
//! `&eacute;` ARE decoded by both Tier-1 and Tier-2.  Truly-unknown
//! entities like `&notarealentityname;` survive as-is.

#![cfg(feature = "testkit")]

use html_to_markdown_rs::prescan::{self, PrescanReport};
use html_to_markdown_rs::tier1::{self, BailReason};
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Prescan `html` then run the Tier-1 scanner directly.
fn tier1_run(html: &str) -> Result<String, BailReason> {
    let (cleaned, report) = prescan::run(html);
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    tier1::run(cleaned.as_ref(), &report, &opts)
}

/// Run `tier1::run` WITHOUT prescanning so entities are not rewritten.
fn tier1_raw(html: &str) -> Result<String, BailReason> {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    tier1::run(html, &PrescanReport::default(), &opts)
}

/// `convert()` via the Tier-2-only path.
fn tier2(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

/// `convert()` with `Tier1` — bails silently and falls back to Tier-2.
fn force_tier1(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

// ── Entity decode tests ───────────────────────────────────────────────────────

/// `&eacute;` (é, U+00E9) is in Tier-1's Latin-1 table and decodes inline.
#[test]
fn decodes_named_entity_eacute() {
    let html = "<p>calf&eacute;</p>";
    let md = tier1_raw(html).expect("eacute decodes via Latin-1 table");
    assert!(md.contains('\u{00E9}'), "expected 'é' in output, got: {md:?}");
}

/// `&agrave;` (à, U+00E0) decodes via the Latin-1 table.
#[test]
fn decodes_named_entity_agrave() {
    let html = "<p>voil&agrave;</p>";
    let md = tier1_raw(html).expect("agrave decodes via Latin-1 table");
    assert!(md.contains('\u{00E0}'), "expected 'à' in output, got: {md:?}");
}

/// `&ouml;` (ö, U+00F6) decodes via the Latin-1 table.
#[test]
fn decodes_named_entity_ouml() {
    let html = "<p>M&ouml;bius</p>";
    let md = tier1_raw(html).expect("ouml decodes via Latin-1 table");
    assert!(md.contains('\u{00F6}'), "expected 'ö' in output, got: {md:?}");
}

/// A genuinely-unknown entity (`&notarealentityname;`) passes through raw —
/// neither Tier-1 nor Tier-2 decode it, so byte-equality is preserved without
/// the historical `UnknownEntity` bail.
#[test]
fn passes_unknown_entity_through_raw() {
    let html = "<p>x &notarealentityname; y</p>";
    let md = tier1_raw(html).expect("unknown entity passes through, no bail");
    assert!(
        md.contains("&notarealentityname;"),
        "expected raw entity preserved, got: {md:?}"
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

/// `convert(html_with_eacute, Some(Auto))` returns the decoded é character —
/// Tier-1 decodes `&eacute;` directly via its Latin-1 table.
#[test]
fn convert_auto_decodes_latin1_entity() {
    let html = "<p>calf&eacute;</p>";
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    let result = convert(html, Some(opts));
    let output = result.expect("convert must not return Err").content.unwrap_or_default();
    assert!(
        output.contains('\u{00E9}'),
        "expected é character in decoded output, got: {output:?}"
    );
}

/// `force_tier1(html)` matches `tier2(html)` for Latin-1 entities — both decode.
#[test]
fn force_tier1_matches_tier2_for_latin1_entity() {
    let html = "<p>calf&eacute;</p>";
    assert_eq!(
        force_tier1(html),
        tier2(html),
        "Tier1 must produce the same output as Tier-2 for Latin-1 entities"
    );
}

/// The prescan does NOT rewrite named entities; Tier-1 still decodes
/// `&eacute;` via its own Latin-1 table after prescanning.
#[test]
fn prescan_preserves_named_entity_and_tier1_decodes() {
    let html = "<p>calf&eacute;</p>";
    let md = tier1_run(html).expect("Tier-1 decodes &eacute; via Latin-1 table");
    assert!(md.contains('\u{00E9}'), "expected 'é' in output, got: {md:?}");
}
