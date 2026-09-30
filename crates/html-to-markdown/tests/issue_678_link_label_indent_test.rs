// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for the line after a hard break inside a link in a list item (issue #678):
//! the line keeps the list item's indent, so it stays in the item and the link stays one link.

use html_to_markdown_rs::options::{ListIndentType, NewlineStyle, OutputFormat};
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn options(tier_strategy: TierStrategy) -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        tier_strategy,
        ..ConversionOptions::default()
    }
}

fn convert_with(html: &str, options: ConversionOptions) -> String {
    convert(html, Some(options))
        .expect("conversion must succeed")
        .content
        .unwrap_or_default()
}

fn render(markdown: &str) -> String {
    comrak::markdown_to_html(markdown, &comrak::Options::default())
}

fn count(html: &str, tag: &str) -> usize {
    html.matches(&format!("<{tag}>")).count() + html.matches(&format!("<{tag} ")).count()
}

/// Asserts the exact output, and that it renders the link once with its line break and no list
/// beyond the ones in `html`.
fn assert_link_kept(html: &str, options: ConversionOptions, expected: &str) {
    let label = format!(
        "{:?} {:?} {:?}",
        options.tier_strategy, options.list_indent_type, options.newline_style
    );
    let markdown = convert_with(html, options);
    assert_eq!(markdown, expected, "{html} with {label}");
    let rendered = render(&markdown);
    assert_eq!(count(&rendered, "a"), 1, "{markdown:?} renders {rendered:?}");
    assert!(rendered.contains("<br />"), "{markdown:?} renders {rendered:?}");
    for tag in ["ul", "ol", "li", "blockquote"] {
        assert_eq!(
            count(&rendered, tag),
            count(html, tag),
            "{tag}: {markdown:?} renders {rendered:?}"
        );
    }
}

#[test]
fn should_indent_a_link_label_line_to_the_list_item_content_column() {
    let html = r#"<ul><li><a href="u">a<br>2. t</a></li></ul>"#;
    for tier in [TierStrategy::Tier1, TierStrategy::Tier2, TierStrategy::Auto] {
        assert_link_kept(html, options(tier), "- [a  \n  2. t](u)\n");
    }
}

#[test]
fn should_indent_every_label_line_in_nested_items_ordered_items_and_quotes() {
    let cases = [
        (
            r#"<ul><li>q<ul><li><a href="u">a<br>2. t</a></li></ul></li></ul>"#,
            "- q\n  * [a  \n    2. t](u)\n",
        ),
        (
            r#"<ol start="10"><li><a href="u">a<br>2. t</a></li></ol>"#,
            "10. [a  \n    2. t](u)\n",
        ),
        (
            r#"<ol start="100"><li><a href="u">a<br>2. t</a></li></ol>"#,
            "100. [a  \n     2. t](u)\n",
        ),
        (
            r#"<ol start="10"><li>q<ul><li><a href="u">a<br>2. t</a></li></ul></li></ol>"#,
            "10. q\n    - [a  \n      2. t](u)\n",
        ),
        (
            r#"<ul><li><a href="u">a<br>b<br>2. t<br>c</a></li></ul>"#,
            "- [a  \n  b  \n  2. t  \n  c](u)\n",
        ),
        (
            r#"<blockquote><ul><li><a href="u">a<br>2. t</a></li></ul></blockquote>"#,
            "> - [a  \n>   2. t](u)\n",
        ),
        (
            r#"<ul><li><b><a href="u">a<br>2. t</a></b></li></ul>"#,
            "- **[a  \n  2. t](u)**\n",
        ),
        (
            r#"<ul><li><a href="u"><b>a<br>2. t</b></a></li></ul>"#,
            "- [**a  \n  2. t**](u)\n",
        ),
        (r#"<ul><li><a href="u">a<br>- t</a></li></ul>"#, "- [a  \n  \\- t](u)\n"),
    ];
    for (html, expected) in cases {
        for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
            assert_link_kept(html, options(tier), expected);
        }
    }
}

#[test]
fn should_indent_a_label_line_with_tabs_a_wider_indent_and_backslash_breaks() {
    let html = r#"<ul><li>q<ul><li><a href="u">a<br>2. t</a></li></ul></li></ul>"#;
    let tabs = ConversionOptions {
        list_indent_type: ListIndentType::Tabs,
        ..options(TierStrategy::Tier2)
    };
    assert_link_kept(html, tabs, "- q\n\t* [a  \n\t\t2. t](u)\n");
    let wide = ConversionOptions {
        list_indent_width: 4,
        ..options(TierStrategy::Tier2)
    };
    assert_link_kept(html, wide, "- q\n    * [a  \n        2. t](u)\n");
    let backslash = ConversionOptions {
        newline_style: NewlineStyle::Backslash,
        ..options(TierStrategy::Tier2)
    };
    assert_link_kept(html, backslash, "- q\n  * [a\\\n    2. t](u)\n");
}

