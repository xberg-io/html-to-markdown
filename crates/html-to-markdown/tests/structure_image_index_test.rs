//! An image node in the document structure points at the bytes extracted for it.
//!
//! `NodeContent::Image::image_index` is documented as the index into
//! `ConversionResult::images` when image extraction is on. A consumer of the structure needs it
//! to put an extracted image where it stands in the document rather than after everything else.

#![allow(missing_docs)]
#![cfg(feature = "inline-images")]

use html_to_markdown_rs::{ConversionOptions, ConversionResult, NodeContent, convert};

const PNG: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";

fn structure_of(html: &str, extract_images: bool) -> ConversionResult {
    convert(
        html,
        Some(ConversionOptions {
            include_document_structure: true,
            extract_images,
            ..ConversionOptions::default()
        }),
    )
    .expect("conversion should succeed")
}

fn image_indices(result: &ConversionResult) -> Vec<Option<u32>> {
    let document = result.document.as_ref().expect("document structure was requested");
    document
        .nodes
        .iter()
        .filter_map(|node| match &node.content {
            NodeContent::Image { image_index, .. } => Some(*image_index),
            _ => None,
        })
        .collect()
}

#[test]
fn should_point_an_inline_image_node_at_its_extracted_bytes() {
    let html = format!(r#"<p>Before</p><p><img src="data:image/png;base64,{PNG}" alt="dot"></p><p>After</p>"#);
    let result = structure_of(&html, true);

    assert_eq!(result.images.len(), 1);
    assert_eq!(result.images[0].description.as_deref(), Some("dot"));
    assert_eq!(image_indices(&result), vec![Some(0)]);
}

#[test]
fn should_index_only_the_images_whose_bytes_were_extracted() {
    // A remote address has no bytes to extract, and a payload that does not decode is skipped,
    // so the second data image is the second entry in `images`, not the fourth.
    let html = format!(
        concat!(
            r#"<p><img src="https://example.com/a.png" alt="remote"></p>"#,
            r#"<p><img src="data:image/png;base64,{png}" alt="first"></p>"#,
            r#"<p><img src="data:image/png;base64,!!!not-base64!!!" alt="broken"></p>"#,
            r#"<p><img src="data:image/png;base64,{png}" alt="second"></p>"#,
        ),
        png = PNG
    );
    let result = structure_of(&html, true);

    let descriptions: Vec<_> = result.images.iter().map(|image| image.description.as_deref()).collect();
    assert_eq!(descriptions, vec![Some("first"), Some("second")]);
    assert_eq!(image_indices(&result), vec![None, Some(0), None, Some(1)]);
}

#[test]
fn should_leave_the_index_empty_when_images_are_not_extracted() {
    let html = format!(r#"<p><img src="data:image/png;base64,{PNG}" alt="dot"></p>"#);
    let result = structure_of(&html, false);

    assert!(result.images.is_empty());
    assert_eq!(image_indices(&result), vec![None]);
}
