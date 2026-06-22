//! MCP request parameter types.
//!
//! [`ConvertConfig`] is a typed, schema-bearing mirror of the settable fields of
//! [`crate::options::ConversionOptions`]. It exists so the `convert_html` tool can
//! advertise every conversion option to MCP clients through its generated
//! `inputSchema` (rmcp requires the tool parameter type to implement
//! [`schemars::JsonSchema`], which the core option types do not). Every field is
//! optional; only the fields a client sends override the corresponding default.
//!
//! The mirror is kept honest by the `mirror_covers_all_core_fields` drift-guard
//! test below, which fails if a field is ever added to `ConversionOptions` without
//! being mirrored here.

use crate::options::{
    CodeBlockStyle, ConversionOptions, ConversionOptionsUpdate, HeadingStyle, HighlightStyle, LinkStyle,
    ListIndentType, NewlineStyle, OutputFormat, PreprocessingOptionsUpdate, PreprocessingPreset, TierStrategy,
    UrlEscapeStyle, WhitespaceMode,
};
use rmcp::schemars;

/// Typed conversion options for the `convert_html` tool.
///
/// Mirrors the settable fields of [`crate::options::ConversionOptions`]. Every
/// field is optional and defaults to the engine default when omitted. Enum-valued
/// options are accepted as strings (case-insensitive) and parsed with the same
/// parsers the core library uses.
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Default, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct ConvertConfig {
    /// Heading style: `"atx"` (default), `"atxclosed"`, or `"underlined"`.
    pub heading_style: Option<String>,
    /// List indentation: `"spaces"` (default) or `"tabs"`.
    pub list_indent_type: Option<String>,
    /// Spaces (or tabs) per list-indent level. Default `2`.
    pub list_indent_width: Option<usize>,
    /// Bullet characters for unordered lists. Default `"-*+"`.
    pub bullets: Option<String>,
    /// Emphasis marker; first character is used (`"*"` default or `"_"`).
    pub strong_em_symbol: Option<String>,
    /// Escape `*` in plain text. Default `false`.
    pub escape_asterisks: Option<bool>,
    /// Escape `_` in plain text. Default `false`.
    pub escape_underscores: Option<bool>,
    /// Escape miscellaneous Markdown metacharacters (`[]()#` etc.). Default `false`.
    pub escape_misc: Option<bool>,
    /// Escape ASCII characters special in some Markdown dialects. Default `false`.
    pub escape_ascii: Option<bool>,
    /// Default language for fenced code blocks with no hint. Default empty.
    pub code_language: Option<String>,
    /// Convert bare URLs into autolinks. Default `true`.
    pub autolinks: Option<bool>,
    /// Emit a default title when no `<title>` is present. Default `false`.
    pub default_title: Option<bool>,
    /// Render `<br>` inside table cells as literal line breaks. Default `false`.
    pub br_in_tables: Option<bool>,
    /// Emit compact (unpadded) GFM tables. Default `false`.
    pub compact_tables: Option<bool>,
    /// `<mark>` rendering: `"doubleequal"` (default), `"html"`, `"bold"`, or `"none"`.
    pub highlight_style: Option<String>,
    /// Populate `metadata` from `<head>`/`<meta>`. Default `true`.
    pub extract_metadata: Option<bool>,
    /// Whitespace handling: `"normalized"` (default) or `"strict"`.
    pub whitespace_mode: Option<String>,
    /// Strip all newlines, producing single-line output. Default `false`.
    pub strip_newlines: Option<bool>,
    /// Wrap long lines at `wrap_width`. Default `false`.
    pub wrap: Option<bool>,
    /// Maximum line width when `wrap` is enabled. Default `80`.
    pub wrap_width: Option<usize>,
    /// Treat the whole document as inline content. Default `false`.
    pub convert_as_inline: Option<bool>,
    /// Markdown notation for subscript text (e.g. `"~"`). Default empty.
    pub sub_symbol: Option<String>,
    /// Markdown notation for superscript text (e.g. `"^"`). Default empty.
    pub sup_symbol: Option<String>,
    /// Hard line-break syntax: `"spaces"` (default) or `"backslash"`.
    pub newline_style: Option<String>,
    /// Fenced code block style: `"backticks"` (default), `"tildes"`, or `"indented"`.
    pub code_block_style: Option<String>,
    /// HTML tags whose `<img>` children stay inline.
    pub keep_inline_images_in: Option<Vec<String>>,
    /// HTML preprocessing (cleanup) options.
    pub preprocessing: Option<PreprocessingParams>,
    /// Expected input character encoding. Default `"utf-8"`.
    pub encoding: Option<String>,
    /// Emit debug information during conversion. Default `false`.
    pub debug: Option<bool>,
    /// HTML tags to strip (remove wrapper, keep children).
    pub strip_tags: Option<Vec<String>>,
    /// HTML tags to preserve verbatim in the output.
    pub preserve_tags: Option<Vec<String>>,
    /// Omit all `<img>` elements from the output. Default `false`.
    pub skip_images: Option<bool>,
    /// URL escaping: `"angle"` (default) or `"percent"`.
    pub url_escape_style: Option<String>,
    /// Link rendering: `"inline"` (default) or `"reference"`.
    pub link_style: Option<String>,
    /// Output format: `"markdown"` (default), `"djot"`, or `"plain"`.
    pub output_format: Option<String>,
    /// Include the structured document tree in the result. Default `false`.
    pub include_document_structure: Option<bool>,
    /// Extract inline images from data URIs and SVGs. Default `false`.
    pub extract_images: Option<bool>,
    /// Maximum decoded image size in bytes. Default `5242880` (5 MB).
    pub max_image_size: Option<u64>,
    /// Capture inline `<svg>` elements as images. Default `false`.
    pub capture_svg: Option<bool>,
    /// Infer image dimensions from data. Default `true`.
    pub infer_dimensions: Option<bool>,
    /// Maximum DOM traversal depth; omit for unlimited.
    pub max_depth: Option<u64>,
    /// CSS selectors for elements to exclude entirely (element + descendants).
    pub exclude_selectors: Option<Vec<String>>,
    /// Conversion tier: `"auto"` (default) or `"tier2"`.
    pub tier_strategy: Option<String>,
}