#[test]
fn should_indent_a_label_line_in_djot_output() {
    let djot = ConversionOptions {
        output_format: OutputFormat::Djot,
        ..options(TierStrategy::Tier2)
    };
    let markdown = convert_with(r#"<ul><li><a href="u">a<br>2. t</a></li></ul>"#, djot);
    assert_eq!(markdown, "- [a\\\n  2. t](u)\n");
}

#[test]
fn should_leave_a_label_line_outside_a_list_unindented() {
    for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
        assert_link_kept(r#"<p><a href="u">a<br>2. t</a></p>"#, options(tier), "[a  \n2. t](u)\n");
        assert_link_kept(
            r#"<blockquote><a href="u">a<br>2. t</a></blockquote>"#,
            options(tier),
            "> [a  \n> 2. t](u)\n",
        );
    }
}

#[test]
fn should_indent_a_label_line_of_a_link_only_item_with_wrap_on() {
    let cases = [
        (r#"<ul><li><a href="u">a<br>2. t</a></li></ul>"#, "- [a  \n  2. t](u)\n"),
        (
            r#"<ul><li>q<ul><li><a href="u"><b>a<br>2. t</b></a></li></ul></li></ul>"#,
            "- q\n  * [**a  \n    2. t**](u)\n",
        ),
    ];
    for (html, expected) in cases {
        for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
            let wrap = ConversionOptions {
                wrap: true,
                wrap_width: 20,
                ..options(tier)
            };
            assert_link_kept(html, wrap, expected);
        }
    }
}

fn djot() -> ConversionOptions {
    ConversionOptions {
        output_format: OutputFormat::Djot,
        ..options(TierStrategy::Tier2)
    }
}

#[test]
fn should_write_a_djot_hard_break_as_a_backslash() {
    assert_eq!(convert_with("<p>a<br>b</p>", djot()), "a\\\nb\n");
    assert_eq!(convert_with("<ul><li>a<br>b</li></ul>", djot()), "- a\\\n  b\n");
    assert_eq!(
        convert_with(r#"<p><a href="u">a<br>b</a></p>"#, djot()),
        "[a\\\nb](u)\n"
    );
    let spaces = ConversionOptions {
        newline_style: NewlineStyle::Spaces,
        ..djot()
    };
    assert_eq!(convert_with("<p>a<br>b</p>", spaces), "a\\\nb\n");
}

#[test]
fn should_escape_each_dash_and_backtick_that_starts_a_djot_line() {
    let cases = [
        ("<p>a<br>---</p>", "a\\\n\\-\\-\\-\n"),
        ("<p>a<br><span>-- t</span></p>", "a\\\n\\-\\- t\n"),
        (
            r#"<ol start="10"><li><a href="u">a<br><span>```</span></a></li></ol>"#,
            "10. [a\\\n    \\`\\`\\`](u)\n",
        ),
        (r#"<ul><li><a href="u">a<br>`x</a></li></ul>"#, "- [a\\\n  \\`x](u)\n"),
        ("<p>a<br>- t</p>", "a\\\n- t\n"),
    ];
    for (html, expected) in cases {
        assert_eq!(convert_with(html, djot()), expected, "{html}");
    }
}

#[test]
fn should_keep_a_link_whole_across_two_hard_breaks() {
    let html = r#"<ul><li><a href="u">a<br><br>b</a></li></ul>"#;
    for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
        assert_link_kept(html, options(tier), "- [a  \n  \\\n  b](u)\n");
    }
    assert_eq!(convert_with(html, djot()), "- [a\\\n\\\n  b](u)\n");
}

#[test]
fn should_escape_a_line_start_that_an_inline_element_writes_after_a_hard_break() {
    let tags = [
        "abbr", "sub", "sup", "label", "span", "small", "cite", "u", "bdi", "time", "x-tag",
    ];
    for tag in tags {
        let html = format!("<p>a<br><{tag}>- t</{tag}></p>");
        for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
            let markdown = convert_with(&html, options(tier));
            assert!(markdown.starts_with("a  \n\\- t"), "{html} {tier:?}: {markdown:?}");
        }
    }
    let blocks = [
        "<p>a<br><ruby>- t<rt>r</rt></ruby></p>",
        "<p>a<br><mark>=</mark></p>",
        "<p>a<br><ins>=</ins></p>",
        "<p>a<br><kbd>```</kbd></p>",
        "<ul><li>a<br><kbd>```</kbd></li></ul>",
    ];
    for html in blocks {
        for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
            let markdown = convert_with(html, options(tier));
            let rendered = render(&markdown);
            for tag in ["h1", "h2", "pre", "hr"] {
                assert_eq!(count(&rendered, tag), 0, "{html} {tier:?}: {markdown:?} {rendered}");
            }
            let items = count(&rendered, "li");
            assert_eq!(items, count(html, "li"), "{html} {tier:?}: {markdown:?} {rendered}");
        }
    }
}

