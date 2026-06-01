//! Comprehensive tests for Tier-1 bail tripwires (M6).
//!
//! Verifies that:
//!   (a) Each tripwire fires with the correct `BailReason` variant.
//!   (b) `convert()` with `Auto` or `ForceTier1` transparently falls back to
//!       Tier-2 and produces output byte-identical to the `Tier2Only` path.

#![cfg(feature = "testkit")]

use html_to_markdown_rs::prescan::{self, PrescanReport};
use html_to_markdown_rs::tier1::{self, BailReason};
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Prescan `html` then run the Tier-1 scanner directly.  Returns the
/// `BailReason` or the successful Markdown string.
fn tier1_run(html: &str) -> Result<String, BailReason> {
    let (cleaned, report) = prescan::run(html);
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::ForceTier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    tier1::run(cleaned.as_ref(), &report, &opts)
}

/// Run `tier1::run` WITHOUT prescanning so constructs like CDATA survive to the
/// scanner.  Useful for testing tripwires that would be pre-sanitized otherwise.
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

/// `convert()` via the Auto path (classifier decides; falls back on bail).
fn auto(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

/// `convert()` with ForceTier1 — bails silently and falls back to Tier-2.
fn force_tier1(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::ForceTier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

// ── Tripwire 1: LiteralLt ─────────────────────────────────────────────────────

#[test]
fn bails_on_literal_lt_in_text() {
    // `<` not followed by a tag-name start byte → LiteralLt.
    // The prescan escapes this to `&lt;` so we must use tier1_raw.
    let html = "<p>a < b</p>";
    assert!(matches!(tier1_raw(html).unwrap_err(), BailReason::LiteralLt { .. }));
    // Fallback via convert still produces correct output.
    assert_eq!(force_tier1(html), tier2(html));
}

#[test]
fn bails_on_literal_lt_numeric() {
    // Another common case: `3 < 5`
    let html = "<p>3 < 5</p>";
    assert!(matches!(tier1_raw(html).unwrap_err(), BailReason::LiteralLt { .. }));
    assert_eq!(force_tier1(html), tier2(html));
}

#[test]
fn literal_lt_offset_is_correct() {
    // The `<` that triggers the bail appears at known offset.
    // `<p>` = 3 bytes, `a ` = 2 bytes → offset 5.
    let html = "<p>a < b</p>";
    if let Err(BailReason::LiteralLt { offset }) = tier1_raw(html) {
        assert_eq!(offset, 5, "expected literal '<' at byte 5");
    } else {
        panic!("expected LiteralLt");
    }
}

// ── Tripwire 2: Cdata ─────────────────────────────────────────────────────────

#[test]
fn scanner_bails_on_cdata_direct() {
    // Call tier1::run without prescanning so `<![CDATA[` survives to the scanner.
    let html = "<p><![CDATA[data]]></p>";
    let err = tier1_raw(html).unwrap_err();
    assert!(matches!(err, BailReason::Cdata { .. }), "expected Cdata, got {:?}", err);
}

#[test]
fn bails_on_cdata_or_classifier_via_prescan() {
    // After prescanning, CDATA is escaped to `&lt;![CDATA[...]` so the scanner
    // won't see it.  SVG is not in the Tier-1 tag table, so the scanner bails
    // on UnknownCustomElement or Classifier depending on tag lookup semantics.
    // Any bail is acceptable here — the key assertion is transparent fallback.
    let html = "<svg><![CDATA[xxx]]></svg><p>x</p>";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(
            err,
            BailReason::Cdata { .. } | BailReason::Classifier | BailReason::UnknownCustomElement { .. }
        ),
        "expected a bail reason, got {:?}",
        err
    );
    assert_eq!(force_tier1(html), tier2(html));
}

// ── Tripwire 3: UnknownCustomElement ─────────────────────────────────────────

#[test]
fn bails_on_custom_element() {
    let html = "<my-thing>x</my-thing>";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::UnknownCustomElement { .. }),
        "expected UnknownCustomElement, got {:?}",
        err
    );
    assert_eq!(force_tier1(html), tier2(html));
}

#[test]
fn bails_on_custom_element_contains_name() {
    let html = "<data-widget>content</data-widget>";
    if let Err(BailReason::UnknownCustomElement { name, .. }) = tier1_run(html) {
        assert!(
            name.contains('-') || name.eq_ignore_ascii_case("data-widget"),
            "expected name to reflect the element, got {:?}",
            name
        );
    } else {
        panic!("expected UnknownCustomElement");
    }
}

#[test]
fn bails_on_custom_element_fallback_matches_tier2() {
    for html in &[
        "<x-button>click</x-button>",
        "<ui-card><p>hi</p></ui-card>",
        "<foo-bar baz=\"1\">text</foo-bar>",
    ] {
        assert_eq!(force_tier1(html), tier2(html), "fallback mismatch for {:?}", html);
    }
}

// ── Tripwire 4: EofWithOpenBlock ─────────────────────────────────────────────

#[test]
fn bails_on_eof_with_open_block() {
    let html = "<p>hello <strong>world";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::EofWithOpenBlock { .. }),
        "expected EofWithOpenBlock, got {:?}",
        err
    );
    assert_eq!(force_tier1(html), tier2(html));
}

#[test]
fn eof_open_count_reflects_stack_depth() {
    // Two unclosed block elements: <p> and <strong>
    let html = "<p>hello <strong>world";
    if let Err(BailReason::EofWithOpenBlock { open_count }) = tier1_run(html) {
        assert_eq!(open_count, 2, "expected 2 unclosed elements, got {}", open_count);
    } else {
        panic!("expected EofWithOpenBlock");
    }
}

