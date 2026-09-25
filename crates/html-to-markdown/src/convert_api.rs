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

/// Convert HTML to Markdown, Djot, or plain text.
///
/// Returns a [`ConversionResult`] with converted content plus optional metadata,
/// document structure, table data, inline images, and warnings depending on the
/// enabled features and conversion options.
///
/// # Arguments
///
/// * `html` — the HTML string to convert.
/// * `options` — conversion options. Rust accepts bare [`ConversionOptions`],
///   `Some(options)`, or `None`. Language bindings expose the same option
///   fields through native constructors or optional parameters.
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
///
/// # Observability
///
/// Emits an `html_to_markdown::convert` span at `INFO` level with fields `input_len`,
/// `output_format`, `wrap`, `extract_metadata`, `extract_images`, and `tier_strategy`. These
/// field names are part of the public observability contract and are kept stable across
/// releases. This crate never installs a `tracing` subscriber — attach one in the consuming
/// application to observe these spans and events.
#[tracing::instrument(
    level = "info",
    name = "html_to_markdown::convert",
    skip_all,
    fields(
        input_len = html.len(),
        output_format = tracing::field::Empty,
        wrap = tracing::field::Empty,
        extract_metadata = tracing::field::Empty,
        extract_images = tracing::field::Empty,
        tier_strategy = tracing::field::Empty,
    )
)]
pub fn convert(html: &str, options: impl Into<Option<ConversionOptions>>) -> Result<ConversionResult> {
    let options = options.into().unwrap_or_default();

    let span = tracing::Span::current();
    span.record("output_format", tracing::field::debug(options.output_format));
    span.record("wrap", options.wrap);
    span.record("extract_metadata", options.extract_metadata);
    span.record("extract_images", options.extract_images);
    span.record("tier_strategy", tracing::field::debug(options.tier_strategy));

    // ~keep Thin generic wrapper. Delegates to the non-generic `convert_inner` so the
    // ~keep large body monomorphises exactly once instead of once per `Into` impl
    // ~keep the caller picks. See xberg-io/html-to-markdown#398.
    convert_inner(html, options)
}

