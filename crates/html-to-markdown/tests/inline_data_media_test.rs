//! Coverage for `ConversionOptions::inline_data_media`, which chooses what the markdown shows for
//! an image, `<graphic>`, inline `<svg>`, `<video>`, `<audio>` or `<iframe>` whose address is an
//! inline `data:` URL.

use html_to_markdown_rs::{ConversionOptions, HighlightStyle, InlineDataMedia, convert};

const PNG: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==";
const MP4: &str = "data:video/mp4;base64,AAAAIGZ0eXBpc29t";
const MP3: &str = "data:audio/mpeg;base64,SUQzBAAAAAAA";
const HTML_PAGE: &str = "data:text/html,%3Cp%3Ehi%3C%2Fp%3E";

fn convert_with(html: &str, choice: InlineDataMedia) -> String {
    let options = ConversionOptions {
        inline_data_media: choice,
        ..ConversionOptions::default()
    };
    convert(html, Some(options))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

fn all_choices(html: &str) -> [String; 3] {
    [
        convert_with(html, InlineDataMedia::Keep),
        convert_with(html, InlineDataMedia::AltTextOnly),
        convert_with(html, InlineDataMedia::DropElement),
    ]
}

#[test]
fn keep_is_the_default() {
    assert_eq!(ConversionOptions::default().inline_data_media, InlineDataMedia::Keep);
}

#[test]
fn an_image_prints_the_payload_the_alt_text_or_nothing() {
    let html = format!(r#"<p>Before <img src="{PNG}" alt="icon"> after</p>"#);
    let [keep, alt, drop] = all_choices(&html);
    assert_eq!(keep, format!("Before ![icon]({PNG}) after\n"));
    assert_eq!(alt, "Before icon after\n");
    assert_eq!(drop, "Before  after\n");
}

#[test]
fn an_image_uses_a_real_lazy_address_before_a_data_one() {
    let html = format!(r#"<p><img src="{PNG}" data-src="{PNG}" srcset="https://example.com/real.png 2x" alt="p"></p>"#);
    let [keep, alt, drop] = all_choices(&html);
    assert_eq!(keep, format!("![p]({PNG})\n"));
    assert_eq!(alt, "![p](https://example.com/real.png)\n");
    assert_eq!(drop, "![p](https://example.com/real.png)\n");
}

#[test]
fn an_image_with_only_data_addresses_gets_the_choice() {
    let html = format!(r#"<p><img src="{PNG}" data-src="{PNG}" alt="p"></p>"#);
    let [keep, alt, drop] = all_choices(&html);
    assert_eq!(keep, format!("![p]({PNG})\n"));
    assert_eq!(alt, "p\n");
    assert_eq!(drop, "");
}

#[test]
fn an_image_with_a_real_src_is_unchanged() {
    let html = r#"<p><img src="https://example.com/a.png" srcset="data:image/gif;base64,R0lG 1x" alt="a"></p>"#;
    for output in all_choices(html) {
        assert_eq!(output, "![a](https://example.com/a.png)\n");
    }
}

#[test]
fn the_scheme_matches_in_any_case() {
    let html = r#"<p><img src="DATA:image/png;base64,AAAA" alt="icon"></p>"#;
    assert_eq!(convert_with(html, InlineDataMedia::AltTextOnly), "icon\n");
    let html = r#"<p><img src="DATA:image/png;base64,AAAA" data-src="https://example.com/r.png" alt="i"></p>"#;
    assert_eq!(
        convert_with(html, InlineDataMedia::AltTextOnly),
        "![i](https://example.com/r.png)\n"
    );
}

#[test]
fn a_graphic_prints_the_payload_the_alt_text_or_nothing() {
    let html = format!(r#"<p><graphic url="{PNG}" alt="g" /></p>"#);
    let [keep, alt, drop] = all_choices(&html);
    assert_eq!(keep, format!("![g]({PNG})\n"));
    assert_eq!(alt, "g\n");
    assert_eq!(drop, "");
}

#[test]
fn a_graphic_uses_its_next_address_attribute_that_is_not_data() {
    let html = format!(r#"<p><graphic url="{PNG}" src="https://example.com/g.png" alt="g" /></p>"#);
    let [keep, alt, drop] = all_choices(&html);
    assert_eq!(keep, format!("![g]({PNG})\n"));
    assert_eq!(alt, "![g](https://example.com/g.png)\n");
    assert_eq!(drop, "![g](https://example.com/g.png)\n");
}

#[test]
fn a_video_drops_the_link_and_keeps_or_drops_its_fallback_text() {
    let html = format!(r#"<video src="{MP4}">Your browser cannot play this.</video>"#);
    let [keep, alt, drop] = all_choices(&html);
    assert!(keep.contains(MP4), "Keep writes the payload: {keep:?}");
    assert_eq!(alt, "Your browser cannot play this.\n");
    assert_eq!(drop, "");
}

#[test]
fn a_video_falls_back_to_a_nested_source_that_is_not_data() {
    let real = "https://example.com/v.mp4";
    let html = format!(r#"<video src="{MP4}"><source src="{MP4}"><source src="{real}"></video>"#);
    let [keep, alt, drop] = all_choices(&html);
    assert!(keep.contains(MP4), "Keep writes the payload: {keep:?}");
    assert_eq!(alt, format!("[{real}]({real})\n"));
    assert_eq!(drop, format!("[{real}]({real})\n"));
}

#[test]
fn an_audio_element_gets_the_choice() {
    let html = format!(r#"<audio src="{MP3}">No audio.</audio>"#);
    let [keep, alt, drop] = all_choices(&html);
    assert!(keep.contains(MP3), "Keep writes the payload: {keep:?}");
    assert_eq!(alt, "No audio.\n");
    assert_eq!(drop, "");
}

#[test]
fn an_iframe_loses_its_link() {
    let html = format!(r#"<iframe src="{HTML_PAGE}"></iframe>"#);
    let [keep, alt, drop] = all_choices(&html);
    assert!(keep.contains(HTML_PAGE), "Keep writes the payload: {keep:?}");
    assert_eq!(alt, "");
    assert_eq!(drop, "");
}

#[test]
fn an_inline_svg_prints_the_payload_the_title_or_nothing() {
    let html = r#"<p><svg width="1" height="1"><title>Logo</title><rect width="1" height="1"/></svg></p>"#;
    let [keep, alt, drop] = all_choices(html);
    assert!(keep.starts_with("![Logo](data:image/svg+xml;base64,"), "{keep:?}");
    assert_eq!(alt, "Logo\n");
    assert_eq!(drop, "");
}

#[test]
fn a_link_to_a_data_url_is_not_media_and_stays() {
    let html = r#"<p><a href="data:text/plain,hello">file</a></p>"#;
    for output in all_choices(html) {
        assert_eq!(output, "[file](data:text/plain,hello)\n");
    }
}

#[test]
fn the_choice_holds_on_the_fast_path_too() {
    // ~keep With metadata extraction off and no `<mark>` styling, a plain document is eligible
    // ~keep for the Tier-1 scanner, which does not implement the choice; the router must send it
    // ~keep to Tier 2.
    let html = format!(r#"<p>Before <img src="{PNG}" alt="icon"> after</p>"#);
    let options = ConversionOptions {
        inline_data_media: InlineDataMedia::AltTextOnly,
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        ..ConversionOptions::default()
    };
    let output = convert(&html, Some(options)).unwrap().content.unwrap_or_default();
    assert_eq!(output, "Before icon after\n");

    let svg = "<p><svg><title>Logo</title><rect/></svg></p>";
    let options = ConversionOptions {
        inline_data_media: InlineDataMedia::DropElement,
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        ..ConversionOptions::default()
    };
    let output = convert(svg, Some(options)).unwrap().content.unwrap_or_default();
    assert_eq!(output, "");
}

#[cfg(feature = "inline-images")]
#[test]
fn extracted_images_do_not_depend_on_the_choice() {
    let html = format!(r#"<p><img src="{PNG}" alt="icon"></p>"#);
    let options = ConversionOptions {
        inline_data_media: InlineDataMedia::DropElement,
        extract_images: true,
        ..ConversionOptions::default()
    };
    let result = convert(&html, Some(options)).unwrap();
    assert_eq!(result.content.unwrap_or_default(), "");
    assert_eq!(result.images.len(), 1);
}

#[test]
fn parse_accepts_any_spelling_and_falls_back_to_keep() {
    assert_eq!(InlineDataMedia::parse("alt_text_only"), InlineDataMedia::AltTextOnly);
    assert_eq!(InlineDataMedia::parse("AltTextOnly"), InlineDataMedia::AltTextOnly);
    assert_eq!(InlineDataMedia::parse("drop-element"), InlineDataMedia::DropElement);
    assert_eq!(InlineDataMedia::parse("keep"), InlineDataMedia::Keep);
    assert_eq!(InlineDataMedia::parse("bogus"), InlineDataMedia::Keep);
}

#[cfg(feature = "serde")]
#[test]
fn the_option_round_trips_through_serde() {
    let options: ConversionOptions = serde_json::from_str(r#"{"inline_data_media":"drop_element"}"#).unwrap();
    assert_eq!(options.inline_data_media, InlineDataMedia::DropElement);
    let json = serde_json::to_value(InlineDataMedia::AltTextOnly).unwrap();
    assert_eq!(
        serde_json::from_value::<InlineDataMedia>(json).unwrap(),
        InlineDataMedia::AltTextOnly
    );
}
