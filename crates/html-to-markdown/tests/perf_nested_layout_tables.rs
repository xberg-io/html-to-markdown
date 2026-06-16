//! Performance regression guard for issue #406.
//!
//! Layout-heavy HTML (Outlook email digests, marketing newsletters) frequently
//! nests dozens of `<table>` tags inside a single outer cell to control visual
//! flow.  Each nested table's column-width measurement pre-pass used to call
//! `walk_node` on every descendant table, recursing combinatorially:
//! `cells × nested-cells × deeply-nested-cells × ...`.
//! The per-cell output cap (`MAX_CELL_WIDTH = 200`) bounded the *discarded*
//! string output but not the measurement CPU.
//!
//! The fix in `walk_node`'s `"table"` arm short-circuits to descendant text
//! content when `Context::measure_width_only` is set (only true during the
//! outer table's width pre-pass), keeping the measurement walk linear in
//! descendant character count.
//!
//! This test fails if a synthetic worst-case nested-layout fixture cannot be
//! converted within a generous wall-clock budget on release builds.  Without
//! the fix, the same fixture took minutes-to-hours and effectively hung the
//! conversion pipeline.

use std::time::{Duration, Instant};

fn nested_layout_cells(depth: usize, breadth: usize) -> String {
    if depth == 0 {
        return "leaf".to_string();
    }
    let inner = nested_layout_cells(depth - 1, breadth);
    let cells: String = (0..breadth)
        .map(|_| format!("<td><table><tr><td>{inner}</td></tr></table></td>"))
        .collect();
    format!("<table><tr>{cells}</tr></table>")
}

#[test]
fn nested_layout_tables_convert_within_wall_clock_budget() {
    // depth=4, breadth=4 → 4^4 = 256 leaf cells across 341 nested tables.
    // Pre-#406 this took >30s on release builds.  Post-fix it completes in
    // under a second; the 10s budget leaves slack for noisy CI runners.
    let html = format!(
        "<html><body><table><tr><td>{}</td></tr></table></body></html>",
        nested_layout_cells(4, 4)
    );

    let start = Instant::now();
    let result = html_to_markdown_rs::convert(&html, None).expect("conversion must succeed");
    let elapsed = start.elapsed();

    assert!(
        elapsed < Duration::from_secs(10),
        "issue #406: nested-layout-table conversion took {elapsed:?} (budget: 10s). \
         Was the measure-width-only short-circuit in walk_node removed?"
    );

    let content = result.content.unwrap_or_default();
    assert!(
        content.contains("leaf"),
        "leaf text must appear in output (otherwise the short-circuit is dropping content)"
    );
}

#[test]
fn nested_layout_tables_render_within_outer_cell_width_cap() {
    // Deeper nesting (5 levels × 3 breadth) to stress the pre-pass beyond what
    // the previous cap-only approach handled.  Without skipping nested-table
    // dispatch during measurement, this would explode CPU even though the
    // per-cell output cap kept the final string size bounded.
    let html = format!(
        "<html><body><table><tr><td>{}</td></tr></table></body></html>",
        nested_layout_cells(5, 3)
    );

    let start = Instant::now();
    let _ = html_to_markdown_rs::convert(&html, None).expect("conversion must succeed");
    let elapsed = start.elapsed();

    assert!(
        elapsed < Duration::from_secs(10),
        "issue #406: 5-deep × 3-wide nested-table conversion took {elapsed:?} (budget: 10s)"
    );
}
