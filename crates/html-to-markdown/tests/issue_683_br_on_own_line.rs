// ~keep The inner attributes below are crate-level Rust attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #683: a `<br>` on its own source line became a paragraph
//! break. The text node before it (`"First\n"`) turned its trailing source newline into a
//! `'\n'` outside a paragraph, so the `<br>`'s hard-break marker landed on a line of its
//! own (`"First\n  \n"`). Cleanup then reduced that whitespace-only line to a blank line.
//! Under the backslash style, the `\` was left stranded on its own line instead.

use std::time::{Duration, Instant};

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
        "First\n<b></b><br>Second",
        "<span>First\n</span><i></i><br>Second",
        "<label>First\n</label><br>Second",
        "<select><option>First\n</option></select><br>Second",
        "<span>First\n</span><span><br>Second</span>",
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

#[test]
fn should_give_the_same_break_in_both_converters_after_a_form_element() {
    for tag in ["progress", "meter", "output"] {
        let html = format!("<{tag}>First\n</{tag}><br>Second");
        assert_eq!(fast(&html), full(&html, NewlineStyle::Spaces), "{html:?}");
    }
}

#[test]
fn should_keep_what_a_closing_tag_writes_after_the_newline_before_a_br() {
    let html = "<abbr title=\"t\">First\n</abbr><br>Second";
    assert!(full(html, NewlineStyle::Spaces).contains("(t)"));
    let fast_output = fast(html);
    assert!(fast_output.contains("(t)"), "{fast_output:?}");
}

#[test]
fn should_keep_a_blank_source_line_before_a_br_as_a_paragraph_break() {
    let html = "<blockquote>a\n\n<br>b</blockquote>";
    assert!(full(html, NewlineStyle::Spaces).starts_with("> a\n>\n"));
    let fast_output = fast(html);
    assert!(fast_output.starts_with("> a\n>\n"), "{fast_output:?}");
}

#[test]
fn should_keep_the_hard_break_past_markup_the_full_converter_does_not_see() {
    for html in [
        "a\n<script>var x;</script><br>y",
        "a\n<style>p { color: red }</style><br>y",
        "<span>a\n</span foo><br>y",
        "<span>a\n</span><script>x</script>\n<br>y",
    ] {
        assert_eq!(full(html, NewlineStyle::Spaces), "a  \ny\n", "full converter: {html:?}");
        assert_eq!(fast(html), "a  \ny\n", "fast converter: {html:?}");
    }
}

/// ~keep Linear code doubles its time when the input doubles, quadratic code quadruples it, and
/// ~keep 3.0 separates the two with room for noise (the instrument of `bare_lt_complexity.rs`).
const MAX_DOUBLING_RATIO: f64 = 3.0;

/// ~keep A failure must reproduce on every independent attempt; one that does not is noise.
const MAX_MEASUREMENT_ATTEMPTS: usize = 3;

/// ~keep Large enough that a quadratic scan of the markup between the text and the `<br>` is
/// ~keep not lost in noise, small enough that it still finishes in seconds in a debug build.
const BASE_SIZE: usize = 2_000;

/// Text ending in a source newline, `n` comments each followed by a form feed and a newline,
/// then a `<br>`: markup between the text and the `<br>` that the fast converter must not scan
/// again for each text node in it.
fn form_feed_runs(n: usize) -> [(&'static str, String); 2] {
    let run = "<!--c-->\x0C\n".repeat(n);
    [
        ("top level", format!("a\n{run}<br>b")),
        ("in a span", format!("<span>a\n{run}</span><br>b")),
    ]
}

fn fastest_fast_conversion(html: &str) -> Duration {
    (0..3)
        .map(|_| {
            let start = Instant::now();
            let output = fast(html);
            let elapsed = start.elapsed();
            assert!(output.ends_with("b\n"), "{output:?}");
            elapsed
        })
        .min()
        .expect("at least one run")
}

/// The two doubling ratios of one shape, or a description of why they look quadratic.
fn measure_form_feed_run(index: usize) -> Result<(), String> {
    let seconds: Vec<f64> = [BASE_SIZE, BASE_SIZE * 2, BASE_SIZE * 4]
        .into_iter()
        .map(|n| {
            fastest_fast_conversion(&form_feed_runs(n)[index].1)
                .as_secs_f64()
                .max(1e-6)
        })
        .collect();
    let first = seconds[1] / seconds[0];
    let second = seconds[2] / seconds[1];
    if first < MAX_DOUBLING_RATIO && second < MAX_DOUBLING_RATIO {
        return Ok(());
    }
    let name = form_feed_runs(1)[index].0;
    Err(format!(
        "{name}: doublings took {first:.1}x and {second:.1}x as long ({:.4}s, {:.4}s, {:.4}s); \
         expected under {MAX_DOUBLING_RATIO}x each",
        seconds[0], seconds[1], seconds[2]
    ))
}

#[test]
fn should_scan_the_markup_before_a_br_in_linear_time() {
    for index in 0..form_feed_runs(1).len() {
        let mut failures = Vec::with_capacity(MAX_MEASUREMENT_ATTEMPTS);
        for attempt in 1..=MAX_MEASUREMENT_ATTEMPTS {
            match measure_form_feed_run(index) {
                Ok(()) => break,
                Err(reason) => failures.push(format!("attempt {attempt}: {reason}")),
            }
        }
        assert!(
            failures.len() < MAX_MEASUREMENT_ATTEMPTS,
            "the fast converter scaled super-linearly on every attempt:\n{}",
            failures.join("\n")
        );
    }
}
