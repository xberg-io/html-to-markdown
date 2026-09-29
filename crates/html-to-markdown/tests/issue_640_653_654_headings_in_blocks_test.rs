// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for headings inside blocks: a heading after text in a quote (issue #640), an
//! underlined heading whose text starts a block (issue #653), and the lines of a list item in a
//! quote or under the tab indent (issue #654).

use html_to_markdown_rs::options::{HeadingStyle, ListIndentType};
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn options(strategy: TierStrategy) -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        tier_strategy: strategy,
        ..ConversionOptions::default()
    }
}

fn underlined(strategy: TierStrategy) -> ConversionOptions {
    ConversionOptions {
        heading_style: HeadingStyle::Underlined,
        ..options(strategy)
    }
}

fn tabs(options: ConversionOptions) -> ConversionOptions {
    ConversionOptions {
        list_indent_type: ListIndentType::Tabs,
        ..options
    }
}

fn render(markdown: &str) -> String {
    comrak::markdown_to_html(markdown, &comrak::Options::default())
}

/// Convert `html` with `options` in Tier 2 and Auto, assert the exact markdown, and assert that
/// its rendering is `rendered` with the line ends removed.
fn assert_converts(html: &str, options: &ConversionOptions, expected: &str, rendered: &str) {
    for strategy in [TierStrategy::Tier2, TierStrategy::Auto] {
        let options = ConversionOptions {
            tier_strategy: strategy,
            ..options.clone()
        };
        let markdown = convert(html, Some(options))
            .expect("conversion must succeed")
            .content
            .unwrap_or_default();
        assert_eq!(markdown, expected, "{html} ({strategy:?})");
        assert_eq!(render(&markdown).replace('\n', ""), rendered, "{html} ({strategy:?})");
    }
}

#[test]
fn should_start_a_heading_after_text_in_a_quote_after_a_blank_line() {
    let html = "<blockquote>a<h2>q</h2></blockquote>";
    let rendered = "<blockquote><p>a</p><h2>q</h2></blockquote>";
    assert_converts(html, &options(TierStrategy::Tier2), "> a\n>\n> ## q\n", rendered);
    assert_converts(html, &underlined(TierStrategy::Tier2), "> a\n>\n> q\n> -\n", rendered);
    let tier1 = convert(html, Some(options(TierStrategy::Tier1))).expect("conversion must succeed");
    assert_eq!(tier1.content.as_deref(), Some("> a\n>\n> ## q\n"));
    assert_converts(
        "<blockquote>a<h1>q</h1></blockquote>",
        &underlined(TierStrategy::Tier2),
        "> a\n>\n> q\n> =\n",
        "<blockquote><p>a</p><h1>q</h1></blockquote>",
    );
    assert_converts(
        "<blockquote><blockquote>a<h2>q</h2></blockquote></blockquote>",
        &underlined(TierStrategy::Tier2),
        "> > a\n> >\n> > q\n> > -\n",
        "<blockquote><blockquote><p>a</p><h2>q</h2></blockquote></blockquote>",
    );
    assert_converts(
        "<ul><li><blockquote>a<h2>q</h2></blockquote></li></ul>",
        &underlined(TierStrategy::Tier2),
        "- > a\n  >\n  > q\n  > -\n",
        "<ul><li><blockquote><p>a</p><h2>q</h2></blockquote></li></ul>",
    );
}

#[test]
fn should_start_an_underlined_heading_after_a_line_of_text_in_a_quote_after_a_blank_line() {
    assert_converts(
        "<blockquote>a<br><h2>q</h2></blockquote>",
        &underlined(TierStrategy::Tier2),
        "> a  \n>\n> q\n> -\n",
        "<blockquote><p>a</p><h2>q</h2></blockquote>",
    );
    assert_converts(
        "<blockquote><ul><li>x</li></ul><h2>r</h2></blockquote>",
        &underlined(TierStrategy::Tier2),
        "> - x\n>\n> r\n> -\n",
        "<blockquote><ul><li>x</li></ul><h2>r</h2></blockquote>",
    );
}

#[test]
fn should_keep_consecutive_underlined_headings_in_a_quote_compact() {
    assert_converts(
        "<blockquote><h2>q</h2><h2>r</h2></blockquote>",
        &underlined(TierStrategy::Tier2),
        "> q\n> -\n> r\n> -\n",
        "<blockquote><h2>q</h2><h2>r</h2></blockquote>",
    );
}

#[test]
fn should_start_a_heading_after_text_in_a_list_item_in_a_quote_on_its_own_line() {
    let html = "<blockquote><ul><li>a<h2>q</h2></li></ul></blockquote>";
    assert_converts(
        html,
        &options(TierStrategy::Tier2),
        "> - a\n>   ## q\n",
        "<blockquote><ul><li>a<h2>q</h2></li></ul></blockquote>",
    );
    assert_converts(
        html,
        &underlined(TierStrategy::Tier2),
        "> - a\n>\n>   q\n>   --\n",
        "<blockquote><ul><li><p>a</p><h2>q</h2></li></ul></blockquote>",
    );
}

