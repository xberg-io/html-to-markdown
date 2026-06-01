//! Concurrency stress test: 64 threads calling `convert` simultaneously.
//!
//! Intent: catch any latent global state, lazy-init races, or `static mut`
//! bugs in core.  Each thread's output must equal the reference produced by a
//! single-threaded serial call.
//!
//! Fixtures are loaded from `tools/benchmark-harness/fixtures/real-world/wikipedia/`
//! relative to the workspace root (resolved from `CARGO_MANIFEST_DIR`).
//! The test is intentionally dependency-free: no `tokio`, no `rayon`.

use std::path::PathBuf;
use std::sync::Arc;

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

const THREAD_COUNT: usize = 64;

/// Fixture file names relative to `tools/benchmark-harness/fixtures/real-world/wikipedia/`.
const FIXTURE_NAMES: &[&str] = &[
    "small_html.html",
    "lists_timeline.html",
    "medium_python.html",
    "tables_countries.html",
];

fn fixtures_dir() -> PathBuf {
    // CARGO_MANIFEST_DIR is the crates/html-to-markdown directory.
    // The workspace root is three levels up.
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // crates/html-to-markdown → workspace root
    let workspace_root = manifest
        .parent() // crates
        .and_then(|p| p.parent()) // workspace root
        .expect("could not resolve workspace root from CARGO_MANIFEST_DIR")
        .to_path_buf();
    workspace_root.join("tools/benchmark-harness/fixtures/real-world/wikipedia")
}

fn default_opts() -> ConversionOptions {
    ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        extract_metadata: false,
        ..ConversionOptions::default()
    }
}

/// Load all fixtures and compute serial reference outputs once, before threading.
fn load_fixtures() -> Vec<(String, String)> {
    let dir = fixtures_dir();
    let mut result = Vec::new();
    for name in FIXTURE_NAMES {
        let path = dir.join(name);
        let html =
            std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read fixture {}: {e}", path.display()));
        let reference = convert(&html, Some(default_opts()))
            .unwrap_or_else(|e| panic!("serial convert failed for {name}: {e}"))
            .content
            .unwrap_or_default();
        result.push((html, reference));
    }
    result
}

#[test]
fn all_threads_produce_output_equal_to_serial_reference() {
    let fixtures = Arc::new(load_fixtures());

    let handles: Vec<_> = (0..THREAD_COUNT)
        .map(|i| {
            let fixtures = Arc::clone(&fixtures);
            std::thread::spawn(move || {
                // Cycle fixture index across threads.
                let (html, reference) = &fixtures[i % fixtures.len()];
                let output = convert(html, Some(default_opts()))
                    .unwrap_or_else(|e| panic!("thread {i} convert failed: {e}"))
                    .content
                    .unwrap_or_default();
                assert_eq!(
                    output,
                    *reference,
                    "thread {i} output differed from serial reference (fixture index {fi})",
                    fi = i % fixtures.len()
                );
            })
        })
        .collect();

    // Join all handles; panic if any thread panicked.
    for handle in handles {
        handle.join().unwrap_or_else(|e| {
            std::panic::resume_unwind(e);
        });
    }
}

#[test]
fn thread_count_and_fixture_count_are_as_expected() {
    // Sanity-check the constants used by the main concurrency test.
    assert_eq!(THREAD_COUNT, 64, "thread count must be 64");
    assert_eq!(FIXTURE_NAMES.len(), 4, "fixture count must be 4");
}
