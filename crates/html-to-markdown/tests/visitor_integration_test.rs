// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![allow(clippy::significant_drop_tightening)]

//! Integration tests for the visitor pattern
//!
//! These tests verify that visitor callbacks are properly invoked during
//! HTML→Markdown conversion and that all `VisitResult` variants work correctly.

#![cfg(feature = "visitor")]

use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, NodeType, VisitResult, VisitorHandle};
use html_to_markdown_rs::{ConversionError, ConversionOptions, ConversionResult};
use std::sync::{Arc, Mutex};

/// Test shim that bridges the legacy 3-arg call shape used throughout this file
/// onto the public 2-arg `convert(html, options)` API. The visitor (if any) is
/// folded into `options.visitor`.
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

/// Visitor receives `visit_form` callback even when preprocessing would normally drop the form
#[test]
fn test_visitor_form_custom_overrides_preprocessing() {
    #[derive(Debug, Default)]
    struct FormVisitor;
    impl HtmlVisitor for FormVisitor {
        fn visit_form(&mut self, _ctx: &NodeContext, action: Option<&str>, _method: Option<&str>) -> VisitResult {
            VisitResult::Custom(format!("[FORM:{}]", action.unwrap_or("none")))
        }
    }
    let html = r#"<div><form action="/submit" method="POST"><label>Name: <input type="text" name="name"></label><button type="submit">Submit</button></form></div>"#;
    let visitor: VisitorHandle = Arc::new(Mutex::new(FormVisitor));
    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();
    assert!(
        result.contains("[FORM:/submit]"),
        "Should contain custom form output, got: {result:?}"
    );
}

/// Visitor receives `visit_input` callback for inputs inside a form that preprocessing would drop
#[test]
fn test_visitor_input_custom_inside_preprocessed_form() {
    #[derive(Debug, Default)]
    struct InputVisitor;
    impl HtmlVisitor for InputVisitor {
        fn visit_input(
            &mut self,
            _ctx: &NodeContext,
            input_type: &str,
            _name: Option<&str>,
            _value: Option<&str>,
        ) -> VisitResult {
            VisitResult::Custom(format!("[INPUT:{input_type}]"))
        }
    }
    let html = r#"<form><input type="text" name="username"></form>"#;
    let visitor: VisitorHandle = Arc::new(Mutex::new(InputVisitor));
    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();
    assert!(
        result.contains("[INPUT:text]"),
        "Should contain custom input output, got: {result:?}"
    );
}

/// Visitor `figure_start` and `figure_end` are both called; `figure_end` receives `figure_start` output
#[test]
fn test_visitor_figure_start_end_both_called() {
    #[derive(Debug, Default)]
    struct FigureVisitor;
    impl HtmlVisitor for FigureVisitor {
        fn visit_figure_start(&mut self, _ctx: &NodeContext) -> VisitResult {
            VisitResult::Custom("[FIGURE_START]".to_string())
        }
        fn visit_figure_end(&mut self, _ctx: &NodeContext, output: &str) -> VisitResult {
            VisitResult::Custom(format!("{output}[/FIGURE]"))
        }
    }
    let html = r#"<figure><img src="test.jpg" alt="test"><figcaption>Caption</figcaption></figure>"#;
    let visitor: VisitorHandle = Arc::new(Mutex::new(FigureVisitor));
    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();
    assert!(
        result.contains("[FIGURE_START]"),
        "Should contain figure start marker, got: {result:?}"
    );
    assert!(
        result.contains("[/FIGURE]"),
        "Should contain figure end marker, got: {result:?}"
    );
}

/// Test visitor that customizes all output
#[derive(Debug, Default)]
struct CustomizingVisitor;

impl HtmlVisitor for CustomizingVisitor {
    fn visit_text(&mut self, _ctx: &NodeContext, text: &str) -> VisitResult {
        VisitResult::Custom(format!("[TEXT:{text}]"))
    }

    fn visit_link(&mut self, _ctx: &NodeContext, href: &str, text: &str, _title: Option<&str>) -> VisitResult {
        VisitResult::Custom(format!("[LINK:{text} -> {href}]"))
    }

    fn visit_image(&mut self, _ctx: &NodeContext, src: &str, alt: &str, _title: Option<&str>) -> VisitResult {
        VisitResult::Custom(format!("[IMAGE:{alt} @ {src}]"))
    }

    fn visit_heading(&mut self, _ctx: &NodeContext, level: u32, text: &str, _id: Option<&str>) -> VisitResult {
        VisitResult::Custom(format!("[H{level}: {text}]"))
    }
}

/// Test visitor that skips certain elements
#[derive(Debug, Default)]
struct SkippingVisitor {
    skip_images: bool,
    skip_links: bool,
}

