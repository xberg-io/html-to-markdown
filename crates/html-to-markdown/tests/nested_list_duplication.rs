#![allow(missing_docs)]

//! Nested `ul > li > ul > li > ol` HTML must not produce duplicated content
//! in the Markdown output or in the document structure collector.

use html_to_markdown_rs::{ConversionOptions, NodeContent};

const NESTED_LIST_HTML: &str = "<ul><li>outer<ul><li>mid<ol><li>inner1</li><li>inner2</li></ol></li></ul></li></ul>";

#[test]
fn markdown_output_no_content_duplication() {
    let result = html_to_markdown_rs::convert(NESTED_LIST_HTML, None).unwrap();
    let content = result.content.unwrap_or_default();
    assert_eq!(
        content.matches("inner1").count(),
        1,
        "inner1 must appear exactly once:\n{content}"
    );
    assert_eq!(
        content.matches("inner2").count(),
        1,
        "inner2 must appear exactly once:\n{content}"
    );
}

#[test]
fn structure_collector_list_items_no_duplication() {
    let options = ConversionOptions {
        include_document_structure: true,
        ..Default::default()
    };
    let result = html_to_markdown_rs::convert(NESTED_LIST_HTML, Some(options)).unwrap();
    let doc = result.document.expect("document structure must be populated");

    let list_item_texts: Vec<&str> = doc
        .nodes
        .iter()
        .filter_map(|node| {
            if let NodeContent::ListItem { text } = &node.content {
                Some(text.as_str())
            } else {
                None
            }
        })
        .collect();

    let inner1_count = list_item_texts.iter().filter(|t| t.contains("inner1")).count();
    let inner2_count = list_item_texts.iter().filter(|t| t.contains("inner2")).count();
    assert_eq!(
        inner1_count, 1,
        "inner1 must appear in exactly one ListItem text; got items: {list_item_texts:?}"
    );
    assert_eq!(
        inner2_count, 1,
        "inner2 must appear in exactly one ListItem text; got items: {list_item_texts:?}"
    );
}
