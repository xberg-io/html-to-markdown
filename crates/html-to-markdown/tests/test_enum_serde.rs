use html_to_markdown_rs::options::ConversionOptions;

#[test]
fn test_deserialize_highlight_bold_lowercase() {
    let json = r#"{"highlight_style":"bold"}"#;
    let result = serde_json::from_str::<ConversionOptions>(json);
    assert!(result.is_ok(), "Failed to deserialize: {:?}", result.err());
    if let Ok(opts) = result {
        assert_eq!(opts.highlight_style, html_to_markdown_rs::HighlightStyle::Bold);
    }
}
