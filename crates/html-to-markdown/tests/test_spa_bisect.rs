#![allow(missing_docs)]

fn convert(
    html: &str,
    opts: Option<html_to_markdown_rs::ConversionOptions>,
) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

use std::fs;
use std::path::PathBuf;

use html_to_markdown_rs::ConversionOptions;

fn fixture_path(name: &str) -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "../../test_documents/html/issues", name]
        .iter()
        .collect()
}

#[test]
fn test_spa_first_half() {
    let html = fs::read_to_string(fixture_path("gh-121-minimal-failing.html")).expect("read html");

    let opts = ConversionOptions {
        extract_metadata: false,
        autolinks: false,
        ..Default::default()
    };

    let result = convert(&html, Some(opts)).unwrap();
    eprintln!("Result length: {}", result.len());
    assert!(!result.is_empty());
}
