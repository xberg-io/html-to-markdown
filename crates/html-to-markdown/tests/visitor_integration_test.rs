#![allow(missing_docs)]

//! Integration tests for the visitor pattern
//!
//! These tests verify that visitor callbacks are properly invoked during
//! HTML→Markdown conversion and that all `VisitResult` variants work correctly.

#![cfg(feature = "visitor")]

use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, NodeType, VisitResult, VisitorHandle};
use html_to_markdown_rs::{ConversionError, ConversionOptions, ConversionResult};
use std::cell::RefCell;
use std::rc::Rc;

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

        if ctx.attributes.contains_key("id") {
            self.saw_heading_with_id = true;
        }

        VisitResult::Continue
    }
}

#[test]
fn test_custom_visitor_transforms_text() {
    let html = r"<p>Hello world</p>";
    let visitor = Rc::new(RefCell::new(CustomizingVisitor));

    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(result.contains("[TEXT:"), "Should contain custom text format");
}

#[test]
fn test_custom_visitor_transforms_links() {
    let html = r#"<a href="https://example.com">Example</a>"#;
    let visitor = Rc::new(RefCell::new(CustomizingVisitor));

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
    let visitor = Rc::new(RefCell::new(CustomizingVisitor));

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
    let visitor = Rc::new(RefCell::new(CustomizingVisitor));

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
    let visitor = Rc::new(RefCell::new(SkippingVisitor {
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

#[test]
fn test_skipping_visitor_removes_images() {
    let html = r#"<p>Text <img src="/test.png" alt="Test"> more text</p>"#;
    let visitor = Rc::new(RefCell::new(SkippingVisitor {
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
    let visitor = Rc::new(RefCell::new(PreservingVisitor { preserve_links: true }));

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
    let visitor = Rc::new(RefCell::new(ContextCheckingVisitor::default()));

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

    let visitor = Rc::new(RefCell::new(CustomizingVisitor));

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

    let visitor = Rc::new(RefCell::new(ContinueVisitor));

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
    let visitor = Rc::new(RefCell::new(ContinueVisitor));

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
    let visitor = Rc::new(RefCell::new(SelectiveSkipper { skip_first_link: true }));

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
    let visitor = Rc::new(RefCell::new(CustomizingVisitor));

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
    let visitor = Rc::new(RefCell::new(CustomizingVisitor));

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
    let visitor = Rc::new(RefCell::new(ErrorVisitor));
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
    let visitor = Rc::new(RefCell::new(CodeBlockVisitor));
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
    let visitor = Rc::new(RefCell::new(InlineCodeVisitor));
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
    let visitor = Rc::new(RefCell::new(ListVisitor::default()));
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
    let visitor = Rc::new(RefCell::new(TableVisitor::default()));
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

#[test]
fn test_visitor_blockquote() {
    #[derive(Debug, Default)]
    struct BlockquoteVisitor;

    impl HtmlVisitor for BlockquoteVisitor {
        fn visit_blockquote(&mut self, _ctx: &NodeContext, content: &str, _depth: usize) -> VisitResult {
            VisitResult::Custom(format!("[QUOTE:{}]", content.trim()))
        }
    }

    let html = r"<blockquote>This is a quote</blockquote>";
    let visitor = Rc::new(RefCell::new(BlockquoteVisitor));
    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(
        result.contains("[QUOTE:This is a quote]"),
        "Should contain custom blockquote format, got: {result}"
    );
}

#[test]
fn test_visitor_inline_formatting() {
    #[derive(Debug, Default)]
    struct FormattingVisitor;

    impl HtmlVisitor for FormattingVisitor {
        fn visit_strong(&mut self, _ctx: &NodeContext, text: &str) -> VisitResult {
            VisitResult::Custom(format!("[STRONG:{text}]"))
        }

        fn visit_emphasis(&mut self, _ctx: &NodeContext, text: &str) -> VisitResult {
            VisitResult::Custom(format!("[EM:{text}]"))
        }

        fn visit_strikethrough(&mut self, _ctx: &NodeContext, text: &str) -> VisitResult {
            VisitResult::Custom(format!("[DEL:{text}]"))
        }
    }

    let html = r"<p><strong>bold</strong> <em>italic</em> <del>struck</del></p>";
    let visitor = Rc::new(RefCell::new(FormattingVisitor));
    let result = convert(html, None, Some(visitor))
        .expect("conversion failed")
        .content
        .unwrap_or_default();

    assert!(result.contains("[STRONG:bold]"), "Should see strong, got: {result}");
    assert!(result.contains("[EM:italic]"), "Should see emphasis, got: {result}");
    assert!(
        result.contains("[DEL:struck]"),
        "Should see strikethrough, got: {result}"
    );
}

#[test]
fn test_no_double_visit_in_links() {
    #[derive(Debug, Default)]
    struct CountingVisitor {
        text_visits: usize,
    }

    impl HtmlVisitor for CountingVisitor {
        fn visit_text(&mut self, _ctx: &NodeContext, _text: &str) -> VisitResult {
            self.text_visits += 1;
            VisitResult::Continue
        }

        fn visit_link(&mut self, _ctx: &NodeContext, _href: &str, _text: &str, _title: Option<&str>) -> VisitResult {
            VisitResult::Continue
        }
    }

    let html = r#"<a href="/url">link text</a>"#;
    let visitor = Rc::new(RefCell::new(CountingVisitor::default()));
    let _result = convert(html, None, Some(visitor.clone())).expect("conversion failed");

    assert_eq!(
        visitor.borrow().text_visits,
        1,
        "Text nodes inside links should only be visited once, got {} visits",
        visitor.borrow().text_visits
    );
}

#[test]
fn test_no_double_visit_in_headings() {
    #[derive(Debug, Default)]
    struct CountingVisitor {
        text_visits: usize,
    }

    impl HtmlVisitor for CountingVisitor {
        fn visit_text(&mut self, _ctx: &NodeContext, _text: &str) -> VisitResult {
            self.text_visits += 1;
            VisitResult::Continue
        }

        fn visit_heading(&mut self, _ctx: &NodeContext, _level: u32, _text: &str, _id: Option<&str>) -> VisitResult {
            VisitResult::Continue
        }
    }

    let html = r"<h1>heading text</h1>";
    let visitor = Rc::new(RefCell::new(CountingVisitor::default()));
    let _result = convert(html, None, Some(visitor.clone())).expect("conversion failed");

    assert_eq!(
        visitor.borrow().text_visits,
        1,
        "Text nodes inside headings should only be visited once, got {} visits",
        visitor.borrow().text_visits
    );
}

// ============================================================================
// Integration tests: Visitor + Feature combinations
// ============================================================================

/// Test that visitor callbacks work correctly when `skip_images` option is enabled
#[test]
fn test_visitor_with_skip_images() {
    #[derive(Debug, Default)]
    struct SkipImageVisitor {
        image_visits: usize,
    }

    impl HtmlVisitor for SkipImageVisitor {
        fn visit_image(&mut self, _ctx: &NodeContext, _src: &str, _alt: &str, _title: Option<&str>) -> VisitResult {
            self.image_visits += 1;
            VisitResult::Continue
        }
    }

    let html = r#"
        <p>Some text</p>
        <img src="/image1.png" alt="Image 1">
        <img src="/image2.png" alt="Image 2">
        <p>More text</p>
    "#;

    // Test with skip_images enabled and visitor
    let options = ConversionOptions {
        skip_images: true,
        ..Default::default()
    };

    let visitor = Rc::new(RefCell::new(SkipImageVisitor::default()));
    let result = convert(html, Some(options), Some(visitor))
        .expect("conversion with skip_images and visitor should succeed")
        .content
        .unwrap_or_default();

    // When skip_images is true, images should not appear in output
    assert!(
        !result.contains("!["),
        "skip_images should prevent image markdown in output, got: {result}"
    );
    assert!(
        !result.contains("image1.png"),
        "skip_images should prevent image src in output, got: {result}"
    );

    // When skip_images is true, the conversion still happens correctly
    // Images are filtered at the conversion level based on the option
    // This verifies that skip_images option and visitor parameters work together
    // without conflicts - both are optional and can be combined
    assert!(
        result.contains("Some text") && result.contains("More text"),
        "Other content should still be present in output, got: {result}"
    );
}

/// Test that the main `convert()` function accepts optional visitor parameter
#[test]
fn test_convert_accepts_visitor_parameter() {
    #[derive(Debug, Default)]
    struct CountingVisitor {
        text_count: usize,
        link_count: usize,
    }

    impl HtmlVisitor for CountingVisitor {
        fn visit_text(&mut self, _ctx: &NodeContext, _text: &str) -> VisitResult {
            self.text_count += 1;
            VisitResult::Continue
        }

        fn visit_link(&mut self, _ctx: &NodeContext, _href: &str, _text: &str, _title: Option<&str>) -> VisitResult {
            self.link_count += 1;
            VisitResult::Continue
        }
    }

    let html = r#"<p>Visit <a href="https://example.com">our site</a> for more info.</p>"#;
    let visitor = Rc::new(RefCell::new(CountingVisitor::default()));

    // Test using the main convert() function with visitor parameter
    let _result = convert(html, None, Some(visitor.clone())).expect("convert with visitor should work");

    let borrowed = visitor.borrow();
    assert!(
        borrowed.text_count >= 2,
        "Should visit text nodes, got {} visits",
        borrowed.text_count
    );
    assert_eq!(
        borrowed.link_count, 1,
        "Should visit exactly 1 link, got {}",
        borrowed.link_count
    );
}

/// Test visitor + `inline_images` feature combination
///
/// In v3, `convert()` handles inline-image extraction via `ConversionResult.images`,
/// and `convert_with_visitor()` handles visitor callbacks. We verify both paths
/// work on the same HTML.
#[cfg(feature = "inline-images")]
#[test]
fn test_convert_with_inline_images_accepts_visitor() {
    #[derive(Debug, Default)]
    struct ImageTrackingVisitor {
        images_seen: usize,
    }

    impl HtmlVisitor for ImageTrackingVisitor {
        fn visit_image(&mut self, _ctx: &NodeContext, src: &str, _alt: &str, _title: Option<&str>) -> VisitResult {
            if !src.starts_with("data:") {
                self.images_seen += 1;
            }
            VisitResult::Continue
        }
    }

    let html = r#"
        <h1>Test Page</h1>
        <img src="/image.png" alt="Test Image">
        <p>Some content</p>
    "#;

    // Verify visitor callbacks fire via convert_with_visitor
    let visitor = Rc::new(RefCell::new(ImageTrackingVisitor::default()));
    let markdown = convert(html, None, Some(visitor.clone()))
        .expect("convert should work")
        .content
        .unwrap_or_default();

    assert_eq!(
        visitor.borrow().images_seen,
        1,
        "Visitor should count 1 non-data-uri image"
    );

    // Markdown should still be generated
    assert!(!markdown.is_empty(), "Should produce markdown output");
}

/// Test visitor + metadata: visitor callbacks fire and metadata is collected.
///
/// In v3, `convert()` always extracts metadata into `ConversionResult.metadata`,
/// and `convert_with_visitor()` handles visitor callbacks. We verify both paths
/// work on the same HTML.
#[cfg(feature = "metadata")]
#[test]
fn test_visitor_and_metadata_both_work() {
    #[derive(Debug, Default)]
    struct MetadataAwareVisitor {
        heading_count: usize,
        link_count: usize,
    }

    impl HtmlVisitor for MetadataAwareVisitor {
        fn visit_heading(&mut self, _ctx: &NodeContext, _level: u32, _text: &str, _id: Option<&str>) -> VisitResult {
            self.heading_count += 1;
            VisitResult::Continue
        }

        fn visit_link(&mut self, _ctx: &NodeContext, _href: &str, _text: &str, _title: Option<&str>) -> VisitResult {
            self.link_count += 1;
            VisitResult::Continue
        }
    }

    let html = r#"
        <html>
        <head><title>Test Page</title></head>
        <body>
            <h1>Main Title</h1>
            <p>Visit <a href="https://example.com">our site</a>.</p>
            <h2>Section</h2>
            <p>More <a href="/page">links</a> here.</p>
        </body>
        </html>
    "#;

    // Verify visitor callbacks fire via convert_with_visitor
    let visitor = Rc::new(RefCell::new(MetadataAwareVisitor::default()));
    let markdown = convert(html, None, Some(visitor.clone()))
        .expect("convert should work")
        .content
        .unwrap_or_default();

    let borrowed = visitor.borrow();
    assert!(
        borrowed.heading_count >= 2,
        "Visitor should see at least 2 headings, got {}",
        borrowed.heading_count
    );
    assert_eq!(
        borrowed.link_count, 2,
        "Visitor should see 2 links, got {}",
        borrowed.link_count
    );
    assert!(!markdown.is_empty(), "Should produce markdown output");
    drop(borrowed);

    // Verify metadata extraction via convert()
    let result = html_to_markdown_rs::convert(html, None).expect("convert should work");
    let metadata = result.metadata;

    assert_eq!(
        metadata.document.title,
        Some("Test Page".to_string()),
        "Metadata should extract title"
    );
    assert!(
        metadata.headers.len() >= 2,
        "Metadata should extract at least 2 headers, got {}",
        metadata.headers.len()
    );
    assert_eq!(
        metadata.links.len(),
        2,
        "Metadata should extract 2 links, got {}",
        metadata.links.len()
    );
}

/// Test visitor + both `inline_images` and `metadata` features together
///
/// In v3, `convert()` handles metadata and inline-image extraction via `ConversionResult`,
/// and `convert_with_visitor()` handles visitor callbacks. We verify both paths
/// work on the same HTML.
#[cfg(all(feature = "inline-images", feature = "metadata"))]
#[test]
fn test_convert_with_all_features_and_visitor() {
    #[derive(Debug, Default)]
    struct ComprehensiveVisitor {
        headings: usize,
        images: usize,
        links: usize,
    }

    impl HtmlVisitor for ComprehensiveVisitor {
        fn visit_heading(&mut self, _ctx: &NodeContext, _level: u32, _text: &str, _id: Option<&str>) -> VisitResult {
            self.headings += 1;
            VisitResult::Continue
        }

        fn visit_image(&mut self, _ctx: &NodeContext, _src: &str, _alt: &str, _title: Option<&str>) -> VisitResult {
            self.images += 1;
            VisitResult::Continue
        }

        fn visit_link(&mut self, _ctx: &NodeContext, _href: &str, _text: &str, _title: Option<&str>) -> VisitResult {
            self.links += 1;
            VisitResult::Continue
        }
    }

    let html = r#"
        <html>
        <body>
            <h1>Gallery</h1>
            <img src="/gallery/image1.jpg" alt="Pic 1">
            <p>See <a href="/more">more</a> content.</p>
            <h2>Details</h2>
            <img src="/gallery/image2.jpg" alt="Pic 2">
            <p>Check <a href="/details">this link</a>.</p>
        </body>
        </html>
    "#;

    // Verify visitor callbacks fire via convert_with_visitor
    let visitor = Rc::new(RefCell::new(ComprehensiveVisitor::default()));
    let markdown = convert(html, None, Some(visitor.clone()))
        .expect("convert should work")
        .content
        .unwrap_or_default();

    // Verify all visitor callbacks were invoked
    let borrowed = visitor.borrow();
    assert!(
        borrowed.headings >= 2,
        "Visitor should see at least 2 headings, got {}",
        borrowed.headings
    );
    assert_eq!(
        borrowed.images, 2,
        "Visitor should see 2 images, got {}",
        borrowed.images
    );
    assert_eq!(borrowed.links, 2, "Visitor should see 2 links, got {}", borrowed.links);
    drop(borrowed);

    // Verify markdown was produced
    assert!(!markdown.is_empty(), "Should produce markdown output");
}

/// Regression test: image visitor returning Custom with metadata extraction used to panic
/// with an out-of-bounds slice.
///
/// When metadata extraction prepends a YAML frontmatter block to `output`, every element's
/// saved `element_output_start` is offset by the frontmatter length.  If a child visitor
/// then returns Custom and truncates the buffer, the parent's saved offset can point
/// past `output.len()`.
#[test]
fn test_image_visitor_with_metadata_does_not_panic() {
    #[derive(Debug)]
    struct ImageVisitor;

    impl HtmlVisitor for ImageVisitor {
        fn visit_image(&mut self, _ctx: &NodeContext, _src: &str, _alt: &str, _title: Option<&str>) -> VisitResult {
            VisitResult::Custom("![img](rewritten.png)".to_string())
        }
    }

    let html = r#"<html><head><meta name="description" content="x"></head><body><p><img src="a.png" alt="a"></p></body></html>"#;
    let options = ConversionOptions {
        extract_metadata: true,
        ..Default::default()
    };

    let result = convert(html, Some(options), Some(Rc::new(RefCell::new(ImageVisitor))));
    assert!(result.is_ok(), "conversion panicked or errored: {:?}", result.err());
}

/// Regression test: `visit_element_end` returning Custom/Skip with metadata extraction used
/// to produce stale parent offsets and either panic or silently drop subsequent content.
#[test]
fn test_element_end_replacement_with_metadata_preserves_subsequent_content() {
    #[derive(Debug)]
    struct FigureReplacingVisitor;

    impl HtmlVisitor for FigureReplacingVisitor {
        fn visit_element_end(&mut self, ctx: &NodeContext, _content: &str) -> VisitResult {
            if ctx.tag_name == "figure" {
                return VisitResult::Custom("[figure]".to_string());
            }
            VisitResult::Continue
        }
    }

    let html = r#"<html><head><meta name="description" content="x"></head><body><figure><img src="a.png"></figure><p>after</p></body></html>"#;
    let options = ConversionOptions {
        extract_metadata: true,
        ..Default::default()
    };

    let result = convert(html, Some(options), Some(Rc::new(RefCell::new(FigureReplacingVisitor))));
    assert!(result.is_ok(), "conversion panicked or errored: {:?}", result.err());
    assert!(
        result.unwrap().content.unwrap_or_default().contains("after"),
        "content after replaced element should not be lost"
    );
}

/// Regression test for issue #331: visitor receives mismatched start/end events for
/// hyphenated tag names that contain XML-style self-closing children.
///
/// When `<ac:parameter ac:name="foo" />` appears inside a hyphenated custom element, the
/// `repair_with_html5ever` fallback (triggered because the outer tag contains a hyphen) used
/// to re-parse with HTML5 semantics.  HTML5 does NOT honour XML-style self-closing on unknown
/// elements, so `<ac:parameter ... />` was treated as an open tag and subsequent siblings were
/// nested inside it.  That caused `visit_element_start("ac:parameter")` for "foo" to be
/// followed by `visit_element_start("ac:parameter")` for "quux", then both ends in reversed
/// order — violating the expected pre-order/post-order pairing.
#[test]
fn test_issue_331_hyphenated_tags_xml_self_closing_visitor_events() {
    #[derive(Debug, Default)]
    struct EventRecorder {
        events: Vec<String>,
    }

    impl HtmlVisitor for EventRecorder {
        fn visit_element_start(&mut self, ctx: &NodeContext) -> VisitResult {
            self.events.push(format!("start({})", ctx.tag_name));
            VisitResult::Continue
        }

        fn visit_element_end(&mut self, ctx: &NodeContext, _output: &str) -> VisitResult {
            self.events.push(format!("end({})", ctx.tag_name));
            VisitResult::Continue
        }
    }

    let html = r#"
<structured-macro>
  <ac:parameter ac:name="foo" />
  <ac:parameter ac:name="quux">lalaland</ac:parameter>
</structured-macro>
"#;

    let visitor = Rc::new(RefCell::new(EventRecorder::default()));
    let result = convert(html, None, Some(visitor.clone()));
    assert!(result.is_ok(), "conversion should succeed: {:?}", result.err());

    let events = visitor.borrow().events.clone();

    // Find the indices of start/end pairs for the two ac:parameter elements.
    // With correct XML self-closing handling:
    //   start(ac:parameter)[foo] → end(ac:parameter)[foo] → start(ac:parameter)[quux] → end(ac:parameter)[quux]
    // With the bug (html5ever treats `/>` as open tag):
    //   start(ac:parameter)[foo] → start(ac:parameter)[quux] → end(ac:parameter)[quux] → end(ac:parameter)[foo]

    // Collect positions of start/end events for ac:parameter
    let ac_param_starts: Vec<usize> = events
        .iter()
        .enumerate()
        .filter(|(_, e)| e.starts_with("start(ac:parameter)"))
        .map(|(i, _)| i)
        .collect();
    let ac_param_ends: Vec<usize> = events
        .iter()
        .enumerate()
        .filter(|(_, e)| e.starts_with("end(ac:parameter)"))
        .map(|(i, _)| i)
        .collect();

    assert_eq!(
        ac_param_starts.len(),
        2,
        "expected exactly 2 ac:parameter start events, got: {events:?}"
    );
    assert_eq!(
        ac_param_ends.len(),
        2,
        "expected exactly 2 ac:parameter end events, got: {events:?}"
    );

    // Each start must come before the corresponding end: start[0] < end[0] < start[1] < end[1]
    assert!(
        ac_param_starts[0] < ac_param_ends[0],
        "first ac:parameter: start must precede end (got start@{}, end@{}); events: {events:?}",
        ac_param_starts[0],
        ac_param_ends[0],
    );
    assert!(
        ac_param_ends[0] < ac_param_starts[1],
        "first ac:parameter end must precede second ac:parameter start (got end@{}, start@{}); events: {events:?}",
        ac_param_ends[0],
        ac_param_starts[1],
    );
    assert!(
        ac_param_starts[1] < ac_param_ends[1],
        "second ac:parameter: start must precede end (got start@{}, end@{}); events: {events:?}",
        ac_param_starts[1],
        ac_param_ends[1],
    );
}
