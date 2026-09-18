//! Tier-1/Tier-2 byte-equality regression tests for the *layout-table* heuristic.
//!
//! Tier-2 decides a table is a layout table — rendered as a bullet list rather
//! than a GFM table — in `converter/block/table/builder.rs`:
//!
//! ```text
//! looks_like_layout = table_scan.nested_table_count > 1
//!                  || (table_scan.has_span && has_border_zero)
//! ```
//!
//! Tier-1's `close_table` (`converter/tier1/scanner.rs`) only ever evaluated the
//! `has_span`/border-zero disjunct as its own check. Nested tables and
//! colspan/rowspan were assumed unreachable — a stale comment claimed both had
//! "already bailed" — but Phase HH renders a nested table inline into the parent
//! cell and `open_table_cell` expands colspan, so both reach Tier-1 intact. Such a
//! table was emitted as a GFM table by Tier-1 while Tier-2 emitted a bullet list,
//! breaking the byte-equality contract on `TierStrategy::Auto`.
//!
//! The fix is a bail (not a Tier-1 layout emitter), so these tests come in pairs:
//! the contract assertion (Auto output == Tier-2 output) and the mechanism
//! assertion (`tier1::run` returns `BailReason::Classifier`). The near-miss cases
//! at the bottom guard the opposite failure: a bail predicate looser than Tier-2's
//! would silently disable the fast path for ordinary tables.
//!
//! Tier-1 also bails on `inconsistent_cols` (ragged row lengths), but that is no
//! longer part of Tier-2's formula above (issue #500): Tier-2 now pads a short row
//! to the table's column count instead (issue #13) rather than treating it as
//! layout. Tier-1 has no padding logic of its own, so it keeps bailing on ragged
//! rows anyway — stricter than Tier-2 needs, but still parity-safe, since a bail
//! only ever routes more input to the (authoritative) Tier-2 fallback. The pair
//! below for that case therefore does not use `assert_auto_matches_tier2_layout_output`
//! (Tier-2 no longer takes the layout path here): it asserts the bail fires and
//! that `Auto` still matches Tier-2's now-tabular output byte for byte.

#![cfg(feature = "testkit")]

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, HighlightStyle, TierStrategy, convert, tier1};

/// Baseline options that clear every classifier gate so `TierStrategy::Auto`
/// genuinely attempts the Tier-1 scanner rather than routing straight to Tier-2
/// for an unrelated reason (`highlight_style`'s non-`None` default is itself a
/// router gate — see `tier1::router::classify`). Identical to the helper in
/// `tier1_scanner_parity_test.rs`, and load-bearing for the same reason: without
/// it an Auto-vs-Tier-2 assertion passes trivially because both sides ran Tier-2.
fn base_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        ..ConversionOptions::default()
    }
}

fn convert_with(html: &str, tier_strategy: TierStrategy) -> String {
    let options = ConversionOptions {
        tier_strategy,
        ..base_options()
    };
    convert(html, Some(options))
        .expect("conversion must succeed")
        .content
        .unwrap_or_default()
}

/// Assert the byte-equality contract for `html`, and that Tier-2 really took the
/// layout (bullet-list) path — otherwise the equality could hold vacuously if the
/// Tier-2 heuristic itself changed.
fn assert_auto_matches_tier2_layout_output(html: &str, case: &str) {
    let tier2 = convert_with(html, TierStrategy::Tier2);
    let auto = convert_with(html, TierStrategy::Auto);
    assert!(
        tier2.starts_with("- "),
        "{case}: precondition — Tier-2 must render this as a layout bullet list, got {tier2:?}"
    );
    assert_eq!(auto, tier2, "{case}: Auto routing must match Tier-2 byte for byte");
}

fn assert_tier1_bails_as_classifier(html: &str, case: &str) {
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &base_options());
    assert!(
        matches!(result, Err(tier1::BailReason::Classifier)),
        "{case}: expected Err(BailReason::Classifier), got {result:?}"
    );
}

