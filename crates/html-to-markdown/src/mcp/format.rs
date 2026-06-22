//! Conversion result formatting for MCP wire output.

use crate::types::ConversionResult;

/// Format a `ConversionResult` as pretty-printed JSON for MCP wire output.
///
/// Mirrors the manual JSON construction in `html-to-markdown-cli/src/convert.rs`
/// to handle the feature-gated `images` field (which does not derive `serde`).
pub fn format_conversion_result(result: &ConversionResult) -> String {
    let mut map = serde_json::Map::new();

    // content
    map.insert(
        "content".into(),
        match &result.content {
            Some(c) => serde_json::Value::String(c.clone()),
            None => serde_json::Value::Null,
        },
    );

    // tables
    let tables_val = serde_json::to_value(&result.tables).unwrap_or(serde_json::Value::Array(Vec::new()));
    map.insert("tables".into(), tables_val);

    // document
    let doc_val = match &result.document {
        Some(doc) => serde_json::to_value(doc).unwrap_or(serde_json::Value::Null),
        None => serde_json::Value::Null,
    };
    map.insert("document".into(), doc_val);

    // metadata (feature-gated)
    #[cfg(feature = "metadata")]
    {
        let meta_val = serde_json::to_value(&result.metadata).unwrap_or(serde_json::Value::Null);
        map.insert("metadata".into(), meta_val);
    }

    // images — manual serialisation because InlineImage does not derive serde
    #[cfg(feature = "inline-images")]
    {
        use base64::Engine as _;
        let images_arr: Vec<serde_json::Value> = result
            .images
            .iter()
            .map(|img| {
                serde_json::json!({
                    "format": format!("{}", img.format),
                    "source": format!("{}", img.source),
                    "filename": img.filename,
                    "description": img.description,
                    "dimensions": img.dimensions.map(|d| serde_json::json!({"width": d.width, "height": d.height})),
                    "data_base64": base64::engine::general_purpose::STANDARD.encode(&img.data),
                    "attributes": img.attributes,
                })
            })
            .collect();
        map.insert("images".into(), serde_json::Value::Array(images_arr));
    }

    // warnings
    let warnings_val = serde_json::to_value(&result.warnings).unwrap_or(serde_json::Value::Array(Vec::new()));
    map.insert("warnings".into(), warnings_val);

    serde_json::to_string_pretty(&serde_json::Value::Object(map)).unwrap_or_default()
}

/// Format extracted [`HtmlMetadata`](crate::metadata::HtmlMetadata) as pretty-printed JSON.
///
/// Used by the `extract_metadata` tool. `HtmlMetadata` derives `Serialize`, so this
/// is a direct serialisation (no feature-gated fields to special-case).
pub fn format_metadata(metadata: &crate::metadata::HtmlMetadata) -> String {
    serde_json::to_string_pretty(metadata).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_result_contains_content_field() {
        let result = ConversionResult {
            content: Some("# Hello".into()),
            ..ConversionResult::default()
        };
        let json = format_conversion_result(&result);
        let parsed: serde_json::Value = serde_json::from_str(&json).expect("must be valid JSON");
        assert_eq!(parsed["content"], "# Hello");
    }

    #[test]
    fn test_format_result_null_content_when_none() {
        let result = ConversionResult {
            content: None,
            ..ConversionResult::default()
        };
        let json = format_conversion_result(&result);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["content"].is_null());
    }

    #[test]
    fn test_format_result_warnings_present() {
        let result = ConversionResult::default();
        let json = format_conversion_result(&result);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.get("warnings").is_some());
    }

    #[test]
    fn test_format_metadata_is_valid_json() {
        let metadata = crate::metadata::HtmlMetadata::default();
        let json = format_metadata(&metadata);
        let parsed: serde_json::Value = serde_json::from_str(&json).expect("metadata must be valid JSON");
        assert!(
            parsed.get("document").is_some(),
            "metadata JSON must have document field"
        );
    }
}
