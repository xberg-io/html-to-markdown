//! Conversion context for HTML to Markdown conversion.
//!
//! The `Context` struct maintains state during the recursive tree walk that transforms
//! HTML to Markdown. It tracks nesting levels, element types, and feature-specific collectors
//! that are passed through the conversion pipeline.

#[cfg(any(feature = "inline-images", feature = "visitor"))]
use std::cell::RefCell;
use std::collections::{BTreeMap, HashSet};
use std::rc::Rc;

#[cfg(feature = "inline-images")]
use crate::inline_images::InlineImageCollector;

use crate::converter::reference_collector::ReferenceCollectorHandle;
use crate::types::structure_collector::StructureCollectorHandle;

/// Handle type for inline image collector when feature is enabled.
#[cfg(feature = "inline-images")]
pub type InlineCollectorHandle = Rc<RefCell<InlineImageCollector>>;
/// Placeholder type when inline-images feature is disabled.
#[cfg(not(feature = "inline-images"))]
pub type InlineCollectorHandle = ();

/// Payload type for image metadata extraction.
#[cfg(feature = "metadata")]
pub type ImageMetadataPayload = (BTreeMap<String, String>, Option<u32>, Option<u32>);

/// Conversion context that tracks state during HTML to Markdown conversion.
///
/// This context is passed through the recursive tree walker and maintains information
/// about the current position in the document tree, nesting levels, and enabled features.
#[derive(Clone)]
pub struct Context {
    /// Are we inside a code-like element (pre, code, kbd, samp)?
    pub(crate) in_code: bool,
    /// Current list item counter for ordered lists
    pub(crate) list_counter: usize,
    /// Are we in an ordered list (vs unordered)?
    pub(crate) in_ordered_list: bool,
    /// Blockquote nesting depth
    pub(crate) blockquote_depth: usize,
    /// Are we inside a table cell (td/th)?
    pub(crate) in_table_cell: bool,
    /// Should we convert block elements as inline?
    pub(crate) convert_as_inline: bool,
    /// Depth of inline formatting elements (strong/emphasis/span/etc).
    pub(crate) inline_depth: usize,
    /// Are we inside a list item?
    pub(crate) in_list_item: bool,
    /// List nesting depth (for indentation)
    pub(crate) list_depth: usize,
    /// Unordered list nesting depth (for bullet cycling)
    pub(crate) ul_depth: usize,
    /// Are we inside any list (ul or ol)?
    pub(crate) in_list: bool,
    /// Is this a "loose" list where all items should have blank lines?
    pub(crate) loose_list: bool,
    /// Did a previous list item have block children?
    pub(crate) prev_item_had_blocks: bool,
    /// Are we inside a heading element (h1-h6)?
    pub(crate) in_heading: bool,
    /// Whether inline images should remain markdown inside the current heading.
    pub(crate) heading_allow_inline_images: bool,
    /// Are we inside a paragraph element?
    pub(crate) in_paragraph: bool,
    /// Output buffer position where the current block's content starts.
    /// Used to distinguish paragraph-break newlines from a previous block
    /// vs. newlines generated within the current block.
    pub(crate) block_content_start: usize,
    /// Are we inside a ruby element?
    pub(crate) in_ruby: bool,
    /// Are we inside a `<strong>` / `<b>` element?
    pub(crate) in_strong: bool,
    /// Are we inside a link element (collecting link label text)?
    pub(crate) in_link: bool,
    /// Tag names that should be stripped during conversion.
    pub(crate) strip_tags: Rc<HashSet<String>>,
    /// Tag names that should be preserved as raw HTML.
    pub(crate) preserve_tags: Rc<HashSet<String>>,
    /// Tag names that allow inline images inside headings.
    pub(crate) keep_inline_images_in: Rc<HashSet<String>>,
    #[cfg(feature = "inline-images")]
    /// Shared collector for inline images when enabled.
    pub(crate) inline_collector: Option<InlineCollectorHandle>,
    #[cfg(feature = "metadata")]
    /// Shared collector for metadata when enabled.
    pub(crate) metadata_collector: Option<crate::metadata::MetadataCollectorHandle>,
    #[cfg(feature = "metadata")]
    pub(crate) metadata_wants_document: bool,
    #[cfg(feature = "metadata")]
    pub(crate) metadata_wants_headers: bool,
    #[cfg(feature = "metadata")]
    pub(crate) metadata_wants_links: bool,
    #[cfg(feature = "metadata")]
    pub(crate) metadata_wants_images: bool,
    #[cfg(feature = "metadata")]
    pub(crate) metadata_wants_structured_data: bool,
    #[cfg(feature = "visitor")]
    /// Optional visitor for custom HTML traversal callbacks.
    pub(crate) visitor: Option<crate::visitor::VisitorHandle>,
    #[cfg(feature = "visitor")]
    /// Stores the first visitor error encountered during traversal.
    pub(crate) visitor_error: Rc<RefCell<Option<String>>>,
    /// Set when DOM depth exceeds `max_depth` during traversal.
    pub(crate) depth_exceeded: std::cell::Cell<bool>,
    /// Optional structure collector for building a [`crate::types::DocumentStructure`].
    ///
    /// Populated when `options.include_document_structure == true`.
    pub(crate) structure_collector: Option<StructureCollectorHandle>,
    /// Optional reference collector for reference-style links.
    pub(crate) reference_collector: Option<ReferenceCollectorHandle>,
}

