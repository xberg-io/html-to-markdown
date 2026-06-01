//! Main HTML to Markdown conversion API.
//!
//! This module provides the primary `convert()` function for converting HTML to Markdown.

use std::borrow::Cow;

#[cfg(any(feature = "metadata", feature = "inline-images"))]
use crate::ConversionError;
use crate::error::Result;
use crate::options::{ConversionOptions, WhitespaceMode};
use crate::text;
use crate::types::ConversionResult;
use crate::validation::{Utf16Encoding, detect_utf16_encoding, validate_input};

#[cfg(feature = "metadata")]
use crate::{HtmlMetadata, MetadataConfig};

/// Convert HTML to Markdown, returning a [`ConversionResult`] with content, metadata, images,
/// and warnings.
///
/// # Arguments
///
/// * `html` — the HTML string to convert.
/// * `options` — conversion options. The parameter bound is
///   `impl Into<Option<ConversionOptions>>`, so any of the following call shapes are accepted:
///   - `convert(html, ConversionOptions::default())` — bare options.
///   - `convert(html, opts)` — bare options.
///   - `convert(html, Some(opts))` — explicit `Option`.
///   - `convert(html, None)` — fall back to [`ConversionOptions::default`].
///
/// # Example
///
/// ```
/// use html_to_markdown_rs::{convert, ConversionOptions};
///
/// let html = "<h1>Hello World</h1>";
///
/// // Bare options — most ergonomic.
/// let result = convert(html, ConversionOptions::default()).unwrap();
/// assert!(result.content.as_deref().unwrap_or("").contains("Hello World"));
///
/// // `None` falls back to defaults.
/// let result = convert(html, None).unwrap();
/// assert!(result.content.as_deref().unwrap_or("").contains("Hello World"));
/// ```
///
/// # Errors
///
/// Returns an error if HTML parsing fails or if the input contains invalid UTF-8.
pub fn convert(html: &str, options: impl Into<Option<ConversionOptions>>) -> Result<ConversionResult> {
    #[cfg(any(feature = "metadata", feature = "inline-images"))]
    use std::cell::RefCell;
    #[cfg(any(feature = "metadata", feature = "inline-images"))]
    use std::rc::Rc;

    let options = options.into().unwrap_or_default();

    // Tier-1 dispatcher.
    //
    // `TierStrategy::Tier2` skips this block entirely and falls straight to
    // the Tier-2 pipeline below.
    //
    // `TierStrategy::Auto` runs the prescan + classifier once.  If the
    // classifier returns `RouterDecision::Tier1`, the scanner is invoked.  On
    // success the result is returned immediately.  On bail the normalized input
    // that was already produced is passed directly to `convert_html_impl`
    // (Tier-2) — no re-normalisation, no second prescan.  The prescan for the
    // Tier-2 path is performed again inside `convert_html_impl`; this wastes
    // one prescan in the bail case but keeps the byte-equality contract intact
    // because `convert_html_impl` owns its own preprocessing state.
    //
    // `TierStrategy::Tier1` (testkit-only) bypasses the classifier and forces
    // the scanner unconditionally, still with Tier-2 fallback on bail.
    match options.tier_strategy {
        crate::options::TierStrategy::Tier2 => {
            // Skip Tier-1 entirely; fall through to the Tier-2 path below.
        }
        crate::options::TierStrategy::Auto => {
            let normalized = normalize_input(html)?;
            let (cleaned, report) = crate::converter::prescan::run(normalized.as_ref());
            let decision = crate::converter::tier1::router::classify(&report, &options);
            if decision == crate::converter::tier1::RouterDecision::Tier1 {
                match crate::converter::tier1::run(cleaned.as_ref(), &report, &options) {
                    Ok(markdown) => {
                        return Ok(crate::types::ConversionResult {
                            content: Some(markdown),
                            document: None,
                            tables: Vec::new(),
                            warnings: Vec::new(),
                            #[cfg(feature = "metadata")]
                            metadata: crate::metadata::HtmlMetadata::default(),
                            #[cfg(feature = "inline-images")]
                            images: Vec::new(),
                        });
                    }
                    Err(_bail) => {
                        // Fall through to the Tier-2 path below.
                        // `normalized_html` is re-computed from `html` below
                        // to keep the Tier-2 path self-contained.
                    }
                }
            }
            // RouterDecision::Tier2 or Tier-1 bail: fall through to Tier-2.
        }
        #[cfg(any(test, feature = "testkit"))]
        crate::options::TierStrategy::Tier1 => {
            // Testkit path: bypass the classifier and force Tier-1, with
            // Tier-2 fallback on bail.
            let normalized = normalize_input(html)?;
            let (cleaned, report) = crate::converter::prescan::run(normalized.as_ref());
            match crate::converter::tier1::run(cleaned.as_ref(), &report, &options) {
                Ok(markdown) => {
                    return Ok(crate::types::ConversionResult {
                        content: Some(markdown),
                        document: None,
                        tables: Vec::new(),
                        warnings: Vec::new(),
                        #[cfg(feature = "metadata")]
                        metadata: crate::metadata::HtmlMetadata::default(),
                        #[cfg(feature = "inline-images")]
                        images: Vec::new(),
                    });
                }
                Err(_bail) => {
                    // Fall through to Tier-2.
                }
            }
        }
    }

    #[cfg(feature = "visitor")]
    let visitor = options.visitor.clone();

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

    #[cfg(not(feature = "visitor"))]
    let visitor: Option<()> = None;

    // Run the conversion pipeline.
    // Pass structure_collector by value — convert_html_impl will consume it via Rc::try_unwrap
    // to return the finished DocumentStructure. We must not hold a second Rc reference.
    let (markdown, document, tables) = {
        #[cfg(all(feature = "metadata", feature = "inline-images"))]
        {
            crate::converter::convert_html_impl(
                normalized_html.as_ref(),
                &options,
                image_collector.as_ref().map(Rc::clone),
                metadata_collector.as_ref().map(Rc::clone),
                visitor,
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
                visitor,
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
                visitor,
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
                visitor,
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
        tables,
        #[cfg(feature = "inline-images")]
        images,
        warnings,
    })
}

/// Validate and normalize HTML input for conversion.
fn normalize_input(html: &str) -> Result<Cow<'_, str>> {
    let decoded = decode_utf16_if_needed(html);
    match decoded {
        Cow::Borrowed(borrowed) => {
            validate_input(borrowed)?;
            let sanitized = strip_nul_bytes(borrowed);
            let line_normalized = match sanitized {
                Cow::Borrowed(b) => normalize_line_endings(b),
                Cow::Owned(o) => Cow::Owned(normalize_line_endings(&o).into_owned()),
            };
            Ok(fix_xhtml_self_closing(line_normalized))
        }
        Cow::Owned(mut owned) => {
            validate_input(&owned)?;
            if owned.contains('\0') {
                owned = owned.replace('\0', "");
            }
            if owned.contains('\r') {
                owned = owned.replace("\r\n", "\n").replace('\r', "\n");
            }
            Ok(fix_xhtml_self_closing(Cow::Owned(owned)))
        }
    }
}

