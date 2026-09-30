// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for block containers in a list item: details, figure, fieldset, menu and
//! hgroup (issue #657), and a custom element after a block of the item (issue #658).

use html_to_markdown_rs::options::{ListIndentType, NewlineStyle};
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn options_for(tier_strategy: TierStrategy) -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        tier_strategy,
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
    options.extension.table = true;
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

/// Assert the same result with the fast converter and the full converter.
fn assert_converts_in_both_tiers(html: &str, expected: &str, rendered: &[&str]) {
    for tier_strategy in [TierStrategy::Tier1, TierStrategy::Tier2] {
        assert_converts(html, &options_for(tier_strategy), expected, rendered);
    }
}

#[test]
fn should_keep_a_details_element_that_starts_a_list_item_in_the_item() {
    assert_converts_in_both_tiers(
        "<ul><li><details><summary>s</summary>d</details></li></ul>",
        "- **s**\n\n  d\n",
        &["<li>\n<p><strong>s</strong></p>\n<p>d</p>\n</li>"],
    );
    assert_converts_in_both_tiers(
        "<ul><li><details><summary>s</summary><ul><li>x</li></ul></details></li></ul>",
        "- **s**\n\n  * x\n",
        &["<li>\n<p><strong>s</strong></p>\n<ul>\n<li>x</li>\n</ul>\n</li>"],
    );
}

#[test]
fn should_keep_a_details_element_after_text_in_the_item_at_every_content_column() {
    assert_converts_in_both_tiers(
        "<ul><li>a<details><summary>s</summary>d</details>b</li><li>c</li></ul>",
        "- a\n\n  **s**\n\n  d\n\n  b\n- c\n",
        &["<p>a</p>\n<p><strong>s</strong></p>\n<p>d</p>\n<p>b</p>\n</li>"],
    );
    assert_converts_in_both_tiers(
        r#"<ol start="10"><li>a<details><summary>s</summary>d</details>b</li></ol>"#,
        "10. a\n\n    **s**\n\n    d\n\n    b\n",
        &["<p>a</p>\n<p><strong>s</strong></p>\n<p>d</p>\n<p>b</p>\n</li>"],
    );
    let tabs = ConversionOptions {
        list_indent_type: ListIndentType::Tabs,
        ..options_for(TierStrategy::Tier2)
    };
    assert_converts(
        "<ul><li>a<details><summary>s</summary>d</details>b</li></ul>",
        &tabs,
        "- a\n\n\t**s**\n\n\td\n\n\tb\n",
        &["<p>a</p>\n<p><strong>s</strong></p>\n<p>d</p>\n<p>b</p>\n</li>"],
    );
}

#[test]
fn should_keep_the_text_after_a_details_element_in_a_task_item() {
    assert_converts_in_both_tiers(
        r#"<ul><li><input type="checkbox"><details><summary>s</summary>d</details></li></ul>"#,
        "- [ ] **s**\n\n  d\n",
        &["<p>d</p>\n</li>"],
    );
}

#[test]
fn should_keep_a_figure_and_its_caption_in_the_list_item() {
    assert_converts_in_both_tiers("<ul><li><figure>f</figure></li></ul>", "- f\n", &["<li>f</li>"]);
    assert_converts_in_both_tiers(
        r#"<ul><li><figure><img src="a.png" alt="a"><figcaption>c</figcaption></figure></li></ul>"#,
        "- ![a](a.png)\n\n  *c*\n",
        &["<p><em>c</em></p>\n</li>"],
    );
    assert_converts_in_both_tiers(
        r#"<ul><li>a<figure><img src="a.png" alt="a"><figcaption>c</figcaption></figure>b</li></ul>"#,
        "- a\n\n  ![a](a.png)\n\n  *c*\n\n  b\n",
        &["<p><em>c</em></p>\n<p>b</p>\n</li>"],
    );
}

#[test]
fn should_keep_the_indent_of_an_image_after_a_caption_in_an_ordered_item() {
    assert_converts_in_both_tiers(
        r#"<ol><li><figure><figcaption>c</figcaption><img src="a.png" alt="a"></figure>t</li></ol>"#,
        "1. *c*\n\n   ![a](a.png)\n\n   t\n",
        &["<p><img src=\"a.png\" alt=\"a\" /></p>\n<p>t</p>\n</li>"],
    );
}

#[test]
fn should_keep_a_fieldset_in_the_list_item() {
    assert_converts_in_both_tiers(
        "<ul><li><fieldset><legend>l</legend>f</fieldset></li></ul>",
        "- **l**\n\n  f\n",
        &["<li>\n<p><strong>l</strong></p>\n<p>f</p>\n</li>"],
    );
    assert_converts_in_both_tiers(
        "<ul><li><fieldset><legend>l</legend><p>f</p></fieldset>t</li></ul>",
        "- **l**\n\n  f\n\n  t\n",
        &["<p>f</p>\n<p>t</p>\n</li>"],
    );
}

