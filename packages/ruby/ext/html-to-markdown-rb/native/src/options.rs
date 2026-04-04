//! Option parsing and building for Ruby bindings.

use crate::types::{arg_error, symbol_to_string};
use html_to_markdown_rs::{
    CodeBlockStyle, ConversionOptions, ConversionOptionsUpdate, HeadingStyle, HighlightStyle, LinkStyle,
    ListIndentType, NewlineStyle, OutputFormat, PreprocessingOptionsUpdate, PreprocessingPreset, WhitespaceMode,
};
use magnus::prelude::*;
use magnus::r_hash::ForEach;
use magnus::{Error, RArray, RHash, Ruby, TryConvert, Value};

pub fn parse_heading_style(value: Value) -> Result<HeadingStyle, Error> {
    match symbol_to_string(value)?.as_str() {
        "underlined" => Ok(HeadingStyle::Underlined),
        "atx" => Ok(HeadingStyle::Atx),
        "atx_closed" => Ok(HeadingStyle::AtxClosed),
        other => Err(arg_error(format!("invalid heading_style: {other}"))),
    }
}

pub fn parse_list_indent_type(value: Value) -> Result<ListIndentType, Error> {
    match symbol_to_string(value)?.as_str() {
        "spaces" => Ok(ListIndentType::Spaces),
        "tabs" => Ok(ListIndentType::Tabs),
        other => Err(arg_error(format!("invalid list_indent_type: {other}"))),
    }
}

pub fn parse_highlight_style(value: Value) -> Result<HighlightStyle, Error> {
    match symbol_to_string(value)?.as_str() {
        "double_equal" => Ok(HighlightStyle::DoubleEqual),
        "html" => Ok(HighlightStyle::Html),
        "bold" => Ok(HighlightStyle::Bold),
        "none" => Ok(HighlightStyle::None),
        other => Err(arg_error(format!("invalid highlight_style: {other}"))),
    }
}

pub fn parse_whitespace_mode(value: Value) -> Result<WhitespaceMode, Error> {
    match symbol_to_string(value)?.as_str() {
        "normalized" => Ok(WhitespaceMode::Normalized),
        "strict" => Ok(WhitespaceMode::Strict),
        other => Err(arg_error(format!("invalid whitespace_mode: {other}"))),
    }
}

pub fn parse_newline_style(value: Value) -> Result<NewlineStyle, Error> {
    match symbol_to_string(value)?.as_str() {
        "spaces" => Ok(NewlineStyle::Spaces),
        "backslash" => Ok(NewlineStyle::Backslash),
        other => Err(arg_error(format!("invalid newline_style: {other}"))),
    }
}

pub fn parse_code_block_style(value: Value) -> Result<CodeBlockStyle, Error> {
    match symbol_to_string(value)?.as_str() {
        "indented" => Ok(CodeBlockStyle::Indented),
        "backticks" => Ok(CodeBlockStyle::Backticks),
        "tildes" => Ok(CodeBlockStyle::Tildes),
        other => Err(arg_error(format!("invalid code_block_style: {other}"))),
    }
}

pub fn parse_output_format(value: Value) -> Result<OutputFormat, Error> {
    match symbol_to_string(value)?.as_str() {
        "markdown" => Ok(OutputFormat::Markdown),
        "djot" => Ok(OutputFormat::Djot),
        "plain" => Ok(OutputFormat::Plain),
        other => Err(arg_error(format!("invalid output_format: {other}"))),
    }
}

pub fn parse_link_style(value: Value) -> Result<LinkStyle, Error> {
    match symbol_to_string(value)?.as_str() {
        "inline" => Ok(LinkStyle::Inline),
        "reference" => Ok(LinkStyle::Reference),
        other => Err(arg_error(format!("invalid link_style: {other}"))),
    }
}

pub fn parse_preset(value: Value) -> Result<PreprocessingPreset, Error> {
    match symbol_to_string(value)?.as_str() {
        "minimal" => Ok(PreprocessingPreset::Minimal),
        "standard" => Ok(PreprocessingPreset::Standard),
        "aggressive" => Ok(PreprocessingPreset::Aggressive),
        other => Err(arg_error(format!("invalid preprocessing preset: {other}"))),
    }
}

pub fn parse_vec_of_strings(value: Value) -> Result<Vec<String>, Error> {
    let array = RArray::from_value(value).ok_or_else(|| arg_error("expected an Array of strings"))?;
    array.to_vec::<String>()
}

