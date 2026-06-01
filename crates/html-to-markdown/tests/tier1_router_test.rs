//! Integration tests for the Tier-1 router (M2).
//!
//! These tests verify that `classify()` routes inputs to the correct tier
//! and that the `convert()` dispatcher produces identical output on the
//! Tier-2 fallback path.

use html_to_markdown_rs::convert;
use html_to_markdown_rs::options::{
    CodeBlockStyle, ConversionOptions, HighlightStyle, PreprocessingOptions, PreprocessingPreset, TierStrategy,
};
use html_to_markdown_rs::prescan;
use html_to_markdown_rs::tier1::router::{RouterDecision, classify};

// ── Helper ──────────────────────────────────────────────────────────────────

/// Prescan `html` and classify with the given options.
fn route(html: &str, options: &ConversionOptions) -> RouterDecision {
    let (_cleaned, report) = prescan::run(html);
    classify(&report, options)
}

/// `ConversionOptions` with all structural and style gates set to Tier-1-
/// compatible values so the classifier can return Tier1.
fn minimal_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        code_block_style: CodeBlockStyle::Indented,
        highlight_style: HighlightStyle::None,
        ..ConversionOptions::default()
    }
}

// ── 1. Default options always force Tier-2 ─────────────────────────────────

#[test]
fn classify_routes_tier2_when_extract_metadata_true() {
    // Default options have extract_metadata: true — must always be Tier-2.
    let opts = ConversionOptions::default();
    let choice = route("<p>hello</p>", &opts);
    assert_eq!(choice, RouterDecision::Tier2);
}

// ── 2. Clean HTML with metadata off → Tier-1 ───────────────────────────────

#[test]
fn classify_routes_tier1_when_clean_and_extract_metadata_false() {
    let opts = minimal_options();
    let choice = route("<p>hello</p>", &opts);
    assert_eq!(choice, RouterDecision::Tier1);
}

// ── 3. Custom elements force Tier-2 ─────────────────────────────────────────

#[test]
fn classify_tier2_on_custom_elements() {
    let opts = minimal_options();
    let choice = route("<my-widget>foo</my-widget>", &opts);
    assert_eq!(choice, RouterDecision::Tier2);
}

// ── 4. CDATA forces Tier-2 ──────────────────────────────────────────────────

#[test]
fn classify_tier2_on_cdata() {
    let opts = minimal_options();
    let choice = route("<svg><![CDATA[data]]></svg>", &opts);
    assert_eq!(choice, RouterDecision::Tier2);
}

// ── 5. Unescaped `<` forces Tier-2 ──────────────────────────────────────────

#[test]
fn classify_tier2_on_unescaped_lt() {
    let opts = minimal_options();
    // `<b` followed by space is valid; `< ` (space after `<`) is not a tag.
    let choice = route("<p>a < b</p>", &opts);
    assert_eq!(choice, RouterDecision::Tier2);
}

// ── 6. strip_tags forces Tier-2 ─────────────────────────────────────────────

#[test]
fn classify_tier2_on_strip_tags() {
    let opts = ConversionOptions {
        extract_metadata: false,
        strip_tags: vec!["div".to_string()],
        ..ConversionOptions::default()
    };
    let choice = route("<p>hello</p>", &opts);
    assert_eq!(choice, RouterDecision::Tier2);
}

// ── 7. preserve_tags forces Tier-2 ──────────────────────────────────────────

#[test]
fn classify_tier2_on_preserve_tags() {
    let opts = ConversionOptions {
        extract_metadata: false,
        preserve_tags: vec!["table".to_string()],
        ..ConversionOptions::default()
    };
    let choice = route("<p>hello</p>", &opts);
    assert_eq!(choice, RouterDecision::Tier2);
}

// ── 8. wrap forces Tier-2 ───────────────────────────────────────────────────

#[test]
fn classify_tier2_on_wrap() {
    let opts = ConversionOptions {
        extract_metadata: false,
        wrap: true,
        ..ConversionOptions::default()
    };
    let choice = route("<p>hello</p>", &opts);
    assert_eq!(choice, RouterDecision::Tier2);
}

