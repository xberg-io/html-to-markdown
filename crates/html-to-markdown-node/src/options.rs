use crate::enums::{
    JsCodeBlockStyle, JsHeadingStyle, JsHighlightStyle, JsLinkStyle, JsListIndentType, JsNewlineStyle, JsOutputFormat,
    JsPreprocessingPreset, JsWhitespaceMode,
};
use html_to_markdown_rs::{
    ConversionOptions as RustConversionOptions, ConversionOptionsUpdate, DEFAULT_INLINE_IMAGE_LIMIT,
    InlineImageConfig as RustInlineImageConfig, InlineImageConfigUpdate,
    PreprocessingOptions as RustPreprocessingOptions, PreprocessingOptionsUpdate,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// HTML preprocessing options
#[napi(object)]
pub struct JsPreprocessingOptions {
    /// Enable preprocessing
    pub enabled: Option<bool>,
    /// Preprocessing preset
    pub preset: Option<JsPreprocessingPreset>,
    /// Remove navigation elements
    pub remove_navigation: Option<bool>,
    /// Remove form elements
    pub remove_forms: Option<bool>,
}

impl From<JsPreprocessingOptions> for PreprocessingOptionsUpdate {
    fn from(val: JsPreprocessingOptions) -> Self {
        Self {
            enabled: val.enabled,
            preset: val.preset.map(Into::into),
            remove_navigation: val.remove_navigation,
            remove_forms: val.remove_forms,
        }
    }
}

impl From<JsPreprocessingOptions> for RustPreprocessingOptions {
    fn from(val: JsPreprocessingOptions) -> Self {
        let update: PreprocessingOptionsUpdate = val.into();
        let mut opts = Self::default();
        opts.apply_update(update);
        opts
    }
}

/// Main conversion options
#[napi(object)]
pub struct JsConversionOptions {
    /// Heading style
    pub heading_style: Option<JsHeadingStyle>,
    /// List indentation type
    pub list_indent_type: Option<JsListIndentType>,
    /// List indentation width (spaces)
    pub list_indent_width: Option<u32>,
    /// Bullet characters for unordered lists
    pub bullets: Option<String>,
    /// Symbol for strong/emphasis (* or _)
    pub strong_em_symbol: Option<String>,
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
    /// Highlight style for <mark> elements
    pub highlight_style: Option<JsHighlightStyle>,
    /// Extract metadata from HTML
    pub extract_metadata: Option<bool>,
    /// Whitespace handling mode
    pub whitespace_mode: Option<JsWhitespaceMode>,
    /// Strip newlines from HTML before processing
    pub strip_newlines: Option<bool>,
    /// Enable text wrapping
    pub wrap: Option<bool>,
    /// Text wrap width
    pub wrap_width: Option<u32>,
    /// Treat block elements as inline
    pub convert_as_inline: Option<bool>,
    /// Subscript symbol
    pub sub_symbol: Option<String>,
    /// Superscript symbol
    pub sup_symbol: Option<String>,
    /// Newline style
    pub newline_style: Option<JsNewlineStyle>,
    /// Code block style
    pub code_block_style: Option<JsCodeBlockStyle>,
    /// Elements where images should remain as markdown
    pub keep_inline_images_in: Option<Vec<String>>,
    /// Preprocessing options
    pub preprocessing: Option<JsPreprocessingOptions>,
    /// Source encoding (informational)
    pub encoding: Option<String>,
    /// Enable debug mode with diagnostic warnings
    pub debug: Option<bool>,
    /// List of HTML tags to strip
    pub strip_tags: Option<Vec<String>>,
    /// List of HTML tags to preserve as-is in the output
    pub preserve_tags: Option<Vec<String>>,
    /// Skip image conversion (keep as HTML)
    pub skip_images: Option<bool>,
    /// Link rendering style
    pub link_style: Option<JsLinkStyle>,
    /// Output format for conversion
    pub output_format: Option<JsOutputFormat>,
    /// Include structured document tree in result
    pub include_document_structure: Option<bool>,
    /// Extract inline images from data URIs and SVGs
    pub extract_images: Option<bool>,
    /// Maximum decoded image size in bytes
    pub max_image_size: Option<BigInt>,
    /// Capture SVG elements as images
    pub capture_svg: Option<bool>,
    /// Infer image dimensions from data
    pub infer_dimensions: Option<bool>,
    /// Maximum DOM tree depth to recurse into
    pub max_depth: Option<u32>,
}

impl From<JsConversionOptions> for ConversionOptionsUpdate {
    fn from(val: JsConversionOptions) -> Self {
        Self {
            heading_style: val.heading_style.map(Into::into),
            list_indent_type: val.list_indent_type.map(Into::into),
            list_indent_width: val.list_indent_width.map(|value| value as usize),
            bullets: val.bullets,
            strong_em_symbol: val.strong_em_symbol.and_then(|s| s.chars().next()),
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
            wrap_width: val.wrap_width.map(|value| value as usize),
            convert_as_inline: val.convert_as_inline,
            sub_symbol: val.sub_symbol,
            sup_symbol: val.sup_symbol,
            newline_style: val.newline_style.map(Into::into),
            code_block_style: val.code_block_style.map(Into::into),
            keep_inline_images_in: val.keep_inline_images_in,
            preprocessing: val.preprocessing.map(Into::into),
            encoding: val.encoding,
            debug: val.debug,
            strip_tags: val.strip_tags,
            preserve_tags: val.preserve_tags,
            skip_images: val.skip_images,
            link_style: val.link_style.map(Into::into),
            output_format: val.output_format.map(Into::into),
            include_document_structure: val.include_document_structure,
            extract_images: val.extract_images,
            max_image_size: val.max_image_size.map(|b| {
                let (_, value, _) = b.get_u64();
                value
            }),
            capture_svg: val.capture_svg,
            infer_dimensions: val.infer_dimensions,
            max_depth: val.max_depth.map(|v| v as usize),
        }
    }
}

impl From<JsConversionOptions> for RustConversionOptions {
    fn from(val: JsConversionOptions) -> Self {
        Self::from(ConversionOptionsUpdate::from(val))
    }
}

/// Inline image configuration
#[napi(object)]
pub struct JsInlineImageConfig {
    /// Maximum decoded size in bytes (default: 5MB)
    pub max_decoded_size_bytes: Option<BigInt>,
    /// Filename prefix for generated filenames
    pub filename_prefix: Option<String>,
    /// Capture inline SVG elements (default: true)
    pub capture_svg: Option<bool>,
    /// Infer image dimensions (default: false)
    pub infer_dimensions: Option<bool>,
}

impl From<JsInlineImageConfig> for InlineImageConfigUpdate {
    fn from(val: JsInlineImageConfig) -> Self {
        let max_decoded_size_bytes = val.max_decoded_size_bytes.map(|b| {
            // Use get_u64 but don't rely on the lossless flag for correct sign detection
            // Instead, check the sign_bit directly from the internal structure
            let (_, value, _) = b.get_u64();
            // The BigInt is positive if sign_bit is false, so we use the value directly
            value
        });
        Self {
            max_decoded_size_bytes,
            filename_prefix: val.filename_prefix,
            capture_svg: val.capture_svg,
            infer_dimensions: val.infer_dimensions,
        }
    }
}

impl From<JsInlineImageConfig> for RustInlineImageConfig {
    fn from(val: JsInlineImageConfig) -> Self {
        let mut cfg = Self::new(DEFAULT_INLINE_IMAGE_LIMIT);
        cfg.apply_update(InlineImageConfigUpdate::from(val));
        cfg
    }
}