/// Typed mirror of [`crate::options::PreprocessingOptions`] for MCP input.
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Default, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct PreprocessingParams {
    /// Enable preprocessing globally. Default `true`.
    pub enabled: Option<bool>,
    /// Preset: `"minimal"`, `"standard"` (default), or `"aggressive"`.
    pub preset: Option<String>,
    /// Remove navigation elements (nav, breadcrumbs, menus, sidebars). Default `true`.
    pub remove_navigation: Option<bool>,
    /// Remove form elements (forms, inputs, buttons). Default `true`.
    pub remove_forms: Option<bool>,
}

impl From<PreprocessingParams> for PreprocessingOptionsUpdate {
    fn from(params: PreprocessingParams) -> Self {
        Self {
            enabled: params.enabled,
            preset: params.preset.map(|s| PreprocessingPreset::parse(&s)),
            remove_navigation: params.remove_navigation,
            remove_forms: params.remove_forms,
        }
    }
}

impl From<ConvertConfig> for ConversionOptionsUpdate {
    fn from(config: ConvertConfig) -> Self {
        Self {
            heading_style: config.heading_style.map(|s| HeadingStyle::parse(&s)),
            list_indent_type: config.list_indent_type.map(|s| ListIndentType::parse(&s)),
            list_indent_width: config.list_indent_width,
            bullets: config.bullets,
            strong_em_symbol: config.strong_em_symbol.and_then(|s| s.chars().next()),
            escape_asterisks: config.escape_asterisks,
            escape_underscores: config.escape_underscores,
            escape_misc: config.escape_misc,
            escape_ascii: config.escape_ascii,
            code_language: config.code_language,
            autolinks: config.autolinks,
            default_title: config.default_title,
            br_in_tables: config.br_in_tables,
            compact_tables: config.compact_tables,
            highlight_style: config.highlight_style.map(|s| HighlightStyle::parse(&s)),
            extract_metadata: config.extract_metadata,
            whitespace_mode: config.whitespace_mode.map(|s| WhitespaceMode::parse(&s)),
            strip_newlines: config.strip_newlines,
            wrap: config.wrap,
            wrap_width: config.wrap_width,
            convert_as_inline: config.convert_as_inline,
            sub_symbol: config.sub_symbol,
            sup_symbol: config.sup_symbol,
            newline_style: config.newline_style.map(|s| NewlineStyle::parse(&s)),
            code_block_style: config.code_block_style.map(|s| CodeBlockStyle::parse(&s)),
            keep_inline_images_in: config.keep_inline_images_in,
            preprocessing: config.preprocessing.map(Into::into),
            encoding: config.encoding,
            debug: config.debug,
            strip_tags: config.strip_tags,
            preserve_tags: config.preserve_tags,
            skip_images: config.skip_images,
            url_escape_style: config.url_escape_style.map(|s| UrlEscapeStyle::parse(&s)),
            link_style: config.link_style.map(|s| LinkStyle::parse(&s)),
            output_format: config.output_format.map(|s| OutputFormat::parse(&s)),
            include_document_structure: config.include_document_structure,
            extract_images: config.extract_images,
            max_image_size: config.max_image_size,
            capture_svg: config.capture_svg,
            infer_dimensions: config.infer_dimensions,
            max_depth: config.max_depth.map(|d| Some(usize::try_from(d).unwrap_or(usize::MAX))),
            exclude_selectors: config.exclude_selectors,
            tier_strategy: config.tier_strategy.map(|s| parse_tier_strategy(&s)),
            #[cfg(feature = "visitor")]
            visitor: None,
        }
    }
}

