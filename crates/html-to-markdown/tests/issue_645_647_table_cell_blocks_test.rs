// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issues #645 and #647: a block in a table cell is separated from the cell
//! content before and after it, and both converters write the same cell.

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert, tier1};

fn tier2_options(br_in_tables: bool) -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        br_in_tables,
        tier_strategy: TierStrategy::Tier2,
        ..ConversionOptions::default()
    }
}

fn tier2(html: &str, br_in_tables: bool) -> String {
    convert(html, Some(tier2_options(br_in_tables)))
        .expect("conversion must succeed")
        .content
        .unwrap_or_default()
}

fn tier1_run(html: &str, br_in_tables: bool) -> Result<String, tier1::BailReason> {
    let options = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        ..tier2_options(br_in_tables)
    };
    tier1::run(html, &PrescanReport::default(), &options)
}

fn render(markdown: &str) -> String {
    let mut options = comrak::Options::default();
    options.extension.table = true;
    comrak::markdown_to_html(markdown, &options)
}

fn table(cell: &str) -> String {
    format!("<table><tr><td>{cell}</td></tr></table>")
}

/// The cell each tier writes, as `(cell html, br_in_tables off, br_in_tables on)`.
fn check(cases: &[(&str, &str, &str)]) {
    let mut failures = Vec::new();
    for (cell, off, on) in cases {
        let html = table(cell);
        for (br_in_tables, expected) in [(false, off), (true, on)] {
            let tier2_out = tier2(&html, br_in_tables);
            let tier1_out = tier1_run(&html, br_in_tables);
            let cell_line = tier2_out.lines().next().unwrap_or_default().to_string();
            if cell_line != format!("| {expected} |") || tier1_out.as_deref().ok() != Some(tier2_out.as_str()) {
                failures.push(format!(
                    "{cell:?} br_in_tables={br_in_tables}: tier2 {tier2_out:?} tier1 {tier1_out:?}, want | {expected} |"
                ));
            }
        }
    }
    assert!(failures.is_empty(), "cell differs:\n{}", failures.join("\n"));
}

#[test]
fn should_separate_text_after_a_paragraph_or_heading_in_a_cell() {
    check(&[
        ("<p>a</p>b", "a b", "a<br>b"),
        ("<h2>a</h2>b", "a b", "a<br>b"),
        ("<h2>a</h2><h2>b</h2>", "a b", "a<br>b"),
        ("<div>a</div>b", "a b", "a<br>b"),
        ("<ul><li>a</li></ul>b", "a b", "a<br>b"),
        ("<pre>a</pre>b", "a b", "a<br>b"),
        ("<p>a</p><b>b</b>", "a **b**", "a<br>**b**"),
        ("<b><h2>a</h2>b</b>", "**a b**", "**a<br>b**"),
    ]);
}

#[test]
fn should_render_the_words_after_a_paragraph_in_a_cell_apart() {
    let markdown = tier2(&table("<p>a</p>b"), false);
    assert!(render(&markdown).contains("<th>a b</th>"), "{markdown:?}");
    let markdown = tier2(&table("<h2>a</h2>b"), false);
    assert!(render(&markdown).contains("<th>a b</th>"), "{markdown:?}");
}

#[test]
fn should_separate_text_before_a_heading_or_code_block_in_a_cell() {
    check(&[
        ("a<h2>b</h2>c", "a b c", "a<br>b<br>c"),
        ("a<pre>b</pre>c", "a b c", "a<br>b<br>c"),
        ("a<pre> b</pre>", "a b", "a<br>b"),
    ]);
}

#[test]
fn should_write_one_break_where_the_cell_already_has_one() {
    check(&[
        ("<p>a</p> b", "a b", "a<br> b"),
        ("<p>a</p><br>b", "a b", "a<br>b"),
        ("<p>a</p>", "a", "a"),
        ("<h2></h2>b", "b", "b"),
        ("a<h2> </h2>b", "a b", "a<br>b"),
        ("a<h2> </h2>", "a", "a"),
        ("a<blockquote> </blockquote>", "a", "a"),
        ("a<b><h2>b</h2></b>", "a**b**", "a**b**"),
        ("<p>a<br></p>b", "a b", "a<br>b"),
        ("<section>a</section>b", "a b", "a<br>b"),
        ("<code>a<h2>b</h2></code>", "`ab`", "`ab`"),
        ("<code>a<pre>b</pre></code>", "`ab`", "`ab`"),
    ]);
}

#[test]
fn should_write_a_quote_in_a_cell_without_its_marker_in_both_tiers() {
    check(&[
        ("<blockquote>a</blockquote>b", "a b", "a<br>b"),
        ("<blockquote><hr></blockquote>", "---", "---"),
        ("a<blockquote>b</blockquote>c", "a b c", "a<br>b<br>c"),
        ("<blockquote><p>a</p><p>b</p></blockquote>", "a b", "a<br>b"),
        ("<blockquote><blockquote>a</blockquote>b</blockquote>", "a b", "a<br>b"),
        ("<code><blockquote>a</blockquote>b</code>", "`a`   `b`", "`a`   `b`"),
    ]);
}

#[test]
fn should_keep_the_citation_of_a_quote_in_a_cell() {
    let html = table(r#"<blockquote cite="http://x/">a</blockquote>b"#);
    assert_eq!(tier2(&html, false).lines().next(), Some("| a — <http://x/> b |"));
    assert_eq!(tier2(&html, true).lines().next(), Some("| a<br>— <http://x/><br>b |"));
}

#[test]
fn should_write_the_same_paragraph_break_in_a_cell_in_both_tiers() {
    check(&[
        ("a<p>b</p>c", "a b c", "a<br>b<br>c"),
        ("a<p>b</p>", "a b", "a<br>b"),
        ("<p>a</p><p>b</p>", "a b", "a<br>b"),
        ("<h2>a</h2><p>b</p>", "a b", "a<br>b"),
    ]);
}

#[test]
fn should_keep_the_code_span_after_an_empty_heading_in_a_cell() {
    check(&[("<hr><code><h2></h2>b c</code>", "--- `b c`", "---<br>`b c`")]);
}
