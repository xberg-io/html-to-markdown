//! Timing harness for a single fixture.
//!
//! Methodology:
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
///
/// Returns `None` when the conversion panics (known core bug on some fixtures).
fn run_once(html: &str, opts: Option<ConversionOptions>) -> Option<usize> {
    let html_owned = html.to_owned();
    match std::panic::catch_unwind(move || html_to_markdown_rs::convert(&html_owned, opts)) {
        Ok(Ok(result)) => Some(result.content.as_deref().map_or(0, str::len)),
        Ok(Err(_)) => Some(0),
        Err(_) => None,
    }
}

/// Benchmark a single HTML string with the given options.
///
/// Returns `(ms_best, output_bytes)` where `ms_best` is the best single-call
/// wall-clock time in milliseconds.  Returns `(0.0, 0)` when the fixture
/// panics during conversion (known core bug on some fixtures).
pub fn run_one(html: &str, opts: Option<ConversionOptions>) -> (f64, usize) {
    // Warmup — check for panics upfront
    let mut output_bytes = 0usize;
    let mut panicked = false;
    for _ in 0..WARMUP_ITERS {
        match run_once(html, opts.clone()) {
            Some(n) => {
                output_bytes = n;
                black_box(output_bytes);
            }
            None => {
                panicked = true;
                break;
            }
        }
    }
    if panicked {
        return (0.0, 0);
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
