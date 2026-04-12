#![deny(clippy::correctness, clippy::suspicious)]
#![allow(clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]

// Module declarations
mod enums;
mod handles;
mod options;
mod types;

// Public re-exports of enums
pub use enums::{
    JsCodeBlockStyle, JsHeadingStyle, JsHighlightStyle, JsListIndentType, JsNewlineStyle, JsOutputFormat,
    JsPreprocessingPreset, JsWhitespaceMode,
};

// Public re-exports of options
pub use options::{JsConversionOptions, JsPreprocessingOptions};

// Public re-exports of types
pub use types::{JsConversionResult, JsConversionTable, JsConversionWarning, JsGridCell, JsTableGrid};

// Re-export NAPI handlers
pub use handles::*;

// Global allocator
#[cfg(all(
    any(windows, unix),
    target_arch = "x86_64",
    not(target_env = "musl"),
    not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading_style_conversion() {
        use html_to_markdown_rs::HeadingStyle;
        let atx: HeadingStyle = JsHeadingStyle::Atx.into();
        assert!(matches!(atx, HeadingStyle::Atx));
    }

    #[test]
    fn test_conversion_options_defaults() {
        use html_to_markdown_rs::ConversionOptions;
        let opts = JsConversionOptions {
            heading_style: Some(JsHeadingStyle::Atx),
            list_indent_type: None,
            list_indent_width: None,
            bullets: None,
            strong_em_symbol: None,
            escape_asterisks: None,
            escape_underscores: None,
            escape_misc: None,
            escape_ascii: None,
            code_language: None,
            autolinks: None,
            default_title: None,
            br_in_tables: None,
            highlight_style: None,
            extract_metadata: None,
            whitespace_mode: None,
            strip_newlines: None,
            wrap: None,
            wrap_width: None,
            convert_as_inline: None,
            sub_symbol: None,
            sup_symbol: None,
            newline_style: None,
            code_block_style: None,
            keep_inline_images_in: None,
            preprocessing: None,
            encoding: None,
            debug: None,
            strip_tags: None,
            preserve_tags: None,
            skip_images: None,
            output_format: None,
            include_document_structure: None,
            extract_images: None,
            max_image_size: None,
            capture_svg: None,
            link_style: None,
            infer_dimensions: None,
        };

        let rust_opts: ConversionOptions = opts.into();
        use html_to_markdown_rs::HeadingStyle;
        assert!(matches!(rust_opts.heading_style, HeadingStyle::Atx));
        assert_eq!(rust_opts.list_indent_width, 2);
    }

    #[test]
    fn test_preprocessing_options() {
        use html_to_markdown_rs::{PreprocessingOptions, PreprocessingPreset};
        let opts = JsPreprocessingOptions {
            enabled: Some(true),
            preset: Some(JsPreprocessingPreset::Aggressive),
            remove_navigation: Some(false),
            remove_forms: Some(true),
        };

        let rust_opts: PreprocessingOptions = opts.into();
        assert!(rust_opts.enabled);
        assert!(matches!(rust_opts.preset, PreprocessingPreset::Aggressive));
        assert!(!rust_opts.remove_navigation);
        assert!(rust_opts.remove_forms);
    }
}
