#![allow(clippy::cast_precision_loss, clippy::cast_sign_loss, clippy::unused_self)]

//! Main conversion options with builder pattern.

use crate::options::preprocessing::PreprocessingOptions;
use crate::options::validation::{
    CodeBlockStyle, HeadingStyle, HighlightStyle, LinkStyle, ListIndentType, NewlineStyle, OutputFormat, WhitespaceMode,
};

/// Main conversion options for HTML to Markdown conversion.
///
/// Use [`ConversionOptions::builder()`] to construct, or [`Default::default()`] for defaults.
///
/// # Example
///
/// ```text
/// use html_to_markdown_rs::ConversionOptions;
///
/// let options = ConversionOptions::builder()
///     .heading_style(HeadingStyle::Atx)
///     .wrap(true)
///     .wrap_width(100)
///     .build();
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(
    any(feature = "serde", feature = "metadata"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", feature = "metadata"), serde(default, deny_unknown_fields))]
pub struct ConversionOptions {
    /// Heading style to use in Markdown output (ATX `#` or Setext underline).
    pub heading_style: HeadingStyle,
    /// How to indent nested list items (spaces or tab).
    pub list_indent_type: ListIndentType,
    /// Number of spaces (or tabs) to use for each level of list indentation.
    pub list_indent_width: usize,
    /// Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`).
    pub bullets: String,
    /// Character used for bold/italic emphasis markers (`*` or `_`).
    pub strong_em_symbol: char,
    /// Escape `*` characters in plain text to avoid unintended bold/italic.
    pub escape_asterisks: bool,
    /// Escape `_` characters in plain text to avoid unintended bold/italic.
    pub escape_underscores: bool,
    /// Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text.
    pub escape_misc: bool,
    /// Escape ASCII characters that have special meaning in certain Markdown dialects.
    pub escape_ascii: bool,
    /// Default language annotation for fenced code blocks that have no language hint.
    pub code_language: String,
    /// Automatically convert bare URLs into Markdown autolinks.
    pub autolinks: bool,
    /// Emit a default title when no `<title>` tag is present.
    pub default_title: bool,
    /// Render `<br>` elements inside table cells as literal line breaks.
    pub br_in_tables: bool,
    /// Style used for `<mark>` / highlighted text (e.g. `==text==`).
    pub highlight_style: HighlightStyle,
    /// Extract `<meta>` and `<head>` information into the result metadata.
    pub extract_metadata: bool,
    /// Controls how whitespace is normalised during conversion.
    pub whitespace_mode: WhitespaceMode,
    /// Strip all newlines from the output, producing a single-line result.
    pub strip_newlines: bool,
    /// Wrap long lines at [`wrap_width`](Self::wrap_width) characters.
    pub wrap: bool,
    /// Maximum line width when [`wrap`](Self::wrap) is enabled (default `80`).
    pub wrap_width: usize,
    /// Treat the entire document as inline content (no block-level wrappers).
    pub convert_as_inline: bool,
    /// Markdown notation for subscript text (e.g. `"~"`).
    pub sub_symbol: String,
    /// Markdown notation for superscript text (e.g. `"^"`).
    pub sup_symbol: String,
    /// How to encode hard line breaks (`<br>`) in Markdown.
    pub newline_style: NewlineStyle,
    /// Style used for fenced code blocks (backticks or tilde).
    pub code_block_style: CodeBlockStyle,
    /// HTML tag names whose `<img>` children are kept inline instead of block.
    pub keep_inline_images_in: Vec<String>,
    /// Pre-processing options applied to the HTML before conversion.
    pub preprocessing: PreprocessingOptions,
    /// Expected character encoding of the input HTML (default `"utf-8"`).
    pub encoding: String,
    /// Emit debug information during conversion.
    pub debug: bool,
    /// HTML tag names whose content is stripped from the output entirely.
    pub strip_tags: Vec<String>,
    /// HTML tag names that are preserved verbatim in the output.
    pub preserve_tags: Vec<String>,
    /// Skip conversion of `<img>` elements (omit images from output).
    pub skip_images: bool,
    /// Link rendering style (inline or reference).
    pub link_style: LinkStyle,
    /// Target output format (Markdown, plain text, etc.).
    pub output_format: OutputFormat,
    /// Include structured document tree in result.
    pub include_document_structure: bool,
    /// Extract inline images from data URIs and SVGs.
    pub extract_images: bool,
    /// Maximum decoded image size in bytes (default 5MB).
    pub max_image_size: u64,
    /// Capture SVG elements as images.
    pub capture_svg: bool,
    /// Infer image dimensions from data.
    pub infer_dimensions: bool,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            heading_style: HeadingStyle::default(),
            list_indent_type: ListIndentType::default(),
            list_indent_width: 2,
            bullets: "-*+".to_string(),
            strong_em_symbol: '*',
            escape_asterisks: false,
            escape_underscores: false,
            escape_misc: false,
            escape_ascii: false,
            code_language: String::new(),
            autolinks: true,
            default_title: false,
            br_in_tables: false,
            highlight_style: HighlightStyle::default(),
            extract_metadata: true,
            whitespace_mode: WhitespaceMode::default(),
            strip_newlines: false,
            wrap: false,
            wrap_width: 80,
            convert_as_inline: false,
            sub_symbol: String::new(),
            sup_symbol: String::new(),
            newline_style: NewlineStyle::Spaces,
            code_block_style: CodeBlockStyle::default(),
            keep_inline_images_in: Vec::new(),
            preprocessing: PreprocessingOptions::default(),
            encoding: "utf-8".to_string(),
            debug: false,
            strip_tags: Vec::new(),
            preserve_tags: Vec::new(),
            skip_images: false,
            link_style: LinkStyle::default(),
            output_format: OutputFormat::default(),
            include_document_structure: false,
            extract_images: false,
            max_image_size: 5_242_880,
            capture_svg: false,
            infer_dimensions: true,
        }
    }
}

