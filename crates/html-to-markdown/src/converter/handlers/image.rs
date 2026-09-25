//! Image element handler for HTML to Markdown conversion.
//!
//! Handles `<img>` elements including:
//! - Basic image markdown output `![alt](src "title")`
//! - Metadata collection for image extraction
//! - Inline data URI image handling
//! - Visitor callback integration

use std::borrow::Cow;
#[cfg(any(feature = "metadata", feature = "inline-images"))]
use std::collections::BTreeMap;

use crate::converter::Context;
use crate::converter::dom_context::DomContext;
use crate::converter::inline::link::{append_url_destination, escape_markdown_title};
use crate::converter::utility::attributes::decoded_attribute;
use crate::converter::utility::content::escape_link_label;
use crate::converter::utility::preprocessing::sanitize_markdown_url;
use crate::options::ConversionOptions;

#[cfg(feature = "inline-images")]
use crate::converter::media::handle_inline_data_image;

#[cfg(feature = "visitor")]
use crate::converter::utility::serialization::serialize_node;

#[cfg(feature = "metadata")]
type ImageMetadataPayload = (BTreeMap<String, String>, Option<u32>, Option<u32>);

/// Handle an `<img>` element and convert to Markdown.
///
/// This handler processes image elements including:
/// - Extracting src, alt, and title attributes
/// - Collecting metadata when the metadata feature is enabled
/// - Handling inline data URIs when the inline-images feature is enabled
/// - Invoking visitor callbacks when the visitor feature is enabled
/// - Generating appropriate markdown output
#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle_img(
    node_handle: &tl::NodeHandle,
    tag: &tl::HTMLTag,
    parser: &tl::Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    let src: Cow<'_, str> = {
        let resolved = resolve_effective_src(tag);
        Cow::Owned(sanitize_markdown_url(&resolved).into_owned())
    };

    let alt = decoded_attribute(tag, "alt").unwrap_or(Cow::Borrowed(""));

    // ~keep An empty `title=""` carries no information, and `[t](u "")` / `![a](i "")` is
    // ~keep noise that no Markdown serializer round-trips: re-rendering the output drops
    // ~keep the empty title, so the second pass no longer matches the first. Treat it as
    // ~keep absent, which is what it means.
    let title = decoded_attribute(tag, "title").filter(|v| !v.is_empty());

    #[cfg(feature = "metadata")]
    #[allow(clippy::useless_let_if_seq)]
    let mut metadata_payload: Option<ImageMetadataPayload> = None;
    #[cfg(feature = "metadata")]
    if ctx.metadata_wants_images {
        let mut attributes_map = BTreeMap::new();
        let mut width: Option<u32> = None;
        let mut height: Option<u32> = None;
        for (key, value_opt) in tag.attributes().iter() {
            let key_str = key.to_string();
            if key_str == "src" {
                continue;
            }
            let value = value_opt.map(|v| v.to_string()).unwrap_or_default();
            if key_str == "width" {
                if let Ok(parsed) = value.parse::<u32>() {
                    width = Some(parsed);
                }
            } else if key_str == "height" {
                if let Ok(parsed) = value.parse::<u32>() {
                    height = Some(parsed);
                }
            }
            attributes_map.insert(key_str, value);
        }
        metadata_payload = Some((attributes_map, width, height));
    }

    #[cfg(feature = "inline-images")]
    if let Some(ref collector_ref) = ctx.inline_collector {
        if src.trim_start().starts_with("data:") {
            let mut attributes_map = BTreeMap::new();
            for (key, value_opt) in tag.attributes().iter() {
                let key_str = key.to_string();
                let keep = key_str == "width"
                    || key_str == "height"
                    || key_str == "filename"
                    || key_str == "aria-label"
                    || key_str.starts_with("data-");
                if keep {
                    let value = value_opt.map(|value| value.to_string()).unwrap_or_default();
                    attributes_map.insert(key_str, value);
                }
            }
            handle_inline_data_image(
                collector_ref,
                src.as_ref(),
                alt.as_ref(),
                title.as_deref(),
                attributes_map,
            );
        }
    }

    // ~keep #492: `|| ctx.link_allow_inline_images` is purely additive relative to the
    // ~keep pre-#492 expression -- it can only turn an image ON, never off, because
    // ~keep `link_allow_inline_images` is `false` whenever "a" is absent from
    // ~keep `keep_inline_images_in` (its only source, `handlers/link.rs`). So callers who
    // ~keep never set the option see byte-identical output. Deliberately NOT mirrored with
    // ~keep the heading pattern's negative clause (`|| (in_link && !link_allow)`): the no-`<p>`
    // ~keep control already renders an image through the inline branch with
    // ~keep `convert_as_inline == false`, so `should_use_alt_text` is `false` there today and
    // ~keep the image survives -- a negative term would delete it, which is a real regression.
    let keep_as_markdown = (ctx.in_heading && ctx.heading_allow_inline_images)
        || ctx.cell_allow_inline_images
        || ctx.link_allow_inline_images;

    let should_use_alt_text =
        !keep_as_markdown && (ctx.convert_as_inline || (ctx.in_heading && !ctx.heading_allow_inline_images));

    #[cfg(feature = "visitor")]
    let image_output = if let Some(ref visitor_handle) = ctx.visitor {
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

        let node_ctx = NodeContext::with_lazy_attributes(
            NodeType::Image,
            Cow::Borrowed("img"),
            tag,
            depth,
            index_in_parent,
            parent_tag.map(Cow::Borrowed),
            true,
        );

        let visit_result = {
            let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
            visitor.visit_image(&node_ctx, &src, &alt, title.as_deref())
        };
        match visit_result {
            VisitResult::Continue => Some(format_image_markdown(
                &src,
                &alt,
                title.as_deref(),
                should_use_alt_text,
                options.link_style,
                options.url_escape_style,
                ctx.reference_collector.as_ref(),
            )),
            VisitResult::Custom(custom) => Some(custom),
            VisitResult::Skip => None,
            VisitResult::Error(err) => {
                if ctx.visitor_error.borrow().is_none() {
                    *ctx.visitor_error.borrow_mut() = Some(err);
                }
                None
            }
            VisitResult::PreserveHtml => Some(serialize_node(node_handle, parser)),
        }
    } else {
        Some(format_image_markdown(
            &src,
            &alt,
            title.as_deref(),
            should_use_alt_text,
            options.link_style,
            options.url_escape_style,
            ctx.reference_collector.as_ref(),
        ))
    };

    #[cfg(not(feature = "visitor"))]
    let image_output = Some(format_image_markdown(
        &src,
        &alt,
        title.as_deref(),
        should_use_alt_text,
        options.link_style,
        options.url_escape_style,
        ctx.reference_collector.as_ref(),
    ));

    if !options.skip_images {
        if let Some(img_text) = image_output {
            output.push_str(&img_text);
        }
    }

    #[cfg(feature = "metadata")]
    if ctx.metadata_wants_images {
        if let Some(ref collector) = ctx.metadata_collector {
            if let Some((attributes_map, width, height)) = metadata_payload {
                if !src.is_empty() {
                    let dimensions = match (width, height) {
                        (Some(w), Some(h)) => Some(crate::metadata::ImageDimensions { width: w, height: h }),
                        _ => None,
                    };
                    collector.borrow_mut().add_image(
                        src.to_string(),
                        if alt.is_empty() { None } else { Some(alt.to_string()) },
                        title.as_deref().map(std::string::ToString::to_string),
                        dimensions,
                        attributes_map,
                    );
                }
            }
        }
    }

    if let Some(ref sc) = ctx.structure_collector {
        let src_opt = if src.is_empty() { None } else { Some(src.as_ref()) };
        let alt_opt = if alt.is_empty() { None } else { Some(alt.as_ref()) };
        sc.borrow_mut().push_image(src_opt, alt_opt);
    }
}

