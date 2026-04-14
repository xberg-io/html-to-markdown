//! Configuration options for HTML to Markdown conversion.
//!
//! This module provides comprehensive configuration options for customizing
//! HTML to Markdown conversion behavior, including output formatting, preprocessing,
//! and metadata extraction options.

pub mod conversion;
pub mod inline_image;
pub mod preprocessing;
pub mod validation;

// Re-exports for easy access
pub use conversion::{ConversionOptions, ConversionOptionsBuilder, ConversionOptionsUpdate};
pub use preprocessing::{PreprocessingOptions, PreprocessingOptionsUpdate, PreprocessingPreset};
pub use validation::{
    CodeBlockStyle, HeadingStyle, HighlightStyle, LinkStyle, ListIndentType, NewlineStyle, OutputFormat, WhitespaceMode,
};

// Note: InlineImageConfig is re-exported from the inline_images module,
// not from this options module, to maintain compatibility with existing imports.
