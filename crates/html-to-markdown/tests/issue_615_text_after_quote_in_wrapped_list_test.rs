// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for issue #615: text after a quote, in a list item inside `<b>` or `<q>`,
//! rendered inside the quote; and for issue #617 and its siblings: a quote, a task item's quote
//! or a rule as the first content of a list item rendered outside the item.

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
    comrak::markdown_to_html(markdown, &comrak::Options::default())
}

/// Every inline wrapper a list can sit in: the emphasis wrappers and `<q>`, which make the list
/// text, and the wrappers that leave it a list or write marker-only spans.
const WRAPPERS: [&str; 20] = [
    "b", "strong", "em", "i", "q", "u", "span", "a", "small", "abbr", "cite", "code", "del", "s", "mark", "ins", "var",
    "dfn", "sub", "sup",
];

/// A block in a list item followed by the item's text `t`.
const BODIES: [&str; 6] = [
    "<ul><li>x<blockquote>q</blockquote>t</li></ul>",
    "<ul><li>x<blockquote><p>q</p></blockquote>t</li></ul>",
    "<ul><li>x<blockquote>q<blockquote>r</blockquote></blockquote>t</li></ul>",
    "<ul><li>x<blockquote>q</blockquote>t</li><li>y</li></ul>",
    "<ul><li>x<blockquote>q</blockquote><b>t</b></li></ul>",
    "<ul><li>x<ul><li>n</li></ul>t</li></ul>",
];