/// Single-URL attributes checked, in priority order, when `src` is missing or a
/// `data:` URI. See [`resolve_effective_src`] for the full precedence rule.
const LAZY_SINGLE_URL_ATTRIBUTES: [&str; 3] = ["data-src", "data-lazy-src", "data-original"];

/// Resolve the effective image address for an `<img>` element, falling back to
/// common lazy-loading attributes when `src` is empty or a `data:` URI.
///
/// ~keep Precedence, and why:
/// ~keep 1. `src`, when non-empty and not a `data:` URI — trusted as-is even if it
/// ~keep    happens to be a tiny placeholder graphic, because this crate cannot fetch
/// ~keep    the URL to inspect its pixel dimensions, and some real pages already have
/// ~keep    the genuine image in `src` while only the *other* attributes carry lazy
/// ~keep    -load scaffolding (see `test_documents/html/issues/gh-190/rbloggers.html`'s
/// ~keep    "jetpack-lazy-image" `<img>`: `src` is already the real photo URL, while
/// ~keep    `srcset` holds nothing but the lazy-load 1x1 `data:` placeholder — treating
/// ~keep    every populated `src` with suspicion would discard that real image).
/// ~keep 2. `data-src`, `data-lazy-src`, `data-original`, in that order — each holds
/// ~keep    exactly one URL, deliberately written by a lazy-load library as "the real
/// ~keep    address", so they outrank the multi-candidate srcset attributes below,
/// ~keep    which require guessing which listed candidate is "best".
/// ~keep 3. `data-srcset`, then `srcset` — each may list several URLs with width/
/// ~keep    density descriptors; the highest-resolution candidate is used (see
/// ~keep    `pick_best_srcset_candidate`). `data-srcset` (explicitly a lazy-load
/// ~keep    attribute) outranks plain `srcset`, which may itself have been
/// ~keep    placeholder-ized by the same lazy-load pass that placeholder-ized `src`.
/// ~keep 4. Otherwise, the original `src` (possibly empty, possibly a `data:` URI) is
/// ~keep    kept unchanged. This is also what makes a plain `<img src="...">` with none
/// ~keep    of the above attributes byte-identical to output produced before this
/// ~keep    fallback existed.
fn resolve_effective_src<'a>(tag: &'a tl::HTMLTag<'a>) -> Cow<'a, str> {
    // ~keep Every read here goes through `decoded_attribute`: a URL attribute carries
    // ~keep character references like any other (`src="i.png?a=1&amp;b=2"`), and `srcset` is
    // ~keep parsed *after* decoding because the entity is not part of its comma/descriptor
    // ~keep grammar. Issue #494.
    let raw_src = decoded_attribute(tag, "src").unwrap_or(Cow::Borrowed(""));

    if !raw_src.trim().is_empty() && !raw_src.trim_start().starts_with("data:") {
        return raw_src;
    }

    for attr_name in LAZY_SINGLE_URL_ATTRIBUTES {
        if let Some(value) = decoded_attribute(tag, attr_name) {
            if !value.trim().is_empty() {
                return value;
            }
        }
    }

    for attr_name in ["data-srcset", "srcset"] {
        if let Some(value) = decoded_attribute(tag, attr_name) {
            if let Some(candidate) = pick_best_srcset_candidate(&value) {
                return Cow::Owned(candidate.to_string());
            }
        }
    }

    raw_src
}

