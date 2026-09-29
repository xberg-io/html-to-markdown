//! Regression test for quadratic time in list items.
//!
//! An item between inline markers checked every earlier item of its list to see whether a
//! paragraph was open before its marker. Each block of an item checked every earlier line of the
//! item to see whether the item was still open (issue #649). A nested list that cannot interrupt
//! a paragraph checks the lines before it the same way (issue #662). The library converts
//! untrusted HTML, so each is a denial-of-service vector.

use std::time::{Duration, Instant};

use html_to_markdown_rs::options::HeadingStyle;
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

/// ~keep Same instrument as `wrap_opener_run_complexity.rs`: linear code doubles its time when
/// ~keep the input doubles, quadratic code quadruples it, and 3.0 separates the two with room for
/// ~keep noise.
const MAX_DOUBLING_RATIO: f64 = 3.0;

/// ~keep A failure must reproduce on every independent attempt; one that does not is noise.
const MAX_MEASUREMENT_ATTEMPTS: usize = 3;

/// ~keep The fastest of this many runs is one sample.
const REPEATS_PER_SAMPLE: usize = 3;

/// ~keep Small enough that the quadratic checks still finish in seconds in a debug build, large
/// ~keep enough that their fourfold growth is not lost in noise.
const BASE_SIZE: usize = 2_000;

/// The name, the HTML for `n` repeats, and whether the headings are underlined.
fn shapes(n: usize) -> [(&'static str, String, bool); 13] {
    [
        (
            "items of paragraphs",
            format!("<ul>{}</ul>", "<li><p>a</p><p>b</p></li>".repeat(n)),
            false,
        ),
        (
            "items of paragraphs around a nested item",
            format!(
                "<ul>{}</ul>",
                "<li>a<ul><li>b<p>x</p><p>y</p></li></ul><p>c</p><p>d</p></li>".repeat(n)
            ),
            false,
        ),
        (
            "items between markers",
            format!("<b><ul><li>a<ol>{}</ol></li></ul></b>", "<li>x</li>".repeat(n)),
            false,
        ),
        (
            "lists between markers",
            format!(
                "<b><ul><li>a{}</li></ul></b>",
                "<ol start=\"2\"><li>x</li></ol>".repeat(n)
            ),
            false,
        ),
        (
            "lists after text in one item",
            format!("<ul><li>a{}</li></ul>", "<ol start=\"2\"><li>x</li></ol>".repeat(n)),
            false,
        ),
        (
            "empty lists after text in one item",
            format!("<ul><li>a{}</li></ul>", "<ul><li></li></ul>".repeat(n)),
            false,
        ),
        (
            "lists of empty items after text in one item",
            format!("<ul><li>a{}</li></ul>", "<ol><li></li></ol>b".repeat(n)),
            false,
        ),
        (
            "underlined headings in one item",
            format!("<ul><li>{}</li></ul>", "a<h2>q</h2>".repeat(n)),
            true,
        ),
        (
            "headings in one item",
            format!("<ul><li>{}</li></ul>", "a<h2>q</h2>".repeat(n)),
            false,
        ),
        (
            "paragraphs and quotes in one item",
            format!("<ul><li>{}</li></ul>", "<p>a</p><blockquote>q</blockquote>".repeat(n)),
            false,
        ),
        (
            "text and quotes in one item",
            format!("<ul><li>{}</li></ul>", "a<blockquote>q</blockquote>".repeat(n)),
            false,
        ),
        (
            "paragraphs in quotes in one item",
            format!(
                "<ul><li>{}</li></ul>",
                "<p>a</p><blockquote><p>x</p><p>y</p></blockquote>".repeat(n)
            ),
            false,
        ),
        (
            "paragraphs in sections in one item",
            format!(
                "<ul><li>{}</li></ul>",
                "<p>a</p><section><p>x</p><p>y</p></section>".repeat(n)
            ),
            false,
        ),
    ]
}

fn fastest_convert(html: &str, underlined: bool) -> Duration {
    let options = ConversionOptions {
        extract_metadata: false,
        tier_strategy: TierStrategy::Tier2,
        heading_style: if underlined {
            HeadingStyle::Underlined
        } else {
            HeadingStyle::Atx
        },
        ..ConversionOptions::default()
    };
    (0..REPEATS_PER_SAMPLE)
        .map(|_| {
            let start = Instant::now();
            let result = convert(html, Some(options.clone()));
            let elapsed = start.elapsed();
            assert!(result.is_ok(), "conversion must succeed");
            elapsed
        })
        .min()
        .expect("at least one run")
}

/// The two doubling ratios of one shape, or a description of why they look quadratic.
fn measure(index: usize) -> Result<(), String> {
    let seconds: Vec<f64> = [BASE_SIZE, BASE_SIZE * 2, BASE_SIZE * 4]
        .into_iter()
        .map(|n| {
            let (_, html, underlined) = &shapes(n)[index];
            fastest_convert(html, *underlined).as_secs_f64().max(1e-6)
        })
        .collect();
    let first = seconds[1] / seconds[0];
    let second = seconds[2] / seconds[1];
    if first < MAX_DOUBLING_RATIO && second < MAX_DOUBLING_RATIO {
        return Ok(());
    }
    let name = shapes(1)[index].0;
    Err(format!(
        "{name}: doublings took {first:.1}x and {second:.1}x as long ({:.4}s, {:.4}s, {:.4}s); \
         expected under {MAX_DOUBLING_RATIO}x each",
        seconds[0], seconds[1], seconds[2]
    ))
}

#[test]
fn list_item_checks_scale_linearly() {
    for index in 0..shapes(1).len() {
        let mut failures = Vec::with_capacity(MAX_MEASUREMENT_ATTEMPTS);
        for attempt in 1..=MAX_MEASUREMENT_ATTEMPTS {
            match measure(index) {
                Ok(()) => break,
                Err(reason) => failures.push(format!("attempt {attempt}: {reason}")),
            }
        }
        assert!(
            failures.len() < MAX_MEASUREMENT_ATTEMPTS,
            "list item checks scaled super-linearly on every attempt:\n{}",
            failures.join("\n")
        );
    }
}
