// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for a nested list whose first item has nothing on its marker line (issue
//! #667), and for such an item after text inside its list. Such a line cannot interrupt the text
//! before it, and a lone `-` under text is a heading underline, so the item starts after a blank
//! line. A line of bare markers that an empty item would complete as a thematic break moves that
//! item's marker to the next line.

use html_to_markdown_rs::options::ListIndentType;
use html_to_markdown_rs::{ConversionOptions, OutputFormat, TierStrategy, convert};

fn tier2_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        tier_strategy: TierStrategy::Tier2,
        ..ConversionOptions::default()
    }
}

fn convert_with(html: &str, options: &ConversionOptions) -> String {
    convert(html, Some(options.clone()))
        .expect("conversion must succeed")
        .content
        .unwrap_or_default()
}

fn render(markdown: &str) -> String {
    comrak::markdown_to_html(markdown, &comrak::Options::default())
}

/// Convert `html`, assert the exact markdown, and assert that its rendering holds `items` list
/// items and no heading or rule.
fn assert_items(html: &str, options: &ConversionOptions, expected: &str, items: usize) {
    let markdown = convert_with(html, options);
    assert_eq!(markdown, expected, "{html}");
    let rendered = render(&markdown);
    assert_eq!(rendered.matches("<li").count(), items, "{html}: {rendered:?}");
    assert!(
        !rendered.contains("<h2") && !rendered.contains("<hr"),
        "{html}: {rendered:?}"
    );
}

#[test]
fn should_start_a_nested_list_whose_first_item_is_empty_after_a_blank_line() {
    for strategy in [TierStrategy::Tier2, TierStrategy::Auto, TierStrategy::Tier1] {
        let options = ConversionOptions {
            tier_strategy: strategy,
            ..tier2_options()
        };
        assert_items("<ol><li>a<ul><li></li></ul></li></ol>", &options, "1. a\n\n   -\n", 2);
        assert_items("<ul><li>a<ul><li></li></ul></li></ul>", &options, "- a\n\n  *\n", 2);
        assert_items("<ul><li>a<ol><li></li></ol></li></ul>", &options, "- a\n\n  1.\n", 2);
        assert_items("<ol><li>a<ol><li></li></ol></li></ol>", &options, "1. a\n\n   1.\n", 2);
        assert_items(
            "<ul><li>a<ol><li></li><li>b</li></ol></li></ul>",
            &options,
            "- a\n\n  1.\n  2. b\n",
            3,
        );
        assert_items(
            "<ul><li>a<ul><li></li></ul></li><li>c</li></ul>",
            &options,
            "- a\n\n  *\n- c\n",
            3,
        );
    }
}

#[test]
fn should_treat_an_item_that_writes_nothing_on_its_marker_line_as_empty() {
    let options = tier2_options();
    for inner in [" ", "\t", "<span></span>", "<div></div>", "<p></p>", "<!-- c -->"] {
        assert_items(
            &format!("<ul><li>a<ul><li>{inner}</li></ul></li></ul>"),
            &options,
            "- a\n\n  *\n",
            2,
        );
    }
    assert_items(
        "<ul><li>a<ul><li><br>b</li></ul></li></ul>",
        &options,
        "- a\n\n  *  \n    b\n",
        2,
    );
    assert_items(
        "<ul><li><b>a</b><ul><li></li></ul></li></ul>",
        &options,
        "- **a**\n\n  *\n",
        2,
    );
}

