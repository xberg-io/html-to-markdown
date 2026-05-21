#![allow(missing_docs)]

//! Regression for #379: stack overflow on curl.se/changes.html.

#[test]
fn curl_changes_does_not_overflow_stack() {
    let html = include_str!("fixtures/regressions/curl_changes.html");
    // Pin a lower bound on the fixture size so a future refetch that
    // shrinks the file fails loudly rather than silently weakening
    // coverage.
    assert!(
        html.len() >= 1_400_000,
        "fixture appears truncated: {} bytes",
        html.len(),
    );
    let start = std::time::Instant::now();
    let result =
        html_to_markdown_rs::convert(html, None).expect("convert must not abort the process on curl.se/changes.html");
    assert!(
        start.elapsed() < std::time::Duration::from_secs(30),
        "conversion took too long: {:?}",
        start.elapsed(),
    );
    assert!(
        result.content.as_deref().unwrap_or("").len() > 100_000,
        "expected non-trivial markdown output, got {} bytes",
        result.content.as_deref().unwrap_or("").len(),
    );
}