/// Byte-equality contract for a table where Tier-1 bails but Tier-2 does NOT take the
/// layout path — the ragged-row case (issue #500), where the bail is stricter than Tier-2
/// needs. Unlike `assert_auto_matches_tier2_layout_output`, this does not assert Tier-2
/// rendered a bullet list; it asserts the opposite (a pipe table), so a future change that
/// makes Tier-2 treat ragged rows as layout again would fail this precondition rather than
/// pass vacuously.
fn assert_auto_matches_tier2_tabular_output(html: &str, case: &str) {
    let tier2 = convert_with(html, TierStrategy::Tier2);
    let auto = convert_with(html, TierStrategy::Auto);
    assert!(
        tier2.contains("| ---"),
        "{case}: precondition — Tier-2 must render this as a pipe table, got {tier2:?}"
    );
    assert_eq!(auto, tier2, "{case}: Auto routing must match Tier-2 byte for byte");
}

fn assert_tier1_takes_the_fast_path(html: &str, case: &str) {
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &base_options());
    assert!(
        result.is_ok(),
        "{case}: Tier-1 must still handle this table, got bail {:?}",
        result.err()
    );
}

// ~keep ── has_span && border="0" ────────────────────────────────────────────────────

/// `border="0"` + a spanning cell. Column counts stay consistent after colspan
/// expansion (2/2/2), so the pre-existing `inconsistent_cols` check never fired.
const BORDERLESS_SPANNING_TABLE: &str = concat!(
    "<table border=\"0\">",
    "<tr><td colspan=\"2\">a</td></tr>",
    "<tr><td>b</td><td>c</td></tr>",
    "<tr><td>d</td><td>e</td></tr>",
    "</table>",
);

#[test]
fn should_match_tier2_output_when_auto_routing_a_borderless_table_with_a_spanning_cell() {
    assert_auto_matches_tier2_layout_output(BORDERLESS_SPANNING_TABLE, "borderless spanning table");
}

#[test]
fn should_bail_from_tier1_when_a_borderless_table_has_a_spanning_cell() {
    assert_tier1_bails_as_classifier(BORDERLESS_SPANNING_TABLE, "borderless spanning table");
}

#[test]
fn should_bail_from_tier1_when_a_borderless_table_has_a_colspan_of_exactly_one() {
    // ~keep Tier-2's `has_span` is attribute *presence*, not a value greater than one
    // ~keep (`block/table/scanner.rs`: `attrs.get("colspan").is_some()`), so this table
    // ~keep takes Tier-2's layout path even though the colspan changes no geometry.
    // ~keep A `> 1` predicate in Tier-1 would move the divergence here instead of
    // ~keep fixing it.
    let html = concat!(
        "<table border=\"0\">",
        "<tr><td colspan=\"1\">a</td><td>b</td></tr>",
        "<tr><td>c</td><td>d</td></tr>",
        "</table>",
    );
    assert_auto_matches_tier2_layout_output(html, "borderless colspan=1");
    assert_tier1_bails_as_classifier(html, "borderless colspan=1");
}

#[test]
fn should_bail_from_tier1_when_a_borderless_table_has_a_rowspan() {
    // ~keep `rowspan` counts towards `has_span` exactly like `colspan` does.
    let html = concat!(
        "<table border=\"0\">",
        "<tr><td rowspan=\"2\">a</td><td>b</td></tr>",
        "<tr><td>c</td><td>d</td></tr>",
        "</table>",
    );
    assert_auto_matches_tier2_layout_output(html, "borderless rowspan");
    assert_tier1_bails_as_classifier(html, "borderless rowspan");
}

// ~keep ── nested_table_count > 1 ───────────────────────────────────────────────────

/// Two sibling nested tables. Tier-1 previously flattened both into their parent
/// cells and emitted a malformed single-column GFM table.
const TWO_NESTED_TABLES: &str = concat!(
    "<table>",
    "<tr><td><table><tr><td>x</td></tr></table></td></tr>",
    "<tr><td><table><tr><td>y</td></tr></table></td></tr>",
    "</table>",
);

