// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issues #583 and #585.
//!
//! #583: a block inside a list item, followed by text in the same item, left the item: a heading
//! joined the item's text (`- A### H`), a rule broke out of the list, and the text after a div,
//! table, blockquote, code block or nested list became a lazy continuation or left the list.
//! `CommonMark` keeps a block in a list item only when its lines start at the item's content column.
//!
//! #585: text after a list or table at the end of an inline wrapper (`<span>`) continued the
//! list's last item or the table's last row.

use html_to_markdown_rs::options::ListIndentType;
use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, HighlightStyle, TierStrategy, convert, tier1};

const GFM_TABLE: &str = "<table><tr><th>h</th></tr><tr><td>c</td></tr></table>";

const LAYOUT_TABLE: &str = r#"<table><tr><td><a href="1">a</a> <a href="2">b</a></td></tr></table>"#;

/// Blocks inside a list item, with a fragment the block renders to.
const BLOCKS: [(&str, &str); 11] = [
    ("<h3>H</h3>", "<h3>H</h3>"),
    ("<hr>", "<hr />"),
    ("<div>d</div>", "<p>d</p>"),
    ("<p>p</p>", "<p>p</p>"),
    ("<ul><li>B</li></ul>", "<ul>\n<li>B</li>\n</ul>"),
    ("<blockquote>q</blockquote>", "<blockquote>\n<p>q</p>\n</blockquote>"),
    (GFM_TABLE, "<td>c</td>"),
    (LAYOUT_TABLE, r#"<a href="2">b</a>"#),
    ("<pre>code</pre>", "<pre><code>code\n</code></pre>"),
    ("<dl><dt>T</dt><dd>D</dd></dl>", "<p>T\nD</p>"),
    ("<span><ul><li>B</li></ul></span>", "<ul>\n<li>B</li>\n</ul>"),
];

/// List item shapes: the HTML around the block, and the rendered end of the item that holds it.
const ITEMS: [(&str, &str); 4] = [
    ("<ul><li>A{}tail</li></ul>", "<p>tail</p>\n</li>\n</ul>\n"),
    ("<ol><li>A{}tail</li></ol>", "<p>tail</p>\n</li>\n</ol>\n"),
    ("<ul><li>X<ul><li>A{}tail</li></ul></li></ul>", "<p>tail</p>\n</li>\n</ul>\n</li>\n</ul>\n"),
    ("<ul><li><div>A{}tail</div></li></ul>", "<p>tail</p>\n</li>\n</ul>\n"),
];

fn tier2_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        tier_strategy: TierStrategy::Tier2,
        ..ConversionOptions::default()
    }
}

fn convert_with(html: &str, options: ConversionOptions) -> String {
    convert(html, Some(options))
        .expect("conversion must succeed")
        .content
        .unwrap_or_default()
}

fn tier2(html: &str) -> String {
    convert_with(html, tier2_options())
}

/// Tier 1 output, or `None` when Tier 1 hands the input to Tier 2.
fn tier1(html: &str) -> Option<String> {
    let options = ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        ..tier2_options()
    };
    tier1::run(html, &PrescanReport::default(), &options).ok()
}

fn render(markdown: &str) -> String {
    let mut options = comrak::Options::default();
    options.extension.table = true;
    comrak::markdown_to_html(markdown, &options)
}

/// Whether the rendered item holds the block and the text after it, in that order.
fn keeps_block_and_text_in_item(rendered: &str, block: &str, item_end: &str) -> bool {
    rendered.ends_with(item_end)
        && rendered
            .find(block)
            .is_some_and(|at| at < rendered.len() - item_end.len())
}

#[test]
fn should_render_the_heading_on_its_own_line_inside_the_item() {
    let out = tier2("<ul><li>A<h3>H</h3>tail</li></ul>");
    assert_eq!(out, "- A\n  ### H\n\n  tail\n", "the heading must start at the content column");
    assert_eq!(
        render(&out),
        "<ul>\n<li>\n<p>A</p>\n<h3>H</h3>\n<p>tail</p>\n</li>\n</ul>\n"
    );
}

#[test]
fn should_keep_a_rule_and_the_text_after_it_inside_the_item() {
    let out = tier2("<ul><li>A<hr>tail</li></ul>");
    assert_eq!(out, "- A\n\n  ---\n\n  tail\n", "the rule must start at the content column");
    assert_eq!(render(&out), "<ul>\n<li>\n<p>A</p>\n<hr />\n<p>tail</p>\n</li>\n</ul>\n");
}

#[test]
fn should_keep_every_block_and_the_text_after_it_inside_the_list_item() {
    let mut failures = Vec::new();
    let mut checked = 0;
    for (item, item_end) in ITEMS {
        for (block, fragment) in BLOCKS {
            let html = item.replace("{}", block);
            let out = tier2(&html);
            let rendered = render(&out);
            if !keeps_block_and_text_in_item(&rendered, fragment, item_end) {
                failures.push(format!("{html:?}: {out:?} renders {rendered:?}"));
            }
            checked += 1;
        }
    }
    assert!(
        failures.is_empty(),
        "the block or the text after it left the list item:\n{}",
        failures.join("\n")
    );
    assert_eq!(checked, ITEMS.len() * BLOCKS.len());
}

