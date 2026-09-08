//! Regression tests for issue #474: empty nested lists in table cells must not
//! invalidate the output positions used by list visitor callbacks.

#![cfg(feature = "visitor")]

use std::fmt::Write;
use std::sync::{Arc, Mutex};

use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult};
use html_to_markdown_rs::{ConversionError, ConversionOptions, convert};

#[derive(Debug)]
struct ContinueVisitor;

impl HtmlVisitor for ContinueVisitor {}

fn nested_table(tags: &[&str]) -> String {
    let mut html = String::from("<table><tr><td>329</td><td>");
    for tag in tags {
        write!(html, "<{tag}><li>").unwrap();
    }
    for tag in tags.iter().rev() {
        write!(html, "</li></{tag}>").unwrap();
    }
    html.push_str("</td></tr></table>");
    html
}

#[test]
fn should_preserve_output_with_noop_visitor_for_empty_nested_lists_in_cells() {
    for tags in [["ol"; 4], ["ul"; 4], ["ol", "ul", "ol", "ul"], ["ul", "ol", "ul", "ol"]] {
        for nesting in 1..=tags.len() {
            let html = nested_table(&tags[..nesting]);
            for br_in_tables in [false, true] {
                let options = ConversionOptions {
                    br_in_tables,
                    ..Default::default()
                };
                let expected = convert(&html, Some(options.clone())).expect("conversion without visitor must succeed");
                let actual = convert(
                    &html,
                    Some(ConversionOptions {
                        visitor: Some(Arc::new(Mutex::new(ContinueVisitor))),
                        ..options
                    }),
                )
                .unwrap_or_else(|err| panic!("conversion with visitor failed for {html}: {err}"));
                assert_eq!(
                    actual.content, expected.content,
                    "HTML: {html}, br_in_tables: {br_in_tables}"
                );
                assert_eq!(actual.content.as_deref(), Some("| 329 |  |\n| --- | --- |\n"));
            }
        }
    }
}

#[derive(Debug)]
struct UnicodeItemVisitor;

impl HtmlVisitor for UnicodeItemVisitor {
    fn visit_list_item(&mut self, ctx: &NodeContext, _ordered: bool, _marker: &str, _text: &str) -> VisitResult {
        if ctx.attributes().contains_key("id") {
            VisitResult::Custom("🚀".to_string())
        } else {
            VisitResult::Continue
        }
    }
}

#[test]
fn should_respect_utf8_boundaries_after_list_item_visitor_replaces_indentation() {
    for tag in ["ol", "ul"] {
        let html = nested_table(&[tag; 3]).replace("<li></li>", "<li id=\"replace\"></li>");
        let options = ConversionOptions {
            visitor: Some(Arc::new(Mutex::new(UnicodeItemVisitor))),
            ..Default::default()
        };
        let result =
            convert(&html, Some(options)).expect("Unicode list item output must not invalidate list positions");
        assert_eq!(result.content.as_deref(), Some("| 329 | 🚀 |\n| --- | --- |\n"));
    }
}

#[derive(Debug)]
struct ListActionVisitor {
    custom_start: bool,
    end_result: VisitResult,
    end_content: Vec<String>,
}

impl HtmlVisitor for ListActionVisitor {
    fn visit_list_start(&mut self, ctx: &NodeContext, _ordered: bool) -> VisitResult {
        if self.custom_start && ctx.attributes().contains_key("id") {
            VisitResult::Custom("[start]".to_string())
        } else {
            VisitResult::Continue
        }
    }

    fn visit_list_end(&mut self, ctx: &NodeContext, _ordered: bool, output: &str) -> VisitResult {
        if ctx.attributes().contains_key("id") {
            self.end_content.push(output.to_string());
            self.end_result.clone()
        } else {
            VisitResult::Continue
        }
    }
}

#[test]
fn should_honor_list_visitor_actions_after_children_shrink_output() {
    for tag in ["ol", "ul"] {
        // The third list starts after indentation that its empty child removes.
        let preserved = format!("<{tag} id=\"target\"><li></li></{tag}>");
        let html = format!(
            "<table><tr><td>329</td><td><{tag}><li><{tag}><li>{preserved}</li></{tag}></li></{tag}></td></tr></table>"
        );
        for (end_result, custom_start, expected_cell) in [
            (VisitResult::Continue, true, Some("[start]")),
            (VisitResult::Custom("[end]".to_string()), false, Some("[end]")),
            (VisitResult::Custom("[end]".to_string()), true, Some("[start][end]")),
            (VisitResult::Skip, true, Some("")),
            (VisitResult::PreserveHtml, true, Some(preserved.as_str())),
            (VisitResult::Error("list rejected".to_string()), true, None),
        ] {
            let visitor = Arc::new(Mutex::new(ListActionVisitor {
                custom_start,
                end_result,
                end_content: Vec::new(),
            }));
            let options = ConversionOptions {
                visitor: Some(visitor.clone()),
                ..Default::default()
            };
            let result = convert(&html, Some(options));
            let end_content = visitor.lock().unwrap().end_content.clone();
            assert!(!end_content.is_empty(), "list end callback must be invoked");
            assert!(
                end_content.iter().all(String::is_empty),
                "HTML: {html}, content: {end_content:?}"
            );
            if let Some(cell) = expected_cell {
                let actual = result
                    .expect("list visitor action must not panic")
                    .content
                    .expect("Markdown content must be present");
                assert_eq!(actual.lines().next(), Some(format!("| 329 | {cell} |").as_str()));
            } else {
                assert!(matches!(result, Err(ConversionError::Visitor(message)) if message == "list rejected"));
            }
        }
    }
}
