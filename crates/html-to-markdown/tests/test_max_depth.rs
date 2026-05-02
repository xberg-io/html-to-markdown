#![allow(missing_docs)]

//! Tests for the `max_depth` recursion-safety option.

use html_to_markdown_rs::ConversionOptions;

fn convert_with_options(html: &str, options: ConversionOptions) -> String {
    html_to_markdown_rs::convert(html, Some(options))
        .expect("conversion should not fail")
        .content
        .unwrap_or_default()
}

/// With the default `max_depth: None`, deeply nested content should be fully converted.
#[test]
fn test_max_depth_none_converts_deeply_nested() {
    // Build 100 levels of nesting around a leaf text node.
    let mut html = String::from("<p>deep</p>");
    for _ in 0..100 {
        html = format!("<div>{html}</div>");
    }

    let options = ConversionOptions {
        extract_metadata: false,
        max_depth: None,
        ..Default::default()
    };

    let result = convert_with_options(&html, options);
    assert!(
        result.contains("deep"),
        "Deeply nested text should be present when max_depth is None. Got:\n{result}"
    );
}

/// With `max_depth: Some(2)`, block elements at depth 2 are not visited, so
/// their text content is excluded from the output.
#[test]
fn test_max_depth_truncates_at_limit() {
    // Depth counting (each handler passes depth+1 to its children):
    // depth 0: outer <div>  — visited
    //   depth 1: <p>        — visited, paragraph handler passes depth+1 to children
    //     depth 2: "shallow" — visited (2 < 3), appears in output
    //   depth 1: inner <div> — visited, div handler passes depth+1 to children
    //     depth 2: <p>       — visited, paragraph handler passes depth+1 to children
    //       depth 3: "deep"  — skipped (3 >= 3), absent from output
    let html = "<div><p>shallow</p><div><p>deep</p></div></div>";

    let options = ConversionOptions {
        extract_metadata: false,
        max_depth: Some(3),
        ..Default::default()
    };

    let result = convert_with_options(html, options);
    assert!(
        result.contains("shallow"),
        "Content at depth < max_depth should be present. Got:\n{result}"
    );
    assert!(
        !result.contains("deep"),
        "Content at depth >= max_depth should be absent. Got:\n{result}"
    );
}

/// With `max_depth: Some(0)`, no nodes are processed and the output is empty or whitespace only.
#[test]
fn test_max_depth_zero_produces_empty() {
    let html = "<p>hello</p>";

    let options = ConversionOptions {
        extract_metadata: false,
        max_depth: Some(0),
        ..Default::default()
    };

    let result = convert_with_options(html, options);
    assert!(
        result.trim().is_empty(),
        "max_depth: Some(0) should produce no output. Got:\n{result}"
    );
}