impl From<ConvertConfig> for ConversionOptions {
    fn from(config: ConvertConfig) -> Self {
        Self::from_update(config.into())
    }
}

/// Parse a [`TierStrategy`] from a string, falling back to the default (`Auto`).
///
/// `TierStrategy` has no public `parse` constructor, so this matches on the
/// normalised token the same way the other option enums do (case- and
/// separator-insensitive), accepting `"auto"` (default) and `"tier2"`.
fn parse_tier_strategy(value: &str) -> TierStrategy {
    match crate::options::validation::normalize_token(value).as_str() {
        "tier2" => TierStrategy::Tier2,
        _ => TierStrategy::Auto,
    }
}

/// Parameters for the `convert_html` MCP tool.
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ConvertHtmlParams {
    /// The HTML string to convert to Markdown.
    pub html: String,

    /// Optional typed conversion options. Omit for engine defaults.
    #[serde(default)]
    pub config: Option<ConvertConfig>,

    /// When `true`, return the full `ConversionResult` serialised as JSON
    /// (content, tables, document structure, metadata, warnings) instead of the
    /// bare Markdown string.
    #[serde(default)]
    pub json: bool,
}

/// Parameters for the `extract_metadata` MCP tool.
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExtractMetadataParams {
    /// The HTML string to extract metadata from.
    pub html: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn json_keys(value: &serde_json::Value) -> BTreeSet<String> {
        value
            .as_object()
            .expect("expected a JSON object")
            .keys()
            .cloned()
            .collect()
    }

    /// Drift guard: every settable `ConversionOptions` field must be mirrored by
    /// `ConvertConfig`. `ConversionOptions` serialises every field except the
    /// serde-skipped `visitor`, and `ConvertConfig` serialises every field as
    /// `null` (no `skip_serializing_if`), so the key sets must match exactly.
    /// Fails loudly, naming the offending field, if a core option is added
    /// without being mirrored here.
    #[test]
    fn mirror_covers_all_core_fields() {
        let core = serde_json::to_value(ConversionOptions::default()).unwrap();
        let mirror = serde_json::to_value(ConvertConfig::default()).unwrap();
        assert_eq!(
            json_keys(&core),
            json_keys(&mirror),
            "ConvertConfig drifted from ConversionOptions — add/remove the mismatched field(s)"
        );
    }

    /// Same drift guard for the nested preprocessing mirror.
    #[test]
    fn preprocessing_mirror_covers_all_core_fields() {
        use crate::options::PreprocessingOptions;
        let core = serde_json::to_value(PreprocessingOptions::default()).unwrap();
        let mirror = serde_json::to_value(PreprocessingParams::default()).unwrap();
        assert_eq!(json_keys(&core), json_keys(&mirror));
    }

    #[test]
    fn test_convert_html_params_minimal() {
        let params: ConvertHtmlParams = serde_json::from_str(r#"{"html": "<h1>Hi</h1>"}"#).unwrap();
        assert_eq!(params.html, "<h1>Hi</h1>");
        assert!(params.config.is_none());
        assert!(!params.json);
    }

    #[test]
    fn test_enum_strings_parse_into_typed_options() {
        let config = ConvertConfig {
            heading_style: Some("atxclosed".into()),
            output_format: Some("djot".into()),
            code_block_style: Some("tildes".into()),
            ..ConvertConfig::default()
        };
        let opts: ConversionOptions = config.into();
        assert_eq!(opts.heading_style, HeadingStyle::AtxClosed);
        assert_eq!(opts.output_format, OutputFormat::Djot);
        assert_eq!(opts.code_block_style, CodeBlockStyle::Tildes);
    }

    #[test]
    fn test_unknown_enum_string_falls_back_to_default() {
        let config = ConvertConfig {
            heading_style: Some("nonsense".into()),
            ..ConvertConfig::default()
        };
        let opts: ConversionOptions = config.into();
        // HeadingStyle::parse falls back to Underlined for unknown input.
        assert_eq!(opts.heading_style, HeadingStyle::Underlined);
    }

    #[test]
    fn test_partial_config_leaves_other_fields_at_default() {
        let config = ConvertConfig {
            wrap: Some(true),
            wrap_width: Some(100),
            ..ConvertConfig::default()
        };
        let opts: ConversionOptions = config.into();
        assert!(opts.wrap);
        assert_eq!(opts.wrap_width, 100);
        // Untouched fields retain defaults.
        assert!(opts.autolinks);
        assert_eq!(opts.bullets, "-*+");
        assert_eq!(opts.heading_style, HeadingStyle::Atx);
    }

    #[test]
    fn test_strong_em_symbol_takes_first_char() {
        let config = ConvertConfig {
            strong_em_symbol: Some("_".into()),
            ..ConvertConfig::default()
        };
        let opts: ConversionOptions = config.into();
        assert_eq!(opts.strong_em_symbol, '_');
    }

    #[test]
    fn test_max_depth_maps_through() {
        let config = ConvertConfig {
            max_depth: Some(5),
            ..ConvertConfig::default()
        };
        let opts: ConversionOptions = config.into();
        assert_eq!(opts.max_depth, Some(5));
    }

    #[test]
    fn test_preprocessing_nested_config() {
        let config = ConvertConfig {
            preprocessing: Some(PreprocessingParams {
                preset: Some("aggressive".into()),
                remove_forms: Some(false),
                ..PreprocessingParams::default()
            }),
            ..ConvertConfig::default()
        };
        let opts: ConversionOptions = config.into();
        assert_eq!(opts.preprocessing.preset, PreprocessingPreset::Aggressive);
        assert!(!opts.preprocessing.remove_forms);
        // Untouched preprocessing fields retain defaults.
        assert!(opts.preprocessing.enabled);
        assert!(opts.preprocessing.remove_navigation);
    }

    #[test]
    fn test_tier_strategy_parses() {
        let config = ConvertConfig {
            tier_strategy: Some("tier2".into()),
            ..ConvertConfig::default()
        };
        let opts: ConversionOptions = config.into();
        assert_eq!(opts.tier_strategy, TierStrategy::Tier2);
    }

    #[test]
    fn test_unknown_top_level_field_is_rejected() {
        // deny_unknown_fields on ConvertConfig rejects typos in option names.
        let result: Result<ConvertConfig, _> = serde_json::from_str(r#"{"unknown_field_xyz": true}"#);
        assert!(result.is_err());
    }

    #[test]
    fn test_schema_generation_exposes_config_fields() {
        let schema = schemars::schema_for!(ConvertHtmlParams);
        let json = serde_json::to_value(&schema).unwrap();
        let text = serde_json::to_string(&json).unwrap();
        // The generated schema must surface the typed options (proves discoverability).
        assert!(text.contains("heading_style"), "schema must expose heading_style");
        assert!(text.contains("output_format"), "schema must expose output_format");
        assert!(text.contains("preprocessing"), "schema must expose preprocessing");
    }
}
