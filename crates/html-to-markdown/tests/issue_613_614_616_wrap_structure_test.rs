// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for issues #613, #614 and #616: wrap mode changed the block structure of the
//! output. It dropped a hard line break, it wrote a line that opens a list where the unwrapped
//! output has text, and it cut a list item's continuation line off into a paragraph of its own.

use html_to_markdown_rs::{ConversionOptions, NewlineStyle, TierStrategy, convert};

fn converted(html: &str, wrap_width: Option<usize>, newline_style: NewlineStyle) -> String {
    let options = ConversionOptions {
        extract_metadata: false,
        tier_strategy: TierStrategy::Tier2,
        wrap: wrap_width.is_some(),
        wrap_width: wrap_width.unwrap_or(80),
        newline_style,
        ..ConversionOptions::default()
    };
    convert(html, Some(options))
        .expect("conversion must succeed")
        .content
        .unwrap_or_default()
}

fn render(markdown: &str) -> String {
    let html = comrak::markdown_to_html(markdown, &comrak::Options::default());
    html.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Asserts that wrapping `html` at each width renders the same HTML as the unwrapped output.
fn assert_wrap_keeps_structure(html: &str, widths: &[usize], newline_style: NewlineStyle) {
    let unwrapped = converted(html, None, newline_style);
    for &width in widths {
        let wrapped = converted(html, Some(width), newline_style);
        assert_eq!(
            render(&wrapped),
            render(&unwrapped),
            "{html:?} at width {width} ({newline_style:?}): wrapped {wrapped:?}, unwrapped {unwrapped:?}"
        );
    }
}

#[test]
fn should_keep_a_list_tight_when_an_item_continues_on_the_next_line() {
    for html in [
        "<ol><li>Refers to the mainland only. The source document states: the data\nfor the country do not include its regions.</li><li>Including the islands.</li></ol>",
        "<ul><li>x<ul><li>one two three\nfour five six seven eight</li><li>y</li></ul></li><li>z</li></ul>",
    ] {
        assert_wrap_keeps_structure(html, &[20, 80], NewlineStyle::Spaces);
    }
}

#[test]
fn should_keep_a_line_that_cannot_start_a_list_in_its_paragraph() {
    for html in [
        "<p>It was first described in\n1990. That year the first browser shipped to more people.</p>",
        "<p>It was part of Firefox\n57) as part of the Gecko engine and the Quantum project.</p>",
    ] {
        assert_wrap_keeps_structure(html, &[20, 80], NewlineStyle::Spaces);
    }
}
