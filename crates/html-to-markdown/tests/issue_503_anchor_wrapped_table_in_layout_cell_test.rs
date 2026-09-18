// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #503: an anchor wrapping a table, inside a cell of a table the
//! layout heuristic renders as a bullet list, crushed the nested table into its own label, where
//! `escape_link_label` turned the inner links into `\[One\](/one)` text.
//!
//! Issue #490 already renders a wrapped table as a separate block after the link, but only
//! outside inline contexts: a layout cell converts as inline, so the deferral refused and the
//! `saw_block` branch walked the table into the label. A layout cell is a list item's text, not
//! a link label -- it can hold the deferred table just as it already holds a bare nested one.
//!
//! The deferred table lands after the link on its own lines, the same shape a bare nested table
//! already takes inside a layout cell; what matters is that every destination survives.
//!
//! Tier 1 bails (`Classifier`) on a table opened inside a link, so the fix is Tier-2 only.

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, HighlightStyle, TierStrategy, convert, tier1};

/// The reporter's input: two rows and three links make the outer table link-heavy, which is
/// what routes it to the layout (bullet-list) renderer.
const ISSUE_503: &str = "<table><tr><td><a href=\"/outer\"><table><tr><td>\
<a href=\"/one\">One</a><a href=\"/two\">Two</a>\
</td></tr></table></a></td></tr><tr><td>After</td></tr></table>";

fn content_with(html: &str, options: ConversionOptions) -> String {
    convert(html, Some(options))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

fn base_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        ..ConversionOptions::default()
    }
}

fn assert_tier1_bails_as_classifier(html: &str) {
    let report = PrescanReport::default();
    let result = tier1::run(html, &report, &base_options());
    assert!(
        matches!(result, Err(tier1::BailReason::Classifier)),
        "expected Err(BailReason::Classifier) for {html:?}, got {result:?}"
    );
}

#[test]
fn should_keep_both_inner_links_clickable_with_br_in_tables_and_compact_tables() {
    let options = ConversionOptions {
        br_in_tables: true,
        compact_tables: true,
        ..ConversionOptions::default()
    };
    let out = content_with(ISSUE_503, options);
    assert_eq!(
        out, "- [/outer](/outer)\n\n| [One](/one)[Two](/two) |\n| --- |\n- After\n",
        "actual: {out:?}"
    );
    assert!(!out.contains("\\["), "inner links must not be escaped: {out:?}");
}

#[test]
fn should_keep_both_inner_links_clickable_with_default_options() {
    let out = content_with(ISSUE_503, ConversionOptions::default());
    assert_eq!(
        out, "- [/outer](/outer)\n\n| [One](/one)[Two](/two) |\n| ---------------------- |\n- After\n",
        "actual: {out:?}"
    );
}

#[test]
fn should_keep_an_inner_link_clickable_when_ragged_rows_route_to_the_layout_renderer() {
    let html = "<table border=\"0\"><tr><td colspan=\"2\"><a href=\"/outer\"><table><tr><td>\
<a href=\"/one\">One</a></td></tr></table></a></td></tr><tr><td>After</td><td>x</td></tr></table>";
    let out = content_with(html, ConversionOptions::default());
    assert_eq!(
        out, "- [/outer](/outer)\n\n| [One](/one) |\n| ----------- |\n- After x\n",
        "actual: {out:?}"
    );
}

#[test]
fn should_bail_tier1_and_match_tier2_under_auto_routing() {
    assert_tier1_bails_as_classifier(ISSUE_503);
    let auto = content_with(
        ISSUE_503,
        ConversionOptions {
            tier_strategy: TierStrategy::Auto,
            ..base_options()
        },
    );
    let tier2 = content_with(
        ISSUE_503,
        ConversionOptions {
            tier_strategy: TierStrategy::Tier2,
            ..base_options()
        },
    );
    assert_eq!(auto, tier2);
}
