// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #504: a whitespace-only inline wrapper nested inside another wrapper
//! lost its separator, joining the words on either side --
//! `<h2><strong>Alpha</strong><strong><em><br></em></strong>Beta</h2>` rendered `## **Alpha**Beta`
//! in 3.14.2 where 3.14.1 kept the space.
//!
//! Issue #501 taught `emit_wrapped_inline` that a whitespace-only body at a line start contributes
//! nothing, keyed on the destination buffer being empty. The destination can also be an enclosing
//! wrapper's fresh scratch buffer, which is empty mid-line: the inner `<em>`'s one space was
//! dropped there, the outer `<strong>` body came out empty and emitted nothing. The line-start rule
//! now fires only on the block's own buffer (`Context::block_output_ptr`), the same address test
//! the text-node fallback already uses.
//!
//! A `<br>` inside a wrapper inside a table cell had the same shape independently: the cell
//! continuation rule suppressed its space against the wrapper's empty scratch buffer.
//!
//! Tier 1 bails on adjacent emphasis, so the fixes are Tier-2 only; the bail is asserted so the
//! parity contract cannot pass vacuously.

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, HighlightStyle, convert, tier1};

const ISSUE_504_HEADING: &str = "<h2><strong>Alpha</strong><strong><em><br></em></strong>Beta</h2>";
const ISSUE_504_PARAGRAPH: &str = "<p><strong>Alpha</strong><strong><em><br></em></strong>Beta</p>";
const ISSUE_504_TABLE_CELL: &str =
    "<table><tr><td><strong>Alpha</strong><strong><em><br></em></strong>Beta</td></tr></table>";

fn content(html: &str) -> String {
    convert(html, Some(ConversionOptions::default()))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

fn base_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        ..ConversionOptions::default()
    }
}

fn assert_tier1_bails(html: &str) {
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &base_options());
    assert!(result.is_err(), "Tier-1 must bail on {html:?}, got {result:?}");
}

#[test]
fn should_keep_the_separator_for_a_br_only_emphasis_inside_an_empty_strong_in_a_heading() {
    let out = content(ISSUE_504_HEADING);
    assert_eq!(out, "## **Alpha** Beta\n", "actual: {out:?}");
    assert_tier1_bails(ISSUE_504_HEADING);
}

#[test]
fn should_keep_the_separator_for_a_br_only_emphasis_inside_an_empty_strong_in_a_paragraph() {
    let out = content(ISSUE_504_PARAGRAPH);
    assert_eq!(out, "**Alpha** Beta\n", "actual: {out:?}");
    assert_tier1_bails(ISSUE_504_PARAGRAPH);
}

#[test]
fn should_keep_the_separator_for_a_br_only_emphasis_inside_an_empty_strong_in_a_table_cell() {
    let out = content(ISSUE_504_TABLE_CELL);
    assert_eq!(out, "| **Alpha** Beta |\n| -------------- |\n", "actual: {out:?}");
    assert_tier1_bails(ISSUE_504_TABLE_CELL);
}

#[test]
fn should_keep_the_separator_for_a_br_only_emphasis_directly_in_a_table_cell() {
    let html = "<table><tr><td><strong>Alpha</strong><em><br></em>Beta</td></tr></table>";
    let out = content(html);
    assert_eq!(out, "| **Alpha** Beta |\n| -------------- |\n", "actual: {out:?}");
}

#[test]
fn should_keep_the_separator_for_a_space_only_italic_inside_an_empty_strong() {
    let html = "<p><strong>Alpha</strong><strong><i> </i></strong>Beta</p>";
    let out = content(html);
    assert_eq!(out, "**Alpha** Beta\n", "actual: {out:?}");
    assert_tier1_bails(html);
}

#[test]
fn should_keep_the_separator_for_a_br_only_emphasis_inside_a_mark() {
    let html = "<p><strong>Alpha</strong><mark><em><br></em></mark>Beta</p>";
    let out = content(html);
    assert_eq!(out, "**Alpha** Beta\n", "actual: {out:?}");
}

#[test]
fn should_still_drop_a_whitespace_only_wrapper_opening_a_paragraph() {
    let out = content("<p>A</p><p><i> </i>B</p>");
    assert_eq!(out, "A\n\nB\n", "actual: {out:?}");
}

#[test]
fn should_still_drop_a_nested_whitespace_only_wrapper_opening_a_paragraph() {
    let out = content("<p>A</p><p><strong><em> </em></strong>B</p>");
    assert_eq!(out, "A\n\nB\n", "actual: {out:?}");
}

#[test]
fn should_still_emit_nothing_for_a_whitespace_only_wrapper_at_document_start() {
    let out = content("<b><span>\n</span></b>Beta");
    assert_eq!(out, "Beta\n", "actual: {out:?}");
}

#[test]
fn should_still_trim_a_whitespace_only_wrapper_opening_a_heading() {
    let out = content("<h1><em> </em>Title</h1>");
    assert_eq!(out, "# Title\n", "actual: {out:?}");
}
