#![allow(clippy::cast_precision_loss, clippy::cast_sign_loss, clippy::unused_self)]
//! Type definitions for metadata extraction.

use std::collections::BTreeMap;

/// Text directionality of document content.
///
/// Corresponds to the HTML `dir` attribute and `bdi` element directionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "metadata", derive(serde::Serialize, serde::Deserialize))]
pub enum TextDirection {
    /// Left-to-right text flow (default for Latin scripts)
    #[cfg_attr(feature = "metadata", serde(rename = "ltr"))]
    LeftToRight,
    /// Right-to-left text flow (Hebrew, Arabic, Urdu, etc.)
    #[cfg_attr(feature = "metadata", serde(rename = "rtl"))]
    RightToLeft,
    /// Automatic directionality detection
    #[cfg_attr(feature = "metadata", serde(rename = "auto"))]
    Auto,
}

impl std::fmt::Display for TextDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftToRight => write!(f, "ltr"),
            Self::RightToLeft => write!(f, "rtl"),
            Self::Auto => write!(f, "auto"),
        }
    }
}

impl TextDirection {
    /// Parse a text direction from string value.
    ///
    /// # Arguments
    ///
    /// * `s` - Direction string ("ltr", "rtl", or "auto")
    ///
    /// # Returns
    ///
    /// `Some(TextDirection)` if valid, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use html_to_markdown_rs::metadata::TextDirection;
    /// assert_eq!(TextDirection::parse("ltr"), Some(TextDirection::LeftToRight));
    /// assert_eq!(TextDirection::parse("rtl"), Some(TextDirection::RightToLeft));
    /// assert_eq!(TextDirection::parse("auto"), Some(TextDirection::Auto));
    /// assert_eq!(TextDirection::parse("invalid"), None);
    /// ```
    #[must_use]
    #[cfg_attr(alef, alef(skip))]
    pub fn parse(s: &str) -> Option<Self> {
        if s.eq_ignore_ascii_case("ltr") {
            return Some(Self::LeftToRight);
        }
        if s.eq_ignore_ascii_case("rtl") {
            return Some(Self::RightToLeft);
        }
        if s.eq_ignore_ascii_case("auto") {
            return Some(Self::Auto);
        }
        None
    }
}

/// Link classification based on href value and document context.
///
/// Used to categorize links during extraction for filtering and analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "metadata", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "metadata", serde(rename_all = "snake_case"))]
pub enum LinkType {
    /// Anchor link within same document (href starts with #)
    Anchor,
    /// Internal link within same domain
    Internal,
    /// External link to different domain
    External,
    /// Email link (mailto:)
    Email,
    /// Phone link (tel:)
    Phone,
    /// Other protocol or unclassifiable
    Other,
}

impl std::fmt::Display for LinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Anchor => write!(f, "anchor"),
            Self::Internal => write!(f, "internal"),
            Self::External => write!(f, "external"),
            Self::Email => write!(f, "email"),
            Self::Phone => write!(f, "phone"),
            Self::Other => write!(f, "other"),
        }
    }
}

/// Image source classification for proper handling and processing.
///
/// Determines whether an image is embedded (data URI), inline SVG, external, or relative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "metadata", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "metadata", serde(rename_all = "snake_case"))]
pub enum ImageType {
    /// Data URI embedded image (base64 or other encoding)
    DataUri,
    /// Inline SVG element
    InlineSvg,
    /// External image URL (http/https)
    External,
    /// Relative image path
    Relative,
}

impl std::fmt::Display for ImageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DataUri => write!(f, "data_uri"),
            Self::InlineSvg => write!(f, "inline_svg"),
            Self::External => write!(f, "external"),
            Self::Relative => write!(f, "relative"),
        }
    }
}

/// Structured data format type.
///
/// Identifies the schema/format used for structured data markup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "metadata", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "metadata", serde(rename_all = "snake_case"))]
pub enum StructuredDataType {
    /// JSON-LD (JSON for Linking Data) script blocks
    #[cfg_attr(feature = "metadata", serde(rename = "json_ld"))]
    JsonLd,
    /// HTML5 Microdata attributes (itemscope, itemtype, itemprop)
    Microdata,
    /// RDF in Attributes (RDFa) markup
    #[cfg_attr(feature = "metadata", serde(rename = "rdfa"))]
    RDFa,
}

