// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![allow(clippy::print_stdout, clippy::print_stderr, clippy::dbg_macro)] // ~keep: tests print by design

fn convert(
    html: &str,
    opts: Option<html_to_markdown_rs::ConversionOptions>,
) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

use html_to_markdown_rs::ConversionOptions;

#[test]
fn test_basic_unordered_list() {
    let html = r"<ul>
    <li>Item 1</li>
    <li>Item 2</li>
    <li>Item 3</li>
    </ul>";

    let result = convert(html, None).unwrap();
    assert!(result.contains("- Item 1"));
    assert!(result.contains("- Item 2"));
    assert!(result.contains("- Item 3"));
}

#[test]
fn test_basic_ordered_list() {
    let html = r"<ol>
    <li>First</li>
    <li>Second</li>
    <li>Third</li>
    </ol>";

    let result = convert(html, None).unwrap();
    assert!(result.contains("1. First"));
    assert!(result.contains("2. Second"));
    assert!(result.contains("3. Third"));
}

#[test]
fn test_nested_lists() {
    let html = r"<ul>
    <li>Item 1
        <ul>
            <li>Nested 1</li>
            <li>Nested 2</li>
        </ul>
    </li>
    <li>Item 2</li>
    </ul>";

    let result = convert(html, None).unwrap();
    assert!(result.contains("- Item 1"));
    assert!(result.contains("* Nested 1"));
    assert!(result.contains("* Nested 2"));
    assert!(result.contains("- Item 2"));
}

#[test]
fn test_ordered_nested_in_unordered() {
    let html = r"<ul>
    <li>Outer item
        <ol>
            <li>Inner item 1</li>
            <li>Inner item 2</li>
        </ol>
    </li>
    </ul>";

    let result = convert(html, None).unwrap();
    assert!(result.contains("- Outer item"));
    assert!(result.contains("1. Inner item 1"));
    assert!(result.contains("2. Inner item 2"));
}

#[test]
fn test_list_with_formatting() {
    let html = r"<ul>
    <li><strong>Bold</strong> item</li>
    <li><em>Italic</em> item</li>
    <li><code>Code</code> item</li>
    </ul>";

    let result = convert(html, None).unwrap();
    assert!(result.contains("- **Bold** item"));
    assert!(result.contains("- *Italic* item"));
    assert!(result.contains("- `Code` item"));
}

#[test]
fn test_list_with_links() {
    let html = r#"<ul>
    <li><a href="https://example.com">Link 1</a></li>
    <li><a href="https://example.org">Link 2</a></li>
    </ul>"#;

    let result = convert(html, None).unwrap();
    assert!(result.contains("[Link 1](https://example.com)"));
    assert!(result.contains("[Link 2](https://example.org)"));
}

#[test]
fn test_task_list() {
    let html = r#"<ul>
    <li><input type="checkbox" checked> Completed task</li>
    <li><input type="checkbox"> Incomplete task</li>
    </ul>"#;

    let result = convert(html, None).unwrap();
    assert!(result.contains("- [x] Completed task"));
    assert!(result.contains("- [ ] Incomplete task"));
}

#[test]
fn test_list_indent_spaces() {
    let html = r"<ul>
    <li>Parent
        <ul>
            <li>Child</li>
        </ul>
    </li>
    </ul>";

    let options = ConversionOptions {
        list_indent_type: html_to_markdown_rs::ListIndentType::Spaces,
        list_indent_width: 2,
        ..Default::default()
    };

    let result = convert(html, Some(options)).unwrap();
    assert!(result.contains("- Parent"));
    assert!(result.contains("  * Child"));
}

#[test]
fn test_list_indent_tabs() {
    let html = r"<ul>
    <li>Parent
        <ul>
            <li>Child</li>
        </ul>
    </li>
    </ul>";

    let options = ConversionOptions {
        list_indent_type: html_to_markdown_rs::ListIndentType::Tabs,
        ..Default::default()
    };

    let result = convert(html, Some(options)).unwrap();
    assert!(result.contains("- Parent"));
    assert!(result.contains("\t* Child"));
}

