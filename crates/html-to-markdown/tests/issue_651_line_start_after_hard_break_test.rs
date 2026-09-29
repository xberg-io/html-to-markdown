// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for the text on the line after a hard break (issue #651): text that would
//! interrupt the paragraph there is written as text.

use html_to_markdown_rs::options::OutputFormat;
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn options(tier_strategy: TierStrategy, wrap: bool) -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        tier_strategy,
        wrap,
        wrap_width: 20,
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

fn count(html: &str, tag: &str) -> usize {
    html.matches(&format!("<{tag}>")).count() + html.matches(&format!("<{tag} ")).count()
}

/// Both tiers, with wrap off and on.
fn every_options() -> Vec<ConversionOptions> {
    let mut all = Vec::new();
    for tier in [TierStrategy::Tier1, TierStrategy::Tier2, TierStrategy::Auto] {
        for wrap in [false, true] {
            all.push(options(tier, wrap));
        }
    }
    all
}

#[test]
fn should_write_a_list_marker_after_a_hard_break_inside_bold_as_text() {
    let html = r#"<b><ul><li>a<br>1) t<ol start="2"><li>y<blockquote>q</blockquote>t</li></ol></li></ul></b>"#;
    for tier in [TierStrategy::Tier1, TierStrategy::Tier2, TierStrategy::Auto] {
        let markdown = convert_with(html, &options(tier, false));
        assert!(markdown.starts_with("**- a  \n  1\\) t\n"), "{tier:?}: {markdown:?}");
        let rendered = render(&markdown);
        assert!(!rendered.contains("<ol"), "{tier:?}: {markdown:?} renders {rendered:?}");
        assert!(
            rendered.contains("<br />\n1) t"),
            "{tier:?}: {markdown:?} renders {rendered:?}"
        );
    }
    for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
        let markdown = convert_with("<b><ul><li>a<br>01. t</li></ul></b>", &options(tier, false));
        assert!(markdown.ends_with("**- a  \n  01\\. t**\n"), "{tier:?}: {markdown:?}");
        let markdown = convert_with("<ul><li><b>a<br>1) t</b></li></ul>", &options(tier, false));
        assert_eq!(markdown, "- **a  \n  1\\) t**\n", "{tier:?}");
    }
}

#[test]
fn should_keep_every_line_that_would_interrupt_a_paragraph_after_a_hard_break_as_text() {
    const TEXTS: [&str; 13] = [
        "1) t",
        "01. t",
        "1. t",
        "000000001. t",
        "- t",
        "+ t",
        "* t",
        "> t",
        "# t",
        "-",
        "---",
        "=",
        "``` t",
    ];
    const WRAPPERS: [(&str, &str); 6] = [
        ("", ""),
        ("<b>", "</b>"),
        ("<em>", "</em>"),
        ("<mark>", "</mark>"),
        (r#"<a href="u">"#, "</a>"),
        ("<b><em>", "</em></b>"),
    ];
    const PLACES: [(&str, &str); 8] = [
        ("", ""),
        ("<p>", "</p>"),
        ("<ul><li>", "</li></ul>"),
        ("<ol><li>", "</li></ol>"),
        ("<blockquote>", "</blockquote>"),
        ("<blockquote><ul><li>", "</li></ul></blockquote>"),
        ("<ul><li><blockquote>", "</blockquote></li></ul>"),
        ("<ul><li>x<ul><li>", "</li></ul></li></ul>"),
    ];
    let mut checked = 0;
    for options in every_options() {
        for (place_open, place_close) in PLACES {
            for (wrapper_open, wrapper_close) in WRAPPERS {
                for text in TEXTS {
                    let html = format!("{place_open}{wrapper_open}a<br>{text}{wrapper_close}{place_close}");
                    let markdown = convert_with(&html, &options);
                    let rendered = render(&markdown);
                    for tag in ["ol", "ul", "li", "blockquote", "h1", "h2", "hr", "pre"] {
                        assert_eq!(
                            count(&rendered, tag),
                            count(&html, tag),
                            "{:?} wrap={}: {html} gives {markdown:?}, which renders {rendered:?}",
                            options.tier_strategy,
                            options.wrap,
                        );
                    }
                    checked += 1;
                }
            }
        }
    }
    assert_eq!(checked, 6 * 8 * 6 * 13);
}

#[test]
fn should_escape_only_the_character_that_starts_the_block() {
    for (text, expected) in [
        ("1) t", "a  \n1\\) t\n"),
        ("01. t", "a  \n01\\. t\n"),
        ("- t", "a  \n\\- t\n"),
        ("> t", "a  \n\\> t\n"),
        ("# t", "a  \n\\# t\n"),
        ("-", "a  \n\\-\n"),
        ("=", "a  \n\\=\n"),
    ] {
        for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
            let html = format!("<p>a<br>{text}</p>");
            assert_eq!(convert_with(&html, &options(tier, false)), expected, "{tier:?}: {html}");
        }
    }
}