#[test]
fn should_indent_an_inline_element_line_after_a_hard_break_in_a_djot_list_item() {
    let cases = [
        ("<ul><li>a<br><abbr>- t</abbr></li></ul>", "- a\\\n  - t\n"),
        ("<ul><li>a<br><label># t</label></li></ul>", "- a\\\n  # t\n"),
        ("<ul><li>a<br><b>---</b></li></ul>", "- a\\\n  *\\-\\-\\-*\n"),
    ];
    for (html, expected) in cases {
        assert_eq!(convert_with(html, djot()), expected, "{html}");
    }
}

#[test]
fn should_keep_a_paragraph_whole_across_two_hard_breaks() {
    let cases = [
        ("<p><b>a<br><br>b</b></p>", "**a  \n  \\\nb**\n"),
        ("<p>a<br><br><br>b</p>", "a  \n  \\\n  \\\nb\n"),
        ("<ul><li><em>a<br><br>b</em></li></ul>", "- *a  \n    \\\n  b*\n"),
        ("<p>a<br><br><i>b</i></p>", "a  \n  \\\n*b*\n"),
        ("<p>a<br><br><a href=\"u\">b</a></p>", "a  \n  \\\n[b](u)\n"),
        ("<p>a<br><br><img src=\"u\" alt=\"i\"></p>", "a  \n  \\\n![i](u)\n"),
    ];
    for (html, expected) in cases {
        for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
            let markdown = convert_with(html, options(tier));
            assert_eq!(markdown, expected, "{html} {tier:?}");
            let rendered = render(&markdown);
            assert_eq!(count(&rendered, "p"), count(html, "p"), "{html} {tier:?}: {rendered}");
            assert!(!rendered.contains('\\'), "{html} {tier:?}: {rendered}");
        }
    }
    let trailing_runs = [
        "<p>a<br><br></p><p>b</p>",
        "<p>a<br><br><b></b></p><p>b</p>",
        "<p>a<br><br><span></span></p><p>b</p>",
    ];
    for html in trailing_runs {
        for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
            let rendered = render(&convert_with(html, options(tier)));
            assert_eq!(count(&rendered, "p"), 2, "{html} {tier:?}: {rendered}");
            assert!(!rendered.contains('\\'), "{html} {tier:?}: {rendered}");
        }
    }
    let quote = "<blockquote><b>a<br><br>b</b></blockquote>";
    let tier1 = convert_with(quote, options(TierStrategy::Tier1));
    assert_eq!(tier1, convert_with(quote, options(TierStrategy::Tier2)));
    assert_eq!(count(&render(&tier1), "p"), 1, "{tier1:?}");
    assert_eq!(count(&render(&tier1), "br"), 2, "{tier1:?}");
}

