//! Option parsing for R bindings.

use extendr_api::prelude::*;
use html_to_markdown_rs::{
    CodeBlockStyle, ConversionOptions, ConversionOptionsUpdate, HeadingStyle, HighlightStyle,
    LinkStyle, ListIndentType, NewlineStyle, OutputFormat, PreprocessingOptionsUpdate,
    PreprocessingPreset, WhitespaceMode,
};

/// Decode an R list into ConversionOptions.
pub fn decode_options(options: Robj) -> std::result::Result<ConversionOptions, String> {
    if options.is_null() {
        return Ok(ConversionOptions::default());
    }

    let list = options
        .as_list()
        .ok_or_else(|| "options must be a named list".to_string())?;

    apply_options(&list)
}

fn apply_options(list: &List) -> std::result::Result<ConversionOptions, String> {
    let mut update = ConversionOptionsUpdate::default();

    for (key, value) in list.iter() {
        // Accept both snake_case and camelCase option keys for compatibility with
        // auto-generated test suites and user convenience.
        match key {
            "heading_style" | "headingStyle" => {
                update.heading_style = Some(parse_heading_style(&value)?)
            }
            "list_indent_type" | "listIndentType" => {
                update.list_indent_type = Some(parse_list_indent_type(&value)?)
            }
            "list_indent_width" | "listIndentWidth" => {
                update.list_indent_width =
                    Some(decode_positive_integer(&value, "list_indent_width")?)
            }
            "bullets" => update.bullets = Some(decode_string(&value, "bullets")?),
            "strong_em_symbol" | "strongEmSymbol" => {
                let symbol = decode_string(&value, "strong_em_symbol")?;
                let ch = symbol
                    .chars()
                    .next()
                    .ok_or_else(|| "strong_em_symbol: must not be empty".to_string())?;
                update.strong_em_symbol = Some(ch);
            }
            "escape_asterisks" | "escapeAsterisks" => {
                update.escape_asterisks = Some(decode_bool(&value, "escape_asterisks")?)
            }
            "escape_underscores" | "escapeUnderscores" => {
                update.escape_underscores = Some(decode_bool(&value, "escape_underscores")?)
            }
            "escape_misc" | "escapeMisc" => {
                update.escape_misc = Some(decode_bool(&value, "escape_misc")?)
            }
            "escape_ascii" | "escapeAscii" => {
                update.escape_ascii = Some(decode_bool(&value, "escape_ascii")?)
            }
            "code_language" | "codeLanguage" => {
                update.code_language = Some(decode_string(&value, "code_language")?)
            }
            "encoding" => update.encoding = Some(decode_string(&value, "encoding")?),
            "autolinks" => update.autolinks = Some(decode_bool(&value, "autolinks")?),
            "default_title" | "defaultTitle" => {
                update.default_title = Some(decode_bool(&value, "default_title")?)
            }
            "keep_inline_images_in" | "keepInlineImagesIn" => {
                update.keep_inline_images_in =
                    Some(decode_string_list(&value, "keep_inline_images_in")?)
            }
            "br_in_tables" | "brInTables" => {
                update.br_in_tables = Some(decode_bool(&value, "br_in_tables")?)
            }
            "highlight_style" | "highlightStyle" => {
                update.highlight_style = Some(parse_highlight_style(&value)?)
            }
            "extract_metadata" | "extractMetadata" => {
                update.extract_metadata = Some(decode_bool(&value, "extract_metadata")?)
            }
            "whitespace_mode" | "whitespaceMode" => {
                update.whitespace_mode = Some(parse_whitespace_mode(&value)?)
            }
            "strip_newlines" | "stripNewlines" => {
                update.strip_newlines = Some(decode_bool(&value, "strip_newlines")?)
            }
            "wrap" => update.wrap = Some(decode_bool(&value, "wrap")?),
            "wrap_width" | "wrapWidth" => {
                update.wrap_width = Some(decode_positive_integer(&value, "wrap_width")?)
            }
            "strip_tags" | "stripTags" => {
                update.strip_tags = Some(decode_string_list(&value, "strip_tags")?)
            }
            "preserve_tags" | "preserveTags" => {
                update.preserve_tags = Some(decode_string_list(&value, "preserve_tags")?)
            }
            "convert_as_inline" | "convertAsInline" => {
                update.convert_as_inline = Some(decode_bool(&value, "convert_as_inline")?)
            }
            "sub_symbol" | "subSymbol" => {
                update.sub_symbol = Some(decode_string(&value, "sub_symbol")?)
            }
            "sup_symbol" | "supSymbol" => {
                update.sup_symbol = Some(decode_string(&value, "sup_symbol")?)
            }
            "newline_style" | "newlineStyle" => {
                update.newline_style = Some(parse_newline_style(&value)?)
            }
            "code_block_style" | "codeBlockStyle" => {
                update.code_block_style = Some(parse_code_block_style(&value)?)
            }
            "output_format" | "outputFormat" => {
                update.output_format = Some(parse_output_format(&value)?)
            }
            "link_style" | "linkStyle" => update.link_style = Some(parse_link_style(&value)?),
            "skip_images" | "skipImages" => {
                update.skip_images = Some(decode_bool(&value, "skip_images")?)
            }
            "include_document_structure" | "includeDocumentStructure" => {
                update.include_document_structure =
                    Some(decode_bool(&value, "include_document_structure")?)
            }
            "extract_images" | "extractImages" => {
                update.extract_images = Some(decode_bool(&value, "extract_images")?)
            }
            "max_image_size" | "maxImageSize" => {
                update.max_image_size =
                    Some(decode_positive_integer(&value, "max_image_size")? as u64)
            }
            "capture_svg" | "captureSvg" => {
                update.capture_svg = Some(decode_bool(&value, "capture_svg")?)
            }
            "infer_dimensions" | "inferDimensions" => {
                update.infer_dimensions = Some(decode_bool(&value, "infer_dimensions")?)
            }
            "preprocessing" => update.preprocessing = Some(decode_preprocessing(&value)?),
            "debug" => update.debug = Some(decode_bool(&value, "debug")?),
            _ => {}
        }
    }

    Ok(ConversionOptions::from(update))
}

