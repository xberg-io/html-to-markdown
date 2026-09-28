// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Pins the exact byte output of nested-list indentation under `ListIndentType::Spaces`.
//!
//! A nested list's indent must equal the CUMULATIVE width of every ancestor `<li>`'s own
//! marker ("- " = 2, "1. " = 3, "10. " = 4, ...), not a uniform `list_depth * list_indent_width`.
//! `"- "` is coincidentally 2 wide (the default `list_indent_width`), so ul-in-ul nesting was
//! already correct; an ordered ancestor's marker is wider, so the uniform scheme under-indents
//! and `CommonMark` parses the nested list as a sibling instead of nested content.

fn convert(html: &str) -> String {
    html_to_markdown_rs::convert(html, None)
        .unwrap()
        .content
        .unwrap_or_default()
}

#[test]
fn should_indent_ol_nested_in_ol_to_parent_marker_width() {
    let html = "<ol><li>outer<ol><li>inner</li></ol></li></ol>";
    let result = convert(html);
    assert_eq!(result, "1. outer\n   1. inner\n");
}

#[test]
fn should_indent_ol_nested_in_ul_to_parent_marker_width() {
    let html = "<ul><li>outer<ol><li>inner</li></ol></li></ul>";
    let result = convert(html);
    // The outer `<ul>` marker "- " is 2 columns wide, same as the default `list_indent_width`,
    // so this combination was already correct before the fix.
    assert_eq!(result, "- outer\n  1. inner\n");
}

#[test]
fn should_indent_ul_nested_in_ol_to_parent_marker_width() {
    let html = "<ol><li>outer<ul><li>inner</li></ul></li></ol>";
    let result = convert(html);
    assert_eq!(result, "1. outer\n   - inner\n");
}

#[test]
fn should_indent_nested_list_to_four_column_marker_when_parent_has_ten_or_more_items() {
    let html = "<ol><li>1</li><li>2</li><li>3</li><li>4</li><li>5</li><li>6</li><li>7</li><li>8</li>\
                <li>9</li><li>ten<ol><li>inner</li></ol></li></ol>";
    let result = convert(html);
    assert_eq!(
        result,
        "1. 1\n2. 2\n3. 3\n4. 4\n5. 5\n6. 6\n7. 7\n8. 8\n9. 9\n10. ten\n    1. inner\n"
    );
}

#[test]
fn should_indent_three_levels_deep_by_cumulative_marker_width() {
    let html = "<ol><li>l1<ol><li>l2<ol><li>l3</li></ol></li></ol></li></ol>";
    let result = convert(html);
    assert_eq!(result, "1. l1\n   1. l2\n      1. l3\n");
}

#[test]
fn should_indent_mixed_ul_ol_ul_nesting_by_cumulative_marker_width() {
    let html = "<ul><li>a<ol><li>b<ul><li>c</li></ul></li></ol></li></ul>";
    let result = convert(html);
    assert_eq!(result, "- a\n  1. b\n     * c\n");
}

#[test]
fn should_align_continuation_paragraph_to_content_column_inside_nested_ordered_item() {
    // Loose list item: a second <p> is a genuine continuation paragraph of the same <li>.
    let html = "<ol><li>outer<ol><li><p>inner</p><p>continuation text</p></li></ol></li></ol>";
    let result = convert(html);
    // "inner" starts at column 6 (3 for the outer "1. " marker + 3 for this item's own "1. "
    // marker); the continuation paragraph must align under that same column.
    assert_eq!(result, "1. outer\n   1. inner\n\n      continuation text\n");
}

#[test]
fn should_pad_same_line_continuation_text_by_content_column_inside_nested_ordered_item() {
    // A stray text node directly followed by a <p> (not wrapped in its own <p>): the paragraph
    // starts after a blank line at the item's content column (issue #583).
    let html = "<ol><li>outer<ol><li>inner<p>continuation text</p></li></ol></li></ol>";
    let result = convert(html);
    assert_eq!(result, "1. outer\n   1. inner\n\n      continuation text\n");
}
