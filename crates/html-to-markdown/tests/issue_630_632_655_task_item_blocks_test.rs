// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for task items: a table as the first content (issue #630), the checkbox
//! with the fast converter (issue #632), and content after an HTML block that `preserve_tags`
//! writes (issue #655).

use html_to_markdown_rs::options::{HeadingStyle, ListIndentType};
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

#[test]
fn should_start_a_table_that_starts_a_task_item_after_a_blank_line() {
    let options = tier2_options();
    for html in [
        r#"<ul><li><input type="checkbox"><table><tr><td>c</td></tr></table></li></ul>"#,
        r#"<ul><li><input type="checkbox"><div><table><tr><td>c</td></tr></table></div></li></ul>"#,
        r#"<ul><li><input type="checkbox"><section><table><tr><td>c</td></tr></table></section></li></ul>"#,
        r#"<ul><li><input type="checkbox"><span><table><tr><td>c</td></tr></table></span></li></ul>"#,
    ] {
        assert_converts(
            html,
            &options,
            "- [ ]\n\n  | c |\n    | --- |\n",
            &["<p>[ ]</p>\n<table>", "<th>c</th>"],
        );
    }
    assert_converts(
        r#"<ul><li><input type="checkbox" checked><table><tr><td><blockquote>q</blockquote></td></tr></table>t</li></ul>"#,
        &options,
        "- [x]\n\n  | > q |\n    | --- |\n\n  t\n",
        &["<th>&gt; q</th>", "<p>t</p>\n</li>"],
    );
    assert_converts(
        r#"<ol><li>a<ul><li><input type="checkbox"><table><tr><td>c</td></tr></table></li></ul></li></ol>"#,
        &options,
        "1. a\n   - [ ]\n\n     | c |\n     | --- |\n",
        &["<p>[ ]</p>\n<table>"],
    );
    let tabs = ConversionOptions {
        list_indent_type: ListIndentType::Tabs,
        ..tier2_options()
    };
    assert_converts(
        r#"<ul><li><input type="checkbox"><table><tr><td>c</td></tr></table></li></ul>"#,
        &tabs,
        "- [ ]\n\n\t| c |\n\t| --- |\n",
        &["<p>[ ]</p>\n<table>"],
    );
}

#[test]
fn should_start_a_block_in_an_element_that_writes_nothing_before_it_on_the_next_line() {
    let options = tier2_options();
    for html in [
        r#"<ul><li><input type="checkbox"><span><h2>h</h2></span>t</li></ul>"#,
        r#"<ul><li><input type="checkbox"><font><h2>h</h2></font>t</li></ul>"#,
    ] {
        assert_converts(html, &options, "- [ ]\n  ## h\n  t\n", &["<li>[ ]\n<h2>h</h2>\nt</li>"]);
    }
    assert_converts(
        r#"<ul><li><input type="checkbox"><hgroup><h2>h</h2></hgroup></li></ul>"#,
        &options,
        "- [ ]\n  ## h\n",
        &["<li>[ ]\n<h2>h</h2>\n</li>"],
    );
    let underlined = ConversionOptions {
        heading_style: HeadingStyle::Underlined,
        ..tier2_options()
    };
    assert_converts(
        r#"<ul><li><input type="checkbox"><hgroup><h2>h</h2></hgroup></li></ul>"#,
        &underlined,
        "- [ ]\n\n  h\n  --\n",
        &["<h2>h</h2>"],
    );
}

#[test]
fn should_keep_a_block_inside_an_inline_marker_on_the_checkbox_line() {
    let options = tier2_options();
    for (html, expected) in [
        (
            r#"<ul><li><input type="checkbox"><b><h2>h</h2></b></li></ul>"#,
            "- [ ] **## h**\n",
        ),
        (
            r#"<ul><li><input type="checkbox"><a href="u"><h2>h</h2></a></li></ul>"#,
            "- [ ] ## [h](u)\n",
        ),
    ] {
        assert_eq!(convert_with(html, &options), expected, "{html}");
    }
}

