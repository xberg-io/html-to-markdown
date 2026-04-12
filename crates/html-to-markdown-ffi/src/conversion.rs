//! Basic HTML to Markdown conversion functions for C FFI.
//!
//! This module provides the primary conversion function.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

use html_to_markdown_rs::safety::guard_panic;

fn serialize_conversion_result(result: html_to_markdown_rs::ConversionResult) -> Result<String, String> {
    let document = match result.document {
        Some(doc) => serde_json::to_value(&doc).map_err(|e| e.to_string())?,
        None => serde_json::Value::Null,
    };

    let tables = serde_json::to_value(&result.tables).map_err(|e| e.to_string())?;
    let warnings = serde_json::to_value(&result.warnings).map_err(|e| e.to_string())?;

    #[cfg(feature = "metadata")]
    let metadata = serde_json::to_value(&result.metadata).map_err(|e| e.to_string())?;
    #[cfg(not(feature = "metadata"))]
    let metadata = serde_json::Value::Null;

    #[cfg(feature = "inline-images")]
    let images = {
        let arr: Vec<serde_json::Value> = result
            .images
            .iter()
            .map(|img| {
                serde_json::json!({
                    "format": img.format.to_string(),
                    "filename": img.filename,
                    "description": img.description,
                    "dimensions": img.dimensions.map(|(w, h)| vec![w, h]),
                    "source": img.source.to_string(),
                    "data_len": img.data.len(),
                })
            })
            .collect();
        serde_json::Value::Array(arr)
    };
    #[cfg(not(feature = "inline-images"))]
    let images = serde_json::Value::Array(vec![]);

    let output = serde_json::json!({
        "content": result.content,
        "document": document,
        "metadata": metadata,
        "tables": tables,
        "images": images,
        "warnings": warnings,
    });

    serde_json::to_string(&output).map_err(|e| e.to_string())
}

use crate::error::{HtmlToMarkdownErrorCode, capture_error, set_last_error, set_last_error_code};
use crate::strings::string_to_c_string;

/// Convert HTML to Markdown, returning a JSON string with structured content, metadata, images,
/// and warnings in a single pass. This is the primary C API entry point.
///
/// The returned JSON has the shape:
/// ```json
/// {
///   "content": "..." | null,
///   "document": {...} | null,
///   "metadata": {...} | null,
///   "tables": [{"cells": [[...]], "markdown": "...", "is_header_row": [...]}],
///   "warnings": [{"message": "...", "kind": "..."}]
/// }
/// ```
///
/// # Safety
///
/// - `html` must be a valid null-terminated C string
/// - `options_json` may be NULL (uses defaults) or a valid null-terminated JSON C string
/// - The returned string must be freed with `html_to_markdown_free_string`
/// - Returns NULL on error (check error with `html_to_markdown_last_error`)
///
/// # Example (C)
///
/// ```c
/// const char* html = "<h1>Hello</h1><p>World</p>";
/// char* json = html_to_markdown_convert(html, NULL);
/// if (json != NULL) {
///     printf("%s\n", json);
///     html_to_markdown_free_string(json);
/// }
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn html_to_markdown_convert(html: *const c_char, options_json: *const c_char) -> *mut c_char {
    if html.is_null() {
        set_last_error(Some("html pointer was null".to_string()));
        set_last_error_code(HtmlToMarkdownErrorCode::Internal);
        return ptr::null_mut();
    }

    let html_str = match unsafe { CStr::from_ptr(html) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_last_error(Some("html must be valid UTF-8".to_string()));
            set_last_error_code(HtmlToMarkdownErrorCode::InvalidUtf8);
            return ptr::null_mut();
        }
    };

    let options = if options_json.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(options_json) }.to_str() {
            Ok("") => None,
            Ok(s) => match html_to_markdown_rs::conversion_options_from_json(s) {
                Ok(opts) => Some(opts),
                Err(e) => {
                    set_last_error(Some(format!("failed to parse options JSON: {e}")));
                    set_last_error_code(HtmlToMarkdownErrorCode::Internal);
                    return ptr::null_mut();
                }
            },
            Err(_) => {
                set_last_error(Some("options_json must be valid UTF-8".to_string()));
                set_last_error_code(HtmlToMarkdownErrorCode::InvalidUtf8);
                return ptr::null_mut();
            }
        }
    };

    match guard_panic(|| html_to_markdown_rs::convert(html_str, options)) {
        Ok(result) => {
            set_last_error(None);
            set_last_error_code(HtmlToMarkdownErrorCode::Ok);

            let json = match serialize_conversion_result(result) {
                Ok(j) => j,
                Err(e) => {
                    set_last_error(Some(format!("failed to serialize result to JSON: {e}")));
                    set_last_error_code(HtmlToMarkdownErrorCode::Internal);
                    return ptr::null_mut();
                }
            };

            match string_to_c_string(json, "convert JSON result") {
                Ok(c_string) => c_string.into_raw(),
                Err(err) => {
                    set_last_error(Some(format!("failed to build CString for convert JSON result: {err}")));
                    set_last_error_code(HtmlToMarkdownErrorCode::Internal);
                    ptr::null_mut()
                }
            }
        }
        Err(err) => {
            capture_error(err);
            ptr::null_mut()
        }
    }
}

/// Free a string returned by html_to_markdown_convert.
///
/// Passing NULL is a safe no-op (similar to `free(NULL)` in C).
///
/// # Safety
///
/// - `s` must be a string previously returned by `html_to_markdown_convert`, or NULL
/// - `s` must not be used after this call
///
/// # Example (C)
///
/// ```c
/// char* markdown = html_to_markdown_convert("<p>text</p>");
/// html_to_markdown_free_string(markdown);
/// // markdown is now invalid
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn html_to_markdown_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe { drop(CString::from_raw(s)) };
    }
}
