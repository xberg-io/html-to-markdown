//! Regression coverage for a Tier-1/Tier-2 divergence: `<blockquote>` never emitted a
//! trailing `"\n\n"` block separator after its own closing `"> "`-prefixed lines.
//!
//! Tier-2's `handle_blockquote` (`src/converter/handlers/blockquote.rs`) unconditionally
//! trims every trailing newline and re-pushes exactly `"\n\n"` at the very end of every
//! call -- top-level or nested -- whenever `!ctx.convert_as_inline && !ctx.in_table_cell
//! && !ctx.in_list_item`. Every existing Tier-1 blockquote test exercised only "nothing
//! follows the blockquote", where a missing trailing separator is invisible (the shared
//! end-of-document trim already collapses the tail to one newline either way). This
//! surfaces the moment real content follows the `</blockquote>` -- including nested
//! inside `<pre>`, the shape originally reported. See `close_blockquote` in
//! `src/converter/tier1/scanner.rs` for the fix, and the paired fix in `close_pre`'s
//! Backticks branch (a fenced code block's closing fence must strip ALL trailing
//! newlines, not just one, once a nested block element can leave more than one).

#![cfg(feature = "testkit")]

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::tier1;
use html_to_markdown_rs::{ConversionOptions, HighlightStyle, TierStrategy, convert};

fn tier1_friendly_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        ..ConversionOptions::default()
    }
}

fn run_tier1(html: &str) -> String {
    let report = PrescanReport::default();
    match tier1::run(html, &report, &tier1_friendly_options()) {
        Ok(markdown) => markdown,
        Err(reason) => panic!("tier1 bailed on {html:?}: {reason:?}"),
    }
}

fn run_tier2(html: &str) -> String {
    let mut options = tier1_friendly_options();
    options.tier_strategy = TierStrategy::Tier2;
    convert(html, Some(options)).unwrap().content.unwrap_or_default()
}

fn assert_tier1_matches_tier2(html: &str) {
    let t1 = run_tier1(html);
    let t2 = run_tier2(html);
    assert_eq!(
        t1, t2,
        "tier1 diverged from tier2\ninput: {html:?}\ntier1: {t1:?}\ntier2: {t2:?}"
    );
}

#[test]
fn top_level_blockquote_followed_by_text_gets_blank_line() {
    let html = "<blockquote>foo</blockquote>after";
    assert_eq!(
        run_tier2(html),
        "> foo\n\nafter\n",
        "tier2 ground truth changed; update this test"
    );
    assert_eq!(run_tier1(html), "> foo\n\nafter\n");
}

#[test]
fn blockquote_after_paragraph_and_before_text_gets_blank_line() {
    assert_tier1_matches_tier2("<p>x</p><blockquote>foo</blockquote>after");
}

#[test]
fn nested_blockquote_followed_by_sibling_text_gets_a_bare_quote_line() {
    let html = "<blockquote><blockquote>a</blockquote>b</blockquote>";
    assert_eq!(
        run_tier2(html),
        "> > a\n>\n> b\n",
        "tier2 ground truth changed; update this test"
    );
    assert_eq!(run_tier1(html), "> > a\n>\n> b\n");
}

#[test]
fn lone_blockquote_still_ends_with_a_single_trailing_newline() {
    // ~keep End-of-document normalization must still collapse the new trailing "\n\n" back
    // ~keep down to one newline when nothing follows.
    assert_eq!(run_tier2("<blockquote>hello</blockquote>"), "> hello\n");
    assert_tier1_matches_tier2("<blockquote>hello</blockquote>");
}

#[test]
fn blockquote_as_only_child_of_pre_has_no_blank_line_before_closing_fence() {
    let html = "<pre><blockquote>foo</blockquote></pre>";
    assert_eq!(
        run_tier2(html),
        "```\n> foo\n```\n",
        "tier2 ground truth changed; update this test"
    );
    assert_eq!(run_tier1(html), "```\n> foo\n```\n");
}

#[test]
fn blockquote_followed_by_text_inside_pre_gets_blank_line() {
    let html = "<pre><blockquote>foo</blockquote>after</pre>";
    assert_eq!(
        run_tier2(html),
        "```\n> foo\n\nafter\n```\n",
        "tier2 ground truth changed; update this test"
    );
    assert_eq!(run_tier1(html), "```\n> foo\n\nafter\n```\n");
}

#[test]
fn text_before_blockquote_inside_pre_still_matches() {
    assert_tier1_matches_tier2("<pre>before<blockquote>foo</blockquote></pre>");
}

#[test]
fn multiline_blockquote_inside_pre_with_text_on_both_sides_matches() {
    assert_tier1_matches_tier2("<pre>a\n<blockquote>foo\nbar</blockquote>\nb</pre>");
}
