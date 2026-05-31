//! Timing harness for a single fixture.
//!
//! Methodology (mirrors `/tmp/bench-h2m-vs-mdream/src/bench.rs`):
//! - 20 warmup iterations (discarded)
//! - Calibrate: find N such that N iterations take ~50 ms
//! - Best-of-3 runs of N iterations; report best observed µs/call
//!
//! This avoids noisy single-sample measurements while keeping total run time
//! bounded for large fixtures.

use std::hint::black_box;
use std::time::{Duration, Instant};

use html_to_markdown_rs::options::ConversionOptions;

/// Target duration for a single calibration run.
const TARGET_CALIBRATION: Duration = Duration::from_millis(50);
/// Warmup iteration count.
const WARMUP_ITERS: u32 = 20;
/// Number of timed runs in best-of-N.
const BEST_OF: u32 = 3;
/// Minimum iters per run to avoid degenerate timing.
const MIN_ITERS: u32 = 1;

/// Run `convert(html, opts)` and return the output length.
fn run_once(html: &str, opts: Option<ConversionOptions>) -> usize {
    let result = html_to_markdown_rs::convert(html, opts).expect("h2m convert");
    result.content.as_deref().unwrap_or("").len()
}

/// Benchmark a single HTML string with the given options.
///
/// Returns `(ms_best, output_bytes)` where `ms_best` is the best single-call
/// wall-clock time in milliseconds.
pub fn run_one(html: &str, opts: Option<ConversionOptions>) -> (f64, usize) {
    // Warmup
    let mut output_bytes = 0usize;
    for _ in 0..WARMUP_ITERS {
        output_bytes = run_once(html, opts.clone());
        black_box(output_bytes);
    }

    // Calibrate: find N such that N iterations take ~50 ms
    let iters = {
        let start = Instant::now();
        let mut n: u32 = 1;
        loop {
            let t0 = Instant::now();
            for _ in 0..n {
                black_box(run_once(html, opts.clone()));
            }
            let elapsed = t0.elapsed();
            if elapsed >= TARGET_CALIBRATION {
                break;
            }
            if elapsed.as_nanos() > 0 {
                let factor = (TARGET_CALIBRATION.as_nanos() as f64 / elapsed.as_nanos() as f64).ceil() as u32;
                n = n.saturating_mul(factor).max(MIN_ITERS);
            } else {
                n = n.saturating_mul(2);
            }
            // Safety cap: don't calibrate for more than 2 s total
            if start.elapsed() > Duration::from_secs(2) {
                break;
            }
        }
        n.max(MIN_ITERS)
    };

    // Best-of-3 timed runs
    let mut best_us = f64::MAX;
    for _ in 0..BEST_OF {
        let t0 = Instant::now();
        for _ in 0..iters {
            black_box(run_once(html, opts.clone()));
        }
        let us = t0.elapsed().as_micros() as f64 / f64::from(iters);
        if us < best_us {
            best_us = us;
        }
    }

    (best_us / 1_000.0, output_bytes)
}

/// Benchmark with mdream (feature-gated).
#[cfg(feature = "compare-mdream")]
pub fn run_one_mdream(html: &str) -> f64 {
    // Warmup
    for _ in 0..WARMUP_ITERS {
        let opts = mdream::types::HTMLToMarkdownOptions::default();
        black_box(mdream::html_to_markdown(html, opts));
    }

    // Calibrate
    let iters = {
        let start = Instant::now();
        let mut n: u32 = 1;
        loop {
            let t0 = Instant::now();
            for _ in 0..n {
                let opts = mdream::types::HTMLToMarkdownOptions::default();
                black_box(mdream::html_to_markdown(html, opts));
            }
            let elapsed = t0.elapsed();
            if elapsed >= TARGET_CALIBRATION {
                break;
            }
            if elapsed.as_nanos() > 0 {
                let factor = (TARGET_CALIBRATION.as_nanos() as f64 / elapsed.as_nanos() as f64).ceil() as u32;
                n = n.saturating_mul(factor).max(MIN_ITERS);
            } else {
                n = n.saturating_mul(2);
            }
            if start.elapsed() > Duration::from_secs(2) {
                break;
            }
        }
        n.max(MIN_ITERS)
    };

    let mut best_us = f64::MAX;
    for _ in 0..BEST_OF {
        let t0 = Instant::now();
        for _ in 0..iters {
            let opts = mdream::types::HTMLToMarkdownOptions::default();
            black_box(mdream::html_to_markdown(html, opts));
        }
        let us = t0.elapsed().as_micros() as f64 / f64::from(iters);
        if us < best_us {
            best_us = us;
        }
    }
    best_us / 1_000.0
}
