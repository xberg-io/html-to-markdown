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
    assert_eq!(
        top_names("<svg><foreignObject>x</foreignobject></svg><p>b</p>"),
        ["svg", "p"]
    );
}

#[test]
fn should_close_an_element_at_an_end_tag_with_white_space_before_its_end() {
    assert_eq!(top_names("<div>a</div ><p>b</p>"), ["div", "p"]);
    assert_eq!(top_names("<div>a</DIV\n\t><p>b</p>"), ["div", "p"]);
}

#[test]
fn should_keep_start_tag_source_after_an_empty_end_tag() {
    let dom = parse("</></><x.y>one</x.y><a href='x'>two</a>", ParserOptions).expect("the page parses");
    let tag = dom.nodes()[0].as_tag().expect("the first node is an element");
    assert_eq!(tag.raw().as_bytes(), b"<x.y>one</x.y>");
}

#[test]
fn should_skip_an_empty_end_tag_between_data_text_tokens() {
    let dom = parse("<p>before</>after</p>", ParserOptions).expect("the page parses");
    assert_eq!(dom.nodes()[0].inner_text(dom.parser()), "beforeafter");
}

#[test]
fn should_skip_an_empty_end_tag_before_encoded_data_text() {
    let dom = parse("<p>before</>&lt;/&gt;after</p>", ParserOptions).expect("the page parses");
    assert_eq!(dom.nodes()[0].inner_text(dom.parser()), "before&lt;/&gt;after");
}

#[test]
fn should_keep_an_empty_end_tag_in_raw_text() {
    let dom = parse("<script></>&lt;/&gt;after</script>", ParserOptions).expect("the page parses");
    assert_eq!(dom.nodes()[0].inner_text(dom.parser()), "</>&lt;/&gt;after");
}
