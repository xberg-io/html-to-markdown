// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #679: the fast converter dropped a line break that no element
//! encloses (`a<br>b` with no `<p>`, `<div>` or `<body>` around it), so the two lines joined.

use html_to_markdown_rs::options::HighlightStyle;
use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert, tier1};

fn options(tier_strategy: TierStrategy) -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        tier_strategy,
        ..ConversionOptions::default()
    }
}

fn tier2(html: &str) -> String {
    convert(html, Some(options(TierStrategy::Tier2)))
        .expect("conversion must succeed")
        .content
        .unwrap_or_default()
}

/// The fast converter's own output: an `Err` is a bail, never a fallback to the full converter.
fn tier1(html: &str) -> String {
    tier1::run(html, &PrescanReport::default(), &options(TierStrategy::Tier1))
        .unwrap_or_else(|reason| panic!("the fast converter must not bail on {html:?}: {reason:?}"))
}

/// A `<br>` that no element encloses, with what comes before and after it varied.
const TOP_LEVEL_BREAKS: &[&str] = &[
    "a<br>b",
    "a<br>1) t",
    "a<br>- t",
    "a<br>2. t",
    "a<br/>b",
    "a<BR>b",
    "x<br>y<br>z",
    "a<br><br>b",
    "a<br><br><br>b",
    "a<br>",
    "a<br><br>",
    "<br>a",
    "<br><br>a",
    "<br>",
    "&amp;<br>b",
    "a&nbsp;<br>b",
    "a <br>b",
    "a<br> b",
    "a <br> b",
    "a<br>  1) t",
    "<b>a</b><br>b",
    "a<br><b>b</b>",
    "<a href=\"u\">a</a><br>b",
    "<code>a</code><br>b",
    "<img src=\"x.png\" alt=\"i\"><br>b",
    "<span>a</span><br>b",
    "a<!-- c --><br>b",
    "<!-- c --><br>a",
    "<p>a</p>b<br>c",
    "<p>a</p><br>b",
    "a<br><p>b</p>",
    "a<br><ul><li>x</li></ul>",
    "<ul><li>x</li></ul><br>a",
];

#[test]
fn should_keep_a_top_level_hard_break_in_the_fast_converter() {
    assert_eq!(tier1("a<br>1) t"), "a  \n1\\) t\n");
}

#[test]
fn should_write_every_top_level_break_the_same_in_both_converters() {
    for html in TOP_LEVEL_BREAKS {
        assert_eq!(tier1(html), tier2(html), "input: {html:?}");
    }
}

#[test]
fn should_write_a_top_level_break_as_it_is_written_inside_body() {
    for html in TOP_LEVEL_BREAKS {
        assert_eq!(tier1(html), tier1(&format!("<body>{html}</body>")), "input: {html:?}");
    }
}

#[test]
fn should_drop_the_leading_space_of_the_line_after_a_hard_break() {
    for html in [
        "<div>a<br> b</div>",
        "<p>a <br> b</p>",
        "<body>a<br>  1) t</body>",
        "<div>a<br>\t- t</div>",
        "<p><b>a</b><br> b</p>",
    ] {
        assert_eq!(tier1(html), tier2(html), "input: {html:?}");
    }
}

#[test]
fn should_keep_a_top_level_hard_break_when_auto_picks_the_fast_converter() {
    // ~keep With metadata off and no highlight marker the router picks the fast converter,
    // ~keep and the scanner does not bail on this input, so Auto's output is the scanner's.
    let auto = ConversionOptions {
        highlight_style: HighlightStyle::None,
        ..options(TierStrategy::Auto)
    };
    let html = "a<br>b";
    let scanned = tier1::run(html, &PrescanReport::default(), &auto).expect("the fast converter must not bail");
    let converted = convert(html, Some(auto)).expect("conversion must succeed").content;
    assert_eq!(scanned, "a  \nb\n");
    assert_eq!(converted.as_deref(), Some("a  \nb\n"));
}
