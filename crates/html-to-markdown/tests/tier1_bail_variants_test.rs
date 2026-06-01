//! One positive trigger per `BailReason` variant — covering the table-specific
//! variants not exercised by `tier1_bail_test.rs`.
//!
//! Coverage:
//!   - `NotImplemented`           — unreachable: the M2 stub has been fully replaced by M9;
//!     no code path in the scanner emits this variant any more.
//!   - `Classifier`               — covered in `tier1_bail_test.rs`
//!   - `DepthMismatch`            — covered in `tier1_bail_test.rs`
//!   - `EofWithOpenBlock`         — covered in `tier1_bail_test.rs`
//!   - `LiteralLt`                — covered in `tier1_bail_test.rs`
//!   - `Cdata`                    — covered in `tier1_bail_test.rs`
//!   - `UnknownCustomElement`     — covered in `tier1_bail_test.rs`
//!   - `TableRowspanColspan`      — NEW (this file)
//!   - `TableBlockChildInCell`    — NEW (this file, two triggers: block child + <br>)
//!   - `TableNestedTable`         — NEW (this file)
//!   - `TableCaption`             — NEW (this file)
//!   - `TableSectionOrder`        — NEW (this file, two orderings)

#![cfg(feature = "testkit")]

use html_to_markdown_rs::prescan;
use html_to_markdown_rs::tier1::{self, BailReason};
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

// ── Helpers ───────────────────────────────────────────────────────────────────

fn tier1_run(html: &str) -> Result<String, BailReason> {
    let (cleaned, report) = prescan::run(html);
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::ForceTier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    tier1::run(cleaned.as_ref(), &report, &opts)
}

fn tier2(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2Only,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

fn force_tier1(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::ForceTier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

// ── TableRowspanColspan ───────────────────────────────────────────────────────

#[test]
fn should_bail_on_table_rowspan_greater_than_one() {
    let html = "<table><tr><td rowspan=\"2\">a</td><td>b</td></tr></table>";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::TableRowspanColspan),
        "expected TableRowspanColspan, got {err:?}"
    );
    // Tier-1 fallback via convert still produces a sensible result.
    let result = force_tier1(html);
    assert!(!result.is_empty(), "expected non-empty fallback output");
    assert_eq!(result, tier2(html), "fallback output must match Tier-2");
}

#[test]
fn should_bail_on_table_colspan_greater_than_one() {
    let html = "<table><tr><th colspan=\"3\">Header</th></tr></table>";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::TableRowspanColspan),
        "expected TableRowspanColspan, got {err:?}"
    );
    assert_eq!(force_tier1(html), tier2(html), "fallback output must match Tier-2");
}

#[test]
fn should_not_bail_when_rowspan_and_colspan_are_one() {
    // Explicit rowspan="1" and colspan="1" must NOT trigger the bail.
    let html = r#"<table>
<thead><tr><th rowspan="1" colspan="1">A</th><th>B</th></tr></thead>
<tbody><tr><td>1</td><td>2</td></tr></tbody>
</table>"#;
    let result = tier1_run(html);
    assert!(result.is_ok(), "rowspan=1/colspan=1 must not bail; got {result:?}");
}

// ── TableBlockChildInCell ─────────────────────────────────────────────────────

#[test]
fn should_bail_when_paragraph_is_child_of_table_cell() {
    let html = "<table><tr><td><p>text</p></td></tr></table>";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::TableBlockChildInCell),
        "expected TableBlockChildInCell, got {err:?}"
    );
    assert_eq!(force_tier1(html), tier2(html), "fallback output must match Tier-2");
}

#[test]
fn should_bail_when_div_is_child_of_table_cell() {
    let html = "<table><tr><td><div>block</div></td></tr></table>";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::TableBlockChildInCell),
        "expected TableBlockChildInCell, got {err:?}"
    );
    assert_eq!(force_tier1(html), tier2(html), "fallback output must match Tier-2");
}

#[test]
fn should_bail_when_br_is_child_of_table_cell() {
    // <br> inside a table cell is a void element that the scanner cannot
    // handle correctly for GFM output — it returns TableBlockChildInCell.
    let html = "<table><tr><td>line1<br>line2</td></tr></table>";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::TableBlockChildInCell),
        "expected TableBlockChildInCell for <br> in cell, got {err:?}"
    );
    assert_eq!(force_tier1(html), tier2(html), "fallback output must match Tier-2");
}

// ── TableNestedTable ──────────────────────────────────────────────────────────

#[test]
fn should_bail_on_nested_table_inside_cell() {
    let html = "<table><tr><td><table><tr><td>inner</td></tr></table></td></tr></table>";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::TableNestedTable),
        "expected TableNestedTable, got {err:?}"
    );
    assert_eq!(force_tier1(html), tier2(html), "fallback output must match Tier-2");
}

// ── TableCaption ──────────────────────────────────────────────────────────────

#[test]
fn should_bail_on_table_caption_element() {
    let html = "<table><caption>My table</caption><tr><td>a</td></tr></table>";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::TableCaption),
        "expected TableCaption, got {err:?}"
    );
    assert_eq!(force_tier1(html), tier2(html), "fallback output must match Tier-2");
}

// ── TableSectionOrder ─────────────────────────────────────────────────────────

#[test]
fn should_bail_when_thead_appears_after_tbody_close() {
    // <thead> opening after a <tbody> has already been closed is unsupported.
    let html = "<table><tbody><tr><td>a</td></tr></tbody><thead><tr><th>h</th></tr></thead></table>";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::TableSectionOrder),
        "expected TableSectionOrder, got {err:?}"
    );
    assert_eq!(force_tier1(html), tier2(html), "fallback output must match Tier-2");
}

#[test]
fn should_bail_when_tbody_appears_after_tfoot() {
    // <tbody> opening after a <tfoot> open is unsupported.
    let html = "<table><tfoot><tr><td>f</td></tr></tfoot><tbody><tr><td>b</td></tr></tbody></table>";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::TableSectionOrder),
        "expected TableSectionOrder for tbody-after-tfoot, got {err:?}"
    );
    assert_eq!(force_tier1(html), tier2(html), "fallback output must match Tier-2");
}

// ── NotImplemented ────────────────────────────────────────────────────────────

// `BailReason::NotImplemented` was the M2 stub sentinel and is no longer emitted
// by any scanner code path in M9.  The variant still exists in the enum for
// Display completeness (tested in `tier1_bail_test.rs::bail_reason_display_is_non_empty`),
// but there is no HTML input that triggers it.  No positive trigger test exists
// for this variant; the gap is intentional and documented here.

// ── BailReason::TableRowspanColspan display ───────────────────────────────────

#[test]
fn table_bail_reason_display_strings_are_non_empty() {
    let reasons = [
        BailReason::TableRowspanColspan,
        BailReason::TableBlockChildInCell,
        BailReason::TableNestedTable,
        BailReason::TableCaption,
        BailReason::TableSectionOrder,
    ];
    for reason in &reasons {
        let s = reason.to_string();
        assert!(!s.is_empty(), "Display for {reason:?} produced empty string");
    }
}