/// Insert a space before `/>` in XHTML-style self-closing tags so the underlying
/// HTML parser does not greedily consume the trailing slash as part of the tag name.
///
/// The bundled astral-tl parser treats `/` as an identifier character, so `<td/>`
/// is parsed as a tag literally named `"td/"` and subsequent siblings become its
/// children — silently truncating the table and dropping the rest of the document.
/// Rewriting to `<td />` (with a space) lets the parser recognise the self-closing
/// syntax correctly. EPUB/XHTML-derived HTML uses this form heavily for empty
/// table cells; see issue #391.
fn fix_xhtml_self_closing(html: Cow<'_, str>) -> Cow<'_, str> {
    use std::sync::OnceLock;
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    // Match `<tag/>` (no whitespace, no attributes) where `tag` is a valid HTML
    // tag name. Tag names per the HTML spec must start with a letter and may
    // contain letters, digits, hyphens, underscores, colons (namespaces), and
    // periods. The replacement inserts a single space before the `/`.
    let re = RE.get_or_init(|| {
        regex::Regex::new(r"<([a-zA-Z][a-zA-Z0-9_:.\-]*)/>").expect("XHTML self-closing regex is well-formed")
    });
    if !html.contains("/>") {
        return html;
    }
    match re.replace_all(html.as_ref(), "<$1 />") {
        Cow::Borrowed(_) => html,
        Cow::Owned(s) => Cow::Owned(s),
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
