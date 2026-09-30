// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for heading text that changes its heading (issue #661): a `#` run at the end
//! of an ATX heading's text, and underlined heading text that reads as a link reference definition.

use html_to_markdown_rs::options::{HeadingStyle, OutputFormat};
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

const TIERS: [TierStrategy; 3] = [TierStrategy::Tier1, TierStrategy::Tier2, TierStrategy::Auto];

fn options(tier_strategy: TierStrategy, heading_style: HeadingStyle) -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        tier_strategy,
        heading_style,
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

#[test]
fn should_keep_heading_text_that_is_only_hashes_in_the_heading() {
    for tier in TIERS {
        let markdown = convert_with("<h1>#</h1>", &options(tier, HeadingStyle::Atx));
        assert_eq!(markdown, "# \\#\n", "{tier:?}");
        assert_eq!(render(&markdown), "<h1>#</h1>\n", "{tier:?}");
        let markdown = convert_with("<h3>##</h3>", &options(tier, HeadingStyle::Atx));
        assert_eq!(markdown, "### \\##\n", "{tier:?}");
        assert_eq!(render(&markdown), "<h3>##</h3>\n", "{tier:?}");
    }
}

#[test]
fn should_keep_a_hash_run_after_a_space_at_the_end_of_heading_text() {
    for (html, expected, heading) in [
        ("<h2>a #</h2>", "## a \\#\n", "<h2>a #</h2>\n"),
        ("<h2>a ##</h2>", "## a \\##\n", "<h2>a ##</h2>\n"),
        ("<h2>a<br>#</h2>", "## a \\#\n", "<h2>a #</h2>\n"),
        (
            "<h2><b>a</b> #</h2>",
            "## **a** \\#\n",
            "<h2><strong>a</strong> #</h2>\n",
        ),
        ("<h2>a <span>#</span></h2>", "## a \\#\n", "<h2>a #</h2>\n"),
    ] {
        for tier in TIERS {
            let markdown = convert_with(html, &options(tier, HeadingStyle::Atx));
            assert_eq!(markdown, expected, "{tier:?} {html}");
            assert_eq!(render(&markdown), heading, "{tier:?} {html}");
        }
    }
}

#[test]
fn should_keep_the_hash_of_a_heading_in_a_list_item_and_a_quote() {
    for tier in TIERS {
        let markdown = convert_with("<ul><li><h1>#</h1></li></ul>", &options(tier, HeadingStyle::Atx));
        assert_eq!(markdown, "- # \\#\n", "{tier:?}");
        assert!(render(&markdown).contains("<h1>#</h1>"), "{tier:?} {markdown:?}");
        let markdown = convert_with(
            "<blockquote><h1>a #</h1></blockquote>",
            &options(tier, HeadingStyle::Atx),
        );
        assert_eq!(markdown, "> # a \\#\n", "{tier:?}");
        assert!(render(&markdown).contains("<h1>a #</h1>"), "{tier:?} {markdown:?}");
    }
}

#[test]
fn should_escape_the_hash_run_of_an_underlined_style_heading_below_level_two() {
    let markdown = convert_with("<h3>#</h3>", &options(TierStrategy::Tier2, HeadingStyle::Underlined));
    assert_eq!(markdown, "### \\#\n");
    assert_eq!(render(&markdown), "<h3>#</h3>\n");
}

#[test]
fn should_leave_heading_text_the_line_does_not_close_on_as_it_is() {
    for (html, expected) in [
        ("<h1>a#</h1>", "# a#\n"),
        ("<h1>#a</h1>", "# #a\n"),
        ("<h1>a # b</h1>", "# a # b\n"),
        ("<h1>a \\#</h1>", "# a \\\\#\n"),
        ("<h1>a <b>#</b></h1>", "# a **#**\n"),
        ("<h1>plain text</h1>", "# plain text\n"),
    ] {
        for tier in TIERS {
            assert_eq!(
                convert_with(html, &options(tier, HeadingStyle::Atx)),
                expected,
                "{tier:?} {html}"
            );
        }
    }
}