fn decode_preprocessing(value: &Robj) -> std::result::Result<PreprocessingOptionsUpdate, String> {
    let list = value
        .as_list()
        .ok_or_else(|| "preprocessing: must be a named list".to_string())?;

    let mut update = PreprocessingOptionsUpdate::default();

    for (key, val) in list.iter() {
        match key {
            "enabled" => update.enabled = Some(decode_bool(&val, "preprocessing.enabled")?),
            "preset" => update.preset = Some(parse_preset(&val)?),
            "remove_navigation" => {
                update.remove_navigation =
                    Some(decode_bool(&val, "preprocessing.remove_navigation")?)
            }
            "remove_forms" => {
                update.remove_forms = Some(decode_bool(&val, "preprocessing.remove_forms")?)
            }
            _ => {}
        }
    }

    Ok(update)
}

fn decode_bool(value: &Robj, field: &str) -> std::result::Result<bool, String> {
    value
        .as_bool()
        .ok_or_else(|| format!("{field}: must be a logical (TRUE/FALSE)"))
}

fn decode_string(value: &Robj, field: &str) -> std::result::Result<String, String> {
    value
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("{field}: must be a character string"))
}

fn decode_string_list(value: &Robj, field: &str) -> std::result::Result<Vec<String>, String> {
    let strs = value
        .as_str_vector()
        .ok_or_else(|| format!("{field}: must be a character vector"))?;
    Ok(strs.into_iter().map(|s| s.to_string()).collect())
}

fn decode_positive_integer(value: &Robj, field: &str) -> std::result::Result<usize, String> {
    let v = value
        .as_integer()
        .or_else(|| value.as_real().map(|r| r as i32))
        .ok_or_else(|| format!("{field}: must be a positive integer"))?;
    if v <= 0 {
        return Err(format!("{field}: must be greater than zero"));
    }
    Ok(v as usize)
}

