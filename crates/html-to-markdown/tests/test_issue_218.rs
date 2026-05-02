//! Test to reproduce issue #218: Rust panic with Cyrillic HTML
//!
//! When converting HTML containing multi-byte UTF-8 characters (e.g., Cyrillic)
//! with tabs between block elements and any visitor, a panic occurs:
//! "byte index N is not a char boundary"

#![cfg(feature = "visitor")]

use std::cell::RefCell;
use std::rc::Rc;

use html_to_markdown_rs::visitor::{HtmlVisitor, VisitorHandle};
use html_to_markdown_rs::{ConversionError, ConversionOptions, ConversionResult};

fn convert(
    html: &str,
    options: Option<ConversionOptions>,
    visitor: Option<VisitorHandle>,
) -> Result<ConversionResult, ConversionError> {
    let mut opts = options.unwrap_or_default();
    if visitor.is_some() {
        opts.visitor = visitor;
    }
    html_to_markdown_rs::convert(html, Some(opts))
}

/// Empty visitor — does nothing, just uses default implementations.
#[derive(Debug, Default)]
struct EmptyVisitor;

impl HtmlVisitor for EmptyVisitor {}

fn make_visitor() -> Rc<RefCell<dyn HtmlVisitor>> {
    Rc::new(RefCell::new(EmptyVisitor))
}

#[test]
fn test_cyrillic_with_tabs_between_divs_and_visitor() {
    // Exact reproduction from the issue
    let html = "<div><span>А</span></div>\t\t\t<div><span>По";
    let result = convert(html, None, Some(make_visitor()));
    assert!(result.is_ok(), "Should not panic: {result:?}");
}

#[test]
fn test_multibyte_utf8_with_tabs_and_visitor() {
    let cases = [
        "<div><span>日本語</span></div>\t\t\t<div><span>テスト",
        "<div><span>한국어</span></div>\t\t\t<div><span>테스트",
        "<div><span>Привет</span></div>\t\t\t<div><span>Мир",
        "<div><span>🎉</span></div>\t\t\t<div><span>🚀",
    ];

    for html in &cases {
        let result = convert(html, None, Some(make_visitor()));
        assert!(result.is_ok(), "Should not panic for: {html}\nError: {result:?}");
    }
}

#[test]
fn test_cyrillic_with_varying_tab_counts_and_visitor() {
    for n in 1..=5 {
        let tabs = "\t".repeat(n);
        let html = format!("<div><span>А</span></div>{tabs}<div><span>По");
        let result = convert(&html, None, Some(make_visitor()));
        assert!(result.is_ok(), "Should not panic with {n} tabs: {result:?}");
    }
}
