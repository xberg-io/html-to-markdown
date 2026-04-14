#![allow(clippy::cast_precision_loss, clippy::cast_sign_loss, clippy::unused_self)]
//! Metadata extraction for HTML to Markdown conversion.
//!
//! This module provides comprehensive, type-safe metadata extraction during HTML-to-Markdown
//! conversion, enabling content analysis, SEO optimization, and document indexing workflows.
//! Metadata includes:
//! - **Document metadata**: Title, description, author, language, canonical URL, Open Graph, Twitter Card
//! - **Headers**: Heading elements (h1-h6) with hierarchy, IDs, and positions
//! - **Links**: Hyperlinks with type classification (anchor, internal, external, email, phone)
//! - **Images**: Image elements with source, alt text, dimensions, and type (data URI, external, etc.)
//! - **Structured data**: JSON-LD, Microdata, and RDFa blocks
//!
//! The implementation follows a single-pass collector pattern for zero-overhead extraction
//! when metadata features are disabled.
//!
//! # Architecture
//!
//! Metadata extraction uses the [`MetadataCollector`] pattern:
//! - **Single-pass collection**: Metadata is gathered during the primary tree traversal without additional passes
//! - **Zero overhead when disabled**: Entire module can be compiled out via feature flags
//! - **Configurable granularity**: Use [`MetadataConfig`] to select which metadata types to extract
//! - **Type-safe APIs**: All metadata types are enum-based with exhaustive matching
//! - **Memory-bounded**: Size limits prevent memory exhaustion from adversarial documents
//! - **Pre-allocated buffers**: Typical documents (32 headers, 64 links, 16 images) handled efficiently
//!
//! # Type Overview
//!
//! ## Enumerations
//!
//! - [`TextDirection`]: Document directionality (LTR, RTL, Auto)
//! - [`LinkType`]: Link classification (Anchor, Internal, External, Email, Phone, Other)
//! - [`ImageType`]: Image source type (DataUri, External, Relative, InlineSvg)
//! - [`StructuredDataType`]: Structured data format (JsonLd, Microdata, RDFa)
//!
//! ## Structures
//!
//! - [`DocumentMetadata`]: Head-level metadata with maps for Open Graph and Twitter Card
//! - [`HeaderMetadata`]: Heading element with level (1-6), text, ID, hierarchy depth, and position
//! - [`LinkMetadata`]: Hyperlink with href, text, title, type, rel attributes, and custom attributes
//! - [`ImageMetadata`]: Image element with src, alt, title, dimensions, type, and attributes
//! - [`StructuredData`]: Structured data block with type and raw JSON
//! - [`MetadataConfig`]: Configuration controlling extraction granularity and size limits
//! - [`HtmlMetadata`]: Top-level result containing all extracted metadata
//!
//! # Examples
//!
//! ## Basic Usage with `convert()`
//!
//! ```text
//! use html_to_markdown_rs::convert;
//!
//! let html = r#"
//!   <html lang="en">
//!     <head>
//!       <title>My Article</title>
//!       <meta name="description" content="An interesting read">
//!     </head>
//!     <body>
//!       <h1 id="main">Title</h1>
//!       <a href="https://example.com">External Link</a>
//!       <img src="photo.jpg" alt="A photo">
//!     </body>
//!   </html>
//! "#;
//!
//! let result = convert(html, None)?;
//! let metadata = result.metadata.unwrap();
//!
//! // Access document metadata
//! assert_eq!(metadata.document.title, Some("My Article".to_string()));
//! assert_eq!(metadata.document.language, Some("en".to_string()));
//!
//! // Access headers
//! assert_eq!(metadata.headers.len(), 1);
//! assert_eq!(metadata.headers[0].level, 1);
//! assert_eq!(metadata.headers[0].id, Some("main".to_string()));
//!
//! // Access links
//! assert_eq!(metadata.links.len(), 1);
//! assert_eq!(metadata.links[0].link_type, LinkType::External);
//!
//! // Access images
//! assert_eq!(metadata.images.len(), 1);
//! assert_eq!(metadata.images[0].image_type, ImageType::Relative);
//! # Ok::<(), html_to_markdown_rs::ConversionError>(())
//! ```
//!
//! ## Selective Extraction
//!
//! ```text
//! use html_to_markdown_rs::{convert, ConversionOptions};
//!
//! let options = ConversionOptions {
//!     extract_metadata: false,  // Disable metadata extraction
//!     ..Default::default()
//! };
//!
//! let result = convert(html, Some(options))?;
//! assert!(result.metadata.is_none());  // Metadata not extracted
//! # Ok::<(), html_to_markdown_rs::ConversionError>(())
//! ```
//!
//! ## Analyzing Link Types
//!
//! ```text
//! use html_to_markdown_rs::convert;
//! use html_to_markdown_rs::metadata::LinkType;
//!
//! let result = convert(html, None)?;
//! let metadata = result.metadata.unwrap();
//!
//! for link in &metadata.links {
//!     match link.link_type {
//!         LinkType::External => println!("External: {}", link.href),
//!         LinkType::Internal => println!("Internal: {}", link.href),
//!         LinkType::Anchor => println!("Anchor: {}", link.href),
//!         LinkType::Email => println!("Email: {}", link.href),
//!         _ => {}
//!     }
//! }
//! # Ok::<(), html_to_markdown_rs::ConversionError>(())
//! ```
//!
//! # Serialization
//!
//! All types in this module support serialization via `serde` when the `metadata` feature is enabled.
//! This enables easy export to JSON, YAML, or other formats:
//!
//! ```text
//! use html_to_markdown_rs::convert;
//!
//! let result = convert(html, None)?;
//! if let Some(metadata) = &result.metadata {
//!     let json = serde_json::to_string_pretty(metadata)?;
//!     println!("{}", json);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod collector;
pub mod config;
pub mod extraction;
pub mod types;

