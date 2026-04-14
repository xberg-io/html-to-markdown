//! Main HTML to Markdown conversion API.
//!
//! This module provides the primary `convert()` function for converting HTML to Markdown.

use std::borrow::Cow;

use crate::error::Result;
use crate::options::{ConversionOptions, WhitespaceMode};
use crate::text;
use crate::types::ConversionResult;
use crate::validation::{Utf16Encoding, detect_utf16_encoding, validate_input};
use crate::{ConversionError, ConversionOptionsUpdate};

#[cfg(feature = "inline-images")]
use crate::InlineImageConfig;
#[cfg(feature = "metadata")]
use crate::{HtmlMetadata, MetadataConfig};

/// Convert HTML to Markdown, returning a [`ConversionResult`] with content, metadata, images,
/// and warnings.
///
/// # Arguments
///
/// * `html` - The HTML string to convert
/// * `options` - Optional conversion options (defaults to `ConversionOptions::default()`)
///
/// # Example
///
/// ```
/// use html_to_markdown_rs::{convert, ConversionOptions};
///
/// let html = "<h1>Hello World</h1>";
/// let result = convert(html, None).unwrap();
/// assert!(result.content.as_deref().unwrap_or("").contains("Hello World"));
/// ```
///
/// # Errors
///
/// Returns an error if HTML parsing fails or if the input contains invalid UTF-8.
pub fn convert(html: &str, options: Option<ConversionOptions>) -> Result<ConversionResult> {
    use std::cell::RefCell;
    use std::rc::Rc;

    let options = options.unwrap_or_default();

    let normalized_html = normalize_input(html)?;

    // Fast path: plain text with no HTML tags — skip full parsing pipeline.
    if !options.wrap {
        if let Some(markdown) = fast_text_only(normalized_html.as_ref(), &options) {
            return Ok(ConversionResult {
                content: Some(markdown),
                ..ConversionResult::default()
            });
        }
    }

    // Determine whether metadata / inline-image extraction is requested.
    #[cfg(feature = "metadata")]
    let wants_metadata = options.extract_metadata;
    #[cfg(not(feature = "metadata"))]
    let wants_metadata = false;

    #[cfg(feature = "inline-images")]
    let wants_images = options.extract_images;
    #[cfg(not(feature = "inline-images"))]
    let wants_images = false;

    // Build optional collectors based on requested features.
    #[cfg(feature = "metadata")]
    let metadata_collector = if wants_metadata {
        Some(Rc::new(RefCell::new(crate::metadata::MetadataCollector::new(
            MetadataConfig::default(),
        ))))
    } else {
        None
    };

    #[cfg(feature = "inline-images")]
    let image_collector = if wants_images {
        use crate::inline_images::{DEFAULT_INLINE_IMAGE_LIMIT, InlineImageConfig as IIC};
        Some(Rc::new(RefCell::new(crate::inline_images::InlineImageCollector::new(
            IIC::new(DEFAULT_INLINE_IMAGE_LIMIT),
        )?)))
    } else {
        None
    };

    // Build optional structure collector when requested.
    let structure_collector: Option<std::rc::Rc<std::cell::RefCell<crate::types::StructureCollector>>> =
        if options.include_document_structure {
            Some(std::rc::Rc::new(std::cell::RefCell::new(
                crate::types::StructureCollector::new(),
            )))
        } else {
            None
        };

    // Run the conversion pipeline.
    // Pass structure_collector by value — convert_html_impl will consume it via Rc::try_unwrap
    // to return the finished DocumentStructure. We must not hold a second Rc reference.
    let (markdown, document) = {
        #[cfg(all(feature = "metadata", feature = "inline-images"))]
        {
            crate::converter::convert_html_impl(
                normalized_html.as_ref(),
                &options,
                image_collector.as_ref().map(Rc::clone),
                metadata_collector.as_ref().map(Rc::clone),
                None,
                structure_collector,
            )?
        }
        #[cfg(all(feature = "metadata", not(feature = "inline-images")))]
        {
            crate::converter::convert_html_impl(
                normalized_html.as_ref(),
                &options,
                None,
                metadata_collector.as_ref().map(Rc::clone),
                None,
                structure_collector,
            )?
        }
        #[cfg(all(not(feature = "metadata"), feature = "inline-images"))]
        {
            crate::converter::convert_html_impl(
                normalized_html.as_ref(),
                &options,
                image_collector.as_ref().map(Rc::clone),
                None,
                None,
                structure_collector,
            )?
        }
        #[cfg(all(not(feature = "metadata"), not(feature = "inline-images")))]
        {
            crate::converter::convert_html_impl(
                normalized_html.as_ref(),
                &options,
                None,
                None,
                None,
                structure_collector,
            )?
        }
    };

    let markdown = if options.wrap {
        crate::wrapper::wrap_markdown(&markdown, &options)
    } else {
        markdown
    };

    // Collect metadata if extracted.
    #[cfg(feature = "metadata")]
    let metadata = if let Some(collector) = metadata_collector {
        Rc::try_unwrap(collector)
            .map_err(|_| ConversionError::Other("failed to recover metadata state".to_string()))?
            .into_inner()
            .finish()
    } else {
        HtmlMetadata::default()
    };

    // Collect inline images if extracted.
    #[cfg(feature = "inline-images")]
    let (images, image_warnings) = if let Some(collector) = image_collector {
        let c = Rc::try_unwrap(collector)
            .map_err(|_| ConversionError::Other("failed to recover inline image state".to_string()))?
            .into_inner();
        c.finish()
    } else {
        (Vec::new(), Vec::new())
    };

    // Map InlineImageWarnings → ProcessingWarnings.
    #[cfg(feature = "inline-images")]
    let warnings: Vec<crate::types::ProcessingWarning> = image_warnings
        .into_iter()
        .map(|w| crate::types::ProcessingWarning {
            kind: crate::types::WarningKind::ImageExtractionFailed,
            message: w.message,
        })
        .collect();
    #[cfg(not(feature = "inline-images"))]
    let warnings: Vec<crate::types::ProcessingWarning> = Vec::new();

    let _ = wants_metadata;
    let _ = wants_images;

    Ok(ConversionResult {
        content: Some(markdown),
        document,
        #[cfg(feature = "metadata")]
        metadata,
        tables: Vec::new(),
        #[cfg(feature = "inline-images")]
        images,
        warnings,
    })
}