#[test]
fn should_leave_the_text_of_a_closed_atx_heading_as_it_is() {
    let markdown = convert_with("<h1>a #</h1>", &options(TierStrategy::Tier2, HeadingStyle::AtxClosed));
    assert_eq!(markdown, "# a # #\n");
    assert_eq!(render(&markdown), "<h1>a #</h1>\n");
}

#[test]
fn should_leave_the_hash_of_a_djot_heading_as_it_is() {
    for tier in TIERS {
        let djot = ConversionOptions {
            output_format: OutputFormat::Djot,
            ..options(tier, HeadingStyle::Atx)
        };
        assert_eq!(convert_with("<h1>#</h1>", &djot), "# #\n", "{tier:?}");
        assert_eq!(convert_with("<h1>a #</h1>", &djot), "# a #\n", "{tier:?}");
    }
}

#[test]
fn should_keep_underlined_heading_text_that_reads_as_a_link_reference_definition() {
    for (html, expected, heading) in [
        ("<h1>[a]: b</h1>", "\\[a]: b\n======\n", "<h1>[a]: b</h1>\n"),
        ("<h2>[a]: b</h2>", "\\[a]: b\n------\n", "<h2>[a]: b</h2>\n"),
        (
            "<h1>[a]:b \"t\"</h1>",
            "\\[a]:b \"t\"\n=========\n",
            "<h1>[a]:b &quot;t&quot;</h1>\n",
        ),
        (
            "<h1>[a]: b (t)</h1>",
            "\\[a]: b (t)\n==========\n",
            "<h1>[a]: b (t)</h1>\n",
        ),
        (
            "<h1>[<b>a</b>]: b</h1>",
            "\\[**a**]: b\n==========\n",
            "<h1>[<strong>a</strong>]: b</h1>\n",
        ),
    ] {
        let markdown = convert_with(html, &options(TierStrategy::Tier2, HeadingStyle::Underlined));
        assert_eq!(markdown, expected, "{html}");
        assert_eq!(render(&markdown), heading, "{html}");
    }
}

#[test]
fn should_escape_a_definition_label_whose_destination_would_be_the_underline() {
    let underlined = options(TierStrategy::Tier2, HeadingStyle::Underlined);
    assert_eq!(convert_with("<h1>[a]:</h1>", &underlined), "\\[a]:\n====\n");
    // ~keep A `-` underline this long is a thematic break, which ends the definition before it.
    assert_eq!(convert_with("<h2>[a]:</h2>", &underlined), "[a]:\n----\n");
    assert_eq!(render("[a]:\n----\n"), "<h2>[a]:</h2>\n");
}

#[test]
fn should_keep_a_definition_shaped_heading_in_a_list_item_and_a_quote() {
    let underlined = options(TierStrategy::Tier2, HeadingStyle::Underlined);
    let markdown = convert_with("<ul><li><h1>[a]: b</h1></li></ul>", &underlined);
    assert_eq!(markdown, "- \\[a]: b\n  ======\n");
    assert!(render(&markdown).contains("<h1>[a]: b</h1>"), "{markdown:?}");
    let markdown = convert_with("<blockquote><h2>[a]: b</h2></blockquote>", &underlined);
    assert_eq!(markdown, "> \\[a]: b\n> ------\n");
    assert!(render(&markdown).contains("<h2>[a]: b</h2>"), "{markdown:?}");
}

#[test]
fn should_leave_underlined_text_that_starts_no_definition_as_it_is() {
    let underlined = options(TierStrategy::Tier2, HeadingStyle::Underlined);
    for (html, expected) in [
        ("<h1>[a]: b c</h1>", "[a]: b c\n========\n"),
        ("<h1>[a] b</h1>", "[a] b\n=====\n"),
        ("<h1>[a]: b \"t\" x</h1>", "[a]: b \"t\" x\n============\n"),
        ("<h1>plain text</h1>", "plain text\n==========\n"),
        ("<h1>a #</h1>", "a #\n===\n"),
    ] {
        let markdown = convert_with(html, &underlined);
        assert_eq!(markdown, expected, "{html}");
        assert!(render(&markdown).starts_with("<h1>"), "{html}: {markdown:?}");
    }
}