pub fn parse_preprocessing_options(_ruby: &Ruby, value: Value) -> Result<PreprocessingOptionsUpdate, Error> {
    let hash = RHash::from_value(value).ok_or_else(|| arg_error("expected preprocessing to be a Hash"))?;

    let mut update = PreprocessingOptionsUpdate::default();

    hash.foreach(|key: Value, val: Value| {
        let key_name = symbol_to_string(key)?;
        match key_name.as_str() {
            "enabled" => {
                update.enabled = Some(bool::try_convert(val)?);
            }
            "preset" => {
                update.preset = Some(parse_preset(val)?);
            }
            "remove_navigation" => {
                update.remove_navigation = Some(bool::try_convert(val)?);
            }
            "remove_forms" => {
                update.remove_forms = Some(bool::try_convert(val)?);
            }
            _ => {}
        }
        Ok(ForEach::Continue)
    })?;

    Ok(update)
}

pub fn build_conversion_options(ruby: &Ruby, options: Option<Value>) -> Result<ConversionOptions, Error> {
    let mut update = ConversionOptionsUpdate::default();

    let Some(options) = options else {
        return Ok(ConversionOptions::default());
    };

    if options.is_nil() {
        return Ok(ConversionOptions::default());
    }

    let hash = RHash::from_value(options).ok_or_else(|| arg_error("options must be provided as a Hash"))?;

    hash.foreach(|key: Value, val: Value| {
        let key_name = symbol_to_string(key)?;
        match key_name.as_str() {
            "heading_style" => {
                update.heading_style = Some(parse_heading_style(val)?);
            }
            "list_indent_type" => {
                update.list_indent_type = Some(parse_list_indent_type(val)?);
            }
            "list_indent_width" => {
                update.list_indent_width = Some(usize::try_convert(val)?);
            }
            "bullets" => {
                update.bullets = Some(String::try_convert(val)?);
            }
            "strong_em_symbol" => {
                let value = String::try_convert(val)?;
                let mut chars = value.chars();
                let ch = chars
                    .next()
                    .ok_or_else(|| arg_error("strong_em_symbol must not be empty"))?;
                if chars.next().is_some() {
                    return Err(arg_error("strong_em_symbol must be a single character"));
                }
                update.strong_em_symbol = Some(ch);
            }
            "escape_asterisks" => {
                update.escape_asterisks = Some(bool::try_convert(val)?);
            }
            "escape_underscores" => {
                update.escape_underscores = Some(bool::try_convert(val)?);
            }
            "escape_misc" => {
                update.escape_misc = Some(bool::try_convert(val)?);
            }
            "escape_ascii" => {
                update.escape_ascii = Some(bool::try_convert(val)?);
            }
            "code_language" => {
                update.code_language = Some(String::try_convert(val)?);
            }
            "autolinks" => {
                update.autolinks = Some(bool::try_convert(val)?);
            }
            "default_title" => {
                update.default_title = Some(bool::try_convert(val)?);
            }
            "br_in_tables" => {
                update.br_in_tables = Some(bool::try_convert(val)?);
            }
            "highlight_style" => {
                update.highlight_style = Some(parse_highlight_style(val)?);
            }
            "extract_metadata" => {
                update.extract_metadata = Some(bool::try_convert(val)?);
            }
            "whitespace_mode" => {
                update.whitespace_mode = Some(parse_whitespace_mode(val)?);
            }
            "strip_newlines" => {
                update.strip_newlines = Some(bool::try_convert(val)?);
            }
            "wrap" => {
                update.wrap = Some(bool::try_convert(val)?);
            }
            "wrap_width" => {
                update.wrap_width = Some(usize::try_convert(val)?);
            }
            "convert_as_inline" => {
                update.convert_as_inline = Some(bool::try_convert(val)?);
            }
            "sub_symbol" => {
                update.sub_symbol = Some(String::try_convert(val)?);
            }
            "sup_symbol" => {
                update.sup_symbol = Some(String::try_convert(val)?);
            }
            "newline_style" => {
                update.newline_style = Some(parse_newline_style(val)?);
            }
            "code_block_style" => {
                update.code_block_style = Some(parse_code_block_style(val)?);
            }
            "keep_inline_images_in" => {
                update.keep_inline_images_in = Some(parse_vec_of_strings(val)?);
            }
            "preprocessing" => {
                update.preprocessing = Some(parse_preprocessing_options(ruby, val)?);
            }
            "encoding" => {
                update.encoding = Some(String::try_convert(val)?);
            }
            "debug" => {
                update.debug = Some(bool::try_convert(val)?);
            }
            "strip_tags" => {
                update.strip_tags = Some(parse_vec_of_strings(val)?);
            }
            "preserve_tags" => {
                update.preserve_tags = Some(parse_vec_of_strings(val)?);
            }
            "link_style" => {
                update.link_style = Some(parse_link_style(val)?);
            }
            "output_format" => {
                update.output_format = Some(parse_output_format(val)?);
            }
            "max_depth" => {
                update.max_depth = Some(usize::try_convert(val)?);
            }
            _ => {}
        }
        Ok(ForEach::Continue)
    })?;

    Ok(ConversionOptions::from(update))
}
