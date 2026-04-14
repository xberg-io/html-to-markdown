//! The primary result type for HTML conversion and extraction.

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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    #[cfg(feature = "metadata")]
    pub metadata: crate::metadata::HtmlMetadata,

    /// Extracted tables with structured cell data and markdown representation.
    pub tables: Vec<TableData>,

    /// Extracted inline images (data URIs and SVGs).
    ///
    /// Populated when `extract_images` is `true` in options.
    #[cfg(feature = "inline-images")]
    #[serde(skip)]
    pub images: Vec<crate::inline_images::InlineImage>,

    /// Non-fatal processing warnings.
    pub warnings: Vec<ProcessingWarning>,
}
