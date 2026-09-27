#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #565: Tier 1's fallback message for a character reference without
//! its `;` must show the reference as the page wrote it, not with a `;` added.

use html_to_markdown_rs::ConversionOptions;
use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::tier1;

fn tier1_message(html: &str) -> Option<String> {
    tier1::run(html, &PrescanReport::default(), &ConversionOptions::default())
        .err()
        .map(|reason| reason.to_string())
}

#[test]
fn fallback_message_shows_a_reference_without_a_semicolon_as_written() {
    for (html, expected) in [
        (
            "<p>it&#39s</p>",
            "HTML entity &#39 is missing its closing semicolon at byte offset 5",
        ),
        (
            "<p>it&#x27s</p>",
            "HTML entity &#x27 is missing its closing semicolon at byte offset 5",
        ),
        (
            "<p>&copy 2024</p>",
            "HTML entity &copy is missing its closing semicolon at byte offset 3",
        ),
    ] {
        assert_eq!(tier1_message(html).as_deref(), Some(expected), "{html:?}");
    }
}

#[test]
fn reference_with_a_semicolon_does_not_fall_back() {
    for html in ["<p>it&#39;s</p>", "<p>&copy; 2024</p>"] {
        assert_eq!(tier1_message(html), None, "{html:?}");
    }
}
