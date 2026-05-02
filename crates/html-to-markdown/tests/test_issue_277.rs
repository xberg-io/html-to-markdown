#![allow(missing_docs)]

//! Regression test for issue #277: silent truncation on large HTML inputs.
//!
//! The bug was caused by `repair_with_html5ever` re-introducing `<script>` elements
//! that had already been stripped, and `preprocess_html` failing to find the closing
//! tag when script content contained unbalanced literal `<script>` strings.

fn convert(html: &str) -> String {
    html_to_markdown_rs::convert(html, None)
        .expect("conversion should not fail")
        .content
        .unwrap_or_default()
}

/// When custom elements trigger html5ever repair, scripts must be re-stripped.
/// Without the fix, content after a script with unbalanced `<script>` literals
/// would be silently truncated.
#[test]
fn test_no_truncation_after_repair_with_scripts() {
    // Custom element triggers repair_with_html5ever
    // Script content has an unbalanced literal `<script>` that confuses depth tracking
    let html = r"<html>
<head>
<script>
  var example = '<script>';
  console.log(example);
</script>
</head>
<body>
<custom-widget>widget</custom-widget>
<p>Content before</p>
<p>Content after scripts that must not be truncated</p>
<p>Final paragraph</p>
</body>
</html>";

    let result = convert(html);
    assert!(
        result.contains("Content before"),
        "Should contain content before script region"
    );
    assert!(
        result.contains("Content after scripts that must not be truncated"),
        "Content after scripts should NOT be silently truncated. Got:\n{result}"
    );
    assert!(
        result.contains("Final paragraph"),
        "Final content should be present. Got:\n{result}"
    );
}

/// Ensure `preprocess_html` doesn't truncate the rest of the document when
/// `find_closing_tag` returns None (unmatched script opening).
#[test]
fn test_preprocess_unmatched_script_preserves_remaining_content() {
    // Even without custom elements, preprocess_html's unwrap_or fallback
    // should not consume the entire rest of the document.
    let html = r"<html><body>
<p>Before</p>
<script>var x = '<script>'; var y = '<script>';</script>
<p>After first script</p>
<script>var z = 1;</script>
<p>After second script</p>
</body></html>";

    let result = convert(html);
    assert!(result.contains("Before"), "Content before scripts should be present");
    assert!(
        result.contains("After first script"),
        "Content after first script should be present. Got:\n{result}"
    );
    assert!(
        result.contains("After second script"),
        "Content after second script should be present. Got:\n{result}"
    );
}
