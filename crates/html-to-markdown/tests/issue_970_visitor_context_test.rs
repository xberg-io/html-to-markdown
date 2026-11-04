#![allow(missing_docs)]
#![cfg(feature = "visitor")]

use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult, VisitorHandle};
use html_to_markdown_rs::{ConversionOptions, convert};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
struct Recorder {
    starts: BTreeMap<String, BTreeMap<String, String>>,
    events: Vec<String>,
    video_src: Option<String>,
    media_action: Option<VisitResult>,
    source_end: Option<VisitResult>,
    source_start: Option<VisitResult>,
}

impl HtmlVisitor for Recorder {
    fn visit_element_start(&mut self, context: &NodeContext<'_>) -> VisitResult {
        self.starts
            .insert(context.tag_name.to_string(), context.attributes().clone());
        self.events.push(format!("+{}", context.tag_name));
        if context.tag_name == "source" {
            return self.source_start.take().unwrap_or_default();
        }
        VisitResult::Continue
    }

    fn visit_element_end(&mut self, context: &NodeContext<'_>, _output: &str) -> VisitResult {
        self.events.push(format!("-{}", context.tag_name));
        if context.tag_name == "source" {
            return self.source_end.take().unwrap_or_default();
        }
        VisitResult::Continue
    }

    fn visit_video(&mut self, _context: &NodeContext<'_>, src: Option<&str>) -> VisitResult {
        self.video_src = src.map(str::to_owned);
        self.media_action.take().unwrap_or_default()
    }
}

fn record(html: &str) -> (Recorder, html_to_markdown_rs::ConversionResult) {
    record_with(html, Recorder::default())
}

fn record_with(html: &str, recorder: Recorder) -> (Recorder, html_to_markdown_rs::ConversionResult) {
    let recorder = Arc::new(Mutex::new(recorder));
    let visitor: VisitorHandle = recorder.clone();
    let result = convert(
        html,
        Some(ConversionOptions {
            visitor: Some(visitor),
            include_document_structure: true,
            ..ConversionOptions::default()
        }),
    )
    .expect("conversion");
    let recorder = std::mem::take(&mut *recorder.lock().expect("visitor recorder"));
    (recorder, result)
}

#[test]
fn visitor_attributes_follow_html_attribute_reference_rules_once() {
    let html = concat!(
        r#"<a href="/x?a=1&amp;b=2" title="A &quot;q&quot; &lt;" data-double="&amp;lt;" "#,
        r#"data-legacy="&notit; &amp= &amp " data-numeric="&#x41;&#65;" data-unknown="&unknown;">t</a>"#,
        r#"<iframe src="/y?a=1&amp;b=2"></iframe>"#,
    );
    let (recorder, _) = record(html);
    let link = &recorder.starts["a"];
    assert_eq!(link["href"], "/x?a=1&b=2");
    assert_eq!(link["title"], "A \"q\" <");
    assert_eq!(link["data-double"], "&lt;");
    assert_eq!(link["data-legacy"], "&notit; &amp= & ");
    assert_eq!(link["data-numeric"], "AA");
    assert_eq!(link["data-unknown"], "&unknown;");
    assert_eq!(recorder.starts["iframe"]["src"], "/y?a=1&b=2");
}

#[test]
fn media_source_and_track_are_visited_in_order_without_duplicating_the_link() {
    let html = concat!(
        r#"<video><source data-src="/lazy.mp4"><track src="/captions.vtt"></video>"#,
        r#"<audio><source src="/song.mp3"><track src="/lyrics.vtt">fallback</audio>"#,
    );
    let (recorder, result) = record(html);
    let markdown = result.content.expect("Markdown");
    assert_eq!(
        recorder.events,
        [
            "+video", "+source", "-source", "+track", "-track", "-video", "+audio", "+source", "-source", "+track",
            "-track", "-audio"
        ]
    );
    assert_eq!(recorder.video_src, None);
    assert_eq!(recorder.starts["source"]["src"], "/song.mp3");
    assert_eq!(recorder.starts["track"]["src"], "/lyrics.vtt");
    let baseline = convert(
        html,
        Some(ConversionOptions {
            include_document_structure: true,
            ..ConversionOptions::default()
        }),
    )
    .expect("no visitor");
    assert_eq!(Some(&markdown), baseline.content.as_ref());
    assert_eq!(result.document, baseline.document);
    assert_eq!(markdown.matches("/song.mp3").count(), 1);
    assert!(!markdown.contains(".vtt"));
    assert!(markdown.contains("fallback"));
}

