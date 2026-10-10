// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn markdown(html: &str, tier_strategy: TierStrategy) -> String {
    convert(
        html,
        Some(ConversionOptions {
            extract_metadata: false,
            tier_strategy,
            ..ConversionOptions::default()
        }),
    )
    .expect("conversion should succeed")
    .content
    .unwrap_or_default()
}

#[test]
fn should_keep_a_table_whose_only_content_is_a_task_checkbox() {
    let html = r#"<table><tr><td><ul><li><input type="checkbox"></li></ul></td></tr></table>"#;

    for tier_strategy in [TierStrategy::Tier1, TierStrategy::Tier2, TierStrategy::Auto] {
        assert_eq!(
            markdown(html, tier_strategy),
            "| - [ ] |\n| ----- |\n",
            "{tier_strategy:?}"
        );
    }
}

#[test]
fn should_keep_a_table_whose_only_content_is_a_checked_input() {
    let html = r#"<table><tr><td><input type="checkbox" checked></td></tr></table>"#;

    for tier_strategy in [TierStrategy::Tier1, TierStrategy::Tier2, TierStrategy::Auto] {
        assert_eq!(markdown(html, tier_strategy), "| [x] |\n| --- |\n", "{tier_strategy:?}");
    }
}

#[test]
fn should_leave_a_checkbox_table_with_text_unchanged() {
    let html = r#"<table><tr><td><ul><li><input type="checkbox"></li></ul></td><td>b</td></tr></table>"#;

    for tier_strategy in [TierStrategy::Tier1, TierStrategy::Tier2, TierStrategy::Auto] {
        assert_eq!(
            markdown(html, tier_strategy),
            "| - [ ] | b |\n| ----- | --- |\n",
            "{tier_strategy:?}"
        );
    }
}
