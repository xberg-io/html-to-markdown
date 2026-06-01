//! Tests for the A1 style-option router gates.
//!
//! Each gated option has a test asserting that setting it to a non-default
//! value forces `RouterDecision::Tier2`, and a companion test confirming the
//! default value still allows `RouterDecision::Tier1` (with all structural
//! blockers turned off).

use html_to_markdown_rs::options::{
    CodeBlockStyle, ConversionOptions, HeadingStyle, HighlightStyle, LinkStyle, ListIndentType, NewlineStyle,
    OutputFormat, UrlEscapeStyle, WhitespaceMode,
};
use html_to_markdown_rs::prescan;
use html_to_markdown_rs::tier1::router::{RouterDecision, classify};

// ── Helper ──────────────────────────────────────────────────────────────────

/// Return the minimal `ConversionOptions` that do not hit any structural gate
/// AND have all style options set to the values Tier-1 hardcodes, so that
/// only the option under test can cause a Tier-2 route.
///
/// This baseline must have:
/// - `extract_metadata: false` (structural gate)
/// - `highlight_style: HighlightStyle::None` — the default `DoubleEqual` triggers
///   the highlight gate because Tier-1 emits `<mark>` content as plain text
/// - `code_block_style: CodeBlockStyle::Indented` — Tier-1 always emits 4-space
///   indented pre blocks regardless of the option; `Backticks` (the default) would
///   fire the `code_block_style` gate
fn base_opts() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        code_block_style: CodeBlockStyle::Indented,
        ..ConversionOptions::default()
    }
}

/// Prescan a trivial paragraph and classify with the given options.
fn route(options: &ConversionOptions) -> RouterDecision {
    let (_cleaned, report) = prescan::run("<p>hello</p>");
    classify(&report, options)
}

// ── heading_style ────────────────────────────────────────────────────────────

