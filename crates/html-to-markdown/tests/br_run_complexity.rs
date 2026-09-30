//! Regression test for quadratic time in a run of hard breaks.
//!
//! Text after a run of `<br>` ends each line of the run with a backslash, so the lines stay
//! breaks and do not end the paragraph. A backslash inserted per line moved the rest of the
//! output each time. The library converts untrusted HTML, and nothing bounds the run.

use std::time::{Duration, Instant};

use html_to_markdown_rs::options::OutputFormat;
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

/// ~keep Same instrument as `list_item_complexity.rs`: linear code doubles its time when the
/// ~keep input doubles, quadratic code quadruples it, and 3.0 separates the two with room for
/// ~keep noise.
const MAX_DOUBLING_RATIO: f64 = 3.0;

/// ~keep A failure must reproduce on every independent attempt; one that does not is noise.
const MAX_MEASUREMENT_ATTEMPTS: usize = 3;

/// ~keep The fastest of this many runs is one sample.
const REPEATS_PER_SAMPLE: usize = 3;

/// ~keep 10k, 20k and 40k breaks: the quadratic insert took four times as long per doubling.
const BASE_SIZE: usize = 10_000;

fn fastest_convert(html: &str, tier: TierStrategy, format: OutputFormat) -> Duration {
    let options = ConversionOptions {
        extract_metadata: false,
        tier_strategy: tier,
        output_format: format,
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

fn measure(tier: TierStrategy, format: OutputFormat) -> Result<(), String> {
    let seconds: Vec<f64> = [BASE_SIZE, BASE_SIZE * 2, BASE_SIZE * 4]
        .into_iter()
        .map(|n| {
            let html = format!("<p>a{}b</p>", "<br>".repeat(n));
            fastest_convert(&html, tier, format).as_secs_f64().max(1e-6)
        })
        .collect();
    let first = seconds[1] / seconds[0];
    let second = seconds[2] / seconds[1];
    if first < MAX_DOUBLING_RATIO && second < MAX_DOUBLING_RATIO {
        return Ok(());
    }
    Err(format!(
        "{tier:?} {format:?}: doublings took {first:.1}x and {second:.1}x as long ({:.4}s, {:.4}s, {:.4}s); \
         expected under {MAX_DOUBLING_RATIO}x each",
        seconds[0], seconds[1], seconds[2]
    ))
}

#[test]
fn a_run_of_hard_breaks_before_text_scales_linearly() {
    for (tier, format) in [
        (TierStrategy::Tier2, OutputFormat::Markdown),
        (TierStrategy::Tier1, OutputFormat::Markdown),
        (TierStrategy::Tier2, OutputFormat::Djot),
    ] {
        let mut failures = Vec::with_capacity(MAX_MEASUREMENT_ATTEMPTS);
        for attempt in 1..=MAX_MEASUREMENT_ATTEMPTS {
            match measure(tier, format) {
                Ok(()) => break,
                Err(reason) => failures.push(format!("attempt {attempt}: {reason}")),
            }
        }
        assert!(
            failures.len() < MAX_MEASUREMENT_ATTEMPTS,
            "a run of hard breaks scaled super-linearly on every attempt:\n{}",
            failures.join("\n")
        );
    }
}