#[test]
fn should_keep_the_text_and_the_link_of_a_backtick_pair_after_a_djot_break() {
    assert_eq!(
        convert_with(r#"<p><a href="u">a<br>`x`</a></p>"#, djot()),
        "[a\\\n\\`x\\`](u)\n"
    );
    assert_eq!(convert_with("<p>a<br>`x`</p>", djot()), "a\\\n\\`x\\`\n");
}

#[test]
fn should_escape_djot_dashes_and_backticks_wherever_they_stand() {
    let cases = [
        ("<p>---</p>", "\\-\\-\\-\n"),
        ("<ul><li>---</li></ul>", "- \\-\\-\\-\n"),
        ("<p>a --- b</p>", "a \\-\\-\\- b\n"),
        ("<p>-- a</p>", "\\-\\- a\n"),
        ("<p>a<br>-<span>--</span></p>", "a\\\n-\\-\\-\n"),
        ("<p>a<br><sub>---</sub></p>", "a\\\n~\\-\\-\\-~\n"),
        ("<p>a<br><b>```</b></p>", "a\\\n*\\`\\`\\`*\n"),
        ("<p>a-b - c</p>", "a-b - c\n"),
        ("<p><code>a--b</code></p>", "`a--b`\n"),
        ("<p>a<span>-</span>-b</p>", "a-\\-b\n"),
    ];
    for (html, expected) in cases {
        assert_eq!(convert_with(html, djot()), expected, "{html}");
    }
    let escape_misc = ConversionOptions {
        escape_misc: true,
        ..djot()
    };
    assert_eq!(convert_with("<p>a--b `x`</p>", escape_misc), "a\\-\\-b \\`x\\`\n");
}

#[test]
fn should_convert_an_inline_element_that_merges_into_the_one_before_it() {
    let markdown = convert_with("<p><em>a</em><em>é</em></p>", options(TierStrategy::Tier2));
    assert_eq!(markdown, "*aé*\n");
}

#[test]
fn should_leave_a_block_inside_an_inline_element_that_no_break_precedes_as_a_block() {
    let cases = [
        r#"<ol start="2"><blockquote><br></blockquote><b><hr></b></ol>"#,
        r#"<ol start="2"><blockquote><span></span><br></blockquote><b><hr></b><pre>x</pre></ol>"#,
        r#"<ul><li><input type="checkbox"><b></b><li></li></li><ol start="2"><blockquote><span></span><br></blockquote><b><hr></b></ol></ul>"#,
        r#"<i>x<hr><br>x<ol start="2"><li><b><p></p><ul><ul>t</ul><b><hr><hr></b>1. y</ul>t</b></li></ol></i>"#,
        "<blockquote><p><li><br> <br></li>t</p></blockquote>",
        r#"<ol start="3"><li><br><q><blockquote>2. z</blockquote></q></li></ol>"#,
        r#"<ol start="3"><li>a<br><i><blockquote>b</blockquote></i></li></ol>"#,
        "<p>a<br><i><h2>t</h2></i></p>",
        "<ul><li>a<br><b><pre>x</pre></b></li></ul>",
        "<p>a<br><span><ul><li>b</li></ul></span></p>",
    ];
    for html in cases {
        for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
            let markdown = convert_with(html, options(tier));
            assert!(!markdown.contains('\\'), "{html} {tier:?}: {markdown:?}");
        }
    }
}

#[test]
fn should_write_a_break_at_the_end_of_an_inline_element_outside_its_closing_marker() {
    for html in [
        "<p><b>a<br> </b>b</p>",
        "<p><b>a<br><br></b>b</p>",
        "<p><em>a<br> </em>b</p>",
    ] {
        for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
            let markdown = convert_with(
                html,
                ConversionOptions {
                    newline_style: NewlineStyle::Backslash,
                    ..options(tier)
                },
            );
            let rendered = render(&markdown);
            assert!(
                rendered.contains("<strong>a</strong>") || rendered.contains("<em>a</em>"),
                "{html} {tier:?}: {markdown:?} renders {rendered:?}"
            );
            assert!(!markdown.contains("\\*"), "{html} {tier:?}: {markdown:?}");
        }
        let djot = convert_with(
            html,
            ConversionOptions {
                output_format: OutputFormat::Djot,
                ..options(TierStrategy::Tier2)
            },
        );
        assert!(!djot.contains("\\*") && !djot.contains("\\_"), "{html} Djot: {djot:?}");
    }
}

#[test]
fn should_keep_the_text_after_breaks_at_an_item_start_in_the_item() {
    for (html, lists) in [
        ("<ol><li><br><br>x</li></ol>", "ol"),
        ("<ol start=\"10\"><li><br><br>x</li></ol>", "ol"),
        ("<ul><li><br><br>x</li></ul>", "ul"),
        ("<ul><li><br>2. z</li></ul>", "ul"),
    ] {
        for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
            let markdown = convert_with(html, options(tier));
            let rendered = render(&markdown);
            assert_eq!(
                count(&rendered, lists),
                1,
                "{html} {tier:?}: {markdown:?} renders {rendered:?}"
            );
            assert_eq!(
                count(&rendered, "li"),
                1,
                "{html} {tier:?}: {markdown:?} renders {rendered:?}"
            );
            assert_eq!(
                count(&rendered, "p"),
                0,
                "{html} {tier:?}: {markdown:?} renders {rendered:?}"
            );
            assert!(
                rendered.trim_end().ends_with("</li>\n</ol>") || rendered.trim_end().ends_with("</li>\n</ul>"),
                "{html} {tier:?}: {markdown:?} renders {rendered:?}"
            );
            assert!(
                !rendered.contains("<pre>"),
                "{html} {tier:?}: {markdown:?} renders {rendered:?}"
            );
        }
    }
}
