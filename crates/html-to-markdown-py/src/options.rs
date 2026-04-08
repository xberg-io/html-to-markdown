use html_to_markdown_rs::{
    CodeBlockStyle, ConversionOptions as RustConversionOptions, HeadingStyle, HighlightStyle, LinkStyle,
    ListIndentType, NewlineStyle, OutputFormat, PreprocessingOptions as RustPreprocessingOptions, PreprocessingPreset,
    WhitespaceMode,
};
use pyo3::prelude::*;

/// Python wrapper for PreprocessingOptions
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PreprocessingOptions {
    #[pyo3(get, set)]
    pub enabled: bool,
    #[pyo3(get, set)]
    pub preset: String,
    #[pyo3(get, set)]
    pub remove_navigation: bool,
    #[pyo3(get, set)]
    pub remove_forms: bool,
}

#[pymethods]
impl PreprocessingOptions {
    #[new]
    #[pyo3(signature = (enabled=false, preset="standard".to_string(), remove_navigation=true, remove_forms=true))]
    pub const fn new(enabled: bool, preset: String, remove_navigation: bool, remove_forms: bool) -> Self {
        Self {
            enabled,
            preset,
            remove_navigation,
            remove_forms,
        }
    }
}

impl PreprocessingOptions {
    /// Convert to Rust PreprocessingOptions
    pub fn to_rust(&self) -> RustPreprocessingOptions {
        RustPreprocessingOptions {
            enabled: self.enabled,
            preset: match self.preset.as_str() {
                "minimal" => PreprocessingPreset::Minimal,
                "aggressive" => PreprocessingPreset::Aggressive,
                _ => PreprocessingPreset::Standard,
            },
            remove_navigation: self.remove_navigation,
            remove_forms: self.remove_forms,
        }
    }
}

/// Python wrapper for ConversionOptions
#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct ConversionOptions {
    #[pyo3(get, set)]
    pub heading_style: String,
    #[pyo3(get, set)]
    pub list_indent_type: String,
    #[pyo3(get, set)]
    pub list_indent_width: usize,
    #[pyo3(get, set)]
    pub bullets: String,
    #[pyo3(get, set)]
    pub strong_em_symbol: char,
    #[pyo3(get, set)]
    pub escape_asterisks: bool,
    #[pyo3(get, set)]
    pub escape_underscores: bool,
    #[pyo3(get, set)]
    pub escape_misc: bool,
    #[pyo3(get, set)]
    pub escape_ascii: bool,
    #[pyo3(get, set)]
    pub code_language: String,
    #[pyo3(get, set)]
    pub autolinks: bool,
    #[pyo3(get, set)]
    pub default_title: bool,
    #[pyo3(get, set)]
    pub br_in_tables: bool,
    #[pyo3(get, set)]
    pub highlight_style: String,
    #[pyo3(get, set)]
    pub extract_metadata: bool,
    #[pyo3(get, set)]
    pub whitespace_mode: String,
    #[pyo3(get, set)]
    pub strip_newlines: bool,
    #[pyo3(get, set)]
    pub wrap: bool,
    #[pyo3(get, set)]
    pub wrap_width: usize,
    #[pyo3(get, set)]
    pub convert_as_inline: bool,
    #[pyo3(get, set)]
    pub sub_symbol: String,
    #[pyo3(get, set)]
    pub sup_symbol: String,
    #[pyo3(get, set)]
    pub newline_style: String,
    #[pyo3(get, set)]
    pub code_block_style: String,
    #[pyo3(get, set)]
    pub keep_inline_images_in: Vec<String>,
    #[pyo3(get, set)]
    pub preprocessing: PreprocessingOptions,
    #[pyo3(get, set)]
    pub debug: bool,
    #[pyo3(get, set)]
    pub strip_tags: Vec<String>,
    #[pyo3(get, set)]
    pub preserve_tags: Vec<String>,
    #[pyo3(get, set)]
    pub encoding: String,
    #[pyo3(get, set)]
    pub skip_images: bool,
    #[pyo3(get, set)]
    pub link_style: String,
    #[pyo3(get, set)]
    pub output_format: String,
    #[pyo3(get, set)]
    pub include_document_structure: bool,
    #[pyo3(get, set)]
    pub extract_images: bool,
    #[pyo3(get, set)]
    pub max_image_size: u64,
    #[pyo3(get, set)]
    pub capture_svg: bool,
    #[pyo3(get, set)]
    pub infer_dimensions: bool,
    #[pyo3(get, set)]
    pub max_depth: Option<usize>,
}