#[test]
fn should_match_tier2_output_when_auto_routing_a_table_containing_two_nested_tables() {
    assert_auto_matches_tier2_layout_output(TWO_NESTED_TABLES, "two nested tables");
}

#[test]
fn should_bail_from_tier1_when_a_table_contains_two_nested_tables() {
    assert_tier1_bails_as_classifier(TWO_NESTED_TABLES, "two nested tables");
}

// ~keep ── inconsistent_cols (Tier-1-only, issue #500) ────────────────────────────

/// Ragged row lengths, headerless, no other layout signal. Tier-2 renders this as a padded
/// pipe table (issue #13); Tier-1 still bails on it rather than learning to pad.
const RAGGED_TABLE: &str = "<table><tr><td>A</td><td>B</td></tr><tr><td>C</td></tr></table>";

#[test]
fn should_match_tier2_tabular_output_when_auto_routing_a_ragged_table() {
    assert_auto_matches_tier2_tabular_output(RAGGED_TABLE, "ragged table");
}

#[test]
fn should_bail_from_tier1_when_a_table_has_ragged_rows() {
    assert_tier1_bails_as_classifier(RAGGED_TABLE, "ragged table");
}

// ~keep ── Control ─────────────────────────────────────────────────────────────────

#[test]
fn should_match_tier2_output_when_auto_routing_a_plain_two_by_two_table() {
    // ~keep Proves the harness itself is sound: an ordinary table is byte-identical
    // ~keep across tiers AND stays on the fast path, so the divergences asserted above
    // ~keep are specific to the layout heuristic rather than to tables in general.
    let html = "<table><tr><td>a</td><td>b</td></tr><tr><td>c</td><td>d</td></tr></table>";
    let tier2 = convert_with(html, TierStrategy::Tier2);
    let auto = convert_with(html, TierStrategy::Auto);
    assert_eq!(auto, tier2, "plain table: Auto routing must match Tier-2 byte for byte");
    assert_tier1_takes_the_fast_path(html, "plain table");
}

// ~keep ── Near misses: the fast path must survive ─────────────────────────────────

#[test]
fn should_keep_the_tier1_fast_path_when_a_spanning_table_is_not_borderless() {
    // ~keep `has_span` alone is not a layout signal in Tier-2 — it is conjoined with
    // ~keep `border="0"`. Bailing on every colspan would disable Tier-1 for the very
    // ~keep common infobox/header-span shape.
    let html = concat!(
        "<table border=\"1\">",
        "<tr><td colspan=\"2\">a</td></tr>",
        "<tr><td>b</td><td>c</td></tr>",
        "</table>",
    );
    assert_tier1_takes_the_fast_path(html, "bordered spanning table");
}

#[test]
fn should_keep_the_tier1_fast_path_when_a_spanning_table_has_no_border_attribute() {
    let html = concat!(
        "<table>",
        "<tr><td colspan=\"2\">a</td></tr>",
        "<tr><td>b</td><td>c</td></tr>",
        "</table>",
    );
    assert_tier1_takes_the_fast_path(html, "spanning table without border attribute");
}

#[test]
fn should_keep_the_tier1_fast_path_when_a_borderless_table_has_no_spanning_cell() {
    let html = concat!(
        "<table border=\"0\">",
        "<tr><td>a</td><td>b</td></tr>",
        "<tr><td>c</td><td>d</td></tr>",
        "</table>",
    );
    assert_tier1_takes_the_fast_path(html, "borderless table without spans");
}

#[test]
fn should_keep_the_tier1_fast_path_when_a_table_contains_exactly_one_nested_table() {
    // A single nested table in a multi-cell row keeps the GFM fast path.
    // Single-cell wrappers instead fall back to Tier-2's unwrapping (#478).
    let html = "<table><tr><td><table><tr><td>x</td></tr></table></td><td>Other</td></tr></table>";
    assert_tier1_takes_the_fast_path(html, "one nested table");
}
