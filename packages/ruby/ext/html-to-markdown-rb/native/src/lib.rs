#![allow(clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]

use html_to_markdown_rs::{error::ConversionError, safety::guard_panic};

mod conversion;
mod options;
mod types;

use options::build_conversion_options;
use types::{arg_error, runtime_error};

#[cfg(feature = "metadata")]
use conversion::extended_metadata_to_ruby;

use magnus::prelude::*;
use magnus::{Error, Ruby, Value, function, scan_args::scan_args};

fn conversion_error(err: ConversionError) -> Error {
    match err {
        ConversionError::ConfigError(msg) => arg_error(msg),
        ConversionError::Panic(message) => {
            runtime_error(format!("html-to-markdown panic during conversion: {message}"))
        }
        other => runtime_error(other.to_string()),
    }
}

fn convert_full_fn(ruby: &Ruby, args: &[Value]) -> Result<Value, Error> {
    let parsed = scan_args::<(String,), (Option<Value>,), (), (), (), ()>(args)?;
    let html = parsed.required.0;
    let options = build_conversion_options(ruby, parsed.optional.0)?;

    let result = guard_panic(|| html_to_markdown_rs::convert(&html, Some(options.clone())))
        .map_err(conversion_error)?;

    let hash = ruby.hash_new();

    // content: Option<String>
    match result.content {
        Some(ref s) => hash.aset(ruby.intern("content"), s.as_str())?,
        None => hash.aset(ruby.intern("content"), ruby.qnil())?,
    }

    // document: not yet exposed
    hash.aset(ruby.intern("document"), ruby.qnil())?;

    // metadata
    #[cfg(feature = "metadata")]
    {
        let metadata_value = extended_metadata_to_ruby(ruby, result.metadata)?;
        hash.aset(ruby.intern("metadata"), metadata_value)?;
    }
    #[cfg(not(feature = "metadata"))]
    hash.aset(ruby.intern("metadata"), ruby.qnil())?;

    // tables: Vec<TableData> with grid and markdown
    {
        let tables_array = ruby.ary_new();
        for table in &result.tables {
            let table_hash = ruby.hash_new();
            let grid_hash = ruby.hash_new();
            grid_hash.aset(ruby.intern("rows"), table.grid.rows as i64)?;
            grid_hash.aset(ruby.intern("cols"), table.grid.cols as i64)?;
            let cells_array = ruby.ary_new();
            for cell in &table.grid.cells {
                let cell_hash = ruby.hash_new();
                cell_hash.aset(ruby.intern("content"), cell.content.as_str())?;
                cell_hash.aset(ruby.intern("row"), cell.row as i64)?;
                cell_hash.aset(ruby.intern("col"), cell.col as i64)?;
                cell_hash.aset(ruby.intern("row_span"), cell.row_span as i64)?;
                cell_hash.aset(ruby.intern("col_span"), cell.col_span as i64)?;
                cell_hash.aset(ruby.intern("is_header"), cell.is_header)?;
                cells_array.push(cell_hash)?;
            }
            grid_hash.aset(ruby.intern("cells"), cells_array)?;
            table_hash.aset(ruby.intern("grid"), grid_hash)?;
            table_hash.aset(ruby.intern("markdown"), table.markdown.as_str())?;
            tables_array.push(table_hash)?;
        }
        hash.aset(ruby.intern("tables"), tables_array)?;
    }

    // images
    #[cfg(feature = "inline-images")]
    {
        use conversion::inline_image_to_value;
        let images_array = ruby.ary_new();
        for image in result.images {
            let image_value = inline_image_to_value(ruby, image)?;
            images_array.push(image_value)?;
        }
        hash.aset(ruby.intern("images"), images_array)?;
    }
    #[cfg(not(feature = "inline-images"))]
    {
        let empty = ruby.ary_new();
        hash.aset(ruby.intern("images"), empty)?;
    }

    // warnings
    {
        let warnings_array = ruby.ary_new();
        for warning in &result.warnings {
            let w_hash = ruby.hash_new();
            w_hash.aset(ruby.intern("message"), warning.message.as_str())?;
            let kind_str = match warning.kind {
                html_to_markdown_rs::WarningKind::ImageExtractionFailed => "image_extraction_failed",
                html_to_markdown_rs::WarningKind::EncodingFallback => "encoding_fallback",
                html_to_markdown_rs::WarningKind::TruncatedInput => "truncated_input",
                html_to_markdown_rs::WarningKind::MalformedHtml => "malformed_html",
                html_to_markdown_rs::WarningKind::SanitizationApplied => "sanitization_applied",
            };
            w_hash.aset(ruby.intern("kind"), kind_str)?;
            warnings_array.push(w_hash)?;
        }
        hash.aset(ruby.intern("warnings"), warnings_array)?;
    }

    Ok(hash.as_value())
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("HtmlToMarkdown")?;
    module.define_singleton_method("convert", function!(convert_full_fn, -1))?;

    Ok(())
}
