#![allow(dead_code)]

use ext_php_rs::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\MetadataConfig")]
pub struct MetadataConfig {
    /// Extract document-level metadata (title, description, author, etc.).
    ///
    /// When enabled, collects metadata from `<head>` section including:
    /// - `<title>` element content
    /// - `<meta name="description">` and other standard meta tags
    /// - Open Graph (og:*) properties for social media optimization
    /// - Twitter Card (twitter:*) properties
    /// - Language and text direction attributes
    /// - Canonical URL and base href references
    #[php(prop, name = "extract_document")]
    pub extract_document: bool,
    /// Extract h1-h6 header elements and their hierarchy.
    ///
    /// When enabled, collects all heading elements with:
    /// - Header level (1-6)
    /// - Text content (normalized)
    /// - HTML id attribute if present
    /// - Document tree depth for hierarchy tracking
    /// - Byte offset in original HTML for positioning
    #[php(prop, name = "extract_headers")]
    pub extract_headers: bool,
    /// Extract anchor (a) elements as links with type classification.
    ///
    /// When enabled, collects all hyperlinks with:
    /// - href attribute value
    /// - Link text content
    /// - Title attribute (tooltip text)
    /// - Automatic link type classification (anchor, internal, external, email, phone, other)
    /// - Rel attribute values
    /// - Additional custom attributes
    #[php(prop, name = "extract_links")]
    pub extract_links: bool,
    /// Extract image elements and data URIs.
    ///
    /// When enabled, collects all image elements with:
    /// - Source URL or data URI
    /// - Alt text for accessibility
    /// - Title attribute
    /// - Dimensions (width, height) if available
    /// - Automatic image type classification (data URI, external, relative, inline SVG)
    /// - Additional custom attributes
    #[php(prop, name = "extract_images")]
    pub extract_images: bool,
    /// Extract structured data (JSON-LD, Microdata, RDFa).
    ///
    /// When enabled, collects machine-readable structured data including:
    /// - JSON-LD script blocks with schema detection
    /// - Microdata attributes (itemscope, itemtype, itemprop)
    /// - RDFa markup
    /// - Extracted schema type if detectable
    #[php(prop, name = "extract_structured_data")]
    pub extract_structured_data: bool,
    /// Maximum total size of structured data to collect (bytes).
    ///
    /// Prevents memory exhaustion attacks on malformed or adversarial documents
    /// containing excessively large structured data blocks. When the accumulated
    /// size of structured data exceeds this limit, further collection stops.
    /// Default: `1_000_000` bytes (1 MB)
    #[php(prop, name = "max_structured_data_size")]
    pub max_structured_data_size: i64,
}

#[php_impl]
impl MetadataConfig {
    pub fn __construct(
        extract_document: Option<bool>,
        extract_headers: Option<bool>,
        extract_links: Option<bool>,
        extract_images: Option<bool>,
        extract_structured_data: Option<bool>,
        max_structured_data_size: Option<i64>,
    ) -> Self {
        Self {
            extract_document: extract_document.unwrap_or(true),
            extract_headers: extract_headers.unwrap_or(true),
            extract_links: extract_links.unwrap_or(true),
            extract_images: extract_images.unwrap_or(true),
            extract_structured_data: extract_structured_data.unwrap_or(true),
            max_structured_data_size: max_structured_data_size.unwrap_or_default(),
        }
    }

    pub fn any_enabled(&self) -> bool {
        let core_self = html_to_markdown_rs::metadata::MetadataConfig {
            extract_document: self.extract_document,
            extract_headers: self.extract_headers,
            extract_links: self.extract_links,
            extract_images: self.extract_images,
            extract_structured_data: self.extract_structured_data,
            max_structured_data_size: self.max_structured_data_size as usize,
        };
        core_self.any_enabled()
    }

