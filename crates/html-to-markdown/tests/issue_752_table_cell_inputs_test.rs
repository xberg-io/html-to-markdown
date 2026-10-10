// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Issue #752: every walk of a table cell knows whether the cell holds only inputs.
//!
//! A cell that holds only a checkbox writes the state of the checkbox. A cell that holds a
//! checkbox beside text writes only the text.

use html_to_markdown_rs::options::PreprocessingOptions;
use html_to_markdown_rs::{ConversionOptions, ConversionResult, TierStrategy, convert};

const DATA_TABLE: &str = r#"<table><tr><th><input type="checkbox"></th><th>Name</th></tr><tr><td><input type="checkbox" checked></td><td><input type="checkbox"> Bob</td></tr></table>"#;

fn options(tier_strategy: TierStrategy) -> ConversionOptions {
    ConversionOptions {
        tier_strategy,
        preprocessing: PreprocessingOptions {
            remove_forms: false,
            ..PreprocessingOptions::default()
        },
        ..ConversionOptions::default()
    }
}

fn result(html: &str, options: ConversionOptions) -> ConversionResult {
    convert(html, Some(options)).expect("conversion must succeed")
}

fn assert_all_converters(html: &str, expected: &str) {
    for tier_strategy in [TierStrategy::Tier2, TierStrategy::Tier1, TierStrategy::Auto] {
        assert_eq!(
            result(html, options(tier_strategy)).content.unwrap_or_default(),
            expected,
            "{tier_strategy:?}: {html}"
        );
    }
}

#[test]
fn should_write_the_checkbox_state_in_a_layout_table_cell_that_holds_only_inputs() {
    for (html, expected) in [
        (
            r#"<table><tr><td><a href="/a">a</a></td><td><input type="checkbox" checked></td><td><a href="/b">b</a></td><td><a href="/c">c</a></td></tr></table>"#,
            "- [a](/a) [x] [b](/b) [c](/c)\n",
        ),
        (
            r#"<table><tr><td><a href="/a">a</a></td><td><input type="checkbox"></td><td><a href="/b">b</a></td><td><a href="/c">c</a></td></tr></table>"#,
            "- [a](/a) [ ] [b](/b) [c](/c)\n",
        ),
        (
            r#"<table><tr><td><a href="/a">a</a></td><td><input type="checkbox" checked> x</td><td><a href="/b">b</a></td><td><a href="/c">c</a></td></tr></table>"#,
            "- [a](/a) x [b](/b) [c](/c)\n",
        ),
    ] {
        assert_all_converters(html, expected);
    }
}

#[test]
fn should_write_the_checkbox_state_in_the_grid_of_a_table() {
    let structured = result(
        DATA_TABLE,
        ConversionOptions {
            include_document_structure: true,
            ..options(TierStrategy::Auto)
        },
    );
    assert_eq!(structured.tables.len(), 1);
    let cells: Vec<&str> = structured.tables[0]
        .grid
        .cells
        .iter()
        .map(|cell| cell.content.as_str())
        .collect();
    assert_eq!(cells, ["[ ]", "Name", "[x]", "Bob"]);
}

#[test]
fn should_write_no_checkbox_state_in_an_element_that_is_not_a_table_cell() {
    assert_all_converters(
        r#"<table><tr><th>Done</th><th>Name</th></tr><row><cell><input type="checkbox" checked></cell><cell>Bob</cell></row></table>"#,
        "| Done | Name |\n| ---- | ---- |\n|      | Bob  |\n",
    );
}

#[cfg(feature = "visitor")]
mod visitor {
    use super::{DATA_TABLE, options, result};
    use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult, VisitorHandle};
    use html_to_markdown_rs::{ConversionOptions, TierStrategy};
    use std::sync::{Arc, Mutex};

    #[derive(Debug, Default)]
    struct Rows(Vec<Vec<String>>);

    impl HtmlVisitor for Rows {
        fn visit_table_row(&mut self, _ctx: &NodeContext<'_>, cells: &[String], _is_header: bool) -> VisitResult {
            self.0.push(cells.to_vec());
            VisitResult::Continue
        }
    }

    #[test]
    fn should_give_a_visitor_the_checkbox_state_of_a_cell_that_holds_only_inputs() {
        let rows = Arc::new(Mutex::new(Rows::default()));
        let visitor: VisitorHandle = rows.clone();
        let converted = result(
            DATA_TABLE,
            ConversionOptions {
                visitor: Some(visitor),
                ..options(TierStrategy::Auto)
            },
        );
        assert_eq!(
            converted.content.as_deref(),
            Some("| [ ] | Name |\n| --- | ---- |\n| [x] | Bob  |\n")
        );
        assert_eq!(
            rows.lock().expect("the visitor lock").0,
            [["[ ]", "Name"], ["[x]", "Bob"]]
        );
    }
}