impl ConversionOptions {
    /// Create a new builder with default values.
    #[must_use]
    pub fn builder() -> ConversionOptionsBuilder {
        ConversionOptionsBuilder(Self::default())
    }
}

// ── Builder ─────────────────────────────────────────────────────────────────

/// Builder for [`ConversionOptions`].
///
/// All fields start with default values. Call `.build()` to produce the final options.
#[derive(Debug, Clone)]
pub struct ConversionOptionsBuilder(ConversionOptions);

macro_rules! builder_setter {
    ($name:ident, $ty:ty) => {
        /// Set the value.
        #[must_use]
        pub fn $name(mut self, value: $ty) -> Self {
            self.0.$name = value;
            self
        }
    };
}

macro_rules! builder_setter_into {
    ($name:ident, $ty:ty) => {
        /// Set the value.
        #[must_use]
        pub fn $name(mut self, value: impl Into<$ty>) -> Self {
            self.0.$name = value.into();
            self
        }
    };
}

impl ConversionOptionsBuilder {
    // Output control
    builder_setter!(output_format, OutputFormat);
    builder_setter!(include_document_structure, bool);
    builder_setter!(extract_metadata, bool);
    builder_setter!(extract_images, bool);

    // Markdown formatting
    builder_setter!(heading_style, HeadingStyle);
    builder_setter!(list_indent_type, ListIndentType);
    builder_setter!(list_indent_width, usize);
    builder_setter_into!(bullets, String);
    builder_setter!(strong_em_symbol, char);
    builder_setter!(code_block_style, CodeBlockStyle);
    builder_setter!(newline_style, NewlineStyle);
    builder_setter!(highlight_style, HighlightStyle);
    builder_setter_into!(code_language, String);
    builder_setter!(link_style, LinkStyle);
    builder_setter!(autolinks, bool);
    builder_setter!(default_title, bool);
    builder_setter!(br_in_tables, bool);
    builder_setter_into!(sub_symbol, String);
    builder_setter_into!(sup_symbol, String);

    // Escaping
    builder_setter!(escape_asterisks, bool);
    builder_setter!(escape_underscores, bool);
    builder_setter!(escape_misc, bool);
    builder_setter!(escape_ascii, bool);

    // Whitespace / wrapping
    builder_setter!(whitespace_mode, WhitespaceMode);
    builder_setter!(strip_newlines, bool);
    builder_setter!(wrap, bool);
    builder_setter!(wrap_width, usize);

    // Element handling
    builder_setter!(convert_as_inline, bool);
    builder_setter!(skip_images, bool);

    /// Set the list of HTML tag names whose content is stripped from output.
    #[must_use]
    pub fn strip_tags(mut self, tags: Vec<String>) -> Self {
        self.0.strip_tags = tags;
        self
    }

    /// Set the list of HTML tag names that are preserved verbatim in output.
    #[must_use]
    pub fn preserve_tags(mut self, tags: Vec<String>) -> Self {
        self.0.preserve_tags = tags;
        self
    }

    /// Set the list of HTML tag names whose `<img>` children are kept inline.
    #[must_use]
    pub fn keep_inline_images_in(mut self, tags: Vec<String>) -> Self {
        self.0.keep_inline_images_in = tags;
        self
    }

    // Image extraction config
    builder_setter!(max_image_size, u64);
    builder_setter!(capture_svg, bool);
    builder_setter!(infer_dimensions, bool);