impl std::fmt::Display for StructuredDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JsonLd => write!(f, "json_ld"),
            Self::Microdata => write!(f, "microdata"),
            Self::RDFa => write!(f, "rdfa"),
        }
    }
}

/// Document-level metadata extracted from `<head>` and top-level elements.
///
/// Contains all metadata typically used by search engines, social media platforms,
/// and browsers for document indexing and presentation.
///
/// # Examples
///
/// ```
/// # use html_to_markdown_rs::metadata::DocumentMetadata;
/// let doc = DocumentMetadata {
///     title: Some("My Article".to_string()),
///     description: Some("A great article about Rust".to_string()),
///     keywords: vec!["rust".to_string(), "programming".to_string()],
///     ..Default::default()
/// };
///
/// assert_eq!(doc.title, Some("My Article".to_string()));
/// ```
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "metadata", derive(serde::Serialize, serde::Deserialize))]
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
    pub text_direction: Option<TextDirection>,

    /// Open Graph metadata (og:* properties) for social media
    /// Keys like "title", "description", "image", "url", etc.
    pub open_graph: BTreeMap<String, String>,

    /// Twitter Card metadata (twitter:* properties)
    /// Keys like "card", "site", "creator", "title", "description", "image", etc.
    pub twitter_card: BTreeMap<String, String>,

    /// Additional meta tags not covered by specific fields
    /// Keys are meta name/property attributes, values are content
    pub meta_tags: BTreeMap<String, String>,
}

/// Header element metadata with hierarchy tracking.
///
/// Captures heading elements (h1-h6) with their text content, identifiers,
/// and position in the document structure.
///
/// # Examples
///
/// ```
/// # use html_to_markdown_rs::metadata::HeaderMetadata;
/// let header = HeaderMetadata {
///     level: 1,
///     text: "Main Title".to_string(),
///     id: Some("main-title".to_string()),
///     depth: 0,
///     html_offset: 145,
/// };
///
/// assert_eq!(header.level, 1);
/// assert!(header.is_valid());
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "metadata", derive(serde::Serialize, serde::Deserialize))]
pub struct HeaderMetadata {
    /// Header level: 1 (h1) through 6 (h6)
    pub level: u8,

    /// Normalized text content of the header
    pub text: String,

    /// HTML id attribute if present
    pub id: Option<String>,

    /// Document tree depth at the header element
    pub depth: usize,

    /// Byte offset in original HTML document
    pub html_offset: usize,
}

impl HeaderMetadata {
    /// Validate that the header level is within valid range (1-6).
    ///
    /// # Returns
    ///
    /// `true` if level is 1-6, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use html_to_markdown_rs::metadata::HeaderMetadata;
    /// let valid = HeaderMetadata {
    ///     level: 3,
    ///     text: "Title".to_string(),
    ///     id: None,
    ///     depth: 2,
    ///     html_offset: 100,
    /// };
    /// assert!(valid.is_valid());
    ///
    /// let invalid = HeaderMetadata {
    ///     level: 7,  // Invalid
    ///     text: "Title".to_string(),
    ///     id: None,
    ///     depth: 2,
    ///     html_offset: 100,
    /// };
    /// assert!(!invalid.is_valid());
    /// ```
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.level >= 1 && self.level <= 6
    }
}

/// Hyperlink metadata with categorization and attributes.
///
/// Represents `<a>` elements with parsed href values, text content, and link type classification.
///
/// # Examples
///
/// ```
/// # use html_to_markdown_rs::metadata::{LinkMetadata, LinkType};
/// let link = LinkMetadata {
///     href: "https://example.com".to_string(),
///     text: "Example".to_string(),
///     title: Some("Visit Example".to_string()),
///     link_type: LinkType::External,
///     rel: vec!["nofollow".to_string()],
///     attributes: Default::default(),
/// };
///
/// assert_eq!(link.link_type, LinkType::External);
/// assert_eq!(link.text, "Example");
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "metadata", derive(serde::Serialize, serde::Deserialize))]
pub struct LinkMetadata {
    /// The href URL value
    pub href: String,

    /// Link text content (normalized, concatenated if mixed with elements)
    pub text: String,

    /// Optional title attribute (often shown as tooltip)
    pub title: Option<String>,

    /// Link type classification
    pub link_type: LinkType,

    /// Rel attribute values (e.g., "nofollow", "stylesheet", "canonical")
    pub rel: Vec<String>,