/// Internal: convert with visitor support. Used by FFI crate.
/// Will be removed when convert() accepts visitor parameter directly.
#[cfg(feature = "visitor")]
#[doc(hidden)]
pub fn convert_with_visitor(
    html: &str,
    options: Option<ConversionOptions>,
    visitor: Option<crate::visitor::VisitorHandle>,
) -> Result<String> {
    let options = options.unwrap_or_default();
    let normalized_html = normalize_input(html)?;
    let markdown = crate::converter::convert_html_with_visitor(normalized_html.as_ref(), &options, visitor)?;
    if options.wrap {
        Ok(crate::wrapper::wrap_markdown(&markdown, &options))
    } else {
        Ok(markdown)
    }
}

/// Validate and normalize HTML input for conversion.
fn normalize_input(html: &str) -> Result<Cow<'_, str>> {
    let decoded = decode_utf16_if_needed(html);
    match decoded {
        Cow::Borrowed(borrowed) => {
            validate_input(borrowed)?;
            let sanitized = strip_nul_bytes(borrowed);
            match sanitized {
                Cow::Borrowed(b) => Ok(normalize_line_endings(b)),
                Cow::Owned(o) => Ok(Cow::Owned(normalize_line_endings(&o).into_owned())),
            }
        }
        Cow::Owned(mut owned) => {
            validate_input(&owned)?;
            if owned.contains('\0') {
                owned = owned.replace('\0', "");
            }
            if owned.contains('\r') {
                owned = owned.replace("\r\n", "\n").replace('\r', "\n");
            }
            Ok(Cow::Owned(owned))
        }
    }
}

/// Attempt to decode UTF-16 HTML that was provided as a lossy UTF-8 string.
///
/// Some callers read raw bytes and convert with `from_utf8_lossy`, which preserves
/// the NUL-byte pattern of UTF-16 input. When we detect that pattern, we can
/// recover the original HTML instead of rejecting it as binary data.
fn decode_utf16_if_needed(html: &str) -> Cow<'_, str> {
    let bytes = html.as_bytes();
    if !bytes.contains(&0) {
        return Cow::Borrowed(html);
    }

    let Some(encoding) = detect_utf16_encoding(bytes) else {
        return Cow::Borrowed(html);
    };

    let decoded = decode_utf16_bytes(bytes, encoding);
    if decoded.is_empty() {
        Cow::Borrowed(html)
    } else {
        Cow::Owned(decoded)
    }
}

fn decode_utf16_bytes(bytes: &[u8], encoding: Utf16Encoding) -> String {
    let (is_little_endian, skip_bom) = match encoding {
        Utf16Encoding::BomLe => (true, true),
        Utf16Encoding::BomBe => (false, true),
        Utf16Encoding::NoBomLe => (true, false),
        Utf16Encoding::NoBomBe => (false, false),
    };

    let mut units = Vec::with_capacity(bytes.len() / 2);
    for chunk in bytes.chunks_exact(2) {
        let unit = if is_little_endian {
            u16::from_le_bytes([chunk[0], chunk[1]])
        } else {
            u16::from_be_bytes([chunk[0], chunk[1]])
        };
        units.push(unit);
    }

    let mut decoded = String::from_utf16_lossy(&units);
    if skip_bom {
        decoded = decoded.trim_start_matches('\u{FEFF}').to_string();
    }
    decoded
}

/// Strip NUL bytes that can appear in malformed HTML inputs.
fn strip_nul_bytes(html: &str) -> Cow<'_, str> {
    if html.contains('\0') {
        Cow::Owned(html.replace('\0', ""))
    } else {
        Cow::Borrowed(html)
    }
}