/// Parse a `srcset`-shaped attribute value and return the URL of its highest
/// -resolution candidate.
///
/// ~keep `srcset` lists one or more `"<url> [descriptor]"` candidates separated by
/// ~keep commas, where a descriptor is a pixel density (`2x`) or a width (`800w`).
/// ~keep Markdown has no responsive-image equivalent, so this picks a single URL: the
/// ~keep candidate with the largest numeric descriptor, i.e. the highest-quality image
/// ~keep offered (a high-resolution image degrades gracefully wherever it is viewed; a
/// ~keep low-resolution one does not). A candidate with no descriptor is treated as the
/// ~keep first candidate when no other candidate carries one either — the HTML spec
/// ~keep allows at most one descriptor-less candidate, so there is nothing to compare it
/// ~keep against in that case.
/// ~keep
/// ~keep Splitting on `,` is a simplification of the full HTML `srcset` grammar, which
/// ~keep in rare cases allows a literal comma inside an unescaped URL; it matches every
/// ~keep real-world `srcset` value this crate has been fed.
fn pick_best_srcset_candidate(value: &str) -> Option<&str> {
    let mut best: Option<(&str, f64)> = None;
    let mut first_url: Option<&str> = None;

    for entry in value.split(',') {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }
        let mut parts = entry.split_whitespace();
        let Some(url) = parts.next() else {
            continue;
        };
        if first_url.is_none() {
            first_url = Some(url);
        }

        let score = parts.next().and_then(|descriptor| {
            if descriptor.len() < 2 || !(descriptor.ends_with('w') || descriptor.ends_with('x')) {
                return None;
            }
            descriptor[..descriptor.len() - 1].parse::<f64>().ok()
        });

        if let Some(score) = score {
            let is_better = match best {
                Some((_, best_score)) => score > best_score,
                None => true,
            };
            if is_better {
                best = Some((url, score));
            }
        }
    }

    best.map(|(url, _)| url).or(first_url)
}

