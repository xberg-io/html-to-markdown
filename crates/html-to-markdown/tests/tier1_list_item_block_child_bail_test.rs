//! Regression tests for `BailReason::ListItemUnsupportedBlockChild`.
//!
//! Forcing `TierStrategy::Tier1` used to reveal wrong Tier-1 output for several
//! block-level elements opening while already inside a list item: the item's own
//! leading separator (blank line vs. single newline) and/or continuation indent
//! diverged from Tier-2 for `<blockquote>`, generic block containers (`<div>`),
//! `<table>`, `<dl>`, a paragraph continuing already-started text, and `<pre>` as
//! the item's own first content. Under `TierStrategy::Auto` this was masked: the
//! classifier already routes almost every real document to Tier-2 via the
//! `extract_metadata`/`highlight_style` gates, so the bug was unreachable with
//! default options -- but fully reachable for a caller who opts out of both
//! (`tier1_friendly_options` below, matching `tests/tier_parity_corpus.rs`).
//!
//! Each shape here now bails (see `BailReason::ListItemUnsupportedBlockChild`'s
//! doc comment for the full root-cause writeup) rather than emitting the wrong
//! Tier-1-native output, so forced Tier-1 always agrees with Tier-2 -- the
//! `assert_matches` calls are the regression guard. The `assert_not_bailed`
//! calls guard the shapes that were already correct and must stay on the native
//! Tier-1 fast path (no coverage regression from an over-broad bail).

#![cfg(feature = "testkit")]

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::tier1;
use html_to_markdown_rs::{ConversionOptions, HighlightStyle, TierStrategy, convert};

// ~keep Mirrors `tier_parity_corpus.rs::tier1_friendly_options` -- clears every
// ~keep `router::classify` gate so `TierStrategy::Auto` would route to Tier-1
// ~keep natively, matching the options under which this bug was reachable.
fn tier1_friendly_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        ..ConversionOptions::default()
    }
}

fn tier1(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        ..tier1_friendly_options()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

fn tier2(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        ..tier1_friendly_options()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

/// Asserts forced Tier-1 and forced Tier-2 agree byte-for-byte.
fn assert_matches(html: &str) {
    let t1 = tier1(html);
    let t2 = tier2(html);
    assert_eq!(
        t1, t2,
        "tier1 diverged from tier2\ninput: {html:?}\ntier1: {t1:?}\ntier2: {t2:?}"
    );
}

/// Asserts `tier1::run` (called directly, bypassing the production Tier-2
/// fallback) bails -- i.e. that this exact shape is covered by the new bail.
fn assert_bailed(html: &str) {
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &tier1_friendly_options());
    assert!(
        result.is_err(),
        "expected {html:?} to bail natively, but Tier-1 produced: {result:?}"
    );
}

/// Asserts `tier1::run` does NOT bail for this shape -- guards against an
/// over-broad bail condition silently swallowing already-correct coverage.
fn assert_not_bailed(html: &str) {
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &tier1_friendly_options());
    assert!(
        result.is_ok(),
        "expected {html:?} to run natively on tier-1, but it bailed: {result:?}"
    );
}

// ── `<blockquote>` inside a list item ──────────────────────────────────────────

#[test]
fn blockquote_continuing_list_item_text_bails_and_matches() {
    let html = "<ul><li>x<blockquote>q</blockquote></li></ul>";
    assert_bailed(html);
    assert_matches(html);
}

#[test]
fn blockquote_as_list_item_first_content_bails_and_matches() {
    let html = "<ul><li><blockquote>q</blockquote></li></ul>";
    assert_bailed(html);
    assert_matches(html);
}

#[test]
fn nested_blockquote_inside_list_item_bails_and_matches() {
    let html = "<ul><li>x<blockquote><blockquote>q</blockquote></blockquote></li></ul>";
    assert_bailed(html);
    assert_matches(html);
}

#[test]
fn blockquote_containing_pre_inside_list_item_bails_and_matches() {
    let html = "<ul><li>x<blockquote><pre>pre</pre></blockquote></li></ul>";
    assert_bailed(html);
    assert_matches(html);
}

#[test]
fn blockquote_before_loose_list_sibling_bails_and_matches() {
    let html = "<ul><li>x<blockquote>q</blockquote></li><li>y</li></ul>";
    assert_bailed(html);
    assert_matches(html);
}

// ── generic block container (`<div>`) inside a list item ───────────────────────

#[test]
fn div_continuing_list_item_text_bails_and_matches() {
    let html = "<ul><li>x<div>d</div></li></ul>";
    assert_bailed(html);
    assert_matches(html);
}

