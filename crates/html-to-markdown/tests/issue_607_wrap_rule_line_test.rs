// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for issue #607: wrap mode folded a rule line, or a heading underline, into the
//! text around it, so the rule or the heading was lost. Wrap mode also joined the keys of the
//! frontmatter into one line, folded the list items, fences, headings and table rows of a quote
//! into text, and reflowed a tilde code block.

use html_to_markdown_rs::{CodeBlockStyle, ConversionOptions, HeadingStyle, TierStrategy, convert};

fn wrapped(html: &str, heading_style: HeadingStyle) -> String {
    let options = ConversionOptions {
        extract_metadata: false,
        tier_strategy: TierStrategy::Tier2,
        wrap: true,
        wrap_width: 20,
        heading_style,
        ..ConversionOptions::default()
    };
    convert(html, Some(options))
        .expect("conversion must succeed")
        .content
        .unwrap_or_default()
}

fn render(markdown: &str) -> String {
    comrak::markdown_to_html(markdown, &comrak::Options::default())
}

#[test]
fn should_keep_a_rule_followed_by_text_on_its_own_line() {
    let out = wrapped(
        "<dl><dt>t</dt><dd><hr> <span>B</span></dd></dl><p>after</p>",
        HeadingStyle::Atx,
    );
    assert_eq!(
        render(&out),
        "<p>t</p>\n<hr />\n<p>B</p>\n<p>after</p>\n",
        "{out:?} must keep the rule"
    );
}

#[test]
fn should_keep_an_underlined_heading_a_heading() {
    for (html, expected) in [
        ("<h2>Heading</h2><p>x</p>", "<h2>Heading</h2>\n<p>x</p>\n"),
        ("<h1>Heading</h1><p>x</p>", "<h1>Heading</h1>\n<p>x</p>\n"),
        ("<p>para</p><h2>Heading</h2>", "<p>para</p>\n<h2>Heading</h2>\n"),
    ] {
        let out = wrapped(html, HeadingStyle::Underlined);
        assert_eq!(render(&out), expected, "{html:?} gave {out:?}");
    }
}

#[test]
fn should_not_reflow_the_text_of_an_underlined_heading() {
    let out = wrapped(
        "<h2>one two three four five six seven</h2><p>x</p>",
        HeadingStyle::Underlined,
    );
    assert!(
        out.starts_with("one two three four five six seven\n---"),
        "the heading text must stay on one line, as an ATX heading does: {out:?}"
    );
    assert_eq!(render(&out), "<h2>one two three four five six seven</h2>\n<p>x</p>\n");
}

#[test]
fn should_keep_a_rule_and_an_underline_on_their_own_lines_in_a_quote() {
    let out = wrapped(
        "<blockquote><dl><dt>t</dt><dd><hr> <span>B</span></dd></dl></blockquote>",
        HeadingStyle::Atx,
    );
    assert_eq!(
        render(&out),
        "<blockquote>\n<p>t</p>\n<hr />\n<p>B</p>\n</blockquote>\n",
        "{out:?} must keep the rule"
    );
    let out = wrapped(
        "<blockquote><h2>Heading</h2><p>x</p></blockquote>",
        HeadingStyle::Underlined,
    );
    assert_eq!(
        render(&out),
        "<blockquote>\n<h2>Heading</h2>\n<p>x</p>\n</blockquote>\n",
        "{out:?} must keep the heading"
    );
}

fn convert_with(html: &str, options: ConversionOptions) -> String {
    convert(html, Some(options))
        .expect("conversion must succeed")
        .content
        .unwrap_or_default()
}

#[test]
fn should_keep_the_frontmatter_a_yaml_block_when_wrapping() {
    let options = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        wrap: true,
        wrap_width: 20,
        ..ConversionOptions::default()
    };
    let out = convert_with(
        "<html><head><title>My Page</title><meta name=\"description\" content=\"A page about things\"></head>\
         <body><p>one two three four five six seven</p></body></html>",
        options,
    );
    assert_eq!(
        out,
        "---\nmeta-description: A page about things\ntitle: My Page\n---\n\none two three four\nfive six seven\n\n",
        "the frontmatter must stay one key per line and only the text after it wraps"
    );
}

#[test]
fn should_keep_every_block_line_in_a_quote_when_wrapping() {
    let failures: Vec<String> = [
        "<blockquote><ul><li>alpha</li><li>beta</li></ul></blockquote>",
        "<blockquote><ol><li>alpha</li><li>beta</li><li>gamma</li></ol></blockquote>",
        "<blockquote><p>text</p><ul><li>alpha</li><li>beta</li></ul></blockquote>",
        "<blockquote><pre><code>x = 1\ny = 2</code></pre></blockquote>",
        "<blockquote><h2>Heading</h2><p>x</p></blockquote>",
        "<blockquote><table><tr><th>h</th><th>i</th></tr><tr><td>x</td><td>y</td></tr></table></blockquote>",
    ]
    .iter()
    .filter_map(|html| {
        let unwrapped = wrapped_at(html, 0, CodeBlockStyle::Backticks);
        let out = wrapped_at(html, 20, CodeBlockStyle::Backticks);
        (render(&out) != render(&unwrapped)).then(|| format!("{html:?}: {out:?}, unwrapped {unwrapped:?}"))
    })
    .collect();
    assert!(
        failures.is_empty(),
        "wrapping must not change the blocks:\n{}",
        failures.join("\n")
    );
}

#[test]
fn should_keep_a_tilde_code_block_as_it_is_when_wrapping() {
    for html in [
        "<pre><code>alpha beta gamma\ndelta epsilon zeta\neta</code></pre><p>after</p>",
        "<blockquote><pre><code>alpha beta gamma\ndelta epsilon zeta</code></pre></blockquote>",
    ] {
        let unwrapped = wrapped_at(html, 0, CodeBlockStyle::Tildes);
        let out = wrapped_at(html, 20, CodeBlockStyle::Tildes);
        assert_eq!(render(&out), render(&unwrapped), "{html:?}: {out:?}");
    }
}

/// Convert `html` wrapped at `width`, or unwrapped when `width` is 0.
fn wrapped_at(html: &str, width: usize, code_block_style: CodeBlockStyle) -> String {
    convert_with(
        html,
        ConversionOptions {
            extract_metadata: false,
            tier_strategy: TierStrategy::Tier2,
            wrap: width > 0,
            wrap_width: width.max(20),
            code_block_style,
            ..ConversionOptions::default()
        },
    )
}