/// Format an image as Markdown syntax.
///
/// If `use_alt_only` is true, returns just the alt text.
/// Otherwise returns the full `![alt](src "title")` syntax.
///
/// The `url_escape_style` controls how the `src` URL is escaped:
/// - [`UrlEscapeStyle::Angle`] (default) — wraps `src` in angle brackets when it contains spaces.
/// - [`UrlEscapeStyle::Percent`] — percent-encodes every non-unreserved character.
fn format_image_markdown(
    src: &str,
    alt: &str,
    title: Option<&str>,
    use_alt_only: bool,
    link_style: crate::options::validation::LinkStyle,
    url_escape_style: crate::options::validation::UrlEscapeStyle,
    reference_collector: Option<&crate::converter::reference_collector::ReferenceCollectorHandle>,
) -> String {
    if use_alt_only {
        return alt.to_string();
    }
    let escaped_alt = escape_link_label(alt);
    if link_style == crate::options::validation::LinkStyle::Reference {
        if let Some(collector) = reference_collector {
            let ref_num = collector.borrow_mut().get_or_insert(src, title);
            let mut buf = String::with_capacity(escaped_alt.len() + 10);
            buf.push_str("![");
            buf.push_str(&escaped_alt);
            buf.push_str("][");
            buf.push_str(&ref_num.to_string());
            buf.push(']');
            return buf;
        }
    }
    let mut buf = String::with_capacity(src.len() + escaped_alt.len() + 10);
    buf.push_str("![");
    buf.push_str(&escaped_alt);
    buf.push_str("](");

    append_url_destination(&mut buf, src, url_escape_style, title.is_some());

    if let Some(title_text) = title {
        buf.push_str(" \"");
        buf.push_str(&escape_markdown_title(title_text));
        buf.push('"');
    }
    buf.push(')');
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::validation::{LinkStyle, UrlEscapeStyle};

    #[test]
    fn format_image_markdown_angle_wraps_space() {
        let result = format_image_markdown(
            "/img (1).png",
            "alt",
            None,
            false,
            LinkStyle::Inline,
            UrlEscapeStyle::Angle,
            None,
        );
        assert_eq!(result, "![alt](</img (1).png>)");
    }

    #[test]
    fn format_image_markdown_percent_encodes_space_and_parens() {
        let result = format_image_markdown(
            "/img (1).png",
            "alt",
            None,
            false,
            LinkStyle::Inline,
            UrlEscapeStyle::Percent,
            None,
        );
        assert_eq!(result, "![alt](/img%20%281%29.png)");
    }

    // ~keep Regression for the CommonMark spec fixpoint gap (spec example 520):
    // ~keep `<img alt="[foo](uri2)">` previously emitted `![[foo](uri2)](uri3)`, which
    // ~keep reparses with the alt text's `[foo](uri2)` becoming a real nested link --
    // ~keep CommonMark parses an image's alt as full inline content, so only `foo`
    // ~keep survived into the alt attribute on a second conversion and `uri2` was lost.
    #[test]
    fn format_image_markdown_escapes_link_shaped_alt_text() {
        let result = format_image_markdown(
            "uri3",
            "[foo](uri2)",
            None,
            false,
            LinkStyle::Inline,
            UrlEscapeStyle::Angle,
            None,
        );
        assert_eq!(result, "![\\[foo\\](uri2)](uri3)");
    }

    #[test]
    fn format_image_markdown_percent_encodes_angle_brackets() {
        let result = format_image_markdown(
            "/img (1) <draft>.png",
            "alt",
            None,
            false,
            LinkStyle::Inline,
            UrlEscapeStyle::Percent,
            None,
        );
        assert_eq!(result, "![alt](/img%20%281%29%20%3Cdraft%3E.png)");
    }

    #[test]
    fn format_image_markdown_angle_plain_url_unchanged() {
        let result = format_image_markdown(
            "https://example.com/img.png",
            "photo",
            None,
            false,
            LinkStyle::Inline,
            UrlEscapeStyle::Angle,
            None,
        );
        assert_eq!(result, "![photo](https://example.com/img.png)");
    }

    #[test]
    fn should_escape_alt_text_that_would_close_the_image_and_open_a_new_link() {
        // ~keep audit #24 finding 1: an inert `alt` containing `]` and `(` must not be able to
        // terminate the image label early and start a second, attacker-controlled link.
        let result = format_image_markdown(
            "x.png",
            "a](https://evil.example/payload)",
            None,
            false,
            LinkStyle::Inline,
            UrlEscapeStyle::Angle,
            None,
        );
        assert_eq!(result, "![a\\](https://evil.example/payload)](x.png)");
    }

    #[test]
    fn should_escape_alt_text_in_reference_style_images_too() {
        // The same alt-escaping bug existed independently in the reference-style branch.
        let collector = crate::converter::reference_collector::ReferenceCollector::new();
        let handle = std::rc::Rc::new(std::cell::RefCell::new(collector));
        let result = format_image_markdown(
            "x.png",
            "a](https://evil.example/payload)",
            None,
            false,
            LinkStyle::Reference,
            UrlEscapeStyle::Angle,
            Some(&handle),
        );
        assert_eq!(result, "![a\\](https://evil.example/payload)][1]");
    }

    #[test]
    fn should_escape_a_quote_in_the_title_that_would_open_a_real_link_after_it() {
        // ~keep audit #24 finding 4: an inert `title` containing `"` must not be able to close the
        // title early, turning the rest of the title text into document markdown (e.g. a link).
        let result = format_image_markdown(
            "a.png",
            "photo",
            Some("x\" [click](https://evil.example)"),
            false,
            LinkStyle::Inline,
            UrlEscapeStyle::Angle,
            None,
        );
        assert_eq!(result, "![photo](a.png \"x\\\" [click](https://evil.example)\")");
    }

    #[test]
    fn should_reject_out_of_order_parens_in_src_like_append_markdown_link_does() {
        // ~keep audit #24 finding 7: a naive open-count == close-count check treats ")(" as
        // balanced. Reuses `append_url_destination`, which applies the same
        // `parens_are_balanced` check already used (and tested) for `<a href>`.
        let result = format_image_markdown(
            "a)(b.png",
            "alt",
            None,
            false,
            LinkStyle::Inline,
            UrlEscapeStyle::Angle,
            None,
        );
        assert_eq!(result, "![alt](a\\)\\(b.png)");
    }
}