fn wrap(tag: &str, body: &str) -> String {
    let open = if tag == "a" { r#"a href="u""# } else { tag };
    format!("<div><{open}>{body}</{tag}></div>")
}

/// Whether the rendered `t` sits inside a quote or a nested list item instead of after it.
fn text_inside_block(rendered: &str) -> bool {
    ["<blockquote>", "<li>n"].iter().any(|opener| {
        rendered.split(opener).skip(1).any(|inside| {
            let end = inside
                .find(if *opener == "<li>n" { "</li>" } else { "</blockquote>" })
                .unwrap_or(inside.len());
            inside[..end]
                .split(|c: char| !c.is_alphanumeric())
                .any(|word| word == "t")
        })
    })
}

#[test]
fn should_write_text_after_a_quote_outside_the_quote_in_a_list_in_bold_or_a_quote() {
    for (html, expected) in [
        (
            "<div><b><ul><li>x<blockquote>q</blockquote>t</li></ul></b></div>",
            "**- x\n  > q\n\nt**\n",
        ),
        (
            "<div><q><ul><li>x<blockquote>q</blockquote>t</li></ul></q></div>",
            "\"- x\n  > q\n\nt\"\n",
        ),
        // ~keep Column 3, the widest column that still starts a block.
        (
            "<div><b><ol><li>x<blockquote>q</blockquote>t</li></ol></b></div>",
            "**1. x\n   > q\n\nt**\n",
        ),
    ] {
        let out = convert_with(html, &tier2_options());
        assert_eq!(out, expected, "{html:?}: the text after the quote joined the quote");
        assert!(
            !text_inside_block(&render(&out)),
            "{html:?}: {out:?} renders the text in the quote"
        );
    }
}

#[test]
fn should_write_text_after_a_block_outside_it_in_a_list_in_any_inline_wrapper() {
    let auto = ConversionOptions {
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    let mut failures = Vec::new();
    for tag in WRAPPERS {
        for body in BODIES {
            let html = wrap(tag, body);
            for (name, options) in [("tier 2", tier2_options()), ("auto", auto.clone())] {
                let out = convert_with(&html, &options);
                let rendered = render(&out);
                if text_inside_block(&rendered) {
                    failures.push(format!("{name} {html:?}: {out:?} renders {rendered:?}"));
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "the text after a block in the item joined the block:\n{}",
        failures.join("\n")
    );
}

#[test]
fn should_write_no_column_after_a_quote_in_a_list_that_is_text_with_tab_indentation() {
    let tabs = ConversionOptions {
        list_indent_type: ListIndentType::Tabs,
        ..tier2_options()
    };
    let mut failures = Vec::new();
    for tag in ["b", "strong", "em", "i", "q"] {
        for body in BODIES {
            let html = wrap(tag, body);
            let out = convert_with(&html, &tabs);
            let rendered = render(&out);
            if rendered.contains("<pre>") || text_inside_block(&rendered) {
                failures.push(format!("{html:?}: {out:?} renders {rendered:?}"));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "a list between markers is text, so the text after its quote gets no column:\n{}",
        failures.join("\n")
    );
}

#[test]
fn should_keep_a_list_between_markers_one_paragraph_when_its_blocks_are_text() {
    let tabs = ConversionOptions {
        list_indent_type: ListIndentType::Tabs,
        ..tier2_options()
    };
    // ~keep No item is real here (`10.` cannot interrupt the paragraph), so at 4 columns or a
    // ~keep tab, and for a table, the block's lines are the paragraph's own text, and no blank
    // ~keep line splits the paragraph between the markers.
    for (html, options, expected) in [
        (
            r#"<div><b><ol start="10"><li>x<blockquote>q</blockquote>t</li></ol></b></div>"#,
            tier2_options(),
            "**10. x\n    > q\n    t**\n",
        ),
        (
            "<div><b><ul><li>x<blockquote>q</blockquote>t</li></ul></b></div>",
            tabs,
            "**- x\n\t> q\n\tt**\n",
        ),
        (
            r#"<div><b><ul><li>a<ol start="10"><li>x<blockquote>q</blockquote>t</li></ol></li></ul></b></div>"#,
            tier2_options(),
            "**- a\n  10. x\n      > q\n      t**\n",
        ),
        (
            "<div><b><ul><li>x<table><tr><td>c</td></tr></table>t</li></ul></b></div>",
            tier2_options(),
            "**- x\n    | c |\n    | --- |\n  t**\n",
        ),
    ] {
        let out = convert_with(html, &options);
        assert_eq!(out, expected, "{html:?}: the paragraph between the markers was split");
        assert!(
            render(&out).contains("<strong>"),
            "{html:?}: {out:?} no longer renders the bold"
        );
    }
}

#[test]
fn should_not_take_a_marker_four_columns_in_after_a_blank_line_for_a_list_item() {
    // ~keep After a blank line a marker 4 or more columns past the real item's column opens a
    // ~keep code block, not an item, so the text after its quote gets no blank line.
    let html =
        r#"<div><b><ol start="10"><li>b<p>p</p><ul><li>x<blockquote>q</blockquote>t</li></ul></li></ol></b></div>"#;
    let out = convert_with(html, &tier2_options());
    assert!(out.contains("- x\n"), "{html:?}: the list was lost: {out:?}");
    assert!(
        !out.contains("> q\n\n"),
        "{html:?}: a blank line split the code block: {out:?}"
    );
}

#[test]
fn should_keep_a_quote_that_starts_a_list_item_in_the_item() {
    let tabs = ConversionOptions {
        list_indent_type: ListIndentType::Tabs,
        ..tier2_options()
    };
    for (html, options, expected) in [
        (
            "<ul><li><blockquote>q</blockquote></li></ul>",
            tier2_options(),
            "- > q\n",
        ),
        (
            "<ul><li><blockquote>q</blockquote>t</li></ul>",
            tier2_options(),
            "- > q\n\n  t\n",
        ),
        (
            "<ol><li><blockquote>q</blockquote></li></ol>",
            tier2_options(),
            "1. > q\n",
        ),
        ("<ul><li><blockquote>q</blockquote>t</li></ul>", tabs, "- > q\n\n\tt\n"),
        // ~keep In a quote that holds the list, the quote-in-quote branch ran first.
        (
            "<blockquote><ul><li><blockquote>q</blockquote></li></ul></blockquote>",
            tier2_options(),
            "> - > q\n",
        ),
        (
            "<blockquote><ul><li><blockquote><p>a</p><p>b</p></blockquote></li></ul></blockquote>",
            tier2_options(),
            "> - > a\n>   >\n>   > b\n",
        ),
    ] {
        let out = convert_with(html, &options);
        assert_eq!(out, expected, "{html:?}: the quote left the item");
        let rendered = render(&out);
        assert!(
            rendered.contains("<li>\n<blockquote>") && !text_inside_block(&rendered),
            "{html:?}: {out:?} renders {rendered:?}"
        );
    }
}

/// A nested list between markers, three deep, a nested list after the item's text, and a list
/// after text: each marker line after the first is a real list item.
const NESTED_BODIES: [&str; 7] = [
    "<ul><li>a<ul><li>x<blockquote>q</blockquote>t</li></ul></li></ul>",
    "<ol><li>a<ol><li>x<blockquote>q</blockquote>t</li></ol></li></ol>",
    "<ul><li>a<ul><li>b<ul><li>x<blockquote>q</blockquote>t</li></ul></li></ul></li></ul>",
    "<ul><li>a<ul><li>x<ul><li>n</li></ul>t</li></ul></li></ul>",
    "<ul><li>a<ul><li>x<blockquote>q</blockquote>t</li></ul>u</li></ul>",
    "a<ul><li>x<blockquote>q</blockquote>t</li></ul>",
    r#"<ul><li>a<ul><li><input type="checkbox">x<blockquote>q</blockquote>t</li></ul></li></ul>"#,
];

#[test]
fn should_write_text_after_a_block_outside_it_in_a_nested_list_between_markers() {
    let tabs = ConversionOptions {
        list_indent_type: ListIndentType::Tabs,
        ..tier2_options()
    };
    for (html, options, expected) in [
        (
            "<div><b><ul><li>a<ul><li>x<blockquote>q</blockquote>t</li></ul></li></ul></b></div>",
            tier2_options(),
            "**- a\n  * x\n    > q\n\n    t**\n",
        ),
        // ~keep Each nested marker is measured from the column of the item that holds it, so the
        // ~keep text goes to the innermost item.
        (
            "<div><b><ul><li>a<ul><li>b<ul><li>x<blockquote>q</blockquote>t</li></ul></li></ul></li></ul></b></div>",
            tier2_options(),
            "**- a\n  * b\n    + x\n      > q\n\n      t**\n",
        ),
        (
            "<div><b><ul><li>a<ul><li>x<ul><li>n</li></ul>t</li></ul></li></ul></b></div>",
            tier2_options(),
            "**- a\n  * x\n    + n\n\n    t**\n",
        ),
        // ~keep The list sits in a real item: the text stays in that item.
        (
            "<ul><li>o<b><ul><li>x<blockquote>q</blockquote>t</li></ul></b></li></ul>",
            tier2_options(),
            "- o *** x\n    > q\n\n  t**\n",
        ),
        (
            "<div><b>a<ul><li>x<blockquote>q</blockquote>t</li></ul></b></div>",
            tabs.clone(),
            "**a\n\n- x\n\t> q\n\n\tt**\n",
        ),
        // ~keep An ordered marker other than 1 starts an item after a blank line only.
        (
            r#"<div><b>a<ol start="3"><li>x<blockquote>q</blockquote>t</li></ol></b></div>"#,
            tier2_options(),
            "**a\n\n3. x\n   > q\n\n   t**\n",
        ),
        (
            "<div><b><ol><li>a</li><li>b<blockquote>q</blockquote>t</li></ol></b></div>",
            tier2_options(),
            "**1. a\n2. b\n   > q\n\nt**\n",
        ),
    ] {
        let out = convert_with(html, &options);
        assert_eq!(out, expected, "{html:?}: the text after the block joined the block");
    }
    let auto = ConversionOptions {
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    let mut failures = Vec::new();
    for body in NESTED_BODIES {
        let mut inputs: Vec<String> = ["b", "strong", "em", "i", "q"]
            .iter()
            .map(|tag| wrap(tag, body))
            .collect();
        inputs.push(format!("<details><summary>{body}</summary>d</details>"));
        inputs.push(format!("<figure><figcaption>{body}</figcaption></figure>"));
        inputs.push(format!("<table><caption>{body}</caption><tr><td>c</td></tr></table>"));
        for html in inputs {
            for (name, options) in [
                ("tier 2", tier2_options()),
                ("auto", auto.clone()),
                ("tabs", tabs.clone()),
            ] {
                let out = convert_with(&html, &options);
                let rendered = render(&out);
                if text_inside_block(&rendered) || rendered.contains("<pre>") {
                    failures.push(format!("{name} {html:?}: {out:?} renders {rendered:?}"));
                }
            }
        }
        // ~keep In a paragraph the first marker moves out of the bold after the list is written,
        // ~keep so with tabs a nested marker four columns in is taken for text: spaces only.
        let html = format!("<p>lead <b>{body}</b> tail</p>");
        for (name, options) in [("tier 2", tier2_options()), ("auto", auto.clone())] {
            let out = convert_with(&html, &options);
            let rendered = render(&out);
            if text_inside_block(&rendered) || rendered.contains("<pre>") {
                failures.push(format!("{name} {html:?}: {out:?} renders {rendered:?}"));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "the text after a block in a nested item joined the block:\n{}",
        failures.join("\n")
    );
}

#[test]
fn should_keep_the_second_paragraph_of_a_quote_at_a_nested_marker_in_a_quote_in_the_item() {
    // ~keep In a quote that holds the list, the quote's later lines take the marker line's width,
    // ~keep its indent included.
    let html =
        "<blockquote><ul><li>a<ul><li><blockquote><p>q</p><p>r</p></blockquote></li></ul></li></ul></blockquote>";
    let out = convert_with(html, &tier2_options());
    let rendered = render(&out);
    let quote = rendered
        .split("<ul>\n<li>\n<blockquote>\n<p>q</p>")
        .nth(1)
        .and_then(|inside| inside.split("</blockquote>").next());
    assert!(
        quote.is_some_and(|inside| inside.contains('r')),
        "{out:?}: the second paragraph left the nested item's quote: {rendered:?}"
    );
}

#[test]
fn should_start_the_first_block_of_a_task_item_on_the_next_line_inside_a_wrapper() {
    for (html, expected, rendered_part) in [
        (
            r#"<ul><li><input type="checkbox"><div><blockquote>q</blockquote></div></li></ul>"#,
            "- [ ]\n  > q\n",
            "]\n<blockquote>",
        ),
        (
            r#"<ul><li><p><input type="checkbox"></p><blockquote>q</blockquote></li></ul>"#,
            "- [ ]\n  > q\n",
            "]\n<blockquote>",
        ),
        (
            r#"<ul><li><label><input type="checkbox"></label><blockquote>q</blockquote></li></ul>"#,
            "- [ ]\n  > q\n",
            "]\n<blockquote>",
        ),
        (
            r#"<ul><li><input type="checkbox"><div><ul><li>n</li></ul></div></li></ul>"#,
            "- [ ]\n  * n\n",
            "]\n<ul>",
        ),
        (
            r#"<ul><li><input type="checkbox"><section><h2>h</h2></section></li></ul>"#,
            "- [ ]\n  ## h\n",
            "]\n<h2>",
        ),
        (
            r#"<ul><li><input type="checkbox"><pre>c</pre></li></ul>"#,
            "- [ ]\n  ```\n  c\n  ```\n",
            "]\n<pre>",
        ),
        // ~keep Under the checkbox line `---` underlines a heading, so a blank line comes first.
        (
            r#"<ul><li><input type="checkbox"><hr></li></ul>"#,
            "- [ ]\n\n  ---\n",
            "[ ]</p>\n<hr />",
        ),
        (
            r#"<ul><li><input type="checkbox"><div><hr></div></li></ul>"#,
            "- [ ]\n\n  ---\n",
            "[ ]</p>\n<hr />",
        ),
    ] {
        let out = convert_with(html, &tier2_options());
        assert_eq!(out, expected, "{html:?}: the block became the task's text");
        assert!(
            render(&out).contains(rendered_part),
            "{html:?}: {out:?} renders {:?}",
            render(&out)
        );
    }
    // ~keep Text first, also inside a wrapper, stays on the checkbox line.
    for (html, expected) in [
        (
            r#"<ul><li><input type="checkbox"><p>&gt; x</p></li></ul>"#,
            "- [ ] > x\n",
        ),
        (
            r#"<ul><li><p><input type="checkbox"> a</p><blockquote>q</blockquote></li></ul>"#,
            "- [ ] a\n  > q\n",
        ),
        (r#"<ul><li><input type="checkbox"><div>d</div></li></ul>"#, "- [ ] d\n"),
        // ~keep An empty container before the text renders nothing on its own walk, so the walk
        // ~keep must keep checking siblings instead of stopping at the empty container.
        (r#"<ul><li><input type="checkbox"><div></div>t</li></ul>"#, "- [ ] t\n"),
    ] {
        assert_eq!(
            convert_with(html, &tier2_options()),
            expected,
            "{html:?}: the text moved to a line of its own"
        );
    }
}

/// Checkbox content that nests past `max_depth`: the converter writes nothing past the limit, so
/// an empty container there does not hide a later quote, and a quote past the limit leaves the
/// text after it on the checkbox line. Neither drops the item or panics.
#[test]
fn should_keep_the_task_item_when_its_content_passes_the_depth_limit() {
    let options = ConversionOptions {
        max_depth: Some(6),
        list_indent_type: ListIndentType::Spaces,
        ..tier2_options()
    };
    let open = "<div>".repeat(10);
    let close = "</div>".repeat(10);
    for (html, expected) in [
        (
            format!(r#"<ul><li><input type="checkbox">{open}{close}<blockquote>q</blockquote></li></ul>"#),
            "- [ ]\n  > q\n",
        ),
        (
            format!(r#"<ul><li><input type="checkbox">{open}<blockquote>q</blockquote>{close}t</li></ul>"#),
            "- [ ] t\n",
        ),
    ] {
        assert_eq!(
            convert_with(&html, &options),
            expected,
            "{html:?}: the depth limit should not drop the item or panic"
        );
    }
}

#[test]
fn should_not_start_a_quote_on_a_line_that_holds_only_a_hyphen_outside_a_list() {
    let out = convert_with("<p>-</p><blockquote>q</blockquote>", &tier2_options());
    assert!(out.contains("> q"), "the quote was lost: {out:?}");
    assert!(!out.contains("- >"), "the quote joined the paragraph: {out:?}");
}

#[test]
fn should_keep_a_quote_that_starts_a_task_item_in_the_item() {
    for (html, expected) in [
        (
            r#"<ul><li><input type="checkbox" checked><blockquote>q</blockquote></li></ul>"#,
            "- [x]\n  > q\n",
        ),
        (
            r#"<ul><li>a<ul><li><input type="checkbox"><blockquote>q</blockquote></li></ul></li></ul>"#,
            "- a\n  - [ ]\n    > q\n",
        ),
        (
            r#"<ul><li><input type="checkbox" checked> <!-- c --><blockquote>q</blockquote></li></ul>"#,
            "- [x]\n  > q\n",
        ),
    ] {
        let out = convert_with(html, &tier2_options());
        assert_eq!(out, expected, "{html:?}: the quote became the task's text");
        assert!(
            render(&out).contains("]\n<blockquote>"),
            "{html:?}: {out:?} renders {:?}",
            render(&out)
        );
    }
    for (html, expected) in [
        (
            r#"<ul><li><input type="checkbox"><ul><li>n</li></ul></li></ul>"#,
            "- [ ]\n  * n\n",
        ),
        (
            r#"<ul><li><input type="checkbox"><h3>h</h3></li></ul>"#,
            "- [ ]\n  ### h\n",
        ),
    ] {
        assert_eq!(
            convert_with(html, &tier2_options()),
            expected,
            "{html:?}: the block became the task's text"
        );
    }
    // ~keep A table cell and inline mode hold no block, so the quote stays on the checkbox line.
    let inline = ConversionOptions {
        convert_as_inline: true,
        ..tier2_options()
    };
    for (html, options, expected) in [
        (
            r#"<table><tr><td><ul><li><input type="checkbox"><blockquote>q</blockquote></li></ul></td></tr></table>"#,
            tier2_options(),
            "| - [ ] q |\n| ------- |\n",
        ),
        (
            r#"<ul><li><input type="checkbox"><blockquote>q</blockquote></li></ul>"#,
            inline,
            "- [ ] q\n",
        ),
    ] {
        assert_eq!(
            convert_with(html, &options),
            expected,
            "{html:?}: the quote left the checkbox line"
        );
    }
    // ~keep Text that reads like a quote stays text, and in a wrapper the item is text, so the
    // ~keep markers keep one paragraph.
    for (html, expected, kept) in [
        (
            r#"<ul><li><input type="checkbox">&gt; x</li></ul>"#,
            "- [ ] > x\n",
            "<li>[ ] &gt; x</li>",
        ),
        (
            r#"<div><var><ul><li><input type="checkbox"><blockquote>q</blockquote></li></ul></var></div>"#,
            "*- [ ] > q*\n",
            "<em>",
        ),
        (
            r#"<div><b><ul><li><input type="checkbox"><blockquote>q</blockquote></li></ul></b></div>"#,
            "**- [ ] > q**\n",
            "<strong>",
        ),
    ] {
        let out = convert_with(html, &tier2_options());
        assert_eq!(out, expected, "{html:?}: the text moved to a line of its own");
        assert!(render(&out).contains(kept), "{html:?}: {out:?}");
    }
}

#[test]
fn should_keep_a_rule_that_starts_a_list_item_in_the_item() {
    let tier1 = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        ..tier2_options()
    };
    for (html, expected) in [
        ("<ul><li><hr></li></ul>", "- ___\n"),
        (r#"<ol start="10"><li><hr></li></ol>"#, "10. ___\n"),
        ("<ul><li>a<ul><li><hr></li></ul></li></ul>", "- a\n  * ___\n"),
        ("<blockquote><ul><li><hr></li></ul></blockquote>", "> - ___\n"),
        ("<ul><li><br><hr>t</li></ul>", "- ___\n\n  t\n"),
    ] {
        let out = convert_with(html, &tier2_options());
        assert_eq!(out, expected, "{html:?}: the rule left the item");
        assert!(
            render(&out).contains("<li>\n<hr />"),
            "{html:?}: {out:?} renders {:?}",
            render(&out)
        );
    }
    let inline = ConversionOptions {
        convert_as_inline: true,
        ..tier2_options()
    };
    assert_eq!(
        convert_with("<ul><li><hr></li></ul>", &inline),
        "- ___\n",
        "inline mode: the rule left the item"
    );
    for html in ["<ul><li><hr></li></ul>", r#"<ol start="10"><li><hr></li></ol>"#] {
        assert_eq!(
            convert_with(html, &tier1),
            convert_with(html, &tier2_options()),
            "{html:?}: Tier 1 differs from Tier 2"
        );
    }
}