#[test]
fn should_leave_a_line_that_cannot_interrupt_a_paragraph_unescaped() {
    for text in ["2. t", "9999999999. t", "0000000001. t", "1.5 t", "-t", "#t", "1)"] {
        for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
            let html = format!("<p>a<br>{text}</p>");
            let markdown = convert_with(&html, &options(tier, false));
            assert_eq!(markdown, format!("a  \n{text}\n"), "{tier:?}: {html}");
            assert!(render(&markdown).contains("<br />"), "{tier:?}: {markdown:?}");
        }
    }
}

#[test]
fn should_leave_code_after_a_hard_break_unchanged() {
    for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
        let markdown = convert_with("<p>a<br><code>1) t</code></p>", &options(tier, false));
        assert_eq!(markdown, "a  \n`1) t`\n", "{tier:?}");
        let markdown = convert_with("<pre>a<br>1) t</pre>", &options(tier, false));
        assert!(markdown.contains("a\n1) t\n"), "{tier:?}: {markdown:?}");
    }
}

#[test]
fn should_leave_a_djot_line_after_a_hard_break_unescaped() {
    let djot = ConversionOptions {
        output_format: OutputFormat::Djot,
        ..options(TierStrategy::Tier2, false)
    };
    let markdown = convert_with("<p>a<br>1) t</p>", &djot);
    assert!(markdown.ends_with("\n1) t\n"), "{markdown:?}");
    let markdown = convert_with("<p>a<br>1) t</p>", &options(TierStrategy::Tier2, false));
    assert!(markdown.ends_with("\n1\\) t\n"), "{markdown:?}");
}

#[test]
fn should_keep_a_line_of_a_link_in_a_nested_list_item_as_text() {
    for text in ["1) t", "1) t\\"] {
        let html = format!(r#"<ul><li>x<ul><li><a href="u">a<br>{text}</a></li></ul></li></ul>"#);
        for tier in [TierStrategy::Tier1, TierStrategy::Tier2] {
            let markdown = convert_with(&html, &options(tier, false));
            let rendered = render(&markdown);
            assert!(
                !rendered.contains("<ol"),
                "{tier:?}: {html} gives {markdown:?}, which renders {rendered:?}"
            );
            assert!(
                rendered.contains("<a href=\"u\">a<br />"),
                "{tier:?}: {markdown:?} renders {rendered:?}"
            );
        }
    }
}

#[test]
fn should_write_a_line_break_in_a_heading_the_same_in_both_tiers() {
    for html in [
        r#"<h1><a href="H">A<br>-<br>B</a></h1>"#,
        r#"<h1><a href="H">A<br>- \<br>B</a></h1>"#,
    ] {
        let tier1 = convert_with(html, &options(TierStrategy::Tier1, false));
        let tier2 = convert_with(html, &options(TierStrategy::Tier2, false));
        assert_eq!(tier1, tier2, "{html}");
        assert!(!tier1.contains("\\-"), "{html}: {tier1:?}");
    }
}
