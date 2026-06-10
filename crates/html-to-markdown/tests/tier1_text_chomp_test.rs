//! Tier-1 text-node chomp (Phase Y).
//!
//! Mirrors Tier-2's `text_node.rs` chomp behaviour: text nodes with
//! leading/trailing whitespace runs containing `\n` get their boundary
//! whitespace folded into a single space (or stripped entirely for a
//! lone trailing `\n`).  Without this, multi-line paragraph source
//! HTML leaks literal `\n` runs into Tier-1's markdown output.

#![cfg(feature = "testkit")]

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

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

fn assert_matches(html: &str) {
    let t1 = tier1(html);
    let t2 = tier2(html);
    assert_eq!(
        t1, t2,
        "tier1 diverged from tier2\ninput: {html:?}\ntier1: {t1:?}\ntier2: {t2:?}"
    );
}

#[test]
fn trailing_newline_run_before_inline_tag() {
    assert_matches("<p>The number of\n  <kbd>#</kbd> you use</p>");
}

#[test]
fn trailing_newline_run_before_em() {
    assert_matches("<p>foo\n  <em>bar</em> baz</p>");
}

#[test]
fn newline_preserved_inside_em() {
    // Inside an inline frame Tier-2 preserves the literal `\n`.  Tier-1
    // must NOT apply the chomp when inside_inline is true.
    assert_matches("<p>foo<em>bar\n  baz</em></p>");
}

#[test]
fn leading_newline_run_after_em() {
    assert_matches("<p>foo <em>bar</em>\n  baz</p>");
}
