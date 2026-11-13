// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #692: in a table cell, the text after a `<center>`, `<search>`,
//! `<hgroup>` or `<dialog>` joined the container's last word, so a heading inside one ran into
//! the text after it. These convert like a `<div>`, so the text after them gets the cell break,
//! and both converters write the same cell.

use html_to_markdown_rs::options::{NewlineStyle, OutputFormat};
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

/// The one-row table with `row` and its separator row.
fn table(row: &str) -> String {
    let dashes: Vec<String> = row
        .trim_matches('|')
        .split('|')
        .map(|cell| "-".repeat(cell.trim().len().max(3)))
        .collect();
    format!("{row}\n| {} |\n", dashes.join(" | "))
}

/// The table each tier writes for `html`, as `(html, br_in_tables off row, br_in_tables on row)`.
fn check(cases: &[(&str, &str, &str)]) {
    let mut failures = Vec::new();
    for (html, off, on) in cases {
        for (br_in_tables, expected) in [(false, off), (true, on)] {
            let tier2_out = tier2(html, br_in_tables);
            let tier1_out = tier1_run(html, br_in_tables);
            let want = table(expected);
            if tier2_out != want || tier1_out.as_deref().ok() != Some(tier2_out.as_str()) {
                failures.push(format!(
                    "{html:?} br_in_tables={br_in_tables}: tier2 {tier2_out:?} tier1 {tier1_out:?}, want {want:?}"
                ));
            }
        }
    }
    assert!(failures.is_empty(), "row differs:\n{}", failures.join("\n"));
}

#[test]
fn should_keep_a_heading_in_a_div_apart_from_the_text_after_it_in_a_cell() {
    check(&[(
        "<table><tr><td>a<div><h2>h</h2>x</div>y</td><td>z</td></tr></table>",
        "| a h x y | z |",
        "| a<br>h<br>x<br>y | z |",
    )]);
}

#[test]
fn should_keep_a_heading_in_a_div_like_container_apart_from_the_text_after_it_in_a_cell() {
    check(&[
        (
            "<table><tr><td>a<address><h2>h</h2>x</address>y</td></tr></table>",
            "| a h x y |",
            "| a<br>h<br>x<br>y |",
        ),
        (
            "<table><tr><td>a<center><h2>h</h2>x</center>y</td></tr></table>",
            "| a h x y |",
            "| a<br>h<br>x<br>y |",
        ),
        (
            "<table><tr><td>a<search><h2>h</h2>x</search>y</td></tr></table>",
            "| a h x y |",
            "| a<br>h<br>x<br>y |",
        ),
        (
            "<table><tr><td>a<hgroup><h2>h</h2>x</hgroup>y</td></tr></table>",
            "| a h x y |",
            "| a<br>h<br>x<br>y |",
        ),
        (
            "<table><tr><td>a<dialog><h2>h</h2>x</dialog>y</td></tr></table>",
            "| a h x y |",
            "| a<br>h<br>x<br>y |",
        ),
    ]);
}

#[test]
fn should_keep_a_heading_in_a_dialog_after_a_line_break_apart_from_the_text_after_it_in_a_cell() {
    check(&[(
        "<table><tr><td>a<br><dialog><h2>h</h2>x</dialog>y</td></tr></table>",
        "| a h x y |",
        "| a<br>h<br>x<br>y |",
    )]);
}

#[test]
fn should_separate_text_after_a_div_like_container_in_a_cell() {
    check(&[
        (
            "<table><tr><td>a<center>x</center>y</td></tr></table>",
            "| a x y |",
            "| a<br>x<br>y |",
        ),
        (
            "<table><tr><td>a<search>x</search>y</td></tr></table>",
            "| a x y |",
            "| a<br>x<br>y |",
        ),
        (
            "<table><tr><td>a<hgroup>x</hgroup>y</td></tr></table>",
            "| a x y |",
            "| a<br>x<br>y |",
        ),
        (
            "<table><tr><td><dialog>x</dialog>y</td></tr></table>",
            "| x y |",
            "| x<br>y |",
        ),
    ]);
}

#[test]
fn should_separate_the_text_before_a_dialog_in_a_cell() {
    check(&[(
        "<table><tr><td>a<dialog>b</dialog>c</td></tr></table>",
        "| a b c |",
        "| a<br>b<br>c |",
    )]);
}

