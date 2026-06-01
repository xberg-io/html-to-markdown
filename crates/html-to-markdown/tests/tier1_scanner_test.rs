//! Tier-1 scanner integration tests (M3c).
//!
//! Each "match" test asserts that the Tier-1 scanner produces byte-identical
//! output to the Tier-2 path for the same input.  The "bail" tests verify that
//! when the scanner encounters an unsupported construct, it falls back to
//! Tier-2 and still produces a valid result (the fallback is transparent to
//! the caller via `convert()`).
//!
//! All tests use `extract_metadata: false` and `hocr_spatial_tables: false` so
//! the Tier-1 classifier can accept inputs it would otherwise reject.

#![cfg(feature = "testkit")]

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

// ── Helpers ───────────────────────────────────────────────────────────────────

fn tier1(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::ForceTier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

fn tier2(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2Only,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

/// Assert that Tier-1 output byte-equals Tier-2 output.
fn assert_matches_tier2(html: &str) {
    let t1 = tier1(html);
    let t2 = tier2(html);
    assert_eq!(
        t1, t2,
        "tier1 diverged from tier2 for input {html:?}\ntier1: {t1:?}\ntier2: {t2:?}"
    );
}

// ── Match tests ───────────────────────────────────────────────────────────────

#[test]
fn tier1_matches_tier2_simple_paragraph() {
    assert_matches_tier2("<p>hello world</p>");
}

#[test]
fn tier1_matches_tier2_heading_h1() {
    assert_matches_tier2("<h1>Title</h1>");
}

#[test]
fn tier1_matches_tier2_heading_h2() {
    assert_matches_tier2("<h2>Subtitle</h2>");
}

#[test]
fn tier1_matches_tier2_heading_h3_through_h6() {
    assert_matches_tier2("<h3>Three</h3>");
    assert_matches_tier2("<h4>Four</h4>");
    assert_matches_tier2("<h5>Five</h5>");
    assert_matches_tier2("<h6>Six</h6>");
}

#[test]
fn tier1_matches_tier2_multiple_headings() {
    assert_matches_tier2("<h1>H1</h1><h2>H2</h2>");
}

#[test]
fn tier1_matches_tier2_strong_emphasis() {
    assert_matches_tier2("<p>This is <strong>bold</strong> and <em>italic</em></p>");
}

#[test]
fn tier1_matches_tier2_nested_emphasis() {
    assert_matches_tier2("<p><strong>bold <em>italic</em></strong></p>");
}

#[test]
fn tier1_matches_tier2_b_and_i_aliases() {
    assert_matches_tier2("<p><b>bold</b> and <i>italic</i></p>");
}

#[test]
fn tier1_matches_tier2_inline_code() {
    assert_matches_tier2("<p>Use <code>println!</code> for output</p>");
}

#[test]
fn tier1_matches_tier2_link_simple() {
    assert_matches_tier2("<a href=\"https://example.com\">click</a>");
}

#[test]
fn tier1_matches_tier2_link_with_title() {
    assert_matches_tier2("<a href=\"https://example.com\" title=\"Example\">click</a>");
}

#[test]
fn tier1_matches_tier2_image_simple() {
    assert_matches_tier2("<img src=\"foo.png\" alt=\"bar\">");
}

#[test]
fn tier1_matches_tier2_image_no_alt() {
    assert_matches_tier2("<img src=\"foo.png\">");
}

#[test]
fn tier1_matches_tier2_image_with_title() {
    assert_matches_tier2("<img src=\"foo.png\" alt=\"bar\" title=\"My Title\">");
}

#[test]
fn tier1_matches_tier2_br_in_paragraph() {
    assert_matches_tier2("<p>a<br>b</p>");
}

#[test]
fn tier1_matches_tier2_hr_alone() {
    assert_matches_tier2("<hr>");
}

#[test]
fn tier1_matches_tier2_hr_after_paragraph() {
    assert_matches_tier2("<p>a</p><hr>");
}

#[test]
fn tier1_matches_tier2_ul_simple() {
    assert_matches_tier2("<ul><li>a</li><li>b</li></ul>");
}

#[test]
fn tier1_matches_tier2_ol_simple() {
    assert_matches_tier2("<ol><li>a</li><li>b</li></ol>");
}

#[test]
fn tier1_matches_tier2_ol_with_start() {
    assert_matches_tier2("<ol start=\"3\"><li>a</li><li>b</li></ol>");
}

#[test]
fn tier1_matches_tier2_nested_ul() {
    assert_matches_tier2("<ul><li>a<ul><li>nested</li></ul></li></ul>");
}

#[test]
fn tier1_matches_tier2_blockquote_simple() {
    assert_matches_tier2("<blockquote>hello</blockquote>");
}

#[test]
fn tier1_matches_tier2_blockquote_with_paragraph() {
    assert_matches_tier2("<blockquote><p>inner</p></blockquote>");
}

#[test]
fn tier1_matches_tier2_blockquote_nested() {
    assert_matches_tier2("<blockquote><blockquote>inner</blockquote></blockquote>");
}

#[test]
fn tier1_matches_tier2_pre_code_block() {
    assert_matches_tier2("<pre><code>fn main()</code></pre>");
}

#[test]
fn tier1_matches_tier2_pre_with_stripped_newlines() {
    assert_matches_tier2("<pre>\ncode\n</pre>");
}

#[test]
fn tier1_matches_tier2_entity_decoding() {
    assert_matches_tier2("<p>5 &lt; 10 &amp; ok</p>");
}

#[test]
fn tier1_matches_tier2_multiple_paragraphs() {
    assert_matches_tier2("<p>hello</p><p>world</p>");
}

#[test]
fn tier1_matches_tier2_div_with_paragraph() {
    assert_matches_tier2("<div><p>hello</p></div>");
}

#[test]
fn tier1_matches_tier2_link_in_paragraph() {
    assert_matches_tier2("<p>before <a href=\"url\">text</a> after</p>");
}

#[test]
fn tier1_matches_tier2_image_in_paragraph() {
    assert_matches_tier2("<p>before <img src=\"x.png\"> after</p>");
}

// ── Bail / fallback tests ─────────────────────────────────────────────────────
//
// These tests verify that when Tier-1 bails, the fallback to Tier-2 still
// produces valid output.  The test just checks that `convert()` succeeds and
// the result is non-empty (or matches Tier-2 directly).

#[test]
fn tier1_bails_on_table_falls_back_to_tier2() {
    let html = "<table><tr><td>cell</td></tr></table>";
    // ForceTier1 will bail and fall back; result must equal Tier-2 output.
    assert_matches_tier2(html);
}

#[test]
fn tier1_bails_on_custom_element_falls_back_to_tier2() {
    let html = "<my-thing>content</my-thing>";
    // Tier-1 bails on custom elements; fallback must succeed.
    let t2 = tier2(html);
    // tier1() calls convert with ForceTier1; bail triggers Tier-2 fallback.
    let result = tier1(html);
    assert_eq!(result, t2, "bail fallback must equal tier-2 output");
}

#[test]
fn tier1_bails_on_cdata_falls_back_to_tier2() {
    // CDATA causes bail; svg wrapper is needed for the prescan to detect it
    // (prescan::run flags had_cdata; the router would normally force Tier-2,
    // but ForceTier1 bypasses the router so the scanner sees it directly).
    let html = "<svg><![CDATA[data]]></svg>";
    let result = convert(
        html,
        Some(ConversionOptions {
            tier_strategy: TierStrategy::ForceTier1,
            extract_metadata: false,
            ..ConversionOptions::default()
        }),
    );
    // Should not error — bail falls through to Tier-2.
    assert!(result.is_ok(), "expected Ok after bail, got: {:?}", result.err());
}

#[test]
fn tier1_bails_on_definition_list_falls_back_to_tier2() {
    let html = "<dl><dt>Term</dt><dd>Definition</dd></dl>";
    assert_matches_tier2(html);
}

#[test]
fn tier1_bails_on_textarea_falls_back_to_tier2() {
    let html = "<textarea>some text</textarea>";
    assert_matches_tier2(html);
}