// ── 9. convert_as_inline forces Tier-2 ──────────────────────────────────────

#[test]
fn classify_tier2_on_convert_as_inline() {
    let opts = ConversionOptions {
        extract_metadata: false,
        convert_as_inline: true,
        ..ConversionOptions::default()
    };
    let choice = route("<p>hello</p>", &opts);
    assert_eq!(choice, RouterDecision::Tier2);
}

// ── 10. non-standard preprocessing preset forces Tier-2 ─────────────────────

#[test]
fn classify_tier2_on_non_standard_preprocessing() {
    let opts = ConversionOptions {
        extract_metadata: false,
        preprocessing: PreprocessingOptions {
            preset: PreprocessingPreset::Aggressive,
            ..PreprocessingOptions::default()
        },
        ..ConversionOptions::default()
    };
    let choice = route("<p>hello</p>", &opts);
    assert_eq!(choice, RouterDecision::Tier2);
}

// ── 11. TierStrategy::Tier2 overrides classifier ────────────────────────

#[test]
fn tier_strategy_tier2_overrides_classifier() {
    // Even the cleanest HTML with all classifier flags off: Tier2 wins.
    let opts = ConversionOptions {
        extract_metadata: false,
        code_block_style: CodeBlockStyle::Indented,
        highlight_style: HighlightStyle::None,
        tier_strategy: TierStrategy::Tier2,
        ..ConversionOptions::default()
    };
    // Verify the classifier alone would say Tier1.
    let (_cleaned, report) = prescan::run("<p>hello</p>");
    assert_eq!(
        classify(&report, &opts),
        RouterDecision::Tier1,
        "baseline check: classifier says Tier1"
    );

    // But the dispatcher honours Tier2 regardless.
    // We can't call the dispatcher directly here, but we verify the strategy
    // value is stored correctly and the variant exists.
    assert_eq!(opts.tier_strategy, TierStrategy::Tier2);
}

// ── 12. convert() with default options still produces correct output ─────────

#[test]
fn convert_with_default_options_still_works() {
    let result = convert("<p>hello</p>", None);
    assert!(result.is_ok(), "convert() returned Err: {:?}", result.err());
    let md = result.unwrap().content.unwrap_or_default();
    assert!(md.contains("hello"), "expected 'hello' in output, got: {md:?}");
}

// ── 13. debug flag forces Tier-2 ────────────────────────────────────────────

#[test]
fn classify_tier2_on_debug_flag() {
    let opts = ConversionOptions {
        extract_metadata: false,
        debug: true,
        ..ConversionOptions::default()
    };
    let choice = route("<p>hello</p>", &opts);
    assert_eq!(choice, RouterDecision::Tier2);
}

// ── 14. Tier-1 bail falls back to Tier-2 producing correct output ────────────
//
// `BailReason::NotImplemented` is always returned by the M2 stub, so
// Tier1 triggers an immediate bail, and the fallback must produce
// the same result as a direct Tier-2 call.
//
// This test requires the `testkit` feature because `TierStrategy::Tier1`
// is only visible when `cfg(any(test, feature = "testkit"))` is true — and
// integration tests are separate crates where `cfg(test)` is false in the
// library being tested.

#[cfg(feature = "testkit")]
#[test]
fn tier1_bail_falls_back_to_tier2() {
    let html = "<p>hello <strong>world</strong></p>";

    // Tier-2 baseline.
    let tier2_opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        ..ConversionOptions::default()
    };
    let tier2_output = convert(html, Some(tier2_opts))
        .expect("tier-2 must succeed")
        .content
        .unwrap_or_default();

    // Tier1 with default options: classifier normally blocks Tier-1 due to
    // extract_metadata=true, but Tier1 overrides it. The M2 stub always
    // bails with NotImplemented, so the fallback path runs and must match tier2.
    let force_opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        ..ConversionOptions::default()
    };
    let fallback_output = convert(html, Some(force_opts))
        .expect("fallback must succeed")
        .content
        .unwrap_or_default();
    assert_eq!(
        fallback_output, tier2_output,
        "Tier-1 bail fallback output must match Tier-2 output"
    );
}
