//! Metadata extraction configuration.

/// Default maximum size for structured data extraction (1 MB)
pub const DEFAULT_MAX_STRUCTURED_DATA_SIZE: usize = 1_000_000;

/// Configuration for metadata extraction granularity.
///
/// Controls which metadata types are extracted and size limits for safety.
/// Enables selective extraction of different metadata categories from HTML documents,
/// allowing fine-grained control over which types of information to collect during
/// the HTML-to-Markdown conversion process.
///
/// # Fields
///
/// - `extract_document`: Enable document-level metadata extraction (title, description, author, Open Graph, Twitter Card, etc.)
/// - `extract_headers`: Enable heading element extraction (h1-h6) with hierarchy tracking
/// - `extract_links`: Enable anchor element extraction with link type classification
/// - `extract_images`: Enable image element extraction with source and dimension metadata
/// - `extract_structured_data`: Enable structured data extraction (JSON-LD, Microdata, RDFa)
/// - `max_structured_data_size`: Safety limit on total structured data size in bytes
///
/// # Examples
///
/// ```
/// # use html_to_markdown_rs::metadata::MetadataConfig;
/// let config = MetadataConfig {
///     extract_document: true,
///     extract_headers: true,
///     extract_links: true,
///     extract_images: true,
///     extract_structured_data: true,
///     max_structured_data_size: 1_000_000,
/// };
///
/// assert!(config.extract_headers);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "metadata", derive(serde::Serialize, serde::Deserialize))]
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
    pub max_structured_data_size: usize,
}

/// Partial update for `MetadataConfig`.
///
/// This struct uses `Option<T>` to represent optional fields that can be selectively updated.
/// Only specified fields (Some values) will override existing config; None values leave the
/// corresponding fields unchanged when applied via [`MetadataConfig::apply_update`].
///
/// # Fields
///
/// - `extract_document`: Optional override for document-level metadata extraction
/// - `extract_headers`: Optional override for heading element extraction
/// - `extract_links`: Optional override for link element extraction
/// - `extract_images`: Optional override for image element extraction
/// - `extract_structured_data`: Optional override for structured data extraction
/// - `max_structured_data_size`: Optional override for structured data size limit
///
/// # Examples
///
/// ```
/// # use html_to_markdown_rs::metadata::{MetadataConfig, MetadataConfigUpdate};
/// let update = MetadataConfigUpdate {
///     extract_document: Some(false),
///     extract_headers: Some(true),
///     extract_links: None,  // No change
///     extract_images: None,  // No change
///     extract_structured_data: None,  // No change
///     max_structured_data_size: None,  // No change
/// };
///
/// let mut config = MetadataConfig::default();
/// config.apply_update(update);
/// assert!(!config.extract_document);
/// assert!(config.extract_headers);
/// ```
#[derive(Debug, Clone, Default)]
#[cfg_attr(any(feature = "serde", feature = "metadata"), derive(serde::Deserialize))]
#[cfg_attr(any(feature = "serde", feature = "metadata"), serde(deny_unknown_fields))]
pub struct MetadataConfigUpdate {
    /// Optional override for extracting document-level metadata.
    ///
    /// When Some(true), enables document metadata extraction; Some(false) disables it.
    /// None leaves the current setting unchanged.
    #[cfg_attr(any(feature = "serde", feature = "metadata"), serde(alias = "extract_document"))]
    pub extract_document: Option<bool>,

    /// Optional override for extracting heading elements (h1-h6).
    ///
    /// When Some(true), enables header extraction; Some(false) disables it.
    /// None leaves the current setting unchanged.
    #[cfg_attr(any(feature = "serde", feature = "metadata"), serde(alias = "extract_headers"))]
    pub extract_headers: Option<bool>,

    /// Optional override for extracting anchor (link) elements.
    ///
    /// When Some(true), enables link extraction; Some(false) disables it.
    /// None leaves the current setting unchanged.
    #[cfg_attr(any(feature = "serde", feature = "metadata"), serde(alias = "extract_links"))]
    pub extract_links: Option<bool>,

    /// Optional override for extracting image elements.
    ///
    /// When Some(true), enables image extraction; Some(false) disables it.
    /// None leaves the current setting unchanged.
    #[cfg_attr(any(feature = "serde", feature = "metadata"), serde(alias = "extract_images"))]
    pub extract_images: Option<bool>,

    /// Optional override for extracting structured data (JSON-LD, Microdata, RDFa).
    ///
    /// When Some(true), enables structured data extraction; Some(false) disables it.
    /// None leaves the current setting unchanged.
    #[cfg_attr(
        any(feature = "serde", feature = "metadata"),
        serde(alias = "extract_structured_data")
    )]
    pub extract_structured_data: Option<bool>,

    /// Optional override for maximum structured data collection size in bytes.
    ///
    /// When Some(size), sets the new size limit. None leaves the current limit unchanged.
    /// Use this to adjust safety thresholds for different documents.
    #[cfg_attr(
        any(feature = "serde", feature = "metadata"),
        serde(alias = "max_structured_data_size")
    )]
    pub max_structured_data_size: Option<usize>,
}

impl Default for MetadataConfig {
    /// Create default metadata configuration.
    ///
    /// Defaults to extracting all metadata types with 1MB limit on structured data.
    fn default() -> Self {
        Self {
            extract_document: true,
            extract_headers: true,
            extract_links: true,
            extract_images: true,
            extract_structured_data: true,
            max_structured_data_size: DEFAULT_MAX_STRUCTURED_DATA_SIZE,
        }
    }
}

