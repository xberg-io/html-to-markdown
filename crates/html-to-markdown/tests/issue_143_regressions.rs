// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

fn convert(
    html: &str,
    opts: Option<html_to_markdown_rs::ConversionOptions>,
) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

use std::fs;
use std::path::PathBuf;

use html_to_markdown_rs::ConversionOptions;

fn fixture_path(name: &str) -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "../../test_documents/html/issues", name]
        .iter()
        .collect()
}

fn options_with_wrap() -> ConversionOptions {
    ConversionOptions {
        wrap: true,
        wrap_width: 80,
        extract_metadata: false,
        autolinks: false,
        ..Default::default()
    }
}

fn normalize_newlines(input: &str) -> String {
    input.replace("\r\n", "\n").replace('\r', "\n")
}

#[test]
fn split_closing_tag_does_not_merge_nested_list() {
    // ~keep An end tag may hold white space and a line break before its `>` (JSX-style
    // ~keep formatting). The link must close there, or it absorbs the nested list.
    let html = r##"<ul>
  <li>
    <a href="#beyond"
      >Beyond triads</a
    >
    <ul>
      <li><a href="#c">Child</a></li>
    </ul>
  </li>
  <li>
    <a href="#sibling"
      >Sibling item</a
    >
    <ul>
      <li><a href="#d">Deep child</a></li>
    </ul>
  </li>
</ul>"##;
    let opts = ConversionOptions {
        wrap: false,
        extract_metadata: false,
        autolinks: false,
        ..Default::default()
    };
    let result = convert(html, Some(opts)).expect("conversion should succeed");
    assert!(
        result.contains("Child"),
        "nested item 'Child' should be present: got {result:?}"
    );
    assert!(
        result.contains("Sibling item"),
        "sibling list item should be present: got {result:?}"
    );
    assert!(
        result.contains("Deep child"),
        "deep nested item should be present: got {result:?}"
    );
    assert!(
        !result.contains("triads * [") && !result.contains("triads - [Child"),
        "nested items must not be merged into parent link text: got {result:?}"
    );
}

#[test]
fn wrap_preserves_link_only_list_items() {
    let html = fs::read_to_string(fixture_path("gh-143-links-wordwrap.html")).unwrap();
    let expected = fs::read_to_string(fixture_path("gh-143-links-wordwrap.md")).unwrap();

    let result = convert(&html, Some(options_with_wrap())).expect("conversion should succeed");

    assert_eq!(
        normalize_newlines(&result).trim(),
        normalize_newlines(&expected).trim(),
        "word wrapping should not merge nested link-only list items"
    );
}

#[test]
fn should_close_each_link_at_an_end_tag_split_over_lines() {
    let html = "<p><a href=\"#a\">A</a\n  >\n<a href=\"#b\">B</a\n> after</p>";
    let result = convert(html, None).expect("conversion should succeed");
    assert_eq!(result, "[A](#a) [B](#b) after\n");
}
