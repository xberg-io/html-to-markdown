#![allow(clippy::significant_drop_tightening)]

//! Test to reproduce issue #187: `visit_div` is not executed
//!
//! This test demonstrates that:
//! 1. There is NO `visit_div`, `visit_script`, `visit_style` method
//! 2. Instead, users should use `visit_element_start` with `tag_name` filtering
//! 3. The documentation incorrectly shows these non-existent methods

#![cfg(feature = "visitor")]

use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult, VisitorHandle};
use html_to_markdown_rs::{ConversionError, ConversionOptions, ConversionResult};
use std::sync::{Arc, Mutex};

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

/// This is what the documentation SHOWS (but doesn't work)
/// The methods `visit_div`, `visit_script`, `visit_style` DO NOT EXIST
#[allow(dead_code)]
#[derive(Debug, Default)]
struct DocumentedButBrokenVisitor {
    skipped_elements: Vec<(String, String)>,
}

// NOTE: These methods will NEVER be called because they don't exist in HtmlVisitor trait!
// impl DocumentedButBrokenVisitor {
//     fn visit_div(&mut self, ctx: &NodeContext, _content: &str) -> VisitResult {
//         // This will NEVER be called!
//         let classes = ctx.attributes.get("class").map(|s| s.as_str()).unwrap_or("");
//         if classes.contains("ad") || classes.contains("advertisement") {
//             self.skipped_elements.push(("div".to_string(), classes.to_string()));
//             return VisitResult::Skip;
//         }
//         VisitResult::Continue
//     }
//
//     fn visit_script(&mut self, ctx: &NodeContext) -> VisitResult {
//         // This will NEVER be called!
//         self.skipped_elements.push(("script".to_string(), "".to_string()));
//         VisitResult::Skip
//     }
// }

/// This is the CORRECT way to filter divs, scripts, and styles
#[derive(Debug, Default)]
struct ContentFilter {
    skipped_elements: Vec<(String, String)>,
}

impl HtmlVisitor for ContentFilter {
    /// Use `visit_element_start` to filter ANY element by tag name
    fn visit_element_start(&mut self, ctx: &NodeContext<'_>) -> VisitResult {
        let tag_name: &str = ctx.tag_name.as_ref();

        match tag_name {
            "div" => {
                // Filter divs with unwanted classes
                let classes = ctx.attributes().get("class").map_or("", std::string::String::as_str);
                if classes.contains("ad")
                    || classes.contains("advertisement")
                    || classes.contains("tracking")
                    || classes.contains("analytics")
                {
                    self.skipped_elements.push(("div".to_string(), classes.to_string()));
                    return VisitResult::Skip;
                }
            }
            "script" => {
                // Always remove script tags
                self.skipped_elements.push(("script".to_string(), String::new()));
                return VisitResult::Skip;
            }
            "style" => {
                // Always remove style tags
                self.skipped_elements.push(("style".to_string(), String::new()));
                return VisitResult::Skip;
            }
            _ => {}
        }

        VisitResult::Continue
    }

    /// Still use specific methods for links and images
    fn visit_image(&mut self, ctx: &NodeContext, src: &str, _alt: &str, _title: Option<&str>) -> VisitResult {
        // Remove tracking pixels (1x1 images)
        let width = ctx.attributes().get("width").map_or("", std::string::String::as_str);
        let height = ctx.attributes().get("height").map_or("", std::string::String::as_str);

        if width == "1" && height == "1" {
            self.skipped_elements
                .push(("img".to_string(), format!("tracking pixel: {src}")));
            return VisitResult::Skip;
        }

        // Skip images with "tracking" or "analytics" in the URL
        if src.to_lowercase().contains("tracking") || src.to_lowercase().contains("analytics") {
            self.skipped_elements
                .push(("img".to_string(), format!("tracking URL: {src}")));
            return VisitResult::Skip;
        }

        VisitResult::Continue
    }

    fn visit_link(&mut self, _ctx: &NodeContext, href: &str, text: &str, _title: Option<&str>) -> VisitResult {
        // Remove links with utm_* tracking parameters
        if href.to_lowercase().contains("utm_") {
            // Strip tracking params but keep the link
            if let Some(base_url) = href.split('?').next() {
                return VisitResult::Custom(format!("[{text}]({base_url})"));
            }
        }

        VisitResult::Continue
    }
}

#[test]
fn test_issue_187_content_filter() {
    let html = r#"
    <article>
        <h1>Blog Post Title</h1>
        <p>This is the main content of the article.</p>

        <div class="ad advertisement">
            <p>This is an advertisement block that should be removed.</p>
        </div>

        <p>More content here.</p>

        <img src="https://tracking.example.com/pixel.gif" width="1" height="1" alt="">

        <div class="content">
            <p>Legitimate content in a div.</p>
            <img src="https://cdn.example.com/image.jpg" alt="Article image" width="800">
        </div>

        <script>
            console.log("This script should be removed");
        </script>

        <p>Read more on <a href="https://example.com/article?utm_source=newsletter&utm_medium=email">our website</a>.</p>

        <div class="tracking analytics">
            <img src="https://analytics.example.com/track.png" alt="">
        </div>
    </article>
    "#;

    let visitor = Arc::new(Mutex::new(ContentFilter::default()));
    let result = convert(html, None, Some(visitor.clone()))
        .unwrap()
        .content
        .unwrap_or_default();

    println!("Converted Markdown:\n{result}");
    println!("\nSkipped Elements:");
    for (tag, info) in &visitor.lock().expect("visitor mutex poisoned").skipped_elements {
        println!("- {tag}: {info}");
    }

    // Verify that unwanted content was filtered out
    assert!(
        !result.contains("advertisement block that should be removed"),
        "Ad div should be removed"
    );
    assert!(!result.contains("console.log"), "Script tag should be removed");

    // Verify that legitimate content remains
    assert!(result.contains("Blog Post Title"), "Heading should be preserved");
    assert!(result.contains("main content"), "Main content should be preserved");
    assert!(result.contains("Legitimate content"), "Content div should be preserved");
    assert!(result.contains("Article image"), "Legitimate image should be preserved");

    // Verify tracking parameters were stripped from link
    assert!(
        result.contains("[our website](https://example.com/article)"),
        "Link tracking params should be stripped"
    );

    // Verify skipped elements were tracked
    let borrowed = visitor.lock().expect("visitor mutex poisoned");
    assert!(
        borrowed.skipped_elements.iter().any(|(tag, _)| tag == "div"),
        "Should have skipped ad divs"
    );
    // Note: script and style tags are stripped during preprocessing before the visitor sees them,
    // so they won't appear in skipped_elements. Only the visitor can control div, img, etc.
    assert!(
        borrowed.skipped_elements.iter().any(|(tag, _)| tag == "img"),
        "Should have skipped tracking pixel"
    );
}