fn convert_inner(html: &str, options: ConversionOptions) -> Result<ConversionResult> {
    #[cfg(any(feature = "metadata", feature = "inline-images"))]
    use std::cell::RefCell;
    #[cfg(any(feature = "metadata", feature = "inline-images"))]
    use std::rc::Rc;

    // ~keep Tier-1 dispatcher.
    // ~keep
    // ~keep `TierStrategy::Tier2` skips this block entirely and falls straight to
    // ~keep the Tier-2 pipeline below.
    // ~keep
    // ~keep `TierStrategy::Auto` runs the prescan + classifier once.  If the
    // ~keep classifier returns `RouterDecision::Tier1`, the scanner is invoked.  On
    // ~keep success the result is returned immediately.  On bail the normalized input
    // ~keep that was already produced is threaded to the Tier-2 pipeline via
    // ~keep `precomputed_normalized` — no re-normalisation.
    // ~keep
    // ~keep `TierStrategy::Tier1` (testkit-only) bypasses the classifier and forces
    // ~keep the scanner unconditionally, still with Tier-2 fallback on bail.
    // ~keep
    // ~keep `precomputed_normalized` carries the `Cow<str>` produced by
    // ~keep `normalize_input` when the Tier-1 path ran it.  The Tier-2 entry point
    // ~keep below uses it directly; the `Tier2` branch leaves it `None` and computes
    // ~keep it there.
    let mut precomputed_normalized: Option<Cow<'_, str>> = None;

    match options.tier_strategy {
        crate::options::TierStrategy::Tier2 => {
            // ~keep Skip Tier-1 entirely; fall through to the Tier-2 path below.
        }
        crate::options::TierStrategy::Auto => {
            // ~keep Phase C: skip the prescan pre-pass for the Tier-1 attempt.  The
            // ~keep scanner now handles every construct prescan used to strip
            // ~keep (script/style, head, meta/link, doctype, comments, self-closing
            // ~keep void tags) inline, and bails cleanly on the constructs the
            // ~keep router used to gate on (SVG, CDATA, custom elements, bare `<`).
            // ~keep For routing we still consult the option-based gates in
            // ~keep `classify`; we pass a default `PrescanReport` whose fields are
            // ~keep all false because the scanner will detect any structural
            // ~keep edge-case during its single walk.
            let normalized = normalize_input(html)?;
            let stub_report = crate::converter::prescan::PrescanReport::default();
            let decision = crate::converter::tier1::router::classify(&stub_report, &options);
            if decision == crate::converter::tier1::RouterDecision::Tier1 {
                match crate::converter::tier1::run(normalized.as_ref(), &stub_report, &options) {
                    Ok(markdown) => {
                        tracing::debug!(
                            target: "html_to_markdown::convert",
                            output_len = markdown.len(),
                            "tier-1 fast-path conversion completed"
                        );
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
                    Err(bail) => {
                        // ~keep Tier-1 bailed — fall through to Tier-2 with the
                        // ~keep already-normalized input.  Tier-2 runs its own
                        // ~keep prescan internally via `convert_html_impl`.
                        tracing::warn!(
                            target: "html_to_markdown::convert",
                            reason = %bail,
                            "tier-1 conversion bailed; falling back to tier-2"
                        );
                        precomputed_normalized = Some(normalized);
                    }
                }
            } else {
                // ~keep RouterDecision::Tier2: fall through with the already-normalized input.
                tracing::debug!(target: "html_to_markdown::convert", "router selected tier-2 conversion path directly");
                precomputed_normalized = Some(normalized);
            }
        }
        #[cfg(any(test, feature = "testkit"))]
        crate::options::TierStrategy::Tier1 => {
            // ~keep Testkit path: bypass the classifier and force Tier-1, with
            // ~keep Tier-2 fallback on bail.  Like the Auto path, skip the prescan
            // ~keep pre-pass — the scanner handles every construct it would have
            // ~keep stripped or bails cleanly.
            let normalized = normalize_input(html)?;
            let stub_report = crate::converter::prescan::PrescanReport::default();
            match crate::converter::tier1::run(normalized.as_ref(), &stub_report, &options) {
                Ok(markdown) => {
                    tracing::debug!(
                        target: "html_to_markdown::convert",
                        output_len = markdown.len(),
                        "tier-1 fast-path conversion completed"
                    );
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
                Err(bail) => {
                    tracing::warn!(
                        target: "html_to_markdown::convert",
                        reason = %bail,
                        "tier-1 conversion bailed; falling back to tier-2"
                    );
                    precomputed_normalized = Some(normalized);
                }
            }
        }
    }

    #[cfg(feature = "visitor")]
    let visitor = options.visitor.clone();

    let normalized_html = match precomputed_normalized {
        Some(n) => n,
        None => normalize_input(html)?,
    };

    if !options.wrap {
        if let Some(markdown) = fast_text_only(normalized_html.as_ref(), &options) {
            return Ok(ConversionResult {
                content: Some(markdown),
                ..ConversionResult::default()
            });
        }
    }

    #[cfg(feature = "metadata")]
    let wants_metadata = options.extract_metadata;
    #[cfg(not(feature = "metadata"))]
    let wants_metadata = false;

    #[cfg(feature = "inline-images")]
    let wants_images = options.extract_images;
    #[cfg(not(feature = "inline-images"))]
    let wants_images = false;

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
        use crate::inline_images::InlineImageConfig as IIC;
        // ~keep `IIC::new` only seeds its own defaults, and two of them are the INVERSE of
        // ~keep the documented `ConversionOptions` defaults (`capture_svg` true vs false,
        // ~keep `infer_dimensions` false vs true). Forwarding is therefore not optional
        // ~keep polish: without it every caller silently gets both flipped, and
        // ~keep `max_image_size` is ignored because it only ever coincided with the constant.
        let mut config = IIC::new(options.max_image_size);
        config.capture_svg = options.capture_svg;
        config.infer_dimensions = options.infer_dimensions;
        Some(Rc::new(RefCell::new(crate::inline_images::InlineImageCollector::new(
            config,
        )?)))
    } else {
        None
    };

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

    // ~keep Pass structure_collector by value — convert_html_impl will consume it via Rc::try_unwrap
    // ~keep to return the finished DocumentStructure. We must not hold a second Rc reference.
    //
    // ~keep The whole pipeline runs inside `catch_unwind`: a panicking visitor callback
    // ~keep poisons the visitor's `Mutex` (std::sync::Mutex poisons on an unwind while
    // ~keep the guard is held). Without a catch here, that panic would unwind straight
    // ~keep out of `convert()`, and any *later* call reusing the same visitor handle
    // ~keep would find it permanently poisoned. Catching it here confines the failure
    // ~keep to this call and lets us clear the poison flag below. See
    // ~keep xberg-io/html-to-markdown#28.
    type ConvertOutput = (
        String,
        Option<crate::types::DocumentStructure>,
        Vec<crate::types::TableData>,
        Option<crate::types::ProcessingWarning>,
    );
    let convert_outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<ConvertOutput> {
        #[cfg(all(feature = "metadata", feature = "inline-images"))]
        {
            crate::converter::convert_html_impl(
                normalized_html.as_ref(),
                &options,
                image_collector.as_ref().map(Rc::clone),
                metadata_collector.as_ref().map(Rc::clone),
                visitor,
                structure_collector,
            )
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
            )
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
            )
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
            )
        }
    }));

    let (markdown, document, tables, depth_warning) = match convert_outcome {
        Ok(result) => result?,
        Err(panic_payload) => {
            // ~keep Clear the poison flag so a later, unrelated conversion that reuses this
            // ~keep same visitor handle is not permanently latched into failure. The state
            // ~keep guarded by this Mutex is the caller's own visitor object; no other
            // ~keep thread can observe it mid-mutation because every access is serialised
            // ~keep through this same lock, so recovering it here cannot expose a torn read.
            #[cfg(feature = "visitor")]
            if let Some(handle) = &options.visitor {
                handle.clear_poison();
            }
            return Err(crate::error::ConversionError::Panic(panic_message(&*panic_payload)));
        }
    };

    let markdown = if options.wrap {
        crate::wrapper::wrap_markdown(&markdown, &options)
    } else {
        markdown
    };

    #[cfg(feature = "metadata")]
    let metadata = if let Some(collector) = metadata_collector {
        Rc::try_unwrap(collector)
            .map_err(|_| ConversionError::Other("failed to recover metadata state".to_string()))?
            .into_inner()
            .finish()
    } else {
        HtmlMetadata::default()
    };

    #[cfg(feature = "inline-images")]
    let (images, image_warnings) = if let Some(collector) = image_collector {
        let c = Rc::try_unwrap(collector)
            .map_err(|_| ConversionError::Other("failed to recover inline image state".to_string()))?
            .into_inner();
        c.finish()
    } else {
        (Vec::new(), Vec::new())
    };

    #[cfg(feature = "inline-images")]
    let mut warnings: Vec<crate::types::ProcessingWarning> = image_warnings
        .into_iter()
        .map(|w| crate::types::ProcessingWarning {
            kind: crate::types::WarningKind::ImageExtractionFailed,
            message: w.message,
        })
        .collect();
    #[cfg(not(feature = "inline-images"))]
    let mut warnings: Vec<crate::types::ProcessingWarning> = Vec::new();
    if let Some(warning) = depth_warning {
        warnings.push(warning);
    }

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