#[test]
fn should_start_a_dialog_on_a_new_paragraph_like_a_div() {
    for container in ["dialog", "div"] {
        let html = format!("<p>a<{container}>b</{container}>c</p>");
        assert_eq!(tier2(&html, false), "a\n\nb\n\nc\n", "{html}");
    }
}

#[test]
fn should_keep_a_div_like_container_on_its_own_line_in_plain_output() {
    for container in ["center", "dialog", "search", "hgroup", "div"] {
        let html = format!("<p><b>a<{container}>b</{container}>c</b></p>");
        let options = ConversionOptions {
            output_format: OutputFormat::Plain,
            ..tier2_options(false)
        };
        let plain = convert(&html, Some(options)).expect("conversion must succeed").content;
        assert_eq!(plain.as_deref(), Some("a\n\nb\n\nc\n"), "{html}");
    }
}

const DIV_LIKE: [&str; 5] = ["center", "dialog", "search", "hgroup", "div"];

#[test]
fn should_drop_the_backslash_break_before_a_div_like_container() {
    for container in DIV_LIKE {
        let html = format!("<p>a<br><{container}>b</{container}>c</p>");
        let options = ConversionOptions {
            newline_style: NewlineStyle::Backslash,
            ..tier2_options(false)
        };
        let out = convert(&html, Some(options)).expect("conversion must succeed").content;
        assert_eq!(out.as_deref(), Some("a\n\nb\n\nc\n"), "{html}");
    }
}

#[test]
fn should_separate_a_dialog_from_the_link_label_around_it_like_a_div() {
    for container in ["dialog", "div"] {
        let html = format!("<p><a href=\"u\">l<{container}>b</{container}>m</a></p>");
        assert_eq!(tier2(&html, false), "[l b m](u)\n", "{html}");
    }
}

#[test]
fn should_close_and_reopen_bold_around_a_div_like_container() {
    for container in DIV_LIKE {
        let html = format!("<p><b>a<{container}>b</{container}>c</b></p>");
        assert_eq!(tier2(&html, false), "**a**\n\n**b**\n\n**c**\n", "{html}");
    }
}

#[test]
fn should_keep_the_words_around_a_div_like_container_apart_in_a_heading_in_both_tiers() {
    let mut failures = Vec::new();
    for container in DIV_LIKE.into_iter().chain(["details", "menu"]) {
        for (shape, want) in [
            ("<h1>a<X>b</X>c</h1>", "# a b c\n"),
            ("<h1>a <X> b </X> c</h1>", "# a b c\n"),
            ("<h1>a<X></X>c</h1>", "# a c\n"),
            ("<h1><X>b</X>c</h1>", "# b c\n"),
        ] {
            let html = shape.replace('X', container);
            let tier2_out = tier2(&html, false);
            let tier1_out = tier1_run(&html, false);
            if tier2_out != want || tier1_out.as_deref().ok() != Some(tier2_out.as_str()) {
                failures.push(format!(
                    "{html:?}: tier2 {tier2_out:?} tier1 {tier1_out:?}, want {want:?}"
                ));
            }
        }
    }
    assert!(failures.is_empty(), "heading differs:\n{}", failures.join("\n"));
}

#[test]
fn should_start_a_list_item_with_a_div_like_container_on_the_marker_line_in_plain_output() {
    let mut failures = Vec::new();
    for container in DIV_LIKE {
        for (html, want) in [
            (
                format!("<ol><li><{container}>a</{container}>b</li></ol>"),
                "1. a\n\nb\n",
            ),
            (format!("<ul><li><{container}>a</{container}></li></ul>"), "- a\n"),
        ] {
            let options = ConversionOptions {
                output_format: OutputFormat::Plain,
                ..tier2_options(false)
            };
            let actual = convert(&html, Some(options)).expect("conversion must succeed").content;
            if actual.as_deref() != Some(want) {
                failures.push(format!("{html:?}: {actual:?}, want {want:?}"));
            }
        }
    }
    assert!(failures.is_empty(), "plain list item differs:\n{}", failures.join("\n"));
}

