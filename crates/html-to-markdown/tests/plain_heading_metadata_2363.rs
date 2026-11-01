#![cfg(feature = "metadata")]

use html_to_markdown_rs::{ConversionOptions, ConversionResult, HeadingStyle, OutputFormat, TierStrategy, convert};

fn options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: true,
        tier_strategy: TierStrategy::Tier2,
        sup_symbol: "^".to_string(),
        ..ConversionOptions::default()
    }
}

fn metadata_without_rendering_changes(html: &str, options: ConversionOptions) -> ConversionResult {
    let control = convert(
        html,
        Some(ConversionOptions {
            extract_metadata: false,
            ..options.clone()
        }),
    )
    .expect("the converter without metadata must succeed");
    let result = convert(html, Some(options.clone())).expect("the metadata converter must succeed");
    assert_eq!(
        result.content, control.content,
        "metadata must not change rendered output"
    );
    assert_eq!(result.document.is_some(), options.include_document_structure);
    assert!(control.metadata.headers.is_empty());
    assert!(
        !result.metadata.headers.is_empty(),
        "the real heading producer must run"
    );
    result
}

fn header_texts(result: &ConversionResult) -> Vec<&str> {
    result
        .metadata
        .headers
        .iter()
        .map(|header| header.text.as_str())
        .collect()
}

#[test]
fn semantic_superscript_has_plain_heading_metadata_and_unchanged_markdown() {
    let result = metadata_without_rendering_changes(r#"<h1 id="person">Jane <sup>Doe</sup></h1>"#, options());
    assert_eq!(result.content.as_deref().map(str::trim), Some("# Jane ^Doe^"));
    assert_eq!(result.metadata.headers.len(), 1);
    assert_eq!(result.metadata.headers[0].level, 1);
    assert_eq!(result.metadata.headers[0].id.as_deref(), Some("person"));
    assert_eq!(header_texts(&result), ["Jane Doe"]);
}

#[test]
fn literal_carets_in_heading_metadata_are_source_text() {
    let result = metadata_without_rendering_changes("<h2>Jane ^Doe^</h2>", options());
    assert!(result.content.as_deref().expect("rendered output").contains("Doe"));
    assert_eq!(header_texts(&result), ["Jane ^Doe^"]);
}

#[test]
fn heading_metadata_is_plain_across_output_formats_and_heading_styles() {
    let mut actual = Vec::new();
    for output_format in [OutputFormat::Markdown, OutputFormat::Djot] {
        for heading_style in [HeadingStyle::Atx, HeadingStyle::AtxClosed, HeadingStyle::Underlined] {
            let result = metadata_without_rendering_changes(
                "<h2>Jane <strong><sup>Doe</sup></strong></h2>",
                ConversionOptions {
                    output_format,
                    heading_style,
                    ..options()
                },
            );
            assert_eq!(result.metadata.headers.len(), 1);
            actual.push(result.metadata.headers[0].text.clone());
        }
    }
    assert_eq!(actual, ["Jane Doe"; 6]);
}

#[test]
fn heading_metadata_obeys_actual_exclusions_hidden_nodes_and_entity_decoding() {
    let html = concat!(
        "<h2>Jane &amp; ",
        r#"<span class="omit"><strong>Excluded </strong></span>"#,
        "<span hidden>Hidden </span><sup>Doe</sup></h2>",
    );
    let result = metadata_without_rendering_changes(
        html,
        ConversionOptions {
            exclude_selectors: vec![".omit".to_string()],
            ..options()
        },
    );
    let content = result.content.as_deref().expect("rendered output");
    assert!(!content.contains("Excluded"));
    assert!(!content.contains("Hidden"));
    assert!(content.contains("Jane &"));
    assert!(content.contains("^Doe^"));
    assert_eq!(header_texts(&result), ["Jane & Doe"]);
}

#[test]
fn heading_metadata_keeps_inline_image_alt_text_without_image_markup() {
    let result = metadata_without_rendering_changes(
        r#"<h2>Jane <img src="name.png" alt="Doe"></h2>"#,
        ConversionOptions {
            keep_inline_images_in: vec!["h2".to_string()],
            ..options()
        },
    );
    assert!(
        result
            .content
            .as_deref()
            .expect("rendered output")
            .contains("![Doe](name.png)")
    );
    assert_eq!(header_texts(&result), ["Jane Doe"]);
}

#[test]
fn heading_metadata_omits_skipped_images_and_preserves_visible_text() {
    let result = metadata_without_rendering_changes(
        r#"<h2>Jane<img src="name.png" alt="Excluded"> Doe</h2>"#,
        ConversionOptions {
            skip_images: true,
            ..options()
        },
    );
    let content = result.content.as_deref().expect("rendered output");
    assert!(!content.contains("Excluded"));
    assert!(!content.contains("name.png"));
    assert_eq!(header_texts(&result), ["Jane Doe"]);
}

#[test]
fn repeated_heading_attributes_do_not_merge_distinct_source_headings() {
    for include_document_structure in [false, true] {
        let result = metadata_without_rendering_changes(
            r#"<h2 id="same">Jane <sup>Doe</sup></h2><h2 id="same">Jane ^Doe^</h2>"#,
            ConversionOptions {
                include_document_structure,
                ..options()
            },
        );
        assert_eq!(result.metadata.headers.len(), 2);
        assert!(
            result
                .metadata
                .headers
                .iter()
                .all(|header| header.id.as_deref() == Some("same"))
        );
        assert_eq!(header_texts(&result), ["Jane Doe", "Jane ^Doe^"]);
    }
}

#[test]
fn heading_metadata_in_a_table_cell_uses_its_own_source_text() {
    let result = metadata_without_rendering_changes(
        "<table><tr><td><h2>Jane <sup>Doe</sup></h2></td></tr></table>",
        options(),
    );
    assert!(result.content.as_deref().expect("rendered output").contains("^Doe^"));
    assert_eq!(header_texts(&result), ["Jane Doe"]);
}

#[cfg(feature = "visitor")]
mod visitor_capture {
    use super::*;
    use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult, VisitorHandle};
    use std::sync::{Arc, Mutex};

    #[derive(Debug, Default)]
    struct ReplacementVisitor {
        text_calls: Vec<String>,
    }

    impl HtmlVisitor for ReplacementVisitor {
        fn visit_text(&mut self, _context: &NodeContext<'_>, text: &str) -> VisitResult {
            self.text_calls.push(text.to_string());
            if text == "Doe" {
                VisitResult::Custom("Smith".to_string())
            } else {
                VisitResult::Continue
            }
        }
    }

    #[test]
    fn heading_metadata_ignores_unaccepted_custom_text_without_reinvoking_the_visitor() {
        let visitor = Arc::new(Mutex::new(ReplacementVisitor::default()));
        let handle: VisitorHandle = visitor.clone();
        let result = convert(
            "<h2>Jane <sup>Doe</sup></h2>",
            Some(ConversionOptions {
                visitor: Some(handle),
                ..options()
            }),
        )
        .expect("the visitor conversion must succeed");
        assert_eq!(visitor.lock().expect("visitor state").text_calls, ["Jane ", "Doe"]);
        assert_eq!(result.content.as_deref().map(str::trim), Some("## Jane ^Doe^"));
        assert!(result.document.is_none());
        assert_eq!(header_texts(&result), ["Jane Doe"]);
    }
}
