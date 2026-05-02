//! Regression coverage for issue #199.

fn convert(
    html: &str,
    opts: Option<html_to_markdown_rs::ConversionOptions>,
) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

#[test]
fn test_link_label_is_not_truncated() {
    let label = "a".repeat(600);
    let html = format!(r#"<a href="https://example.com">{label}</a>"#);

    let markdown = convert(&html, None).expect("conversion should succeed");
    let expected = format!("[{label}](https://example.com)");

    assert!(markdown.contains(&expected));
    assert!(!markdown.contains('…'));
}
