#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #496: a link label or image `alt` that spans several lines can
//! have one of those lines read as the start of a *block*, which destroys the construct
//! outright rather than merely restyling it.
//!
//! `CommonMark` parses block structure before inline structure (spec 0.31.2 appendix A), so
//! the `[` / `![` never reaches its `]`: `![A\n-\n](S)` renders as `<h2>![A</h2><p>](S)</p>`
//! -- an `<h2>` and some stray text, with the image gone. A hard line break does not protect
//! the following line either, because hard breaks are inline and block parsing has already
//! finished by the time they are considered.
//!
//! Multi-line `alt` text is not exotic: `TeX4ht` emits it for every formula it cannot render.
//! Multi-line link labels arrive via `<br>`.
//!
//! Every expectation below was checked against comrak (the crate's own reference renderer in
//! `commonmark_spec_fixpoint.rs`): the unescaped form loses the image or link, the escaped
//! form round-trips to the original element.

use html_to_markdown_rs::prescan;
use html_to_markdown_rs::tier1::{self, BailReason};
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn content(html: &str) -> String {
    convert(html, Some(ConversionOptions::default()))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

fn tier1_run(html: &str) -> Result<String, BailReason> {
    let (cleaned, report) = prescan::run(html);
    let options = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        ..ConversionOptions::default()
    };
    tier1::run(cleaned.as_ref(), &report, &options)
}

/// Both tiers must agree byte-for-byte. #496 was a shared defect -- the escaping lives in the
/// `escape_link_label` helper both tiers call -- so parity held before the fix and must hold
/// after it.
fn assert_tiers_agree(html: &str, expected: &str) {
    assert_eq!(content(html), expected, "tier-2 output for {html:?}");
    match tier1_run(html) {
        Ok(tier1) => assert_eq!(tier1, expected, "tier-1 output for {html:?} must match tier-2"),
        Err(reason) => panic!("tier-1 must not bail on {html:?}: {reason:?}"),
    }
}

/// An `<img>` whose `alt` is `A`, then `marker` on its own line, then `Z`.
fn image_with_alt_line(marker: &str) -> String {
    format!("<img alt=\"A\n{marker}\nZ\" src=\"S\">")
}

// ~keep ── The reporter's own two cases ───────────────────────────────────────────────

#[test]
fn should_escape_a_setext_underline_in_multiline_image_alt_text() {
    assert_tiers_agree("<img alt=\"A\n-\n\" src=\"S\">", "![A\n\\-\n](S)\n");
}