#[test]
fn gate_heading_style_atx_allows_tier1() {
    let opts = ConversionOptions {
        heading_style: HeadingStyle::Atx,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_heading_style_setext_forces_tier2() {
    let opts = ConversionOptions {
        heading_style: HeadingStyle::Underlined,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

#[test]
fn gate_heading_style_atx_closed_forces_tier2() {
    let opts = ConversionOptions {
        heading_style: HeadingStyle::AtxClosed,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

// ── code_block_style ─────────────────────────────────────────────────────────

#[test]
fn gate_code_block_style_indented_allows_tier1() {
    let opts = ConversionOptions {
        code_block_style: CodeBlockStyle::Indented,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_code_block_style_backticks_forces_tier2() {
    let opts = ConversionOptions {
        code_block_style: CodeBlockStyle::Backticks,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

#[test]
fn gate_code_block_style_tildes_forces_tier2() {
    let opts = ConversionOptions {
        code_block_style: CodeBlockStyle::Tildes,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

// ── strong_em_symbol ─────────────────────────────────────────────────────────

#[test]
fn gate_strong_em_symbol_asterisk_allows_tier1() {
    let opts = ConversionOptions {
        strong_em_symbol: '*',
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_strong_em_symbol_underscore_forces_tier2() {
    let opts = ConversionOptions {
        strong_em_symbol: '_',
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

// ── bullets ──────────────────────────────────────────────────────────────────

#[test]
fn gate_bullets_dash_allows_tier1() {
    let opts = ConversionOptions {
        bullets: "-".to_string(),
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_bullets_asterisk_forces_tier2() {
    let opts = ConversionOptions {
        bullets: "*".to_string(),
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

#[test]
fn gate_bullets_plus_forces_tier2() {
    let opts = ConversionOptions {
        bullets: "+".to_string(),
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

// ── list_indent_width ────────────────────────────────────────────────────────

#[test]
fn gate_list_indent_width_2_allows_tier1() {
    let opts = ConversionOptions {
        list_indent_width: 2,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_list_indent_width_4_forces_tier2() {
    let opts = ConversionOptions {
        list_indent_width: 4,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

#[test]
fn gate_list_indent_width_0_forces_tier2() {
    let opts = ConversionOptions {
        list_indent_width: 0,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

// ── list_indent_type ─────────────────────────────────────────────────────────

#[test]
fn gate_list_indent_type_spaces_allows_tier1() {
    let opts = ConversionOptions {
        list_indent_type: ListIndentType::Spaces,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_list_indent_type_tabs_forces_tier2() {
    let opts = ConversionOptions {
        list_indent_type: ListIndentType::Tabs,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

// ── whitespace_mode ───────────────────────────────────────────────────────────

#[test]
fn gate_whitespace_mode_normalized_allows_tier1() {
    let opts = ConversionOptions {
        whitespace_mode: WhitespaceMode::Normalized,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_whitespace_mode_strict_forces_tier2() {
    let opts = ConversionOptions {
        whitespace_mode: WhitespaceMode::Strict,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

// ── newline_style ─────────────────────────────────────────────────────────────

#[test]
fn gate_newline_style_spaces_allows_tier1() {
    let opts = ConversionOptions {
        newline_style: NewlineStyle::Spaces,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_newline_style_backslash_forces_tier2() {
    let opts = ConversionOptions {
        newline_style: NewlineStyle::Backslash,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

// ── default_title ─────────────────────────────────────────────────────────────

#[test]
fn gate_default_title_false_allows_tier1() {
    let opts = ConversionOptions {
        default_title: false,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_default_title_true_forces_tier2() {
    let opts = ConversionOptions {
        default_title: true,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

// ── sub_symbol ────────────────────────────────────────────────────────────────

#[test]
fn gate_sub_symbol_empty_allows_tier1() {
    let opts = ConversionOptions {
        sub_symbol: String::new(),
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_sub_symbol_nonempty_forces_tier2() {
    let opts = ConversionOptions {
        sub_symbol: "~".to_string(),
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

// ── sup_symbol ────────────────────────────────────────────────────────────────

#[test]
fn gate_sup_symbol_empty_allows_tier1() {
    let opts = ConversionOptions {
        sup_symbol: String::new(),
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_sup_symbol_nonempty_forces_tier2() {
    let opts = ConversionOptions {
        sup_symbol: "^".to_string(),
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

// ── highlight_style ───────────────────────────────────────────────────────────

#[test]
fn gate_highlight_style_none_allows_tier1() {
    let opts = ConversionOptions {
        highlight_style: HighlightStyle::None,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_highlight_style_double_equal_forces_tier2() {
    let opts = ConversionOptions {
        highlight_style: HighlightStyle::DoubleEqual,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

#[test]
fn gate_highlight_style_html_forces_tier2() {
    let opts = ConversionOptions {
        highlight_style: HighlightStyle::Html,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

#[test]
fn gate_highlight_style_bold_forces_tier2() {
    let opts = ConversionOptions {
        highlight_style: HighlightStyle::Bold,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

// ── output_format ─────────────────────────────────────────────────────────────

#[test]
fn gate_output_format_markdown_allows_tier1() {
    let opts = ConversionOptions {
        output_format: OutputFormat::Markdown,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_output_format_djot_forces_tier2() {
    let opts = ConversionOptions {
        output_format: OutputFormat::Djot,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

#[test]
fn gate_output_format_plain_forces_tier2() {
    let opts = ConversionOptions {
        output_format: OutputFormat::Plain,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

// ── escape flags ──────────────────────────────────────────────────────────────

#[test]
fn gate_escape_asterisks_false_allows_tier1() {
    let opts = ConversionOptions {
        escape_asterisks: false,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_escape_asterisks_true_forces_tier2() {
    let opts = ConversionOptions {
        escape_asterisks: true,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

#[test]
fn gate_escape_underscores_false_allows_tier1() {
    let opts = ConversionOptions {
        escape_underscores: false,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_escape_underscores_true_forces_tier2() {
    let opts = ConversionOptions {
        escape_underscores: true,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

#[test]
fn gate_escape_misc_false_allows_tier1() {
    let opts = ConversionOptions {
        escape_misc: false,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_escape_misc_true_forces_tier2() {
    let opts = ConversionOptions {
        escape_misc: true,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

#[test]
fn gate_escape_ascii_false_allows_tier1() {
    let opts = ConversionOptions {
        escape_ascii: false,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_escape_ascii_true_forces_tier2() {
    let opts = ConversionOptions {
        escape_ascii: true,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

// ── link_style ────────────────────────────────────────────────────────────────

#[test]
fn gate_link_style_inline_allows_tier1() {
    let opts = ConversionOptions {
        link_style: LinkStyle::Inline,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_link_style_reference_forces_tier2() {
    let opts = ConversionOptions {
        link_style: LinkStyle::Reference,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

// ── url_escape_style ──────────────────────────────────────────────────────────

#[test]
fn gate_url_escape_style_angle_allows_tier1() {
    let opts = ConversionOptions {
        url_escape_style: UrlEscapeStyle::Angle,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_url_escape_style_percent_forces_tier2() {
    let opts = ConversionOptions {
        url_escape_style: UrlEscapeStyle::Percent,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}

// ── compact_tables ────────────────────────────────────────────────────────────

#[test]
fn gate_compact_tables_false_allows_tier1() {
    let opts = ConversionOptions {
        compact_tables: false,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier1);
}

#[test]
fn gate_compact_tables_true_forces_tier2() {
    let opts = ConversionOptions {
        compact_tables: true,
        ..base_opts()
    };
    assert_eq!(route(&opts), RouterDecision::Tier2);
}
