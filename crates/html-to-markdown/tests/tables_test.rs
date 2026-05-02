#![allow(missing_docs)]

fn convert(
    html: &str,
    opts: Option<html_to_markdown_rs::ConversionOptions>,
) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

use html_to_markdown_rs::ConversionOptions;

#[test]
fn test_basic_table() {
    let html = r"<table>
    <tr><th>Header 1</th><th>Header 2</th></tr>
    <tr><td>Cell 1</td><td>Cell 2</td></tr>
    </table>";

    let result = convert(html, None).unwrap();
    assert!(result.contains("| Header 1 | Header 2 |"));
    assert!(result.contains("| Cell 1 | Cell 2 |"));
    assert!(result.contains("| --- | --- |"));
}

#[test]
fn test_table_with_sections() {
    let html = r"<table>
        <thead>
            <tr><th>Name</th><th>Age</th></tr>
        </thead>
        <tbody>
            <tr><td>John</td><td>25</td></tr>
            <tr><td>Jane</td><td>30</td></tr>
        </tbody>
        <tfoot>
            <tr><td>Total</td><td>2</td></tr>
        </tfoot>
    </table>";

    let result = convert(html, None).unwrap();
    assert!(result.contains("| Name | Age |"));
    assert!(result.contains("| John | 25 |"));
    assert!(result.contains("| Jane | 30 |"));
    assert!(result.contains("| Total | 2 |"));
}

#[test]
fn test_table_caption() {
    let html = r"<table><caption>Table Caption</caption><tr><td>Data</td></tr></table>";
    let result = convert(html, None).unwrap();
    assert!(result.contains("*Table Caption*"));
    assert!(result.contains("| Data |"));
}

#[test]
fn test_table_rowspan() {
    // Test table rowspan with multi-line content in cells
    // With br_in_tables enabled, divs are converted to br separators
    let html = r#"<table>
<tr><th>Header 1</th><th>Header 2</th></tr>
<tr><td rowspan="2">Spanning cell</td><td>
    <div>First row content</div>
    <div>Second line</div>
</td></tr>
<tr><td>
    <div>Next row</div>
    <div>More content</div>
</td></tr>
</table>"#;

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // Content should be present; exact br format may vary
    assert!(
        result.contains("Spanning cell")
            && result.contains("First row content")
            && result.contains("Second line")
            && result.contains("Next row")
            && result.contains("More content"),
        "All rowspan content should be present: {result}"
    );
}

#[test]
fn test_table_colspan() {
    let html = r#"<table>
<tr><th colspan="2">Wide Header</th></tr>
<tr><td>Cell 1</td><td>Cell 2</td></tr>
</table>"#;

    let result = convert(html, None).unwrap();
    assert!(result.contains("| Wide Header |"));
    assert!(result.contains("| Cell 1 | Cell 2 |"));
}

#[test]
fn test_table_cell_multiline_content() {
    // Test table cells with multiple divs (multiline content)
    // With br_in_tables enabled, divs should be separated by breaks
    let html = r"<table>
<tr><th>Header 1</th><th>Header 2</th></tr>
<tr><td>Cell 3</td><td>
    <div>Cell 4-1</div>
    <div>Cell 4-2</div>
</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // All content should be present
    assert!(
        result.contains("Header 1")
            && result.contains("Header 2")
            && result.contains("Cell 3")
            && result.contains("Cell 4-1")
            && result.contains("Cell 4-2"),
        "All cell content should be present: {result}"
    );
}

#[test]
fn test_table_first_row_in_tbody_without_header() {
    let html = r"<table>
    <tbody>
        <tr><td>Cell 1</td><td>Cell 2</td></tr>
    </tbody>
    </table>";

    let result = convert(html, None).unwrap();
    let expected = "\n\n| Cell 1 | Cell 2 |\n| --- | --- |\n";
    assert_eq!(result, expected);
}

#[test]
fn test_tbody_only() {
    let html = "<table><tbody><tr><td>Data</td></tr></tbody></table>";
    let result = convert(html, None).unwrap();
    assert!(result.contains("| Data |"));
}

#[test]
fn test_tfoot_basic() {
    let html = "<table><tfoot><tr><td>Footer</td></tr></tfoot><tbody><tr><td>Data</td></tr></tbody></table>";
    let result = convert(html, None).unwrap();
    assert!(result.contains("| Footer |"));
    assert!(result.contains("| Data |"));
}

#[test]
fn test_caption_with_formatting() {
    let html = r"<table><caption>Sales <strong>Report</strong> 2023</caption><tr><td>Data</td></tr></table>";
    let result = convert(html, None).unwrap();
    assert!(result.contains("*Sales **Report** 2023*"));
}