impl HtmlVisitor for SkippingVisitor {
    fn visit_link(&mut self, _ctx: &NodeContext, _href: &str, _text: &str, _title: Option<&str>) -> VisitResult {
        if self.skip_links {
            VisitResult::Skip
        } else {
            VisitResult::Continue
        }
    }

    fn visit_image(&mut self, _ctx: &NodeContext, _src: &str, _alt: &str, _title: Option<&str>) -> VisitResult {
        if self.skip_images {
            VisitResult::Skip
        } else {
            VisitResult::Continue
        }
    }
}

/// Test visitor that preserves HTML for certain elements
#[derive(Debug, Default)]
struct PreservingVisitor {
    preserve_links: bool,
}

impl HtmlVisitor for PreservingVisitor {
    fn visit_link(&mut self, _ctx: &NodeContext, _href: &str, _text: &str, _title: Option<&str>) -> VisitResult {
        if self.preserve_links {
            VisitResult::PreserveHtml
        } else {
            VisitResult::Continue
        }
    }
}

/// Test visitor that validates node context
#[derive(Debug, Default)]
struct ContextCheckingVisitor {
    saw_heading_with_id: bool,
}

impl HtmlVisitor for ContextCheckingVisitor {
    fn visit_heading(&mut self, ctx: &NodeContext, _level: u32, _text: &str, _id: Option<&str>) -> VisitResult {
        assert_eq!(ctx.node_type, NodeType::Heading);
        assert_eq!(ctx.tag_name, "h1");

        if ctx.attributes().contains_key("id") {
            self.saw_heading_with_id = true;
        }

        VisitResult::Continue
    }
}

#[test]
fn test_custom_visitor_transforms_text() {
    let html = r"<p>Hello world</p>";
    let visitor = Arc::new(Mutex::new(CustomizingVisitor));

    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(result.contains("[TEXT:"), "Should contain custom text format");
}

#[test]
fn test_custom_visitor_transforms_links() {
    let html = r#"<a href="https://example.com">Example</a>"#;
    let visitor = Arc::new(Mutex::new(CustomizingVisitor));

    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(
        result.contains("[LINK:Example -> https://example.com]"),
        "Should contain custom link format, got: {result}"
    );
}

#[test]
fn test_custom_visitor_transforms_images() {
    let html = r#"<img src="/test.png" alt="Test">"#;
    let visitor = Arc::new(Mutex::new(CustomizingVisitor));

    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(
        result.contains("[IMAGE:Test @ /test.png]"),
        "Should contain custom image format, got: {result}"
    );
}

#[test]
fn test_custom_visitor_transforms_headings() {
    let html = r"<h2>My Heading</h2>";
    let visitor = Arc::new(Mutex::new(CustomizingVisitor));

    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(
        result.contains("[H2: My Heading]"),
        "Should contain custom heading format, got: {result}"
    );
}

#[test]
fn test_skipping_visitor_removes_links() {
    let html = r#"<p>Text with <a href="https://example.com">a link</a> inside.</p>"#;
    let visitor = Arc::new(Mutex::new(SkippingVisitor {
        skip_links: true,
        skip_images: false,
    }));

    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(
        !result.contains("example.com"),
        "Should not contain link URL when skipped, got: {result}"
    );
}