#[test]
fn test_custom_bullet_symbols() {
    let html = r"<ul>
    <li>Item 1</li>
    <li>Item 2</li>
    </ul>";

    let options = ConversionOptions {
        bullets: "*+-".to_string(),
        ..Default::default()
    };

    let result = convert(html, Some(options)).unwrap();
    assert!(result.contains("* Item 1") || result.contains("* Item 2"));
}

#[test]
fn test_empty_list_item() {
    let html = r"<ul>
    <li>Item 1</li>
    <li></li>
    <li>Item 3</li>
    </ul>";

    let result = convert(html, None).unwrap();
    assert!(result.contains("- Item 1"));
    assert!(result.contains("- Item 3"));
}

#[test]
fn test_list_with_code_block() {
    let html = r#"<ul>
    <li>
        <p>Item with code:</p>
        <pre><code>fn main() {
    println!("Hello");
}</code></pre>
    </li>
    </ul>"#;

    let result = convert(html, None).unwrap();
    println!("Result:\n{result}");
    assert!(result.contains("- Item with code:"));
    assert!(result.contains("fn main()"));
}

// Audit #10: `start` is untrusted external input. HTML permits any signed integer (browsers
// count downward from a negative `start`), and the value can overflow every fixed-width
// integer type, so the counter must never panic or silently wrap.

#[test]
fn should_render_zero_as_first_marker_when_start_attribute_is_zero() {
    let html = r#"<ol start="0"><li>First</li><li>Second</li></ol>"#;

    let result = convert(html, None).unwrap();
    assert_eq!(result, "0. First\n1. Second\n");
}

#[test]
fn should_count_downward_when_start_attribute_is_negative() {
    let html = r#"<ol start="-5"><li>First</li><li>Second</li><li>Third</li></ol>"#;

    let result = convert(html, None).unwrap();
    assert_eq!(result, "-5. First\n-4. Second\n-3. Third\n");
}

#[test]
fn should_default_to_one_when_start_attribute_is_not_a_valid_integer() {
    let html = r#"<ol start="not-a-number"><li>First</li><li>Second</li></ol>"#;

    let result = convert(html, None).unwrap();
    assert_eq!(result, "1. First\n2. Second\n");
}

#[test]
fn should_clamp_to_i64_max_when_start_attribute_equals_u64_max() {
    let html = r#"<ol start="18446744073709551615"><li>Only</li></ol>"#;

    let result = convert(html, None).unwrap();
    assert_eq!(result, format!("{}. Only\n", i64::MAX));
}

#[test]
fn should_clamp_to_i64_max_when_start_attribute_overflows_every_integer_type() {
    let html = r#"<ol start="999999999999999999999999999999999999999999"><li>Only</li></ol>"#;

    let result = convert(html, None).unwrap();
    assert_eq!(result, format!("{}. Only\n", i64::MAX));
}

#[test]
fn should_clamp_to_i64_min_when_start_attribute_negative_overflows_every_integer_type() {
    let html = r#"<ol start="-999999999999999999999999999999999999999999"><li>Only</li></ol>"#;

    let result = convert(html, None).unwrap();
    assert_eq!(result, format!("{}. Only\n", i64::MIN));
}

