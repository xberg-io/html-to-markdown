//! The primary result type for HTML conversion and extraction.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::document::DocumentStructure;
use super::tables::TableData;
use super::warnings::ProcessingWarning;

/// The primary result of HTML conversion and extraction.
///
/// Contains the converted text output, optional structured document tree,
/// metadata, extracted tables, images, and processing warnings.
///
/// # Example
///
/// ```text
/// use html_to_markdown_rs::{convert, ConversionOptions};
///
/// let result = convert("<h1>Hello</h1><p>World</p>", None)?;
/// assert!(result.content.is_some());
/// assert!(result.warnings.is_empty());
/// ```
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConversionResult {
    /// Converted text output (markdown, djot, or plain text).
    ///
    /// `None` when `output_format` is set to `OutputFormat::None`,
    /// indicating extraction-only mode.
    pub content: Option<String>,

    /// Structured document tree with semantic elements.
    ///
    /// Populated when `ConversionOptions::include_document_structure` is `true`. `None`
    /// otherwise (the default), which avoids the overhead of building the tree.
    ///
    /// When present, the tree mirrors the converted document: headings open
    /// [`crate::types::document::NodeContent::Group`] sections, paragraphs and list items carry
    /// inline [`crate::types::document::TextAnnotation`]s, and tables reference the same
    /// [`crate::types::tables::TableGrid`] data exposed in [`Self::tables`].
    ///
    /// Note: this field is independent of the `metadata` feature flag. Document structure
    /// collection is always available at runtime; it is gated only by the runtime option, not
    /// by a compile-time feature.
    pub document: Option<DocumentStructure>,

    /// Extracted HTML metadata (title, OG, links, images, structured data).
    #[cfg(feature = "metadata")]
    pub metadata: crate::metadata::HtmlMetadata,

    /// Extracted tables with structured cell data and markdown representation.
    pub tables: Vec<TableData>,

    /// Extracted inline images (data URIs and SVGs).
    ///
    /// Populated when `extract_images` is `true` in options.
    #[cfg(feature = "inline-images")]
    #[cfg_attr(feature = "serde", serde(skip))]
    pub images: Vec<crate::inline_images::InlineImage>,

    /// Non-fatal processing warnings.
    pub warnings: Vec<ProcessingWarning>,
}
