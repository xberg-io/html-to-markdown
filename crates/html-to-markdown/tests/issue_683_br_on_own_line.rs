// ~keep The inner attributes below are crate-level Rust attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #683: a `<br>` on its own source line became a paragraph
//! break. The text node before it (`"First\n"`) turned its trailing source newline into a
//! `'\n'` outside a paragraph, so the `<br>`'s hard-break marker landed on a line of its
//! own (`"First\n  \n"`). Cleanup then reduced that whitespace-only line to a blank line.
//! Under the backslash style, the `\` was left stranded on its own line instead.

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, NewlineStyle, TierStrategy, convert, tier1};

fn options(newline_style: NewlineStyle, tier_strategy: TierStrategy) -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        newline_style,
        tier_strategy,
        ..ConversionOptions::default()
    }
}

/// The full converter's output.
fn full(html: &str, newline_style: NewlineStyle) -> String {
    convert(html, Some(options(newline_style, TierStrategy::Tier2)))
        .expect("conversion must succeed")
        .content
        .unwrap_or_default()
}

/// The fast converter's own output: an `Err` is a bail, never a fallback to the full converter.
/// It writes only the spaces style (the router sends the backslash style to the full converter).
fn fast(html: &str) -> String {
    tier1::run(
        html,
        &PrescanReport::default(),
        &options(NewlineStyle::Spaces, TierStrategy::Tier1),
    )
    .unwrap_or_else(|reason| panic!("the fast converter must not bail on {html:?}: {reason:?}"))
}

/// `own_line` gives what `inline`, the same markup without the source newlines, gives, in both
/// converters.
fn assert_matches_inline_form(own_line: &str, inline: &str) {
    assert_eq!(
        full(own_line, NewlineStyle::Spaces),
        full(inline, NewlineStyle::Spaces),
        "full converter: {own_line:?} vs {inline:?}"
    );
    assert_eq!(
        fast(own_line),
        fast(inline),
        "fast converter: {own_line:?} vs {inline:?}"
    );
}

#[test]
fn should_keep_a_top_level_br_on_its_own_line_as_a_hard_break() {
    let html = "First\n<br>\nSecond";
    assert_eq!(full(html, NewlineStyle::Spaces), "First  \nSecond\n");
    assert_eq!(fast(html), "First  \nSecond\n");
}

#[test]
fn should_attach_the_backslash_marker_to_the_preceding_line() {
    for html in [
        "First\n<br>\nSecond",
        "First\n<!-- c --><br>Second",
        "<span>First\n</span><br>Second",
        "<span>First\n<!-- c --></span><br>Second",
        "<small>First\n</small><br>Second",
    ] {
        assert_eq!(full(html, NewlineStyle::Backslash), "First\\\nSecond\n", "{html:?}");
    }
    assert_eq!(
        full("<ul><li><span>a\n</span><br>b</li></ul>", NewlineStyle::Backslash),
        "- a\\\n  b\n"
    );
}

#[test]
fn should_keep_a_br_on_its_own_line_in_a_block_as_a_hard_break() {
    assert_matches_inline_form("<div>First\n<br>\nSecond</div>", "<div>First<br>Second</div>");
    assert_matches_inline_form("<ul><li>a\n<br>\nb</li></ul>", "<ul><li>a<br>b</li></ul>");
    assert_matches_inline_form("<blockquote>a\n<br>\nb</blockquote>", "<blockquote>a<br>b</blockquote>");
}

#[test]
fn should_keep_a_br_preceded_only_by_a_source_newline_as_a_hard_break() {
    assert_matches_inline_form("First\n<br>Second", "First<br>Second");
}

#[test]
fn should_look_past_a_comment_before_the_br() {
    assert_matches_inline_form("First\n<!-- c -->\n<br>\nSecond", "First<br>Second");
}

#[test]
fn should_match_the_inline_form_for_a_br_run_on_their_own_lines() {
    assert_matches_inline_form("A\n<br>\n<br>\nB", "A<br><br>B");
    assert_matches_inline_form("A\n<br>\n<br>\n<br>\nB", "A<br><br><br>B");
    assert_eq!(
        full("A\n<br>\n<br>\nB", NewlineStyle::Backslash),
        full("A<br><br>B", NewlineStyle::Backslash)
    );
}

#[test]
fn should_keep_a_br_after_text_that_ends_inside_a_span_as_a_hard_break() {
    assert_matches_inline_form("<span>First\n</span><br>Second", "<span>First</span><br>Second");
    assert_matches_inline_form(
        "<div><span>First\n</span>\n<br>\nSecond</div>",
        "<div><span>First</span><br>Second</div>",
    );
    assert_matches_inline_form(
        "<span><span>First\n</span>\n</span>\n<br>Second",
        "<span><span>First</span></span><br>Second",
    );
    assert_matches_inline_form(
        "<ul><li><span>a\n</span><br>b</li></ul>",
        "<ul><li><span>a</span><br>b</li></ul>",
    );
    assert_matches_inline_form(
        "<span>First\n<!-- c --></span >\n<br>Second",
        "<span>First</span><br>Second",
    );
}

#[test]
fn should_keep_a_br_after_text_that_ends_inside_other_elements_as_a_hard_break() {
    for tag in ["b", "i", "em", "small", "u", "del", "sub", "abbr", "cite", "time"] {
        assert_matches_inline_form(
            &format!("<{tag}>First\n</{tag}><br>Second"),
            &format!("<{tag}>First</{tag}><br>Second"),
        );
    }
    assert_matches_inline_form(
        "<a href=\"u\">First\n</a><br>Second",
        "<a href=\"u\">First</a><br>Second",
    );
    assert_matches_inline_form(
        "<b><span>First\n</span></b><br>Second",
        "<b><span>First</span></b><br>Second",
    );
    assert_matches_inline_form("<x-y>First\n</x-y><br>Second", "<x-y>First</x-y><br>Second");
    // ~keep The fast converter hands these two to the full converter.
    for tag in ["font", "mark"] {
        assert_eq!(
            full(&format!("<{tag}>First\n</{tag}><br>Second"), NewlineStyle::Spaces),
            full(&format!("<{tag}>First</{tag}><br>Second"), NewlineStyle::Spaces),
            "{tag}"
        );
    }
    assert_matches_inline_form(
        "<span><b>First\n</b></span><br>Second",
        "<span><b>First</b></span><br>Second",
    );
}

#[test]
fn should_keep_the_source_newline_when_no_br_ends_the_line() {
    for html in [
        "<span>First\n</span>Second",
        "<span>First\n</span>more<br>Second",
        "<div><span>First\n</span></div><br>Second",
        "<span>First\n</span><em>Second</em>",
        "<div><div>First\n</div><br>Second</div>",
        "<ul><li>First\n</li><br>Second</ul>",
    ] {
        assert_eq!(fast(html), full(html, NewlineStyle::Spaces), "{html:?}");
    }
    assert_eq!(
        full("<span>First\n</span>Second", NewlineStyle::Spaces),
        "First\nSecond\n"
    );
    assert_eq!(
        full("<span>First\n</span><em>Second</em>", NewlineStyle::Spaces),
        "First\n*Second*\n"
    );
}
