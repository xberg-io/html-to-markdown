//! Visitor rollback must not retain text after an excluded child shifts its output boundary.

#![cfg(feature = "visitor")]

use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult, VisitorHandle};
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
struct SkipSpan {
    outputs: Vec<String>,
}

impl HtmlVisitor for SkipSpan {
    fn visit_element_end(&mut self, context: &NodeContext<'_>, output: &str) -> VisitResult {
        if context.tag_name == "span" {
            self.outputs.push(output.to_string());
            VisitResult::Skip
        } else {
            VisitResult::Continue
        }
    }
}

fn assert_skipped_child(html: &str, expected: &str, observed: &str, expected_metadata: &str) {
    let modes = std::iter::once(false);
    #[cfg(feature = "metadata")]
    let modes = modes.chain(std::iter::once(true));
    #[cfg(not(feature = "metadata"))]
    let _ = expected_metadata;
    for extract_metadata in modes {
        let visitor = Arc::new(Mutex::new(SkipSpan::default()));
        let handle: VisitorHandle = visitor.clone();
        let result = convert(
            html,
            Some(ConversionOptions {
                visitor: Some(handle),
                extract_metadata,
                tier_strategy: TierStrategy::Tier2,
                exclude_selectors: vec![".omit".to_string()],
                ..ConversionOptions::default()
            }),
        )
        .expect("the real visitor rollback must succeed");
        assert_eq!(result.content.as_deref().map(str::trim), Some(expected));
        assert_eq!(visitor.lock().expect("visitor state").outputs, [observed]);
        #[cfg(feature = "metadata")]
        if extract_metadata {
            assert_eq!(
                result
                    .metadata
                    .headers
                    .iter()
                    .map(|header| header.text.as_str())
                    .collect::<Vec<_>>(),
                [expected_metadata]
            );
        }
    }
}

#[test]
fn end_skip_removes_all_child_text_after_exclusion_trims_its_start() {
    assert_skipped_child(
        r#"<h2>Jane <span><i class="omit">Excluded</i>Ignored</span> Doe</h2>"#,
        "## Jane Doe",
        "Ignored",
        "Jane Doe",
    );
}

#[test]
fn end_skip_tracks_utf8_boundaries_after_excluded_child_trimming() {
    assert_skipped_child(
        r#"<h2>Jäné <span><i class="omit">Excluded</i>漢字Ignored</span> Doe</h2>"#,
        "## Jäné Doe",
        "漢字Ignored",
        "Jäné Doe",
    );
}

#[test]
fn end_skip_preserves_preceding_inline_code_spaces() {
    assert_skipped_child(
        r#"<h2>Jane <code>a  b</code> <span><i class="omit">Excluded</i>Ignored</span> Doe</h2>"#,
        "## Jane `a  b` Doe",
        "Ignored",
        "Jane a  b Doe",
    );
}

#[test]
fn table_cell_trimming_does_not_move_an_ancestor_checkpoint_in_another_buffer() {
    fn rendered(html: &str) -> String {
        let visitor = Arc::new(Mutex::new(SkipSpan::default()));
        let handle: VisitorHandle = visitor.clone();
        let result = convert(
            html,
            Some(ConversionOptions {
                visitor: Some(handle),
                tier_strategy: TierStrategy::Tier2,
                exclude_selectors: vec![".omit".to_string()],
                ..ConversionOptions::default()
            }),
        )
        .expect("the nested-buffer conversion must succeed");
        assert_eq!(visitor.lock().expect("visitor state").outputs.len(), 1);
        result.content.expect("rendered output")
    }
    let actual = rendered(
        r#"<div>Before <span><table><tr><td>Cell <i class="omit">Excluded</i>Tail</td></tr></table></span> After</div>"#,
    );
    let control = rendered("<div>Before <span>Ignored</span> After</div>");
    assert!(control.contains("Before"));
    assert!(control.contains("After"));
    assert_eq!(actual, control);
}
