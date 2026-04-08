use crate::enums::{
    WasmCodeBlockStyle, WasmHeadingStyle, WasmHighlightStyle, WasmLinkStyle, WasmListIndentType, WasmNewlineStyle,
    WasmOutputFormat, WasmPreprocessingPreset, WasmWhitespaceMode,
};
use html_to_markdown_rs::{ConversionOptionsUpdate, PreprocessingOptionsUpdate};
use serde::{Deserialize, Serialize};

/// HTML preprocessing options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WasmPreprocessingOptions {
    /// Enable preprocessing
    #[serde(default)]
    pub enabled: bool,
    /// Preprocessing preset
    #[serde(default)]
    pub preset: Option<WasmPreprocessingPreset>,
    /// Remove navigation elements
    #[serde(default = "default_true")]
    pub remove_navigation: bool,
    /// Remove form elements
    #[serde(default = "default_true")]
    pub remove_forms: bool,
}

fn default_true() -> bool {
    true
}

impl From<WasmPreprocessingOptions> for PreprocessingOptionsUpdate {
    fn from(val: WasmPreprocessingOptions) -> Self {
        Self {
            enabled: Some(val.enabled),
            preset: val.preset.map(Into::into),
            remove_navigation: Some(val.remove_navigation),
            remove_forms: Some(val.remove_forms),
        }
    }
}

/// Main conversion options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WasmConversionOptions {
    /// Heading style
    pub heading_style: Option<WasmHeadingStyle>,
    /// List indentation type
    pub list_indent_type: Option<WasmListIndentType>,
    /// List indentation width (spaces)
    pub list_indent_width: Option<usize>,
    /// Bullet characters for unordered lists
    pub bullets: Option<String>,
    /// Symbol for strong/emphasis (* or _)
    pub strong_em_symbol: Option<char>,
    /// Escape asterisks in text
    pub escape_asterisks: Option<bool>,
    /// Escape underscores in text
    pub escape_underscores: Option<bool>,
    /// Escape misc markdown characters
    pub escape_misc: Option<bool>,
    /// Escape all ASCII punctuation
    pub escape_ascii: Option<bool>,
    /// Default code language
    pub code_language: Option<String>,
    /// Use autolinks for bare URLs
    pub autolinks: Option<bool>,
    /// Add default title if none exists
    pub default_title: Option<bool>,
    /// Use <br> in tables instead of spaces
    pub br_in_tables: Option<bool>,
    /// Highlight style for `<mark>` elements
    pub highlight_style: Option<WasmHighlightStyle>,
    /// Extract metadata from HTML
    pub extract_metadata: Option<bool>,
    /// Whitespace handling mode
    pub whitespace_mode: Option<WasmWhitespaceMode>,
    /// Strip newlines from HTML before processing
    pub strip_newlines: Option<bool>,
    /// Enable text wrapping
    pub wrap: Option<bool>,
    /// Text wrap width
    pub wrap_width: Option<usize>,
    /// Treat block elements as inline
    pub convert_as_inline: Option<bool>,
    /// Subscript symbol
    pub sub_symbol: Option<String>,
    /// Superscript symbol
    pub sup_symbol: Option<String>,
    /// Newline style
    pub newline_style: Option<WasmNewlineStyle>,
    /// Code block style
    pub code_block_style: Option<WasmCodeBlockStyle>,
    /// Elements where images should remain as markdown
    pub keep_inline_images_in: Option<Vec<String>>,
    /// Skip images during conversion
    pub skip_images: Option<bool>,
    /// Preprocessing options
    pub preprocessing: Option<WasmPreprocessingOptions>,
    /// Source encoding (informational)
    pub encoding: Option<String>,
    /// Enable debug mode with diagnostic warnings
    pub debug: Option<bool>,
    /// List of HTML tags to strip
    pub strip_tags: Option<Vec<String>>,
    /// List of HTML tags to preserve as-is in the output
    pub preserve_tags: Option<Vec<String>>,
    /// Link rendering style
    pub link_style: Option<WasmLinkStyle>,
    /// Output format for conversion
    pub output_format: Option<WasmOutputFormat>,
    /// Include structured document tree in result
    pub include_document_structure: Option<bool>,
    /// Extract inline images from data URIs and SVGs
    pub extract_images: Option<bool>,
    /// Maximum decoded image size in bytes
    pub max_image_size: Option<u64>,
    /// Capture SVG elements as images
    pub capture_svg: Option<bool>,
    /// Infer image dimensions from data
    pub infer_dimensions: Option<bool>,
    /// Maximum DOM tree depth to recurse into
    pub max_depth: Option<usize>,
}

impl From<WasmConversionOptions> for ConversionOptionsUpdate {
    fn from(val: WasmConversionOptions) -> Self {
        Self {
            heading_style: val.heading_style.map(Into::into),
            list_indent_type: val.list_indent_type.map(Into::into),
            list_indent_width: val.list_indent_width,
            bullets: val.bullets,
            strong_em_symbol: val.strong_em_symbol,
            escape_asterisks: val.escape_asterisks,
            escape_underscores: val.escape_underscores,
            escape_misc: val.escape_misc,
            escape_ascii: val.escape_ascii,
            code_language: val.code_language,
            autolinks: val.autolinks,
            default_title: val.default_title,
            br_in_tables: val.br_in_tables,
            highlight_style: val.highlight_style.map(Into::into),
            extract_metadata: val.extract_metadata,
            whitespace_mode: val.whitespace_mode.map(Into::into),
            strip_newlines: val.strip_newlines,
            wrap: val.wrap,
            wrap_width: val.wrap_width,
            convert_as_inline: val.convert_as_inline,
            sub_symbol: val.sub_symbol,
            sup_symbol: val.sup_symbol,
            newline_style: val.newline_style.map(Into::into),
            code_block_style: val.code_block_style.map(Into::into),
            keep_inline_images_in: val.keep_inline_images_in,
            skip_images: val.skip_images,
            preprocessing: val.preprocessing.map(Into::into),
            encoding: val.encoding,
            debug: val.debug,
            strip_tags: val.strip_tags,
            preserve_tags: val.preserve_tags,
            link_style: val.link_style.map(Into::into),
            output_format: val.output_format.map(Into::into),
            include_document_structure: val.include_document_structure,
            extract_images: val.extract_images,
            max_image_size: val.max_image_size,
            capture_svg: val.capture_svg,
            infer_dimensions: val.infer_dimensions,
            max_depth: val.max_depth,
        }
    }
}

impl From<WasmConversionOptions> for html_to_markdown_rs::ConversionOptions {
    fn from(val: WasmConversionOptions) -> Self {
        html_to_markdown_rs::ConversionOptions::from(ConversionOptionsUpdate::from(val))
    }
}