/// A block inside an inline wrapper or another block that opens the item also starts on the
/// marker line.
#[test]
fn should_start_a_list_item_with_a_nested_block_on_the_marker_line_in_plain_output() {
    let mut failures = Vec::new();
    for container in ["div", "center", "dialog", "details", "menu"] {
        for (shape, want) in [
            ("<ul><li><span><X>a</X></span></li></ul>", "- a\n"),
            ("<ul><li><div><X>a</X></div></li></ul>", "- a\n"),
            ("<ul><li><b><i><X>a</X></i></b></li></ul>", "- a\n"),
            ("<ul><li><span><X>a</X></span>b</li></ul>", "- a\n\nb\n"),
            ("<ol><li><div><X>a</X>b</div></li></ol>", "1. a\n\nb\n"),
            ("<ul><li>x<span><X>a</X></span></li></ul>", "- x\n\na\n"),
        ] {
            let html = shape.replace('X', container);
            let options = ConversionOptions {
                output_format: OutputFormat::Plain,
                ..tier2_options(false)
            };
            let actual = convert(&html, Some(options)).expect("conversion must succeed").content;
            if actual.as_deref() != Some(want) {
                failures.push(format!("{html:?}: {actual:?}, want {want:?}"));
            }
        }
    }
    assert!(failures.is_empty(), "plain list item differs:\n{}", failures.join("\n"));
}

#[test]
fn should_keep_text_that_looks_like_a_list_marker_apart_from_a_block_in_plain_output() {
    let mut failures = Vec::new();
    for (html, want) in [
        ("<span>- </span><div>b</div>", "-\n\nb\n"),
        ("<p>x</p>1. <div>b</div>", "x\n\n1.\n\nb\n"),
        ("<p>x</p>a. <div>b</div>", "x\n\na.\n\nb\n"),
        ("<ol><li>x</li><li><div>a</div>b</li></ol>", "1. x\n2. a\n\nb\n"),
        ("<p>x</p><ul><li><div>a</div></li></ul>", "x\n\n- a\n"),
        ("<ul><li>a</li></ul><div>b</div>", "- a\n\nb\n"),
        (
            "<table><tr><td><ul><li>a</li></ul></td><td>xy<div>b</div></td></tr></table>",
            "- a\txy\n\nb\n",
        ),
        (
            "<table><tr><td><ol><li>a</li></ol></td><td>xyz<div>b</div></td></tr></table>",
            "1. a\txyz\n\nb\n",
        ),
    ] {
        let options = ConversionOptions {
            output_format: OutputFormat::Plain,
            ..tier2_options(false)
        };
        let actual = convert(html, Some(options)).expect("conversion must succeed").content;
        if actual.as_deref() != Some(want) {
            failures.push(format!("{html:?}: {actual:?}, want {want:?}"));
        }
    }
    assert!(failures.is_empty(), "plain output differs:\n{}", failures.join("\n"));
}

/// The heading spacing stays out of a heading in a table cell, which already has its cell break,
/// and out of a layout cell, which converts inline without being a heading. Tier 1 leaves a
/// layout table to Tier 2.
#[test]
fn should_keep_the_heading_spacing_out_of_cells() {
    let mut failures = Vec::new();
    let layout = r#"<table border="0"><tr><td><div>A</div><div>B</div></td><td colspan="1">x</td></tr></table>"#;
    for (html, br_in_tables, want) in [
        (
            "<table><tr><td><h2>a<div>b</div>c</h2></td></tr></table>",
            false,
            Some("| a b c |\n| ----- |\n"),
        ),
        (
            "<table><tr><td><h2>a<div>b</div>c</h2></td></tr></table>",
            true,
            Some("| a<br>b<br>c |\n| ----------- |\n"),
        ),
        (
            "<table><tr><td><h2>a<menu>b</menu>c</h2></td></tr></table>",
            false,
            Some("| a b c |\n| ----- |\n"),
        ),
        (layout, false, Some("- A B x\n")),
        (layout, true, Some("- A<br>B x\n")),
    ] {
        let tier2_out = tier2(html, br_in_tables);
        let tier1_out = tier1_run(html, br_in_tables);
        let tier1_agrees = tier1_out
            .as_ref()
            .map_or_else(|_| want.is_some(), |out| *out == tier2_out);
        if want.is_some_and(|want| tier2_out != want) || !tier1_agrees {
            failures.push(format!(
                "{html:?} br_in_tables={br_in_tables}: tier2 {tier2_out:?} tier1 {tier1_out:?}, want {want:?}"
            ));
        }
    }
    assert!(failures.is_empty(), "cell differs:\n{}", failures.join("\n"));
}

