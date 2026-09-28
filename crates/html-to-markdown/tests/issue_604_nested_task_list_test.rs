// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #604: a list item took the checkbox of an item in its nested list,
//! became a task item, and flattened the nested list into its own text.

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert, tier1};

fn options(tier_strategy: TierStrategy) -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        tier_strategy,
        ..ConversionOptions::default()
    }
}

fn convert_with(html: &str, tier_strategy: TierStrategy) -> String {
    convert(html, Some(options(tier_strategy)))
        .expect("conversion must succeed")
        .content
        .unwrap_or_default()
}

/// Each input with the Tier-2 output it must give.
const NESTED_TASKS: [(&str, &str); 5] = [
    (
        "<ul><li>X<ul><li><input type=\"checkbox\" checked> A</li></ul>ZZ</li></ul>",
        "- X\n  - [x] A\n\n  ZZ\n",
    ),
    (
        "<ul><li>X<ol><li><input type=\"checkbox\" checked> A</li></ol>ZZ</li></ul>",
        "- X\n  - [x] A\n\n  ZZ\n",
    ),
    (
        "<ul><li>X<ul><li><input type=\"checkbox\" checked> A</li></ul></li></ul>",
        "- X\n  - [x] A\n",
    ),
    (
        "<ul><li><input type=\"checkbox\"> X<ul><li><input type=\"checkbox\" checked> A</li></ul>ZZ</li></ul>",
        "- [ ] X\n  - [x] A\n\n  ZZ\n",
    ),
    ("<ul><li><p><input type=\"checkbox\"> A</p></li></ul>", "- [ ] A\n"),
];

#[test]
fn should_keep_a_nested_task_list_inside_its_own_item() {
    let failures: Vec<String> = NESTED_TASKS
        .iter()
        .filter_map(|(html, expected)| {
            let out = convert_with(html, TierStrategy::Tier2);
            (out != *expected).then(|| format!("{html:?}: {out:?}, want {expected:?}"))
        })
        .collect();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn should_hand_text_after_a_nested_task_list_to_the_full_converter() {
    let html = "<ul><li>X<ul><li><input type=\"checkbox\" checked> A</li></ul>ZZ</li></ul>";
    assert!(
        tier1::run(html, &PrescanReport::default(), &options(TierStrategy::Auto)).is_err(),
        "Tier 1 must not convert text after a nested list in a plain item"
    );
}

#[test]
fn should_give_the_same_nested_task_list_in_auto_mode() {
    let mut failures = Vec::new();
    for (html, _) in NESTED_TASKS {
        let tier2_out = convert_with(html, TierStrategy::Tier2);
        let auto_out = convert_with(html, TierStrategy::Auto);
        if auto_out != tier2_out {
            failures.push(format!("{html:?}: auto {auto_out:?} vs tier2 {tier2_out:?}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