#[pymethods]
impl ConversionOptions {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        heading_style="underlined".to_string(),
        list_indent_type="spaces".to_string(),
        list_indent_width=4,
        bullets="*+-".to_string(),
        strong_em_symbol='*',
        escape_asterisks=false,
        escape_underscores=false,
        escape_misc=false,
        escape_ascii=false,
        code_language="".to_string(),
        autolinks=true,
        default_title=false,
        br_in_tables=false,
        highlight_style="double-equal".to_string(),
        extract_metadata=true,
        whitespace_mode="normalized".to_string(),
        strip_newlines=false,
        wrap=false,
        wrap_width=80,
        convert_as_inline=false,
        sub_symbol="".to_string(),
        sup_symbol="".to_string(),
        newline_style="spaces".to_string(),
        code_block_style="indented".to_string(),
        keep_inline_images_in=Vec::new(),
        preprocessing=None,
        debug=false,
        strip_tags=Vec::new(),
        preserve_tags=Vec::new(),
        encoding="utf-8".to_string(),
        skip_images=false,
        link_style="inline".to_string(),
        output_format="markdown".to_string(),
        include_document_structure=false,
        extract_images=false,
        max_image_size=5242880u64,
        capture_svg=false,
        infer_dimensions=true,
        max_depth=None
    ))]
    pub fn new(
        heading_style: String,
        list_indent_type: String,
        list_indent_width: usize,
        bullets: String,
        strong_em_symbol: char,
        escape_asterisks: bool,
        escape_underscores: bool,
        escape_misc: bool,
        escape_ascii: bool,
        code_language: String,
        autolinks: bool,
        default_title: bool,
        br_in_tables: bool,
        highlight_style: String,
        extract_metadata: bool,
        whitespace_mode: String,
        strip_newlines: bool,
        wrap: bool,
        wrap_width: usize,
        convert_as_inline: bool,
        sub_symbol: String,
        sup_symbol: String,
        newline_style: String,
        code_block_style: String,
        keep_inline_images_in: Vec<String>,
        preprocessing: Option<PreprocessingOptions>,
        debug: bool,
        strip_tags: Vec<String>,
        preserve_tags: Vec<String>,
        encoding: String,
        skip_images: bool,
        link_style: String,
        output_format: String,
        include_document_structure: bool,
        extract_images: bool,
        max_image_size: u64,
        capture_svg: bool,
        infer_dimensions: bool,
        max_depth: Option<usize>,
    ) -> Self {
        Self {
            heading_style,
            list_indent_type,
            list_indent_width,
            bullets,
            strong_em_symbol,
            escape_asterisks,
            escape_underscores,
            escape_misc,
            escape_ascii,
            code_language,
            autolinks,
            default_title,
            br_in_tables,
            highlight_style,
            extract_metadata,
            whitespace_mode,
            strip_newlines,
            wrap,
            wrap_width,
            convert_as_inline,
            sub_symbol,
            sup_symbol,
            newline_style,
            code_block_style,
            keep_inline_images_in,
            preprocessing: preprocessing
                .unwrap_or_else(|| PreprocessingOptions::new(false, "standard".to_string(), true, true)),
            debug,
            strip_tags,
            preserve_tags,
            encoding,
            skip_images,
            link_style,
            output_format,
            include_document_structure,
            extract_images,
            max_image_size,
            capture_svg,
            infer_dimensions,
            max_depth,
        }
    }
}

impl ConversionOptions {
    /// Convert to Rust ConversionOptions
    pub fn to_rust(&self) -> RustConversionOptions {
        RustConversionOptions {
            heading_style: HeadingStyle::parse(self.heading_style.as_str()),
            list_indent_type: ListIndentType::parse(self.list_indent_type.as_str()),
            list_indent_width: self.list_indent_width,
            bullets: self.bullets.clone(),
            strong_em_symbol: self.strong_em_symbol,
            escape_asterisks: self.escape_asterisks,
            escape_underscores: self.escape_underscores,
            escape_misc: self.escape_misc,
            escape_ascii: self.escape_ascii,
            code_language: self.code_language.clone(),
            autolinks: self.autolinks,
            default_title: self.default_title,
            br_in_tables: self.br_in_tables,
            highlight_style: HighlightStyle::parse(self.highlight_style.as_str()),
            extract_metadata: self.extract_metadata,
            whitespace_mode: WhitespaceMode::parse(self.whitespace_mode.as_str()),
            strip_newlines: self.strip_newlines,
            wrap: self.wrap,
            wrap_width: self.wrap_width,
            convert_as_inline: self.convert_as_inline,
            sub_symbol: self.sub_symbol.clone(),
            sup_symbol: self.sup_symbol.clone(),
            newline_style: NewlineStyle::parse(self.newline_style.as_str()),
            code_block_style: CodeBlockStyle::parse(self.code_block_style.as_str()),
            keep_inline_images_in: self.keep_inline_images_in.clone(),
            preprocessing: self.preprocessing.to_rust(),
            encoding: self.encoding.clone(),
            debug: self.debug,
            strip_tags: self.strip_tags.clone(),
            preserve_tags: self.preserve_tags.clone(),
            skip_images: self.skip_images,
            link_style: LinkStyle::parse(self.link_style.as_str()),
            output_format: OutputFormat::parse(self.output_format.as_str()),
            include_document_structure: self.include_document_structure,
            extract_images: self.extract_images,
            max_image_size: self.max_image_size,
            capture_svg: self.capture_svg,
            infer_dimensions: self.infer_dimensions,
            max_depth: self.max_depth,
        }
    }
}
