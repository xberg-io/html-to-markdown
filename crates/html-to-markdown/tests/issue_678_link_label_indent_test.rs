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
    assert_eq!(markdown, "- [a  \n  2. t](u)\n");
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
