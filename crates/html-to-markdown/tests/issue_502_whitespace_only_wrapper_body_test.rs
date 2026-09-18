// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #502: an inline wrapper whose whole body is whitespace that reaches
//! it through a nested element or a `<br>` was discarded outright, and the words on either side
//! were joined -- `<b>Alpha</b><b><span>\n</span></b><b>Beta</b>` rendered `**AlphaBeta**`.
//!
//! Two drops with one shape. A newline-only text node returned early because the wrapper's
//! scratch buffer was empty, although real content already preceded it in the document; and a
//! `<br>` with nothing before it in that buffer left a bare `\n`, which `chomp_inline` did not
//! count as a space. Both now surface as the single separating space issue #481 already gives a
//! literal `<b> </b>`.
//!
//! Tier 1 bails on every one of these shapes (adjacent emphasis, or a whitespace-only body), so
//! the fix is Tier-2 only; the bail is asserted so the parity contract cannot pass vacuously.

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, HighlightStyle, convert, tier1};

const ISSUE_502_NEWLINE: &str = "<b>Alpha</b><b><span>\n</span></b><b>Beta</b>";
const ISSUE_502_BR: &str = "<b>Alpha</b><b><span><br></span></b><b>Beta</b>";

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
fn should_keep_a_separator_for_a_newline_only_span_inside_an_empty_bold_wrapper() {
    let out = content(ISSUE_502_NEWLINE);
    assert_eq!(out, "**Alpha** **Beta**\n", "actual: {out:?}");
    assert_tier1_bails(ISSUE_502_NEWLINE);
}

#[test]
fn should_keep_a_separator_for_a_br_only_span_inside_an_empty_bold_wrapper() {
    let out = content(ISSUE_502_BR);
    assert_eq!(out, "**Alpha** **Beta**\n", "actual: {out:?}");
    assert_tier1_bails(ISSUE_502_BR);
}

#[test]
fn should_keep_a_separator_for_a_br_only_bold_wrapper() {
    let out = content("<b>Alpha</b><b><br></b><b>Beta</b>");
    assert_eq!(out, "**Alpha** **Beta**\n", "actual: {out:?}");
}

#[test]
fn should_keep_a_separator_for_a_newline_only_italic_wrapper_between_words() {
    let html = "Alpha<i>\n</i>Beta";
    let out = content(html);
    assert_eq!(out, "Alpha Beta\n", "actual: {out:?}");
    assert_tier1_bails(html);
}

#[test]
fn should_keep_a_separator_for_a_newline_only_span_opening_a_bold_body() {
    let out = content("Alpha<b><span>\n</span>Text</b>");
    assert_eq!(out, "Alpha **Text**\n", "actual: {out:?}");
}

#[test]
fn should_still_emit_nothing_for_a_truly_empty_bold_wrapper() {
    let out = content("<b>Alpha</b><b></b><b>Beta</b>");
    assert_eq!(out, "**AlphaBeta**\n", "actual: {out:?}");
}

#[test]
fn should_not_double_a_separator_the_previous_sibling_already_emitted() {
    let out = content("Alpha <b><span>\n</span></b>Beta");
    assert_eq!(out, "Alpha Beta\n", "actual: {out:?}");
}

#[test]
fn should_emit_nothing_for_a_whitespace_only_wrapper_at_document_start() {
    let out = content("<b><span>\n</span></b>Beta");
    assert_eq!(out, "Beta\n", "actual: {out:?}");
}