#[test]
fn should_keep_the_block_inside_the_item_with_tab_indentation() {
    let options = ConversionOptions {
        list_indent_type: ListIndentType::Tabs,
        ..tier2_options()
    };
    let mut failures = Vec::new();
    // ~keep A fenced code block indented by a tab keeps the tab's extra columns in its content,
    // ~keep with or without text after it, so the code block is left out here.
    for (block, fragment) in BLOCKS.iter().filter(|(block, _)| !block.starts_with("<pre>")) {
        let html = format!("<ul><li>A{block}tail</li></ul>");
        let out = convert_with(&html, options.clone());
        let rendered = render(&out);
        if !keeps_block_and_text_in_item(&rendered, fragment, "<p>tail</p>\n</li>\n</ul>\n") {
            failures.push(format!("{html:?}: {out:?} renders {rendered:?}"));
        }
    }
    assert!(failures.is_empty(), "with tabs:\n{}", failures.join("\n"));
}

#[test]
fn should_agree_across_tiers_or_leave_the_item_to_tier2() {
    let mut failures = Vec::new();
    for (item, _) in ITEMS {
        for (block, _) in BLOCKS {
            let html = item.replace("{}", block);
            if let Some(tier1_out) = tier1(&html) {
                let tier2_out = tier2(&html);
                if tier1_out != tier2_out {
                    failures.push(format!("{html:?}: tier1 {tier1_out:?} vs tier2 {tier2_out:?}"));
                }
            }
        }
    }
    assert!(failures.is_empty(), "Tier 1 must match Tier 2:\n{}", failures.join("\n"));
}

#[test]
fn should_leave_blocks_that_start_the_item_or_need_no_separation_unchanged() {
    for (html, expected) in [
        ("<ul><li><h3>H</h3></li></ul>", "- ### H\n"),
        ("<ul><li><p>A</p><p>B</p></li></ul>", "- A\n\n  B\n"),
        ("<ul><li>A<ul><li>B</li></ul></li><li>C</li></ul>", "- A\n  * B\n- C\n"),
        ("<ul><li>A<blockquote>q</blockquote></li></ul>", "- A\n  > q\n"),
        ("<ul><li>A<pre>code</pre></li></ul>", "- A\n\n  ```\n  code\n  ```\n"),
        ("<ul><li>A<br>B</li></ul>", "- A  \n  B\n"),
    ] {
        assert_eq!(tier2(html), expected, "{html:?} changed");
        if let Some(tier1_out) = tier1(html) {
            assert_eq!(tier1_out, expected, "Tier 1 changed {html:?}");
        }
    }
}

#[test]
fn should_leave_text_between_the_items_of_a_nested_list_alone() {
    // ~keep Text that is a child of the list, not of an item, is not item content.
    let html = "<ul><li>A<ul><li>B</li>x<li>C</li></ul></li></ul>";
    let out = tier2(html);
    assert!(!out.contains("\n\n"), "the nested list must stay tight: {out:?}");
}

/// Wrappers around a block, then the text after the wrapper, with the rendered block end.
const WRAPPED: [(&str, &str); 6] = [
    ("<div><span><ul><li>A</li></ul></span>para</div>", "</ul>\n<p>para</p>\n"),
    ("<div><span><span><ul><li>A</li></ul></span></span>para</div>", "</ul>\n<p>para</p>\n"),
    ("<div><span><ul><li>A</li></ul> </span><span>para</span></div>", "</ul>\n<p>para</p>\n"),
    ("<span><ul><li>A</li></ul></span><b>para</b>", "</ul>\n<p><strong>para</strong></p>\n"),
    ("<div><span><table><tr><th>h</th></tr><tr><td>c</td></tr></table></span>para</div>", "</table>\n<p>para</p>\n"),
    ("<div><span><hr></span>para</div>", "<hr />\n<p>para</p>\n"),
];

#[test]
fn should_start_a_paragraph_after_a_block_at_the_end_of_an_inline_wrapper() {
    for (html, rendered_end) in WRAPPED {
        let out = tier2(html);
        let rendered = render(&out);
        assert!(
            rendered.ends_with(rendered_end),
            "{html:?}: the text must start a paragraph after the block: {out:?} renders {rendered:?}"
        );
        let tier1_out = tier1(html).expect("tier1 converts an inline wrapper");
        assert_eq!(tier1_out, out, "Tier 1 must match on {html:?}");
    }
}

#[test]
fn should_not_separate_text_that_continues_the_wrapper_after_its_block() {
    // ~keep The wrapper's own text after the block is the paragraph; the next text joins it.
    let html = "<div><span><ul><li>A</li></ul>x</span>para</div>";
    let out = tier2(html);
    assert_eq!(out, "- A\n\nxpara\n");
    assert_eq!(tier1(html).as_deref(), Some("- A\n\nxpara\n"), "Tier 1 must match on {html:?}");
}
