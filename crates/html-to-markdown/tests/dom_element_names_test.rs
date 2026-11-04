// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

use html_to_markdown_rs::dom::{ParserOptions, parse};

fn names(html: &str) -> Vec<String> {
    let dom = parse(html, ParserOptions).expect("the page parses");
    dom.nodes()
        .iter()
        .filter_map(|node| node.as_tag())
        .map(|tag| tag.name().as_utf8_str().into_owned())
        .collect()
}

#[test]
fn should_read_tag_names_in_lower_case() {
    assert_eq!(names("<DIV><Span>x</SPAN></DIV>"), ["div", "span"]);
}

#[test]
fn should_spell_svg_element_names_in_mixed_case_inside_svg_only() {
    let html = "<svg><CLIPPATH></CLIPPATH><foreignObject><clippath></clippath></foreignObject></svg><clippath>";
    assert_eq!(
        names(html),
        ["svg", "clipPath", "foreignObject", "clippath", "clippath"]
    );
}

fn top_names(html: &str) -> Vec<String> {
    let dom = parse(html, ParserOptions).expect("the page parses");
    dom.children()
        .iter()
        .filter_map(|handle| handle.get(dom.parser()))
        .filter_map(|node| node.as_tag())
        .map(|tag| tag.name().as_utf8_str().into_owned())
        .collect()
}

#[test]
fn should_close_an_element_at_an_end_tag_in_the_spelling_of_its_start_tag() {
    assert_eq!(top_names("<DIV>a</DIV><p>b</p>"), ["div", "p"]);
    assert_eq!(top_names("<svg><clippath></clippath></svg><p>b</p>"), ["svg", "p"]);
}

#[test]
fn should_close_an_element_at_an_end_tag_in_any_letter_case() {
    assert_eq!(top_names("<div>a</DIV><p>b</p>"), ["div", "p"]);
    assert_eq!(top_names("<DIV>a</div><P>b</p>"), ["div", "p"]);
    assert_eq!(top_names("<svg><clipPath></CLIPPATH></svg><p>b</p>"), ["svg", "p"]);
}
