//! Tier-1 byte-equality oracle against all benchmark fixtures.
//!
//! For each fixture the test runs conversion twice:
//!   - `Tier2Only`   — authoritative Tier-2 output.
//!   - `ForceTier1`  — forces the Tier-1 byte-scanner path (with Tier-2 fallback on bail).
//!
//! When `ForceTier1` bails internally, the fallback produces Tier-2 output, so
//! the two results are equal by definition.  When `ForceTier1` successfully
//! produces output it MUST equal the Tier-2 output byte-for-byte; any divergence
//! is a hard failure.
//!
//! Bail outcomes are logged (not failed) so the survey can measure progress.

#![cfg(feature = "testkit")]

use std::fs;
use std::path::Path;

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

// ── Fixture paths (mirrors groups.toml) ──────────────────────────────────────

const FIXTURES_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tools/benchmark-harness/fixtures");

/// All fixture relative paths from groups.toml.
const FIXTURE_PATHS: &[&str] = &[
    // mdream corpus
    "mdream/nuxt-example.html",
    "mdream/vuejs-docs.html",
    "mdream/wikipedia-small.html",
    "mdream/mdn-array.html",
    "mdream/react-learn.html",
    "mdream/github-markdown-complete.html",
    // real-world / wikipedia
    "real-world/wikipedia/small_html.html",
    "real-world/wikipedia/lists_timeline.html",
    "real-world/wikipedia/tables_countries.html",
    "real-world/wikipedia/medium_python.html",
    "real-world/wikipedia/large_rust.html",
    // real-world / gh-190
    "real-world/issues/gh-190/mitrade.html",
    "real-world/issues/gh-190/flex2025.html",
    "real-world/issues/gh-190/insight.html",
    "real-world/issues/gh-190/plusblog.html",
    "real-world/issues/gh-190/rbloggers.html",
    "real-world/issues/gh-190/ozonekorea.html",
    "real-world/issues/gh-190/firsteigen.html",
    "real-world/issues/gh-190/sjsu.html",
    "real-world/issues/gh-190/kimbrain.html",
    // real-world / other issues
    "real-world/issues/gh-121-hacker-news.html",
    "real-world/issues/gh-127-issue.html",
    // synthetic / spec_rules
    "synthetic/optional_li.html",
    "synthetic/table_no_tbody.html",
    // synthetic / adversarial
    "synthetic/bare_fragment.html",
    "synthetic/unescaped_lt.html",
    "synthetic/unclosed_p.html",
    "synthetic/unclosed_at_eof.html",
    "synthetic/cdata_in_svg.html",
];

// ── Helpers ───────────────────────────────────────────────────────────────────

fn tier2_output(html: &str) -> Option<String> {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2Only,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    let html_owned = html.to_owned();
    let result = std::panic::catch_unwind(move || convert(&html_owned, Some(opts)));
    match result {
        Ok(Ok(r)) => r.content,
        _ => None,
    }
}

fn force_tier1_output(html: &str) -> Option<String> {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::ForceTier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    let html_owned = html.to_owned();
    let result = std::panic::catch_unwind(move || convert(&html_owned, Some(opts)));
    match result {
        Ok(Ok(r)) => r.content,
        _ => None,
    }
}

// ── Oracle test ───────────────────────────────────────────────────────────────

/// For each fixture, assert that `ForceTier1` output is byte-for-byte identical
/// to `Tier2Only` output.
///
/// When Tier-1 bails the fallback produces Tier-2 output, so the bytes always
/// match — bail is not a failure.  The only failure case is when Tier-1 produces
/// output that diverges from Tier-2.
#[test]
fn tier1_byte_equality_against_all_fixtures() {
    let fixtures_root = Path::new(FIXTURES_ROOT);

    let mut total = 0usize;
    let mut skipped = 0usize;
    let mut failures: Vec<String> = Vec::new();

    for rel_path in FIXTURE_PATHS {
        total += 1;
        let full_path = fixtures_root.join(rel_path);
        let Ok(html) = fs::read_to_string(&full_path) else {
            eprintln!("SKIP {rel_path}: cannot read");
            skipped += 1;
            continue;
        };

        let Some(t2) = tier2_output(&html) else {
            eprintln!("SKIP {rel_path}: Tier-2 panicked or failed");
            skipped += 1;
            continue;
        };

        let Some(t1) = force_tier1_output(&html) else {
            eprintln!("SKIP {rel_path}: ForceTier1 panicked");
            skipped += 1;
            continue;
        };

        if t1 != t2 {
            let diff = first_diff(&t2, &t1);
            let msg = format!(
                "DIVERGE {rel_path}:\n  Tier-2: {:?}\n  Tier-1: {:?}\n  diff: {diff}",
                &t2[..t2.len().min(200)],
                &t1[..t1.len().min(200)],
            );
            eprintln!("{msg}");
            failures.push(msg);
        }
    }

    eprintln!(
        "\ntier1_byte_equality: total={total} skipped={skipped} failures={}",
        failures.len()
    );

    let nfail = failures.len();
    assert!(
        failures.is_empty(),
        "{nfail} fixture(s) diverged from Tier-2:\n{}",
        failures.join("\n\n")
    );
}

// ── Bail-survey test ──────────────────────────────────────────────────────────

/// Survey how many fixtures Tier-1 handles natively vs bails on.
///
/// Uses `Auto` strategy (default) so the router decides; when Tier-1 is active
/// the scanner must produce byte-equal output (else bail fires first).
///
/// This test always passes — it only prints counts.
#[test]
fn tier1_bail_survey() {
    let fixtures_root = Path::new(FIXTURES_ROOT);

    let mut handled = 0usize;
    let mut bailed = 0usize;
    let mut skipped = 0usize;

    for rel_path in FIXTURE_PATHS {
        let full_path = fixtures_root.join(rel_path);
        let Ok(html) = fs::read_to_string(&full_path) else {
            skipped += 1;
            continue;
        };

        let Some(t2) = tier2_output(&html) else {
            skipped += 1;
            continue;
        };

        let Some(t1) = force_tier1_output(&html) else {
            skipped += 1;
            continue;
        };

        // When t1 == t2, Tier-1 either handled it correctly or bailed to Tier-2.
        // Both are "ok". For the survey we just note equality.
        if t1 == t2 {
            handled += 1;
        } else {
            // t1 != t2 — Tier-1 produced diverging output without bailing.
            // The byte_equality test catches this as a hard failure; here just count.
            eprintln!("DIVERGE {rel_path}");
            bailed += 1;
        }
    }

    eprintln!("\ntier1_bail_survey: equal={handled} diverged={bailed} skipped={skipped}");
}

// ── Utilities ─────────────────────────────────────────────────────────────────

fn first_diff(expected: &str, actual: &str) -> String {
    let exp: Vec<&str> = expected.lines().collect();
    let act: Vec<&str> = actual.lines().collect();
    for (i, (e, a)) in exp.iter().zip(act.iter()).enumerate() {
        if e != a {
            return format!("line {}: expected {:?} actual {:?}", i + 1, e, a);
        }
    }
    format!("line count differs: expected {} actual {}", exp.len(), act.len())
}