#[test]
fn should_escape_an_underlined_heading_text_that_starts_a_block() {
    for (html, rendered) in [
        ("<h2>-</h2>", "<h2>-</h2>"),
        ("<h2>*</h2>", "<h2>*</h2>"),
        ("<h2>+</h2>", "<h2>+</h2>"),
        ("<h2>1.</h2>", "<h2>1.</h2>"),
        ("<h2>1)</h2>", "<h2>1)</h2>"),
        ("<h1>-</h1>", "<h1>-</h1>"),
        ("<h2>- a</h2>", "<h2>- a</h2>"),
        ("<h2>2. a</h2>", "<h2>2. a</h2>"),
        ("<h2>&gt; a</h2>", "<h2>&gt; a</h2>"),
        ("<h2># a</h2>", "<h2># a</h2>"),
        ("<h2>---</h2>", "<h2>---</h2>"),
        (
            "<blockquote><h2>-</h2></blockquote>",
            "<blockquote><h2>-</h2></blockquote>",
        ),
    ] {
        for strategy in [TierStrategy::Tier2, TierStrategy::Auto] {
            let markdown = convert(html, Some(underlined(strategy)))
                .expect("conversion must succeed")
                .content
                .unwrap_or_default();
            assert_eq!(render(&markdown).replace('\n', ""), rendered, "{html}: {markdown:?}");
        }
    }
    assert_converts("<h2>-</h2>", &underlined(TierStrategy::Tier2), "\\-\n-\n", "<h2>-</h2>");
    assert_converts(
        "<h2>2. a</h2>",
        &underlined(TierStrategy::Tier2),
        "2\\. a\n----\n",
        "<h2>2. a</h2>",
    );
}

#[test]
fn should_keep_an_underlined_heading_whose_text_is_a_list_marker_in_its_list_item() {
    assert_converts(
        "<ul><li><h2>-</h2><p>t</p></li></ul>",
        &underlined(TierStrategy::Tier2),
        "- \\-\n  --\n\n  t\n",
        "<ul><li><h2>-</h2><p>t</p></li></ul>",
    );
    assert_converts(
        "<ul><li><h2>1.</h2><p>t</p></li></ul>",
        &underlined(TierStrategy::Tier2),
        "- 1\\.\n  --\n\n  t\n",
        "<ul><li><h2>1.</h2><p>t</p></li></ul>",
    );
}

#[test]
fn should_leave_heading_text_that_starts_no_block_unescaped() {
    for (html, expected) in [
        ("<h2>-a</h2>", "-a\n--\n"),
        ("<h2>1.5</h2>", "1.5\n---\n"),
        ("<h2>1234567890. a</h2>", "1234567890. a\n-------------\n"),
        ("<h2>12</h2>", "12\n--\n"),
        ("<h3>-</h3>", "### -\n"),
    ] {
        assert_converts(
            html,
            &underlined(TierStrategy::Tier2),
            expected,
            &render(expected).replace('\n', ""),
        );
    }
    let escaped = ConversionOptions {
        escape_misc: true,
        ..underlined(TierStrategy::Tier2)
    };
    assert_converts("<h2>-</h2>", &escaped, "\\-\n--\n", "<h2>-</h2>");
    assert_converts("<h2>-</h2>", &options(TierStrategy::Tier2), "## -\n", "<h2>-</h2>");
}

#[test]
fn should_keep_every_line_of_a_list_item_in_a_quote_at_its_content_column() {
    assert_converts(
        "<ul><li><blockquote><ul><li>a<ul><li><h2>q</h2><p>t</p></li></ul></li></ul></blockquote></li></ul>",
        &underlined(TierStrategy::Tier2),
        "- > * a\n  >   + q\n  >     --\n  >\n  >     t\n",
        "<ul><li><blockquote><ul><li>a<ul><li><h2>q</h2><p>t</p></li></ul></li></ul></blockquote></li></ul>",
    );
    assert_converts(
        "<ol><li><ol><li><blockquote><ul><li>a<ul><li><p>q</p><p>t</p></li></ul></li></ul></blockquote></li></ol></li></ol>",
        &options(TierStrategy::Tier2),
        "1. 1. > - a\n      >   * q\n      >\n      >     t\n",
        "<ol><li><ol><li><blockquote><ul><li>a<ul><li><p>q</p><p>t</p></li></ul></li></ul></blockquote></li></ol></li></ol>",
    );
    assert_converts(
        "<ul><li><blockquote><ul><li><h2>q</h2><p>t</p></li></ul></blockquote></li></ul>",
        &underlined(TierStrategy::Tier2),
        "- > * q\n  >   --\n  >\n  >   t\n",
        "<ul><li><blockquote><ul><li><h2>q</h2><p>t</p></li></ul></blockquote></li></ul>",
    );
}

