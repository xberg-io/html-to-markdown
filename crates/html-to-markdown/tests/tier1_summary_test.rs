//! Tier-1 `<summary>` strong-wrap tests (Phase R).
//!
//! Each test asserts that forcing Tier-1 produces byte-identical output to
//! Tier-2, exercising the buffer-redirect and strong-wrap path added in Phase R.

#![cfg(feature = "testkit")]

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

// ── Helpers ───────────────────────────────────────────────────────────────────

fn tier1(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

fn tier2(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

/// Assert Tier-1 and Tier-2 produce byte-identical output for `html`.
fn assert_matches_tier2(html: &str) {
    let t1 = tier1(html);
    let t2 = tier2(html);
    assert_eq!(
        t1, t2,
        "tier1 diverged from tier2\ninput: {html:?}\ntier1: {t1:?}\ntier2: {t2:?}"
    );
}

// ── Test cases ────────────────────────────────────────────────────────────────

/// Test 1: bare `<summary>` with simple text content.
#[test]
fn summary_bare_simple_text() {
    assert_matches_tier2("<summary>x</summary>");
}

/// Test 2: nested block children — `<div>` elements inside `<summary>`.
///
/// Block children must render inline in the accumulation buffer so the
/// trimmed content becomes a single strong-wrapped paragraph.
#[test]
fn summary_nested_block_children() {
    assert_matches_tier2("<summary><div>x</div><div>y</div></summary>");
}

/// Test 3: empty `<summary>` — Tier-2 emits nothing; Tier-1 must match.
#[test]
fn summary_empty() {
    assert_matches_tier2("<summary></summary>");
}

/// Test 4: whitespace-only content — treated as empty, no output.
#[test]
fn summary_whitespace_only() {
    assert_matches_tier2("<summary>   \n  </summary>");
}

/// Test 5: leading and trailing whitespace is trimmed before strong-wrapping.
#[test]
fn summary_leading_trailing_whitespace() {
    assert_matches_tier2("<summary>  Baseline  </summary>");
}

/// Test 6: inline children (`<span>`, `<strong>`) render into the buffer.
#[test]
fn summary_inline_children() {
    assert_matches_tier2("<summary><span>a</span> <strong>b</strong></summary>");
}

/// Test 7: pathologically nested `<summary>` (malformed HTML) — must not panic.
///
/// Tier-2 handles this via the DOM; Tier-1 uses a stack so it also recovers
/// without panicking.  The output should be byte-equal by either matching T2
/// or bailing and falling back to T2.
#[test]
fn summary_nested_malformed() {
    assert_matches_tier2("<summary><summary>x</summary></summary>");
}

/// Test 8: minimal mdn-array repro — `<details><summary>` with inline and block children.
///
/// Derived from the problematic section in the mdn-array benchmark fixture:
/// a `<div>` inside `<summary>` with text and nested `<span>` elements.
#[test]
fn summary_inside_details_with_div() {
    let html = "<details>\
        <summary>\
            <div>Baseline <span>Widely available</span> *</div>\
        </summary>\
        <div class=\"extra\">More info here.</div>\
    </details>";
    assert_matches_tier2(html);
}

/// Test 9: preceding content before `<summary>` — ensures the blank-line
/// separator before the strong-wrapped block is correct.
#[test]
fn summary_with_preceding_paragraph() {
    assert_matches_tier2("<p>before</p><details><summary>x</summary></details>");
}

/// Test 10: `<summary>` inside a table cell — transparent mode, no strong-wrapping.
///
/// In this context `cell_or_output_mut` returns the cell buffer, so
/// `open_summary` must not push an accumulation buffer and the content
/// must appear verbatim in the cell.
#[test]
fn summary_inside_table_cell_transparent() {
    let html = "<table><thead><tr><th>Header</th></tr></thead>\
        <tbody><tr><td><summary>cell content</summary></td></tr></tbody>\
    </table>";
    assert_matches_tier2(html);
}
