// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #504, a 3.14.2 regression: a `<br>` (or other whitespace-only
//! body) inside doubly-nested inline formatting lost its separator, joining the words on
//! either side -- `<strong>Alpha</strong><strong><em><br></em></strong>Beta` rendered
//! `**Alpha**Beta` (3.14.1 kept `**Alpha** Beta`).
//!
//! The #501 fix (issue #501, `<p>A</p><p><i> </i>B</p>`) taught `emit_wrapped_inline` to treat
//! an empty `output` buffer as a genuine line start and suppress the separator there. But
//! `output` is not always the real block buffer: `<em>` nested inside `<strong>` writes into
//! `<strong>`'s own fresh, empty scratch buffer, which is indistinguishable from a true line
//! start by emptiness alone even though "Alpha" already precedes it in the document. The
//! dropped separator then left the outer `<strong>`'s own `content` empty too, so it emitted
//! nothing at all -- both the space AND the delimiters vanished.
//!
//! Tier 1 bails on nested emphasis, so the fix is Tier-2 only.

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, convert, tier1};

fn content(html: &str) -> String {
    convert(html, Some(ConversionOptions::default()))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

fn assert_tier1_bails(html: &str) {
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &ConversionOptions::default());
    assert!(result.is_err(), "Tier-1 must bail on {html:?}, got {result:?}");
}

#[test]
fn should_keep_a_separator_for_a_br_inside_doubly_nested_emphasis_in_a_heading() {
    let html = "<h2><strong>Alpha</strong><strong><em><br></em></strong>Beta</h2>";
    let out = content(html);
    assert_eq!(out, "## **Alpha** Beta\n", "actual: {out:?}");
    assert_tier1_bails(html);
}

#[test]
fn should_keep_a_separator_for_a_newline_only_span_inside_doubly_nested_emphasis() {
    let html = "<p><strong>Alpha</strong><strong><em><span>\n</span></em></strong>Beta</p>";
    let out = content(html);
    assert_eq!(out, "**Alpha** Beta\n", "actual: {out:?}");
    assert_tier1_bails(html);
}

#[test]
fn should_keep_a_separator_for_a_br_inside_a_table_cell() {
    let html = "<table><tr><td><strong>Alpha</strong><strong><em><br></em></strong>Beta</td></tr></table>";
    let out = content(html);
    assert!(
        out.contains("Alpha") && out.contains("Beta") && !out.contains("AlphaBeta"),
        "words must stay separated: {out:?}"
    );
}
