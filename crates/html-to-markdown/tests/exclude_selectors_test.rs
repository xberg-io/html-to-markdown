#![allow(missing_docs)]

use html_to_markdown_rs::ConversionOptions;

fn convert(html: &str, opts: Option<ConversionOptions>) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

#[test]
fn test_exclude_selectors_drops_matching_elements() {
    let html = r#"<body>
        <div class="cookie-banner">Accept cookies</div>
        <article><p>Main content here.</p></article>
        <div id="ad-container">Buy stuff</div>
    </body>"#;

    let options = ConversionOptions {
        exclude_selectors: vec![".cookie-banner".to_string(), "#ad-container".to_string()],
        ..Default::default()
    };

    let result = convert(html, Some(options)).unwrap();

    assert!(result.contains("Main content"), "Should keep main content");
    assert!(!result.contains("cookie"), "Should drop .cookie-banner element");
    assert!(!result.contains("Buy stuff"), "Should drop #ad-container element");
}

#[test]
fn test_exclude_selectors_drops_nested_content() {
    let html = r#"<body>
        <aside class="sidebar">
            <h2>Related articles</h2>
            <p>Some sidebar content</p>
        </aside>
        <main><p>Primary content.</p></main>
    </body>"#;

    let options = ConversionOptions {
        exclude_selectors: vec![".sidebar".to_string()],
        ..Default::default()
    };

    let result = convert(html, Some(options)).unwrap();

    assert!(result.contains("Primary content"), "Should keep main content");
    assert!(
        !result.contains("Related articles"),
        "Should drop heading inside excluded element"
    );
    assert!(
        !result.contains("sidebar content"),
        "Should drop paragraph inside excluded element"
    );
}

#[test]
fn test_exclude_selectors_empty_list_is_noop() {
    let html = r"<body><p>Hello world</p></body>";

    let options = ConversionOptions {
        exclude_selectors: vec![],
        ..Default::default()
    };

    let result = convert(html, Some(options)).unwrap();
    assert!(
        result.contains("Hello world"),
        "Empty exclude_selectors should not affect output"
    );
}

#[test]
fn test_exclude_selectors_invalid_selector_is_skipped() {
    let html = r"<body><p>Visible text</p></body>";

    // An empty string or garbled selector should not panic or error — just be ignored.
    let options = ConversionOptions {
        exclude_selectors: vec![String::new(), "p".to_string()],
        ..Default::default()
    };

    // Should not return an error; whether the paragraph is excluded depends on the
    // selector, but it must not panic.
    let _ = convert(html, Some(options));
}

#[test]
fn test_exclude_selectors_attribute_selector() {
    let html = r#"<body>
        <div role="complementary">Sidebar</div>
        <p>Main text</p>
    </body>"#;

    let options = ConversionOptions {
        exclude_selectors: vec!["[role='complementary']".to_string()],
        ..Default::default()
    };

    let result = convert(html, Some(options)).unwrap();

    assert!(result.contains("Main text"), "Should keep non-excluded content");
    assert!(
        !result.contains("Sidebar"),
        "Should drop element matching attribute selector"
    );
}

#[test]
fn test_exclude_selectors_plain_text_output() {
    let html = r#"<body>
        <div class="nav">Navigation links</div>
        <p>Article body text.</p>
    </body>"#;

    let options = ConversionOptions {
        exclude_selectors: vec![".nav".to_string()],
        output_format: html_to_markdown_rs::OutputFormat::Plain,
        ..Default::default()
    };

    let result = convert(html, Some(options)).unwrap();

    assert!(
        result.contains("Article body text"),
        "Should keep body text in plain output"
    );
    assert!(
        !result.contains("Navigation links"),
        "Should drop excluded element in plain output"
    );
}
