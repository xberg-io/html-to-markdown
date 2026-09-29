// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #646: a table whose only cell holds a line break was taken for
//! a blank spacer table under `br_in_tables: true`, so the full converter dropped the whole
//! table while the fast converter kept it.

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert, tier1};

fn tier2_options(br_in_tables: bool) -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        br_in_tables,
        tier_strategy: TierStrategy::Tier2,
        ..ConversionOptions::default()
    }
}

fn tier2(html: &str, br_in_tables: bool) -> String {
    convert(html, Some(tier2_options(br_in_tables)))
        .expect("conversion must succeed")
        .content
        .unwrap_or_default()
}

fn tier1_run(html: &str, br_in_tables: bool) -> Result<String, tier1::BailReason> {
    let options = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        ..tier2_options(br_in_tables)
    };
    tier1::run(html, &PrescanReport::default(), &options)
}

fn render(markdown: &str) -> String {
    let mut options = comrak::Options::default();
    options.extension.table = true;
    comrak::markdown_to_html(markdown, &options)
}

/// A line break in a cell, several ways, with the markdown both tiers write once
/// `br_in_tables` is on.
const BR_ONLY_CELL: [(&str, &str); 5] = [
    ("<table><tr><td><br></td></tr></table>", "| <br> |\n| ---- |\n"),
    (
        "<table><tr><td><span><br></span></td></tr></table>",
        "| <br> |\n| ---- |\n",
    ),
    (
        "<table><tr><td><div><br></div></td></tr></table>",
        "| <br> |\n| ---- |\n",
    ),
    (
        "<table><tr><td><br><br></td></tr></table>",
        "| <br><br> |\n| -------- |\n",
    ),
    (
        "<table><tr><td><br></td><td><br></td></tr></table>",
        "| <br> | <br> |\n| ---- | ---- |\n",
    ),
];

#[test]
fn should_keep_the_table_when_a_cell_holds_only_a_line_break_under_br_in_tables() {
    let mut failures = Vec::new();
    for (html, expected) in BR_ONLY_CELL {
        let markdown = tier2(html, true);
        if markdown != expected || !render(&markdown).starts_with("<table>") {
            failures.push(format!("{html:?}: {markdown:?}"));
        }
    }
    assert!(
        failures.is_empty(),
        "line break in a cell lost the table under br_in_tables:\n{}",
        failures.join("\n")
    );
}

#[test]
fn should_still_drop_the_table_when_a_cell_holds_only_a_line_break_without_br_in_tables() {
    for (html, _) in BR_ONLY_CELL {
        assert_eq!(tier2(html, false), "", "{html:?}");
    }
}

#[test]
fn should_write_the_same_table_in_both_tiers() {
    let mut failures = Vec::new();
    for (html, expected) in BR_ONLY_CELL {
        let t2 = tier2(html, true);
        match tier1_run(html, true) {
            Ok(t1) if t1 == expected && t2 == expected => {}
            other => failures.push(format!("{html:?}: tier1 {other:?} tier2 {t2:?} expected {expected:?}")),
        }
    }
    assert!(failures.is_empty(), "tiers differ:\n{}", failures.join("\n"));
}

#[test]
fn should_keep_a_header_row_table_and_write_the_break_in_the_body_cell() {
    let html = "<table><tr><th>H</th></tr><tr><td><br></td></tr></table>";
    assert_eq!(tier2(html, false), "| H |\n| --- |\n|   |\n");
    assert_eq!(tier2(html, true), "| H    |\n| ---- |\n| <br> |\n");
    assert!(matches!(
        tier1_run(html, false).as_deref(),
        Ok("| H |\n| --- |\n|   |\n")
    ));
    assert!(matches!(
        tier1_run(html, true).as_deref(),
        Ok("| H    |\n| ---- |\n| <br> |\n")
    ));
}

/// Siblings of the fixed shape that must stay unaffected: a truly blank cell, an
/// nbsp-only cell (including a styled email spacer shape), under both settings.
const UNAFFECTED_BLANK_CELL: [&str; 3] = [
    "<table><tr><td></td></tr></table>",
    "<table><tr><td>&nbsp;</td></tr></table>",
    "<table><tr><td style=\"height:1px\">&nbsp;</td></tr></table>",
];

#[test]
fn should_still_drop_other_blank_spacer_shapes() {
    for html in UNAFFECTED_BLANK_CELL {
        for br_in_tables in [false, true] {
            assert_eq!(tier2(html, br_in_tables), "", "{html:?} br_in_tables={br_in_tables}");
        }
    }
}
