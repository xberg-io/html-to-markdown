use ext_php_rs::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
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
    pub extract_document: bool,
    /// Extract h1-h6 header elements and their hierarchy.
    ///
    /// When enabled, collects all heading elements with:
    /// - Header level (1-6)
    /// - Text content (normalized)
    /// - HTML id attribute if present
    /// - Document tree depth for hierarchy tracking
    /// - Byte offset in original HTML for positioning
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
    pub extract_images: bool,
    /// Extract structured data (JSON-LD, Microdata, RDFa).
    ///
    /// When enabled, collects machine-readable structured data including:
    /// - JSON-LD script blocks with schema detection
    /// - Microdata attributes (itemscope, itemtype, itemprop)
    /// - RDFa markup
    /// - Extracted schema type if detectable
    pub extract_structured_data: bool,
    /// Maximum total size of structured data to collect (bytes).
    ///
    /// Prevents memory exhaustion attacks on malformed or adversarial documents
    /// containing excessively large structured data blocks. When the accumulated
    /// size of structured data exceeds this limit, further collection stops.
    /// Default: `1_000_000` bytes (1 MB)
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

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> MetadataConfig {
        html_to_markdown_rs::metadata::MetadataConfig::default().into()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
pub struct MetadataConfigUpdate {
    /// Optional override for extracting document-level metadata.
    ///
    /// When Some(true), enables document metadata extraction; Some(false) disables it.
    /// None leaves the current setting unchanged.
    pub extract_document: Option<bool>,
    /// Optional override for extracting heading elements (h1-h6).
    ///
    /// When Some(true), enables header extraction; Some(false) disables it.
    /// None leaves the current setting unchanged.
    pub extract_headers: Option<bool>,
    /// Optional override for extracting anchor (link) elements.
    ///
    /// When Some(true), enables link extraction; Some(false) disables it.
    /// None leaves the current setting unchanged.
    pub extract_links: Option<bool>,
    /// Optional override for extracting image elements.
    ///
    /// When Some(true), enables image extraction; Some(false) disables it.
    /// None leaves the current setting unchanged.
    pub extract_images: Option<bool>,
    /// Optional override for extracting structured data (JSON-LD, Microdata, RDFa).
    ///
    /// When Some(true), enables structured data extraction; Some(false) disables it.
    /// None leaves the current setting unchanged.
    pub extract_structured_data: Option<bool>,
    /// Optional override for maximum structured data collection size in bytes.
    ///
    /// When Some(size), sets the new size limit. None leaves the current limit unchanged.
    /// Use this to adjust safety thresholds for different documents.
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
#[allow(clippy::similar_names)]
pub struct ConversionOptions {
    /// Heading style to use in Markdown output (ATX `#` or Setext underline).
    pub heading_style: String,
    /// How to indent nested list items (spaces or tab).
    pub list_indent_type: String,
    /// Number of spaces (or tabs) to use for each level of list indentation.
    pub list_indent_width: i64,
    /// Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`).
    pub bullets: String,
    /// Character used for bold/italic emphasis markers (`*` or `_`).
    pub strong_em_symbol: String,
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
    pub highlight_style: String,
    /// Extract `<meta>` and `<head>` information into the result metadata.
    pub extract_metadata: bool,
    /// Controls how whitespace is normalised during conversion.
    pub whitespace_mode: String,
    /// Strip all newlines from the output, producing a single-line result.
    pub strip_newlines: bool,
    /// Wrap long lines at [`wrap_width`](Self::wrap_width) characters.
    pub wrap: bool,
    /// Maximum line width when [`wrap`](Self::wrap) is enabled (default `80`).
    pub wrap_width: i64,
    /// Treat the entire document as inline content (no block-level wrappers).
    pub convert_as_inline: bool,
    /// Markdown notation for subscript text (e.g. `"~"`).
    pub sub_symbol: String,
    /// Markdown notation for superscript text (e.g. `"^"`).
    pub sup_symbol: String,
    /// How to encode hard line breaks (`<br>`) in Markdown.
    pub newline_style: String,
    /// Style used for fenced code blocks (backticks or tilde).
    pub code_block_style: String,
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
    pub link_style: String,
    /// Target output format (Markdown, plain text, etc.).
    pub output_format: String,
    /// Include structured document tree in result.
    pub include_document_structure: bool,
    /// Extract inline images from data URIs and SVGs.
    pub extract_images: bool,
    /// Maximum decoded image size in bytes (default 5MB).
    pub max_image_size: i64,
    /// Capture SVG elements as images.
    pub capture_svg: bool,
    /// Infer image dimensions from data.
    pub infer_dimensions: bool,
}

#[php_impl]
impl ConversionOptions {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> ConversionOptions {
        html_to_markdown_rs::ConversionOptions::default().into()
    }

    pub fn builder() -> ConversionOptionsBuilder {
        ConversionOptionsBuilder {
            inner: Arc::new(html_to_markdown_rs::ConversionOptions::builder()),
        }
    }
}

#[derive(Clone)]
#[php_class]
pub struct ConversionOptionsBuilder {
    inner: Arc<html_to_markdown_rs::ConversionOptionsBuilder>,
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
#[allow(clippy::similar_names)]
pub struct ConversionOptionsUpdate {
    /// Optional override for [`ConversionOptions::heading_style`].
    pub heading_style: Option<String>,
    /// Optional override for [`ConversionOptions::list_indent_type`].
    pub list_indent_type: Option<String>,
    /// Optional override for [`ConversionOptions::list_indent_width`].
    pub list_indent_width: Option<i64>,
    /// Optional override for [`ConversionOptions::bullets`].
    pub bullets: Option<String>,
    /// Optional override for [`ConversionOptions::strong_em_symbol`].
    pub strong_em_symbol: Option<String>,
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
    pub highlight_style: Option<String>,
    /// Optional override for [`ConversionOptions::extract_metadata`].
    pub extract_metadata: Option<bool>,
    /// Optional override for [`ConversionOptions::whitespace_mode`].
    pub whitespace_mode: Option<String>,
    /// Optional override for [`ConversionOptions::strip_newlines`].
    pub strip_newlines: Option<bool>,
    /// Optional override for [`ConversionOptions::wrap`].
    pub wrap: Option<bool>,
    /// Optional override for [`ConversionOptions::wrap_width`].
    pub wrap_width: Option<i64>,
    /// Optional override for [`ConversionOptions::convert_as_inline`].
    pub convert_as_inline: Option<bool>,
    /// Optional override for [`ConversionOptions::sub_symbol`].
    pub sub_symbol: Option<String>,
    /// Optional override for [`ConversionOptions::sup_symbol`].
    pub sup_symbol: Option<String>,
    /// Optional override for [`ConversionOptions::newline_style`].
    pub newline_style: Option<String>,
    /// Optional override for [`ConversionOptions::code_block_style`].
    pub code_block_style: Option<String>,
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
    pub link_style: Option<String>,
    /// Optional override for [`ConversionOptions::output_format`].
    pub output_format: Option<String>,
    /// Optional override for [`ConversionOptions::include_document_structure`].
    pub include_document_structure: Option<bool>,
    /// Optional override for [`ConversionOptions::extract_images`].
    pub extract_images: Option<bool>,
    /// Optional override for [`ConversionOptions::max_image_size`].
    pub max_image_size: Option<i64>,
    /// Optional override for [`ConversionOptions::capture_svg`].
    pub capture_svg: Option<bool>,
    /// Optional override for [`ConversionOptions::infer_dimensions`].
    pub infer_dimensions: Option<bool>,
}

#[php_impl]
impl ConversionOptionsUpdate {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
pub struct PreprocessingOptions {
    /// Enable HTML preprocessing globally
    pub enabled: bool,
    /// Preprocessing preset level (Minimal, Standard, Aggressive)
    pub preset: String,
    /// Remove navigation elements (nav, breadcrumbs, menus, sidebars)
    pub remove_navigation: bool,
    /// Remove form elements (forms, inputs, buttons, etc.)
    pub remove_forms: bool,
}

#[php_impl]
impl PreprocessingOptions {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> PreprocessingOptions {
        html_to_markdown_rs::PreprocessingOptions::default().into()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
pub struct PreprocessingOptionsUpdate {
    /// Optional global preprocessing enablement override
    pub enabled: Option<bool>,
    /// Optional preprocessing preset level override (Minimal, Standard, Aggressive)
    pub preset: Option<String>,
    /// Optional navigation element removal override (nav, breadcrumbs, menus, sidebars)
    pub remove_navigation: Option<bool>,
    /// Optional form element removal override (forms, inputs, buttons, etc.)
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
pub struct ConversionResult {
    /// Converted text output (markdown, djot, or plain text).
    ///
    /// `None` when `output_format` is set to `OutputFormat::None`,
    /// indicating extraction-only mode.
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
    pub images: Vec<String>,
    /// Non-fatal processing warnings.
    pub warnings: Vec<ProcessingWarning>,
}

#[php_impl]
impl ConversionResult {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[php_class]
pub struct DocumentStructure {
    /// All nodes in document reading order.
    pub nodes: Vec<DocumentNode>,
    /// The source format (always "html" for this crate).
    pub source_format: Option<String>,
}

#[php_impl]
impl DocumentStructure {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[php_class]
pub struct DocumentNode {
    /// Deterministic node identifier.
    pub id: String,
    /// The semantic content of this node.
    pub content: String,
    /// Index of the parent node (None for root nodes).
    pub parent: Option<u32>,
    /// Indices of child nodes in reading order.
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
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[php_class]
pub struct TextAnnotation {
    /// Start byte offset (inclusive) into the parent node's text.
    pub start: u32,
    /// End byte offset (exclusive) into the parent node's text.
    pub end: u32,
    /// The type of annotation.
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
#[allow(clippy::similar_names)]
pub struct TableGrid {
    /// Number of rows.
    pub rows: u32,
    /// Number of columns.
    pub cols: u32,
    /// All cells in the table (may be fewer than rows*cols due to spans).
    pub cells: Vec<GridCell>,
}

#[php_impl]
impl TableGrid {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[php_class]
#[allow(clippy::similar_names)]
pub struct GridCell {
    /// The text content of the cell.
    pub content: String,
    /// 0-indexed row position.
    pub row: u32,
    /// 0-indexed column position.
    pub col: u32,
    /// Number of rows this cell spans (default 1).
    pub row_span: u32,
    /// Number of columns this cell spans (default 1).
    pub col_span: u32,
    /// Whether this is a header cell (`<th>`).
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

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[php_class]
pub struct TableData {
    /// The structured table grid.
    pub grid: TableGrid,
    /// The markdown rendering of this table.
    pub markdown: String,
}

#[php_impl]
impl TableData {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[php_class]
pub struct ProcessingWarning {
    /// Human-readable warning message.
    pub message: String,
    /// The category of warning.
    pub kind: String,
}

#[php_impl]
impl ProcessingWarning {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[php_class]
pub struct DocumentMetadata {
    /// Document title from `<title>` tag
    pub title: Option<String>,
    /// Document description from `<meta name="description">` tag
    pub description: Option<String>,
    /// Document keywords from `<meta name="keywords">` tag, split on commas
    pub keywords: Vec<String>,
    /// Document author from `<meta name="author">` tag
    pub author: Option<String>,
    /// Canonical URL from `<link rel="canonical">` tag
    pub canonical_url: Option<String>,
    /// Base URL from `<base href="">` tag for resolving relative URLs
    pub base_href: Option<String>,
    /// Document language from `lang` attribute
    pub language: Option<String>,
    /// Document text direction from `dir` attribute
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
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[php_class]
pub struct HeaderMetadata {
    /// Header level: 1 (h1) through 6 (h6)
    pub level: u8,
    /// Normalized text content of the header
    pub text: String,
    /// HTML id attribute if present
    pub id: Option<String>,
    /// Document tree depth at the header element
    pub depth: i64,
    /// Byte offset in original HTML document
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
        let core_self = html_to_markdown_rs::HeaderMetadata {
            level: self.level,
            text: self.text.clone(),
            id: self.id.clone(),
            depth: self.depth as usize,
            html_offset: self.html_offset as usize,
        };
        core_self.is_valid()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[php_class]
pub struct LinkMetadata {
    /// The href URL value
    pub href: String,
    /// Link text content (normalized, concatenated if mixed with elements)
    pub text: String,
    /// Optional title attribute (often shown as tooltip)
    pub title: Option<String>,
    /// Link type classification
    pub link_type: String,
    /// Rel attribute values (e.g., "nofollow", "stylesheet", "canonical")
    pub rel: Vec<String>,
    /// Additional HTML attributes
    pub attributes: HashMap<String, String>,
}

#[php_impl]
impl LinkMetadata {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }

    pub fn classify_link(href: String) -> String {
        format!("{:?}", html_to_markdown_rs::LinkMetadata::classify_link(&href))
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[php_class]
pub struct ImageMetadata {
    /// Image source (URL, data URI, or SVG content identifier)
    pub src: String,
    /// Alternative text from alt attribute (for accessibility)
    pub alt: Option<String>,
    /// Title attribute (often shown as tooltip)
    pub title: Option<String>,
    /// Image dimensions as (width, height) if available
    pub dimensions: Option<String>,
    /// Image type classification
    pub image_type: String,
    /// Additional HTML attributes
    pub attributes: HashMap<String, String>,
}

#[php_impl]
impl ImageMetadata {
    pub fn from_json(json: String) -> PhpResult<Self> {
        serde_json::from_str(&json).map_err(|e| PhpException::default(e.to_string()))
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[php_class]
pub struct StructuredData {
    /// Type of structured data (JSON-LD, Microdata, RDFa)
    pub data_type: String,
    /// Raw JSON string (for JSON-LD) or serialized representation
    pub raw_json: String,
    /// Schema type if detectable (e.g., "Article", "Event", "Product")
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
}

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

#[php_function]
pub fn convert(html: String, options: Option<&ConversionOptions>) -> PhpResult<ConversionResult> {
    let options_core: Option<html_to_markdown_rs::ConversionOptions> = options.map(|v| v.clone().into());
    let result = html_to_markdown_rs::convert(&html, options_core)
        .map_err(|e| ext_php_rs::exception::PhpException::default(e.to_string()))?;
    Ok(result.into())
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

impl From<ConversionOptions> for html_to_markdown_rs::ConversionOptions {
    fn from(val: ConversionOptions) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::ConversionOptions> for ConversionOptions {
    fn from(val: html_to_markdown_rs::ConversionOptions) -> Self {
        Self {
            heading_style: format!("{:?}", val.heading_style),
            list_indent_type: format!("{:?}", val.list_indent_type),
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
            highlight_style: format!("{:?}", val.highlight_style),
            extract_metadata: val.extract_metadata,
            whitespace_mode: format!("{:?}", val.whitespace_mode),
            strip_newlines: val.strip_newlines,
            wrap: val.wrap,
            wrap_width: val.wrap_width as i64,
            convert_as_inline: val.convert_as_inline,
            sub_symbol: val.sub_symbol,
            sup_symbol: val.sup_symbol,
            newline_style: format!("{:?}", val.newline_style),
            code_block_style: format!("{:?}", val.code_block_style),
            keep_inline_images_in: val.keep_inline_images_in,
            preprocessing: val.preprocessing.into(),
            encoding: val.encoding,
            debug: val.debug,
            strip_tags: val.strip_tags,
            preserve_tags: val.preserve_tags,
            skip_images: val.skip_images,
            link_style: format!("{:?}", val.link_style),
            output_format: format!("{:?}", val.output_format),
            include_document_structure: val.include_document_structure,
            extract_images: val.extract_images,
            max_image_size: val.max_image_size as i64,
            capture_svg: val.capture_svg,
            infer_dimensions: val.infer_dimensions,
        }
    }
}

impl From<ConversionOptionsUpdate> for html_to_markdown_rs::ConversionOptionsUpdate {
    fn from(val: ConversionOptionsUpdate) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::ConversionOptionsUpdate> for ConversionOptionsUpdate {
    fn from(val: html_to_markdown_rs::ConversionOptionsUpdate) -> Self {
        Self {
            heading_style: val.heading_style.as_ref().map(|v| format!("{:?}", v)),
            list_indent_type: val.list_indent_type.as_ref().map(|v| format!("{:?}", v)),
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
            highlight_style: val.highlight_style.as_ref().map(|v| format!("{:?}", v)),
            extract_metadata: val.extract_metadata,
            whitespace_mode: val.whitespace_mode.as_ref().map(|v| format!("{:?}", v)),
            strip_newlines: val.strip_newlines,
            wrap: val.wrap,
            wrap_width: val.wrap_width.map(|v| v as i64),
            convert_as_inline: val.convert_as_inline,
            sub_symbol: val.sub_symbol,
            sup_symbol: val.sup_symbol,
            newline_style: val.newline_style.as_ref().map(|v| format!("{:?}", v)),
            code_block_style: val.code_block_style.as_ref().map(|v| format!("{:?}", v)),
            keep_inline_images_in: val.keep_inline_images_in,
            preprocessing: val.preprocessing.map(Into::into),
            encoding: val.encoding,
            debug: val.debug,
            strip_tags: val.strip_tags,
            preserve_tags: val.preserve_tags,
            skip_images: val.skip_images,
            link_style: val.link_style.as_ref().map(|v| format!("{:?}", v)),
            output_format: val.output_format.as_ref().map(|v| format!("{:?}", v)),
            include_document_structure: val.include_document_structure,
            extract_images: val.extract_images,
            max_image_size: val.max_image_size.map(|v| v as i64),
            capture_svg: val.capture_svg,
            infer_dimensions: val.infer_dimensions,
        }
    }
}

impl From<PreprocessingOptions> for html_to_markdown_rs::PreprocessingOptions {
    fn from(val: PreprocessingOptions) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::PreprocessingOptions> for PreprocessingOptions {
    fn from(val: html_to_markdown_rs::PreprocessingOptions) -> Self {
        Self {
            enabled: val.enabled,
            preset: format!("{:?}", val.preset),
            remove_navigation: val.remove_navigation,
            remove_forms: val.remove_forms,
        }
    }
}

impl From<PreprocessingOptionsUpdate> for html_to_markdown_rs::PreprocessingOptionsUpdate {
    fn from(val: PreprocessingOptionsUpdate) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::PreprocessingOptionsUpdate> for PreprocessingOptionsUpdate {
    fn from(val: html_to_markdown_rs::PreprocessingOptionsUpdate) -> Self {
        Self {
            enabled: val.enabled,
            preset: val.preset.as_ref().map(|v| format!("{:?}", v)),
            remove_navigation: val.remove_navigation,
            remove_forms: val.remove_forms,
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
            content: format!("{:?}", val.content),
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
            kind: format!("{:?}", val.kind),
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
            kind: format!("{:?}", val.kind),
        }
    }
}

impl From<DocumentMetadata> for html_to_markdown_rs::DocumentMetadata {
    fn from(val: DocumentMetadata) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::DocumentMetadata> for DocumentMetadata {
    fn from(val: html_to_markdown_rs::DocumentMetadata) -> Self {
        Self {
            title: val.title,
            description: val.description,
            keywords: val.keywords,
            author: val.author,
            canonical_url: val.canonical_url,
            base_href: val.base_href,
            language: val.language,
            text_direction: val.text_direction.as_ref().map(|v| format!("{:?}", v)),
            open_graph: val.open_graph.into_iter().collect(),
            twitter_card: val.twitter_card.into_iter().collect(),
            meta_tags: val.meta_tags.into_iter().collect(),
        }
    }
}

impl From<HeaderMetadata> for html_to_markdown_rs::HeaderMetadata {
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

impl From<html_to_markdown_rs::HeaderMetadata> for HeaderMetadata {
    fn from(val: html_to_markdown_rs::HeaderMetadata) -> Self {
        Self {
            level: val.level,
            text: val.text,
            id: val.id,
            depth: val.depth as i64,
            html_offset: val.html_offset as i64,
        }
    }
}

impl From<LinkMetadata> for html_to_markdown_rs::LinkMetadata {
    fn from(val: LinkMetadata) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::LinkMetadata> for LinkMetadata {
    fn from(val: html_to_markdown_rs::LinkMetadata) -> Self {
        Self {
            href: val.href,
            text: val.text,
            title: val.title,
            link_type: format!("{:?}", val.link_type),
            rel: val.rel,
            attributes: val.attributes.into_iter().collect(),
        }
    }
}

impl From<ImageMetadata> for html_to_markdown_rs::ImageMetadata {
    fn from(val: ImageMetadata) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::ImageMetadata> for ImageMetadata {
    fn from(val: html_to_markdown_rs::ImageMetadata) -> Self {
        Self {
            src: val.src,
            alt: val.alt,
            title: val.title,
            dimensions: val.dimensions.as_ref().map(|v| format!("{:?}", v)),
            image_type: format!("{:?}", val.image_type),
            attributes: val.attributes.into_iter().collect(),
        }
    }
}

impl From<StructuredData> for html_to_markdown_rs::StructuredData {
    fn from(val: StructuredData) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::StructuredData> for StructuredData {
    fn from(val: html_to_markdown_rs::StructuredData) -> Self {
        Self {
            data_type: format!("{:?}", val.data_type),
            raw_json: val.raw_json,
            schema_type: val.schema_type,
        }
    }
}

impl From<HtmlMetadata> for html_to_markdown_rs::HtmlMetadata {
    fn from(val: HtmlMetadata) -> Self {
        let json = serde_json::to_string(&val).expect("alef: serialize binding type");
        serde_json::from_str(&json).expect("alef: deserialize to core type")
    }
}

impl From<html_to_markdown_rs::HtmlMetadata> for HtmlMetadata {
    fn from(val: html_to_markdown_rs::HtmlMetadata) -> Self {
        Self {
            document: val.document.into(),
            headers: val.headers.into_iter().map(Into::into).collect(),
            links: val.links.into_iter().map(Into::into).collect(),
            images: val.images.into_iter().map(Into::into).collect(),
            structured_data: val.structured_data.into_iter().map(Into::into).collect(),
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
        .class::<ConversionOptions>()
        .class::<ConversionOptionsBuilder>()
        .class::<ConversionOptionsUpdate>()
        .class::<PreprocessingOptions>()
        .class::<PreprocessingOptionsUpdate>()
        .class::<ConversionResult>()
        .class::<DocumentStructure>()
        .class::<DocumentNode>()
        .class::<TextAnnotation>()
        .class::<TableGrid>()
        .class::<GridCell>()
        .class::<TableData>()
        .class::<ProcessingWarning>()
        .class::<DocumentMetadata>()
        .class::<HeaderMetadata>()
        .class::<LinkMetadata>()
        .class::<ImageMetadata>()
        .class::<StructuredData>()
        .class::<HtmlMetadata>()
        .function(wrap_function!(convert))
}
