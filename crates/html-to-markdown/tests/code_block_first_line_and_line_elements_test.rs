#![allow(missing_docs)]

//! A code block starts where a browser starts it, and each plain block container in it is one line.
//!
//! Each expected block was compared with the lines that Chrome renders for the same `pre`. An HTML
//! parser drops one line feed right after the `pre` tag; a second one is a blank first line of code.

use html_to_markdown_rs::options::WhitespaceMode;
use html_to_markdown_rs::{CodeBlockStyle, ConversionOptions, TierStrategy, convert};

fn options(tier_strategy: TierStrategy) -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        tier_strategy,
        ..ConversionOptions::default()
    }
}

fn converted(html: &str, options: Option<ConversionOptions>) -> String {
    convert(html, options)
        .expect("conversion must succeed")
        .content
        .unwrap_or_default()
}

fn assert_none(failures: &[String], checked: usize) {
    assert!(
        failures.is_empty(),
        "{} of {checked} checks differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

/// The tiers a test can ask for by name: Tier-1 only where the test kit exposes it.
fn tiers() -> Vec<TierStrategy> {
    vec![
        TierStrategy::Tier2,
        #[cfg(feature = "testkit")]
        TierStrategy::Tier1,
    ]
}

/// Converts each input in `style` on every tier and reports the outputs that differ.
fn assert_style(style: CodeBlockStyle, cases: &[(&str, &str)]) {
    let mut failures = Vec::new();
    let mut checked = 0;
    for (html, expected) in cases {
        for tier_strategy in tiers() {
            checked += 1;
            let options = ConversionOptions {
                code_block_style: style,
                ..options(tier_strategy)
            };
            let actual = converted(html, Some(options));
            if actual != *expected {
                failures.push(format!(
                    "{html:?} ({style:?}, {tier_strategy:?}):\n  expected {expected:?}\n  actual   {actual:?}"
                ));
            }
        }
    }
    assert_eq!(checked, cases.len() * tiers().len());
    assert_none(&failures, checked);
}

/// Every name of the list of plain block containers that the two converters read.
const LINE_ELEMENTS: [&str; 13] = [
    "address", "article", "aside", "center", "dialog", "div", "footer", "header", "hgroup", "main", "nav", "search",
    "section",
];

#[test]
fn should_write_one_line_for_each_plain_block_container_in_a_code_block() {
    const EXPECTED: &str = "```\na\n  b\n```\n";
    let mut failures = Vec::new();
    let mut checked = 0;
    for name in LINE_ELEMENTS {
        // A browser shows only an open dialog.
        let open = if name == "dialog" { " open" } else { "" };
        let html = format!("<pre><{name}{open}>a</{name}><{name}{open}>  b</{name}></pre>");
        for tier_strategy in tiers() {
            checked += 1;
            // The default options remove a `nav` element with its content.
            let mut options = options(tier_strategy);
            options.preprocessing.remove_navigation = false;
            #[cfg(feature = "testkit")]
            if matches!(tier_strategy, TierStrategy::Tier1) {
                let direct = html_to_markdown_rs::tier1::run(
                    &html,
                    &html_to_markdown_rs::prescan::PrescanReport::default(),
                    &options,
                )
                .map_err(|reason| reason.to_string());
                if direct.as_deref() != Ok(EXPECTED) {
                    failures.push(format!("{name} (tier 1 with no fallback): {direct:?}"));
                }
            }
            let actual = converted(&html, Some(options));
            if actual != EXPECTED {
                failures.push(format!("{name} ({tier_strategy:?}): {actual:?}"));
            }
        }
    }
    assert_eq!(checked, LINE_ELEMENTS.len() * tiers().len());
    assert_none(&failures, checked);
}

/// Chrome shows two blank lines here: a parser drops a line feed only right after the `pre` tag.
/// Both tiers drop the first line feed of the content, as the fast one did before.
#[test]
fn should_drop_one_line_feed_at_the_start_of_the_code_element_of_a_block() {
    assert_style(
        CodeBlockStyle::Backticks,
        &[("<pre><code>\n\na\n</code></pre>", "```\n\na\n```\n")],
    );
}

#[test]
fn should_not_start_an_indented_code_block_with_a_blank_line() {
    assert_style(
        CodeBlockStyle::Indented,
        &[
            ("<p>x</p><pre>\n\na\nb\n</pre><p>y</p>", "x\n\n    a\n    b\n\ny\n"),
            ("<pre>\n\na\nb\n</pre>", "    a\n    b\n"),
            ("<ul><li>t<pre>\n\na\nb\n</pre></li></ul>", "- t\n\n      a\n      b\n"),
            ("<blockquote><pre>\n\na\nb\n</pre></blockquote>", ">     a\n>     b\n"),
        ],
    );
}

#[test]
fn should_drop_only_the_first_line_feed_of_a_block_in_strict_white_space_mode() {
    let cases = [
        ("<pre>\na\nb\n</pre>", "```\na\nb\n```\n"),
        ("<pre>\n\na\nb\n</pre>", "```\n\na\nb\n```\n"),
        ("<pre>\n\n\na\n</pre>", "```\n\n\na\n```\n"),
        (
            "<ul><li>t<pre>\n\na\nb\n</pre></li></ul>",
            "- t\n\n  ```\n\n  a\n  b\n  ```\n",
        ),
    ];
    for (html, expected) in cases {
        for tier_strategy in tiers() {
            let options = ConversionOptions {
                whitespace_mode: WhitespaceMode::Strict,
                ..options(tier_strategy)
            };
            assert_eq!(converted(html, Some(options)), expected, "{html:?} ({tier_strategy:?})");
        }
    }
}

#[test]
fn should_drop_one_line_feed_before_content_of_white_space() {
    assert_style(
        CodeBlockStyle::Backticks,
        &[("<p>x</p><pre>\n  \n</pre><p>y</p>", "x\n\n```\n  \n```\n\ny\n")],
    );
}
