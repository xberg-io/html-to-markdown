// ============================================================================
// Crate-wide lint configuration
//
// Lints suppressed here fire in many places due to structural constraints of the
// DOM traversal pipeline and cannot be easily fixed without API breakage or
// significant complexity increase. Each allow is justified below.
// ============================================================================

// reason: converter handler functions take (node_handle, tag, parser, output, options,
// ctx, depth, dom_ctx) — 8 arguments minimum imposed by the DOM traversal API.
// Reducing to fewer arguments would require threading a single "Context" mega-struct
// through everything, which would make call sites less explicit, not clearer.
#![allow(clippy::too_many_arguments)]
// reason: many converter functions are long because they handle all HTML variants of a tag
// (e.g., code_block handles <pre>, <code>, <samp>, language attrs, visitor callbacks,
// metadata collection, and feature-gated paths) — splitting would obscure the logic.
#![allow(clippy::too_many_lines)]
// reason: DOM API uses tl::NodeHandle (4-byte Copy) passed as &NodeHandle throughout
// because that is the established calling convention from the tl parser crate.
// Changing to pass-by-value would require updating hundreds of call sites and
// would break the parallel with html5ever's API which also uses handles by reference.
#![allow(clippy::trivially_copy_pass_by_ref)]
// reason: ConversionOptions and related config structs use bool fields to represent
// independent toggles. Using bitflags or enums would add complexity for no readability gain
// since each flag has a distinct semantic meaning.
#![allow(clippy::struct_excessive_bools, clippy::fn_params_excessive_bools)]
// reason: usize→u32 casts occur when converting DOM-internal counts to protocol/metadata
// fields. On 32-bit targets usize == u32 so no truncation; on 64-bit targets the
// values are bounded by HTML document sizes which fit in u32. A TryFrom would add
// error handling for a condition that cannot occur with valid HTML.
#![allow(clippy::cast_possible_truncation)]
// reason: float ratio computations in validation.rs use usize→f64 casts to compute
// proportions. The precision loss is intentional and acceptable for threshold comparisons.
// Cast sign loss is impossible because usize is unsigned.
#![allow(clippy::cast_precision_loss, clippy::cast_sign_loss)]
// reason: several methods need &self for trait conformance or future extensibility
// even though they don't access self fields currently.
#![allow(clippy::unused_self)]
// reason: collapsible_if and collapsible_match fire extensively in the DOM scanner
// where the nested structure mirrors the hierarchical HTML grammar and readability
// is better preserved by keeping the nesting explicit.
#![allow(clippy::collapsible_if, clippy::collapsible_else_if, clippy::collapsible_match)]
// reason: these idiom lints (map_or, let..else, option_if_let_else, match_same_arms)
// fire in 80+ places throughout the converter. Applying them would change many
// correct patterns without improving correctness; they are style preferences only.
#![allow(
    clippy::map_unwrap_or,
    clippy::manual_let_else,
    clippy::option_if_let_else,
    clippy::match_same_arms,
    clippy::branches_sharing_code,
    clippy::items_after_statements,
    clippy::match_wildcard_for_single_variants,
    clippy::needless_pass_by_value
)]
// reason: doc_markdown fires on HTML element names and CSS identifiers in doc comments
// (e.g., "CommonMark", "data-*", "aria-label") that clippy considers should be in backticks.
// The majority of these are in internal modules without public doc coverage requirements.
#![allow(clippy::doc_markdown, clippy::missing_errors_doc)]
// reason: default_trait_access (Default::default() vs Foo::default()) is a style preference;
// the explicit Default::default() form is clearer at call sites with multiple defaults.
#![allow(clippy::default_trait_access)]

//! High-performance HTML to Markdown converter.
//!
//! Built with html5ever for fast, memory-efficient HTML parsing.
//!
//! ## Optional inline image extraction
//!
//! Enable the `inline-images` Cargo feature to collect embedded data URI images and inline SVG
//! assets alongside the produced Markdown.

// ============================================================================
// Module Declarations
// ============================================================================

