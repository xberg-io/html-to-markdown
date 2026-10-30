//! Table sections follow HTML rendering order in text and structured output.

#![cfg(feature = "testkit")]

use html_to_markdown_rs::{ConversionOptions, OutputFormat, TierStrategy, convert};

#[test]
fn table_sections_follow_header_body_footer_order() {
    let head = "<thead><tr><th>Header</th></tr></thead>";
    let body = "<tbody><tr><td>Body</td></tr></tbody>";
    let foot = "<tfoot><tr><td>Footer</td></tr></tfoot>";
    let direct = "<tr><td>Direct</td></tr>";
    for (sections, expected) in [
        (format!("{head}{foot}{body}"), vec!["Header", "Body", "Footer"]),
        (format!("{foot}{head}{body}"), vec!["Header", "Body", "Footer"]),
        (
            format!("{head}{foot}{body}{body}"),
            vec!["Header", "Body", "Body", "Footer"],
        ),
        (
            format!("{head}{body}{foot}{body}"),
            vec!["Header", "Body", "Body", "Footer"],
        ),
        (format!("{head}{foot}{direct}"), vec!["Header", "Direct", "Footer"]),
    ] {
        let html = format!("<table>{sections}</table>");
        for tier_strategy in [TierStrategy::Auto, TierStrategy::Tier1, TierStrategy::Tier2] {
            for output_format in [OutputFormat::Markdown, OutputFormat::Plain] {
                for structure in [false, true] {
                    assert_table_order(&html, &expected, tier_strategy, output_format, structure);
                }
            }
        }
    }
}

fn assert_table_order(
    html: &str,
    expected: &[&str],
    tier_strategy: TierStrategy,
    output_format: OutputFormat,
    include_document_structure: bool,
) {
    let result = convert(
        html,
        Some(ConversionOptions {
            tier_strategy,
            output_format,
            include_document_structure,
            ..ConversionOptions::default()
        }),
    )
    .unwrap();
    let content = result.content.unwrap();
    let words: Vec<_> = content
        .split(|c: char| !c.is_alphabetic())
        .filter(|word| !word.is_empty())
        .collect();
    assert_eq!(words, expected, "{html} {tier_strategy:?} {output_format:?}");
    if include_document_structure {
        assert_eq!(result.tables.len(), 1);
        let cells: Vec<_> = result.tables[0]
            .grid
            .cells
            .iter()
            .map(|cell| cell.content.as_str())
            .collect();
        assert_eq!(cells, expected, "structured grid: {html}");
    }
}
