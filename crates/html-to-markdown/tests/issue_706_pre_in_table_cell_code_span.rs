#![allow(missing_docs)]
#![cfg(feature = "testkit")]

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{ConversionOptions, HighlightStyle, TierStrategy, convert, tier1};

fn options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        ..ConversionOptions::default()
    }
}

fn assert_tiers(html: &str, expected: &str) {
    assert_tiers_with_options(html, options(), expected);
}

fn assert_tiers_with_options(html: &str, base: ConversionOptions, expected: &str) {
    let tier1_output = tier1::run(html, &PrescanReport::default(), &base).expect("tier1 should accept the table");
    let tier2_output = convert(
        html,
        Some(ConversionOptions {
            tier_strategy: TierStrategy::Tier2,
            ..base
        }),
    )
    .expect("tier2 conversion should succeed")
    .content
    .unwrap_or_default();

    assert_eq!(tier1_output, tier2_output, "converter tiers diverged for {html:?}");
    assert_eq!(tier2_output, expected);
}

/// A page that Tier-1 hands over to Tier-2: a block inside `<pre>`. Tier-1 must refuse it, and
/// `convert` must write `expected`, as Tier-2 does.
fn assert_handed_over(html: &str, base: &ConversionOptions, expected: &str) {
    let tier1_output = tier1::run(html, &PrescanReport::default(), base);
    assert!(
        matches!(tier1_output, Err(tier1::BailReason::Classifier)),
        "tier1 must hand {html:?} over to tier2, got {tier1_output:?}"
    );
    let run = |tier_strategy| {
        convert(
            html,
            Some(ConversionOptions {
                tier_strategy,
                ..base.clone()
            }),
        )
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
    };
    let output = run(TierStrategy::Auto);
    assert_eq!(
        output,
        run(TierStrategy::Tier2),
        "convert diverged from tier2 for {html:?}"
    );
    assert_eq!(output, expected);
}

#[test]
fn should_render_preformatted_table_cell_text_as_a_code_span() {
    assert_tiers(
        r"<table><tr><td><pre>*x* a\*b</pre></td><td>z</td></tr></table>",
        "| `*x* a\\*b` | z |\n| ---------- | --- |\n",
    );
}

#[test]
fn should_trim_a_preformatted_table_cells_trailing_line_break_before_the_code_span() {
    assert_tiers(
        "<table><tr><td><pre>a\n</pre></td><td>z</td></tr></table>",
        "| `a` | z |\n| --- | --- |\n",
    );
}

#[test]
fn should_use_a_delimiter_longer_than_backticks_in_preformatted_table_cell_text() {
    assert_tiers(
        "<table><tr><td><pre>a`b</pre></td><td>z</td></tr></table>",
        "| ``a`b`` | z |\n| ------- | --- |\n",
    );
}

#[test]
fn should_keep_a_br_between_preformatted_segments_outside_the_code_spans() {
    let options = ConversionOptions {
        br_in_tables: true,
        ..options()
    };
    assert_handed_over(
        "<table><tr><td><pre>a<br><div>b</div></pre></td><td>z</td></tr></table>",
        &options,
        "| `a`<br>`b` | z |\n| ---------- | --- |\n",
    );
}

#[test]
fn should_keep_a_nested_br_between_preformatted_segments_outside_the_code_spans() {
    let options = ConversionOptions {
        br_in_tables: true,
        ..options()
    };
    assert_tiers_with_options(
        "<table><tr><td><pre><code>a<br><span>b</span></code></pre></td><td>z</td></tr></table>",
        options,
        "| `a`<br>`b` | z |\n| ---------- | --- |\n",
    );
}

#[test]
fn should_keep_consecutive_br_boundaries_between_preformatted_segments() {
    let options = ConversionOptions {
        br_in_tables: true,
        ..options()
    };
    assert_tiers_with_options(
        "<table><tr><td><pre>a<br><br>b</pre></td><td>z</td></tr></table>",
        options,
        "| `a`<br><br>`b` | z |\n| -------------- | --- |\n",
    );
}

#[test]
fn should_dedent_preformatted_table_cell_content_before_splitting_at_br() {
    let options = ConversionOptions {
        br_in_tables: true,
        ..options()
    };
    assert_tiers_with_options(
        "<table><tr><td><pre>a<br>  b</pre></td><td>z</td></tr></table>",
        options,
        "| `a`<br>`  b` | z |\n| ------------ | --- |\n",
    );
}

#[test]
fn should_keep_literal_br_text_inside_a_preformatted_code_span() {
    let options = ConversionOptions {
        br_in_tables: true,
        ..options()
    };
    assert_tiers_with_options(
        "<table><tr><td><pre>&lt;br></pre></td><td>z</td></tr></table>",
        options,
        "| `<br>` | z |\n| ------ | --- |\n",
    );
}