#[test]
fn should_write_the_checkbox_of_a_task_item_with_the_fast_converter() {
    let tier1 = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        ..tier2_options()
    };
    for (html, expected) in [
        (r#"<ul><li><input type="checkbox"><p>p</p></li></ul>"#, "- [ ] p\n"),
        (
            r#"<ul><li><input type="checkbox" checked>p</li><li>q</li></ul>"#,
            "- [x] p\n- q\n",
        ),
        (r#"<ul><li><p><input type="CHECKBOX">p</p></li></ul>"#, "- [ ] p\n"),
        (
            r#"<ul><li>a<ul><li><input type="checkbox">b</li></ul></li></ul>"#,
            "- a\n  - [ ] b\n",
        ),
    ] {
        assert_eq!(convert_with(html, &tier1), expected, "{html}");
        assert_eq!(convert_with(html, &tier2_options()), expected, "{html}");
    }
    let unchecked = r#"<p><input type="checkbox">p</p>"#;
    assert_eq!(
        convert_with(unchecked, &tier1),
        convert_with(unchecked, &tier2_options())
    );
}

#[cfg(feature = "testkit")]
#[test]
fn should_hand_a_list_item_with_a_checkbox_to_the_full_converter() {
    use html_to_markdown_rs::prescan::PrescanReport;
    use html_to_markdown_rs::tier1;
    use html_to_markdown_rs::tier1::BailReason;

    let options = tier2_options();
    for html in [
        r#"<ul><li><input type="checkbox">p</li></ul>"#,
        r#"<ol><li><span><input type="checkbox"></span>p</li></ol>"#,
    ] {
        let result = tier1::run(html, &PrescanReport::default(), &options);
        assert!(
            matches!(result, Err(BailReason::ListItemCheckbox)),
            "{html}: {result:?}"
        );
    }
    for html in [
        r#"<p><input type="checkbox">p</p>"#,
        r#"<ul><li><input type="text">p</li></ul>"#,
    ] {
        let result = tier1::run(html, &PrescanReport::default(), &options);
        assert!(result.is_ok(), "{html}: {result:?}");
    }
}

#[test]
fn should_leave_a_blank_line_after_a_preserved_html_block_in_a_task_item() {
    let options = ConversionOptions {
        preserve_tags: ["div", "section"].map(String::from).to_vec(),
        ..tier2_options()
    };
    for element in ["<div></div>", "<section></section>", "<div>x</div>"] {
        assert_converts(
            &format!(r#"<ul><li><input type="checkbox">{element}<h2>h</h2>t</li></ul>"#),
            &options,
            &format!("- [ ]\n  {element}\n\n  ## h\n  t\n"),
            &["<h2>h</h2>\n<p>t</p>\n</li>"],
        );
    }
    assert_converts(
        r#"<ul><li><input type="checkbox"><div></div><ul><li>x</li></ul>t</li></ul>"#,
        &options,
        "- [ ]\n  <div></div>\n\n  * x\n\n  t\n",
        &["<li>x</li>", "<p>t</p>"],
    );
    assert_converts(
        r#"<ul><li><input type="checkbox"><div></div>t <em>e</em></li></ul>"#,
        &options,
        "- [ ]\n  <div></div>\n\n  t *e*\n",
        &["<p>t <em>e</em></p>"],
    );
}

#[test]
fn should_leave_a_blank_line_after_a_preserved_html_block_in_a_list_item_or_the_document() {
    let options = ConversionOptions {
        preserve_tags: ["div", "h2"].map(String::from).to_vec(),
        ..tier2_options()
    };
    for (html, expected, rendered) in [
        (
            "<ul><li><div></div><h2>h</h2>t</li></ul>",
            "- <div></div>\n\n  <h2>h</h2>\n\n  t\n",
            "<p>t</p>",
        ),
        (
            "<ul><li><div></div><ol><li>x</li></ol></li></ul>",
            "- <div></div>\n\n  1. x\n",
            "<ol>\n<li>x</li>",
        ),
        (
            "<ol><li><p>a</p><div>x</div><em>e</em></li></ol>",
            "1. a\n\n   <div>x</div>\n\n   *e*\n",
            "<p><em>e</em></p>",
        ),
        ("<div></div><em>e</em>", "<div></div>\n\n*e*\n", "<p><em>e</em></p>"),
        ("<div>x</div>t", "<div>x</div>\n\nt\n", "<p>t</p>"),
    ] {
        assert_converts(html, &options, expected, &[rendered]);
    }
}

#[test]
fn should_keep_a_preserved_element_inside_text_in_the_text() {
    let options = ConversionOptions {
        preserve_tags: ["div", "my-el"].map(String::from).to_vec(),
        ..tier2_options()
    };
    for (html, expected) in [
        ("<p>a<div></div>b</p>", "a<div></div>b\n"),
        ("<b><div></div>c</b>", "**<div></div>c**\n"),
        (
            "<ul><li><my-el></my-el><h2>h</h2></li></ul>",
            "- <my-el></my-el>\n  ## h\n",
        ),
        (
            "<table><tr><td><div></div>c</td></tr></table>",
            "| <div></div>c |\n| ------------ |\n",
        ),
        ("<pre><div></div>c</pre>", "```\n<div></div>c\n```\n"),
        (
            "<ul><li><b><div></div></b><h2>h</h2></li></ul>",
            "- **<div></div>**\n  ## h\n",
        ),
        ("<ul><li><p>a</p><h2>h</h2></li></ul>", "- a\n\n  ## h\n"),
    ] {
        assert_eq!(convert_with(html, &options), expected, "{html}");
    }
    let inline = ConversionOptions {
        convert_as_inline: true,
        ..options
    };
    assert_eq!(convert_with("<div></div>t", &inline), "<div></div>t\n");
}