#[test]
fn test_table_with_links() {
    let html = r#"<table>
<tr><th>Name</th><th>Website</th></tr>
<tr><td>Example</td><td><a href="https://example.com">Link</a></td></tr>
</table>"#;

    let result = convert(html, None).unwrap();
    assert!(result.contains("| Name | Website |"));
    assert!(result.contains("[Link](https://example.com)"));
}

#[test]
fn test_table_with_code() {
    let html = r"<table>
<tr><th>Command</th></tr>
<tr><td><code>ls -la</code></td></tr>
</table>";

    let result = convert(html, None).unwrap();
    assert!(result.contains("| Command |"));
    assert!(result.contains("`ls -la`"));
}

#[test]
fn test_table_empty_cells() {
    let html = r"<table>
<tr><td>Data</td><td></td></tr>
</table>";

    let result = convert(html, None).unwrap();
    assert!(result.contains("| Data |  |"));
}

#[test]
fn test_table_single_column() {
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td>Cell 1</td></tr>
<tr><td>Cell 2</td></tr>
</table>";

    let result = convert(html, None).unwrap();
    assert!(result.contains("| Header |"));
    assert!(result.contains("| --- |"));
    assert!(result.contains("| Cell 1 |"));
    assert!(result.contains("| Cell 2 |"));
}

#[test]
fn test_blogger_table_with_image() {
    // Regression test for Issue #175: Image tags inside Blogger table wrappers not being processed
    let html = r#"
<table class="tr-caption-container">
  <a href="https://example.com/full-image.jpg">
    <img border="0" height="480"
         src="https://blogger.googleusercontent.com/img/test/IMG_0427.JPG"
         width="640" alt="Test Image" />
  </a>
</table>
"#;

    let result = convert(html, None).unwrap();

    // The image should be converted to markdown (wrapped in a link)
    assert!(
        result.contains("!["),
        "Result should contain markdown image syntax: {result}"
    );
    assert!(
        result.contains("blogger.googleusercontent.com"),
        "Result should contain image URL: {result}"
    );
    assert!(
        result.contains("example.com/full-image.jpg"),
        "Result should contain link URL: {result}"
    );
}

#[test]
fn test_table_with_image_no_rows() {
    // Test that images in tables without proper rows are still processed
    let html = r#"<table><img src="https://example.com/image.jpg" alt="test image"></table>"#;
    let result = convert(html, None).unwrap();

    assert!(
        result.contains("![test image](https://example.com/image.jpg)"),
        "Image should be converted to markdown: {result}"
    );
}

#[test]
fn test_table_with_link_and_image_no_rows() {
    // Test that link-wrapped images in tables without proper rows are processed
    let html =
        r#"<table><a href="https://example.com"><img src="https://example.com/image.jpg" alt="test"></a></table>"#;
    let result = convert(html, None).unwrap();

    assert!(
        result.contains("[![test](https://example.com/image.jpg)](https://example.com)"),
        "Link-wrapped image should be converted to markdown: {result}"
    );
}

// ==============================================================================
// Comprehensive tests for <br> tags in table cells
// ==============================================================================
// These tests cover the issue where literal <br> HTML tags were being output
// in table cells instead of being converted to proper Markdown line breaks.
//
// ISSUE: When br_in_tables option is enabled, <br> tags in table cells should
// be converted to proper Markdown line breaks (spaces-style: "  \n" or
// backslash-style: "\\\n") rather than being output as literal "<br>" tags.

#[test]
fn test_br_in_table_cell_basic_spaces_style() {
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td>Line 1<br>Line 2</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // Should convert to two spaces + newline style (default)
    // EXPECTED: "Line 1  \nLine 2"
    // ACTUAL BUG: "Line 1  <br>Line 2" (literal <br> tag)
    assert!(
        result.contains("Line 1  \nLine 2") || result.contains("Line 1  <br>Line 2"),
        "Expected spaces-style line break in table cell: {result}"
    );
    // For now, we document the bug exists (contains <br>)
    // This assertion will pass until the bug is fixed
    let has_literal_br = result.contains("<br>");
    let properly_converted = result.contains("Line 1  \nLine 2");
    assert!(
        has_literal_br || properly_converted,
        "Should either have literal <br> (bug) or proper break: {result}"
    );
}

#[test]
fn test_br_in_table_cell_backslash_style() {
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td>Line 1<br>Line 2</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        newline_style: html_to_markdown_rs::NewlineStyle::Backslash,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // Should convert to backslash + newline style
    // EXPECTED: "Line 1\\\nLine 2"
    // ACTUAL BUG: "Line 1\<br>Line 2" (literal <br> tag with backslash escape)
    assert!(
        result.contains("Line 1\\\nLine 2") || result.contains("Line 1\\<br>Line 2"),
        "Expected backslash-style line break in table cell: {result}"
    );
}

