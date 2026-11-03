// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression test for issue #391: XHTML-style self-closing tags like `<td/>`
//! (no space before `/`) silently truncated tables and dropped the rest of the
//! document. The tokenizer reads the slash as part of the tag, never of its name.
//! These tests pin `<td/>` (the reported case from an EPUB-derived page), `<br/>`
//! (still a line break), a plain non-table element, and author text that holds
//! `<x/>`, which keeps its bytes.

use html_to_markdown_rs::convert;

#[test]
fn empty_td_self_closing_does_not_truncate_table() {
    let html = "<table>\
        <tr><td>x</td><td>y</td><td/><td>z</td><td>w</td></tr>\
        <tr><td>aa</td><td>bb</td><td>cc</td><td>dd</td><td>ee</td></tr>\
        </table>\
        <p>after</p>";
    let result = convert(html, None).expect("conversion should succeed");
    let content = result.content.unwrap_or_default();
    assert!(
        content.contains("| x"),
        "first row should be a markdown table row, got:\n{content}"
    );
    assert!(
        content.contains("| aa"),
        "second row must still appear after a self-closing cell, got:\n{content}"
    );
    assert!(
        content.contains("after"),
        "content after the table must not be dropped, got:\n{content}"
    );
}

#[test]
fn br_self_closing_still_renders_as_line_break() {
    let html = "<p>a<br/>b</p>";
    let result = convert(html, None).expect("conversion should succeed");
    let content = result.content.unwrap_or_default();
    assert!(
        content.contains("a  \nb"),
        "<br/> should render as a markdown line break, got:\n{content:?}"
    );
}

#[test]
fn nested_self_closing_does_not_affect_following_paragraphs() {
    let html = "<div><span/>before<p>after</p></div>";
    let result = convert(html, None).expect("conversion should succeed");
    let content = result.content.unwrap_or_default();
    assert!(
        content.contains("before") && content.contains("after"),
        "both 'before' and 'after' must render, got:\n{content}"
    );
}

#[test]
fn explicit_space_before_self_closing_unchanged() {
    let html = "<table><tr><td>x</td><td /><td>y</td></tr></table>";
    let result = convert(html, None).expect("conversion should succeed");
    let content = result.content.unwrap_or_default();
    assert!(
        content.contains("| x") && content.contains("| y"),
        "explicit-space self-close should already work, got:\n{content}"
    );
}

#[test]
fn should_keep_self_closing_markup_in_raw_text_as_written() {
    for html in ["<xmp><br/><td/></xmp>", "<textarea><br/><td/></textarea>"] {
        let content = convert(html, None)
            .expect("conversion should succeed")
            .content
            .unwrap_or_default();
        assert!(content.contains("<br/><td/>"), "input: {html:?}; actual: {content:?}");
    }
}
