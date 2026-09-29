// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #628: a table whose cells hold only rules was taken for a blank
//! spacer table, so the full converter dropped the whole table.

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

/// A rule in each block position a cell can hold, with the markdown both tiers write.
const RULE_IN_CELL: [(&str, &str); 7] = [
    (
        "<table><tr><td><ul><li><hr></li></ul></td></tr></table>",
        "| --- |\n| --- |\n",
    ),
    ("<table><tr><td><hr></td></tr></table>", "| --- |\n| --- |\n"),
    (
        "<table><tr><td><ol><li><hr></li></ol></td></tr></table>",
        "| --- |\n| --- |\n",
    ),
    (
        "<table><tr><td><ul><li><ul><li><hr></li></ul></li></ul></td></tr></table>",
        "| --- |\n| --- |\n",
    ),
    (
        "<table><tr><td><hr></td><td><hr></td></tr></table>",
        "| --- | --- |\n| --- | --- |\n",
    ),
    (
        "<table><tr><td><hr><hr></td></tr></table>",
        "| ---  --- |\n| -------- |\n",
    ),
    (
        "<table><tr><th>H</th></tr><tr><td><ul><li><hr></li></ul></td></tr></table>",
        "| H   |\n| --- |\n| --- |\n",
    ),
];

#[test]
fn should_keep_the_table_when_a_list_item_in_a_cell_holds_only_a_rule() {
    let html = "<table><tr><td><ul><li><hr></li></ul></td></tr></table>";
    assert_eq!(tier2(html, false), "| --- |\n| --- |\n");
    assert_eq!(
        render(&tier2(html, false)),
        "<table>\n<thead>\n<tr>\n<th>---</th>\n</tr>\n</thead>\n</table>\n"
    );
}

#[test]
fn should_keep_the_table_for_a_rule_in_each_block_of_a_cell() {
    let mut failures = Vec::new();
    for br_in_tables in [false, true] {
        for (html, expected) in RULE_IN_CELL {
            let markdown = tier2(html, br_in_tables);
            if markdown != expected || !render(&markdown).starts_with("<table>") {
                failures.push(format!("{html:?} br_in_tables={br_in_tables}: {markdown:?}"));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "rule in a cell lost the table:\n{}",
        failures.join("\n")
    );
}

/// A rule in a quote in a cell. Tier 1 writes a quote in a cell without its marker, so only the
/// full converter's table is checked here.
const RULE_IN_QUOTE_IN_CELL: [&str; 2] = [
    "<table><tr><td><blockquote><hr></blockquote></td></tr></table>",
    "<table><tr><td><blockquote><ul><li><hr></li></ul></blockquote></td></tr></table>",
];

#[test]
fn should_keep_the_table_for_a_rule_in_a_quote_in_a_cell() {
    for br_in_tables in [false, true] {
        for html in RULE_IN_QUOTE_IN_CELL {
            let markdown = tier2(html, br_in_tables);
            assert_eq!(
                markdown, "| > --- |\n| ----- |\n",
                "{html:?} br_in_tables={br_in_tables}"
            );
            assert!(render(&markdown).starts_with("<table>"), "{html:?}: {markdown:?}");
        }
    }
}

#[test]
fn should_write_the_same_table_in_both_tiers() {
    let mut failures = Vec::new();
    for br_in_tables in [false, true] {
        for (html, expected) in RULE_IN_CELL {
            match tier1_run(html, br_in_tables) {
                Ok(markdown) if markdown == expected => {}
                other => failures.push(format!("{html:?} br_in_tables={br_in_tables}: {other:?}")),
            }
        }
    }
    assert!(failures.is_empty(), "tier 1 differs:\n{}", failures.join("\n"));
}

/// Text in a cell before a list that looks like a list marker, and a rule in a later item.
const TEXT_BEFORE_RULE_IN_CELL: [(&str, bool, &str); 5] = [
    (
        "<table><tr><td>- <ul><li><hr></li></ul></td></tr></table>",
        false,
        "| -  --- |\n| ------ |\n",
    ),
    (
        "<table><tr><td>1. <ol><li><hr></li></ol></td></tr></table>",
        false,
        "| 1.  --- |\n| ------- |\n",
    ),
    (
        "<table><tr><td>- <ul><li><hr></li></ul></td></tr></table>",
        true,
        "| -  --- |\n| ------ |\n",
    ),
    (
        "<table><tr><td><ul><li>a</li><li><hr></li></ul></td></tr></table>",
        false,
        "| a  --- |\n| ------ |\n",
    ),
    (
        "<table><tr><td><ul><li>a</li><li><hr></li></ul></td></tr></table>",
        true,
        "| a<br>  --- |\n| ---------- |\n",
    ),
];

#[test]
fn should_write_the_same_rule_after_text_in_a_cell_in_both_tiers() {
    let mut failures = Vec::new();
    for (html, br_in_tables, expected) in TEXT_BEFORE_RULE_IN_CELL {
        let tier2_out = tier2(html, br_in_tables);
        let tier1_out = tier1_run(html, br_in_tables);
        if tier2_out != expected || tier1_out.as_deref().ok() != Some(expected) {
            failures.push(format!(
                "{html:?} br_in_tables={br_in_tables}: tier2 {tier2_out:?} tier1 {tier1_out:?}"
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "rule after text in a cell:\n{}",
        failures.join("\n")
    );
}

#[test]
fn should_keep_a_nested_table_whose_cell_holds_only_a_rule() {
    let html = "<table><tr><td><table><tr><td><ul><li><hr></li></ul></td></tr></table></td></tr></table>";
    let markdown = tier2(html, false);
    assert_eq!(markdown, "| --- |\n| --- |\n");
    assert!(render(&markdown).starts_with("<table>"), "{markdown:?}");
}

#[test]
fn should_still_drop_a_blank_table() {
    assert_eq!(tier2("<table><tr><td></td></tr></table>", false), "");
    assert_eq!(tier2("<table><tr><td> </td><td></td></tr></table>", true), "");
}
