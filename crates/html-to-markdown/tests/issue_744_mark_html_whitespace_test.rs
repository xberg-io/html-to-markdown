#![allow(missing_docs)]
#![cfg(feature = "testkit")]

use html_to_markdown_rs::{ConversionOptions, HighlightStyle, convert};

fn convert_with_style(html: &str, highlight_style: HighlightStyle) -> String {
    convert(
        html,
        Some(ConversionOptions {
            extract_metadata: false,
            highlight_style,
            ..ConversionOptions::default()
        }),
    )
    .expect("conversion should succeed")
    .content
    .unwrap_or_default()
}

#[test]
fn should_keep_edge_spaces_inside_html_mark_in_paragraphs() {
    assert_eq!(
        convert_with_style("<p><mark>a </mark>b</p>", HighlightStyle::Html),
        "<mark>a </mark>b\n"
    );
    assert_eq!(
        convert_with_style("<p>a<mark> b</mark></p>", HighlightStyle::Html),
        "a<mark> b</mark>\n"
    );
}

#[test]
fn should_keep_edge_spaces_inside_html_mark_in_table_cells() {
    assert_eq!(
        convert_with_style(
            "<table><tr><td><mark>a </mark>b</td><td>x</td></tr></table>",
            HighlightStyle::Html,
        ),
        "| <mark>a </mark>b | x |\n| ---------------- | --- |\n"
    );
    assert_eq!(
        convert_with_style(
            "<table><tr><td>a<mark> b</mark></td><td>x</td></tr></table>",
            HighlightStyle::Html,
        ),
        "| a<mark> b</mark> | x |\n| ---------------- | --- |\n"
    );
}

#[test]
fn should_keep_edge_spaces_outside_markdown_highlight_delimiters() {
    assert_eq!(
        convert_with_style("<p><mark>a </mark>b</p>", HighlightStyle::DoubleEqual),
        "==a== b\n"
    );
    assert_eq!(
        convert_with_style("<p>a<mark> b</mark></p>", HighlightStyle::Bold),
        "a **b**\n"
    );
}