// Re-export public types
pub use collector::MetadataCollector;
pub use config::{DEFAULT_MAX_STRUCTURED_DATA_SIZE, MetadataConfig, MetadataConfigUpdate};
pub use types::{
    DocumentMetadata, HeaderMetadata, HtmlMetadata, ImageMetadata, ImageType, LinkMetadata, LinkType, StructuredData,
    StructuredDataType, TextDirection,
};

// Internal handle type for shared mutable access during tree traversal
use std::cell::RefCell;
use std::rc::Rc;

/// Handle to a metadata collector via reference-counted mutable cell.
///
/// Used internally for sharing collector state across the tree traversal.
///
/// # Examples
///
/// ```text
/// let collector = MetadataCollector::new(MetadataConfig::default());
/// let handle = Rc::new(RefCell::new(collector));
///
/// // In tree walk, can be passed and borrowed
/// handle.borrow_mut().add_header(1, "Title".to_string(), None, 0, 100);
///
/// let metadata = handle.take().finish();
/// ```
#[allow(dead_code)]
pub(crate) type MetadataCollectorHandle = Rc<RefCell<MetadataCollector>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_direction_parse() {
        assert_eq!(TextDirection::parse("ltr"), Some(TextDirection::LeftToRight));
        assert_eq!(TextDirection::parse("rtl"), Some(TextDirection::RightToLeft));
        assert_eq!(TextDirection::parse("auto"), Some(TextDirection::Auto));
        assert_eq!(TextDirection::parse("invalid"), None);
        assert_eq!(TextDirection::parse("LTR"), Some(TextDirection::LeftToRight));
    }

    #[test]
    fn test_text_direction_display() {
        assert_eq!(TextDirection::LeftToRight.to_string(), "ltr");
        assert_eq!(TextDirection::RightToLeft.to_string(), "rtl");
        assert_eq!(TextDirection::Auto.to_string(), "auto");
    }

    #[test]
    fn test_link_classification() {
        assert_eq!(LinkMetadata::classify_link("#section"), LinkType::Anchor);
        assert_eq!(LinkMetadata::classify_link("mailto:test@example.com"), LinkType::Email);
        assert_eq!(LinkMetadata::classify_link("tel:+1234567890"), LinkType::Phone);
        assert_eq!(LinkMetadata::classify_link("https://example.com"), LinkType::External);
        assert_eq!(LinkMetadata::classify_link("http://example.com"), LinkType::External);
        assert_eq!(LinkMetadata::classify_link("/path/to/page"), LinkType::Internal);
        assert_eq!(LinkMetadata::classify_link("../relative"), LinkType::Internal);
        assert_eq!(LinkMetadata::classify_link("./same"), LinkType::Internal);
    }

    #[test]
    fn test_header_validation() {
        let valid = HeaderMetadata {
            level: 3,
            text: "Title".to_string(),
            id: None,
            depth: 2,
            html_offset: 100,
        };
        assert!(valid.is_valid());

        let invalid_high = HeaderMetadata {
            level: 7,
            text: "Title".to_string(),
            id: None,
            depth: 2,
            html_offset: 100,
        };
        assert!(!invalid_high.is_valid());

        let invalid_low = HeaderMetadata {
            level: 0,
            text: "Title".to_string(),
            id: None,
            depth: 2,
            html_offset: 100,
        };
        assert!(!invalid_low.is_valid());
    }

    #[test]
    fn test_document_metadata_default() {
        let doc = DocumentMetadata::default();

        assert!(doc.title.is_none());
        assert!(doc.description.is_none());
        assert!(doc.keywords.is_empty());
        assert!(doc.open_graph.is_empty());
        assert!(doc.twitter_card.is_empty());
        assert!(doc.meta_tags.is_empty());
    }

    #[test]
    fn test_image_type_classification() {
        let data_uri = ImageMetadata {
            src: "data:image/png;base64,iVBORw0KG...".to_string(),
            alt: None,
            title: None,
            dimensions: None,
            image_type: ImageType::DataUri,
            attributes: Default::default(),
        };
        assert_eq!(data_uri.image_type, ImageType::DataUri);

        let external = ImageMetadata {
            src: "https://example.com/image.jpg".to_string(),
            alt: None,
            title: None,
            dimensions: None,
            image_type: ImageType::External,
            attributes: Default::default(),
        };
        assert_eq!(external.image_type, ImageType::External);
    }

    #[test]
    fn test_link_type_display() {
        assert_eq!(LinkType::Anchor.to_string(), "anchor");
        assert_eq!(LinkType::Internal.to_string(), "internal");
        assert_eq!(LinkType::External.to_string(), "external");
        assert_eq!(LinkType::Email.to_string(), "email");
        assert_eq!(LinkType::Phone.to_string(), "phone");
        assert_eq!(LinkType::Other.to_string(), "other");
    }

    #[test]
    fn test_structured_data_type_display() {
        assert_eq!(StructuredDataType::JsonLd.to_string(), "json_ld");
        assert_eq!(StructuredDataType::Microdata.to_string(), "microdata");
        assert_eq!(StructuredDataType::RDFa.to_string(), "rdfa");
    }
}
