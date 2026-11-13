#![allow(missing_docs)]
#![cfg(feature = "testkit")]

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn convert_with(html: &str, tier_strategy: TierStrategy) -> String {
    let options = ConversionOptions {
        extract_metadata: false,
        tier_strategy,
        ..ConversionOptions::default()
    };
    convert(html, Some(options))
        .expect("conversion must succeed")
        .content
        .unwrap_or_default()
}

fn assert_all_tiers(html: &str, expected: &str) {
    for tier_strategy in [TierStrategy::Tier1, TierStrategy::Tier2, TierStrategy::Auto] {
        assert_eq!(
            convert_with(html, tier_strategy),
            expected,
            "{html} ({tier_strategy:?})"
        );
    }
}

#[test]
fn should_bold_each_summary_or_legend_block_separately() {
    assert_all_tiers("<summary><p>one</p><p>two</p></summary>", "**one**\n\n**two**\n");
    assert_all_tiers("<legend><p>one</p><p>two</p></legend>", "**one**\n\n**two**\n");
}

#[test]
fn should_keep_summary_markers_inside_the_surrounding_list_item() {
    assert_all_tiers(
        "<ul><li>a<summary><p>p</p>t</summary>u</li></ul>",
        "- a\n\n  **p**\n\n  **t**\n\n  u\n",
    );
}

#[test]
fn should_keep_a_structured_summary_list_in_one_emphasis_run() {
    assert_all_tiers(
        "<details><summary><ul><li>x<dl><dd>d</dd></dl>t</li></ul></summary></details>",
        "**- x\n\nd\n\nt**\n",
    );
}

#[test]
fn should_split_strong_and_emphasis_markers_around_block_children() {
    assert_all_tiers("<strong>a<div>b</div>c</strong>", "**a**\n\n**b**\n\n**c**\n");
    assert_all_tiers("<em>a<div>b</div>c</em>", "*a*\n\n*b*\n\n*c*\n");
}

#[test]
fn should_place_emphasis_inside_headings() {
    assert_all_tiers(
        "<details><summary><h3>Question</h3><p>Subtitle</p></summary><p>Answer</p></details>",
        "### **Question**\n\n**Subtitle**\n\nAnswer\n",
    );
    assert_all_tiers("<strong><h3>Title</h3></strong>", "### **Title**\n");
    assert_all_tiers("<em><h3>Title</h3></em>", "### *Title*\n");
    assert_all_tiers(
        "<figcaption><h3>Title</h3><p>Description</p></figcaption>",
        "### *Title*\n\n*Description*\n",
    );
    assert_all_tiers("<del><h3>Title</h3></del>", "### ~~Title~~\n");
    assert_all_tiers("<mark><h3>Title</h3></mark>", "### ==Title==\n");
}