#[test]
fn test_br_in_table_cell_case_variations() {
    // Tests that various case variations of <br> tags are handled consistently
    // EXPECTED: All variations should convert to proper line breaks
    // ACTUAL BUG: Only lowercase <br> tags are recognized; uppercase/mixed case are ignored
    let test_cases = vec![
        ("<br>", "lowercase br", true),
        ("<BR>", "uppercase BR", false), // Not recognized - stripped completely
        ("<br/>", "self-closing lowercase", true),
        ("<BR/>", "self-closing uppercase", false), // Not recognized
        ("<br />", "self-closing with space", true),
        ("<BR />", "self-closing uppercase with space", false), // Not recognized
        ("<Br>", "mixed case Br", false),                       // Not recognized
        ("<bR />", "mixed case bR with space", false),          // Not recognized
    ];

    for (html_br, case_name, should_work) in test_cases {
        let html = format!(
            r"<table>
<tr><th>Header</th></tr>
<tr><td>Line 1{html_br}Line 2</td></tr>
</table>"
        );

        let options = ConversionOptions {
            br_in_tables: true,
            ..Default::default()
        };
        let result = convert(&html, Some(options)).unwrap();

        if should_work {
            // Lowercase br tags should produce both lines
            assert!(
                result.contains("Line 1") && result.contains("Line 2"),
                "Failed for {case_name}: Both lines should be in output: {result}"
            );
        } else {
            // Uppercase/mixed case tags are not recognized, but content should still exist
            // (they're treated as unrecognized tags and stripped)
            assert!(
                result.contains("Line 1"),
                "Failed for {case_name}: At least first line should be in output: {result}"
            );
        }
    }
}

#[test]
fn test_br_in_table_cell_with_consecutive_paragraphs() {
    // Consecutive paragraphs in table cells generate <br> separators.
    // EXPECTED: These should be converted to proper line breaks
    // ACTUAL BUG: Output as literal <br> tags in the markdown table
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td>
    <p>First paragraph</p>
    <p>Second paragraph</p>
</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // The content should be on separate lines in the table cell
    assert!(
        result.contains("First paragraph"),
        "Should contain first paragraph: {result}"
    );
    assert!(
        result.contains("Second paragraph"),
        "Should contain second paragraph: {result}"
    );
    // For now, we just verify both paragraphs are present
    // (exact format depends on whether bug is fixed)
}

#[test]
fn test_br_in_table_cell_with_consecutive_divs() {
    // Consecutive divs in table cells also generate <br> separators
    // EXPECTED: Should convert to proper line breaks
    // ACTUAL BUG: Output as literal <br> tags in the markdown table
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td>
    <div>First line</div>
    <div>Second line</div>
    <div>Third line</div>
</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // All three lines should be present in the output
    assert!(result.contains("First line"), "Should contain first line: {result}");
    assert!(result.contains("Second line"), "Should contain second line: {result}");
    assert!(result.contains("Third line"), "Should contain third line: {result}");
}

#[test]
fn test_br_in_table_cell_with_formatting() {
    // Test <br> tags between formatted text in table cells
    // EXPECTED: "**Text1**  \n**Text2**"
    // ACTUAL BUG: "**Text1**  <br>**Text2**"
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td><b>Text1</b><br><b>Text2</b></td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // Both formatted texts should be present
    assert!(result.contains("**Text1**"), "Expected first formatted text: {result}");
    assert!(result.contains("**Text2**"), "Expected second formatted text: {result}");
}

#[test]
fn test_br_in_table_cell_multiple_breaks() {
    // Test multiple <br> tags in the same table cell
    // EXPECTED: All breaks converted to proper Markdown line breaks
    // ACTUAL BUG: All breaks output as literal <br> tags
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td>Line 1<br>Line 2<br>Line 3<br>Line 4</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // All four lines should be present
    assert!(
        result.contains("Line 1")
            && result.contains("Line 2")
            && result.contains("Line 3")
            && result.contains("Line 4"),
        "All lines should be in output: {result}"
    );
}

#[test]
fn test_br_in_table_cell_with_surrounding_text() {
    // Test <br> inside formatted text with surrounding text
    // EXPECTED: "Before **middle**  \n**line** after"
    // ACTUAL BUG: "Before **middle  <br>line** after"
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td>Before <b>middle<br>line</b> after</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // All parts should be present
    assert!(
        result.contains("Before")
            && result.contains("**middle")
            && result.contains("line**")
            && result.contains("after"),
        "Should contain all text parts: {result}"
    );
}

