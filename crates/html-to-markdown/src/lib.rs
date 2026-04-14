#![allow(
    clippy::too_many_lines,
    clippy::option_if_let_else,
    clippy::match_wildcard_for_single_variants,
    clippy::needless_pass_by_value,
    clippy::struct_excessive_bools,
    clippy::fn_params_excessive_bools,
    clippy::branches_sharing_code,
    clippy::match_same_arms,
    clippy::missing_errors_doc,
    clippy::items_after_statements,
    clippy::doc_markdown,
    clippy::cast_sign_loss,
    clippy::default_trait_access,
    clippy::unused_self,
    clippy::cast_precision_loss,
    clippy::collapsible_if,
    clippy::too_many_arguments,
    clippy::collapsible_else_if,
    clippy::extra_unused_lifetimes,
    clippy::unnecessary_lazy_evaluations,
    clippy::must_use_candidate,
    clippy::trivially_copy_pass_by_ref,
    clippy::explicit_iter_loop,
    clippy::missing_const_for_fn,
    clippy::manual_assert,
    clippy::return_self_not_must_use,
    clippy::collapsible_match,
    clippy::cast_possible_truncation,
    clippy::map_unwrap_or,
    clippy::manual_let_else,
    clippy::used_underscore_binding,
    clippy::assigning_clones,
    clippy::uninlined_format_args
)]

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

pub mod converter;
pub mod error;
#[cfg(feature = "inline-images")]
mod inline_images;
#[cfg(feature = "metadata")]
pub mod metadata;
pub mod options;
pub mod safety;
pub mod text;
pub mod types;
#[cfg(feature = "visitor")]
pub mod visitor;
#[cfg(feature = "visitor")]
pub mod visitor_helpers;
pub mod wrapper;

// Internal modules (not part of public API)
mod convert_api;
mod exports;
pub mod prelude;
mod rcdom;
mod validation;

// ============================================================================
// Public Re-exports (from exports module)
// ============================================================================

pub use exports::*;
pub use types::{
    AnnotationKind, ConversionResult, DocumentNode, DocumentStructure, GridCell, NodeContent, ProcessingWarning,
    TableData, TableGrid, TextAnnotation, WarningKind,
};

// ============================================================================
// Main Public API Functions
// ============================================================================

pub use convert_api::convert;

#[cfg(any(feature = "serde", feature = "metadata"))]
pub use convert_api::{conversion_options_from_json, conversion_options_update_from_json};

#[cfg(feature = "metadata")]
pub use convert_api::metadata_config_from_json;

#[cfg(feature = "inline-images")]
pub use convert_api::inline_image_config_from_json;

#[cfg(feature = "visitor")]
#[doc(hidden)]
pub use convert_api::convert_with_visitor;

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