#[test]
fn should_nest_a_menu_in_the_list_item_like_a_list() {
    assert_converts_in_both_tiers(
        "<ul><li><menu><li>m</li></menu></li></ul>",
        "- - m\n",
        &["<li>\n<ul>\n<li>m</li>\n</ul>\n</li>"],
    );
    assert_converts_in_both_tiers(
        "<ul><li>a<menu><li>m</li><li>n</li></menu>b</li></ul>",
        "- a\n  - m\n  - n\n\n  b\n",
        &["<ul>\n<li>m</li>\n<li>n</li>\n</ul>\n<p>b</p>\n</li>"],
    );
    assert_converts_in_both_tiers(
        "<ol><li><menu><li>m</li><li>n</li></menu></li><li>c</li></ol>",
        "1. - m\n   - n\n2. c\n",
        &["<ol>\n<li>\n<ul>\n<li>m</li>\n<li>n</li>\n</ul>\n</li>\n<li>c</li>\n</ol>"],
    );
    assert_converts_in_both_tiers(
        r#"<ul><li><input type="checkbox"><menu><li>m</li></menu></li></ul>"#,
        "- [ ]\n  - m\n",
        &["<ul>\n<li>m</li>\n</ul>\n</li>"],
    );
}

#[test]
fn should_keep_the_text_after_an_hgroup_in_the_list_item() {
    assert_converts_in_both_tiers(
        "<ul><li><hgroup><h2>h</h2></hgroup>t</li></ul>",
        "- ## h\n\n  t\n",
        &["<h2>h</h2>\n<p>t</p>\n</li>"],
    );
    assert_converts_in_both_tiers(
        r#"<ul><li><input type="checkbox"><hgroup><h2>h</h2></hgroup>t</li></ul>"#,
        "- [ ]\n  ## h\n\n  t\n",
        &["<h2>h</h2>\n<p>t</p>\n</li>"],
    );
}

#[test]
fn should_keep_a_preserved_custom_element_after_a_paragraph_in_the_list_item() {
    for tier_strategy in [TierStrategy::Tier1, TierStrategy::Tier2] {
        let options = ConversionOptions {
            preserve_tags: vec!["my-el".to_string()],
            ..options_for(tier_strategy)
        };
        assert_converts(
            "<ol><li><p>a</p><my-el></my-el><h2>h</h2>t</li></ol>",
            &options,
            "1. a\n\n   <my-el></my-el>\n   ## h\n   t\n",
            &["<!-- raw HTML omitted --></p>\n<h2>h</h2>\n<p>t</p>\n</li>"],
        );
        assert_converts(
            "<ul><li><p>a</p><my-el>x</my-el>t</li></ul>",
            &options,
            "- a\n\n  <my-el>x</my-el>t\n",
            &["<p>a</p>\n<p><!-- raw HTML omitted -->x<!-- raw HTML omitted -->t</p>\n</li>"],
        );
    }
}

#[test]
fn should_keep_a_custom_element_after_a_paragraph_in_the_list_item() {
    assert_converts_in_both_tiers(
        "<ul><li><p>a</p><x-foo>y</x-foo>t</li></ul>",
        "- a\n\n  yt\n",
        &["<p>a</p>\n<p>yt</p>\n</li>"],
    );
    assert_converts_in_both_tiers(
        "<ul><li><p>a</p><x-foo><div>d</div></x-foo>t</li></ul>",
        "- a\n\n  d\n\n  t\n",
        &["<p>d</p>\n<p>t</p>\n</li>"],
    );
}

#[test]
fn should_keep_a_form_in_the_list_item_when_forms_are_kept() {
    let options = ConversionOptions {
        preprocessing: html_to_markdown_rs::options::PreprocessingOptions {
            remove_forms: false,
            ..Default::default()
        },
        ..options_for(TierStrategy::Tier2)
    };
    assert_converts(
        "<ul><li>a<form><p>f</p><p>g</p></form>b</li></ul>",
        &options,
        "- a\n\n  f\n\n  g\n\n  b\n",
        &["<p>a</p>\n<p>f</p>\n<p>g</p>\n<p>b</p>\n</li>"],
    );
}

#[test]
fn should_keep_the_items_of_a_menu_in_a_heading_on_the_heading_line() {
    assert_converts_in_both_tiers("<h2>t <menu><li>m</li></menu></h2>", "## t - m\n", &["<h2>t - m</h2>"]);
}

#[test]
fn should_keep_text_between_the_items_of_a_menu_in_a_list_item_in_the_item() {
    assert_converts_in_both_tiers(
        "<ul><li>a<menu>x<li>m</li>y<li>n</li></menu></li></ul>",
        "- a\n  x\n  - m\n  y\n  - n\n",
        &["<li>a\nx\n<ul>"],
    );
}

#[test]
fn should_make_the_list_loose_when_a_menu_in_an_item_is_loose() {
    assert_converts_in_both_tiers(
        "<ul><li>a<menu><li><p>m</p></li><li>n</li></menu></li><li>b</li></ul>",
        "- a\n\n  - m\n\n  - n\n\n- b\n",
        &["<li>\n<p>a</p>\n<ul>\n<li>\n<p>m</p>\n</li>\n<li>\n<p>n</p>\n</li>\n</ul>\n</li>\n<li>\n<p>b</p>\n</li>"],
    );
}

