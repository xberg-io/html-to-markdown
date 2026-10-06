#![allow(missing_docs)]
#![cfg(feature = "testkit")]

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn convert_with(html: &str, tier_strategy: TierStrategy) -> String {
    convert(
        html,
        Some(ConversionOptions {
            extract_metadata: false,
            tier_strategy,
            ..ConversionOptions::default()
        }),
    )
    .expect("conversion must succeed")
    .content
    .unwrap_or_default()
}

#[test]
fn should_preserve_all_line_feeds_without_emitting_a_blank_line_in_image_alt_text() {
    let html = "<img alt=\"A\n\nB\nC\n\" src=\"S\"/>";
    let expected = "![A&#10;\nB\nC\n](S)\n";

    for tier_strategy in [TierStrategy::Tier1, TierStrategy::Tier2, TierStrategy::Auto] {
        assert_eq!(convert_with(html, tier_strategy), expected, "{tier_strategy:?}");
    }
}

#[test]
fn should_encode_space_and_tab_only_blank_lines_in_image_alt_text() {
    for alt in ["A\n \nB", "A\n\t\nB"] {
        let html = format!("<img alt=\"{alt}\" src=\"S\"/>");

        for tier_strategy in [TierStrategy::Tier1, TierStrategy::Tier2, TierStrategy::Auto] {
            assert_eq!(
                convert_with(&html, tier_strategy),
                "![A&#10;\nB](S)\n",
                "{tier_strategy:?}: {alt:?}"
            );
        }
    }
}

#[test]
fn should_normalize_carriage_return_blank_lines_in_image_alt_text() {
    for alt in ["A\r\rB", "A\r\n\r\nB"] {
        let html = format!("<img alt=\"{alt}\" src=\"S\"/>");

        for tier_strategy in [TierStrategy::Tier1, TierStrategy::Tier2, TierStrategy::Auto] {
            assert_eq!(
                convert_with(&html, tier_strategy),
                "![A&#10;\nB](S)\n",
                "{tier_strategy:?}: {alt:?}"
            );
        }
    }
}