#[test]
fn should_start_an_empty_nested_item_after_a_blank_line_in_a_quote_and_with_other_indents() {
    assert_items(
        "<blockquote><ul><li>a<ul><li></li></ul></li></ul></blockquote>",
        &tier2_options(),
        "> - a\n>\n>   *\n",
        2,
    );
    let tabs = ConversionOptions {
        list_indent_type: ListIndentType::Tabs,
        ..tier2_options()
    };
    assert_items("<ul><li>a<ul><li></li></ul></li></ul>", &tabs, "- a\n\n\t*\n", 2);
    let width = ConversionOptions {
        list_indent_width: 4,
        ..tier2_options()
    };
    assert_items("<ul><li>a<ul><li></li></ul></li></ul>", &width, "- a\n\n    *\n", 2);
    assert_items("<ul><li>a<ul><li>b</li></ul></li></ul>", &tabs, "- a\n\t* b\n", 2);
    assert_items("<ul><li>a<ul><li>b</li></ul></li></ul>", &width, "- a\n    * b\n", 2);
    let dash = ConversionOptions {
        bullets: "-".into(),
        ..tier2_options()
    };
    assert_items("<ul><li>a<ul><li></li></ul></li></ul>", &dash, "- a\n\n  -\n", 2);
}

#[test]
fn should_keep_a_nested_item_on_the_line_after_its_text_when_it_can_interrupt_it() {
    let options = tier2_options();
    assert_items("<ul><li>a<ul><li>b</li></ul></li></ul>", &options, "- a\n  * b\n", 2);
    assert_items(
        "<ul><li>a<ul><li><img src=\"x.png\"></li></ul></li></ul>",
        &options,
        "- a\n  * ![](x.png)\n",
        2,
    );
    assert_items(
        "<ul><li>a<ul><li><ul><li></li></ul></li></ul></li></ul>",
        &options,
        "- a\n  * +\n",
        3,
    );
    // ~keep A block before the list already ends the paragraph.
    assert_items(
        "<ul><li><p>a</p><ul><li></li></ul></li></ul>",
        &options,
        "- a\n\n  *\n",
        2,
    );
    assert_items(
        "<ul><li>a<blockquote>q</blockquote><ul><li></li></ul></li></ul>",
        &options,
        "- a\n  > q\n  *\n",
        2,
    );
    // ~keep A list between inline markers is indented past the item's content column.
    assert_eq!(
        convert_with(
            "<div><mark><ol start=\"10\"><li>x<ul><li>n</li></ul>t</li></ol></mark></div>",
            &options
        ),
        "==10. x\n    - n\n    t==\n"
    );
    // ~keep Only the list's first item follows the text.
    assert_items(
        "<ul><li>a<ul><li>b</li><li></li></ul></li></ul>",
        &options,
        "- a\n  * b\n  *\n",
        3,
    );
}

#[test]
fn should_start_an_empty_item_after_text_inside_its_list_after_a_blank_line() {
    for strategy in [TierStrategy::Tier2, TierStrategy::Auto, TierStrategy::Tier1] {
        let options = ConversionOptions {
            tier_strategy: strategy,
            ..tier2_options()
        };
        assert_items("<ul>t<li></li></ul>", &options, "t\n\n-\n", 1);
        assert_items("<ol>t<li></li></ol>", &options, "t\n\n1.\n", 1);
        assert_items(
            "<blockquote><ul>t<li></li></ul></blockquote>",
            &options,
            "> t\n>\n> -\n",
            1,
        );
    }
    let options = tier2_options();
    assert_items("<ul><span>t</span><li></li></ul>", &options, "t\n\n-\n", 1);
    assert_items("<ul>t<li> </li><li>b</li></ul>", &options, "t\n\n-\n- b\n", 2);
    assert_items(
        "<ul><li>a<ul>t<li></li></ul></li></ul>",
        &options,
        "- a\n  t\n\n  *\n",
        2,
    );
    assert_items("<ol>t<li></li></ol>", &options, "t\n\n1.\n", 1);
    // ~keep An item with content interrupts the text.
    assert_items("<ul>t<li>b</li></ul>", &options, "t\n- b\n", 1);
}

