use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendHashTable, Zval};
use html_to_markdown_rs::{ConversionOptions, ConversionOptionsUpdate};

use crate::enums::*;
use crate::types::*;

/// Parse a ConversionOptions table from PHP hash table.
pub fn parse_conversion_options(table: &ZendHashTable) -> PhpResult<ConversionOptions> {
    let mut update = ConversionOptionsUpdate::default();

    for (key, value) in table {
        let key_str = key_to_string(&key)?;

        if value.is_null() {
            continue;
        }

        match key_str.as_str() {
            "heading_style" | "headingStyle" => {
                update.heading_style = Some(parse_heading_style(value, &key_str)?);
            }
            "list_indent_type" | "listIndentType" => {
                update.list_indent_type = Some(parse_list_indent_type(value, &key_str)?);
            }
            "list_indent_width" | "listIndentWidth" => {
                update.list_indent_width = Some(read_usize(value, &key_str)?);
            }
            "bullets" => {
                update.bullets = Some(read_string(value, &key_str)?);
            }
            "strong_em_symbol" | "strongEmSymbol" => {
                update.strong_em_symbol = Some(parse_single_char(value, &key_str)?);
            }
            "escape_asterisks" | "escapeAsterisks" => {
                update.escape_asterisks = Some(read_bool(value, &key_str)?);
            }
            "escape_underscores" | "escapeUnderscores" => {
                update.escape_underscores = Some(read_bool(value, &key_str)?);
            }
            "escape_misc" | "escapeMisc" => {
                update.escape_misc = Some(read_bool(value, &key_str)?);
            }
            "escape_ascii" | "escapeAscii" => {
                update.escape_ascii = Some(read_bool(value, &key_str)?);
            }
            "code_language" | "codeLanguage" => {
                update.code_language = Some(read_string(value, &key_str)?);
            }
            "autolinks" => {
                update.autolinks = Some(read_bool(value, &key_str)?);
            }
            "default_title" | "defaultTitle" => {
                update.default_title = Some(read_bool(value, &key_str)?);
            }
            "br_in_tables" | "brInTables" => {
                update.br_in_tables = Some(read_bool(value, &key_str)?);
            }
            "highlight_style" | "highlightStyle" => {
                update.highlight_style = Some(parse_highlight_style(value, &key_str)?);
            }
            "extract_metadata" | "extractMetadata" => {
                update.extract_metadata = Some(read_bool(value, &key_str)?);
            }
            "whitespace_mode" | "whitespaceMode" => {
                update.whitespace_mode = Some(parse_whitespace_mode(value, &key_str)?);
            }
            "strip_newlines" | "stripNewlines" => {
                update.strip_newlines = Some(read_bool(value, &key_str)?);
            }
            "wrap" => {
                update.wrap = Some(read_bool(value, &key_str)?);
            }
            "wrap_width" | "wrapWidth" => {
                update.wrap_width = Some(read_usize(value, &key_str)?);
            }
            "convert_as_inline" | "convertAsInline" => {
                update.convert_as_inline = Some(read_bool(value, &key_str)?);
            }
            "sub_symbol" | "subSymbol" => {
                update.sub_symbol = Some(read_string(value, &key_str)?);
            }
            "sup_symbol" | "supSymbol" => {
                update.sup_symbol = Some(read_string(value, &key_str)?);
            }
            "newline_style" | "newlineStyle" => {
                update.newline_style = Some(parse_newline_style(value, &key_str)?);
            }
            "code_block_style" | "codeBlockStyle" => {
                update.code_block_style = Some(parse_code_block_style(value, &key_str)?);
            }
            "keep_inline_images_in" | "keepInlineImagesIn" => {
                update.keep_inline_images_in = Some(read_string_list(value, &key_str)?);
            }
            "preprocessing" => {
                update.preprocessing = Some(parse_preprocessing_options(value, &key_str)?);
            }
            "encoding" => {
                update.encoding = Some(read_string(value, &key_str)?);
            }
            "debug" => {
                update.debug = Some(read_bool(value, &key_str)?);
            }
            "skip_images" | "skipImages" => {
                update.skip_images = Some(read_bool(value, &key_str)?);
            }
            "strip_tags" | "stripTags" => {
                update.strip_tags = Some(read_string_list(value, &key_str)?);
            }
            "preserve_tags" | "preserveTags" => {
                update.preserve_tags = Some(read_string_list(value, &key_str)?);
            }
            "link_style" | "linkStyle" => {
                update.link_style = Some(parse_link_style(value, &key_str)?);
            }
            "output_format" | "outputFormat" => {
                update.output_format = Some(parse_output_format(value, &key_str)?);
            }
            "include_document_structure" | "includeDocumentStructure" => {
                update.include_document_structure = Some(read_bool(value, &key_str)?);
            }
            "extract_images" | "extractImages" => {
                update.extract_images = Some(read_bool(value, &key_str)?);
            }
            "max_image_size" | "maxImageSize" => {
                update.max_image_size = Some(read_u64(value, &key_str)?);
            }
            "capture_svg" | "captureSvg" => {
                update.capture_svg = Some(read_bool(value, &key_str)?);
            }
            "infer_dimensions" | "inferDimensions" => {
                update.infer_dimensions = Some(read_bool(value, &key_str)?);
            }
            "max_depth" | "maxDepth" => {
                update.max_depth = Some(read_usize(value, &key_str)?);
            }
            _ => {}
        }
    }

    Ok(ConversionOptions::from(update))
}

/// Parse preprocessing options from a PHP hash table.
fn parse_preprocessing_options(value: &Zval, key: &str) -> PhpResult<html_to_markdown_rs::PreprocessingOptionsUpdate> {
    let table = value
        .array()
        .ok_or_else(|| PhpException::default(format!("'{key}' must be an associative array")))?;

    let mut update = html_to_markdown_rs::PreprocessingOptionsUpdate::default();

    for (entry_key, entry_value) in table {
        let entry_name = key_to_string(&entry_key)?;

        if entry_value.is_null() {
            continue;
        }

        match entry_name.as_str() {
            "enabled" => {
                update.enabled = Some(read_bool(entry_value, &format!("{key}.enabled"))?);
            }
            "preset" => {
                update.preset = Some(parse_preprocessing_preset(entry_value, &format!("{key}.preset"))?);
            }
            "remove_navigation" | "removeNavigation" => {
                update.remove_navigation = Some(read_bool(entry_value, &format!("{key}.remove_navigation"))?);
            }
            "remove_forms" | "removeForms" => {
                update.remove_forms = Some(read_bool(entry_value, &format!("{key}.remove_forms"))?);
            }
            _ => {}
        }
    }

    Ok(update)
}