#[test]
fn should_escape_a_setext_underline_between_hard_breaks_in_a_link_label() {
    assert_tiers_agree(r#"<a href="H">A<br>-<br>B</a>"#, "[A  \n\\-  \nB](H)\n");
}

// ~keep ── Every block opener the reporter listed, plus the ones they did not ─────────

#[test]
fn should_escape_a_setext_h1_underline() {
    assert_tiers_agree(&image_with_alt_line("="), "![A\n\\=\nZ](S)\n");
}

#[test]
fn should_escape_every_thematic_break_character() {
    assert_tiers_agree(&image_with_alt_line("---"), "![A\n\\---\nZ](S)\n");
    assert_tiers_agree(&image_with_alt_line("***"), "![A\n\\***\nZ](S)\n");
    assert_tiers_agree(&image_with_alt_line("___"), "![A\n\\___\nZ](S)\n");
}

#[test]
fn should_escape_an_atx_heading() {
    assert_tiers_agree(&image_with_alt_line("# h"), "![A\n\\# h\nZ](S)\n");
    assert_tiers_agree(&image_with_alt_line("###### h"), "![A\n\\###### h\nZ](S)\n");
}

#[test]
fn should_escape_a_code_fence_of_either_character() {
    assert_tiers_agree(&image_with_alt_line("```"), "![A\n\\```\nZ](S)\n");
    assert_tiers_agree(&image_with_alt_line("~~~"), "![A\n\\~~~\nZ](S)\n");
}

#[test]
fn should_escape_a_block_quote_marker() {
    assert_tiers_agree(&image_with_alt_line("> q"), "![A\n\\> q\nZ](S)\n");
}

#[test]
fn should_escape_every_bullet_list_marker() {
    assert_tiers_agree(&image_with_alt_line("- i"), "![A\n\\- i\nZ](S)\n");
    assert_tiers_agree(&image_with_alt_line("+ i"), "![A\n\\+ i\nZ](S)\n");
    assert_tiers_agree(&image_with_alt_line("* i"), "![A\n\\* i\nZ](S)\n");
}

// ~keep A digit cannot carry a backslash escape, so the `.`/`)` delimiter is escaped instead
// ~keep -- `1\. i` is the spec's own way of writing a line that starts with "1." as text.
#[test]
fn should_escape_the_delimiter_of_an_ordered_list_marker() {
    assert_tiers_agree(&image_with_alt_line("1. i"), "![A\n1\\. i\nZ](S)\n");
    assert_tiers_agree(&image_with_alt_line("1) i"), "![A\n1\\) i\nZ](S)\n");
}

#[test]
fn should_escape_an_html_block_opener() {
    assert_tiers_agree(&image_with_alt_line("<div>"), "![A\n\\<div>\nZ](S)\n");
    assert_tiers_agree(&image_with_alt_line("</div>"), "![A\n\\</div>\nZ](S)\n");
    assert_tiers_agree(&image_with_alt_line("<!-- c -->"), "![A\n\\<!-- c -->\nZ](S)\n");
    assert_tiers_agree(&image_with_alt_line("<pre>"), "![A\n\\<pre>\nZ](S)\n");
}

#[test]
fn should_escape_a_block_opener_behind_up_to_three_columns_of_indent() {
    assert_tiers_agree(&image_with_alt_line("   - i"), "![A\n   \\- i\nZ](S)\n");
}

// ~keep ── CONTROL cases: lines that open no block must come through untouched ─────────────

// ~keep Four columns of indent is an indented code block, and an indented code block cannot
// ~keep interrupt a paragraph -- the line is already inert and escaping it would be noise.
#[test]
fn should_not_escape_a_line_indented_into_code_range() {
    assert_tiers_agree(&image_with_alt_line("    code"), "![A\n    code\nZ](S)\n");
    assert_tiers_agree(&image_with_alt_line("\tcode"), "![A\n\tcode\nZ](S)\n");
}

// ~keep An HTML block of type 7 -- any complete open tag alone on its line whose name is not
// ~keep one of the known block names -- deliberately cannot interrupt a paragraph, so inline
// ~keep markup is left alone.
#[test]
fn should_not_escape_inline_html_on_a_continuation_line() {
    assert_tiers_agree(&image_with_alt_line("<span>x</span>"), "![A\n<span>x</span>\nZ](S)\n");
}

#[test]
fn should_not_escape_a_marker_character_that_opens_no_block() {
    assert_tiers_agree(&image_with_alt_line("-x"), "![A\n-x\nZ](S)\n");
    assert_tiers_agree(&image_with_alt_line("2. i"), "![A\n2. i\nZ](S)\n");
    assert_tiers_agree(&image_with_alt_line("####### h"), "![A\n####### h\nZ](S)\n");
    assert_tiers_agree(&image_with_alt_line("`` x"), "![A\n`` x\nZ](S)\n");
}

// ~keep The FIRST line of a label is preceded on that same line by the caller's `[` / `![`,
// ~keep so it cannot start a block however it begins -- escaping it would change output for
// ~keep every single-line alt in existence.
#[test]
fn should_not_escape_a_block_marker_on_the_labels_first_line() {
    assert_tiers_agree("<img alt=\"- A\nZ\" src=\"S\">", "![- A\nZ](S)\n");
    assert_tiers_agree(r##"<img alt="# A" src="S">"##, "![# A](S)\n");
}

#[test]
fn should_leave_a_single_line_label_untouched() {
    assert_tiers_agree(r#"<a href="H">plain</a>"#, "[plain](H)\n");
    assert_tiers_agree(r#"<img alt="plain" src="S">"#, "![plain](S)\n");
}

// ~keep A heading and a table cell are both single-line, so `<br>` folds to a space before the
// ~keep label is assembled: there is no continuation line left to escape, and this pins that
// ~keep the new escaping does not invent one.
// ~keep
// ~keep Tier-1 used to fold late, in `close_heading`/`close_table_cell`, which runs AFTER
// ~keep `close_link` escapes the label -- so without folding in `close_link` the marker
// ~keep reached `escape_link_label` as a real line break and picked up an escape Tier-2 never
// ~keep applies. Folding there also fixes a pre-existing Tier-1 cell divergence in passing:
// ~keep the late fold turned each `"  \n"` into THREE spaces, so Tier-1 emitted
// ~keep `| [A   -   B](H) |` where Tier-2 emitted `| [A - B](H) |`.
#[test]
fn should_fold_a_multiline_label_in_a_heading_or_cell_rather_than_escape_it() {
    assert_tiers_agree(r#"<h1><a href="H">A<br>-<br>B</a></h1>"#, "# [A - B](H)\n");
    assert_tiers_agree(
        r#"<table><tr><td><a href="H">A<br>-<br>B</a></td></tr></table>"#,
        "| [A - B](H) |\n| ---------- |\n",
    );
}