#[test]
fn should_keep_an_empty_nested_item_between_inline_markers_as_text() {
    let options = tier2_options();
    assert_eq!(
        convert_with(
            "<div><mark><ol start=\"10\"><li>x<ul><li></li></ul>t</li></ol></mark></div>",
            &options
        ),
        "==10. x\n    -\n    t==\n"
    );
    let tabs = ConversionOptions {
        list_indent_type: ListIndentType::Tabs,
        ..tier2_options()
    };
    let width = ConversionOptions {
        list_indent_width: 4,
        ..tier2_options()
    };
    for options in [&options, &tabs, &width] {
        for html in [
            "<mark><ul><li>a<ul><li></li></ul></li></ul></mark>",
            "<mark><ul>t<li></li></ul></mark>",
        ] {
            let markdown = convert_with(html, options);
            assert!(!markdown.contains("\n\n"), "{html}: {markdown:?}");
            assert!(!render(&markdown).contains("<pre"), "{html}: {markdown:?}");
        }
    }
}

#[test]
fn should_write_one_blank_line_before_an_empty_nested_item_in_djot() {
    let options = ConversionOptions {
        output_format: OutputFormat::Djot,
        ..tier2_options()
    };
    assert_eq!(
        convert_with("<ul><li>a<ul><li></li></ul></li></ul>", &options),
        "- a\n\n  *\n"
    );
}

#[test]
fn should_start_an_empty_item_that_completes_a_thematic_break_on_the_next_line() {
    for bullets in ["-", "*"] {
        let options = ConversionOptions {
            bullets: bullets.into(),
            ..tier2_options()
        };
        let b = bullets;
        assert_items(
            "<ul><li><ul><li><ul><li></li></ul></li></ul></li></ul>",
            &options,
            &format!("{b} {b}\n    {b}\n"),
            3,
        );
        assert_items(
            "<ul><li><ul><li><ul><li><ul><li></li></ul></li></ul></li></ul></li></ul>",
            &options,
            &format!("{b} {b}\n    {b}\n      {b}\n"),
            4,
        );
        assert_items(
            "<ul><li>a<ul><li><ul><li><ul><li></li></ul></li></ul></li></ul></li></ul>",
            &options,
            &format!("{b} a\n  {b} {b}\n      {b}\n"),
            4,
        );
        assert_items(
            "<ol><li><ul><li><ul><li><ul><li></li></ul></li></ul></li></ul></li></ol>",
            &options,
            &format!("1. {b} {b}\n       {b}\n"),
            4,
        );
        assert_items(
            "<ul><li><ul><li><ul><li>x</li></ul></li></ul></li></ul>",
            &options,
            &format!("{b} {b} {b} x\n"),
            3,
        );
        // ~keep The moved marker keeps the column it has on the line of markers.
        let width = ConversionOptions {
            list_indent_width: 4,
            ..options.clone()
        };
        assert_items(
            "<ul><li><ul><li><ul><li></li></ul></li></ul></li></ul>",
            &width,
            &format!("{b} {b}\n    {b}\n"),
            3,
        );
    }
}

#[cfg(feature = "testkit")]
#[test]
fn should_hand_an_empty_nested_item_to_the_full_converter() {
    use html_to_markdown_rs::prescan::PrescanReport;
    use html_to_markdown_rs::tier1;
    use html_to_markdown_rs::tier1::BailReason;

    let options = tier2_options();
    for html in [
        "<ul><li>a<ul><li></li></ul></li></ul>",
        "<ul><li>a<ul><li><br>b</li></ul></li></ul>",
        "<ul><li>a<ul><li>b</li><li></li></ul></li></ul>",
        "<blockquote><ul><li>a<ul><li></li></ul></li></ul></blockquote>",
        "<ul><li>a<ul><li><li>b</ul></li></ul>",
        "<ul>t<li></li></ul>",
    ] {
        let result = tier1::run(html, &PrescanReport::default(), &options);
        assert!(
            matches!(result, Err(BailReason::EmptyNestedListItem)),
            "{html}: {result:?}"
        );
    }
    for html in [
        "<ul><li></li></ul>",
        "<ul><li>a<ul><li>b</li></ul></li></ul>",
        "<ul><li>a</li><li></li></ul>",
        "<ul>t<li>b</li></ul>",
    ] {
        let result = tier1::run(html, &PrescanReport::default(), &options);
        assert!(result.is_ok(), "{html}: {result:?}");
    }
}