#[test]
fn source_callback_replaces_only_child_output() {
    let html = r#"<p>before<video><source src="/clip.mp4">fallback</video>after</p>"#;
    let (_, result) = record_with(
        html,
        Recorder {
            source_end: Some(VisitResult::Custom("caption ".into())),
            ..Recorder::default()
        },
    );
    let markdown = result.content.expect("Markdown");
    assert_eq!(markdown.matches("[video](/clip.mp4)").count(), 1);
    assert!(markdown.contains("caption fallback"));
    assert!(markdown.contains("before"));
    assert!(markdown.contains("after"));
}

#[test]
fn skipping_source_does_not_erase_parent_link_or_adjacent_link_spacing() {
    let html = r#"<a href="/outer">before<video><source src="/clip.mp4"></video>after</a>"#;
    let (_, result) = record_with(
        html,
        Recorder {
            source_end: Some(VisitResult::Skip),
            ..Recorder::default()
        },
    );
    assert_eq!(result.content.as_deref(), Some("[before video after](/outer)\n"));
}

#[test]
fn preserving_media_html_keeps_literal_source_spelling_and_skips_child_callbacks() {
    let html = r#"<video><source src="/clip.mp4?a=1&amp;b=2" data-nested="&amp;lt;"></video>"#;
    let (recorder, result) = record_with(
        html,
        Recorder {
            media_action: Some(VisitResult::PreserveHtml),
            ..Recorder::default()
        },
    );
    assert_eq!(recorder.events, ["+video", "-video"]);
    assert_eq!(recorder.video_src.as_deref(), Some("/clip.mp4?a=1&b=2"));
    let markdown = result.content.expect("Markdown");
    assert!(markdown.contains(r#"src="/clip.mp4?a=1&amp;b=2""#));
    assert!(markdown.contains(r#"data-nested="&amp;lt;""#));
}

#[test]
fn preserve_tags_ignores_direct_media_sources_without_hiding_callbacks() {
    let html = r#"<video><source src="/media.mp4"></video><source src="/outside.mp4">"#;
    let options = ConversionOptions {
        preserve_tags: vec!["source".into()],
        ..ConversionOptions::default()
    };
    let baseline = convert(html, Some(options.clone())).expect("no visitor");
    let recorder = Arc::new(Mutex::new(Recorder::default()));
    let visitor: VisitorHandle = recorder.clone();
    let observed = convert(
        html,
        Some(ConversionOptions {
            visitor: Some(visitor),
            ..options
        }),
    )
    .expect("visitor");

    assert_eq!(observed.content, baseline.content);
    let content = observed.content.expect("Markdown");
    assert!(!content.contains(r#"<source src="/media.mp4">"#));
    assert!(content.contains(r#"<source src="/outside.mp4">"#));
    assert_eq!(
        &recorder.lock().expect("recorder").events[..4],
        ["+video", "+source", "-source", "-video"]
    );
}

#[test]
fn explicit_source_preserve_html_emits_the_raw_declaration_and_calls_end() {
    let html = r#"<video><source src="/media.mp4?a=1&amp;b=2"></video>"#;
    let raw_source = r#"<source src="/media.mp4?a=1&amp;b=2">"#;
    let choices = [
        (Some(VisitResult::PreserveHtml), None),
        (None, Some(VisitResult::PreserveHtml)),
    ];
    for (source_start, source_end) in choices {
        let (recorder, result) = record_with(
            html,
            Recorder {
                source_end,
                source_start,
                ..Recorder::default()
            },
        );
        assert_eq!(recorder.events, ["+video", "+source", "-source", "-video"]);
        assert_eq!(result.content.expect("Markdown").matches(raw_source).count(), 1);
    }
}
