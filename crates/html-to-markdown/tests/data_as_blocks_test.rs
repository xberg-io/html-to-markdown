#![allow(missing_docs)]

use html_to_markdown_rs::{ConversionOptions, convert};

fn markdown(html: &str) -> String {
    convert(html, None).unwrap().content.unwrap_or_default()
}

#[test]
fn should_convert_adjacent_data_as_p_spans_to_paragraphs() {
    let html = r#"<div><span data-as="p">First paragraph.</span><span data-as="p">Second paragraph.</span></div>"#;
    assert_eq!(markdown(html), "First paragraph.\n\nSecond paragraph.\n");
    assert_eq!(
        markdown(html),
        markdown("<div><p>First paragraph.</p><p>Second paragraph.</p></div>")
    );
}

#[test]
fn should_keep_inline_content_inside_a_data_as_paragraph() {
    let html = r#"<span data-as="p">A <strong>bold</strong> word</span><span data-as="p">B</span>"#;
    assert_eq!(markdown(html), "A **bold** word\n\nB\n");
}

#[test]
fn should_map_data_as_to_heading_list_and_quote_elements() {
    for (as_value, tag) in [("h2", "h2"), ("H3", "h3"), ("blockquote", "blockquote"), ("pre", "pre")] {
        let spanned = format!(r#"<div><span data-as="{as_value}">Text</span><span data-as="p">After</span></div>"#);
        let native = format!("<div><{tag}>Text</{tag}><p>After</p></div>");
        assert_eq!(markdown(&spanned), markdown(&native), "data-as={as_value}");
    }
    let spanned = r#"<span data-as="ul"><span data-as="li">one</span><span data-as="li">two</span></span>"#;
    assert_eq!(markdown(spanned), markdown("<ul><li>one</li><li>two</li></ul>"));
    assert_eq!(markdown(spanned), "- one\n- two\n");
}

#[test]
fn should_leave_spans_without_a_known_data_as_inline() {
    assert_eq!(markdown("<div><span>a</span><span>b</span></div>"), "ab\n");
    assert_eq!(
        markdown(r#"<div><span data-as="strong">a</span><span>b</span></div>"#),
        "ab\n"
    );
    assert_eq!(
        markdown(r#"<div><span data-as="">a</span><span>b</span></div>"#),
        "ab\n"
    );
}

#[test]
fn should_ignore_data_as_on_elements_other_than_span() {
    assert_eq!(
        markdown(r#"<div><em data-as="p">a</em><em>b</em></div>"#),
        markdown("<div><em>a</em><em>b</em></div>")
    );
}

fn markdown_with(html: &str, elements: &[&str]) -> String {
    let options = ConversionOptions::builder()
        .data_as_elements(elements.iter().map(|element| (*element).to_owned()).collect())
        .build();
    convert(html, Some(options)).unwrap().content.unwrap_or_default()
}

#[test]
fn should_treat_a_div_with_data_as_as_the_element_by_default() {
    let html = r#"<section><div data-as="p">A</div><div data-as="p">B</div></section>"#;
    assert_eq!(markdown(html), "A\n\nB\n");
}

#[test]
fn should_not_rename_a_link_with_data_as_by_default() {
    let html = r#"<p><a href="https://example.com" data-as="p">go</a></p>"#;
    assert_eq!(markdown(html), "[go](https://example.com)\n");
}

#[test]
fn should_rename_the_elements_of_a_user_list() {
    let html = r#"<div><a data-as="p">A</a><a data-as="p">B</a></div>"#;
    assert_eq!(markdown_with(html, &["a"]), "A\n\nB\n");
    let html = r#"<div><section data-as="p">A</section><section data-as="p">B</section></div>"#;
    assert_eq!(markdown_with(html, &["section"]), "A\n\nB\n");
}

#[test]
fn should_leave_the_default_elements_alone_when_the_user_list_omits_them() {
    let html = r#"<div><span data-as="p">A</span><span data-as="p">B</span></div>"#;
    assert_eq!(markdown_with(html, &["section"]), "AB\n");
}

#[test]
fn should_turn_data_as_off_with_an_empty_list() {
    let html = r#"<div><span data-as="p">A</span><span data-as="p">B</span></div>"#;
    assert_eq!(markdown_with(html, &[]), "AB\n");
}

#[test]
fn should_match_element_names_and_values_without_regard_to_case() {
    let html = r#"<div><SPAN data-as="P">A</SPAN><SPAN data-as="P">B</SPAN></div>"#;
    assert_eq!(markdown(html), "A\n\nB\n");
    let html = r#"<div><span data-as="p">A</span><span data-as="p">B</span></div>"#;
    assert_eq!(markdown_with(html, &["SPAN"]), "A\n\nB\n");
}

#[test]
fn should_use_the_default_elements_when_data_as_elements_is_unset() {
    let html = r#"<div><span data-as="p">A</span><span data-as="p">B</span></div>"#;
    let options = ConversionOptions::default();
    assert_eq!(options.data_as_elements, None);
    assert_eq!(
        convert(html, Some(options)).unwrap().content.unwrap_or_default(),
        "A\n\nB\n"
    );
}

#[test]
fn should_disable_data_as_only_for_some_empty_list_not_for_none() {
    let html = r#"<div><span data-as="p">A</span><span data-as="p">B</span></div>"#;
    let off = ConversionOptions::builder().data_as_elements(Vec::new()).build();
    assert_eq!(off.data_as_elements, Some(Vec::new()));
    assert_eq!(convert(html, Some(off)).unwrap().content.unwrap_or_default(), "AB\n");
}

#[test]
fn should_use_a_custom_list_as_given() {
    let html = r#"<div><b data-as="p">A</b><b data-as="p">B</b></div>"#;
    assert_eq!(markdown_with(html, &["b"]), "A\n\nB\n");
}

#[test]
fn should_decode_character_references_in_the_data_as_value() {
    let encoded = r#"<div><span data-as="&#112;">A</span><span data-as="&#x70;">B</span></div>"#;
    let plain = r#"<div><span data-as="p">A</span><span data-as="p">B</span></div>"#;
    assert_eq!(markdown(encoded), markdown(plain));
    assert_eq!(markdown(encoded), "A\n\nB\n");
    let unsupported = r#"<div><span data-as="&#115;trong">A</span><span>B</span></div>"#;
    assert_eq!(markdown(unsupported), "AB\n");
}

#[test]
fn should_not_repair_a_correctly_closed_data_as_span_on_the_final_open_chain() {
    let html = r#"<div><span data-as="p">A</span></div><span data-as="p">B <em>x</em></span>"#;
    assert_eq!(markdown(html), markdown("<div><p>A</p></div><p>B <em>x</em></p>"));
    let result = convert(html, None).unwrap();
    assert!(result.warnings.is_empty(), "{:?}", result.warnings);
    assert_eq!(result.content.unwrap_or_default(), "A\n\nB *x*\n");
}