#[test]
fn should_render_i64_max_start_without_panicking_on_single_item() {
    let html = format!(r#"<ol start="{}"><li>Only</li></ol>"#, i64::MAX);

    let result = convert(&html, None).unwrap();
    assert_eq!(result, format!("{}. Only\n", i64::MAX));
}

#[test]
fn should_pin_counter_at_i64_max_without_overflow_when_list_exceeds_range() {
    let start = i64::MAX - 1;
    let html = format!(r#"<ol start="{start}"><li>First</li><li>Second</li><li>Third</li></ol>"#);

    let result = convert(&html, None).unwrap();
    assert_eq!(
        result,
        format!(
            "{start}. First\n{max}. Second\n{max}. Third\n",
            start = start,
            max = i64::MAX
        )
    );
}

/// A nested list directly following inline content that ends in whitespace INSIDE an inline
/// wrapper (`<strong>`, `<em>`) must still get its own line. `add_list_leading_separator`'s
/// bare-marker check used to look only at the last two bytes of the buffer (`"* "`, `"- "`,
/// `". "`); the closing `"**"` of `<strong>` followed by the wrapper's migrated trailing space
/// produces the exact same two bytes as a real `"* "` bullet, so the separator was skipped and
/// the nested list was flattened onto the parent line -- destroying it as a list on reparse.
#[test]
fn nested_list_after_strong_with_trailing_space_gets_its_own_line() {
    let html = "<ul><li><strong>b </strong><ul><li>sub</li></ul></li></ul>";
    let result = convert(html, None).unwrap();
    assert_eq!(result, "- **b**\n  * sub\n");
}

/// Same trigger as above, but with a real-world Unicode whitespace character (EN SPACE,
/// U+2002) instead of an ASCII space -- the shape that surfaced this in the generated fixture
/// corpus (`generated:seed=15118233204572906709`).
#[test]
fn nested_list_after_strong_with_trailing_unicode_space_gets_its_own_line() {
    let html = "<ul><li><strong>b\u{2002}</strong><ul><li>sub</li></ul></li></ul>";
    let result = convert(html, None).unwrap();
    assert_eq!(result, "- **b**\n  * sub\n");
}

/// Same trigger via `<em>` rather than `<strong>`: the closing single `"*"` plus the migrated
/// trailing space is likewise indistinguishable from a bare `"* "` bullet by a plain suffix
/// check.
#[test]
fn nested_list_after_emphasis_with_trailing_space_gets_its_own_line() {
    let html = "<ul><li><em>b </em><ul><li>sub</li></ul></li></ul>";
    let result = convert(html, None).unwrap();
    assert_eq!(result, "- *b*\n  * sub\n");
}

/// A bare-marker check based on the buffer's last two bytes must not misfire on real inline
/// content either: three single-child lists nested directly inside each other stack their bare
/// markers on one physical line with nothing else between them (`CommonMark` spec example 299),
/// and that must still be recognized as "just markers so far", not forced onto separate lines.
#[test]
fn deeply_nested_single_child_lists_stay_on_one_line() {
    let html = "<ol><li><ul><li><ol start=\"2\"><li>foo</li></ol></li></ul></li></ol>";
    let result = convert(html, None).unwrap();
    assert_eq!(result, "1. - 2. foo\n");
}

#[test]
fn task_item_leading_blockquote_starts_its_own_line() {
    // Anything between the checkbox and the quote that renders only whitespace (or nothing)
    // must not glue the quote's `>` onto the `[ ]` line as literal task text.
    for between in [
        "",
        "<br>",
        "<span>&nbsp;</span>",
        "<template>x</template>",
        "<noscript>x</noscript>",
    ] {
        let html = format!(r#"<ul><li><input type="checkbox">{between}<blockquote>q</blockquote></li></ul>"#);
        let result = convert(&html, None).unwrap();
        assert_eq!(result, "- [ ] &#32;\n  > q\n", "between = {between:?}");
    }
}

#[test]
fn nested_task_item_leading_blockquote_uses_nested_indent() {
    let html = r#"<ul><li><input type="checkbox"> a<ul><li><input type="checkbox"><br><blockquote>q</blockquote></li></ul></li></ul>"#;
    let result = convert(html, None).unwrap();
    assert_eq!(result, "- [ ] a\n  - [ ] &#32;\n    > q\n");
}

#[test]
fn task_item_text_before_blockquote_stays_on_marker_line() {
    let html = r#"<ul><li><input type="checkbox">a<blockquote>q</blockquote></li></ul>"#;
    let result = convert(html, None).unwrap();
    assert_eq!(result, "- [ ] a\n  > q\n");
}