/// Normalize line endings in HTML input.
///
/// Converts CRLF and CR line endings to LF for consistent processing.
fn normalize_line_endings(html: &str) -> Cow<'_, str> {
    if html.contains('\r') {
        Cow::Owned(html.replace("\r\n", "\n").replace('\r', "\n"))
    } else {
        Cow::Borrowed(html)
    }
}

/// Fast path for plain text (no HTML) conversion.
///
/// Skips HTML parsing if no angle brackets are present.
fn fast_text_only(html: &str, options: &ConversionOptions) -> Option<String> {
    if html.contains('<') {
        return None;
    }

    let mut decoded = text::decode_html_entities_cow(html);
    if options.strip_newlines && (decoded.contains('\n') || decoded.contains('\r')) {
        decoded = Cow::Owned(decoded.replace(&['\r', '\n'][..], " "));
    }
    let trimmed = decoded.trim_end_matches('\n');
    if trimmed.is_empty() {
        return Some(String::new());
    }

    let normalized = if options.whitespace_mode == WhitespaceMode::Normalized {
        text::normalize_whitespace_cow(trimmed)
    } else {
        Cow::Borrowed(trimmed)
    };

    let escaped = if options.output_format == crate::options::OutputFormat::Plain {
        normalized.into_owned()
    } else if options.escape_misc || options.escape_asterisks || options.escape_underscores || options.escape_ascii {
        text::escape(
            normalized.as_ref(),
            options.escape_misc,
            options.escape_asterisks,
            options.escape_underscores,
            options.escape_ascii,
        )
        .into_owned()
    } else {
        normalized.into_owned()
    };

    let mut output = String::with_capacity(escaped.len() + 1);
    output.push_str(&escaped);
    while output.ends_with(' ') || output.ends_with('\t') {
        output.pop();
    }
    output.push('\n');
    Some(output)
}

// ============================================================================
// JSON Configuration Parsing (requires serde feature)
// ============================================================================

#[cfg(any(feature = "serde", feature = "metadata"))]
fn parse_json<T: serde::de::DeserializeOwned>(json: &str) -> Result<T> {
    serde_json::from_str(json).map_err(|err| ConversionError::ConfigError(err.to_string()))
}

#[cfg(any(feature = "serde", feature = "metadata"))]
/// Parse JSON string into `ConversionOptions`.
///
/// Deserializes a JSON string into a full set of conversion options.
/// The JSON can be either a complete or partial options object.
///
/// # Arguments
///
/// * `json` - JSON string representing conversion options
///
/// # Returns
///
/// Fully populated `ConversionOptions` with defaults applied to any unspecified values
///
/// # Errors
///
/// Returns `ConversionError::ConfigError` if JSON parsing fails or contains invalid option values
pub fn conversion_options_from_json(json: &str) -> Result<ConversionOptions> {
    let update: ConversionOptionsUpdate = parse_json(json)?;
    Ok(ConversionOptions::from(update))
}

#[cfg(any(feature = "serde", feature = "metadata"))]
/// Parse JSON string into partial `ConversionOptions` update.
///
/// Deserializes a JSON string into a partial set of conversion options.
/// Only specified options are included; unspecified options are None.
///
/// # Arguments
///
/// * `json` - JSON string representing partial conversion options
///
/// # Returns
///
/// `ConversionOptionsUpdate` with only specified fields populated
///
/// # Errors
///
/// Returns `ConversionError::ConfigError` if JSON parsing fails or contains invalid option values
pub fn conversion_options_update_from_json(json: &str) -> Result<ConversionOptionsUpdate> {
    parse_json(json)
}

#[cfg(all(feature = "inline-images", any(feature = "serde", feature = "metadata")))]
/// Parse JSON string into `InlineImageConfig` (requires `inline-images` feature).
///
/// Deserializes a JSON string into inline image extraction configuration.
/// The JSON can be either a complete or partial configuration object.
///
/// # Arguments
///
/// * `json` - JSON string representing inline image configuration
///
/// # Returns
///
/// Fully populated `InlineImageConfig` with defaults applied to any unspecified values
///
/// # Errors
///
/// Returns `ConversionError::ConfigError` if JSON parsing fails or contains invalid configuration values
pub fn inline_image_config_from_json(json: &str) -> Result<InlineImageConfig> {
    let update: crate::InlineImageConfigUpdate = parse_json(json)?;
    Ok(InlineImageConfig::from_update(update))
}

#[cfg(all(feature = "metadata", any(feature = "serde", feature = "metadata")))]
/// Parse JSON string into `MetadataConfig` (requires `metadata` feature).
///
/// Deserializes a JSON string into metadata extraction configuration.
/// The JSON can be either a complete or partial configuration object.
///
/// # Arguments
///
/// * `json` - JSON string representing metadata extraction configuration
///
/// # Returns
///
/// Fully populated `MetadataConfig` with defaults applied to any unspecified values
///
/// # Errors
///
/// Returns `ConversionError::ConfigError` if JSON parsing fails or contains invalid configuration values
pub fn metadata_config_from_json(json: &str) -> Result<MetadataConfig> {
    let update: crate::MetadataConfigUpdate = parse_json(json)?;
    Ok(MetadataConfig::from(update))
}
