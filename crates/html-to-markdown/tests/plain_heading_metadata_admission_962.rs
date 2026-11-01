//! Heading metadata includes only admitted labels and shares rendering boundaries.

#![cfg(feature = "metadata")]

use html_to_markdown_rs::options::WhitespaceMode;
use html_to_markdown_rs::{ConversionOptions, ConversionResult, TierStrategy, convert};

fn options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: true,
        tier_strategy: TierStrategy::Tier2,
        ..ConversionOptions::default()
    }
}

fn unchanged(html: &str, options: &ConversionOptions) -> ConversionResult {
    let control = convert(
        html,
        Some(ConversionOptions {
            extract_metadata: false,
            ..options.clone()
        }),
    )
    .expect("the actual converter without metadata must admit the source");
    assert!(control.metadata.headers.is_empty());
    let result = convert(html, Some(options.clone())).expect("metadata conversion must succeed");
    assert_eq!(
        result.content, control.content,
        "metadata must not alter rendered bytes"
    );
    assert!(result.document.is_none());
    result
}

fn heading(result: &ConversionResult, expected: &str) {
    assert_eq!(result.metadata.headers.len(), 1, "one actual heading must be collected");
    assert_eq!(result.metadata.headers[0].level, 2);
    assert_eq!(result.metadata.headers[0].text, expected);
}

#[test]
fn heading_metadata_includes_the_actual_svg_title_label() {
    let result = unchanged("<h2>Head <svg><title>T</title></svg></h2>", &options());
    assert_eq!(result.content.as_deref().map(str::trim), Some("## Head T"));
    heading(&result, "Head T");
}

#[test]
fn svg_only_heading_metadata_keeps_the_admitted_label() {
    let result = unchanged("<h2><svg><title>T</title></svg></h2>", &options());
    assert_eq!(result.content.as_deref().map(str::trim), Some("## T"));
    heading(&result, "T");
}

#[test]
fn heading_metadata_omits_svg_labels_when_images_are_skipped() {
    let result = unchanged(
        "<h2>Head <svg><title>Excluded</title></svg></h2>",
        &ConversionOptions {
            skip_images: true,
            ..options()
        },
    );
    assert_eq!(result.content.as_deref().map(str::trim), Some("## Head"));
    heading(&result, "Head");
}