fn parse_heading_style(value: &Robj) -> std::result::Result<HeadingStyle, String> {
    let s = value
        .as_str()
        .ok_or_else(|| "heading_style: must be a character string".to_string())?;
    match s {
        "atx" => Ok(HeadingStyle::Atx),
        // Accept both snake_case and camelCase variants for ATX closed style.
        "atx_closed" | "atxclosed" | "atxClosed" => Ok(HeadingStyle::AtxClosed),
        "underlined" => Ok(HeadingStyle::Underlined),
        _ => Err(format!("heading_style: invalid value: {s}")),
    }
}

fn parse_list_indent_type(value: &Robj) -> std::result::Result<ListIndentType, String> {
    let s = value
        .as_str()
        .ok_or_else(|| "list_indent_type: must be a character string".to_string())?;
    match s {
        "spaces" => Ok(ListIndentType::Spaces),
        "tabs" => Ok(ListIndentType::Tabs),
        _ => Err(format!("list_indent_type: invalid value: {s}")),
    }
}

fn parse_highlight_style(value: &Robj) -> std::result::Result<HighlightStyle, String> {
    let s = value
        .as_str()
        .ok_or_else(|| "highlight_style: must be a character string".to_string())?;
    let normalized = s.replace('-', "_");
    match normalized.as_str() {
        "double_equal" => Ok(HighlightStyle::DoubleEqual),
        "html" => Ok(HighlightStyle::Html),
        "bold" => Ok(HighlightStyle::Bold),
        "none" => Ok(HighlightStyle::None),
        _ => Err(format!("highlight_style: invalid value: {s}")),
    }
}

fn parse_whitespace_mode(value: &Robj) -> std::result::Result<WhitespaceMode, String> {
    let s = value
        .as_str()
        .ok_or_else(|| "whitespace_mode: must be a character string".to_string())?;
    match s {
        "normalized" => Ok(WhitespaceMode::Normalized),
        "strict" => Ok(WhitespaceMode::Strict),
        _ => Err(format!("whitespace_mode: invalid value: {s}")),
    }
}

fn parse_newline_style(value: &Robj) -> std::result::Result<NewlineStyle, String> {
    let s = value
        .as_str()
        .ok_or_else(|| "newline_style: must be a character string".to_string())?;
    match s {
        "spaces" => Ok(NewlineStyle::Spaces),
        "backslash" => Ok(NewlineStyle::Backslash),
        _ => Err(format!("newline_style: invalid value: {s}")),
    }
}

fn parse_code_block_style(value: &Robj) -> std::result::Result<CodeBlockStyle, String> {
    let s = value
        .as_str()
        .ok_or_else(|| "code_block_style: must be a character string".to_string())?;
    match s {
        "indented" => Ok(CodeBlockStyle::Indented),
        "backticks" => Ok(CodeBlockStyle::Backticks),
        "tildes" => Ok(CodeBlockStyle::Tildes),
        _ => Err(format!("code_block_style: invalid value: {s}")),
    }
}

fn parse_output_format(value: &Robj) -> std::result::Result<OutputFormat, String> {
    let s = value
        .as_str()
        .ok_or_else(|| "output_format: must be a character string".to_string())?;
    match s {
        "markdown" => Ok(OutputFormat::Markdown),
        "djot" => Ok(OutputFormat::Djot),
        "plain" | "plaintext" | "text" => Ok(OutputFormat::Plain),
        _ => Err(format!("output_format: invalid value: {s}")),
    }
}

fn parse_link_style(value: &Robj) -> std::result::Result<LinkStyle, String> {
    let s = value
        .as_str()
        .ok_or_else(|| "link_style: must be a character string".to_string())?;
    match s {
        "inline" => Ok(LinkStyle::Inline),
        "reference" => Ok(LinkStyle::Reference),
        _ => Err(format!("link_style: invalid value: {s}")),
    }
}

fn parse_preset(value: &Robj) -> std::result::Result<PreprocessingPreset, String> {
    let s = value
        .as_str()
        .ok_or_else(|| "preprocessing.preset: must be a character string".to_string())?;
    let normalized = s.replace('-', "_");
    match normalized.as_str() {
        "minimal" => Ok(PreprocessingPreset::Minimal),
        "aggressive" => Ok(PreprocessingPreset::Aggressive),
        "standard" => Ok(PreprocessingPreset::Standard),
        _ => Err(format!("preprocessing.preset: invalid value: {s}")),
    }
}
