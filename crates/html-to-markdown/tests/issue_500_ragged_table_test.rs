// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #500, two compounding defects around a headerless table whose
//! rows have differing cell counts ("ragged" rows):
//!
//! 1. `looks_like_layout` (`converter/block/table/builder.rs`) used to treat ragged row
//!    lengths alone as a layout-table signal (`distinct_counts.len() > 1`), so
//!    `<table><tr><td>A</td><td>B</td></tr><tr><td>C</td></tr></table>` rendered as a bullet
//!    list (`- A B\n- C\n`) instead of the padded pipe table GFM tables already support
//!    (issue #13). A headerless table with ragged rows is ordinary tabular data far more
//!    often than it is an email-signature-style layout table, so that disjunct is dropped;
//!    layout still triggers on nested tables, colspan/rowspan combined with `border="0"`, a
//!    blank table, or a short table dense with links.
//!
//! 2. `append_layout_cell_text` only kept an inline image as markdown in a layout row when
//!    `keep_inline_images_in` named the cell tag (issue #433), degrading it to alt text
//!    otherwise. A layout row renders as a list item, and list items keep inline images by
//!    default (only headings degrade an image to its alt text), so a layout cell now always
//!    keeps them too, independent of `keep_inline_images_in`.

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert, prescan, tier1};

fn base_options() -> ConversionOptions {
    ConversionOptions::default()
}

fn content(html: &str, options: ConversionOptions) -> String {
    convert(html, Some(options)).unwrap().content.unwrap_or_default()
}

/// The reporter's minimal repro: two rows, the second short by one cell, no header, no caption.
const RAGGED_TABLE: &str = "<table><tr><td>A</td><td>B</td></tr><tr><td>C</td></tr></table>";

#[test]
fn should_render_a_ragged_headerless_table_as_a_padded_pipe_table() {
    assert_eq!(
        content(RAGGED_TABLE, base_options()),
        "| A | B |\n| --- | --- |\n| C |   |\n"
    );
}

#[test]
fn should_render_a_ragged_headerless_table_as_a_padded_pipe_table_when_compact() {
    let options = ConversionOptions {
        compact_tables: true,
        ..base_options()
    };
    assert_eq!(content(RAGGED_TABLE, options), "| A | B |\n| --- | --- |\n| C |  |\n");
}

/// The same shape with an image in the first cell: the second half of #500 is that an image in
/// a ragged (now ordinary) table cell was never at risk -- only a genuine *layout* cell
/// degraded an image to alt text -- but this pins that an image in an ordinary cell always
/// stayed markdown, both before and after this fix.
#[test]
fn should_keep_an_image_as_markdown_in_a_ragged_pipe_table_cell() {
    let html = r#"<table><tr><td><img src="a.svg" alt="A"></td><td>B</td></tr><tr><td>C</td></tr></table>"#;
    assert_eq!(
        content(html, base_options()),
        "| ![A](a.svg) | B |\n| ----------- | --- |\n| C           |   |\n"
    );
}

/// A layout-path table (link-heavy, so still classified as layout after #500) whose cell holds
/// an image: the image must appear as `![A](a.svg)` inside the bullet, not degrade to `A`.
#[test]
fn should_keep_an_image_as_markdown_in_a_layout_row() {
    let html = concat!(
        r#"<table><tr><td><img src="a.svg" alt="A"> <a href="/1">x</a></td>"#,
        r#"<td><a href="/2">y</a></td></tr>"#,
        r#"<tr><td><a href="/3">z</a></td></tr></table>"#,
    );
    let out = content(html, base_options());
    assert!(
        out.contains("![A](a.svg)"),
        "image in a layout cell must stay markdown: {out:?}"
    );
    assert!(out.starts_with("- "), "expected a layout bullet list: {out:?}");
}

/// Tier-1 still bails on ragged rows (`BailReason::Classifier`) rather than learning to pad --
/// a stricter-than-Tier-2 bail is parity-safe, since it only ever routes more input to the
/// (authoritative) Tier-2 fallback. `TierStrategy::Auto` must still match Tier-2 byte for byte.
#[test]
fn should_bail_tier1_but_match_tier2_for_a_ragged_table() {
    let (cleaned, report) = prescan::run(RAGGED_TABLE);
    let tier1_options = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        ..base_options()
    };
    let tier1_result = tier1::run(cleaned.as_ref(), &report, &tier1_options);
    assert!(
        matches!(tier1_result, Err(tier1::BailReason::Classifier)),
        "expected Tier-1 to bail with BailReason::Classifier, got {tier1_result:?}"
    );

    let tier2_output = content(
        RAGGED_TABLE,
        ConversionOptions {
            tier_strategy: TierStrategy::Tier2,
            ..base_options()
        },
    );
    let auto_output = content(
        RAGGED_TABLE,
        ConversionOptions {
            tier_strategy: TierStrategy::Auto,
            ..base_options()
        },
    );
    assert_eq!(
        auto_output, tier2_output,
        "Auto routing must match Tier-2 byte for byte"
    );
}
