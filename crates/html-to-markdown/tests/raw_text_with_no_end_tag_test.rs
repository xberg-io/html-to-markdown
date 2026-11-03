#![allow(missing_docs)]

//! ~keep The byte passes before the parse escape the body of an `xmp` that closes. An `xmp` with no
//! ~keep end tag reaches the parser with its markup as written, and the parser reads that markup.

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn markdown(html: &str, tier_strategy: TierStrategy) -> String {
    let options = ConversionOptions {
        tier_strategy,
        ..ConversionOptions::default()
    };
    let content = convert(html, options).expect("conversion succeeds").content;
    content.unwrap_or_default().trim_end().to_owned()
}

fn assert_on_both_routes(cases: &[(&str, &str)]) {
    for (html, expected) in cases {
        assert_eq!(&markdown(html, TierStrategy::Auto), expected, "auto: {html}");
        assert_eq!(&markdown(html, TierStrategy::Tier2), expected, "tier2: {html}");
    }
}

#[test]
fn should_not_show_an_end_tag_that_the_list_pass_adds_to_an_xmp_with_no_end_tag() {
    assert_on_both_routes(&[("<li><xmp>", "-"), ("<li><xmp>x", "- x"), ("<LI><XMP>", "-")]);
}

#[test]
fn should_not_show_markup_in_an_xmp_with_no_end_tag_after_a_raw_text_tag_with_no_end_tag() {
    assert_on_both_routes(&[
        ("<script><xmp><b>", ""),
        ("<script><xmp></b>", ""),
        ("<style><xmp><b>", ""),
        ("<style><XMP><b>", ""),
    ]);
}

#[test]
fn should_read_a_raw_text_tag_in_a_body_that_is_read_again_as_markup() {
    assert_on_both_routes(&[
        ("<script><xmp><xmp><b>", ""),
        ("<script><xmp><noembed><b>", ""),
        ("<noframes><xmp><b>", ""),
        ("<li><noembed><xmp>", "-"),
    ]);
}

#[test]
fn should_keep_the_text_of_an_xmp_that_closes() {
    assert_on_both_routes(&[
        ("<xmp><b>c</b></xmp><p>x</p>", "<b>c</b>\n\nx"),
        ("<xmp>a &amp; b</xmp>", "a &amp; b"),
    ]);
}

#[test]
fn should_read_a_tag_name_that_runs_into_the_next_tag_as_one_name() {
    assert_on_both_routes(&[("<b<i>t</i>", "t")]);
}

#[test]
fn should_read_a_stray_quote_in_a_tag_as_part_of_the_tag_name() {
    assert_on_both_routes(&[("<b\">hello", "hello")]);
}

#[test]
fn should_not_nest_a_later_link_on_the_exact_page_of_issue_965() {
    let html = concat!(
        r#"<a href="/1"><div>One</div><b>-<x.y>%</x.y></b></a>"#,
        r#"<a href="/2">Two</a><a href="/3">Three</a>"#,
    );
    assert_on_both_routes(&[(html, "[One **-%**](/1)[Two](/2)[Three](/3)")]);
}
