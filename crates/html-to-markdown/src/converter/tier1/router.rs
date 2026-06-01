//! Tier router — decides whether an input goes to Tier-1 or Tier-2.

use crate::converter::prescan::PrescanReport;
use crate::options::ConversionOptions;

/// The tier to use for a given input + options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TierChoice {
    /// Use the Tier-1 single-pass byte scanner.
    Tier1,
    /// Use the Tier-2 `tl::parse` + walk-node path.
    Tier2,
}

/// Classify the input against the given options and prescan report.
///
/// Returns `Tier2` if ANY of the conditions below are true; otherwise
/// returns `Tier1`.
///
/// # Structural / prescan gates (pre-existing)
///
/// - `report.had_custom_elements` — custom elements require spec-edge handling
/// - `report.had_cdata` — CDATA sections are not supported by Tier-1
/// - `report.had_unescaped_lt` — bare `<` that the prescan escaped
/// - `report.has_svg` — inline SVG triggers Tier-1 bail mid-scan; route
///   directly to Tier-2 to skip the wasted setup
/// - `options.wrap` — wrapping logic lives in the Tier-2 path (for now)
/// - `options.convert_as_inline` — inline-conversion mode not yet in Tier-1
/// - `options.hocr_spatial_tables` — hOCR spatial reconstruction is Tier-2 only
/// - `options.preprocessing.preset != PreprocessingPreset::Standard`
///   — non-standard preprocessing has Tier-2-specific semantics
/// - `!options.strip_tags.is_empty()` — tag stripping requires DOM awareness
/// - `!options.preserve_tags.is_empty()` — tag preservation requires DOM awareness
/// - `options.debug` — debug output is consistent only on Tier-2
///
/// `options.extract_metadata` no longer forces Tier-2: Tier-1 re-parses the
/// prescan's `head_range` slice and produces byte-identical YAML frontmatter.
///
/// `options.keep_inline_images_in` (inline-images feature) is now handled
/// natively in Tier-1; it no longer forces a Tier-2 route.
///
/// # Style-option gates (A1 — router style-option gate)
///
/// Tier-1 hardcodes certain output style choices.  When a `ConversionOptions`
/// value deviates from Tier-1's hardcoded value, the output would silently
/// differ from Tier-2's, breaking the byte-equality contract.  The following
/// table documents each style option and the gate added here:
///
/// | Option               | Tier-1 hardcoded value                  | Gate added?                              |
/// |----------------------|-----------------------------------------|------------------------------------------|
/// | `output_format`      | `OutputFormat::Markdown`                | Yes — non-Markdown                       |
/// | `heading_style`      | `HeadingStyle::Atx`                     | Yes — non-Atx                            |
/// | `code_block_style`   | `CodeBlockStyle::Indented`              | Yes — non-Indented                       |
/// | `strong_em_symbol`   | `'*'` (asterisk)                        | Yes — any other char                     |
/// | `bullets`            | `"-"` (first char `'-'`)                | Yes — other first char                   |
/// | `list_indent_width`  | `2` spaces                              | Yes — `!= 2`                             |
/// | `list_indent_type`   | `ListIndentType::Spaces`                | Yes — Tabs                               |
/// | `escape_asterisks`   | `false` (no escaping)                   | Yes — `true`                             |
/// | `escape_underscores` | `false` (no escaping)                   | Yes — `true`                             |
/// | `escape_misc`        | `false` (no escaping)                   | Yes — `true`                             |
/// | `escape_ascii`       | `false` (no escaping)                   | Yes — `true`                             |
/// | `whitespace_mode`    | `WhitespaceMode::Normalized`            | Yes — Strict                             |
/// | `newline_style`      | `NewlineStyle::Spaces`                  | Yes — Backslash                          |
/// | `code_language`      | irrelevant (Indented style)             | No — gated via `code_block_style`        |
/// | `autolinks`          | not implemented in Tier-1               | No — Tier-1 never transforms bare URLs   |
/// | `default_title`      | `false` (not honored)                   | Yes — `true`                             |
/// | `sub_symbol`         | `""` (transparent pass-through)         | Yes — non-empty                          |
/// | `sup_symbol`         | `""` (transparent pass-through)         | Yes — non-empty                          |
/// | `highlight_style`    | transparent (`<mark>` → plain text)     | Yes — non-None                           |
/// | `link_style`         | `LinkStyle::Inline`                     | Yes — Reference                          |
/// | `url_escape_style`   | `UrlEscapeStyle::Angle` (raw href)      | Yes — Percent                            |
/// | `compact_tables`     | `false` (padded cells: `\| cell \|`)    | Yes — `true`                             |
/// | `br_in_tables`       | bails on `<br>` in cells                | No — covered by scanner bail             |
/// | `hocr_spatial_tables`| Tier-2 only (structural gate)           | Already gated above                      |
pub fn classify(report: &PrescanReport, options: &ConversionOptions) -> TierChoice {
    use crate::options::{
        CodeBlockStyle, HeadingStyle, HighlightStyle, LinkStyle, ListIndentType, NewlineStyle, OutputFormat,
        PreprocessingPreset, UrlEscapeStyle, WhitespaceMode,
    };

    if report.had_custom_elements
        || report.had_cdata
        || report.had_unescaped_lt
        || report.has_svg
        || options.wrap
        || options.convert_as_inline
        || options.preprocessing.preset != PreprocessingPreset::Standard
        || !options.strip_tags.is_empty()
        || !options.preserve_tags.is_empty()
        || options.debug
        // ── Style-option gates ────────────────────────────────────────────────
        // output_format: Tier-1 only produces Markdown; other formats are Tier-2 only.
        || options.output_format != OutputFormat::Markdown
        // heading_style: Tier-1 hardcodes ATX; non-ATX headings differ.
        || options.heading_style != HeadingStyle::Atx
        // code_block_style: Tier-1 always emits 4-space indented blocks.
        || options.code_block_style != CodeBlockStyle::Indented
        // strong_em_symbol: Tier-1 hardcodes `*`/`**`.
        || options.strong_em_symbol != '*'
        // bullets: Tier-1 hardcodes `-` as the unordered-list bullet.
        || !options.bullets.starts_with('-')
        // list_indent_width: Tier-1 hardcodes 2-space indentation per depth level.
        || options.list_indent_width != 2
        // list_indent_type: Tier-1 hardcodes spaces for list indentation.
        || options.list_indent_type != ListIndentType::Spaces
        // escape_*: Tier-1 does not perform any text escaping. If the caller
        // requests escaping, Tier-2 must handle it.
        || options.escape_asterisks
        || options.escape_underscores
        || options.escape_misc
        || options.escape_ascii
        // whitespace_mode: Tier-1 always normalizes whitespace (collapses runs).
        || options.whitespace_mode != WhitespaceMode::Normalized
        // newline_style: Tier-1 emits `  \n` for `<br>` (two-space style).
        || options.newline_style != NewlineStyle::Spaces
        // default_title: Tier-1 does not insert a default document title.
        || options.default_title
        // sub_symbol / sup_symbol: Tier-1 passes <sub>/<sup> content through
        // as plain text (no wrapping symbol). Only safe when symbol is empty.
        || !options.sub_symbol.is_empty()
        || !options.sup_symbol.is_empty()
        // highlight_style: Tier-1 passes <mark> content through as plain text.
        // This is byte-identical to Tier-2 only when style is None (no wrapping).
        || options.highlight_style != HighlightStyle::None
        // link_style: Tier-1 always emits inline `[text](href)` links; reference
        // style (with a link-reference block at end of document) is Tier-2 only.
        || options.link_style != LinkStyle::Inline
        // url_escape_style: Tier-1 emits hrefs verbatim (no angle-bracket wrapping
        // or percent-encoding). This matches Tier-2's Angle behaviour for URLs that
        // contain no spaces, but diverges for Percent (which percent-encodes).
        || options.url_escape_style != UrlEscapeStyle::Angle
        // compact_tables: Tier-1 always emits padded `| cell |` GFM tables.
        // compact_tables=true would produce `|cell|`, which Tier-1 never does.
        || options.compact_tables
    {
        return TierChoice::Tier2;
    }

    TierChoice::Tier1
}
