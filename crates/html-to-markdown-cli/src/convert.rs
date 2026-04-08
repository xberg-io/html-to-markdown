#![allow(clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]

use crate::args::Cli;
use crate::output::output_debug_info;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use html_to_markdown_rs::{ConversionOptions, OutputFormat, PreprocessingOptions, convert};
use serde_json::json;

fn base64_encode(data: &[u8]) -> String {
    BASE64.encode(data)
}

pub fn build_conversion_options(cli: &Cli) -> ConversionOptions {
    let defaults = ConversionOptions::default();

    let preprocessing = PreprocessingOptions {
        enabled: cli.preprocess,
        preset: cli.preset.map(Into::into).unwrap_or_default(),
        remove_navigation: !cli.keep_navigation,
        remove_forms: !cli.keep_forms,
    };

    ConversionOptions {
        heading_style: cli.heading_style.map_or(defaults.heading_style, Into::into),
        list_indent_type: cli.list_indent_type.map_or(defaults.list_indent_type, Into::into),
        list_indent_width: cli.list_indent_width.map_or(defaults.list_indent_width, |w| w as usize),
        bullets: cli.bullets.clone().unwrap_or(defaults.bullets),
        strong_em_symbol: cli.strong_em_symbol.unwrap_or(defaults.strong_em_symbol),
        escape_asterisks: cli.escape_asterisks,
        escape_underscores: cli.escape_underscores,
        escape_misc: cli.escape_misc,
        escape_ascii: cli.escape_ascii,
        code_language: cli.code_language.clone().unwrap_or(defaults.code_language),
        autolinks: cli.autolinks,
        default_title: cli.default_title,
        br_in_tables: cli.br_in_tables,
        highlight_style: cli.highlight_style.map_or(defaults.highlight_style, Into::into),
        extract_metadata: cli.extract_metadata,
        whitespace_mode: cli.whitespace_mode.map_or(defaults.whitespace_mode, Into::into),
        strip_newlines: cli.strip_newlines,
        wrap: cli.wrap,
        wrap_width: cli.wrap_width.map_or(defaults.wrap_width, |w| w as usize),
        convert_as_inline: cli.convert_as_inline,
        sub_symbol: cli.sub_symbol.clone().unwrap_or(defaults.sub_symbol),
        sup_symbol: cli.sup_symbol.clone().unwrap_or(defaults.sup_symbol),
        newline_style: cli.newline_style.map_or(defaults.newline_style, Into::into),
        code_block_style: cli.code_block_style.map_or(defaults.code_block_style, Into::into),
        keep_inline_images_in: cli
            .keep_inline_images_in
            .clone()
            .unwrap_or(defaults.keep_inline_images_in),
        link_style: cli.link_style.map_or(defaults.link_style, Into::into),
        skip_images: false,
        preprocessing,
        encoding: cli.encoding.clone(),
        debug: cli.debug,
        strip_tags: cli.strip_tags.clone().unwrap_or(defaults.strip_tags),
        preserve_tags: Vec::new(),
        output_format: cli.output_format.map_or(OutputFormat::default(), Into::into),
        include_document_structure: cli.include_structure,
        extract_images: cli.extract_inline_images,
        max_image_size: 5_242_880,
        capture_svg: false,
        infer_dimensions: true,
        max_depth: None,
    }
}

pub fn perform_conversion(
    html: &str,
    options: ConversionOptions,
    cli: &Cli,
) -> Result<String, Box<dyn std::error::Error>> {
    let output_content = if cli.json {
        // --json path: serialize full ConversionResult fields
        let result = convert(html, Some(options)).map_err(|e| format!("Error converting HTML: {e}"))?;

        // Emit warnings to stderr if requested
        if cli.show_warnings {
            for warning in &result.warnings {
                eprintln!("Warning [{:?}]: {}", warning.kind, warning.message);
            }
        }

        output_debug_info(
            cli,
            &format!(
                "Generated {} bytes of markdown (JSON mode)",
                result.content.as_deref().unwrap_or("").len()
            ),
        );

        // Build JSON output manually to handle feature-gated fields
        let mut json_output = serde_json::Map::new();

        // content field — null when --no-content
        if !cli.no_content {
            json_output.insert(
                "content".into(),
                serde_json::Value::String(result.content.clone().unwrap_or_default()),
            );
        } else {
            json_output.insert("content".into(), serde_json::Value::Null);
        }

        // tables field — always present
        let tables_json = serde_json::to_value(&result.tables).map_err(|e| format!("Error serializing tables: {e}"))?;
        json_output.insert("tables".into(), tables_json);

        // document field — present when --include-structure was set
        let document_json = match &result.document {
            Some(doc) => serde_json::to_value(doc).map_err(|e| format!("Error serializing document: {e}"))?,
            None => serde_json::Value::Null,
        };
        json_output.insert("document".into(), document_json);

        // metadata field — populated via the metadata feature
        let metadata_json =
            serde_json::to_value(&result.metadata).map_err(|e| format!("Error serializing metadata: {e}"))?;
        json_output.insert("metadata".into(), metadata_json);

        // images field — serialized manually since InlineImage doesn't derive serde
        let images_arr: Vec<serde_json::Value> = result
            .images
            .iter()
            .map(|img| {
                json!({
                    "format": format!("{}", img.format),
                    "source": format!("{}", img.source),
                    "filename": img.filename,
                    "description": img.description,
                    "dimensions": img.dimensions.map(|(w, h)| json!({"width": w, "height": h})),
                    "data_base64": base64_encode(&img.data),
                    "attributes": img.attributes,
                })
            })
            .collect();
        json_output.insert("images".into(), serde_json::Value::Array(images_arr));

        // warnings field — always present
        let warnings_json =
            serde_json::to_value(&result.warnings).map_err(|e| format!("Error serializing warnings: {e}"))?;
        json_output.insert("warnings".into(), warnings_json);

        serde_json::to_string_pretty(&serde_json::Value::Object(json_output))
            .map_err(|e| format!("Error serializing JSON output: {e}"))?
    } else {
        // Plain markdown path
        let result = convert(html, Some(options)).map_err(|e| format!("Error converting HTML: {e}"))?;

        // Emit warnings to stderr if requested
        if cli.show_warnings {
            for warning in &result.warnings {
                eprintln!("Warning [{:?}]: {}", warning.kind, warning.message);
            }
        }

        let markdown = result.content.unwrap_or_default();
        output_debug_info(cli, &format!("Generated {} bytes of markdown", markdown.len()));
        markdown
    };

    Ok(output_content)
}