/// A link the visitor answers with `Continue` renders with its escaped label, destination and title.
#[test]
fn test_visitor_continue_renders_the_link() {
    let html = r#"<p>Text with <a href="https://example.com" title="Example">a ] link</a> inside.</p>"#;
    let visitor = Arc::new(Mutex::new(SkippingVisitor {
        skip_links: false,
        skip_images: false,
    }));

    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(
        result.contains(r#"Text with [a \] link](https://example.com "Example") inside."#),
        "Should render the link with its label, destination and title, got: {result}"
    );
}

#[test]
fn test_skipping_visitor_removes_images() {
    let html = r#"<p>Text <img src="/test.png" alt="Test"> more text</p>"#;
    let visitor = Arc::new(Mutex::new(SkippingVisitor {
        skip_links: false,
        skip_images: true,
    }));

    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(
        !result.contains("test.png") && !result.contains("!["),
        "Should not contain image when skipped, got: {result}"
    );
}

#[test]
fn test_preserving_visitor_keeps_html() {
    let html = r#"<a href="https://example.com" class="special">Example</a>"#;
    let visitor = Arc::new(Mutex::new(PreservingVisitor { preserve_links: true }));

    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(
        result.contains("<a") && result.contains("href"),
        "Should preserve HTML tags when PreserveHtml is returned, got: {result}"
    );
}

#[test]
fn test_visitor_receives_node_context() {
    let html = r#"<h1 id="title" class="main">Title</h1>"#;
    let visitor = Arc::new(Mutex::new(ContextCheckingVisitor::default()));

    let _result = convert(html, None, Some(visitor)).expect("conversion failed");
}

#[test]
fn test_visitor_works_with_complex_document() {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head><title>Test</title></head>
        <body>
            <h1>Main Title</h1>
            <p>Introduction with <a href="/link">a link</a>.</p>
            <h2>Section</h2>
            <p>Text with <strong>bold</strong> and <em>italic</em>.</p>
            <img src="/image.png" alt="Diagram">
            <h3>Subsection</h3>
            <p>More content.</p>
        </body>
        </html>
    "#;

    let visitor = Arc::new(Mutex::new(CustomizingVisitor));

    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(result.contains("[H1:"));
    assert!(result.contains("[H2:"));
    assert!(result.contains("[H3:"));
    assert!(result.contains("[LINK:"));
    assert!(result.contains("[IMAGE:"));
    assert!(result.contains("[TEXT:"));
}

#[test]
fn test_visitor_with_conversion_options() {
    #[derive(Debug, Default)]
    struct ContinueVisitor;

    impl HtmlVisitor for ContinueVisitor {}

    let html = r"<h1>Title</h1><p>Text with *asterisks* and _underscores_.</p>";

    let options = ConversionOptions {
        escape_asterisks: true,
        escape_underscores: true,
        ..Default::default()
    };

    let visitor = Arc::new(Mutex::new(ContinueVisitor));

    let result = convert(html, Some(options), Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(
        result.contains(r"\*") || result.contains(r"\_"),
        "Should respect escape options with visitor, got: {result}"
    );
}

#[test]
fn test_visitor_continue_result_produces_default_markdown() {
    #[derive(Debug, Default)]
    struct ContinueVisitor;

    impl HtmlVisitor for ContinueVisitor {
        fn visit_heading(&mut self, _ctx: &NodeContext, _level: u32, _text: &str, _id: Option<&str>) -> VisitResult {
            VisitResult::Continue
        }
    }

    let html = r"<h1>Title</h1>";
    let visitor = Arc::new(Mutex::new(ContinueVisitor));

    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(
        result.contains("# Title"),
        "Continue should produce default markdown, got: {result}"
    );
}

#[test]
fn test_visitor_skip_vs_continue() {
    #[derive(Debug)]
    struct SelectiveSkipper {
        skip_first_link: bool,
    }

    impl HtmlVisitor for SelectiveSkipper {
        fn visit_link(&mut self, _ctx: &NodeContext, _href: &str, _text: &str, _title: Option<&str>) -> VisitResult {
            if self.skip_first_link {
                self.skip_first_link = false;
                VisitResult::Skip
            } else {
                VisitResult::Continue
            }
        }
    }

    let html = r#"<p><a href="/first">First</a> and <a href="/second">Second</a></p>"#;
    let visitor = Arc::new(Mutex::new(SelectiveSkipper { skip_first_link: true }));

    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(!result.contains("/first"));
    assert!(result.contains("/second"));
}

#[test]
fn test_multiple_elements_of_same_type() {
    let html = r"<h1>First</h1><h2>Second</h2><h3>Third</h3>";
    let visitor = Arc::new(Mutex::new(CustomizingVisitor));

    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(result.contains("[H1: First]"));
    assert!(result.contains("[H2: Second]"));
    assert!(result.contains("[H3: Third]"));
}

#[test]
fn test_nested_elements_invoke_visitor() {
    let html = r#"<p>Text with <a href="/url">a <strong>bold</strong> link</a></p>"#;
    let visitor = Arc::new(Mutex::new(CustomizingVisitor));

    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(result.contains("[TEXT:"));
    assert!(result.contains("[LINK:"));
}

#[test]
fn test_visitor_error_stops_conversion() {
    #[derive(Debug, Default)]
    struct ErrorVisitor;

    impl HtmlVisitor for ErrorVisitor {
        fn visit_text(&mut self, _ctx: &NodeContext, _text: &str) -> VisitResult {
            VisitResult::Error("test error".to_string())
        }
    }

    let html = "<p>text</p>";
    let visitor = Arc::new(Mutex::new(ErrorVisitor));
    let result = convert(html, None, Some(visitor));

    assert!(result.is_err(), "Should return error when visitor returns Error");
    assert!(
        result.unwrap_err().to_string().contains("test error"),
        "Error message should contain visitor's error"
    );
}

#[test]
fn test_visitor_code_block() {
    #[derive(Debug, Default)]
    struct CodeBlockVisitor;

    impl HtmlVisitor for CodeBlockVisitor {
        fn visit_code_block(&mut self, _ctx: &NodeContext, language: Option<&str>, code: &str) -> VisitResult {
            let lang = language.unwrap_or("text");
            VisitResult::Custom(format!("[CODE_BLOCK:{} -> {}]", lang, code.trim()))
        }
    }

    let html = r#"<pre><code class="language-rust">fn main() {}</code></pre>"#;
    let visitor = Arc::new(Mutex::new(CodeBlockVisitor));
    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(
        result.contains("[CODE_BLOCK:rust -> fn main() {}]"),
        "Should contain custom code block format, got: {result}"
    );
}

#[test]
fn test_visitor_code_inline() {
    #[derive(Debug, Default)]
    struct InlineCodeVisitor;

    impl HtmlVisitor for InlineCodeVisitor {
        fn visit_code_inline(&mut self, _ctx: &NodeContext, code: &str) -> VisitResult {
            VisitResult::Custom(format!("[CODE:{code}]"))
        }
    }

    let html = r"<p>Use <code>println!</code> macro</p>";
    let visitor = Arc::new(Mutex::new(InlineCodeVisitor));
    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(
        result.contains("[CODE:println!]"),
        "Should contain custom inline code format, got: {result}"
    );
}

#[test]
fn test_visitor_list_callbacks() {
    #[derive(Debug, Default)]
    struct ListVisitor {
        list_depth: usize,
    }

    impl HtmlVisitor for ListVisitor {
        fn visit_list_start(&mut self, _ctx: &NodeContext, ordered: bool) -> VisitResult {
            self.list_depth += 1;
            VisitResult::Custom(format!(
                "[LIST_START:{}:{}]",
                if ordered { "OL" } else { "UL" },
                self.list_depth
            ))
        }

        fn visit_list_item(&mut self, _ctx: &NodeContext, _ordered: bool, _marker: &str, text: &str) -> VisitResult {
            VisitResult::Custom(format!("[LI:{}:{}]", self.list_depth, text.trim()))
        }

        fn visit_list_end(&mut self, _ctx: &NodeContext, _ordered: bool, _output: &str) -> VisitResult {
            let result = VisitResult::Custom(format!("[LIST_END:{}]", self.list_depth));
            self.list_depth = self.list_depth.saturating_sub(1);
            result
        }
    }

    let html = r"<ul><li>First</li><li>Second</li></ul>";
    let visitor = Arc::new(Mutex::new(ListVisitor::default()));
    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(
        result.contains("[LIST_START:UL:1]"),
        "Should see list start, got: {result}"
    );
    assert!(result.contains("[LI:1:First]"), "Should see first item, got: {result}");
    assert!(
        result.contains("[LI:1:Second]"),
        "Should see second item, got: {result}"
    );
    assert!(result.contains("[LIST_END:1]"), "Should see list end, got: {result}");
}

#[test]
fn test_visitor_table_callbacks() {
    #[derive(Debug, Default)]
    struct TableVisitor {
        row_count: usize,
    }

    impl HtmlVisitor for TableVisitor {
        fn visit_table_start(&mut self, _ctx: &NodeContext) -> VisitResult {
            self.row_count = 0;
            VisitResult::Custom("[TABLE_START]".to_string())
        }

        fn visit_table_row(&mut self, _ctx: &NodeContext, cells: &[String], is_header: bool) -> VisitResult {
            self.row_count += 1;
            VisitResult::Custom(format!(
                "[ROW:{}:{}:{}]",
                if is_header { "HEADER" } else { "DATA" },
                self.row_count,
                cells.join("|")
            ))
        }

        fn visit_table_end(&mut self, _ctx: &NodeContext, _output: &str) -> VisitResult {
            VisitResult::Custom(format!("[TABLE_END:{}]", self.row_count))
        }
    }

    let html = r"<table><tr><th>Name</th><th>Age</th></tr><tr><td>Alice</td><td>30</td></tr></table>";
    let visitor = Arc::new(Mutex::new(TableVisitor::default()));
    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(
        result.contains("[TABLE_START]"),
        "Should see table start, got: {result}"
    );
    assert!(
        result.contains("[ROW:HEADER:1:Name|Age]"),
        "Should see header row, got: {result}"
    );
    assert!(
        result.contains("[ROW:DATA:2:Alice|30]"),
        "Should see data row, got: {result}"
    );
    assert!(result.contains("[TABLE_END:2]"), "Should see table end, got: {result}");
}

include!("support/visitor_integration_extended.rs");

#[test]
fn should_keep_navigation_preprocessing_enabled_with_a_visitor() {
    #[derive(Debug)]
    struct EmptyVisitor;
    impl HtmlVisitor for EmptyVisitor {}

    let html = "<html><body><nav><a href='/'>Home</a></nav><main><p>Content</p></main></body></html>";
    let visitor: VisitorHandle = Arc::new(Mutex::new(EmptyVisitor));
    let options = ConversionOptions::default();
    assert!(options.preprocessing.enabled && options.preprocessing.remove_navigation);
    assert_eq!(
        convert(html, Some(options), Some(visitor)).unwrap().content.as_deref(),
        Some("Content\n"),
    );
}
