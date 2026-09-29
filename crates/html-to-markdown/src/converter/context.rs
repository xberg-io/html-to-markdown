//! Conversion context for HTML to Markdown conversion.
//!
//! The `Context` struct maintains state during the recursive tree walk that transforms
//! HTML to Markdown. It tracks nesting levels, element types, and feature-specific collectors
//! that are passed through the conversion pipeline.

use std::cell::Cell;
#[cfg(any(feature = "inline-images", feature = "visitor"))]
use std::cell::RefCell;
#[cfg(feature = "metadata")]
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::rc::Rc;

#[cfg(feature = "inline-images")]
use crate::inline_images::InlineImageCollector;

use crate::converter::reference_collector::ReferenceCollectorHandle;
use crate::options::InlineDataMedia;
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
    /// Are we inside a `<pre>` code BLOCK specifically, as opposed to an inline code SPAN
    /// (`<code>`/`<kbd>`/`<samp>` outside a `<pre>`)?
    ///
    /// Set only by `handle_pre` (`handlers/code_block.rs`); `handle_code` and
    /// `inline::code::handle` never set it themselves, so it propagates unchanged through
    /// their `..ctx.clone()` spread into any code nested inside a `<pre>`. `line_break.rs`
    /// needs this distinction that `in_code` alone cannot make: a `<br>` inside a code
    /// BLOCK keeps its literal `\n` (real content structure), while a `<br>` inside a code
    /// SPAN must split the span in two rather than embed a newline inside the backticks
    /// (issue #487). ~keep
    pub(crate) in_code_block: bool,
    /// Current list item counter for ordered lists.
    ///
    /// Signed so a negative `start` attribute (permitted by the HTML spec and honored by
    /// browsers) can be represented without a separate sign flag.
    pub(crate) list_counter: i64,
    /// Are we in an ordered list (vs unordered)?
    pub(crate) in_ordered_list: bool,
    /// Blockquote nesting depth
    pub(crate) blockquote_depth: usize,
    /// The `list_indent_columns` where the innermost quote starts: a line in the quote's buffer
    /// is indented only by the columns of the items inside the quote, and the quote writes the
    /// indent of the items around it.
    pub(crate) quote_list_columns: usize,
    /// Are we inside a table cell (td/th)?
    pub(crate) in_table_cell: bool,
    /// Are we inside a *layout*-table cell, whose row renders as a list item rather than a
    /// pipe row?
    ///
    /// Such a cell converts as inline, so neither the ordinary block separator nor the
    /// `in_table_cell` continuation rule applied and adjacent `<p>`/`<div>` children abutted
    /// (issue #470). This flag routes only the sibling-boundary decision through
    /// `emit_table_cell_break`; it deliberately does not enable `in_table_cell`'s pipe and
    /// emphasis escaping, which a list item does not need.
    pub(crate) in_layout_cell: bool,
    /// Should we convert block elements as inline?
    pub(crate) convert_as_inline: bool,
    /// Depth of inline formatting elements (strong/emphasis/span/etc).
    pub(crate) inline_depth: usize,
    /// Are we inside a list item?
    pub(crate) in_list_item: bool,
    /// Whether the list item is still open where the current output buffer will be written:
    /// a block written after other content then starts at the item's content column.
    ///
    /// ~keep A container that renders its children into a buffer of its own (a definition
    /// ~keep list, a sectioning element, a figure, a details element, a form) writes that buffer
    /// ~keep as a whole, so the buffer cannot show whether the item has already ended. The
    /// ~keep container works that out from its own output before it renders the children, or
    /// ~keep passes false when it writes at the start of the line (issue #583).
    pub(crate) list_item_open: bool,
    /// Whether the current output buffer is written between inline markers (a summary's `**`,
    /// a caption's `*`): the first line of a list rendered into it is text, so its items are
    /// not open and write no content column for a block (issues #583, #615).
    pub(crate) text_in_markers: bool,
    /// Whether the current output buffer is written between the markers of a marker-only
    /// wrapper that does not count in `inline_depth` (a `<mark>`'s `==`, a `<del>`'s `~~`).
    ///
    /// ~keep A rule written there is text (issue #603). A list there still writes the content
    /// ~keep column, unlike under `text_in_markers`: text after a quote in its item then stays out
    /// ~keep of the quote.
    pub(crate) in_marker_span: bool,
    /// Whether the current output buffer escapes every `-` once it is written (a table
    /// caption): a `-` list marker there is text.
    pub(crate) escapes_hyphens: bool,
    /// List nesting depth (for indentation)
    pub(crate) list_depth: usize,
    /// Cumulative column width (in the `Spaces` indent type) that a nested list item at this
    /// point must be indented by: the sum of every ancestor `<li>`'s own marker width
    /// (`"- "` = 2, `"1. "` = 3, `"10. "` = 4, ...), honouring `list_indent_width` as a floor.
    ///
    /// Uniform per-depth indentation (`list_depth * list_indent_width`) is only correct when
    /// every ancestor list is unordered — an ordered ancestor's marker is wider than 2 columns,
    /// so a nested list must be indented to that marker's actual content column or CommonMark
    /// parses the child as a sibling instead of nested content.
    pub(crate) list_indent_columns: usize,
    /// The `list_indent_columns` of the innermost enclosing item whose marker line starts a list
    /// item in the output (an open item, or a marker between markers that starts its own line),
    /// or 0 where none does.
    ///
    /// ~keep A line between the markers starts a block only within 3 columns of that item's
    /// ~keep content column; further in, it is the paragraph's text. The first item's marker
    /// ~keep follows the opening marker, so it is text, but a nested item's marker starts its
    /// ~keep own line and is a real list item (issue #615).
    pub(crate) real_item_columns: usize,
    /// Whether a paragraph was open before the previous marker line of the innermost item's
    /// lists: the next marker's check stops there instead of walking back over every earlier item.
    pub(crate) previous_marker: crate::converter::list::utils::PreviousMarker,
    /// Whether the innermost list item is still open after the lines of a buffer checked so
    /// far: each block of the item checks only the lines written since.
    pub(crate) item_lines: crate::converter::list::utils::ItemLineScan,
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
    /// Whether inline images should remain markdown inside the current
    /// layout-table cell (its tag name is in `keep_inline_images_in`).
    pub(crate) cell_allow_inline_images: bool,
    /// Whether inline images should remain markdown inside the current `<a>` link label
    /// ("a" is in `keep_inline_images_in`). Set once per `handle_link` call and carried
    /// unchanged into both the block-label and inline-label branches, so the option means
    /// the same thing whether or not the anchor happens to contain a block child (#492).
    pub(crate) link_allow_inline_images: bool,
    /// Are we inside a paragraph element?
    pub(crate) in_paragraph: bool,
    /// Output buffer position where the current block's content starts.
    /// Used to distinguish paragraph-break newlines from a previous block
    /// vs. newlines generated within the current block.
    pub(crate) block_content_start: usize,
    /// Address of the `output` buffer `block_content_start` was measured against (set
    /// alongside it in `block/paragraph.rs`), as `*const String as usize`.
    ///
    /// `block_content_start` alone cannot tell "this text node's `output` is the SAME
    /// buffer the paragraph started measuring" from "it coincidentally has the same
    /// length" -- an inline wrapper (`em`/`strong`/link) builds its content into a
    /// *different, fresh local `String`* before splicing it into the real output, and that
    /// scratch buffer's length can coincidentally equal the ANCESTOR paragraph's
    /// `block_content_start` even when real content already precedes this point in the
    /// actual document (issue #481: `<p>A<i> </i>B</p>` saw the space inside `<i>`'s own
    /// empty scratch buffer misidentified as the paragraph's own leading whitespace and
    /// dropped). A raw address comparison, unlike `ctx.inline_depth == 0`, correctly
    /// still fires when a block-level tag-soup wrapper (e.g. a stray `<b>` wrapping whole
    /// paragraphs, as Google Docs exports) puts a genuine, buffer-sharing paragraph body
    /// at non-zero `inline_depth`.
    pub(crate) block_output_ptr: usize,
    /// Shared flag: true until the first non-whitespace text content is emitted.
    ///
    /// ~keep `block_content_start` cannot answer "is this text node at the very start of a
    /// ~keep fresh block" on its own: it is a byte offset compared against `output.len()`,
    /// ~keep but every inline wrapper (`sub`/`sup`/`em`/`strong`/links/...) builds its content
    /// ~keep into a *fresh local `String`* before splicing it into the real output, so that
    /// ~keep comparison sees an empty scratch buffer (len 0) that can coincidentally equal a
    /// ~keep real block start of 0 -- indistinguishable from genuine document start. This
    /// ~keep flag is `Rc<Cell<bool>>` instead so every clone of `Context` -- scratch-buffer
    /// ~keep handlers included -- observes one shared, buffer-independent truth about
    /// ~keep whether real content already precedes this point in the actual document.
    /// ~keep Currently reset to `true` only at document start (`Context::new`); nothing
    /// ~keep flips it back to `true` mid-document, so it only ever fires exactly once, but
    /// ~keep the mechanism is ready for additional reset points (e.g. per-block handlers) to
    /// ~keep extend the same "insignificant leading whitespace" treatment to fresh blocks
    /// ~keep beyond the very first one, without changing how consumers read it.
    pub(crate) at_fresh_block_start: Rc<Cell<bool>>,
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
    /// Node IDs matching `exclude_selectors` — these nodes and all descendants are dropped.
    pub(crate) excluded_node_ids: Rc<HashSet<u32>>,
    /// Shared flag set when the guarded DOM walk reaches its effective depth limit.
    pub(crate) depth_limit_reached: Rc<Cell<bool>>,
    /// Shared flag set when `inline_data_media` replaced or dropped an element. A link reads it to
    /// tell a label emptied by that option from an empty one.
    pub(crate) inline_data_replaced: Rc<Cell<bool>>,
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
    /// Optional structure collector for building a [`crate::types::DocumentStructure`].
    ///
    /// Populated when `options.include_document_structure == true`.
    pub(crate) structure_collector: Option<StructureCollectorHandle>,
    /// Optional reference collector for reference-style links.
    pub(crate) reference_collector: Option<ReferenceCollectorHandle>,
    /// When `true`, all visitor-hook Mutex acquisitions are skipped.
    ///
    /// Set during the table column-width pre-pass, which walks every cell to
    /// measure rendered text width but never uses visitor results.  The pre-pass
    /// runs before the main rendering pass, so acquiring the Mutex there is pure
    /// overhead (especially costly post-`Arc<Mutex>` migration in 7f6178f25).
    pub(crate) skip_visitor_hooks: bool,
    /// When `true`, `walk_node` short-circuits nested-table dispatch — the
    /// outer table is currently in its width-measurement pre-pass and the
    /// nested table's full markdown rendering (which itself runs a pre-pass on
    /// every descendant cell) would explode CPU on layout-heavy HTML.
    ///
    /// The per-cell width cap (`MAX_CELL_WIDTH = 200` in
    /// `cells.rs::collect_row_cell_widths`) bounds the discarded *output*, but
    /// the underlying *measurement* still walked every nested table fully.
    /// Skipping the nested-table dispatch during measurement keeps the
    /// pre-pass linear in descendant text length without changing rendered
    /// output (the column width is slightly underestimated for cells
    /// containing tables, which is fine — the cap dominates anyway).
    ///
    /// Set together with `skip_visitor_hooks` when constructing `prepass_ctx`
    /// in `block::table::builder::handle_table`. Resolves issue #406.
    pub(crate) measure_width_only: bool,
    /// Effective base URL for resolving relative `href`/`src` destinations, already
    /// combined with any document `<base href>` (see `converter::url_resolve`).
    /// `None` when `options.base_url` is unset -- every resolution call becomes a
    /// no-op then, so output is byte-identical to before this option existed.
    pub(crate) base_url: Option<Rc<url::Url>>,
}

