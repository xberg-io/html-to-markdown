//! Accessible-name policy for links and drawn SVG text.

#![cfg(any(feature = "serde", feature = "metadata"))]

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn visible_text_options() -> ConversionOptions {
    serde_json::from_value(serde_json::json!({
        "inline_data_media": "alt_text_only",
        "use_accessible_names": false
    }))
    .expect("accessible-name opt-out should deserialize")
}

#[test]
fn should_exclude_accessible_names_from_the_three_reported_icon_links() {
    let cases = [
        (
            r#"<p><span><a href="https://example.com/v" title="Minimum version"><span><svg width="16" height="16" viewBox="0 0 16 16"><path d="M0 0h16v16z"></path></svg></span></a></span><span><a href="https://example.com/c">0.2.0</a></span> rest</p>"#,
            "[https://example.com/v](https://example.com/v \"Minimum version\")[0.2.0](https://example.com/c) rest\n",
        ),
        (
            r#"<p>alpha <a href="https://example.com/h" aria-label="View commit history for this file."><svg width="16" height="16"><path d="M0 0h16v16z"></path></svg></a> omega</p>"#,
            "alpha [https://example.com/h](https://example.com/h) omega\n",
        ),
        (
            r##"<p>see <a href="https://example.com/x">GitHub<svg aria-label="(opens in new tab)" width="13" height="13"><use href="#ext"></use></svg></a> now</p>"##,
            "see [GitHub](https://example.com/x) now\n",
        ),
    ];
    for tier_strategy in [TierStrategy::Auto, TierStrategy::Tier2] {
        for (html, expected) in cases {
            let options = ConversionOptions {
                tier_strategy,
                ..visible_text_options()
            };
            assert_eq!(convert(html, Some(options)).unwrap().content.unwrap(), expected);
        }
    }
}

#[test]
fn should_keep_drawn_svg_text_but_exclude_labels_and_tooltips() {
    let html = r#"<p><svg aria-label="Accessible"><title>Tooltip</title><desc>Description</desc><text>Drawn</text><svg><title>Nested tooltip</title><text>Nested drawn</text></svg><foreignObject><svg><title>HTML tooltip</title><text>HTML drawn</text></svg></foreignObject></svg></p>"#;
    assert_eq!(
        convert(html, Some(visible_text_options())).unwrap().content.unwrap(),
        "Drawn Nested drawn HTML drawn\n"
    );
}

#[test]
fn should_keep_accessible_names_by_default() {
    let html = r##"<p>see <a href="https://example.com/x">GitHub<svg aria-label="(opens in new tab)"><use href="#ext"></use></svg></a> now</p>"##;
    let options = serde_json::from_value(serde_json::json!({"inline_data_media": "alt_text_only"})).unwrap();
    assert_eq!(
        convert(html, Some(options)).unwrap().content.unwrap(),
        "see [GitHub (opens in new tab)](https://example.com/x) now\n"
    );
}

#[test]
fn should_exclude_svg_names_from_plain_text_output() {
    let mut options = visible_text_options();
    options.output_format = html_to_markdown_rs::OutputFormat::Plain;
    let html =
        r#"<p><svg aria-label="Accessible"><title>Tooltip</title><desc>Description</desc><text>Drawn</text></svg></p>"#;
    assert_eq!(convert(html, Some(options)).unwrap().content.unwrap(), "Drawn\n");
}

#[test]
fn should_support_building_serializing_and_updating_the_name_policy() {
    use html_to_markdown_rs::ConversionOptionsUpdate;

    let mut options = ConversionOptions::builder().use_accessible_names(false).build();
    assert!(!options.use_accessible_names);
    let serialized = serde_json::to_value(&options).unwrap();
    assert_eq!(serialized["use_accessible_names"], false);
    let restored: ConversionOptions = serde_json::from_value(serialized).unwrap();
    assert!(!restored.use_accessible_names);
    options.apply_update(ConversionOptionsUpdate::default());
    assert!(!options.use_accessible_names);
    let update = serde_json::from_value(serde_json::json!({"use_accessible_names": true})).unwrap();
    options.apply_update(update);
    assert!(options.use_accessible_names);
    assert!(
        serde_json::from_value::<ConversionOptions>(serde_json::json!({}))
            .unwrap()
            .use_accessible_names
    );
}

#[test]
fn should_preserve_visible_link_text_image_alt_text_and_link_title_metadata() {
    let html = r#"<p><a href="https://example.com" aria-label="Accessible" title="Tooltip">Visible</a> <img src="data:image/png;base64,AA==" alt="Image alt"></p>"#;
    assert_eq!(
        convert(html, Some(visible_text_options())).unwrap().content.unwrap(),
        "[Visible](https://example.com \"Tooltip\") Image alt\n"
    );
}

#[cfg(feature = "testkit")]
#[test]
fn should_omit_accessible_svg_image_captions_in_both_conversion_tiers() {
    let html = r#"<svg aria-label="Accessible"><title>Tooltip</title><desc>Description</desc><text>Drawn</text></svg>"#;
    let options = ConversionOptions::builder().use_accessible_names(false).build();
    let scanned =
        html_to_markdown_rs::tier1::run(html, &html_to_markdown_rs::prescan::PrescanReport::default(), &options)
            .expect("the SVG scanner must run without falling back to Tier-2");
    let options = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        ..options
    };
    let walked = convert(html, Some(options)).unwrap().content.unwrap();
    for content in [scanned, walked] {
        assert_eq!(content.split_once("](data:image/svg+xml;base64,").unwrap().0, "![Drawn");
    }
}
