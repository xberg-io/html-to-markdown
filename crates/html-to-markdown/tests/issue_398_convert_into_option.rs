//! Regression coverage for #398: `convert` accepts `impl Into<Option<ConversionOptions>>`.
//!
//! All four call shapes must produce identical output:
//!   - `convert(html, None)`
//!   - `convert(html, Some(opts))`
//!   - `convert(html, opts)`
//!   - `convert(html, ConversionOptions::default())`

use html_to_markdown_rs::{ConversionOptions, HeadingStyle, convert};

const HTML: &str = "<h1>Hello</h1><p>World</p>";

#[test]
fn convert_accepts_none() {
    let result = convert(HTML, None).expect("convert(html, None) must succeed");
    assert!(result.content.as_deref().unwrap_or("").contains("Hello"));
}

#[test]
fn convert_accepts_bare_default_options() {
    let result =
        convert(HTML, ConversionOptions::default()).expect("convert(html, ConversionOptions::default()) must succeed");
    assert!(result.content.as_deref().unwrap_or("").contains("Hello"));
}

#[test]
fn convert_accepts_some_default_options() {
    let result = convert(HTML, Some(ConversionOptions::default())).expect("convert(html, Some(default)) must succeed");
    assert!(result.content.as_deref().unwrap_or("").contains("Hello"));
}

#[test]
fn none_and_default_produce_identical_output() {
    let none_result = convert(HTML, None).unwrap();
    let default_result = convert(HTML, ConversionOptions::default()).unwrap();
    let some_default_result = convert(HTML, Some(ConversionOptions::default())).unwrap();

    assert_eq!(none_result.content, default_result.content);
    assert_eq!(none_result.content, some_default_result.content);
}

#[test]
fn bare_and_some_wrapped_options_produce_identical_output() {
    let opts = ConversionOptions {
        heading_style: HeadingStyle::Underlined,
        ..ConversionOptions::default()
    };

    let bare = convert(HTML, opts.clone()).unwrap();
    let wrapped = convert(HTML, Some(opts)).unwrap();

    assert_eq!(bare.content, wrapped.content);
}

#[test]
fn non_default_options_take_effect_via_bare_form() {
    // Underlined headings render `# H1` as `H1\n===`. Confirm the bare-options form
    // actually threads through to the converter and is not silently dropped.
    let opts = ConversionOptions {
        heading_style: HeadingStyle::Underlined,
        ..ConversionOptions::default()
    };

    let result = convert("<h1>Title</h1>", opts).unwrap();
    let content = result.content.unwrap_or_default();
    assert!(
        content.contains("==="),
        "expected underlined-style heading with `===` rule; got: {content:?}"
    );
}