impl Context {
    /// Set the pre-computed set of node IDs that match `exclude_selectors`.
    ///
    /// Called in `convert_html_impl` after DOM parsing, before the walk starts.
    pub(crate) fn set_excluded_node_ids(&mut self, ids: HashSet<u32>) {
        self.excluded_node_ids = Rc::new(ids);
    }

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
        base_url: Option<Rc<url::Url>>,
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
            in_code_block: false,
            list_counter: 0,
            in_ordered_list: false,
            blockquote_depth: 0,
            quote_list_columns: 0,
            in_table_cell: false,
            in_layout_cell: false,
            convert_as_inline: options.convert_as_inline,
            inline_depth: 0,
            in_list_item: false,
            list_item_open: false,
            text_in_markers: false,
            in_marker_span: false,
            escapes_hyphens: false,
            list_depth: 0,
            list_indent_columns: 0,
            real_item_columns: 0,
            previous_marker: crate::converter::list::utils::PreviousMarker::default(),
            item_lines: crate::converter::list::utils::ItemLineScan::default(),
            ul_depth: 0,
            in_list: false,
            loose_list: false,
            prev_item_had_blocks: false,
            in_heading: false,
            heading_allow_inline_images: false,
            cell_allow_inline_images: false,
            link_allow_inline_images: false,
            in_paragraph: false,
            block_content_start: 0,
            block_output_ptr: 0,
            at_fresh_block_start: Rc::new(Cell::new(true)),
            in_ruby: false,
            in_strong: false,
            in_link: false,
            strip_tags: Rc::new(options.strip_tags.iter().cloned().collect()),
            preserve_tags: Rc::new(options.preserve_tags.iter().cloned().collect()),
            keep_inline_images_in: Rc::new(options.keep_inline_images_in.iter().cloned().collect()),
            excluded_node_ids: Rc::new(HashSet::new()),
            depth_limit_reached: Rc::new(Cell::new(false)),
            inline_data_replaced: Rc::new(Cell::new(false)),
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
            structure_collector,
            reference_collector,
            skip_visitor_hooks: false,
            measure_width_only: false,
            base_url,
        }
    }

    /// Whether the current output buffer is written between inline markers: an inline wrapper's
    /// (`inline_depth`, a link's brackets included), a summary's or caption's (`text_in_markers`)
    /// or a marker-only wrapper's (`in_marker_span`). A rule written there is text, since the
    /// markers cannot span it.
    pub(crate) const fn in_marker_text(&self) -> bool {
        self.inline_depth > 0 || self.text_in_markers || self.in_marker_span
    }

    /// What to write for an element whose chosen address is `address`, as
    /// [`crate::converter::media::inline_data_treatment`] decides, recording in
    /// [`Self::inline_data_replaced`] when it is not [`InlineDataMedia::Keep`].
    pub(crate) fn inline_data_treatment(&self, choice: InlineDataMedia, address: &str) -> InlineDataMedia {
        let treatment = crate::converter::media::inline_data_treatment(choice, address);
        if treatment != InlineDataMedia::Keep {
            self.inline_data_replaced.set(true);
        }
        treatment
    }

    /// Resolve a `href`/`src` attribute value against [`Self::base_url`].
    ///
    /// Returns `None` (meaning: use the original text unchanged) whenever `base_url`
    /// is unset, `value` is empty, already absolute, or fails to resolve. See
    /// `converter::url_resolve::resolve_attribute_url` for the full contract.
    pub(crate) fn resolve_url(&self, value: &str) -> Option<String> {
        crate::converter::url_resolve::resolve_attribute_url(self.base_url.as_deref()?, value)
    }
}
