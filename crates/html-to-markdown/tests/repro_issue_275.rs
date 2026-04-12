//! Regression test for issue #275: Metadata lost when using a visitor.

use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult};
use html_to_markdown_rs::{ConversionOptions, convert_with_visitor_result};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct DummyVisitor;
impl HtmlVisitor for DummyVisitor {
    fn visit_element_start(&mut self, _ctx: &NodeContext) -> VisitResult {
        VisitResult::Continue
    }
}

#[test]
fn test_repro_issue_275_with_doctype() {
    let html = r"<!DOCTYPE html>
<html>
<head>
    <title>Sample Document</title>
</head>
<body>
    <h1>Hello</h1>
</body>
</html>";

    let options = ConversionOptions {
        extract_metadata: true,
        ..ConversionOptions::default()
    };

    // 1. Without visitor
    let result_no_visitor = convert_with_visitor_result(html, Some(options.clone()), None).unwrap();
    assert_eq!(
        result_no_visitor.metadata.document.title,
        Some("Sample Document".to_string())
    );

    // 2. With visitor
    let visitor = Rc::new(RefCell::new(DummyVisitor));
    let result_with_visitor = convert_with_visitor_result(html, Some(options), Some(visitor)).unwrap();

    // THIS IS THE CRITICAL ASSERTION
    assert_eq!(
        result_with_visitor.metadata.document.title,
        Some("Sample Document".to_string()),
        "Metadata title lost when visitor is present!"
    );
}