#[test]
fn multiple_div_siblings_continuing_list_item_text_bails_and_matches() {
    let html = "<ul><li>x<div>d1</div><div>d2</div></li></ul>";
    assert_bailed(html);
    assert_matches(html);
}

#[test]
fn div_before_loose_list_sibling_bails_and_matches() {
    let html = "<ul><li>x<div>d</div></li><li>y</li></ul>";
    assert_bailed(html);
    assert_matches(html);
}

/// `<div>` as the item's OWN first content (directly after the bare bullet, no
/// preceding text) was ALSO wrong before this bail (Tier-1 forced a blank-line
/// separator; Tier-2's `div.rs` adds none at all in that shape) -- bails too.
#[test]
fn div_as_list_item_first_content_bails_and_matches() {
    let html = "<ul><li><div>d</div></li></ul>";
    assert_bailed(html);
    assert_matches(html);
}

/// An explicitly-closed, empty `<div>` as the item's sole content is a known
/// existing shape (see `tests/list_item_unclosed_block_sibling_test.rs`) whose
/// loose-list "\n\n" marker (via `close_list_item`'s own separate heuristic,
/// unaffected by this bail) must be preserved across the Tier-2 fallback.
#[test]
fn empty_div_as_list_item_only_content_bails_and_matches() {
    let html = "<ul><li><div></div></li><li>foo</li></ul>";
    assert_bailed(html);
    assert_matches(html);
}

// ── `<table>` inside a list item ────────────────────────────────────────────────

#[test]
fn table_continuing_list_item_text_bails_and_matches() {
    let html = "<ul><li>x<table><tr><td>c</td></tr></table></li></ul>";
    assert_bailed(html);
    assert_matches(html);
}

// ── `<dl>` inside a list item ────────────────────────────────────────────────────

#[test]
fn definition_list_inside_list_item_bails_and_matches() {
    let html = "<ul><li>x<dl><dt>t</dt><dd>d</dd></dl></li></ul>";
    assert_bailed(html);
    assert_matches(html);
}

// ── `<p>` inside a list item ─────────────────────────────────────────────────────

#[test]
fn paragraph_continuing_list_item_text_bails_and_matches() {
    let html = "<ul><li>x<p>p</p></li></ul>";
    assert_bailed(html);
    assert_matches(html);
}

/// `<p>` as the item's own first content (Phase EE inline join) was already
/// correct and must stay on the native Tier-1 path.
#[test]
fn paragraph_as_list_item_first_content_does_not_bail_and_matches() {
    let html = "<ul><li><p>x</p></li></ul>";
    assert_not_bailed(html);
    assert_matches(html);
}

// ── `<pre>` inside a list item ───────────────────────────────────────────────────

#[test]
fn pre_as_list_item_first_content_bails_and_matches() {
    let html = "<ul><li><pre>pre</pre></li></ul>";
    assert_bailed(html);
    assert_matches(html);
}

/// `<pre>` continuing already-started item text was already correct (its own
/// `ensure_blank_line` is list-item-agnostic and coincides with Tier-2 there)
/// and must stay on the native Tier-1 path.
#[test]
fn pre_continuing_list_item_text_does_not_bail_and_matches() {
    let html = "<ul><li>x<pre>pre</pre></li></ul>";
    assert_not_bailed(html);
    assert_matches(html);
}

#[test]
fn multiple_pre_siblings_continuing_list_item_text_does_not_bail_and_matches() {
    let html = "<ul><li>x<pre>p1</pre><pre>p2</pre></li></ul>";
    assert_not_bailed(html);
    assert_matches(html);
}

// ── shapes that must remain unaffected (regression guards) ─────────────────────

#[test]
fn hr_inside_list_item_does_not_bail_and_matches() {
    assert_not_bailed("<ul><li><hr></li></ul>");
    assert_matches("<ul><li><hr></li></ul>");
    // ~keep After the item's text Tier 2 starts the rule at the content column (issue #583).
    assert_bailed("<ul><li>x<hr><hr></li></ul>");
}

#[test]
fn nested_unordered_list_does_not_bail_and_matches() {
    assert_not_bailed("<ul><li><ul><li>y</li></ul></li></ul>");
    assert_matches("<ul><li><ul><li>y</li></ul></li></ul>");
    // ~keep Text after the nested list starts a paragraph at the content column (issue #583).
    assert_bailed("<ul><li>x<ul><li>y</li></ul>z</li></ul>");
}
