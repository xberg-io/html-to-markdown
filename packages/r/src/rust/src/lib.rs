#![allow(clippy::let_unit_value, deprecated)]

use extendr_api::prelude::*;

mod options;
mod types;

use options::decode_options;
use types::conversion_result_to_robj;

/// Convert HTML to Markdown, returning a named list with:
///   content, metadata, tables, warnings.
/// @param html A character string of HTML content.
/// @param options A named list of conversion options, or NULL for defaults.
/// @return A named list with content (character or NULL), metadata (list), tables (list), warnings (list).
/// @export
#[extendr]
fn convert(html: &str, options: Robj) -> Result<Robj> {
    let opts = decode_options(options).map_err(|e| Error::Other(e))?;
    let result = html_to_markdown_rs::convert(html, Some(opts.clone()))
        .map_err(|e| Error::Other(e.to_string()))?;
    Ok(conversion_result_to_robj(result))
}

extendr_module! {
    mod htmltomarkdown;
    fn convert;
}