#[test]
fn should_give_the_checkbox_in_a_menu_item_to_that_item() {
    assert_converts_in_both_tiers(
        r#"<ul><li><menu><li><input type="checkbox">c</li></menu></li></ul>"#,
        "- - [ ] c\n",
        &["<li>\n<ul>\n<li>[ ] c</li>\n</ul>\n</li>"],
    );
}

#[test]
fn should_keep_the_text_after_a_center_search_or_dialog_element_in_the_list_item() {
    for tag in ["center", "search", "dialog"] {
        let html = format!("<ul><li><{tag} open>c</{tag}>t</li></ul>");
        assert_converts_in_both_tiers(&html, "- c\n\n  t\n", &["<p>c</p>\n<p>t</p>\n</li>"]);
    }
}

#[test]
fn should_keep_a_details_element_in_a_list_item_in_a_table_cell_on_the_cell_line() {
    assert_converts_in_both_tiers(
        "<table><tr><td><ul><li><details><summary>s</summary>d</details></li></ul></td></tr></table>",
        "| **s**  d |\n| -------- |\n",
        &["<th><strong>s</strong>  d</th>"],
    );
    assert_converts(
        "<table><tr><td><ul><li>a<details><summary>s</summary>d</details>b</li></ul></td></tr></table>",
        &options_for(TierStrategy::Tier2),
        "| a  **s**  d  b |\n| -------------- |\n",
        &["<th>a  <strong>s</strong>  d  b</th>"],
    );
}

#[test]
fn should_keep_a_details_element_in_a_list_item_in_a_quote_in_the_item() {
    assert_converts_in_both_tiers(
        "<blockquote><ul><li><details><summary>s</summary>d</details></li></ul></blockquote>",
        "> - **s**\n>\n>   d\n",
        &["<blockquote>\n<ul>\n<li>\n<p><strong>s</strong></p>\n<p>d</p>\n</li>"],
    );
}

#[test]
fn should_keep_a_hard_break_before_an_element_that_continues_the_paragraph() {
    let backslash = ConversionOptions {
        newline_style: NewlineStyle::Backslash,
        ..options_for(TierStrategy::Tier2)
    };
    let spaces = options_for(TierStrategy::Tier2);
    for (tag, text) in [("dialog", "x"), ("summary", "**x**"), ("legend", "**x**")] {
        let html = format!("<p>a<br><{tag}>x</{tag}>y</p>");
        assert_converts(&html, &backslash, &format!("a\\\n{text}\n\ny\n"), &["a<br />"]);
        assert_converts(&html, &spaces, &format!("a  \n{text}\n\ny\n"), &["a<br />"]);
        let html = format!("<ul><li>a<br><{tag}>x</{tag}>y</li></ul>");
        assert_converts(&html, &backslash, &format!("- a\\\n  {text}\n\n  y\n"), &["a<br />"]);
        assert_converts(&html, &spaces, &format!("- a  \n  {text}\n\n  y\n"), &["a<br />"]);
    }
}

#[test]
fn should_keep_a_hard_break_in_a_wrapper_or_before_an_empty_element_in_the_paragraph() {
    let backslash = ConversionOptions {
        newline_style: NewlineStyle::Backslash,
        ..options_for(TierStrategy::Tier2)
    };
    assert_converts(
        "<p><b>a<br></b><dialog>x</dialog>y</p>",
        &backslash,
        "**a**\\\nx\n\ny\n",
        &["a</strong><br />"],
    );
    assert_converts("<p>a<br><dialog></dialog>y</p>", &backslash, "a\\\ny\n", &["a<br />"]);
}

#[test]
fn should_put_back_only_a_single_hard_break_before_an_element_in_the_paragraph() {
    let backslash = ConversionOptions {
        newline_style: NewlineStyle::Backslash,
        ..options_for(TierStrategy::Tier2)
    };
    assert_converts(
        "<p>a<br><br><br><dialog><br>x</dialog>y</p>",
        &backslash,
        "a\n\n\\\nx\n\ny\n",
        &[],
    );
}

#[test]
fn should_drop_a_hard_break_before_an_element_that_ends_the_paragraph() {
    let backslash = ConversionOptions {
        newline_style: NewlineStyle::Backslash,
        ..options_for(TierStrategy::Tier2)
    };
    for (html, expected) in [
        ("<p>a<br><center>x</center>y</p>", "a\n\nx\n\ny\n"),
        (
            "<p>a<br><details><summary>s</summary>x</details>y</p>",
            "a\n\n**s**\n\nx\n\ny\n",
        ),
        ("<ul><li>a<br><h2>x</h2>y</li></ul>", "- a\n  ## x\n  y\n"),
        (
            "<blockquote><p>a<br><h2>x</h2>y</p></blockquote>",
            "> a\n> ## x\n>\n> y\n",
        ),
    ] {
        assert_converts(html, &backslash, expected, &[]);
        assert!(!convert_with(html, &backslash).contains('\\'), "{html}");
    }
}
