// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for ordered list markers: a task item in an ordered list keeps its number
//! (issue #659), and a nested ordered list that does not start at 1 starts after a blank line
//! when it follows the text of its item (issue #662).

use html_to_markdown_rs::options::ListIndentType;
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

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
    let mut options = comrak::Options::default();
    options.extension.tasklist = true;
    comrak::markdown_to_html(markdown, &options)
}

/// Convert `html` with `options`, assert the exact markdown, and assert that its rendering holds
/// every one of `rendered`.
fn assert_converts(html: &str, options: &ConversionOptions, expected: &str, rendered: &[&str]) {
    let markdown = convert_with(html, options);
    assert_eq!(markdown, expected, "{html}");
    let html_out = render(&markdown);
    for fragment in rendered {
        assert!(html_out.contains(fragment), "{html}: {fragment:?} not in {html_out:?}");
    }
}

#[test]
fn should_keep_the_number_of_a_task_item_in_an_ordered_list() {
    for strategy in [TierStrategy::Tier2, TierStrategy::Auto, TierStrategy::Tier1] {
        let options = ConversionOptions {
            tier_strategy: strategy,
            ..tier2_options()
        };
        assert_converts(
            r#"<ol><li><input type="checkbox">p</li></ol>"#,
            &options,
            "1. [ ] p\n",
            &["<ol>", r#"<input type="checkbox" disabled="" /> p"#],
        );
    }
    let options = tier2_options();
    assert_converts(
        r#"<ol start="10"><li><input type="checkbox" checked>p</li><li>q</li></ol>"#,
        &options,
        "10. [x] p\n11. q\n",
        &[r#"<ol start="10">"#, "checked", "<li>q</li>"],
    );
    // ~keep Text in the list before the item: the numbered task marker cannot interrupt it.
    assert_converts(
        r#"<ol start="3">how<li><input type="checkbox">do</li></ol>"#,
        &options,
        "how\n\n3. [ ] do\n",
        &["<p>how</p>", r#"<ol start="3">"#],
    );
    assert_converts(
        r#"<ul><li><input type="checkbox">p</li></ul>"#,
        &options,
        "- [ ] p\n",
        &["<ul>", "checkbox"],
    );
}

#[test]
fn should_start_the_content_of_an_ordered_task_item_at_its_content_column() {
    let options = tier2_options();
    assert_converts(
        r#"<ol start="9"><li><input type="checkbox">a<ul><li>b</li></ul></li><li><input type="checkbox">c<ol><li>d</li></ol></li></ol>"#,
        &options,
        "9. [ ] a\n   - b\n10. [ ] c\n    1. d\n",
        &[r#"<ol start="9">"#, "<li>b</li>", "<li>d</li>"],
    );
    assert_converts(
        r#"<ol start="10"><li><input type="checkbox"><blockquote>q</blockquote></li></ol>"#,
        &options,
        "10. [ ]\n    > q\n",
        &[r#"<ol start="10">"#, "<blockquote>\n<p>q</p>"],
    );
    assert_converts(
        r#"<ol start="10"><li><input type="checkbox"><p>a</p><p>b</p></li></ol>"#,
        &options,
        "10. [ ] a\n\n    b\n",
        &[r#"<ol start="10">"#, "<p>b</p>\n</li>"],
    );
    let tabs = ConversionOptions {
        list_indent_type: ListIndentType::Tabs,
        ..tier2_options()
    };
    assert_converts(
        r#"<ol start="10"><li><input type="checkbox">a<ul><li>b</li></ul></li></ol>"#,
        &tabs,
        "10. [ ] a\n\t- b\n",
        &["<li>b</li>"],
    );
}

#[test]
fn should_start_a_nested_ordered_list_that_does_not_start_at_one_after_a_blank_line() {
    for strategy in [TierStrategy::Tier2, TierStrategy::Auto, TierStrategy::Tier1] {
        let options = ConversionOptions {
            tier_strategy: strategy,
            ..tier2_options()
        };
        assert_converts(
            r#"<ol start="10"><li>a<ol start="100"><li>q</li></ol></li></ol>"#,
            &options,
            "10. a\n\n    100. q\n",
            &[r#"<ol start="100">"#, "<li>q</li>"],
        );
    }
    let options = tier2_options();
    assert_converts(
        r#"<ul><li>a<ol start="2"><li>q</li><li>r</li><li>s</li></ol></li></ul>"#,
        &options,
        "- a\n\n  2. q\n  3. r\n  4. s\n",
        &[r#"<ol start="2">"#, "<li>q</li>\n<li>r</li>\n<li>s</li>"],
    );
    assert_converts(
        r#"<ul><li>a<ol start="0"><li>q</li></ol></li></ul>"#,
        &options,
        "- a\n\n  0. q\n",
        &[r#"<ol start="0">"#],
    );
    assert_converts(
        r#"<ul><li>a<br><ol start="3"><li>c</li></ol></li></ul>"#,
        &options,
        "- a  \n\n  3. c\n",
        &[r#"<ol start="3">"#],
    );
    assert_converts(
        r#"<ul><li><b>a</b><ol start="3"><li>c</li></ol></li></ul>"#,
        &options,
        "- **a**\n\n  3. c\n",
        &[r#"<ol start="3">"#],
    );
    assert_converts(
        r#"<ul><li>a<ol start="3"><li>c<ol start="7"><li>d</li></ol></li></ol></li></ul>"#,
        &options,
        "- a\n\n  3. c\n\n     7. d\n",
        &[r#"<ol start="3">"#, r#"<ol start="7">"#],
    );
    assert_converts(
        r#"<ol start="10"><li><input type="checkbox">a<ol start="100"><li>q</li></ol></li></ol>"#,
        &options,
        "10. [ ] a\n\n    100. q\n",
        &[r#"<ol start="100">"#],
    );
    // ~keep Only the first item can interrupt: the lines of an item before the next one are no
    // ~keep paragraph of the item around the list.
    assert_converts(
        r#"<ul><li>a<ol start="3"><li>x<br>y</li><li>z</li></ol></li></ul>"#,
        &options,
        "- a\n\n  3. x  \n     y\n  4. z\n",
        &["<li>z</li>"],
    );
    let tabs = ConversionOptions {
        list_indent_type: ListIndentType::Tabs,
        ..tier2_options()
    };
    assert_converts(
        r#"<ol start="10"><li>a<ol start="100"><li>q</li></ol></li></ol>"#,
        &tabs,
        "10. a\n\n\t100. q\n",
        &[r#"<ol start="100">"#],
    );
}

#[test]
fn should_keep_a_nested_list_that_can_interrupt_the_text_or_follows_no_text_on_the_next_line() {
    let options = tier2_options();
    for (html, expected) in [
        ("<ul><li>a<ol><li>q</li></ol></li></ul>", "- a\n  1. q\n"),
        ("<ul><li>a<ul><li>q</li></ul></li></ul>", "- a\n  * q\n"),
        (
            r#"<ul><li>a<ul><li>b</li></ul><ol start="3"><li>c</li></ol></li></ul>"#,
            "- a\n  * b\n  3. c\n",
        ),
        (r#"<ul><li><ol start="3"><li>x</li></ol></li></ul>"#, "- 3. x\n"),
        (r#"<ul><li>a</li><ol start="3"><li>c</li></ol></ul>"#, "- a\n3. c\n"),
        (
            r#"<ol start="3"><li>a<br>b</li><li>c</li></ol>"#,
            "3. a  \n   b\n4. c\n",
        ),
        (
            r#"<ul><li>a<ol start="3">how<li>do</li></ol></li></ul>"#,
            "- a\n  how\n\n  3. do\n",
        ),
    ] {
        assert_eq!(convert_with(html, &options), expected, "{html}");
    }
    // ~keep Between inline markers the list is text, and a blank line would end the bold or
    // ~keep the summary's markers.
    assert_eq!(
        convert_with(r#"<ul><li><b>a<ol start="3"><li>c</li></ol></b></li></ul>"#, &options),
        "- **a\n  3. c**\n"
    );
    assert_eq!(
        convert_with(
            r#"<ul><li>a<details><summary>s<ol start="3"><li>x</li></ol></summary></details></li></ul>"#,
            &options
        ),
        "- a\n\n**s\n  3. x**\n"
    );
}

#[test]
fn should_start_each_of_several_nested_ordered_lists_after_the_text_as_a_list() {
    let options = tier2_options();
    let html =
        r#"<ul><li>a<ol start="3"><li>x</li></ol><ol start="5"><li>y</li></ol><ol start="7"><li>z</li></ol></li></ul>"#;
    let markdown = convert_with(html, &options);
    let rendered = render(&markdown);
    assert!(rendered.contains(r#"<ol start="3">"#), "{markdown:?} -> {rendered:?}");
    assert!(!rendered.contains("a\n3."), "{markdown:?} -> {rendered:?}");
}

#[cfg(feature = "testkit")]
#[test]
fn should_hand_an_ordered_task_item_and_a_nested_ordered_list_to_the_full_converter() {
    use html_to_markdown_rs::prescan::PrescanReport;
    use html_to_markdown_rs::tier1;
    use html_to_markdown_rs::tier1::BailReason;

    let options = tier2_options();
    let result = tier1::run(
        r#"<ol><li><input type="checkbox">p</li></ol>"#,
        &PrescanReport::default(),
        &options,
    );
    assert!(matches!(result, Err(BailReason::ListItemCheckbox)), "{result:?}");
    let result = tier1::run(
        r#"<ol start="10"><li>a<ol start="100"><li>q</li></ol></li></ol>"#,
        &PrescanReport::default(),
        &options,
    );
    assert!(matches!(result, Err(BailReason::ListNestedOrdered)), "{result:?}");
}
