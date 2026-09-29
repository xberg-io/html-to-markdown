// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for issue #615: text after a quote, in a list item inside `<b>` or `<q>`,
//! rendered inside the quote.

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
    // ~keep At 4 columns or a tab, and for a table, the block's lines are the paragraph's own
    // ~keep text, so no blank line splits the paragraph between the markers.
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
