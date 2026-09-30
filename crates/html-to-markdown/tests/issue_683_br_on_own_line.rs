// ~keep The inner attribute below is a crate-level Rust attribute, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for issue #683: a `<br>` on its own source line became a paragraph
//! break. The text node before it (`"First\n"`) turned its trailing source newline into a
//! `'\n'` outside a paragraph, so the `<br>`'s hard-break marker landed on a line of its
//! own (`"First\n  \n"`). Cleanup then reduced that whitespace-only line to a blank line.
//! Under the backslash style, the `\` was left stranded on its own line instead.

use html_to_markdown_rs::{ConversionOptions, NewlineStyle};

fn convert(html: &str, options: ConversionOptions) -> String {
    html_to_markdown_rs::convert(html, Some(options))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

// ~keep Each case is checked on the default route and with `debug: true`, the router's
// ~keep Tier-2-forcing gate, so the fix holds whichever tier the markup is routed to.
fn assert_both_tiers(html: &str, newline_style: NewlineStyle, expected: &str) {
    for debug in [false, true] {
        let options = ConversionOptions {
            newline_style,
            debug,
            ..Default::default()
        };
        assert_eq!(convert(html, options), expected, "html={html:?} debug={debug}");
    }
}

#[test]
fn should_keep_a_top_level_br_on_its_own_line_as_a_hard_break() {
    assert_both_tiers("First\n<br>\nSecond", NewlineStyle::Spaces, "First  \nSecond\n");
}

#[test]
fn should_keep_a_br_on_its_own_line_in_a_div_as_a_hard_break() {
    assert_both_tiers(
        "<div>First\n<br>\nSecond</div>",
        NewlineStyle::Spaces,
        "First  \nSecond\n",
    );
}

#[test]
fn should_keep_a_br_preceded_only_by_a_source_newline_as_a_hard_break() {
    assert_both_tiers("First\n<br>Second", NewlineStyle::Spaces, "First  \nSecond\n");
}

#[test]
fn should_attach_the_backslash_marker_to_the_preceding_line() {
    assert_both_tiers("First\n<br>\nSecond", NewlineStyle::Backslash, "First\\\nSecond\n");
}

#[test]
fn should_match_the_inline_form_for_a_br_run_on_their_own_lines() {
    assert_both_tiers("A\n<br>\n<br>\nB", NewlineStyle::Spaces, "A  \n\nB\n");
    assert_both_tiers("A\n<br>\n<br>\nB", NewlineStyle::Backslash, "A\\\n\\\nB\n");
}

#[test]
fn should_still_open_a_line_for_a_leading_top_level_br() {
    // ~keep Pinned by `integration_test.rs::test_breaks_and_newlines_issue_112`: the
    // ~keep fix only affects text *followed* by a <br>, not a <br> with nothing before it.
    assert_both_tiers("<br>\n1\n2\n<b>3</b>", NewlineStyle::Spaces, "\n1\n2\n**3**\n");
}
