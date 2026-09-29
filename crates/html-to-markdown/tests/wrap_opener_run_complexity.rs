//! Regression test for quadratic time in wrap mode's reflow on runs of words that open a block.
//!
//! The reflow moves a word that would open a block (`-`, `*`, `---`) to the end of the line
//! before it and checks lines again after a move. Checking a growing line again, or moving a
//! word through a run of `*` lines one line at a time, made a paragraph of such words O(n^2).
//! The library converts untrusted HTML, so this is a denial-of-service vector.

use std::time::{Duration, Instant};

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

/// ~keep Same instrument as `bare_lt_complexity.rs`: linear code doubles its time when the input
/// ~keep doubles, quadratic code quadruples it, and 3.0 separates the two with room for noise.
const MAX_DOUBLING_RATIO: f64 = 3.0;

/// ~keep A failure must reproduce on every independent attempt; one that does not is noise.
const MAX_MEASUREMENT_ATTEMPTS: usize = 3;

/// ~keep The fastest of this many runs is one sample.
const REPEATS_PER_SAMPLE: usize = 3;

/// ~keep Small enough that the quadratic reflow still finishes in seconds, large enough that
/// ~keep its fourfold growth is not lost in noise.
const BASE_SIZE: usize = 5_000;

/// A paragraph of `n` repeats of a word run, and the wrap width that makes the reflow move them.
fn shapes(n: usize) -> [(&'static str, String, usize); 4] {
    [
        ("rule words", format!("<p>{}x</p>", "--- ".repeat(n)), 1),
        ("single dashes", format!("<p>a {}x</p>", "- ".repeat(n)), 1),
        ("star lines", format!("<p>a {}x</p>", "* ".repeat(n)), 10),
        (
            "star lines then items",
            format!("<p>a {}{}x</p>", "* ".repeat(n), "- x ".repeat(n)),
            1,
        ),
    ]
}

fn fastest_wrap(html: &str, width: usize) -> Duration {
    let options = ConversionOptions {
        extract_metadata: false,
        tier_strategy: TierStrategy::Tier2,
        wrap: true,
        wrap_width: width,
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
            let (_, html, width) = &shapes(n)[index];
            fastest_wrap(html, *width).as_secs_f64().max(1e-6)
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
fn wrap_reflow_of_opener_runs_scales_linearly() {
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
            "wrap reflow scaled super-linearly on every attempt:\n{}",
            failures.join("\n")
        );
    }
}
