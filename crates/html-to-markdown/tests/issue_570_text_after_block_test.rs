// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issues #570 and #571: inline content right after a list or a table, in
//! the same container, continued the block's last line. After a list it became a lazy
//! continuation of the last item (`- A\npara`); after a table it became another table row.
//!
//! In HTML, inline content that follows a block in the same container starts a new block of its
//! own, so every block type followed by every kind of inline content must render that content as
//! a paragraph after the block.

use html_to_markdown_rs::options::WhitespaceMode;
use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, HighlightStyle, TierStrategy, convert, tier1};

const LAYOUT_TABLE: &str = r#"<table><tr><td><a href="1">a</a> <a href="2">b</a> <a href="3">c</a></td></tr></table>"#;

const GFM_TABLE: &str = "<table><tr><th>h</th><th>g</th></tr><tr><td>c</td><td>d</td></tr></table>";

/// Blocks that end their last line with a single newline, then the ones that already end with a
/// blank line, as controls.
const BLOCKS: [&str; 12] = [
    "<ul><li>A</li></ul>",
    "<ol><li>A</li></ol>",
    "<ul><li>A<ul><li>B</li></ul></li></ul>",
    LAYOUT_TABLE,
    GFM_TABLE,
    "<hr>",
    "<blockquote>A</blockquote>",
    "<pre>A</pre>",
    "<h2>A</h2>",
    "<dl><dt>T</dt><dd>D</dd></dl>",
    "<p>A</p>",
    "<div>A</div>",
];

/// Inline content after the block, with the paragraph a `CommonMark` renderer must produce for it.
const FOLLOWERS: [(&str, &str); 7] = [
    ("para", "<p>para</p>"),
    ("\npara", "<p>para</p>"),
    ("<span>para</span>", "<p>para</p>"),
    ("<b>para</b>", "<p><strong>para</strong></p>"),
    (r#"<a href="x">para</a>"#, r#"<p><a href="x">para</a></p>"#),
    (r#"<img src="i.png" alt="x">"#, r#"<p><img src="i.png" alt="x" /></p>"#),
    ("<code>para</code>", "<p><code>para</code></p>"),
];

fn tier2_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        tier_strategy: TierStrategy::Tier2,
        ..ConversionOptions::default()
    }
}

fn tier2(html: &str) -> String {
    convert(html, Some(tier2_options()))
        .expect("tier2 conversion must succeed")
        .content
        .unwrap_or_default()
}

fn render(markdown: &str) -> String {
    let mut options = comrak::Options::default();
    options.extension.table = true;
    comrak::markdown_to_html(markdown, &options)
}

/// Every block, then every follower, inside a `<div>`, at the document root, and inside a
/// `<blockquote>`, with the rendered HTML the follower's paragraph must close.
fn cases() -> Vec<(String, String)> {
    let mut cases = Vec::new();
    for block in BLOCKS {
        for (follower, paragraph) in FOLLOWERS {
            cases.push((format!("<div>{block}{follower}</div>"), format!("{paragraph}\n")));
            cases.push((format!("{block}{follower}"), format!("{paragraph}\n")));
            cases.push((
                format!("<blockquote>{block}{follower}</blockquote>"),
                format!("{paragraph}\n</blockquote>\n"),
            ));
        }
    }
    cases
}

#[test]
fn should_render_text_after_a_list_as_a_paragraph_after_the_list() {
    let out = tier2("<div><ul><li>A</li></ul>para</div>");
    assert_eq!(out, "- A\n\npara\n", "the text must start a paragraph after the list");
    assert!(
        render(&out).ends_with("</ul>\n<p>para</p>\n"),
        "para must render outside the list item: {:?}",
        render(&out)
    );
}

#[test]
fn should_render_text_after_a_table_as_a_paragraph_not_a_table_row() {
    let out = tier2(&format!("<div>{GFM_TABLE}para</div>"));
    assert_eq!(
        out, "| h | g |\n| --- | --- |\n| c | d |\n\npara\n",
        "the text must start a paragraph after the table"
    );
    assert!(
        render(&out).ends_with("</table>\n<p>para</p>\n"),
        "para must render after the table, not as a row: {:?}",
        render(&out)
    );
}