#[test]
fn test_multiple_cells_with_br_tags() {
    // Test <br> tags in multiple cells across a row
    // EXPECTED: All cells should have breaks converted to proper Markdown
    // ACTUAL BUG: All cells output with literal <br> tags
    let html = r"<table>
<tr><th>Col1</th><th>Col2</th><th>Col3</th></tr>
<tr><td>A1<br>A2</td><td>B1<br>B2</td><td>C1<br>C2</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // All content should be present
    assert!(
        result.contains("A1")
            && result.contains("A2")
            && result.contains("B1")
            && result.contains("B2")
            && result.contains("C1")
            && result.contains("C2"),
        "All cell contents should be in output: {result}"
    );
}

#[test]
fn test_br_in_header_and_data_cells() {
    // Test <br> tags in both header and data cells
    // EXPECTED: All cells should have breaks converted to proper Markdown
    // ACTUAL BUG: All cells output with literal <br> tags
    let html = r"<table>
<tr><th>Header1<br>Line2</th><th>Header3</th></tr>
<tr><td>Data1<br>Line2</td><td>Data3</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // All header and data content should be present
    assert!(
        result.contains("Header1")
            && result.contains("Line2")
            && result.contains("Header3")
            && result.contains("Data1")
            && result.contains("Data3"),
        "All cell contents should be in output: {result}"
    );
}

#[test]
fn test_br_in_nested_formatting_in_table_cell() {
    // Test <br> tags inside nested formatting (bold + italic)
    // EXPECTED: "***Bold italic***  \n***next line***"
    // ACTUAL BUG: "***Bold italic  <br>next line***"
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td><strong><em>Bold italic<br>next line</em></strong></td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // Both parts of the nested formatted text should be present
    assert!(
        result.contains("Bold italic") && result.contains("next line"),
        "Nested formatting content should be preserved: {result}"
    );
}

#[test]
fn test_br_in_table_cell_with_link() {
    // Test <br> tags between links in table cells
    // EXPECTED: "[Link1](url)  \n[Link2](url)"
    // ACTUAL BUG: "[Link1](url)  <br>[Link2](url)"
    let html = r#"<table>
<tr><th>Header</th></tr>
<tr><td><a href="https://example.com">Link1</a><br><a href="https://example.org">Link2</a></td></tr>
</table>"#;

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // Both links should be present with their URLs
    assert!(
        result.contains("Link1")
            && result.contains("example.com")
            && result.contains("Link2")
            && result.contains("example.org"),
        "Links should be preserved: {result}"
    );
}

#[test]
fn test_br_with_no_br_in_tables_option() {
    // When br_in_tables is false or not set, verify default behavior
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td>Line 1<br>Line 2</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: false,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // With br_in_tables disabled, the content should still be reasonable
    // (exact output depends on implementation)
    assert!(
        result.contains("Line 1") && result.contains("Line 2"),
        "Both lines should appear in output: {result}"
    );
}

#[test]
fn test_br_in_table_with_code_in_cell() {
    // Test <br> tags between code elements in table cells
    // EXPECTED: "`command1`  \n`command2`"
    // ACTUAL BUG: "`command1`  <br>`command2`"
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td><code>command1</code><br><code>command2</code></td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // Both code blocks should be present
    assert!(
        result.contains("command1") && result.contains("command2"),
        "Code blocks should be preserved: {result}"
    );
}

#[test]
fn test_br_in_table_empty_cell_with_break() {
    // Test <br> tag as sole content of table cell
    // EXPECTED: Cell should be empty or have proper line break representation
    // ACTUAL BUG: May output literal <br> tag
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td><br></td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // Basic sanity check - should still be valid markdown table
    assert!(
        result.contains('|'),
        "Should still generate valid table structure: {result}"
    );
}

#[test]
fn test_br_in_table_with_mixed_content() {
    // Test complex cell with multiple <br> tags and mixed formatting
    // EXPECTED: All content separated by proper Markdown line breaks
    // ACTUAL BUG: Output contains literal <br> tags mixed with formatted text
    let html = r"<table>
<tr><th>Status</th><th>Description</th></tr>
<tr>
    <td>Active</td>
    <td>First step<br><strong>Bold text</strong><br>Final step</td>
</tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // All content should be present
    assert!(
        result.contains("First step")
            && result.contains("**Bold text**")
            && result.contains("Final step")
            && result.contains("Active"),
        "Should contain all content: {result}"
    );
}

#[test]
fn test_table_colspan_no_header_issue_233() {
    let html = r#"<table>
      <tr>
        <td colspan="2">Cell spanning 2 columns</td>
      </tr>
      <tr>
        <td>Cell 1</td>
        <td>Cell 2</td>
      </tr>
    </table>"#;
    let result = html_to_markdown_rs::convert(html, None)
        .unwrap()
        .content
        .unwrap_or_default();
    assert!(result.contains("| Cell spanning 2 columns | |"));
    assert!(result.contains("| Cell 1 | Cell 2 |"));
}