impl Context {
    /// Create a new conversion context from options and optional collectors.
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(
        any(not(feature = "inline-images"), not(feature = "metadata"), not(feature = "visitor")),
        allow(unused_variables)
    )]
    pub fn new(
        options: &crate::options::ConversionOptions,
        inline_collector: Option<InlineCollectorHandle>,
        #[cfg(feature = "metadata")] metadata_collector: Option<crate::metadata::MetadataCollectorHandle>,
        #[cfg(not(feature = "metadata"))] _metadata_collector: Option<()>,
        #[cfg(feature = "visitor")] visitor: Option<crate::visitor::VisitorHandle>,
        #[cfg(not(feature = "visitor"))] _visitor: Option<()>,
        structure_collector: Option<StructureCollectorHandle>,
        reference_collector: Option<ReferenceCollectorHandle>,
    ) -> Self {
        #[cfg(feature = "metadata")]
        let (
            metadata_wants_document,
            metadata_wants_headers,
            metadata_wants_links,
            metadata_wants_images,
            metadata_wants_structured_data,
        ) = if let Some(ref collector) = metadata_collector {
            let guard = collector.borrow();
            (
                guard.wants_document(),
                guard.wants_headers(),
                guard.wants_links(),
                guard.wants_images(),
                guard.wants_structured_data(),
            )
        } else {
            (false, false, false, false, false)
        };

        Self {
            in_code: false,
            list_counter: 0,
            in_ordered_list: false,
            blockquote_depth: 0,
            in_table_cell: false,
            convert_as_inline: options.convert_as_inline,
            inline_depth: 0,
            in_list_item: false,
            list_depth: 0,
            ul_depth: 0,
            in_list: false,
            loose_list: false,
            prev_item_had_blocks: false,
            in_heading: false,
            heading_allow_inline_images: false,
            in_paragraph: false,
            block_content_start: 0,
            in_ruby: false,
            in_strong: false,
            in_link: false,
            strip_tags: Rc::new(options.strip_tags.iter().cloned().collect()),
            preserve_tags: Rc::new(options.preserve_tags.iter().cloned().collect()),
            keep_inline_images_in: Rc::new(options.keep_inline_images_in.iter().cloned().collect()),
            #[cfg(feature = "inline-images")]
            inline_collector,
            #[cfg(feature = "metadata")]
            metadata_collector,
            #[cfg(feature = "metadata")]
            metadata_wants_document,
            #[cfg(feature = "metadata")]
            metadata_wants_headers,
            #[cfg(feature = "metadata")]
            metadata_wants_links,
            #[cfg(feature = "metadata")]
            metadata_wants_images,
            #[cfg(feature = "metadata")]
            metadata_wants_structured_data,
            #[cfg(feature = "visitor")]
            visitor: visitor.clone(),
            #[cfg(feature = "visitor")]
            visitor_error: Rc::new(RefCell::new(None)),
            depth_exceeded: std::cell::Cell::new(false),
            structure_collector,
            reference_collector,
        }
    }
}