#[test]
fn should_render_inline_content_after_every_block_as_its_own_paragraph() {
    let mut failures = Vec::new();
    for (html, paragraph_end) in cases() {
        let out = tier2(&html);
        let rendered = render(&out);
        if !rendered.ends_with(&paragraph_end) {
            failures.push(format!("{html:?}: {out:?} renders {rendered:?}"));
        }
    }
    assert!(
        failures.is_empty(),
        "the content after the block is not its own paragraph:\n{}",
        failures.join("\n")
    );
}

#[test]
fn should_render_inline_content_after_every_block_as_its_own_paragraph_in_strict_whitespace_mode() {
    let options = ConversionOptions {
        whitespace_mode: WhitespaceMode::Strict,
        ..tier2_options()
    };
    let mut failures = Vec::new();
    for (html, paragraph_end) in cases() {
        let out = convert(&html, Some(options.clone()))
            .expect("strict conversion must succeed")
            .content
            .unwrap_or_default();
        let rendered = render(&out);
        if !rendered.ends_with(&paragraph_end) {
            failures.push(format!("{html:?}: {out:?} renders {rendered:?}"));
        }
    }
    assert!(
        failures.is_empty(),
        "the content after the block is not its own paragraph:\n{}",
        failures.join("\n")
    );
}

#[test]
fn should_agree_across_tiers_on_inline_content_after_a_block() {
    // ~keep Tier 1 bails on some of these (tables, for one); the count assertion keeps this
    // ~keep from passing when every case bails.
    let report = PrescanReport::default();
    let options = ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        ..tier2_options()
    };
    let mut compared = 0;
    let mut failures = Vec::new();
    for (html, _) in cases() {
        if let Ok(tier1_out) = tier1::run(&html, &report, &options) {
            let tier2_out = tier2(&html);
            if tier1_out != tier2_out {
                failures.push(format!("{html:?}: tier1 {tier1_out:?} vs tier2 {tier2_out:?}"));
            }
            compared += 1;
        }
    }
    assert!(
        failures.is_empty(),
        "Tier 1 and Tier 2 must agree:\n{}",
        failures.join("\n")
    );
    assert!(compared >= 100, "Tier 1 converted only {compared} cases");
}

#[test]
fn should_leave_output_without_inline_content_after_a_block_unchanged() {
    assert_eq!(tier2("<ul><li>A</li><li>B</li></ul>"), "- A\n- B\n");
    assert_eq!(tier2("<div><ul><li>A</li></ul><br>para</div>"), "- A\n\npara\n");
    assert_eq!(tier2("<div><ul><li>A</li></ul></div>"), "- A\n");
    assert_eq!(tier2("<div>A<br>B</div>"), "A  \nB\n");
}

#[test]
fn should_not_add_a_blank_line_after_a_block_that_wrote_nothing() {
    let report = PrescanReport::default();
    let options = ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        ..tier2_options()
    };
    let tier1_out = tier1::run("<div><ul></ul>para</div>", &report, &options).expect("tier1 converts an empty list");
    assert!(
        !tier1_out.starts_with("\n\n"),
        "an empty list must not open a blank line: {tier1_out:?}"
    );
    let strict = ConversionOptions {
        whitespace_mode: WhitespaceMode::Strict,
        ..tier2_options()
    };
    let strict_out = convert("<div><div></div>\n<span> para</span></div>", Some(strict))
        .expect("strict conversion must succeed")
        .content
        .unwrap_or_default();
    assert!(
        !strict_out.starts_with("\n\n"),
        "an empty block must not open a blank line: {strict_out:?}"
    );
}

#[test]
fn should_keep_text_after_a_nested_list_inside_the_outer_list_item() {
    // ~keep Inside a list item the text after a nested list is continued by the item's own
    // ~keep indentation rules; a blank line there would move the text out of the list.
    let html = "<ul><li>A<ul><li>B</li></ul>tail</li></ul>";
    let out = tier2(html);
    let rendered = render(&out);
    assert!(
        !rendered.contains("<p>tail</p>") && rendered.trim_end().ends_with("</ul>"),
        "tail must stay inside the outer list: {out:?} renders {rendered:?}"
    );
    let options = ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        ..tier2_options()
    };
    let tier1_out = tier1::run(html, &PrescanReport::default(), &options).expect("tier1 converts a nested list");
    assert_eq!(tier1_out, out, "Tier 1 and Tier 2 must agree on {html:?}");
}