    pub fn apply_update(&self, _update: &MetadataConfigUpdate) {
        ()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> MetadataConfig {
        html_to_markdown_rs::metadata::MetadataConfig::default().into()
    }

    pub fn from_update(_update: &MetadataConfigUpdate) -> MetadataConfig {
        panic!("alef: from_update not auto-delegatable")
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from(_update: &MetadataConfigUpdate) -> MetadataConfig {
        panic!("alef: from not auto-delegatable")
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\MetadataConfigUpdate")]
pub struct MetadataConfigUpdate {
    /// Optional override for extracting document-level metadata.
    ///
    /// When Some(true), enables document metadata extraction; Some(false) disables it.
    /// None leaves the current setting unchanged.
    #[php(prop, name = "extract_document")]
    pub extract_document: Option<bool>,
    /// Optional override for extracting heading elements (h1-h6).
    ///
    /// When Some(true), enables header extraction; Some(false) disables it.
    /// None leaves the current setting unchanged.
    #[php(prop, name = "extract_headers")]
    pub extract_headers: Option<bool>,
    /// Optional override for extracting anchor (link) elements.
    ///
    /// When Some(true), enables link extraction; Some(false) disables it.
    /// None leaves the current setting unchanged.
    #[php(prop, name = "extract_links")]
    pub extract_links: Option<bool>,
    /// Optional override for extracting image elements.
    ///
    /// When Some(true), enables image extraction; Some(false) disables it.
    /// None leaves the current setting unchanged.
    #[php(prop, name = "extract_images")]
    pub extract_images: Option<bool>,
    /// Optional override for extracting structured data (JSON-LD, Microdata, RDFa).
    ///
    /// When Some(true), enables structured data extraction; Some(false) disables it.
    /// None leaves the current setting unchanged.
    #[php(prop, name = "extract_structured_data")]
    pub extract_structured_data: Option<bool>,
    /// Optional override for maximum structured data collection size in bytes.
    ///
    /// When Some(size), sets the new size limit. None leaves the current limit unchanged.
    /// Use this to adjust safety thresholds for different documents.
    #[php(prop, name = "max_structured_data_size")]
    pub max_structured_data_size: Option<i64>,
}

#[php_impl]
impl MetadataConfigUpdate {
    pub fn __construct(
        extract_document: Option<bool>,
        extract_headers: Option<bool>,
        extract_links: Option<bool>,
        extract_images: Option<bool>,
        extract_structured_data: Option<bool>,
        max_structured_data_size: Option<i64>,
    ) -> Self {
        Self {
            extract_document,
            extract_headers,
            extract_links,
            extract_images,
            extract_structured_data,
            max_structured_data_size,
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\DocumentMetadata")]
pub struct DocumentMetadata {
    /// Document title from `<title>` tag
    #[php(prop, name = "title")]
    pub title: Option<String>,
    /// Document description from `<meta name="description">` tag
    #[php(prop, name = "description")]
    pub description: Option<String>,
    /// Document keywords from `<meta name="keywords">` tag, split on commas
    #[php(prop, name = "keywords")]
    pub keywords: Vec<String>,
    /// Document author from `<meta name="author">` tag
    #[php(prop, name = "author")]
    pub author: Option<String>,
    /// Canonical URL from `<link rel="canonical">` tag
    #[php(prop, name = "canonical_url")]
    pub canonical_url: Option<String>,
    /// Base URL from `<base href="">` tag for resolving relative URLs
    #[php(prop, name = "base_href")]
    pub base_href: Option<String>,
    /// Document language from `lang` attribute
    #[php(prop, name = "language")]
    pub language: Option<String>,
    /// Document text direction from `dir` attribute
    #[php(prop, name = "text_direction")]
    pub text_direction: Option<String>,
    /// Open Graph metadata (og:* properties) for social media
    /// Keys like "title", "description", "image", "url", etc.
    pub open_graph: HashMap<String, String>,
    /// Twitter Card metadata (twitter:* properties)
    /// Keys like "card", "site", "creator", "title", "description", "image", etc.
    pub twitter_card: HashMap<String, String>,
    /// Additional meta tags not covered by specific fields
    /// Keys are meta name/property attributes, values are content
    pub meta_tags: HashMap<String, String>,
}

#[php_impl]
impl DocumentMetadata {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_open_graph(&self) -> HashMap<String, String> {
        self.open_graph.clone()
    }

    #[php(getter)]
    pub fn get_twitter_card(&self) -> HashMap<String, String> {
        self.twitter_card.clone()
    }

    #[php(getter)]
    pub fn get_meta_tags(&self) -> HashMap<String, String> {
        self.meta_tags.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\HeaderMetadata")]
pub struct HeaderMetadata {
    /// Header level: 1 (h1) through 6 (h6)
    #[php(prop, name = "level")]
    pub level: u8,
    /// Normalized text content of the header
    #[php(prop, name = "text")]
    pub text: String,
    /// HTML id attribute if present
    #[php(prop, name = "id")]
    pub id: Option<String>,
    /// Document tree depth at the header element
    #[php(prop, name = "depth")]
    pub depth: i64,
    /// Byte offset in original HTML document
    #[php(prop, name = "html_offset")]
    pub html_offset: i64,
}

#[php_impl]
impl HeaderMetadata {
    pub fn __construct(level: u8, text: String, depth: i64, html_offset: i64, id: Option<String>) -> Self {
        Self {
            level,
            text,
            id,
            depth,
            html_offset,
        }
    }

    pub fn is_valid(&self) -> bool {
        let core_self = html_to_markdown_rs::metadata::HeaderMetadata {
            level: self.level,
            text: self.text.clone(),
            id: self.id.clone(),
            depth: self.depth as usize,
            html_offset: self.html_offset as usize,
        };
        core_self.is_valid()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\LinkMetadata")]
pub struct LinkMetadata {
    /// The href URL value
    #[php(prop, name = "href")]
    pub href: String,
    /// Link text content (normalized, concatenated if mixed with elements)
    #[php(prop, name = "text")]
    pub text: String,
    /// Optional title attribute (often shown as tooltip)
    #[php(prop, name = "title")]
    pub title: Option<String>,
    /// Link type classification
    #[php(prop, name = "link_type")]
    pub link_type: String,
    /// Rel attribute values (e.g., "nofollow", "stylesheet", "canonical")
    #[php(prop, name = "rel")]
    pub rel: Vec<String>,
    /// Additional HTML attributes
    pub attributes: HashMap<String, String>,
}

#[php_impl]
impl LinkMetadata {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_attributes(&self) -> HashMap<String, String> {
        self.attributes.clone()
    }

    pub fn classify_link(href: String) -> String {
        format!(
            "{:?}",
            html_to_markdown_rs::metadata::LinkMetadata::classify_link(&href)
        )
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\ImageMetadata")]
pub struct ImageMetadata {
    /// Image source (URL, data URI, or SVG content identifier)
    #[php(prop, name = "src")]
    pub src: String,
    /// Alternative text from alt attribute (for accessibility)
    #[php(prop, name = "alt")]
    pub alt: Option<String>,
    /// Title attribute (often shown as tooltip)
    #[php(prop, name = "title")]
    pub title: Option<String>,
    /// Image dimensions as (width, height) if available
    #[php(prop, name = "dimensions")]
    pub dimensions: Option<String>,
    /// Image type classification
    #[php(prop, name = "image_type")]
    pub image_type: String,
    /// Additional HTML attributes
    pub attributes: HashMap<String, String>,
}

#[php_impl]
impl ImageMetadata {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_attributes(&self) -> HashMap<String, String> {
        self.attributes.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\StructuredData")]
pub struct StructuredData {
    /// Type of structured data (JSON-LD, Microdata, RDFa)
    #[php(prop, name = "data_type")]
    pub data_type: String,
    /// Raw JSON string (for JSON-LD) or serialized representation
    #[php(prop, name = "raw_json")]
    pub raw_json: String,
    /// Schema type if detectable (e.g., "Article", "Event", "Product")
    #[php(prop, name = "schema_type")]
    pub schema_type: Option<String>,
}

#[php_impl]
impl StructuredData {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\HtmlMetadata")]
pub struct HtmlMetadata {
    /// Document-level metadata (title, description, canonical, etc.)
    pub document: DocumentMetadata,
    /// Extracted header elements with hierarchy
    pub headers: Vec<HeaderMetadata>,
    /// Extracted hyperlinks with type classification
    pub links: Vec<LinkMetadata>,
    /// Extracted images with source and dimensions
    pub images: Vec<ImageMetadata>,
    /// Extracted structured data blocks
    pub structured_data: Vec<StructuredData>,
}

#[php_impl]
impl HtmlMetadata {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_document(&self) -> DocumentMetadata {
        self.document.clone()
    }

    #[php(getter)]
    pub fn get_headers(&self) -> Vec<HeaderMetadata> {
        self.headers.clone()
    }

    #[php(getter)]
    pub fn get_links(&self) -> Vec<LinkMetadata> {
        self.links.clone()
    }

    #[php(getter)]
    pub fn get_images(&self) -> Vec<ImageMetadata> {
        self.images.clone()
    }

    #[php(getter)]
    pub fn get_structured_data(&self) -> Vec<StructuredData> {
        self.structured_data.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\ConversionOptions")]
#[allow(clippy::similar_names)]
pub struct ConversionOptions {
    /// Heading style to use in Markdown output (ATX `#` or Setext underline).
    #[php(prop, name = "heading_style")]
    pub heading_style: String,
    /// How to indent nested list items (spaces or tab).
    #[php(prop, name = "list_indent_type")]
    pub list_indent_type: String,
    /// Number of spaces (or tabs) to use for each level of list indentation.
    #[php(prop, name = "list_indent_width")]
    pub list_indent_width: i64,
    /// Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`).
    #[php(prop, name = "bullets")]
    pub bullets: String,
    /// Character used for bold/italic emphasis markers (`*` or `_`).
    #[php(prop, name = "strong_em_symbol")]
    pub strong_em_symbol: String,
    /// Escape `*` characters in plain text to avoid unintended bold/italic.
    #[php(prop, name = "escape_asterisks")]
    pub escape_asterisks: bool,
    /// Escape `_` characters in plain text to avoid unintended bold/italic.
    #[php(prop, name = "escape_underscores")]
    pub escape_underscores: bool,
    /// Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text.
    #[php(prop, name = "escape_misc")]
    pub escape_misc: bool,
    /// Escape ASCII characters that have special meaning in certain Markdown dialects.
    #[php(prop, name = "escape_ascii")]
    pub escape_ascii: bool,
    /// Default language annotation for fenced code blocks that have no language hint.
    #[php(prop, name = "code_language")]
    pub code_language: String,
    /// Automatically convert bare URLs into Markdown autolinks.
    #[php(prop, name = "autolinks")]
    pub autolinks: bool,
    /// Emit a default title when no `<title>` tag is present.
    #[php(prop, name = "default_title")]
    pub default_title: bool,
    /// Render `<br>` elements inside table cells as literal line breaks.
    #[php(prop, name = "br_in_tables")]
    pub br_in_tables: bool,
    /// Style used for `<mark>` / highlighted text (e.g. `==text==`).
    #[php(prop, name = "highlight_style")]
    pub highlight_style: String,
    /// Extract `<meta>` and `<head>` information into the result metadata.
    #[php(prop, name = "extract_metadata")]
    pub extract_metadata: bool,
    /// Controls how whitespace is normalised during conversion.
    #[php(prop, name = "whitespace_mode")]
    pub whitespace_mode: String,
    /// Strip all newlines from the output, producing a single-line result.
    #[php(prop, name = "strip_newlines")]
    pub strip_newlines: bool,
    /// Wrap long lines at [`wrap_width`](Self::wrap_width) characters.
    #[php(prop, name = "wrap")]
    pub wrap: bool,
    /// Maximum line width when [`wrap`](Self::wrap) is enabled (default `80`).
    #[php(prop, name = "wrap_width")]
    pub wrap_width: i64,
    /// Treat the entire document as inline content (no block-level wrappers).
    #[php(prop, name = "convert_as_inline")]
    pub convert_as_inline: bool,
    /// Markdown notation for subscript text (e.g. `"~"`).
    #[php(prop, name = "sub_symbol")]
    pub sub_symbol: String,
    /// Markdown notation for superscript text (e.g. `"^"`).
    #[php(prop, name = "sup_symbol")]
    pub sup_symbol: String,
    /// How to encode hard line breaks (`<br>`) in Markdown.
    #[php(prop, name = "newline_style")]
    pub newline_style: String,
    /// Style used for fenced code blocks (backticks or tilde).
    #[php(prop, name = "code_block_style")]
    pub code_block_style: String,
    /// HTML tag names whose `<img>` children are kept inline instead of block.
    #[php(prop, name = "keep_inline_images_in")]
    pub keep_inline_images_in: Vec<String>,
    /// Pre-processing options applied to the HTML before conversion.
    pub preprocessing: PreprocessingOptions,
    /// Expected character encoding of the input HTML (default `"utf-8"`).
    #[php(prop, name = "encoding")]
    pub encoding: String,
    /// Emit debug information during conversion.
    #[php(prop, name = "debug")]
    pub debug: bool,
    /// HTML tag names whose content is stripped from the output entirely.
    #[php(prop, name = "strip_tags")]
    pub strip_tags: Vec<String>,
    /// HTML tag names that are preserved verbatim in the output.
    #[php(prop, name = "preserve_tags")]
    pub preserve_tags: Vec<String>,
    /// Skip conversion of `<img>` elements (omit images from output).
    #[php(prop, name = "skip_images")]
    pub skip_images: bool,
    /// Link rendering style (inline or reference).
    #[php(prop, name = "link_style")]
    pub link_style: String,
    /// Target output format (Markdown, plain text, etc.).
    #[php(prop, name = "output_format")]
    pub output_format: String,
    /// Include structured document tree in result.
    #[php(prop, name = "include_document_structure")]
    pub include_document_structure: bool,
    /// Extract inline images from data URIs and SVGs.
    #[php(prop, name = "extract_images")]
    pub extract_images: bool,
    /// Maximum decoded image size in bytes (default 5MB).
    #[php(prop, name = "max_image_size")]
    pub max_image_size: i64,
    /// Capture SVG elements as images.
    #[php(prop, name = "capture_svg")]
    pub capture_svg: bool,
    /// Infer image dimensions from data.
    #[php(prop, name = "infer_dimensions")]
    pub infer_dimensions: bool,
    /// Maximum DOM traversal depth. `None` means unlimited.
    /// When set, subtrees beyond this depth are silently truncated.
    #[php(prop, name = "max_depth")]
    pub max_depth: Option<i64>,
}

#[php_impl]
impl ConversionOptions {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_preprocessing(&self) -> PreprocessingOptions {
        self.preprocessing.clone()
    }

    pub fn apply_update(&self, _update: &ConversionOptionsUpdate) {
        ()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> ConversionOptions {
        html_to_markdown_rs::options::ConversionOptions::default().into()
    }

    pub fn builder() -> ConversionOptionsBuilder {
        ConversionOptionsBuilder {
            inner: Arc::new(html_to_markdown_rs::options::ConversionOptions::builder()),
        }
    }

    pub fn from_update(_update: &ConversionOptionsUpdate) -> ConversionOptions {
        panic!("alef: from_update not auto-delegatable")
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from(_update: &ConversionOptionsUpdate) -> ConversionOptions {
        panic!("alef: from not auto-delegatable")
    }
}

#[derive(Clone)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\ConversionOptionsBuilder")]
pub struct ConversionOptionsBuilder {
    inner: Arc<html_to_markdown_rs::options::ConversionOptionsBuilder>,
}

#[php_impl]
impl ConversionOptionsBuilder {
    pub fn strip_tags(&self, tags: Vec<String>) -> ConversionOptionsBuilder {
        Self {
            inner: Arc::new((*self.inner).clone().strip_tags(tags)),
        }
    }

    pub fn preserve_tags(&self, tags: Vec<String>) -> ConversionOptionsBuilder {
        Self {
            inner: Arc::new((*self.inner).clone().preserve_tags(tags)),
        }
    }

    pub fn keep_inline_images_in(&self, tags: Vec<String>) -> ConversionOptionsBuilder {
        Self {
            inner: Arc::new((*self.inner).clone().keep_inline_images_in(tags)),
        }
    }

    pub fn preprocessing(&self, preprocessing: &PreprocessingOptions) -> ConversionOptionsBuilder {
        Self {
            inner: Arc::new((*self.inner).clone().preprocessing(preprocessing.clone().into())),
        }
    }

    pub fn build(&self) -> ConversionOptions {
        (*self.inner).clone().build().into()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\ConversionOptionsUpdate")]
#[allow(clippy::similar_names)]
pub struct ConversionOptionsUpdate {
    /// Optional override for [`ConversionOptions::heading_style`].
    #[php(prop, name = "heading_style")]
    pub heading_style: Option<String>,
    /// Optional override for [`ConversionOptions::list_indent_type`].
    #[php(prop, name = "list_indent_type")]
    pub list_indent_type: Option<String>,
    /// Optional override for [`ConversionOptions::list_indent_width`].
    #[php(prop, name = "list_indent_width")]
    pub list_indent_width: Option<i64>,
    /// Optional override for [`ConversionOptions::bullets`].
    #[php(prop, name = "bullets")]
    pub bullets: Option<String>,
    /// Optional override for [`ConversionOptions::strong_em_symbol`].
    #[php(prop, name = "strong_em_symbol")]
    pub strong_em_symbol: Option<String>,
    /// Optional override for [`ConversionOptions::escape_asterisks`].
    #[php(prop, name = "escape_asterisks")]
    pub escape_asterisks: Option<bool>,
    /// Optional override for [`ConversionOptions::escape_underscores`].
    #[php(prop, name = "escape_underscores")]
    pub escape_underscores: Option<bool>,
    /// Optional override for [`ConversionOptions::escape_misc`].
    #[php(prop, name = "escape_misc")]
    pub escape_misc: Option<bool>,
    /// Optional override for [`ConversionOptions::escape_ascii`].
    #[php(prop, name = "escape_ascii")]
    pub escape_ascii: Option<bool>,
    /// Optional override for [`ConversionOptions::code_language`].
    #[php(prop, name = "code_language")]
    pub code_language: Option<String>,
    /// Optional override for [`ConversionOptions::autolinks`].
    #[php(prop, name = "autolinks")]
    pub autolinks: Option<bool>,
    /// Optional override for [`ConversionOptions::default_title`].
    #[php(prop, name = "default_title")]
    pub default_title: Option<bool>,
    /// Optional override for [`ConversionOptions::br_in_tables`].
    #[php(prop, name = "br_in_tables")]
    pub br_in_tables: Option<bool>,
    /// Optional override for [`ConversionOptions::highlight_style`].
    #[php(prop, name = "highlight_style")]
    pub highlight_style: Option<String>,
    /// Optional override for [`ConversionOptions::extract_metadata`].
    #[php(prop, name = "extract_metadata")]
    pub extract_metadata: Option<bool>,
    /// Optional override for [`ConversionOptions::whitespace_mode`].
    #[php(prop, name = "whitespace_mode")]
    pub whitespace_mode: Option<String>,
    /// Optional override for [`ConversionOptions::strip_newlines`].
    #[php(prop, name = "strip_newlines")]
    pub strip_newlines: Option<bool>,
    /// Optional override for [`ConversionOptions::wrap`].
    #[php(prop, name = "wrap")]
    pub wrap: Option<bool>,
    /// Optional override for [`ConversionOptions::wrap_width`].
    #[php(prop, name = "wrap_width")]
    pub wrap_width: Option<i64>,
    /// Optional override for [`ConversionOptions::convert_as_inline`].
    #[php(prop, name = "convert_as_inline")]
    pub convert_as_inline: Option<bool>,
    /// Optional override for [`ConversionOptions::sub_symbol`].
    #[php(prop, name = "sub_symbol")]
    pub sub_symbol: Option<String>,
    /// Optional override for [`ConversionOptions::sup_symbol`].
    #[php(prop, name = "sup_symbol")]
    pub sup_symbol: Option<String>,
    /// Optional override for [`ConversionOptions::newline_style`].
    #[php(prop, name = "newline_style")]
    pub newline_style: Option<String>,
    /// Optional override for [`ConversionOptions::code_block_style`].
    #[php(prop, name = "code_block_style")]
    pub code_block_style: Option<String>,
    /// Optional override for [`ConversionOptions::keep_inline_images_in`].
    #[php(prop, name = "keep_inline_images_in")]
    pub keep_inline_images_in: Option<Vec<String>>,
    /// Optional override for [`ConversionOptions::preprocessing`].
    pub preprocessing: Option<PreprocessingOptionsUpdate>,
    /// Optional override for [`ConversionOptions::encoding`].
    #[php(prop, name = "encoding")]
    pub encoding: Option<String>,
    /// Optional override for [`ConversionOptions::debug`].
    #[php(prop, name = "debug")]
    pub debug: Option<bool>,
    /// Optional override for [`ConversionOptions::strip_tags`].
    #[php(prop, name = "strip_tags")]
    pub strip_tags: Option<Vec<String>>,
    /// Optional override for [`ConversionOptions::preserve_tags`].
    #[php(prop, name = "preserve_tags")]
    pub preserve_tags: Option<Vec<String>>,
    /// Optional override for [`ConversionOptions::skip_images`].
    #[php(prop, name = "skip_images")]
    pub skip_images: Option<bool>,
    /// Optional override for [`ConversionOptions::link_style`].
    #[php(prop, name = "link_style")]
    pub link_style: Option<String>,
    /// Optional override for [`ConversionOptions::output_format`].
    #[php(prop, name = "output_format")]
    pub output_format: Option<String>,
    /// Optional override for [`ConversionOptions::include_document_structure`].
    #[php(prop, name = "include_document_structure")]
    pub include_document_structure: Option<bool>,
    /// Optional override for [`ConversionOptions::extract_images`].
    #[php(prop, name = "extract_images")]
    pub extract_images: Option<bool>,
    /// Optional override for [`ConversionOptions::max_image_size`].
    #[php(prop, name = "max_image_size")]
    pub max_image_size: Option<i64>,
    /// Optional override for [`ConversionOptions::capture_svg`].
    #[php(prop, name = "capture_svg")]
    pub capture_svg: Option<bool>,
    /// Optional override for [`ConversionOptions::infer_dimensions`].
    #[php(prop, name = "infer_dimensions")]
    pub infer_dimensions: Option<bool>,
    /// Optional override for [`ConversionOptions::max_depth`].
    #[php(prop, name = "max_depth")]
    pub max_depth: Option<i64>,
}

#[php_impl]
impl ConversionOptionsUpdate {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_preprocessing(&self) -> Option<PreprocessingOptionsUpdate> {
        self.preprocessing.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\PreprocessingOptions")]
pub struct PreprocessingOptions {
    /// Enable HTML preprocessing globally
    #[php(prop, name = "enabled")]
    pub enabled: bool,
    /// Preprocessing preset level (Minimal, Standard, Aggressive)
    #[php(prop, name = "preset")]
    pub preset: String,
    /// Remove navigation elements (nav, breadcrumbs, menus, sidebars)
    #[php(prop, name = "remove_navigation")]
    pub remove_navigation: bool,
    /// Remove form elements (forms, inputs, buttons, etc.)
    #[php(prop, name = "remove_forms")]
    pub remove_forms: bool,
}

#[php_impl]
impl PreprocessingOptions {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    pub fn apply_update(&self, _update: &PreprocessingOptionsUpdate) {
        ()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> PreprocessingOptions {
        html_to_markdown_rs::options::PreprocessingOptions::default().into()
    }

    pub fn from_update(_update: &PreprocessingOptionsUpdate) -> PreprocessingOptions {
        panic!("alef: from_update not auto-delegatable")
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from(_update: &PreprocessingOptionsUpdate) -> PreprocessingOptions {
        panic!("alef: from not auto-delegatable")
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\PreprocessingOptionsUpdate")]
pub struct PreprocessingOptionsUpdate {
    /// Optional global preprocessing enablement override
    #[php(prop, name = "enabled")]
    pub enabled: Option<bool>,
    /// Optional preprocessing preset level override (Minimal, Standard, Aggressive)
    #[php(prop, name = "preset")]
    pub preset: Option<String>,
    /// Optional navigation element removal override (nav, breadcrumbs, menus, sidebars)
    #[php(prop, name = "remove_navigation")]
    pub remove_navigation: Option<bool>,
    /// Optional form element removal override (forms, inputs, buttons, etc.)
    #[php(prop, name = "remove_forms")]
    pub remove_forms: Option<bool>,
}

#[php_impl]
impl PreprocessingOptionsUpdate {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\DocumentStructure")]
pub struct DocumentStructure {
    /// All nodes in document reading order.
    pub nodes: Vec<DocumentNode>,
    /// The source format (always "html" for this crate).
    #[php(prop, name = "source_format")]
    pub source_format: Option<String>,
}

#[php_impl]
impl DocumentStructure {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_nodes(&self) -> Vec<DocumentNode> {
        self.nodes.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\DocumentNode")]
pub struct DocumentNode {
    /// Deterministic node identifier.
    #[php(prop, name = "id")]
    pub id: String,
    /// The semantic content of this node.
    #[php(prop, name = "content")]
    pub content: String,
    /// Index of the parent node (None for root nodes).
    #[php(prop, name = "parent")]
    pub parent: Option<u32>,
    /// Indices of child nodes in reading order.
    #[php(prop, name = "children")]
    pub children: Vec<u32>,
    /// Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text.
    pub annotations: Vec<TextAnnotation>,
    /// Format-specific attributes (e.g. class, id, data-* attributes).
    pub attributes: Option<HashMap<String, String>>,
}

#[php_impl]
impl DocumentNode {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_annotations(&self) -> Vec<TextAnnotation> {
        self.annotations.clone()
    }

    #[php(getter)]
    pub fn get_attributes(&self) -> Option<HashMap<String, String>> {
        self.attributes.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\TextAnnotation")]
pub struct TextAnnotation {
    /// Start byte offset (inclusive) into the parent node's text.
    #[php(prop, name = "start")]
    pub start: u32,
    /// End byte offset (exclusive) into the parent node's text.
    #[php(prop, name = "end")]
    pub end: u32,
    /// The type of annotation.
    #[php(prop, name = "kind")]
    pub kind: String,
}

#[php_impl]
impl TextAnnotation {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\ConversionResult")]
pub struct ConversionResult {
    /// Converted text output (markdown, djot, or plain text).
    ///
    /// `None` when `output_format` is set to `OutputFormat::None`,
    /// indicating extraction-only mode.
    #[php(prop, name = "content")]
    pub content: Option<String>,
    /// Structured document tree with semantic elements.
    ///
    /// Populated when `include_document_structure` is `true` in options.
    pub document: Option<DocumentStructure>,
    /// Extracted HTML metadata (title, OG, links, images, structured data).
    pub metadata: HtmlMetadata,
    /// Extracted tables with structured cell data and markdown representation.
    pub tables: Vec<TableData>,
    /// Extracted inline images (data URIs and SVGs).
    ///
    /// Populated when `extract_images` is `true` in options.
    #[php(prop, name = "images")]
    pub images: Vec<String>,
    /// Non-fatal processing warnings.
    pub warnings: Vec<ProcessingWarning>,
}

#[php_impl]
impl ConversionResult {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_document(&self) -> Option<DocumentStructure> {
        self.document.clone()
    }

    #[php(getter)]
    pub fn get_metadata(&self) -> HtmlMetadata {
        self.metadata.clone()
    }

    #[php(getter)]
    pub fn get_tables(&self) -> Vec<TableData> {
        self.tables.clone()
    }

    #[php(getter)]
    pub fn get_warnings(&self) -> Vec<ProcessingWarning> {
        self.warnings.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\TableGrid")]
#[allow(clippy::similar_names)]
pub struct TableGrid {
    /// Number of rows.
    #[php(prop, name = "rows")]
    pub rows: u32,
    /// Number of columns.
    #[php(prop, name = "cols")]
    pub cols: u32,
    /// All cells in the table (may be fewer than rows*cols due to spans).
    pub cells: Vec<GridCell>,
}

#[php_impl]
impl TableGrid {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_cells(&self) -> Vec<GridCell> {
        self.cells.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\GridCell")]
#[allow(clippy::similar_names)]
pub struct GridCell {
    /// The text content of the cell.
    #[php(prop, name = "content")]
    pub content: String,
    /// 0-indexed row position.
    #[php(prop, name = "row")]
    pub row: u32,
    /// 0-indexed column position.
    #[php(prop, name = "col")]
    pub col: u32,
    /// Number of rows this cell spans (default 1).
    #[php(prop, name = "row_span")]
    pub row_span: u32,
    /// Number of columns this cell spans (default 1).
    #[php(prop, name = "col_span")]
    pub col_span: u32,
    /// Whether this is a header cell (`<th>`).
    #[php(prop, name = "is_header")]
    pub is_header: bool,
}

#[php_impl]
impl GridCell {
    pub fn __construct(content: String, row: u32, col: u32, row_span: u32, col_span: u32, is_header: bool) -> Self {
        Self {
            content,
            row,
            col,
            row_span,
            col_span,
            is_header,
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\TableData")]
pub struct TableData {
    /// The structured table grid.
    pub grid: TableGrid,
    /// The markdown rendering of this table.
    #[php(prop, name = "markdown")]
    pub markdown: String,
}

#[php_impl]
impl TableData {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[php(getter)]
    pub fn get_grid(&self) -> TableGrid {
        self.grid.clone()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\ProcessingWarning")]
pub struct ProcessingWarning {
    /// Human-readable warning message.
    #[php(prop, name = "message")]
    pub message: String,
    /// The category of warning.
    #[php(prop, name = "kind")]
    pub kind: String,
}

#[php_impl]
impl ProcessingWarning {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }
}

// TextDirection enum values
pub const TEXTDIRECTION_LEFTTORIGHT: &str = "LeftToRight";
pub const TEXTDIRECTION_RIGHTTOLEFT: &str = "RightToLeft";
pub const TEXTDIRECTION_AUTO: &str = "Auto";

// LinkType enum values
pub const LINKTYPE_ANCHOR: &str = "Anchor";
pub const LINKTYPE_INTERNAL: &str = "Internal";
pub const LINKTYPE_EXTERNAL: &str = "External";
pub const LINKTYPE_EMAIL: &str = "Email";
pub const LINKTYPE_PHONE: &str = "Phone";
pub const LINKTYPE_OTHER: &str = "Other";

// ImageType enum values
pub const IMAGETYPE_DATAURI: &str = "DataUri";
pub const IMAGETYPE_INLINESVG: &str = "InlineSvg";
pub const IMAGETYPE_EXTERNAL: &str = "External";
pub const IMAGETYPE_RELATIVE: &str = "Relative";

// StructuredDataType enum values
pub const STRUCTUREDDATATYPE_JSONLD: &str = "JsonLd";
pub const STRUCTUREDDATATYPE_MICRODATA: &str = "Microdata";
pub const STRUCTUREDDATATYPE_RDFA: &str = "RDFa";

// PreprocessingPreset enum values
pub const PREPROCESSINGPRESET_MINIMAL: &str = "Minimal";
pub const PREPROCESSINGPRESET_STANDARD: &str = "Standard";
pub const PREPROCESSINGPRESET_AGGRESSIVE: &str = "Aggressive";

// HeadingStyle enum values
pub const HEADINGSTYLE_UNDERLINED: &str = "Underlined";
pub const HEADINGSTYLE_ATX: &str = "Atx";
pub const HEADINGSTYLE_ATXCLOSED: &str = "AtxClosed";

// ListIndentType enum values
pub const LISTINDENTTYPE_SPACES: &str = "Spaces";
pub const LISTINDENTTYPE_TABS: &str = "Tabs";

// WhitespaceMode enum values
pub const WHITESPACEMODE_NORMALIZED: &str = "Normalized";
pub const WHITESPACEMODE_STRICT: &str = "Strict";

// NewlineStyle enum values
pub const NEWLINESTYLE_SPACES: &str = "Spaces";
pub const NEWLINESTYLE_BACKSLASH: &str = "Backslash";

// CodeBlockStyle enum values
pub const CODEBLOCKSTYLE_INDENTED: &str = "Indented";
pub const CODEBLOCKSTYLE_BACKTICKS: &str = "Backticks";
pub const CODEBLOCKSTYLE_TILDES: &str = "Tildes";

// HighlightStyle enum values
pub const HIGHLIGHTSTYLE_DOUBLEEQUAL: &str = "DoubleEqual";
pub const HIGHLIGHTSTYLE_HTML: &str = "Html";
pub const HIGHLIGHTSTYLE_BOLD: &str = "Bold";
pub const HIGHLIGHTSTYLE_NONE: &str = "None";

// LinkStyle enum values
pub const LINKSTYLE_INLINE: &str = "Inline";
pub const LINKSTYLE_REFERENCE: &str = "Reference";

// OutputFormat enum values
pub const OUTPUTFORMAT_MARKDOWN: &str = "Markdown";
pub const OUTPUTFORMAT_DJOT: &str = "Djot";
pub const OUTPUTFORMAT_PLAIN: &str = "Plain";

// NodeContent enum values
pub const NODECONTENT_HEADING: &str = "Heading";
pub const NODECONTENT_PARAGRAPH: &str = "Paragraph";
pub const NODECONTENT_LIST: &str = "List";
pub const NODECONTENT_LISTITEM: &str = "ListItem";
pub const NODECONTENT_TABLE: &str = "Table";
pub const NODECONTENT_IMAGE: &str = "Image";
pub const NODECONTENT_CODE: &str = "Code";
pub const NODECONTENT_QUOTE: &str = "Quote";
pub const NODECONTENT_DEFINITIONLIST: &str = "DefinitionList";
pub const NODECONTENT_DEFINITIONITEM: &str = "DefinitionItem";
pub const NODECONTENT_RAWBLOCK: &str = "RawBlock";
pub const NODECONTENT_METADATABLOCK: &str = "MetadataBlock";
pub const NODECONTENT_GROUP: &str = "Group";

// AnnotationKind enum values
pub const ANNOTATIONKIND_BOLD: &str = "Bold";
pub const ANNOTATIONKIND_ITALIC: &str = "Italic";
pub const ANNOTATIONKIND_UNDERLINE: &str = "Underline";
pub const ANNOTATIONKIND_STRIKETHROUGH: &str = "Strikethrough";
pub const ANNOTATIONKIND_CODE: &str = "Code";
pub const ANNOTATIONKIND_SUBSCRIPT: &str = "Subscript";
pub const ANNOTATIONKIND_SUPERSCRIPT: &str = "Superscript";
pub const ANNOTATIONKIND_HIGHLIGHT: &str = "Highlight";
pub const ANNOTATIONKIND_LINK: &str = "Link";

// WarningKind enum values
pub const WARNINGKIND_IMAGEEXTRACTIONFAILED: &str = "ImageExtractionFailed";
pub const WARNINGKIND_ENCODINGFALLBACK: &str = "EncodingFallback";
pub const WARNINGKIND_TRUNCATEDINPUT: &str = "TruncatedInput";
pub const WARNINGKIND_MALFORMEDHTML: &str = "MalformedHtml";
pub const WARNINGKIND_SANITIZATIONAPPLIED: &str = "SanitizationApplied";
pub const WARNINGKIND_DEPTHLIMITEXCEEDED: &str = "DepthLimitExceeded";

#[php_class]
#[php(name = "Html\\To\\Markdown\\Rs\\HtmlToMarkdownRsApi")]
pub struct HtmlToMarkdownRsApi;

#[php_impl]
impl HtmlToMarkdownRsApi {
    pub fn convert(html: String, options: Option<&ConversionOptions>) -> PhpResult<ConversionResult> {
        let options_core: Option<html_to_markdown_rs::ConversionOptions> = options.map(|v| v.clone().into());
        let result = html_to_markdown_rs::convert(&html, options_core)
            .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
        Ok(result.into())
    }
}

impl From<MetadataConfig> for html_to_markdown_rs::metadata::MetadataConfig {
    fn from(val: MetadataConfig) -> Self {
        Self {
            extract_document: val.extract_document,
            extract_headers: val.extract_headers,
            extract_links: val.extract_links,
            extract_images: val.extract_images,
            extract_structured_data: val.extract_structured_data,
            max_structured_data_size: val.max_structured_data_size as usize,
        }
    }
}

impl From<html_to_markdown_rs::metadata::MetadataConfig> for MetadataConfig {
    fn from(val: html_to_markdown_rs::metadata::MetadataConfig) -> Self {
        Self {
            extract_document: val.extract_document,
            extract_headers: val.extract_headers,
            extract_links: val.extract_links,
            extract_images: val.extract_images,
            extract_structured_data: val.extract_structured_data,
            max_structured_data_size: val.max_structured_data_size as i64,
        }
    }
}

impl From<MetadataConfigUpdate> for html_to_markdown_rs::metadata::MetadataConfigUpdate {
    fn from(val: MetadataConfigUpdate) -> Self {
        Self {
            extract_document: val.extract_document,
            extract_headers: val.extract_headers,
            extract_links: val.extract_links,
            extract_images: val.extract_images,
            extract_structured_data: val.extract_structured_data,
            max_structured_data_size: val.max_structured_data_size.map(|v| v as usize),
        }
    }
}

impl From<html_to_markdown_rs::metadata::MetadataConfigUpdate> for MetadataConfigUpdate {
    fn from(val: html_to_markdown_rs::metadata::MetadataConfigUpdate) -> Self {
        Self {
            extract_document: val.extract_document,
            extract_headers: val.extract_headers,
            extract_links: val.extract_links,
            extract_images: val.extract_images,
            extract_structured_data: val.extract_structured_data,
            max_structured_data_size: val.max_structured_data_size.map(|v| v as i64),
        }
    }
}

impl From<DocumentMetadata> for html_to_markdown_rs::metadata::DocumentMetadata {
    fn from(val: DocumentMetadata) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::metadata::DocumentMetadata> for DocumentMetadata {
    fn from(val: html_to_markdown_rs::metadata::DocumentMetadata) -> Self {
        Self {
            title: val.title,
            description: val.description,
            keywords: val.keywords,
            author: val.author,
            canonical_url: val.canonical_url,
            base_href: val.base_href,
            language: val.language,
            text_direction: val.text_direction.as_ref().map(|v| {
                serde_json::to_value(v)
                    .ok()
                    .and_then(|s| s.as_str().map(String::from))
                    .unwrap_or_default()
            }),
            open_graph: val.open_graph.into_iter().collect(),
            twitter_card: val.twitter_card.into_iter().collect(),
            meta_tags: val.meta_tags.into_iter().collect(),
        }
    }
}

impl From<HeaderMetadata> for html_to_markdown_rs::metadata::HeaderMetadata {
    fn from(val: HeaderMetadata) -> Self {
        Self {
            level: val.level,
            text: val.text,
            id: val.id,
            depth: val.depth as usize,
            html_offset: val.html_offset as usize,
        }
    }
}

impl From<html_to_markdown_rs::metadata::HeaderMetadata> for HeaderMetadata {
    fn from(val: html_to_markdown_rs::metadata::HeaderMetadata) -> Self {
        Self {
            level: val.level,
            text: val.text,
            id: val.id,
            depth: val.depth as i64,
            html_offset: val.html_offset as i64,
        }
    }
}

impl From<LinkMetadata> for html_to_markdown_rs::metadata::LinkMetadata {
    fn from(val: LinkMetadata) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::metadata::LinkMetadata> for LinkMetadata {
    fn from(val: html_to_markdown_rs::metadata::LinkMetadata) -> Self {
        Self {
            href: val.href,
            text: val.text,
            title: val.title,
            link_type: serde_json::to_value(val.link_type)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            rel: val.rel,
            attributes: val.attributes.into_iter().collect(),
        }
    }
}

impl From<ImageMetadata> for html_to_markdown_rs::metadata::ImageMetadata {
    fn from(val: ImageMetadata) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::metadata::ImageMetadata> for ImageMetadata {
    fn from(val: html_to_markdown_rs::metadata::ImageMetadata) -> Self {
        Self {
            src: val.src,
            alt: val.alt,
            title: val.title,
            dimensions: val.dimensions.as_ref().map(|v| format!("{:?}", v)),
            image_type: serde_json::to_value(val.image_type)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            attributes: val.attributes.into_iter().collect(),
        }
    }
}

impl From<StructuredData> for html_to_markdown_rs::metadata::StructuredData {
    fn from(val: StructuredData) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::metadata::StructuredData> for StructuredData {
    fn from(val: html_to_markdown_rs::metadata::StructuredData) -> Self {
        Self {
            data_type: serde_json::to_value(val.data_type)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            raw_json: val.raw_json,
            schema_type: val.schema_type,
        }
    }
}

impl From<HtmlMetadata> for html_to_markdown_rs::metadata::HtmlMetadata {
    fn from(val: HtmlMetadata) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::metadata::HtmlMetadata> for HtmlMetadata {
    fn from(val: html_to_markdown_rs::metadata::HtmlMetadata) -> Self {
        Self {
            document: val.document.into(),
            headers: val.headers.into_iter().map(Into::into).collect(),
            links: val.links.into_iter().map(Into::into).collect(),
            images: val.images.into_iter().map(Into::into).collect(),
            structured_data: val.structured_data.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ConversionOptions> for html_to_markdown_rs::options::ConversionOptions {
    fn from(val: ConversionOptions) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::options::ConversionOptions> for ConversionOptions {
    fn from(val: html_to_markdown_rs::options::ConversionOptions) -> Self {
        Self {
            heading_style: serde_json::to_value(val.heading_style)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            list_indent_type: serde_json::to_value(val.list_indent_type)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            list_indent_width: val.list_indent_width as i64,
            bullets: val.bullets,
            strong_em_symbol: val.strong_em_symbol.to_string(),
            escape_asterisks: val.escape_asterisks,
            escape_underscores: val.escape_underscores,
            escape_misc: val.escape_misc,
            escape_ascii: val.escape_ascii,
            code_language: val.code_language,
            autolinks: val.autolinks,
            default_title: val.default_title,
            br_in_tables: val.br_in_tables,
            highlight_style: serde_json::to_value(val.highlight_style)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            extract_metadata: val.extract_metadata,
            whitespace_mode: serde_json::to_value(val.whitespace_mode)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            strip_newlines: val.strip_newlines,
            wrap: val.wrap,
            wrap_width: val.wrap_width as i64,
            convert_as_inline: val.convert_as_inline,
            sub_symbol: val.sub_symbol,
            sup_symbol: val.sup_symbol,
            newline_style: serde_json::to_value(val.newline_style)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            code_block_style: serde_json::to_value(val.code_block_style)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            keep_inline_images_in: val.keep_inline_images_in,
            preprocessing: val.preprocessing.into(),
            encoding: val.encoding,
            debug: val.debug,
            strip_tags: val.strip_tags,
            preserve_tags: val.preserve_tags,
            skip_images: val.skip_images,
            link_style: serde_json::to_value(val.link_style)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            output_format: serde_json::to_value(val.output_format)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            include_document_structure: val.include_document_structure,
            extract_images: val.extract_images,
            max_image_size: val.max_image_size as i64,
            capture_svg: val.capture_svg,
            infer_dimensions: val.infer_dimensions,
            max_depth: val.max_depth.map(|v| v as i64),
        }
    }
}

impl From<ConversionOptionsUpdate> for html_to_markdown_rs::options::ConversionOptionsUpdate {
    fn from(val: ConversionOptionsUpdate) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::options::ConversionOptionsUpdate> for ConversionOptionsUpdate {
    fn from(val: html_to_markdown_rs::options::ConversionOptionsUpdate) -> Self {
        Self {
            heading_style: val.heading_style.as_ref().map(|v| {
                serde_json::to_value(v)
                    .ok()
                    .and_then(|s| s.as_str().map(String::from))
                    .unwrap_or_default()
            }),
            list_indent_type: val.list_indent_type.as_ref().map(|v| {
                serde_json::to_value(v)
                    .ok()
                    .and_then(|s| s.as_str().map(String::from))
                    .unwrap_or_default()
            }),
            list_indent_width: val.list_indent_width.map(|v| v as i64),
            bullets: val.bullets,
            strong_em_symbol: val.strong_em_symbol.map(|c| c.to_string()),
            escape_asterisks: val.escape_asterisks,
            escape_underscores: val.escape_underscores,
            escape_misc: val.escape_misc,
            escape_ascii: val.escape_ascii,
            code_language: val.code_language,
            autolinks: val.autolinks,
            default_title: val.default_title,
            br_in_tables: val.br_in_tables,
            highlight_style: val.highlight_style.as_ref().map(|v| {
                serde_json::to_value(v)
                    .ok()
                    .and_then(|s| s.as_str().map(String::from))
                    .unwrap_or_default()
            }),
            extract_metadata: val.extract_metadata,
            whitespace_mode: val.whitespace_mode.as_ref().map(|v| {
                serde_json::to_value(v)
                    .ok()
                    .and_then(|s| s.as_str().map(String::from))
                    .unwrap_or_default()
            }),
            strip_newlines: val.strip_newlines,
            wrap: val.wrap,
            wrap_width: val.wrap_width.map(|v| v as i64),
            convert_as_inline: val.convert_as_inline,
            sub_symbol: val.sub_symbol,
            sup_symbol: val.sup_symbol,
            newline_style: val.newline_style.as_ref().map(|v| {
                serde_json::to_value(v)
                    .ok()
                    .and_then(|s| s.as_str().map(String::from))
                    .unwrap_or_default()
            }),
            code_block_style: val.code_block_style.as_ref().map(|v| {
                serde_json::to_value(v)
                    .ok()
                    .and_then(|s| s.as_str().map(String::from))
                    .unwrap_or_default()
            }),
            keep_inline_images_in: val.keep_inline_images_in,
            preprocessing: val.preprocessing.map(Into::into),
            encoding: val.encoding,
            debug: val.debug,
            strip_tags: val.strip_tags,
            preserve_tags: val.preserve_tags,
            skip_images: val.skip_images,
            link_style: val.link_style.as_ref().map(|v| {
                serde_json::to_value(v)
                    .ok()
                    .and_then(|s| s.as_str().map(String::from))
                    .unwrap_or_default()
            }),
            output_format: val.output_format.as_ref().map(|v| {
                serde_json::to_value(v)
                    .ok()
                    .and_then(|s| s.as_str().map(String::from))
                    .unwrap_or_default()
            }),
            include_document_structure: val.include_document_structure,
            extract_images: val.extract_images,
            max_image_size: val.max_image_size.map(|v| v as i64),
            capture_svg: val.capture_svg,
            infer_dimensions: val.infer_dimensions,
            max_depth: val.max_depth.flatten().map(|v| v as i64),
        }
    }
}

impl From<PreprocessingOptions> for html_to_markdown_rs::options::PreprocessingOptions {
    fn from(val: PreprocessingOptions) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::options::PreprocessingOptions> for PreprocessingOptions {
    fn from(val: html_to_markdown_rs::options::PreprocessingOptions) -> Self {
        Self {
            enabled: val.enabled,
            preset: serde_json::to_value(val.preset)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            remove_navigation: val.remove_navigation,
            remove_forms: val.remove_forms,
        }
    }
}

impl From<PreprocessingOptionsUpdate> for html_to_markdown_rs::options::PreprocessingOptionsUpdate {
    fn from(val: PreprocessingOptionsUpdate) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::options::PreprocessingOptionsUpdate> for PreprocessingOptionsUpdate {
    fn from(val: html_to_markdown_rs::options::PreprocessingOptionsUpdate) -> Self {
        Self {
            enabled: val.enabled,
            preset: val.preset.as_ref().map(|v| {
                serde_json::to_value(v)
                    .ok()
                    .and_then(|s| s.as_str().map(String::from))
                    .unwrap_or_default()
            }),
            remove_navigation: val.remove_navigation,
            remove_forms: val.remove_forms,
        }
    }
}

impl From<DocumentStructure> for html_to_markdown_rs::DocumentStructure {
    fn from(val: DocumentStructure) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::DocumentStructure> for DocumentStructure {
    fn from(val: html_to_markdown_rs::DocumentStructure) -> Self {
        Self {
            nodes: val.nodes.into_iter().map(Into::into).collect(),
            source_format: val.source_format,
        }
    }
}

impl From<DocumentNode> for html_to_markdown_rs::DocumentNode {
    fn from(val: DocumentNode) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::DocumentNode> for DocumentNode {
    fn from(val: html_to_markdown_rs::DocumentNode) -> Self {
        Self {
            id: val.id,
            content: serde_json::to_value(val.content)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
            parent: val.parent,
            children: val.children,
            annotations: val.annotations.into_iter().map(Into::into).collect(),
            attributes: val.attributes.map(|m| m.into_iter().collect()),
        }
    }
}

impl From<TextAnnotation> for html_to_markdown_rs::TextAnnotation {
    fn from(val: TextAnnotation) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::TextAnnotation> for TextAnnotation {
    fn from(val: html_to_markdown_rs::TextAnnotation) -> Self {
        Self {
            start: val.start,
            end: val.end,
            kind: serde_json::to_value(val.kind)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
        }
    }
}

impl From<ConversionResult> for html_to_markdown_rs::ConversionResult {
    fn from(val: ConversionResult) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::ConversionResult> for ConversionResult {
    fn from(val: html_to_markdown_rs::ConversionResult) -> Self {
        Self {
            content: val.content,
            document: val.document.map(Into::into),
            metadata: val.metadata.into(),
            tables: val.tables.into_iter().map(Into::into).collect(),
            images: val.images.iter().map(|i| format!("{:?}", i)).collect(),
            warnings: val.warnings.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<TableGrid> for html_to_markdown_rs::TableGrid {
    fn from(val: TableGrid) -> Self {
        Self {
            rows: val.rows,
            cols: val.cols,
            cells: val.cells.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<html_to_markdown_rs::TableGrid> for TableGrid {
    fn from(val: html_to_markdown_rs::TableGrid) -> Self {
        Self {
            rows: val.rows,
            cols: val.cols,
            cells: val.cells.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<GridCell> for html_to_markdown_rs::GridCell {
    fn from(val: GridCell) -> Self {
        Self {
            content: val.content,
            row: val.row,
            col: val.col,
            row_span: val.row_span,
            col_span: val.col_span,
            is_header: val.is_header,
        }
    }
}

impl From<html_to_markdown_rs::GridCell> for GridCell {
    fn from(val: html_to_markdown_rs::GridCell) -> Self {
        Self {
            content: val.content,
            row: val.row,
            col: val.col,
            row_span: val.row_span,
            col_span: val.col_span,
            is_header: val.is_header,
        }
    }
}

impl From<TableData> for html_to_markdown_rs::TableData {
    fn from(val: TableData) -> Self {
        Self {
            grid: val.grid.into(),
            markdown: val.markdown,
        }
    }
}

impl From<html_to_markdown_rs::TableData> for TableData {
    fn from(val: html_to_markdown_rs::TableData) -> Self {
        Self {
            grid: val.grid.into(),
            markdown: val.markdown,
        }
    }
}

impl From<ProcessingWarning> for html_to_markdown_rs::ProcessingWarning {
    fn from(val: ProcessingWarning) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::ProcessingWarning> for ProcessingWarning {
    fn from(val: html_to_markdown_rs::ProcessingWarning) -> Self {
        Self {
            message: val.message,
            kind: serde_json::to_value(val.kind)
                .ok()
                .and_then(|s| s.as_str().map(String::from))
                .unwrap_or_default(),
        }
    }
}

/// Convert a `html_to_markdown_rs::error::ConversionError` error to a PHP exception.
#[allow(dead_code)]
fn conversion_error_to_php_err(e: html_to_markdown_rs::error::ConversionError) -> ext_php_rs::exception::PhpException {
    let msg = e.to_string();
    #[allow(unreachable_patterns)]
    match &e {
        html_to_markdown_rs::error::ConversionError::ParseError(..) => {
            ext_php_rs::exception::PhpException::default(format!("[ParseError] {}", msg))
        }
        html_to_markdown_rs::error::ConversionError::SanitizationError(..) => {
            ext_php_rs::exception::PhpException::default(format!("[SanitizationError] {}", msg))
        }
        html_to_markdown_rs::error::ConversionError::ConfigError(..) => {
            ext_php_rs::exception::PhpException::default(format!("[ConfigError] {}", msg))
        }
        html_to_markdown_rs::error::ConversionError::IoError(..) => {
            ext_php_rs::exception::PhpException::default(format!("[IoError] {}", msg))
        }
        html_to_markdown_rs::error::ConversionError::Panic(..) => {
            ext_php_rs::exception::PhpException::default(format!("[Panic] {}", msg))
        }
        html_to_markdown_rs::error::ConversionError::InvalidInput(..) => {
            ext_php_rs::exception::PhpException::default(format!("[InvalidInput] {}", msg))
        }
        html_to_markdown_rs::error::ConversionError::Other(..) => {
            ext_php_rs::exception::PhpException::default(format!("[Other] {}", msg))
        }
        _ => ext_php_rs::exception::PhpException::default(msg),
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .class::<MetadataConfig>()
        .class::<MetadataConfigUpdate>()
        .class::<DocumentMetadata>()
        .class::<HeaderMetadata>()
        .class::<LinkMetadata>()
        .class::<ImageMetadata>()
        .class::<StructuredData>()
        .class::<HtmlMetadata>()
        .class::<ConversionOptions>()
        .class::<ConversionOptionsBuilder>()
        .class::<ConversionOptionsUpdate>()
        .class::<PreprocessingOptions>()
        .class::<PreprocessingOptionsUpdate>()
        .class::<DocumentStructure>()
        .class::<DocumentNode>()
        .class::<TextAnnotation>()
        .class::<ConversionResult>()
        .class::<TableGrid>()
        .class::<GridCell>()
        .class::<TableData>()
        .class::<ProcessingWarning>()
        .class::<HtmlToMarkdownRsApi>()
}