/// A link label converts inline like a heading, so in a cell the text after a block in the label
/// also gets the cell break. Tier 1 leaves a link around a block to Tier 2.
#[test]
fn should_separate_the_text_after_a_block_in_a_link_label_in_a_cell() {
    let mut failures = Vec::new();
    for (inner, off, on) in [
        ("<span>a<div>b</div>c</span>", "| [a b c](u) |", "| [a<br>b<br>c](u) |"),
        (
            "<b>a<div>b</div>c</b>",
            "| [**a b c**](u) |",
            "| [**a<br>b<br>c**](u) |",
        ),
        (
            "<span>a<ul><li>x</li></ul>c</span>",
            "| [a x c](u) |",
            "| [a<br>x<br>c](u) |",
        ),
        ("<span>a<p>b</p>c</span>", "| [a b c](u) |", "| [a<br>b<br>c](u) |"),
    ] {
        let html = format!(r#"<table><tr><td><a href="u">{inner}</a></td></tr></table>"#);
        for (br_in_tables, expected) in [(false, off), (true, on)] {
            let tier2_out = tier2(&html, br_in_tables);
            let tier1_out = tier1_run(&html, br_in_tables);
            let want = table(expected);
            if tier2_out != want || tier1_out.as_deref().is_ok_and(|out| out != tier2_out) {
                failures.push(format!(
                    "{html:?} br_in_tables={br_in_tables}: tier2 {tier2_out:?} tier1 {tier1_out:?}, want {want:?}"
                ));
            }
        }
    }
    assert!(failures.is_empty(), "link label differs:\n{}", failures.join("\n"));
}

/// Inside a summary or a figcaption, Tier 1 writes the break before a block into that element's text.
#[test]
fn should_break_before_a_block_in_a_summary_or_figcaption_the_same_in_both_tiers() {
    let mut failures = Vec::new();
    for (html, want) in [
        (
            "<summary><h3>for T<div>where</div></h3></summary>",
            "### **for T where**\n",
        ),
        (
            "<figure><figcaption><h3>for T<div>where</div></h3></figcaption></figure>",
            "### *for T where*\n",
        ),
    ] {
        let tier2_out = tier2(html, false);
        let tier1_out = tier1_run(html, false);
        if tier2_out != want || tier1_out.as_deref().ok() != Some(tier2_out.as_str()) {
            failures.push(format!(
                "{html:?}: tier2 {tier2_out:?} tier1 {tier1_out:?}, want {want:?}"
            ));
        }
    }
    assert!(failures.is_empty(), "tiers differ:\n{}", failures.join("\n"));
}

/// A menu writes its items with a `-` bullet whatever bullets the options set.
#[test]
fn should_write_menu_items_with_a_dash_bullet() {
    let options = ConversionOptions {
        bullets: "*+".to_string(),
        ..tier2_options(false)
    };
    let out = convert("<menu><li>x</li><li>y</li></menu>", Some(options))
        .expect("conversion must succeed")
        .content;
    assert_eq!(out.as_deref(), Some("- x\n- y\n"));
}

/// Only a block that is the list item's own child starts on the marker line: a block in a table
/// cell inside the item keeps its blank line, whatever the cell holds before it.
#[test]
fn should_keep_a_block_in_a_cell_inside_a_list_item_apart_in_plain_output() {
    let options = ConversionOptions {
        output_format: OutputFormat::Plain,
        ..tier2_options(false)
    };
    let html = "<ul><li><table><tr><td>xy<div>b</div></td></tr></table></li></ul>";
    let out = convert(html, Some(options))
        .expect("conversion must succeed")
        .content
        .unwrap_or_default();
    assert!(out.contains("xy\n\nb"), "{out:?}");
}

/// The same block test decides where text after these containers goes in a list item, so it
/// starts a paragraph in the item there, as it does after a `<div>`.
#[test]
fn should_keep_text_after_a_div_like_container_in_its_list_item_like_after_a_div() {
    let mut failures = Vec::new();
    for container in ["center", "search", "hgroup", "dialog"] {
        for shape in [
            "<ul><li>a<X>b</X>c</li></ul>",
            "<ol><li><X>a</X>b</li></ol>",
            "<ul><li>a<X><h2>b</h2></X>c</li></ul>",
        ] {
            let html = shape.replace('X', container);
            let expected = tier2(&shape.replace('X', "div"), false);
            let actual = tier2(&html, false);
            if actual != expected {
                failures.push(format!("{html:?}: {actual:?}, want {expected:?}"));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "list item differs from a div:\n{}",
        failures.join("\n")
    );
}
