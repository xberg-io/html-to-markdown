//! Option parsing for R bindings.

use extendr_api::prelude::*;
use html_to_markdown_rs::{
    CodeBlockStyle, ConversionOptions, ConversionOptionsUpdate, HeadingStyle,
    HighlightStyle, ListIndentType, NewlineStyle,
    PreprocessingOptionsUpdate, PreprocessingPreset, WhitespaceMode,
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
        match key {
            "heading_style" => update.heading_style = Some(parse_heading_style(&value)?),
            "list_indent_type" => update.list_indent_type = Some(parse_list_indent_type(&value)?),
            "list_indent_width" => {
                update.list_indent_width = Some(decode_positive_integer(&value, "list_indent_width")?)
            }
            "bullets" => update.bullets = Some(decode_string(&value, "bullets")?),
            "strong_em_symbol" => {
                let symbol = decode_string(&value, "strong_em_symbol")?;
                let ch = symbol
                    .chars()
                    .next()
                    .ok_or_else(|| "strong_em_symbol: must not be empty".to_string())?;
                update.strong_em_symbol = Some(ch);
            }
            "escape_asterisks" => update.escape_asterisks = Some(decode_bool(&value, "escape_asterisks")?),
            "escape_underscores" => update.escape_underscores = Some(decode_bool(&value, "escape_underscores")?),
            "escape_misc" => update.escape_misc = Some(decode_bool(&value, "escape_misc")?),
            "escape_ascii" => update.escape_ascii = Some(decode_bool(&value, "escape_ascii")?),
            "code_language" => update.code_language = Some(decode_string(&value, "code_language")?),
            "encoding" => update.encoding = Some(decode_string(&value, "encoding")?),
            "autolinks" => update.autolinks = Some(decode_bool(&value, "autolinks")?),
            "default_title" => update.default_title = Some(decode_bool(&value, "default_title")?),
            "keep_inline_images_in" => {
                update.keep_inline_images_in = Some(decode_string_list(&value, "keep_inline_images_in")?)
            }
            "br_in_tables" => update.br_in_tables = Some(decode_bool(&value, "br_in_tables")?),
            "highlight_style" => update.highlight_style = Some(parse_highlight_style(&value)?),
            "extract_metadata" => update.extract_metadata = Some(decode_bool(&value, "extract_metadata")?),
            "whitespace_mode" => update.whitespace_mode = Some(parse_whitespace_mode(&value)?),
            "strip_newlines" => update.strip_newlines = Some(decode_bool(&value, "strip_newlines")?),
            "wrap" => update.wrap = Some(decode_bool(&value, "wrap")?),
            "wrap_width" => update.wrap_width = Some(decode_positive_integer(&value, "wrap_width")?),
            "strip_tags" => update.strip_tags = Some(decode_string_list(&value, "strip_tags")?),
            "preserve_tags" => update.preserve_tags = Some(decode_string_list(&value, "preserve_tags")?),
            "convert_as_inline" => update.convert_as_inline = Some(decode_bool(&value, "convert_as_inline")?),
            "sub_symbol" => update.sub_symbol = Some(decode_string(&value, "sub_symbol")?),
            "sup_symbol" => update.sup_symbol = Some(decode_string(&value, "sup_symbol")?),
            "newline_style" => update.newline_style = Some(parse_newline_style(&value)?),
            "code_block_style" => update.code_block_style = Some(parse_code_block_style(&value)?),
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
                update.remove_navigation = Some(decode_bool(&val, "preprocessing.remove_navigation")?)
            }
            "remove_forms" => update.remove_forms = Some(decode_bool(&val, "preprocessing.remove_forms")?),
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
        "atx_closed" => Ok(HeadingStyle::AtxClosed),
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
