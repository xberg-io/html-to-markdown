//! Count how many bench fixtures Tier-1 handles natively vs bails on.
//!
//! Per-fixture: `TIER1` if the scanner completes cleanly, otherwise the
//! BailReason category. Summary at the end shows total per category — the
//! signal we need for prioritising the next Tier-1 expansion phase.
//!
//! Run:
//!   cargo run --release --example tier1_routing -p html-to-markdown-bench

use std::collections::BTreeMap;
use std::path::PathBuf;

use html_to_markdown_bench::fixture::Loader;
use html_to_markdown_rs::testkit::tier1;

fn main() -> anyhow::Result<()> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    let loader = Loader::new(dir);
    let fixtures = loader.load(None)?;

    let stub = html_to_markdown_rs::testkit::prescan::PrescanReport::default();
    let opts = html_to_markdown_rs::ConversionOptions {
        extract_metadata: false,
        ..Default::default()
    };

    let mut ok = 0usize;
    let mut bail = 0usize;
    let mut by_reason: BTreeMap<String, usize> = BTreeMap::new();

    for fix in &fixtures {
        let html = std::fs::read_to_string(&fix.path)?;
        match tier1::run(&html, &stub, &opts) {
            Ok(_) => {
                ok += 1;
                println!("{:<58} TIER1", fix.rel_path);
            }
            Err(reason) => {
                bail += 1;
                let key = bail_category(&format!("{:?}", reason));
                *by_reason.entry(key).or_default() += 1;
                println!("{:<58} BAIL  {}", fix.rel_path, reason);
            }
        }
    }

    println!();
    println!("=== Tier-1 routing ===");
    println!("native: {}/{}", ok, ok + bail);
    println!("bail:   {}/{}", bail, ok + bail);
    println!();
    println!("=== Bail reasons (top first) ===");
    let mut entries: Vec<_> = by_reason.into_iter().collect();
    entries.sort_by_key(|e| std::cmp::Reverse(e.1));
    for (k, v) in entries {
        println!("  {:>3}  {}", v, k);
    }
    Ok(())
}

fn bail_category(dbg: &str) -> String {
    dbg.split([' ', '{', '(']).next().unwrap_or(dbg).trim().to_string()
}
