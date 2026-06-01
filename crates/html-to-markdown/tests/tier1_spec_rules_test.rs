//! Tests for M4: HTML5 optional-close-tag spec rules in the Tier-1 scanner.
//!
//! Inputs with explicit close tags are validated against Tier-2 output.
//! Inputs that exercise implicit-close transitions (no `</li>`, no `</p>`) are
//! validated against the expected canonical Markdown output, because Tier-2's
//! HTML parser may treat missing close tags differently.
//!
//! See: <https://html.spec.whatwg.org/multipage/syntax.html#optional-tags>

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

/// Assert that Tier-1 output byte-equals Tier-2 output.
fn assert_matches_tier2(html: &str) {
    let t1 = tier1(html);
    let t2 = tier2(html);
    assert_eq!(
        t1, t2,
        "tier1 diverged from tier2 for input {html:?}\ntier1: {t1:?}\ntier2: {t2:?}"
    );
}

// ── Implicit-close: <li> with explicit </li> ──────────────────────────────────
//
// These use fully-explicit close tags, so both Tier-1 and Tier-2 agree on output.

#[test]
fn explicit_close_still_works_li() {
    assert_matches_tier2("<ul><li>a</li><li>b</li></ul>");
}

#[test]
fn explicit_close_still_works_p() {
    assert_matches_tier2("<p>hello</p><p>world</p>");
}

#[test]
fn nested_ul_in_li_explicit() {
    assert_matches_tier2("<ul><li>outer<ul><li>inner</li></ul></li></ul>");
}

#[test]
fn nested_p_inside_blockquote() {
    assert_matches_tier2("<blockquote><p>a</p><p>b</p></blockquote>");
}

// ── Implicit-close: <li> without </li> (canonical output) ────────────────────
//
// Tier-2's HTML parser does not implicitly close <li> when it sees the next
// <li> — it treats the second <li> as a child.  Tier-1 with M4 implements the
// HTML5 spec correctly.  These tests verify the canonical Markdown output.

#[test]
fn implicit_close_li_consecutive() {
    // HTML5: <li>a<li>b<li>c</ul> → three sibling items, not nested.
    let html = "<ul><li>a<li>b<li>c</ul>";
    let output = tier1(html);
    assert_eq!(output, "- a\n- b\n- c\n", "got: {output:?}");
}

#[test]
fn implicit_close_ol_li() {
    let html = "<ol><li>first<li>second<li>third</ol>";
    let output = tier1(html);
    assert_eq!(output, "1. first\n2. second\n3. third\n", "got: {output:?}");
}

#[test]
fn three_consecutive_li_no_close() {
    let html = "<ul><li>one<li>two<li>three</ul>";
    let output = tier1(html);
    assert_eq!(output, "- one\n- two\n- three\n", "got: {output:?}");
}

#[test]
fn mixed_explicit_and_implicit_li() {
    // First item explicitly closed, second and third not.
    let html = "<ul><li>a</li><li>b<li>c</ul>";
    let output = tier1(html);
    assert_eq!(output, "- a\n- b\n- c\n", "got: {output:?}");
}

#[test]
fn mixed_implicit_then_explicit_li() {
    // First two items implicitly closed, last explicitly closed.
    let html = "<ul><li>a<li>b<li>c</li></ul>";
    let output = tier1(html);
    assert_eq!(output, "- a\n- b\n- c\n", "got: {output:?}");
}

#[test]
fn li_inside_closed_ul_implicit_li_close() {
    // Single <li> with no </li> before </ul>.
    let html = "<ul><li>only item</ul>";
    let output = tier1(html);
    assert_eq!(output, "- only item\n", "got: {output:?}");
}

// ── Implicit-close: <p> without </p> (canonical output) ──────────────────────

#[test]
fn implicit_close_p_consecutive() {
    // HTML5: <p>one<p>two<p>three → three separate paragraphs.
    let html = "<p>one<p>two<p>three";
    let output = tier1(html);
    assert_eq!(output, "one\n\ntwo\n\nthree\n", "got: {output:?}");
}

#[test]
fn p_no_close_tag_at_eof() {
    // A bare <p> with no </p> — should implicitly close at EOF.
    let html = "<p>hello";
    let output = tier1(html);
    assert_eq!(output, "hello\n", "got: {output:?}");
}

// ── Nested structures with implicit close ────────────────────────────────────

#[test]
fn nested_ul_in_li_no_explicit_close() {
    // Inner <li> not explicitly closed before </ul>.
    assert_matches_tier2("<ul><li>outer<ul><li>inner</ul></li></ul>");
}

#[test]
fn consecutive_p_inside_div() {
    // Explicit close tags on <p> — both tiers agree.
    assert_matches_tier2("<div><p>first</p><p>second</p><p>third</p></div>");
}

#[test]
fn consecutive_p_inside_div_no_close() {
    // <p> elements without close tags inside a <div>.
    let html = "<div><p>first<p>second<p>third</div>";
    let output = tier1(html);
    assert_eq!(output, "first\n\nsecond\n\nthird\n", "got: {output:?}");
}

// ── Ol with explicit start attribute ─────────────────────────────────────────

#[test]
fn ol_with_start_implicit_close() {
    let html = "<ol start=\"3\"><li>a<li>b<li>c</ol>";
    let output = tier1(html);
    assert_eq!(output, "3. a\n4. b\n5. c\n", "got: {output:?}");
}