pub mod error;
#[cfg(feature = "metadata")]
pub mod metadata;
pub mod options;
pub mod types;
#[cfg(feature = "visitor")]
pub mod visitor;

// Internal modules (not part of public API)
mod convert_api;
// reason: converter is a large pub(crate) module; some sub-items are only used when
// specific feature flags are active. The allow silences cross-feature dead_code noise
// that would otherwise require fine-grained #[cfg] gates on every internal helper.
#[allow(dead_code)]
pub(crate) mod converter;
mod exports;

// Re-export internal test/benchmark modules when the testkit feature is active.
// This lets integration tests and the bench harness access prescan and tier1
// without making them part of the stable public API.
//
// We use a pub mod alias so tests can use both the short path (`crate::prescan`)
// and the original path (`crate::converter::prescan`) via the re-export below.
#[cfg(any(test, feature = "testkit"))]
// reason: re-exports are gated on test/testkit; `use` items inside may not all be
// consumed by every test file, producing unused_imports warnings in some configurations.
#[allow(unused_imports)]
/// Re-exports of internal modules for integration tests and the bench harness.
pub mod testkit {
    pub use crate::converter::prescan;
    pub use crate::converter::tier1;
}
#[cfg(any(test, feature = "testkit"))]
pub use converter::prescan;
#[cfg(any(test, feature = "testkit"))]
pub use converter::tier1;
#[cfg(feature = "inline-images")]
mod inline_images;
pub(crate) mod prelude;
mod rcdom;
pub(crate) mod text;
mod validation;
#[cfg(feature = "visitor")]
// reason: visitor_helpers exposes functions that accept Option<&X> parameters
// where &Option<X> (ref_option) would be the clippy preferred form; using Option<&X>
// matches the calling convention for the visitor dispatch API at every call site.
#[allow(clippy::ref_option)]
pub(crate) mod visitor_helpers;
pub(crate) mod wrapper;

// ============================================================================
// Public Re-exports (from exports module)
// ============================================================================

pub use exports::*;
pub use types::{
    AnnotationKind, ConversionResult, DocumentNode, DocumentStructure, GridCell, ImageDimensions, MetadataEntry,
    NodeContent, ProcessingWarning, TableData, TableGrid, TextAnnotation, WarningKind,
};
#[cfg(feature = "visitor")]
pub use visitor::{NodeContext, NodeType, VisitResult, VisitorHandle};

// ============================================================================
// Main Public API Functions
// ============================================================================

pub use convert_api::convert;

#[cfg(feature = "mcp")]
pub mod mcp;

// Tests
// ============================================================================

#[cfg(test)]
mod basic_tests {
    use super::*;

    #[test]
    fn test_binary_input_rejected() {
        let html = format!("abc{}def", "\0".repeat(20));
        let result = convert(&html, None);
        assert!(matches!(result, Err(ConversionError::InvalidInput(_))));
    }

    #[test]
    fn test_binary_magic_rejected() {
        let html = "%PDF-1.7";
        let result = convert(html, None);
        assert!(matches!(result, Err(ConversionError::InvalidInput(_))));
    }

    #[test]
    fn test_utf16_hint_recovered() {
        let html = String::from_utf8_lossy(b"\xFF\xFE<\0h\0t\0m\0l\0>\0").to_string();
        let result = convert(&html, None);
        assert!(result.is_ok(), "UTF-16 input should be recovered instead of rejected");
    }

    #[test]
    fn test_plain_text_allowed() {
        let result = convert("Just text", None).unwrap();
        let content = result.content.unwrap_or_default();
        assert!(content.contains("Just text"));
    }

    #[test]
    fn test_plain_text_escaped_when_enabled() {
        let options = ConversionOptions {
            escape_asterisks: true,
            escape_underscores: true,
            ..ConversionOptions::default()
        };
        let result = convert("Text *asterisks* _underscores_", Some(options)).unwrap();
        let content = result.content.unwrap_or_default();
        assert!(content.contains(r"\*asterisks\*"));
        assert!(content.contains(r"\_underscores\_"));
    }
}