#[test]
fn should_keep_the_blocks_of_a_list_item_in_a_quote_in_the_item() {
    assert_converts(
        "<blockquote><ul><li>x<blockquote>q</blockquote></li></ul></blockquote>",
        &options(TierStrategy::Tier2),
        "> - x\n>   > q\n",
        "<blockquote><ul><li>x<blockquote><p>q</p></blockquote></li></ul></blockquote>",
    );
    assert_converts(
        "<blockquote><ul><li>x<hr>y</li></ul></blockquote>",
        &options(TierStrategy::Tier2),
        "> - x\n>\n>   ---\n>\n>   y\n",
        "<blockquote><ul><li><p>x</p><hr /><p>y</p></li></ul></blockquote>",
    );
    assert_converts(
        "<blockquote><ul><li>x<dl><dt>t</dt><dd>d</dd></dl></li></ul></blockquote>",
        &options(TierStrategy::Tier2),
        "> - x\n>\n>   t\n>   d\n",
        "<blockquote><ul><li><p>x</p><p>td</p></li></ul></blockquote>",
    );
    assert_converts(
        "<blockquote><ul><li>x<section><p>s</p></section></li></ul></blockquote>",
        &options(TierStrategy::Tier2),
        "> - x\n>\n>   s\n",
        "<blockquote><ul><li><p>x</p><p>s</p></li></ul></blockquote>",
    );
    assert_converts(
        "<blockquote><ul><li><blockquote>q</blockquote>t</li></ul></blockquote>",
        &options(TierStrategy::Tier2),
        "> - > q\n>\n>   t\n",
        "<blockquote><ul><li><blockquote><p>q</p></blockquote><p>t</p></li></ul></blockquote>",
    );
}

#[test]
fn should_keep_every_line_of_a_nested_item_at_its_content_column_with_tab_indent() {
    assert_converts(
        "<ul><li>a<ul><li><h2>q</h2><p>t</p></li></ul></li></ul>",
        &tabs(underlined(TierStrategy::Tier2)),
        "- a\n\t* q\n\t\t--\n\n\t\tt\n",
        "<ul><li>a<ul><li><h2>q</h2><p>t</p></li></ul></li></ul>",
    );
    assert_converts(
        "<ul><li>a<ul><li>b<ul><li><h1>q</h1><p>t</p></li></ul></li></ul></li></ul>",
        &tabs(underlined(TierStrategy::Tier2)),
        "- a\n\t* b\n\t\t+ q\n\t\t\t=\n\n\t\t\tt\n",
        "<ul><li>a<ul><li>b<ul><li><h1>q</h1><p>t</p></li></ul></li></ul></li></ul>",
    );
    assert_converts(
        "<ul><li>a<ul><li><p>q</p><p>t</p></li></ul></li></ul>",
        &tabs(options(TierStrategy::Tier2)),
        "- a\n\t* q\n\n\t\tt\n",
        "<ul><li>a<ul><li><p>q</p><p>t</p></li></ul></li></ul>",
    );
}

#[test]
fn should_start_a_nested_list_at_the_content_column_of_a_wide_ordered_item_with_tab_indent() {
    assert_converts(
        "<ol start=\"1000\"><li>a<ul><li><p>q</p><p>t</p></li></ul></li></ol>",
        &tabs(options(TierStrategy::Tier2)),
        "1000. a\n\t\t- q\n\n\t\t\tt\n",
        "<ol start=\"1000\"><li>a<ul><li><p>q</p><p>t</p></li></ul></li></ol>",
    );
}

#[test]
fn should_keep_the_blocks_of_a_nested_item_between_inline_markers_in_the_item_with_tab_indent() {
    assert_converts(
        "<p>lead <b><ul><li>a<ul><li>b<ul><li>x<blockquote>q</blockquote>t</li></ul></li></ul></li></ul></b> tail</p>",
        &tabs(options(TierStrategy::Tier2)),
        "lead\n\n- **a\n\t* b\n\t\t+ x\n\t\t\t> q\n\n\t\t\tt**\n\ntail\n",
        "<p>lead</p><ul><li>**a<ul><li>b<ul><li><p>x</p><blockquote><p>q</p></blockquote><p>t**</p></li></ul></li></ul></li></ul><p>tail</p>",
    );
}

fn wrapped(options: ConversionOptions) -> ConversionOptions {
    ConversionOptions {
        wrap: true,
        wrap_width: 20,
        ..options
    }
}

