// ~keep reason: MetadataCollector and its impl methods are only called when the metadata
// ~keep feature is active. The struct/impl is compiled regardless to keep the module
// ~keep structure consistent, but most methods are unused in non-metadata builds.
#![allow(dead_code)]

//! Metadata collector for single-pass extraction.

use super::config::MetadataConfig;
use super::extraction::{extract_document_metadata, extract_structured_data};
use super::types::{HtmlMetadata, ImageDimensions, ImageMetadata, ImageType, LinkMetadata};
use std::collections::BTreeMap;

/// Internal metadata collector for single-pass extraction.
///
/// Follows a pattern for efficient metadata extraction during tree traversal.
/// Maintains state for:
/// - Document metadata from head elements
/// - Header hierarchy tracking
/// - Link accumulation
/// - Structured data collection
/// - Language and directionality attributes
///
/// # Architecture
///
/// The collector is designed to be:
/// - **Performant**: Pre-allocated collections, minimal cloning
/// - **Single-pass**: Collects during main tree walk without separate passes
/// - **Optional**: Zero overhead when disabled via feature flags
/// - **Type-safe**: Strict separation of collection and result types
#[derive(Debug)]
pub struct MetadataCollector {
    pub(super) head_metadata: BTreeMap<String, String>,
    pub(super) headers: Vec<super::types::HeaderMetadata>,
    pub(super) header_stack: Vec<usize>,
    pub(super) links: Vec<LinkMetadata>,
    pub(super) images: Vec<ImageMetadata>,
    pub(super) json_ld: Vec<String>,
    pub(super) structured_data_size: usize,
    pub(super) config: MetadataConfig,
    pub(super) lang: Option<String>,
    pub(super) dir: Option<String>,
}

impl MetadataCollector {
    /// Create a new metadata collector with configuration.
    ///
    /// Pre-allocates collections based on typical document sizes
    /// for efficient append operations during traversal.
    ///
    /// # Arguments
    ///
    /// * `config` - Extraction configuration specifying which types to collect
    ///
    /// # Returns
    ///
    /// A new collector ready for use during tree traversal.
    pub(crate) fn new(config: MetadataConfig) -> Self {
        Self {
            head_metadata: BTreeMap::new(),
            headers: Vec::with_capacity(32),
            header_stack: Vec::with_capacity(6),
            links: Vec::with_capacity(64),
            images: Vec::with_capacity(16),
            json_ld: Vec::with_capacity(4),
            structured_data_size: 0,
            config,
            lang: None,
            dir: None,
        }
    }

    /// Add a header element to the collection.
    ///
    /// Validates that level is in range 1-6 and tracks hierarchy via depth.
    ///
    /// # Arguments
    ///
    /// * `level` - Header level (1-6)
    /// * `text` - Normalized header text content
    /// * `id` - Optional HTML id attribute
    /// * `depth` - Current document nesting depth
    /// * `html_offset` - Byte offset in original HTML
    pub(crate) fn add_header(&mut self, level: u8, text: String, id: Option<String>, depth: usize, html_offset: usize) {
        if !self.config.extract_headers {
            return;
        }

        if !(1..=6).contains(&level) {
            return;
        }

        let header = super::types::HeaderMetadata {
            level,
            text,
            id,
            depth,
            html_offset,
        };

        self.headers.push(header);
    }

