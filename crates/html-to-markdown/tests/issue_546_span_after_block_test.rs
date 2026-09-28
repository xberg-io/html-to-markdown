// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #546: a `<span>` that directly follows a list or a layout table
//! removed the newline that ends the list item's line. `<br>` inside the span then became a hard
//! break at the end of that item, and the text after it rendered inside the item as a lazy
//! continuation line. Without a `<br>` the span's text was joined onto the item itself.
//!
//! A `<span>` has no Markdown meaning of its own, so wrapping inline content in one must never
//! change the output.

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, HighlightStyle, TierStrategy, convert, tier1};

const LAYOUT_TABLE: &str = r#"<table><tr><td><a href="1">a</a> <a href="2">b</a> <a href="3">c</a></td></tr></table>"#;

/// The plusblog structure reduced: a layout table, the whitespace a pretty-printer leaves after
/// it, then a `<span>` that opens with a `<br>` and carries the next paragraph.
const ISSUE_546: &str = concat!(
    r#"<div><span><table><tr><td><a href="1">a</a> <a href="2">b</a> <a href="3">c</a></td></tr></table> </span>"#,
    "<span>\n<br>\npara</span></div>"
);

/// Every block that ends its last line with a single newline, plus the ones that end with a
/// blank line, as controls.
const BLOCKS: [&str; 9] = [
    LAYOUT_TABLE,
    "<ul><li>A</li></ul>",
    "<ol><li>A</li></ol>",
    "<ul><li>A<ul><li>B</li></ul></li></ul>",
    "<hr>",
    "<div>A</div>",
    "<blockquote>A</blockquote>",
    "<pre>A</pre>",
    "<p>A</p>",
];

fn tier2_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        tier_strategy: TierStrategy::Tier2,
        ..ConversionOptions::default()
    }
}

fn content(html: &str) -> String {
    convert(html, Some(ConversionOptions::default()))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

fn tier2(html: &str) -> String {
    convert(html, Some(tier2_options()))
        .expect("tier2 conversion must succeed")
        .content
        .unwrap_or_default()
}

fn render(markdown: &str) -> String {
    comrak::markdown_to_html(markdown, &comrak::Options::default())
}

#[test]
fn should_keep_the_paragraph_after_a_layout_table_out_of_the_list_item() {
    let out = content(ISSUE_546);
    assert_eq!(out, "- [a](1) [b](2) [c](3)\n\npara\n", "actual: {out:?}");
    let html = render(&out);
    assert!(
        html.contains("</ul>\n<p>para</p>"),
        "para must be its own paragraph after the list, rendered: {html:?}"
    );
}

#[test]
fn should_render_a_block_then_a_span_the_same_as_the_block_then_the_bare_content() {
    let mut failures = Vec::new();
    for block in BLOCKS {
        for tail in ["<br>para", "para", "\n<br>\npara"] {
            let wrapped = format!("<div>{block}<span>{tail}</span></div>");
            let bare = format!("<div>{block}{tail}</div>");
            let wrapped_out = tier2(&wrapped);
            let bare_out = tier2(&bare);
            if wrapped_out != bare_out {
                failures.push(format!("{wrapped:?}: span {wrapped_out:?} vs bare {bare_out:?}"));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "a span changed the output:\n{}",
        failures.join("\n")
    );
}

#[test]
fn should_keep_the_text_after_a_br_that_follows_a_block_out_of_that_block() {
    let mut failures = Vec::new();
    for block in BLOCKS {
        for html in [
            format!("<div>{block}<span><br>para</span></div>"),
            format!("<div>{block}<br>para</div>"),
        ] {
            let out = tier2(&html);
            if !render(&out).contains("<p>para</p>") {
                failures.push(format!("{html:?}: {out:?}"));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "para is not its own paragraph:\n{}",
        failures.join("\n")
    );
}

#[test]
fn should_not_join_a_span_onto_the_line_a_block_ends_with() {
    let list = tier2("<div><ul><li>A</li></ul><span>para</span></div>");
    assert!(
        list.starts_with("- A\n"),
        "the item's line must end before the span: {list:?}"
    );
    let rule = tier2("<div><hr><span>para</span></div>");
    assert!(
        render(&rule).contains("<hr />"),
        "the horizontal rule must survive: {rule:?}"
    );
}

#[test]
fn should_agree_across_tiers_where_tier1_converts_a_block_then_a_span() {
    // ~keep Tier 1 bails on tables, so only the blocks it converts itself are compared. The
    // ~keep count assertion keeps this from passing when every case bails.
    let report = PrescanReport::default();
    let options = ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        ..tier2_options()
    };
    let mut compared = 0;
    for block in BLOCKS {
        for tail in ["<br>para", "para"] {
            let html = format!("<div>{block}<span>{tail}</span></div>");
            if let Ok(tier1_out) = tier1::run(&html, &report, &options) {
                assert_eq!(tier1_out, tier2(&html), "Tier 1 and Tier 2 must agree on {html:?}");
                compared += 1;
            }
        }
    }
    assert!(compared >= 4, "Tier 1 converted only {compared} cases");
}
