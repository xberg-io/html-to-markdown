#![allow(missing_docs)]

fn convert(
    html: &str,
    opts: Option<html_to_markdown_rs::ConversionOptions>,
) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

/// Regression test for <https://github.com/kreuzberg-dev/html-to-markdown/issues/212>
///
/// When `\n` precedes an `<a>` tag inside a `<p>`, the whitespace was
/// inconsistently handled between the first and subsequent paragraphs.
/// The bug was stateful: identical HTML structures produced different
/// results depending on their position in the document.
#[test]
fn consistent_whitespace_before_link_across_paragraphs_issue_212() {
    let html = r#"<p>text before
<a href="https://example.com">the link</a>
after</p>
<p>text before
<a href="https://example.com">the link</a>
after</p>"#;

    let result = convert(html, None).unwrap();
    assert_eq!(
        result,
        "text before [the link](https://example.com) after\n\ntext before [the link](https://example.com) after\n"
    );
}

/// Same bug but with three paragraphs to ensure no accumulation effects.
#[test]
fn consistent_whitespace_three_paragraphs_issue_212() {
    let html = r#"<p>click
<a href="/a">here</a></p>
<p>click
<a href="/b">here</a></p>
<p>click
<a href="/c">here</a></p>"#;

    let result = convert(html, None).unwrap();
    assert_eq!(result, "click [here](/a)\n\nclick [here](/b)\n\nclick [here](/c)\n");
}

/// Verify the fix doesn't break whitespace between text nodes without links.
#[test]
fn newline_before_inline_elements_consistent_issue_212() {
    let html = r"<p>before
<strong>bold</strong> after</p>
<p>before
<strong>bold</strong> after</p>";

    let result = convert(html, None).unwrap();
    assert_eq!(result, "before **bold** after\n\nbefore **bold** after\n");
}

/// Verify with `<em>` tags across multiple paragraphs.
#[test]
fn newline_before_em_across_paragraphs_issue_212() {
    let html = r"<p>some text
<em>emphasized</em> end</p>
<p>some text
<em>emphasized</em> end</p>";

    let result = convert(html, None).unwrap();
    assert_eq!(result, "some text *emphasized* end\n\nsome text *emphasized* end\n");
}
