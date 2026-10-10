#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for the review of pull request #791 and for issue #822.

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn convert_with(html: &str, tier_strategy: TierStrategy) -> String {
    convert(
        html,
        Some(ConversionOptions {
            tier_strategy,
            extract_metadata: false,
            ..ConversionOptions::default()
        }),
    )
    .expect("conversion must succeed")
    .content
    .unwrap_or_default()
}

fn check(html: &str, expected: &str) {
    for tier_strategy in [TierStrategy::Auto, TierStrategy::Tier2] {
        assert_eq!(
            convert_with(html, tier_strategy),
            expected,
            "{tier_strategy:?}: {html:?}"
        );
    }
}

#[test]
fn should_keep_the_word_space_before_a_description_in_a_heading() {
    check("<h2>a <dl><dd>b</dd></dl></h2>", "## a b\n");
}

#[test]
fn should_write_a_label_in_a_code_block_as_written() {
    check("<pre>a\n<label>  b\n</label>c</pre>", "```\na\n  b\nc\n```\n");
}

#[test]
fn should_write_a_label_in_a_code_block_as_a_span_there() {
    for body in ["a\n<X>  b\n</X>c", "a<X><div>x</div></X>b"] {
        let label = format!("<pre>{}</pre>", body.replace('X', "label"));
        let span = format!("<pre>{}</pre>", body.replace('X', "span"));
        for tier_strategy in [TierStrategy::Auto, TierStrategy::Tier2] {
            assert_eq!(
                convert_with(&label, tier_strategy),
                convert_with(&span, tier_strategy),
                "{tier_strategy:?}: {label:?}"
            );
        }
    }
}

#[test]
fn should_label_a_media_link_with_the_lower_case_element_name() {
    check("<p>x</p><VIDEO src=\"/v.mp4\"></VIDEO>", "x\n\n[video](/v.mp4)\n");
}
