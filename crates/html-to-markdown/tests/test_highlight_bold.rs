use html_to_markdown_rs::{ConversionOptions, HighlightStyle};

#[test]
fn test_highlight_bold_rendering() {
    let options = ConversionOptions::builder()
        .highlight_style(HighlightStyle::Bold)
        .build();

    let result =
        html_to_markdown_rs::convert("<p>Text with <mark>highlighted</mark> text.</p>", Some(options)).unwrap();

    let content = result.content.unwrap_or_default();
    assert!(
        content.contains("**highlighted**"),
        "Expected **highlighted** in output but got: {content}"
    );
}

#[test]
fn test_highlight_bold_from_json() {
    let options = serde_json::from_str::<ConversionOptions>(r#"{"highlight_style":"bold"}"#).unwrap();
    assert_eq!(options.highlight_style, HighlightStyle::Bold);

    let result =
        html_to_markdown_rs::convert("<p>Text with <mark>highlighted</mark> text.</p>", Some(options)).unwrap();

    let content = result.content.unwrap_or_default();
    assert!(
        content.contains("**highlighted**"),
        "Expected **highlighted** in output but got: {content}"
    );
}
