//! Metadata observes the exact subtree retained by the preserved-HTML serializer.

use crate::options::conversion::NATIVE_STACK_SAFE_DEPTH;
use crate::{ConversionOptions, ConversionResult, TierStrategy, convert};

fn source(wrappers: usize, inner: &str, prefix: &str) -> String {
    format!(
        "<h2>{prefix}<x-keep>{}{inner}{}</x-keep></h2>",
        "<span>".repeat(wrappers),
        "</span>".repeat(wrappers),
    )
}

fn inspect(html: &str, target: &str, expected_depth: usize) -> Vec<(usize, String)> {
    let dom = crate::dom::parse(html, crate::dom::ParserOptions).expect("actual source/output must parse");
    let root = dom
        .query_selector("x-keep")
        .expect("the real preserved element must be selectable")
        .next()
        .expect("one real preserved root");
    let mut stack = vec![(root, 0)];
    let mut found = Vec::new();
    let mut raw = Vec::new();
    while let Some((node, depth)) = stack.pop() {
        match node.get(dom.parser()) {
            Some(crate::dom::Node::Tag(tag)) => {
                if tag.name().as_utf8_str() == target {
                    found.push(depth);
                }
                stack.extend(
                    tag.children()
                        .top()
                        .as_slice()
                        .iter()
                        .rev()
                        .map(|child| (*child, depth + 1)),
                );
            }
            Some(crate::dom::Node::Raw(bytes)) => raw.push((depth, bytes.as_utf8_str().to_string())),
            _ => {}
        }
    }
    assert_eq!(
        found,
        [expected_depth],
        "the actual serialized element depth must be qualified"
    );
    raw
}

fn unchanged(html: &str) -> ConversionResult {
    let options = ConversionOptions {
        extract_metadata: false,
        tier_strategy: TierStrategy::Tier2,
        preserve_tags: vec!["x-keep".to_string()],
        ..ConversionOptions::default()
    };
    let control = convert(html, Some(options.clone())).expect("unredacted actual serialization must succeed");
    assert!(control.metadata.headers.is_empty());
    let result = convert(
        html,
        Some(ConversionOptions {
            extract_metadata: true,
            ..options
        }),
    )
    .expect("metadata conversion must succeed");
    assert_eq!(
        result.content, control.content,
        "metadata must not change accepted serialization"
    );
    result
}

fn heading(result: &ConversionResult, expected: &str) {
    assert_eq!(result.metadata.headers.len(), 1);
    assert_eq!(result.metadata.headers[0].level, 2);
    assert_eq!(result.metadata.headers[0].text, expected);
}

#[test]
fn preserved_svg_metadata_omits_title_text_beyond_the_actual_serialization_limit() {
    let html = source(NATIVE_STACK_SAFE_DEPTH - 2, "<svg><title>Secret</title></svg>", "Head");
    assert_eq!(
        inspect(&html, "title", NATIVE_STACK_SAFE_DEPTH),
        [(NATIVE_STACK_SAFE_DEPTH + 1, "Secret".to_string())]
    );
    let result = unchanged(&html);
    let rendered = result.content.as_deref().expect("actual preserved HTML output");
    assert!(rendered.starts_with("## Head<x-keep>"));
    assert!(rendered.contains("<svg><title></title></svg>"));
    assert!(!rendered.contains("Secret"));
    assert_eq!(
        inspect(rendered, "title", NATIVE_STACK_SAFE_DEPTH),
        Vec::<(usize, String)>::new()
    );
    heading(&result, "Head");
}

#[test]
fn preserved_img_metadata_keeps_alt_attributes_at_the_actual_serialization_limit() {
    let html = source(NATIVE_STACK_SAFE_DEPTH - 1, r#"<img src="i.png" alt="pic">"#, "Head ");
    assert_eq!(
        inspect(&html, "img", NATIVE_STACK_SAFE_DEPTH),
        Vec::<(usize, String)>::new()
    );
    let result = unchanged(&html);
    let rendered = result.content.as_deref().expect("actual preserved HTML output");
    assert!(rendered.starts_with("## Head <x-keep>"));
    assert!(rendered.contains(r#"<img src="i.png" alt="pic">"#));
    assert!(
        !rendered.contains("</img>"),
        "the admitted element must remain a real void tag"
    );
    assert_eq!(
        inspect(rendered, "img", NATIVE_STACK_SAFE_DEPTH),
        Vec::<(usize, String)>::new()
    );
    heading(&result, "Head pic");
}

#[test]
fn preserved_br_metadata_keeps_an_admitted_break_at_the_actual_serialization_limit() {
    let html = source(NATIVE_STACK_SAFE_DEPTH - 1, "Jane<br>Doe", "");
    let expected_raw = [
        (NATIVE_STACK_SAFE_DEPTH, "Jane".to_string()),
        (NATIVE_STACK_SAFE_DEPTH, "Doe".to_string()),
    ];
    assert_eq!(inspect(&html, "br", NATIVE_STACK_SAFE_DEPTH), expected_raw);
    let result = unchanged(&html);
    let rendered = result.content.as_deref().expect("actual preserved HTML output");
    assert!(rendered.contains("Jane<br>Doe"));
    assert!(
        !rendered.contains("</br>"),
        "the admitted element must remain a real void tag"
    );
    assert_eq!(inspect(rendered, "br", NATIVE_STACK_SAFE_DEPTH), expected_raw);
    heading(&result, "Jane Doe");
}