    /// Additional HTML attributes
    pub attributes: BTreeMap<String, String>,
}

impl LinkMetadata {
    /// Classify a link based on href value.
    ///
    /// # Arguments
    ///
    /// * `href` - The href attribute value
    ///
    /// # Returns
    ///
    /// Appropriate [`LinkType`] based on protocol and content.
    ///
    /// # Examples
    ///
    /// ```
    /// # use html_to_markdown_rs::metadata::{LinkMetadata, LinkType};
    /// assert_eq!(LinkMetadata::classify_link("#section"), LinkType::Anchor);
    /// assert_eq!(LinkMetadata::classify_link("mailto:test@example.com"), LinkType::Email);
    /// assert_eq!(LinkMetadata::classify_link("tel:+1234567890"), LinkType::Phone);
    /// assert_eq!(LinkMetadata::classify_link("https://example.com"), LinkType::External);
    /// ```
    #[must_use]
    #[cfg_attr(alef, alef(skip))]
    pub fn classify_link(href: &str) -> LinkType {
        if href.starts_with('#') {
            LinkType::Anchor
        } else if href.starts_with("mailto:") {
            LinkType::Email
        } else if href.starts_with("tel:") {
            LinkType::Phone
        } else if href.starts_with("http://") || href.starts_with("https://") {
            LinkType::External
        } else if href.starts_with('/') || href.starts_with("../") || href.starts_with("./") {
            LinkType::Internal
        } else {
            LinkType::Other
        }
    }
}

/// Image metadata with source and dimensions.
///
/// Captures `<img>` elements and inline `<svg>` elements with metadata
/// for image analysis and optimization.
///
/// # Examples
///
/// ```
/// # use html_to_markdown_rs::metadata::{ImageMetadata, ImageType};
/// let img = ImageMetadata {
///     src: "https://example.com/image.jpg".to_string(),
///     alt: Some("An example image".to_string()),
///     title: Some("Example".to_string()),
///     dimensions: Some((800, 600)),
///     image_type: ImageType::External,
///     attributes: Default::default(),
/// };
///
/// assert_eq!(img.image_type, ImageType::External);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "metadata", derive(serde::Serialize, serde::Deserialize))]
pub struct ImageMetadata {
    /// Image source (URL, data URI, or SVG content identifier)
    pub src: String,

    /// Alternative text from alt attribute (for accessibility)
    pub alt: Option<String>,

    /// Title attribute (often shown as tooltip)
    pub title: Option<String>,

    /// Image dimensions as (width, height) if available
    pub dimensions: Option<(u32, u32)>,

    /// Image type classification
    pub image_type: ImageType,

    /// Additional HTML attributes
    pub attributes: BTreeMap<String, String>,
}

/// Structured data block (JSON-LD, Microdata, or RDFa).
///
/// Represents machine-readable structured data found in the document.
/// JSON-LD blocks are collected as raw JSON strings for flexibility.
///
/// # Examples
///
/// ```
/// # use html_to_markdown_rs::metadata::{StructuredData, StructuredDataType};
/// let schema = StructuredData {
///     data_type: StructuredDataType::JsonLd,
///     raw_json: r#"{"@context":"https://schema.org","@type":"Article"}"#.to_string(),
///     schema_type: Some("Article".to_string()),
/// };
///
/// assert_eq!(schema.data_type, StructuredDataType::JsonLd);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "metadata", derive(serde::Serialize, serde::Deserialize))]
pub struct StructuredData {
    /// Type of structured data (JSON-LD, Microdata, RDFa)
    pub data_type: StructuredDataType,

    /// Raw JSON string (for JSON-LD) or serialized representation
    pub raw_json: String,

    /// Schema type if detectable (e.g., "Article", "Event", "Product")
    pub schema_type: Option<String>,
}

/// Comprehensive metadata extraction result from HTML document.
///
/// Contains all extracted metadata types in a single structure,
/// suitable for serialization and transmission across language boundaries.
///
/// # Examples
///
/// ```
/// # use html_to_markdown_rs::metadata::HtmlMetadata;
/// let metadata = HtmlMetadata {
///     document: Default::default(),
///     headers: Vec::new(),
///     links: Vec::new(),
///     images: Vec::new(),
///     structured_data: Vec::new(),
/// };
///
/// assert!(metadata.headers.is_empty());
/// ```
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "metadata", derive(serde::Serialize, serde::Deserialize))]
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