/// Extract a human-readable message from a `catch_unwind` panic payload.
///
/// Panics started via `panic!("{}", msg)` carry a `String`; panics started via a string
/// literal (`panic!("msg")`) carry a `&'static str`. Anything else falls back to a generic
/// message rather than losing the error entirely.
fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(message) = payload.downcast_ref::<&str>() {
        (*message).to_string()
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else {
        "visitor callback panicked during conversion".to_string()
    }
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
    // ~keep Whitespace-only input renders as nothing in a browser, so it converts to "" —
    // ~keep not to the lone "\n" the trailing-space pop plus unconditional newline below
    // ~keep would otherwise emit.
    if trimmed.bytes().all(|byte| byte.is_ascii_whitespace()) {
        return Some(String::new());
    }

    let normalized = if options.whitespace_mode == WhitespaceMode::Normalized {
        text::normalize_whitespace_cow(trimmed)
    } else {
        Cow::Borrowed(trimmed)
    };

    let escaped = if options.output_format == crate::options::OutputFormat::Plain {
        normalized.into_owned()
    } else if options.escape_misc
        || options.escape_asterisks
        || options.escape_underscores
        || options.escape_ascii
        // ~keep `text::escape` escapes a literal backslash regardless of the flags above,
        // ~keep so this gate must open on a bare backslash too or the tag-free fast path
        // ~keep would silently skip it under default options.
        || normalized.contains('\\')
    {
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
