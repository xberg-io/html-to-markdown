#![allow(missing_docs, clippy::print_stdout)]
#![cfg(feature = "testkit")]

use std::fmt::Write;

use html_to_markdown_rs::{ConversionOptions, convert};

fn formatting_tags(count: usize) -> String {
    (0..count).fold(String::new(), |mut bold, id| {
        write!(bold, "<b id='b{id}'>").expect("writing to String succeeds");
        bold
    })
}

#[test]
fn amplified_repair_should_fall_back_without_losing_authored_text() {
    let bold = formatting_tags(250);
    for paragraphs in [1_000, 4_000] {
        let html = format!("<p>{bold}x</p>{}", "<p>y</p>".repeat(paragraphs));
        let result = convert(
            &html,
            ConversionOptions {
                max_depth: Some(512),
                ..Default::default()
            },
        )
        .expect("amplified input converts");
        let content = result.content.expect("Markdown exists");
        assert_eq!(content.matches('x').count(), 1);
        assert_eq!(content.matches('y').count(), paragraphs);
        assert!(
            result
                .warnings
                .iter()
                .any(|warning| warning.kind == html_to_markdown_rs::WarningKind::MalformedHtml)
        );
    }
}

#[test]
fn ordinary_formatting_repair_should_keep_all_text_without_a_budget_warning() {
    for (formatting, paragraphs) in [(5, 200), (250, 2)] {
        let bold = formatting_tags(formatting);
        let html = format!("<p>{bold}x</p>{}", "<p>y</p>".repeat(paragraphs));
        let result = convert(
            &html,
            ConversionOptions {
                max_depth: Some(512),
                ..Default::default()
            },
        )
        .expect("ordinary formatting converts");
        let content = result.content.expect("Markdown exists");
        assert_eq!(content.matches('x').count(), 1);
        assert_eq!(content.matches('y').count(), paragraphs);
        assert!(result.warnings.is_empty(), "{:?}", result.warnings);
    }
}

#[test]
fn bounded_deep_repair_should_keep_a_repaired_shallow_cell_and_warn() {
    for prefix in ["<table><td>visible", "<b><p>visible</p></b>", "<x-custom>visible"] {
        let html = format!("{prefix}{}tail", "<div>".repeat(20_000));
        let result = convert(&html, ConversionOptions::default()).expect("deep input converts");
        let content = result.content.expect("Markdown exists");
        assert_eq!(content.matches("visible").count(), 1, "{content:?}");
        assert_eq!(content.matches("tail").count(), 0);
        assert!(
            result
                .warnings
                .iter()
                .any(|warning| warning.kind == html_to_markdown_rs::WarningKind::DepthLimitExceeded)
        );
    }
}

fn fastest_deep_repair(prefix: &str, count: usize) -> std::time::Duration {
    let html = format!("{prefix}{}tail", "<div>".repeat(count));
    (0..3)
        .map(|_| {
            let start = std::time::Instant::now();
            let result = convert(&html, ConversionOptions::default()).expect("deep input converts");
            assert_eq!(
                result.content.as_deref().unwrap_or_default().matches("visible").count(),
                1
            );
            assert!(
                result
                    .warnings
                    .iter()
                    .any(|warning| warning.kind == html_to_markdown_rs::WarningKind::DepthLimitExceeded)
            );
            start.elapsed()
        })
        .min()
        .expect("three measurements")
}

#[test]
fn deep_repair_should_scale_linearly_on_every_fallback_route() {
    for prefix in ["<table><td>visible", "<b><p>visible</p></b>", "<x-custom>visible"] {
        let mut measurements = Vec::new();
        for _ in 0..3 {
            let small = fastest_deep_repair(prefix, 5_000).as_secs_f64();
            let medium = fastest_deep_repair(prefix, 10_000).as_secs_f64();
            let large = fastest_deep_repair(prefix, 20_000).as_secs_f64();
            measurements.push((small, medium, large));
            if medium < small * 3.0 && large < medium * 3.0 {
                break;
            }
        }
        println!("{prefix}: {measurements:?}");
        assert!(
            measurements
                .iter()
                .any(|(small, medium, large)| *medium < small * 3.0 && *large < medium * 3.0),
            "{measurements:?}"
        );
    }
}