    /// Add a link element to the collection.
    ///
    /// Classifies the link based on href value and stores with metadata.
    pub(crate) fn add_link(
        &mut self,
        href: String,
        text: String,
        title: Option<String>,
        rel: Option<String>,
        attributes: BTreeMap<String, String>,
    ) {
        if !self.config.extract_links {
            return;
        }

        let link_type = super::types::LinkMetadata::classify_link(&href);

        let rel_vec = rel
            .map(|r| {
                r.split_whitespace()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let link = LinkMetadata {
            href,
            text,
            title,
            link_type,
            rel: rel_vec,
            attributes,
        };

        self.links.push(link);
    }

    /// Add an image element to the collection.
    ///
    /// # Arguments
    ///
    /// * `src` - Image source (URL or data URI)
    /// * `alt` - Optional alt text
    /// * `title` - Optional title attribute
    /// * `dimensions` - Optional image dimensions
    pub(crate) fn add_image(
        &mut self,
        src: String,
        alt: Option<String>,
        title: Option<String>,
        dimensions: Option<ImageDimensions>,
        attributes: BTreeMap<String, String>,
    ) {
        if !self.config.extract_images {
            return;
        }

        let image_type = if crate::converter::media::is_inline_data(&src) {
            ImageType::DataUri
        } else if src.starts_with("http://") || src.starts_with("https://") {
            ImageType::External
        } else if src.starts_with('<') && src.contains("svg") {
            ImageType::InlineSvg
        } else {
            ImageType::Relative
        };

        let image = ImageMetadata {
            src,
            alt,
            title,
            dimensions,
            image_type,
            attributes,
        };

        self.images.push(image);
    }

    /// Add a JSON-LD structured data block.
    ///
    /// Accumulates JSON content with size validation against configured limits.
    pub(crate) fn add_json_ld(&mut self, json_content: String) {
        if !self.config.extract_structured_data {
            return;
        }

        let content_size = json_content.len();
        if content_size > self.config.max_structured_data_size {
            return;
        }
        if self.structured_data_size + content_size > self.config.max_structured_data_size {
            return;
        }

        self.structured_data_size += content_size;
        self.json_ld.push(json_content);
    }

    /// Set document head metadata from extracted head section.
    ///
    /// Merges metadata pairs from head elements (meta, title, link, etc.)
    /// into the collector's head metadata store.
    pub(crate) fn set_head_metadata(&mut self, metadata: BTreeMap<String, String>) {
        if !self.config.extract_document {
            return;
        }
        self.head_metadata.extend(metadata);
    }

    /// Set document language attribute.
    ///
    /// Usually from `lang` attribute on `<html>` or `<body>` tag.
    /// Only sets if not already set (first occurrence wins).
    pub(crate) fn set_language(&mut self, lang: String) {
        if !self.config.extract_document {
            return;
        }
        if self.lang.is_none() {
            self.lang = Some(lang);
        }
    }

    /// Set document text direction attribute.
    ///
    /// Usually from `dir` attribute on `<html>` or `<body>` tag.
    /// Only sets if not already set (first occurrence wins).
    pub(crate) fn set_text_direction(&mut self, dir: String) {
        if !self.config.extract_document {
            return;
        }
        if self.dir.is_none() {
            self.dir = Some(dir);
        }
    }

    pub(crate) const fn wants_document(&self) -> bool {
        self.config.extract_document
    }

    pub(crate) const fn wants_headers(&self) -> bool {
        self.config.extract_headers
    }

    pub(crate) const fn wants_links(&self) -> bool {
        self.config.extract_links
    }

    pub(crate) const fn wants_images(&self) -> bool {
        self.config.extract_images
    }

    pub(crate) const fn wants_structured_data(&self) -> bool {
        self.config.extract_structured_data
    }

    /// Finish collection and return all extracted metadata.
    ///
    /// Performs final processing, validation, and consolidation of all
    /// collected data into the [`HtmlMetadata`] output structure.
    pub(crate) fn finish(self) -> HtmlMetadata {
        let structured_data = extract_structured_data(self.json_ld);
        let document = extract_document_metadata(self.head_metadata, self.lang, self.dir);

        HtmlMetadata {
            document,
            headers: self.headers,
            links: self.links,
            images: self.images,
            structured_data,
        }
    }

    /// Categorize links by type for analysis and filtering.
    ///
    /// Separates collected links into groups by [`LinkType`](super::types::LinkType).
    pub(crate) fn categorize_links(&self) -> BTreeMap<String, Vec<&LinkMetadata>> {
        let mut categorized: BTreeMap<String, Vec<&LinkMetadata>> = BTreeMap::new();

        for link in &self.links {
            let category = link.link_type.to_string();
            categorized.entry(category).or_default().push(link);
        }

        categorized
    }

    /// Count headers by level for structural analysis.
    ///
    /// Returns count of headers at each level (1-6).
    pub(crate) fn header_counts(&self) -> BTreeMap<String, usize> {
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();

        for header in &self.headers {
            *counts.entry(header.level.to_string()).or_insert(0) += 1;
        }

        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_collector_new() {
        let config = MetadataConfig::default();
        let collector = MetadataCollector::new(config);

        assert_eq!(collector.headers.capacity(), 32);
        assert_eq!(collector.links.capacity(), 64);
        assert_eq!(collector.images.capacity(), 16);
        assert_eq!(collector.json_ld.capacity(), 4);
    }

    #[test]
    fn test_metadata_collector_add_header() {
        let config = MetadataConfig::default();
        let mut collector = MetadataCollector::new(config);

        collector.add_header(1, "Title".to_string(), Some("title".to_string()), 0, 100);
        assert_eq!(collector.headers.len(), 1);

        let header = &collector.headers[0];
        assert_eq!(header.level, 1);
        assert_eq!(header.text, "Title");
        assert_eq!(header.id, Some("title".to_string()));

        collector.add_header(7, "Invalid".to_string(), None, 0, 200);
        assert_eq!(collector.headers.len(), 1);
    }

    #[test]
    fn test_metadata_collector_add_link() {
        let config = MetadataConfig::default();
        let mut collector = MetadataCollector::new(config);

        collector.add_link(
            "https://example.com".to_string(),
            "Example".to_string(),
            Some("Visit".to_string()),
            Some("nofollow external".to_string()),
            BTreeMap::from([("data-id".to_string(), "example".to_string())]),
        );

        assert_eq!(collector.links.len(), 1);

        let link = &collector.links[0];
        assert_eq!(link.href, "https://example.com");
        assert_eq!(link.text, "Example");
    }

    #[test]
    fn test_metadata_collector_respects_config() {
        let config = MetadataConfig {
            extract_document: false,
            extract_headers: false,
            extract_links: false,
            extract_images: false,
            extract_structured_data: false,
            max_structured_data_size: 1_000_000,
        };
        let mut collector = MetadataCollector::new(config);

        collector.add_header(1, "Title".to_string(), None, 0, 100);
        collector.add_link(
            "https://example.com".to_string(),
            "Link".to_string(),
            None,
            None,
            BTreeMap::new(),
        );
        collector.add_image(
            "https://example.com/img.jpg".to_string(),
            None,
            None,
            None,
            BTreeMap::new(),
        );
        collector.add_json_ld("{}".to_string());

        assert!(collector.headers.is_empty());
        assert!(collector.links.is_empty());
        assert!(collector.images.is_empty());
        assert!(collector.json_ld.is_empty());
    }

    #[test]
    fn test_metadata_collector_finish() {
        let config = MetadataConfig::default();
        let mut collector = MetadataCollector::new(config);

        collector.set_language("en".to_string());
        collector.add_header(1, "Main Title".to_string(), None, 0, 100);
        collector.add_link(
            "https://example.com".to_string(),
            "Example".to_string(),
            None,
            None,
            BTreeMap::new(),
        );

        let metadata = collector.finish();

        assert_eq!(metadata.document.language, Some("en".to_string()));
        assert_eq!(metadata.headers.len(), 1);
        assert_eq!(metadata.links.len(), 1);
    }

    #[test]
    fn test_categorize_links() {
        let config = MetadataConfig::default();
        let mut collector = MetadataCollector::new(config);

        collector.add_link("#anchor".to_string(), "Anchor".to_string(), None, None, BTreeMap::new());
        collector.add_link(
            "https://example.com".to_string(),
            "External".to_string(),
            None,
            None,
            BTreeMap::new(),
        );
        collector.add_link(
            "mailto:test@example.com".to_string(),
            "Email".to_string(),
            None,
            None,
            BTreeMap::new(),
        );

        let categorized = collector.categorize_links();

        assert_eq!(categorized.get("anchor").map(std::vec::Vec::len), Some(1));
        assert_eq!(categorized.get("external").map(std::vec::Vec::len), Some(1));
        assert_eq!(categorized.get("email").map(std::vec::Vec::len), Some(1));
    }

    #[test]
    fn test_header_counts() {
        let config = MetadataConfig::default();
        let mut collector = MetadataCollector::new(config);

        collector.add_header(1, "H1".to_string(), None, 0, 100);
        collector.add_header(2, "H2".to_string(), None, 1, 200);
        collector.add_header(2, "H2b".to_string(), None, 1, 300);
        collector.add_header(3, "H3".to_string(), None, 2, 400);

        let counts = collector.header_counts();

        assert_eq!(counts.get("1").copied(), Some(1));
        assert_eq!(counts.get("2").copied(), Some(2));
        assert_eq!(counts.get("3").copied(), Some(1));
    }
}
