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

#[test]
fn heading_metadata_folds_actual_breaks_inside_and_outside_table_cells() {
    let mut actual = Vec::new();
    for html in [
        "<h2>Jane<br><sup>Doe</sup></h2>",
        "<table><tr><td><h2>Jane<br><sup>Doe</sup></h2></td></tr></table>",
    ] {
        let result = metadata_without_rendering_changes(html, options());
        assert!(
            result
                .content
                .as_deref()
                .expect("rendered output")
                .contains("Jane ^Doe^")
        );
        assert_eq!(result.metadata.headers.len(), 1);
        actual.push(result.metadata.headers[0].text.clone());
    }
    assert_eq!(actual, ["Jane Doe", "Jane Doe"]);
}

#[cfg(feature = "visitor")]
mod capture_transactions {
    use super::*;
    use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult, VisitorHandle};
    use std::sync::{Arc, Mutex};

    #[derive(Clone, Copy, Debug)]
    enum Action {
        Keep,
        Skip,
        Custom(&'static str),
    }

    impl Action {
        fn result(self) -> VisitResult {
            match self {
                Self::Keep => VisitResult::Continue,
                Self::Skip => VisitResult::Skip,
                Self::Custom(text) => VisitResult::Custom(text.to_string()),
            }
        }
    }

    #[derive(Debug)]
    struct EndVisitor {
        tag: &'static str,
        action: Action,
        outputs: Vec<String>,
    }

    impl HtmlVisitor for EndVisitor {
        fn visit_element_end(&mut self, context: &NodeContext<'_>, output: &str) -> VisitResult {
            if context.tag_name == self.tag {
                self.outputs.push(output.to_string());
                self.action.result()
            } else {
                VisitResult::Continue
            }
        }
    }

    fn end_conversion(html: &str, tag: &'static str, action: Action) -> (ConversionResult, Vec<String>) {
        let visitor = Arc::new(Mutex::new(EndVisitor {
            tag,
            action,
            outputs: Vec::new(),
        }));
        let handle: VisitorHandle = visitor.clone();
        let result = convert(
            html,
            Some(ConversionOptions {
                visitor: Some(handle),
                ..options()
            }),
        )
        .expect("the end-hook conversion must succeed");
        let outputs = visitor.lock().expect("end-hook state").outputs.clone();
        assert_eq!(outputs.len(), 1, "the selected real end hook must fire exactly once");
        assert!(result.document.is_none());
        (result, outputs)
    }

    const ANCESTOR: &str = "<h2>Keep</h2><div><h2>Jane <sup>Doe</sup></h2></div>";

    #[test]
    fn ancestor_end_skip_discards_metadata_of_the_already_rendered_child_heading() {
        let (result, observed) = end_conversion(ANCESTOR, "div", Action::Skip);
        assert!(observed[0].contains("## Jane ^Doe^"));
        assert_eq!(result.content.as_deref().map(str::trim), Some("## Keep"));
        assert_eq!(header_texts(&result), ["Keep"]);
    }

    #[test]
    fn ancestor_end_custom_discards_metadata_of_the_replaced_child_heading() {
        let (result, observed) = end_conversion(ANCESTOR, "div", Action::Custom("Replacement"));
        assert!(observed[0].contains("## Jane ^Doe^"));
        let rendered = result.content.as_deref().expect("rendered output");
        assert!(rendered.contains("## Keep"));
        assert!(rendered.contains("Replacement"));
        assert!(!rendered.contains("Jane"));
        assert!(!rendered.contains("Doe"));
        assert_eq!(header_texts(&result), ["Keep"]);
    }

    #[test]
    fn accepted_inline_end_custom_and_skip_capture_only_the_actual_replacement_text() {
        let html = "<h2>Jane <span><sup>Doe</sup></span></h2>";
        let mut actual = Vec::new();
        for (action, expected) in [(Action::Custom("Smith"), "## Jane Smith"), (Action::Skip, "## Jane")] {
            let (result, observed) = end_conversion(html, "span", action);
            assert_eq!(observed[0], "^Doe^");
            assert_eq!(result.content.as_deref().map(str::trim), Some(expected));
            assert_eq!(result.metadata.headers.len(), 1);
            actual.push(result.metadata.headers[0].text.clone());
        }
        assert_eq!(actual, ["Jane Smith", "Jane"]);
    }

    #[derive(Debug)]
    struct ImageVisitor {
        action: Action,
        calls: Vec<(String, String)>,
    }

    impl HtmlVisitor for ImageVisitor {
        fn visit_image(
            &mut self,
            _context: &NodeContext<'_>,
            src: &str,
            alt: &str,
            _title: Option<&str>,
        ) -> VisitResult {
            self.calls.push((src.to_string(), alt.to_string()));
            self.action.result()
        }
    }

    #[test]
    fn heading_image_metadata_uses_actual_visitor_admission_without_repeating_image_calls() {
        let mut actual = Vec::new();
        for (action, expected) in [
            (Action::Keep, "## Jane ![Doe](name.png)"),
            (Action::Custom("Smith"), "## Jane Smith"),
            (Action::Skip, "## Jane"),
        ] {
            let visitor = Arc::new(Mutex::new(ImageVisitor {
                action,
                calls: Vec::new(),
            }));
            let handle: VisitorHandle = visitor.clone();
            let result = convert(
                r#"<h2>Jane <img src="name.png" alt="Doe"></h2>"#,
                Some(ConversionOptions {
                    visitor: Some(handle),
                    keep_inline_images_in: vec!["h2".to_string()],
                    ..options()
                }),
            )
            .expect("the image-hook conversion must succeed");
            assert_eq!(
                visitor.lock().expect("image-hook state").calls,
                [("name.png".to_string(), "Doe".to_string())]
            );
            assert_eq!(result.content.as_deref().map(str::trim), Some(expected));
            assert!(result.document.is_none());
            assert_eq!(result.metadata.headers.len(), 1);
            actual.push(result.metadata.headers[0].text.clone());
        }
        assert_eq!(actual, ["Jane Doe", "Jane Smith", "Jane"]);
    }
}