impl MetadataConfig {
    /// Check if any metadata extraction is enabled.
    ///
    /// Returns `true` if at least one extraction category is enabled, `false` if all are disabled.
    /// This is useful for early exit optimization when the application doesn't need metadata.
    ///
    /// # Returns
    ///
    /// `true` if any of the extraction flags are enabled, `false` if all are disabled.
    ///
    /// # Examples
    ///
    /// ```
    /// # use html_to_markdown_rs::metadata::MetadataConfig;
    /// // All enabled
    /// let config = MetadataConfig::default();
    /// assert!(config.any_enabled());
    ///
    /// // Selectively enabled
    /// let config = MetadataConfig {
    ///     extract_headers: true,
    ///     extract_document: false,
    ///     extract_links: false,
    ///     extract_images: false,
    ///     extract_structured_data: false,
    ///     max_structured_data_size: 1_000_000,
    /// };
    /// assert!(config.any_enabled());
    ///
    /// // All disabled
    /// let config = MetadataConfig {
    ///     extract_document: false,
    ///     extract_headers: false,
    ///     extract_links: false,
    ///     extract_images: false,
    ///     extract_structured_data: false,
    ///     max_structured_data_size: 1_000_000,
    /// };
    /// assert!(!config.any_enabled());
    /// ```
    #[must_use]
    pub const fn any_enabled(&self) -> bool {
        self.extract_document
            || self.extract_headers
            || self.extract_links
            || self.extract_images
            || self.extract_structured_data
    }

    /// Apply a partial update to this metadata configuration.
    ///
    /// Any specified fields in the update (Some values) will override the current values.
    /// Unspecified fields (None) are left unchanged. This allows selective modification
    /// of configuration without affecting unrelated settings.
    ///
    /// # Arguments
    ///
    /// * `update` - Partial metadata config update with fields to override
    ///
    /// # Examples
    ///
    /// ```
    /// # use html_to_markdown_rs::metadata::{MetadataConfig, MetadataConfigUpdate};
    /// let mut config = MetadataConfig::default();
    /// // config starts with all extraction enabled
    ///
    /// let update = MetadataConfigUpdate {
    ///     extract_document: Some(false),
    ///     extract_images: Some(false),
    ///     // All other fields are None, so they won't change
    ///     ..Default::default()
    /// };
    ///
    /// config.apply_update(update);
    ///
    /// assert!(!config.extract_document);
    /// assert!(!config.extract_images);
    /// assert!(config.extract_headers);  // Unchanged
    /// assert!(config.extract_links);    // Unchanged
    /// ```
    pub const fn apply_update(&mut self, update: MetadataConfigUpdate) {
        if let Some(extract_document) = update.extract_document {
            self.extract_document = extract_document;
        }
        if let Some(extract_headers) = update.extract_headers {
            self.extract_headers = extract_headers;
        }
        if let Some(extract_links) = update.extract_links {
            self.extract_links = extract_links;
        }
        if let Some(extract_images) = update.extract_images {
            self.extract_images = extract_images;
        }
        if let Some(extract_structured_data) = update.extract_structured_data {
            self.extract_structured_data = extract_structured_data;
        }
        if let Some(max_structured_data_size) = update.max_structured_data_size {
            self.max_structured_data_size = max_structured_data_size;
        }
    }

    /// Create new metadata configuration from a partial update.
    ///
    /// Creates a new `MetadataConfig` struct with defaults, then applies the update.
    /// Fields not specified in the update (None) keep their default values.
    /// This is a convenience method for constructing a configuration from a partial specification
    /// without needing to explicitly call `.default()` first.
    ///
    /// # Arguments
    ///
    /// * `update` - Partial metadata config update with fields to set
    ///
    /// # Returns
    ///
    /// New `MetadataConfig` with specified updates applied to defaults
    ///
    /// # Examples
    ///
    /// ```
    /// # use html_to_markdown_rs::metadata::{MetadataConfig, MetadataConfigUpdate};
    /// let update = MetadataConfigUpdate {
    ///     extract_document: Some(false),
    ///     extract_headers: Some(true),
    ///     extract_links: Some(true),
    ///     extract_images: None,  // Will use default (true)
    ///     extract_structured_data: None,  // Will use default (true)
    ///     max_structured_data_size: None,  // Will use default (1MB)
    /// };
    ///
    /// let config = MetadataConfig::from_update(update);
    ///
    /// assert!(!config.extract_document);
    /// assert!(config.extract_headers);
    /// assert!(config.extract_links);
    /// assert!(config.extract_images);  // Default
    /// assert!(config.extract_structured_data);  // Default
    /// ```
    #[must_use]
    pub fn from_update(update: MetadataConfigUpdate) -> Self {
        let mut config = Self::default();
        config.apply_update(update);
        config
    }
}

impl From<MetadataConfigUpdate> for MetadataConfig {
    fn from(update: MetadataConfigUpdate) -> Self {
        Self::from_update(update)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_config_default() {
        let config = MetadataConfig::default();

        assert!(config.extract_headers);
        assert!(config.extract_links);
        assert!(config.extract_images);
        assert!(config.extract_structured_data);
        assert_eq!(config.max_structured_data_size, DEFAULT_MAX_STRUCTURED_DATA_SIZE);
    }

    #[test]
    fn test_metadata_config_any_enabled() {
        let all_enabled = MetadataConfig::default();
        assert!(all_enabled.any_enabled());

        let some_enabled = MetadataConfig {
            extract_headers: true,
            extract_document: false,
            extract_links: false,
            extract_images: false,
            extract_structured_data: false,
            max_structured_data_size: 1_000_000,
        };
        assert!(some_enabled.any_enabled());

        let none_enabled = MetadataConfig {
            extract_document: false,
            extract_headers: false,
            extract_links: false,
            extract_images: false,
            extract_structured_data: false,
            max_structured_data_size: 1_000_000,
        };
        assert!(!none_enabled.any_enabled());
    }
}
