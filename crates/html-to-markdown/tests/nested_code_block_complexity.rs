#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! An indented code block deep in block quotes must not cost the square of the depth.
//!
//! Each block quote read every line of its content twice to learn which lines are indented code.
//! A code block that is `n` quotes deep was read `2n` times, each time through more quote markers.
//! The library converts untrusted HTML, so such growth is a denial-of-service vector.
//!
//! The bound is on the growth between two depths, not on a time: twice the depth may take at
//! most three times as long. One read for the whole document doubles the time, a read at each
//! level quadruples it.

use std::time::{Duration, Instant};

use html_to_markdown_rs::{CodeBlockStyle, ConversionOptions, TierStrategy, convert};

/// ~keep Same instrument as `list_item_complexity.rs`: 3.0 separates linear growth from quadratic
/// ~keep growth with room for noise.
const MAX_DOUBLING_RATIO: f64 = 3.0;

/// ~keep A failure must reproduce on every independent attempt; one that does not is noise.
const MAX_MEASUREMENT_ATTEMPTS: usize = 3;

/// ~keep The fastest of this many runs is one sample.
const REPEATS_PER_SAMPLE: usize = 3;

/// ~keep The converter stops at a depth limit past these, and writes no code block there.
const DEPTHS: [usize; 4] = [5, 10, 20, 40];

/// ~keep Enough lines that the reads of the code outweigh the fixed cost of a conversion.
const CODE_LINES: usize = 400;

/// A code block that is `depth` block quotes deep.
fn page(depth: usize) -> String {
    let mut html = "<blockquote>".repeat(depth);
    html.push_str("<pre>");
    html.push_str(&"value = 1\n".repeat(CODE_LINES));
    html.push_str("</pre>");
    html.push_str(&"</blockquote>".repeat(depth));
    html
}

fn fastest_convert(html: &str, tier_strategy: TierStrategy) -> Duration {
    let options = ConversionOptions {
        extract_metadata: false,
        code_block_style: CodeBlockStyle::Indented,
        tier_strategy,
        ..ConversionOptions::default()
    };
    (0..REPEATS_PER_SAMPLE)
        .map(|_| {
            let start = Instant::now();
            let output = convert(std::hint::black_box(html), Some(options.clone()))
                .expect("conversion must succeed")
                .content
                .unwrap_or_default();
            let elapsed = start.elapsed();
            // ~keep A conversion that stops at the depth limit is fast and proves nothing.
            assert_eq!(
                output.lines().filter(|line| line.contains("     value = 1")).count(),
                CODE_LINES,
                "every line of the code is indented code in the output"
            );
            elapsed
        })
        .min()
        .expect("at least one run")
}

/// The doubling ratios of one converter, or a description of why they look quadratic.
fn measure(tier_strategy: TierStrategy) -> Result<(), String> {
    let seconds: Vec<f64> = DEPTHS
        .into_iter()
        .map(|depth| fastest_convert(&page(depth), tier_strategy).as_secs_f64().max(1e-6))
        .collect();
    let ratios: Vec<f64> = seconds.windows(2).map(|pair| pair[1] / pair[0]).collect();
    if ratios.iter().all(|ratio| *ratio < MAX_DOUBLING_RATIO) {
        return Ok(());
    }
    Err(format!(
        "{tier_strategy:?}: depths {DEPTHS:?} took {seconds:.4?} seconds, ratios {ratios:.2?}; \
         expected under {MAX_DOUBLING_RATIO}x for each doubling"
    ))
}

#[test]
fn an_indented_code_block_in_nested_block_quotes_scales_linearly_in_both_tiers() {
    // ~keep Both converters are measured before the test fails, so one red run names each of them.
    let mut slow = Vec::new();
    for tier_strategy in [TierStrategy::Tier1, TierStrategy::Tier2] {
        let mut failures = Vec::with_capacity(MAX_MEASUREMENT_ATTEMPTS);
        for attempt in 1..=MAX_MEASUREMENT_ATTEMPTS {
            match measure(tier_strategy) {
                Ok(()) => break,
                Err(reason) => failures.push(format!("attempt {attempt}: {reason}")),
            }
        }
        if failures.len() == MAX_MEASUREMENT_ATTEMPTS {
            slow.extend(failures);
        }
    }
    assert!(
        slow.is_empty(),
        "the time grew faster than the depth on every attempt:\n{}",
        slow.join("\n")
    );
}