    // Preprocessing
    /// Set the pre-processing options applied to the HTML before conversion.
    #[must_use]
    pub fn preprocessing(mut self, preprocessing: PreprocessingOptions) -> Self {
        self.0.preprocessing = preprocessing;
        self
    }

    // Encoding
    builder_setter_into!(encoding, String);

    // Debug
    builder_setter!(debug, bool);

    /// Build the final [`ConversionOptions`].
    #[must_use]
    pub fn build(self) -> ConversionOptions {
        self.0
    }
}

// ── ConversionOptionsUpdate (for binding crate compatibility) ────────────

use crate::options::preprocessing::PreprocessingOptionsUpdate;

/// Partial update for `ConversionOptions`.
///
/// Uses `Option<T>` fields for selective updates. Bindings use this to construct
/// options from language-native types. Prefer [`ConversionOptionsBuilder`] for Rust code.
#[derive(Debug, Clone, Default)]
#[cfg_attr(
    any(feature = "serde", feature = "metadata"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", feature = "metadata"), serde(deny_unknown_fields))]
pub struct ConversionOptionsUpdate {
    /// Optional override for [`ConversionOptions::heading_style`].
    pub heading_style: Option<HeadingStyle>,
    /// Optional override for [`ConversionOptions::list_indent_type`].
    pub list_indent_type: Option<ListIndentType>,
    /// Optional override for [`ConversionOptions::list_indent_width`].
    pub list_indent_width: Option<usize>,
    /// Optional override for [`ConversionOptions::bullets`].
    pub bullets: Option<String>,
    /// Optional override for [`ConversionOptions::strong_em_symbol`].
    pub strong_em_symbol: Option<char>,
    /// Optional override for [`ConversionOptions::escape_asterisks`].
    pub escape_asterisks: Option<bool>,
    /// Optional override for [`ConversionOptions::escape_underscores`].
    pub escape_underscores: Option<bool>,
    /// Optional override for [`ConversionOptions::escape_misc`].
    pub escape_misc: Option<bool>,
    /// Optional override for [`ConversionOptions::escape_ascii`].
    pub escape_ascii: Option<bool>,
    /// Optional override for [`ConversionOptions::code_language`].
    pub code_language: Option<String>,
    /// Optional override for [`ConversionOptions::autolinks`].
    pub autolinks: Option<bool>,
    /// Optional override for [`ConversionOptions::default_title`].
    pub default_title: Option<bool>,
    /// Optional override for [`ConversionOptions::br_in_tables`].
    pub br_in_tables: Option<bool>,
    /// Optional override for [`ConversionOptions::highlight_style`].
    pub highlight_style: Option<HighlightStyle>,
    /// Optional override for [`ConversionOptions::extract_metadata`].
    pub extract_metadata: Option<bool>,
    /// Optional override for [`ConversionOptions::whitespace_mode`].
    pub whitespace_mode: Option<WhitespaceMode>,
    /// Optional override for [`ConversionOptions::strip_newlines`].
    pub strip_newlines: Option<bool>,
    /// Optional override for [`ConversionOptions::wrap`].
    pub wrap: Option<bool>,
    /// Optional override for [`ConversionOptions::wrap_width`].
    pub wrap_width: Option<usize>,
    /// Optional override for [`ConversionOptions::convert_as_inline`].
    pub convert_as_inline: Option<bool>,
    /// Optional override for [`ConversionOptions::sub_symbol`].
    pub sub_symbol: Option<String>,
    /// Optional override for [`ConversionOptions::sup_symbol`].
    pub sup_symbol: Option<String>,
    /// Optional override for [`ConversionOptions::newline_style`].
    pub newline_style: Option<NewlineStyle>,
    /// Optional override for [`ConversionOptions::code_block_style`].
    pub code_block_style: Option<CodeBlockStyle>,
    /// Optional override for [`ConversionOptions::keep_inline_images_in`].
    pub keep_inline_images_in: Option<Vec<String>>,
    /// Optional override for [`ConversionOptions::preprocessing`].
    pub preprocessing: Option<PreprocessingOptionsUpdate>,
    /// Optional override for [`ConversionOptions::encoding`].
    pub encoding: Option<String>,
    /// Optional override for [`ConversionOptions::debug`].
    pub debug: Option<bool>,
    /// Optional override for [`ConversionOptions::strip_tags`].
    pub strip_tags: Option<Vec<String>>,
    /// Optional override for [`ConversionOptions::preserve_tags`].
    pub preserve_tags: Option<Vec<String>>,
    /// Optional override for [`ConversionOptions::skip_images`].
    pub skip_images: Option<bool>,
    /// Optional override for [`ConversionOptions::link_style`].
    pub link_style: Option<LinkStyle>,
    /// Optional override for [`ConversionOptions::output_format`].
    pub output_format: Option<OutputFormat>,
    /// Optional override for [`ConversionOptions::include_document_structure`].
    pub include_document_structure: Option<bool>,
    /// Optional override for [`ConversionOptions::extract_images`].
    pub extract_images: Option<bool>,
    /// Optional override for [`ConversionOptions::max_image_size`].
    pub max_image_size: Option<u64>,
    /// Optional override for [`ConversionOptions::capture_svg`].
    pub capture_svg: Option<bool>,
    /// Optional override for [`ConversionOptions::infer_dimensions`].
    pub infer_dimensions: Option<bool>,
}

impl ConversionOptions {
    /// Apply a partial update to these conversion options.
    pub fn apply_update(&mut self, update: ConversionOptionsUpdate) {
        macro_rules! apply {
            ($field:ident) => {
                if let Some(v) = update.$field {
                    self.$field = v;
                }
            };
        }
        apply!(heading_style);
        apply!(list_indent_type);
        apply!(list_indent_width);
        apply!(bullets);
        apply!(strong_em_symbol);
        apply!(escape_asterisks);
        apply!(escape_underscores);
        apply!(escape_misc);
        apply!(escape_ascii);
        apply!(code_language);
        apply!(autolinks);
        apply!(default_title);
        apply!(br_in_tables);
        apply!(highlight_style);
        apply!(extract_metadata);
        apply!(whitespace_mode);
        apply!(strip_newlines);
        apply!(wrap);
        apply!(wrap_width);
        apply!(convert_as_inline);
        apply!(sub_symbol);
        apply!(sup_symbol);
        apply!(newline_style);
        apply!(code_block_style);
        apply!(keep_inline_images_in);
        apply!(encoding);
        apply!(debug);
        apply!(strip_tags);
        apply!(preserve_tags);
        apply!(skip_images);
        apply!(link_style);
        apply!(output_format);
        apply!(include_document_structure);
        apply!(extract_images);
        apply!(max_image_size);
        apply!(capture_svg);
        apply!(infer_dimensions);
        if let Some(preprocessing) = update.preprocessing {
            self.preprocessing.apply_update(preprocessing);
        }
    }

    /// Create from a partial update, applying to defaults.
    #[must_use]
    pub fn from_update(update: ConversionOptionsUpdate) -> Self {
        let mut options = Self::default();
        options.apply_update(update);
        options
    }
}

impl From<ConversionOptionsUpdate> for ConversionOptions {
    fn from(update: ConversionOptionsUpdate) -> Self {
        Self::from_update(update)
    }
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(all(test, any(feature = "serde", feature = "metadata")))]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_options_serde() {
        let options = ConversionOptions::builder()
            .heading_style(HeadingStyle::AtxClosed)
            .list_indent_width(4)
            .bullets("*")
            .escape_asterisks(true)
            .whitespace_mode(WhitespaceMode::Strict)
            .build();

        let json = serde_json::to_string(&options).expect("Failed to serialize");
        let deserialized: ConversionOptions = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(deserialized.list_indent_width, 4);
        assert_eq!(deserialized.bullets, "*");
        assert!(deserialized.escape_asterisks);
        assert_eq!(deserialized.heading_style, HeadingStyle::AtxClosed);
        assert_eq!(deserialized.whitespace_mode, WhitespaceMode::Strict);
    }

    #[test]
    fn test_conversion_options_partial_deserialization() {
        let partial_json = r#"{
            "heading_style": "atxclosed",
            "list_indent_width": 4,
            "bullets": "*"
        }"#;

        let deserialized: ConversionOptions =
            serde_json::from_str(partial_json).expect("Failed to deserialize partial JSON");

        assert_eq!(deserialized.heading_style, HeadingStyle::AtxClosed);
        assert_eq!(deserialized.list_indent_width, 4);
        assert_eq!(deserialized.bullets, "*");
        assert!(!deserialized.escape_asterisks);
        assert!(!deserialized.escape_underscores);
        assert_eq!(deserialized.list_indent_type, ListIndentType::Spaces);
    }

    #[test]
    fn test_builder_pattern() {
        let options = ConversionOptions::builder()
            .heading_style(HeadingStyle::Underlined)
            .wrap(true)
            .wrap_width(100)
            .include_document_structure(true)
            .extract_images(true)
            .build();

        assert_eq!(options.heading_style, HeadingStyle::Underlined);
        assert!(options.wrap);
        assert_eq!(options.wrap_width, 100);
        assert!(options.include_document_structure);
        assert!(options.extract_images);
    }
}