#[test]
fn should_keep_text_after_a_nested_quote_in_a_list_in_bold_in_a_quote_out_of_a_code_block() {
    assert_converts(
        "<ul><li>a<ul><li>b<blockquote><b><ul><li>x<blockquote>q</blockquote>t</li></ul></b></blockquote></li></ul></li></ul>",
        &options(TierStrategy::Tier2),
        "- a\n  * b\n    > **+ x\n    >   > q\n    >\n    > t**\n",
        "<ul><li>a<ul><li>b<blockquote><p>**+ x</p><blockquote><p>q</p></blockquote><p>t**</p></blockquote></li></ul></li></ul>",
    );
    assert_converts(
        "<ul><li>a<ul><li>b<ul><li>c<blockquote><b><ul><li>x<blockquote>q</blockquote>t</li></ul></b></blockquote></li></ul></li></ul></li></ul>",
        &options(TierStrategy::Tier2),
        "- a\n  * b\n    + c\n      > **- x\n      >   > q\n      >\n      > t**\n",
        "<ul><li>a<ul><li>b<ul><li>c<blockquote><p>**- x</p><blockquote><p>q</p></blockquote><p>t**</p></blockquote></li></ul></li></ul></li></ul>",
    );
}

#[test]
fn should_keep_text_after_a_nested_quote_in_a_list_in_bold_in_a_quote_out_of_a_code_block_with_tab_indent() {
    let html = "<ul><li>a<ul><li>b<blockquote><b><ul><li>x<blockquote>q</blockquote>t</li></ul></b></blockquote></li></ul></li></ul>";
    for strategy in [TierStrategy::Tier2, TierStrategy::Auto] {
        let markdown = convert(html, Some(tabs(options(strategy))))
            .expect("conversion must succeed")
            .content
            .unwrap_or_default();
        assert_eq!(
            markdown, "- a\n\t* b\n\t\t> **+ x\n\t\t> \t> q\n\t\t> \tt**\n",
            "{strategy:?}"
        );
        assert!(!render(&markdown).contains("<pre>"), "{markdown:?} ({strategy:?})");
    }
}

#[test]
fn should_keep_the_lists_in_a_quote_in_a_list_in_bold_at_their_own_columns() {
    let html = "<ul><li><b>x<ul><li>y<blockquote><ul><li>z<blockquote>q</blockquote>t</li></ul></blockquote>u</li></ul></b></li></ul>";
    let rendered = "<ul><li>**x<ul><li><p>y</p><blockquote><ul><li><p>z</p><blockquote><p>q</p></blockquote><p>t</p></li></ul></blockquote><p>u**</p></li></ul></li></ul>";
    let two_spaces = "- **x\n  * y\n    > + z\n    >   > q\n    >\n    >   t\n\n    u**\n";
    assert_converts(html, &options(TierStrategy::Tier2), two_spaces, rendered);
    assert_converts(html, &wrapped(options(TierStrategy::Tier2)), two_spaces, rendered);
    let with_tabs = "- **x\n\t* y\n\t\t> + z\n\t\t> \t> q\n\t\t>\n\t\t> \tt\n\n\t\tu**\n";
    assert_converts(html, &tabs(options(TierStrategy::Tier2)), with_tabs, rendered);
    assert_converts(html, &tabs(underlined(TierStrategy::Tier2)), with_tabs, rendered);
    assert_converts(
        html,
        &wrapped(tabs(options(TierStrategy::Tier2))),
        "- **x\n\t* y\n\t\t> + z\n\t\t> \t> q\n\t\t>\n\t\t> \tt\n\n\t\tu**\n\n",
        rendered,
    );
    assert_converts(
        html,
        &ConversionOptions {
            list_indent_width: 4,
            ..options(TierStrategy::Tier2)
        },
        "- **x\n    * y\n        > + z\n        >     > q\n        >\n        >     t\n\n        u**\n",
        rendered,
    );
}

#[test]
fn should_not_turn_text_after_a_nested_quote_in_a_list_in_a_caption_into_a_code_block() {
    for html in [
        "<ul><li>a<ul><li>b<table><caption><blockquote><ul><li>x<ul><li>y<blockquote>q</blockquote>t</li></ul></li></ul></blockquote></caption><tr><td>c</td></tr></table></li></ul></li></ul>",
        "<ul><li>a<table><caption>v<ul><li>y<blockquote><ul><li>x<ul><li>y<blockquote>q</blockquote>t</li></ul></li></ul></blockquote>u</li></ul></caption><tr><td>c</td></tr></table></li></ul>",
    ] {
        for options in [options(TierStrategy::Tier2), tabs(options(TierStrategy::Tier2))] {
            let markdown = convert(html, Some(options))
                .expect("conversion must succeed")
                .content
                .unwrap_or_default();
            assert!(!render(&markdown).contains("<pre>"), "{html}: {markdown:?}");
        }
    }
}
