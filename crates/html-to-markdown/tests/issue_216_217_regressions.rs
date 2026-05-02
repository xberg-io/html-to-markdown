//! Regression tests for issues #216 and #217.
//!
//! Both issues report a panic at `text_node.rs:191`:
//! "byte index N is out of bounds of \`\`"
//!
//! Root cause: When inline handlers (strong, em, etc.) collect children into a
//! fresh String buffer while inheriting a parent context with `block_content_start`
//! set by a paragraph handler, the index points into the wrong buffer.

fn convert(
    html: &str,
    opts: Option<html_to_markdown_rs::ConversionOptions>,
) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

/// Minimal reproducer: a <details> containing a <p> with <strong> inside.
/// The <details> handler collects into a fresh buffer, the <p> sets
/// `block_content_start`, and the <strong> handler creates yet another fresh
/// buffer — causing the index to be out of bounds.
#[test]
fn test_issue_216_217_details_paragraph_strong_no_panic() {
    let html = r"
    <div>some preceding content</div>
    <details>
        <summary>Summary text</summary>
        <p><strong>Bold text inside details paragraph
</strong></p>
    </details>
    ";

    let result = convert(html, None);
    assert!(result.is_ok());
}

/// Same issue can occur with emphasis inside a paragraph inside details.
#[test]
fn test_issue_216_217_details_paragraph_em_no_panic() {
    let html = r"
    <div>some preceding content</div>
    <details>
        <summary>Summary</summary>
        <p><em>Italic text inside details paragraph
</em></p>
    </details>
    ";

    let result = convert(html, None);
    assert!(result.is_ok());
}

/// The panic can also occur with nested inline elements inside paragraphs
/// collected by any handler that creates a fresh buffer.
#[test]
fn test_issue_216_217_nested_strong_in_paragraph_no_panic() {
    let html = r"
    <details>
        <p>Some text <strong>bold text with trailing newline
</strong> more text</p>
    </details>
    ";

    let result = convert(html, None);
    assert!(result.is_ok());
}

/// Test with the actual structure pattern from the reported URL.
#[test]
fn test_issue_216_217_complex_details_structure() {
    let html = r#"
    <div class="content">
        <section>
            <details>
                <summary>Stratégie adaptation</summary>
                <p><strong>Risque élevé pour les populations
</strong></p>
                <p>Description du risque avec des détails supplémentaires.</p>
            </details>
        </section>
    </div>
    "#;

    let result = convert(html, None);
    assert!(result.is_ok());
    let md = result.unwrap();
    assert!(md.contains("Risque"));
}