#[test]
fn heading_metadata_keeps_actual_image_boundary_spaces_without_source_whitespace() {
    let result = unchanged(r#"<h2>hot<img src="i.png" alt="pic">end</h2>"#, &options());
    assert_eq!(result.content.as_deref().map(str::trim), Some("## hot pic end"));
    heading(&result, "hot pic end");
}

#[test]
fn strict_heading_metadata_folds_the_actual_source_newline() {
    let result = unchanged(
        "<h2>Jane\nDoe</h2>",
        &ConversionOptions {
            whitespace_mode: WhitespaceMode::Strict,
            ..options()
        },
    );
    assert_eq!(result.content.as_deref().map(str::trim), Some("## Jane Doe"));
    heading(&result, "Jane Doe");
}

#[test]
fn heading_metadata_removes_the_space_trimmed_at_an_excluded_span() {
    let result = unchanged(
        r#"<h2>Jane <span class="omit">Excluded</span> Doe</h2>"#,
        &ConversionOptions {
            exclude_selectors: vec![".omit".to_string()],
            ..options()
        },
    );
    assert_eq!(result.content.as_deref().map(str::trim), Some("## Jane Doe"));
    assert!(!result.content.as_deref().unwrap().contains("Excluded"));
    heading(&result, "Jane Doe");
}

#[test]
fn heading_metadata_keeps_double_spaces_inside_actual_inline_code() {
    let result = unchanged("<h2>Jane <code>a  b</code></h2>", &options());
    assert_eq!(result.content.as_deref().map(str::trim), Some("## Jane `a  b`"));
    heading(&result, "Jane a  b");
}

#[test]
fn heading_metadata_includes_descendant_text_of_an_actual_preserved_span() {
    let result = unchanged(
        "<h2>Jane <span>Doe</span></h2>",
        &ConversionOptions {
            preserve_tags: vec!["span".to_string()],
            ..options()
        },
    );
    assert_eq!(
        result.content.as_deref().map(str::trim),
        Some("## Jane <span>Doe</span>")
    );
    heading(&result, "Jane Doe");
}

#[cfg(feature = "visitor")]
mod callbacks {
    use super::*;
    use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult, VisitorHandle};
    use std::sync::{Arc, Mutex};

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum Kind {
        Break,
        Form,
        Button,
    }

    #[derive(Debug)]
    struct Replacement {
        kind: Kind,
        calls: Vec<(String, Option<String>)>,
        original_button: Vec<String>,
    }

    impl Replacement {
        fn visit(&mut self, kind: Kind, context: &NodeContext<'_>) -> VisitResult {
            assert_eq!(kind, self.kind, "the actual specialized callback must be reached");
            self.calls.push((
                context.tag_name.to_string(),
                context.parent_tag.as_deref().map(str::to_string),
            ));
            VisitResult::Custom("Doe".to_string())
        }
    }

    impl HtmlVisitor for Replacement {
        fn visit_line_break(&mut self, context: &NodeContext<'_>) -> VisitResult {
            self.visit(Kind::Break, context)
        }

        fn visit_form(
            &mut self,
            context: &NodeContext<'_>,
            _action: Option<&str>,
            _method: Option<&str>,
        ) -> VisitResult {
            self.visit(Kind::Form, context)
        }

        fn visit_button(&mut self, context: &NodeContext<'_>, text: &str) -> VisitResult {
            self.original_button.push(text.to_string());
            self.visit(Kind::Button, context)
        }
    }

    fn replacement(html: &str, kind: Kind, metadata: bool, tag: &str) -> ConversionResult {
        let visitor = Arc::new(Mutex::new(Replacement {
            kind,
            calls: Vec::new(),
            original_button: Vec::new(),
        }));
        let handle: VisitorHandle = visitor.clone();
        let result = convert(
            html,
            Some(ConversionOptions {
                extract_metadata: metadata,
                visitor: Some(handle),
                ..options()
            }),
        )
        .expect("the actual specialized Custom callback conversion must succeed");
        let receipt = visitor.lock().expect("actual callback receipt");
        assert_eq!(receipt.calls, [(tag.to_string(), Some("h2".to_string()))]);
        if kind == Kind::Button {
            assert_eq!(receipt.original_button, ["Original"]);
        } else {
            assert_eq!(receipt.original_button, Vec::<String>::new());
        }
        drop(receipt);
        assert!(result.document.is_none());
        assert_eq!(result.content.as_deref().map(str::trim), Some("## Jane Doe"));
        assert!(!result.content.as_deref().unwrap().contains("Original"));
        result
    }

    fn accepted(html: &str, kind: Kind, tag: &str) {
        let control = replacement(html, kind, false, tag);
        assert!(control.metadata.headers.is_empty());
        let result = replacement(html, kind, true, tag);
        assert_eq!(
            result.content, control.content,
            "metadata must not repeat or alter the callback"
        );
        heading(&result, "Jane Doe");
    }

    #[test]
    fn heading_metadata_includes_the_actual_custom_line_break_replacement_once() {
        accepted("<h2>Jane <br></h2>", Kind::Break, "br");
    }

    #[test]
    fn heading_metadata_includes_the_actual_custom_form_replacement_once() {
        accepted("<h2>Jane <form>Original</form></h2>", Kind::Form, "form");
    }

    #[test]
    fn heading_metadata_includes_the_actual_custom_button_replacement_once() {
        accepted("<h2>Jane <button>Original</button></h2>", Kind::Button, "button");
    }

    #[test]
    fn custom_form_metadata_keeps_the_actual_inserted_boundaries_without_source_spaces() {
        let mut actual = Vec::new();
        for metadata in [false, true] {
            let visitor = Arc::new(Mutex::new(Replacement {
                kind: Kind::Form,
                calls: Vec::new(),
                original_button: Vec::new(),
            }));
            let handle: VisitorHandle = visitor.clone();
            let result = convert(
                "<h2>Jane<form>Original</form>Tail</h2>",
                Some(ConversionOptions {
                    extract_metadata: metadata,
                    visitor: Some(handle),
                    ..options()
                }),
            )
            .expect("the actual specialized form callback must succeed");
            assert_eq!(
                visitor.lock().unwrap().calls,
                [("form".to_string(), Some("h2".to_string()))]
            );
            assert_eq!(result.content.as_deref().map(str::trim), Some("## Jane Doe Tail"));
            assert!(!result.content.as_deref().unwrap().contains("Original"));
            if metadata {
                actual.push(result);
            } else {
                assert!(result.metadata.headers.is_empty());
            }
        }
        assert_eq!(actual.len(), 1);
        heading(&actual[0], "Jane Doe Tail");
    }
}

#[test]
fn preserved_span_metadata_keeps_descendants_the_actual_serializer_does_not_exclude() {
    let result = unchanged(
        r#"<h2>Jane <span>Doe <b class="omit">Visible</b></span></h2>"#,
        &ConversionOptions {
            preserve_tags: vec!["span".to_string()],
            exclude_selectors: vec![".omit".to_string()],
            ..options()
        },
    );
    assert_eq!(
        result.content.as_deref().map(str::trim),
        Some(r#"## Jane <span>Doe <b class="omit">Visible</b></span>"#)
    );
    heading(&result, "Jane Doe Visible");
}

#[test]
fn preserved_span_metadata_decodes_entities_and_keeps_literal_code_whitespace() {
    let result = unchanged(
        "<h2>Jane <span>Doe &amp; &lt;tag&gt; <code>a  b</code></span></h2>",
        &ConversionOptions {
            preserve_tags: vec!["span".to_string()],
            ..options()
        },
    );
    assert_eq!(
        result.content.as_deref().map(str::trim),
        Some("## Jane <span>Doe &amp; &lt;tag&gt; <code>a  b</code></span>")
    );
    heading(&result, "Jane Doe & <tag> a  b");
}