#[cfg(feature = "visitor")]
mod specialized_table_transactions {
    use super::*;
    use html_to_markdown_rs::HiddenContent;
    use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult, VisitorHandle};
    use std::sync::{Arc, Mutex};

    #[derive(Debug)]
    struct TableEndVisitor {
        skip: bool,
        outputs: Vec<String>,
    }

    impl HtmlVisitor for TableEndVisitor {
        fn visit_table_end(&mut self, _context: &NodeContext<'_>, output: &str) -> VisitResult {
            self.outputs.push(output.to_string());
            if self.skip {
                VisitResult::Skip
            } else {
                VisitResult::Custom("Tail".to_string())
            }
        }
    }

    fn table_end_case(skip: bool) -> (ConversionResult, Vec<String>) {
        let visitor = Arc::new(Mutex::new(TableEndVisitor {
            skip,
            outputs: Vec::new(),
        }));
        let handle: VisitorHandle = visitor.clone();
        let result = convert(
            "<h2>Keep</h2><table><tr><td><h2>Jane <sup>Doe</sup></h2></td></tr></table>",
            Some(ConversionOptions {
                visitor: Some(handle),
                compact_tables: true,
                ..options()
            }),
        )
        .expect("the dedicated table-end route must succeed");
        let outputs = visitor.lock().expect("table-end state").outputs.clone();
        assert_eq!(outputs.len(), 1, "the real dedicated table-end hook must fire once");
        assert!(outputs[0].contains("Jane ^Doe^"));
        assert!(result.document.is_none());
        (result, outputs)
    }

    #[test]
    fn dedicated_table_end_skip_discards_the_already_captured_cell_heading() {
        let (result, _) = table_end_case(true);
        assert_eq!(result.content.as_deref().map(str::trim), Some("## Keep"));
        assert_eq!(header_texts(&result), ["Keep"]);
    }

    #[test]
    fn dedicated_table_end_custom_appends_output_and_retains_source_cell_headings() {
        let (result, _) = table_end_case(false);
        let output = result.content.as_deref().expect("rendered output");
        assert!(output.contains("## Keep"));
        assert!(output.contains("Jane ^Doe^"));
        assert!(output.trim_end().ends_with("Tail"));
        assert_eq!(header_texts(&result), ["Keep", "Jane Doe"]);
    }

    #[derive(Debug)]
    struct InertEndVisitor {
        tag: &'static str,
        skip: bool,
        outputs: Vec<String>,
    }

    impl HtmlVisitor for InertEndVisitor {
        fn visit_element_end(&mut self, context: &NodeContext<'_>, output: &str) -> VisitResult {
            if context.tag_name != self.tag {
                return VisitResult::Continue;
            }
            self.outputs.push(output.to_string());
            if self.skip {
                VisitResult::Skip
            } else {
                VisitResult::Custom("Replacement".to_string())
            }
        }
    }

    fn inert_end_case(tag: &'static str, skip: bool) -> ConversionResult {
        let html = format!(
            "<table><tr><th><h2>Before</h2></th></tr><{tag}><tr><td><h2>Jane <sup>Doe</sup></h2></td></tr></{tag}><tr><td><h2>After</h2></td></tr></table>"
        );
        let visitor = Arc::new(Mutex::new(InertEndVisitor {
            tag,
            skip,
            outputs: Vec::new(),
        }));
        let handle: VisitorHandle = visitor.clone();
        let result = convert(
            &html,
            Some(ConversionOptions {
                visitor: Some(handle),
                hidden_content: HiddenContent::All,
                compact_tables: true,
                ..options()
            }),
        )
        .expect("the kept inert-wrapper table route must succeed");
        let observed = &visitor.lock().expect("inert end state").outputs;
        assert_eq!(observed.len(), 1, "the actual kept wrapper end hook must fire once");
        assert!(observed[0].contains("Jane ^Doe^"));
        let output = result.content.as_deref().expect("rendered output");
        assert!(output.contains("Before"));
        assert!(output.contains("After"));
        assert!(output.find("Before").unwrap() < output.find("After").unwrap());
        assert!(!output.contains("Jane"));
        assert!(!output.contains("Doe"));
        assert_eq!(output.contains("Replacement"), !skip);
        assert!(result.document.is_none());
        result
    }

    #[test]
    fn kept_table_inert_end_replacement_discards_only_its_own_child_heading_metadata() {
        let mut actual = Vec::new();
        for tag in ["template", "noscript"] {
            for skip in [true, false] {
                let result = inert_end_case(tag, skip);
                actual.push(
                    result
                        .metadata
                        .headers
                        .iter()
                        .map(|header| header.text.clone())
                        .collect::<Vec<_>>(),
                );
            }
        }
        assert_eq!(actual, vec![vec!["Before".to_string(), "After".to_string()]; 4]);
    }

    #[derive(Debug, Default)]
    struct SkippedImageVisitor {
        calls: Vec<(String, String)>,
    }

    impl HtmlVisitor for SkippedImageVisitor {
        fn visit_image(
            &mut self,
            _context: &NodeContext<'_>,
            src: &str,
            alt: &str,
            _title: Option<&str>,
        ) -> VisitResult {
            self.calls.push((src.to_string(), alt.to_string()));
            VisitResult::Custom("Smith".to_string())
        }
    }

    #[test]
    fn skip_images_omits_custom_image_output_after_the_actual_callback() {
        let visitor = Arc::new(Mutex::new(SkippedImageVisitor::default()));
        let handle: VisitorHandle = visitor.clone();
        let result = convert(
            r#"<h2>Jane <img src="name.png" alt="Doe"></h2>"#,
            Some(ConversionOptions {
                visitor: Some(handle),
                skip_images: true,
                ..options()
            }),
        )
        .expect("the skipped-image callback route must succeed");
        assert_eq!(
            visitor.lock().expect("skipped-image state").calls,
            [("name.png".to_string(), "Doe".to_string())]
        );
        assert_eq!(result.content.as_deref().map(str::trim), Some("## Jane"));
        assert!(result.document.is_none());
        assert_eq!(header_texts(&result), ["Jane"]);
    }
}
