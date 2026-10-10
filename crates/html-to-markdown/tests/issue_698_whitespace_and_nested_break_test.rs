// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

use html_to_markdown_rs::{ConversionOptions, NewlineStyle, TierStrategy, convert};

fn markdown(html: &str, tier_strategy: TierStrategy, newline_style: NewlineStyle) -> String {
    convert(
        html,
        Some(ConversionOptions {
            extract_metadata: false,
            tier_strategy,
            newline_style,
            ..ConversionOptions::default()
        }),
    )
    .expect("conversion should succeed")
    .content
    .unwrap_or_default()
}

#[test]
fn should_fold_non_ascii_whitespace_after_a_hard_break_in_every_tier() {
    for (html, expected) in [
        ("<div>a<br>&#12;b</div>", "a  \n\u{c}b\n"),
        ("<div>a<br>\u{c}b</div>", "a  \n\u{c}b\n"),
        ("<div>a<br>&#x3000;b</div>", "a  \nb\n"),
        ("<div>a<br>\u{3000}b</div>", "a  \nb\n"),
    ] {
        for tier_strategy in [TierStrategy::Tier1, TierStrategy::Tier2, TierStrategy::Auto] {
            assert_eq!(
                markdown(html, tier_strategy, NewlineStyle::Spaces),
                expected,
                "{tier_strategy:?}: {html:?}"
            );
        }
    }
}

#[test]
fn should_keep_both_breaks_in_one_list_item_paragraph() {
    let html = "<ul><li>a<br><b><br>b</b></li></ul>";
    let expected = "- a  \n  \\\n  **b**\n";

    for tier_strategy in [TierStrategy::Tier1, TierStrategy::Tier2, TierStrategy::Auto] {
        let output = markdown(html, tier_strategy, NewlineStyle::Spaces);
        assert_eq!(output, expected, "{tier_strategy:?}");
        assert_eq!(
            comrak::markdown_to_html(&output, &comrak::Options::default()),
            "<ul>\n<li>a<br />\n<br />\n<strong>b</strong></li>\n</ul>\n",
            "{tier_strategy:?}"
        );
    }
}

#[test]
fn should_keep_both_backslash_breaks_in_one_list_item_paragraph() {
    let html = "<ul><li>a<br><b><br>b</b></li></ul>";

    assert_eq!(
        markdown(html, TierStrategy::Tier2, NewlineStyle::Backslash),
        "- a\\\n  \\\n  **b**\n"
    );
}