#[test]
fn bails_on_eof_single_unclosed() {
    let html = "<div>some text without closing tag";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::EofWithOpenBlock { open_count: 1 }),
        "expected EofWithOpenBlock{{open_count:1}}, got {:?}",
        err
    );
    assert_eq!(force_tier1(html), tier2(html));
}

#[test]
fn bails_on_eof_nested_open() {
    // Three levels open: ul > li > strong
    let html = "<ul><li>item <strong>bold";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::EofWithOpenBlock { .. }),
        "expected EofWithOpenBlock, got {:?}",
        err
    );
    assert_eq!(force_tier1(html), tier2(html));
}

// ── Tripwire 5: DepthMismatch ─────────────────────────────────────────────────

#[test]
fn bails_on_depth_mismatch_close_without_open() {
    // Close tag with nothing open → DepthMismatch.
    let html = "</p>orphan close";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::DepthMismatch { .. }),
        "expected DepthMismatch, got {:?}",
        err
    );
    assert_eq!(force_tier1(html), tier2(html));
}

#[test]
fn bails_on_depth_mismatch_wrong_close() {
    // Open <p> but close </div> — stack mismatch.
    let html = "<p>text</div>";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::DepthMismatch { .. }),
        "expected DepthMismatch, got {:?}",
        err
    );
    assert_eq!(force_tier1(html), tier2(html));
}

#[test]
fn depth_mismatch_contains_tag_name() {
    let html = "<p>text</div>";
    if let Err(BailReason::DepthMismatch { tag, .. }) = tier1_run(html) {
        assert_eq!(tag, "div", "expected tag name 'div', got {:?}", tag);
    } else {
        panic!("expected DepthMismatch");
    }
}

// ── Classifier tripwire (Unsupported tag kinds) ───────────────────────────────

#[test]
fn table_now_handled_by_tier1() {
    // M9: simple tables are now handled by Tier-1 (no longer bail).
    // Tier-1 output must be byte-identical to Tier-2.
    let html = "<table><tr><td>a</td></tr></table>";
    let result = tier1_run(html);
    assert!(result.is_ok(), "expected success, got {:?}", result);
    assert_eq!(force_tier1(html), tier2(html));
}

#[test]
fn bails_on_definition_list() {
    let html = "<dl><dt>key</dt><dd>value</dd></dl>";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::Classifier),
        "expected Classifier, got {:?}",
        err
    );
    assert_eq!(force_tier1(html), tier2(html));
}

#[test]
fn bails_on_script() {
    let html = "<script>var x = 1;</script><p>ok</p>";
    // Note: prescan strips script content; the scanner sees <script></script>.
    // script is RawText(Script) → Classifier.
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::Classifier),
        "expected Classifier, got {:?}",
        err
    );
    assert_eq!(force_tier1(html), tier2(html));
}

#[test]
fn bails_on_textarea() {
    let html = "<textarea>some text</textarea>";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::Classifier),
        "expected Classifier, got {:?}",
        err
    );
    assert_eq!(force_tier1(html), tier2(html));
}

// ── Auto routing tests ────────────────────────────────────────────────────────

#[test]
fn auto_routes_through_tier1_for_clean_paragraph() {
    // Clean paragraph with no complex constructs → Auto picks Tier-1 when
    // extract_metadata=false and hocr_spatial_tables=false.
    // Both tiers must produce identical output.
    let html = "<p>Hello <strong>world</strong></p>";
    assert_eq!(auto(html), tier2(html));
}

#[test]
fn auto_routes_correctly_for_complex_input() {
    // M9: tables are handled by Tier-1. Auto, ForceTier1, and Tier2Only
    // must all produce byte-identical output for a simple table.
    let html = "<table><tr><td>cell</td></tr></table>";
    assert_eq!(auto(html), tier2(html));
    assert_eq!(force_tier1(html), tier2(html));
}

// ── BailReason Display ────────────────────────────────────────────────────────

#[test]
fn bail_reason_display_is_non_empty() {
    let reasons: Vec<BailReason> = vec![
        BailReason::NotImplemented,
        BailReason::Classifier,
        BailReason::DepthMismatch {
            tag: "div".into(),
            expected: 1,
            actual: 0,
        },
        BailReason::EofWithOpenBlock { open_count: 3 },
        BailReason::LiteralLt { offset: 42 },
        BailReason::Cdata { offset: 10 },
        BailReason::UnknownCustomElement {
            name: "x-button".into(),
            offset: 0,
        },
    ];
    for reason in &reasons {
        let s = reason.to_string();
        assert!(!s.is_empty(), "Display for {:?} produced empty string", reason);
    }
}

#[test]
fn bail_reason_display_contains_contextual_info() {
    let r = BailReason::DepthMismatch {
        tag: "section".into(),
        expected: 1,
        actual: 0,
    };
    let s = r.to_string();
    assert!(s.contains("section"), "Display should contain tag name: {}", s);

    let r2 = BailReason::LiteralLt { offset: 99 };
    let s2 = r2.to_string();
    assert!(s2.contains("99"), "Display should contain offset: {}", s2);

    let r3 = BailReason::EofWithOpenBlock { open_count: 5 };
    let s3 = r3.to_string();
    assert!(s3.contains("5"), "Display should contain open_count: {}", s3);
}
