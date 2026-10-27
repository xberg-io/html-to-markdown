//! Accessible-name policy for links and drawn SVG text.

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert, convert_with_accessible_names};

fn visible_text_options() -> ConversionOptions {
    ConversionOptions {
        inline_data_media: html_to_markdown_rs::InlineDataMedia::AltTextOnly,
        ..ConversionOptions::default()
    }
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
            assert_eq!(
                convert_with_accessible_names(html, false, Some(options))
                    .unwrap()
                    .content
                    .unwrap(),
                expected
            );
        }
    }
}

#[test]
fn should_keep_drawn_svg_text_but_exclude_labels_and_tooltips() {
    let html = r#"<p><svg aria-label="Accessible"><title>Tooltip</title><desc>Description</desc><text>Drawn</text><svg><title>Nested tooltip</title><text>Nested drawn</text></svg><foreignObject><svg><title>HTML tooltip</title><text>HTML drawn</text></svg></foreignObject></svg></p>"#;
    assert_eq!(
        convert_with_accessible_names(html, false, Some(visible_text_options()))
            .unwrap()
            .content
            .unwrap(),
        "Drawn Nested drawn HTML drawn\n"
    );
}

#[test]
fn should_keep_accessible_names_by_default() {
    let html = r##"<p>see <a href="https://example.com/x">GitHub<svg aria-label="(opens in new tab)"><use href="#ext"></use></svg></a> now</p>"##;
    let options = visible_text_options();
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
    assert_eq!(
        convert_with_accessible_names(html, false, Some(options))
            .unwrap()
            .content
            .unwrap(),
        "Drawn\n"
    );
}

#[cfg(any(feature = "serde", feature = "metadata"))]
#[test]
fn should_keep_accessible_policy_outside_serialized_options() {
    let options = serde_json::to_value(ConversionOptions::default()).unwrap();
    assert!(options.get("use_accessible_names").is_none());
    let update = serde_json::to_value(html_to_markdown_rs::ConversionOptionsUpdate::default()).unwrap();
    assert!(update.get("use_accessible_names").is_none());
    let old_key = serde_json::json!({"use_accessible_names": false});
    let options_error = serde_json::from_value::<ConversionOptions>(old_key.clone()).unwrap_err();
    let update_error = serde_json::from_value::<html_to_markdown_rs::ConversionOptionsUpdate>(old_key).unwrap_err();
    for error in [options_error, update_error] {
        assert!(error.to_string().contains("unknown field `use_accessible_names`"));
    }
}

#[test]
fn should_preserve_visible_link_text_image_alt_text_and_link_title_metadata() {
    let html = r#"<p><a href="https://example.com" aria-label="Accessible" title="Tooltip">Visible</a> <img src="data:image/png;base64,AA==" alt="Image alt"></p>"#;
    assert_eq!(
        convert_with_accessible_names(html, false, Some(visible_text_options()))
            .unwrap()
            .content
            .unwrap(),
        "[Visible](https://example.com \"Tooltip\") Image alt\n"
    );
}

#[cfg(feature = "testkit")]
#[test]
fn should_omit_accessible_svg_image_captions_in_both_conversion_tiers() {
    let html = r#"<svg aria-label="Accessible"><title>Tooltip</title><desc>Description</desc><text>Drawn</text></svg>"#;
    for tier_strategy in [TierStrategy::Tier1, TierStrategy::Tier2] {
        let options = ConversionOptions {
            tier_strategy,
            ..ConversionOptions::default()
        };
        let content = convert_with_accessible_names(html, false, options)
            .unwrap()
            .content
            .unwrap();
        assert_eq!(content.split_once("](data:image/svg+xml;base64,").unwrap().0, "![Drawn");
    }
}

#[test]
fn should_isolate_opposite_policies_across_concurrent_calls() {
    let html = r#"<p><svg aria-label="Name"><text>Drawn</text></svg></p>"#;
    let workers: Vec<_> = (0..8)
        .map(|index| {
            std::thread::spawn(move || {
                let enabled = index % 2 == 0;
                for _ in 0..20 {
                    let result = convert_with_accessible_names(html, enabled, visible_text_options()).unwrap();
                    assert_eq!(
                        result.content.as_deref(),
                        Some(if enabled { "Name Drawn\n" } else { "Drawn\n" })
                    );
                }
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
}

#[cfg(feature = "visitor")]
#[test]
fn should_preserve_outer_policy_when_a_visitor_reenters_conversion() {
    use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult, VisitorHandle};
    use std::sync::{Arc, Mutex};

    #[derive(Debug)]
    struct ReentrantVisitor(bool);
    impl HtmlVisitor for ReentrantVisitor {
        fn visit_text(&mut self, _: &NodeContext<'_>, _: &str) -> VisitResult {
            let inner = convert_with_accessible_names(
                r#"<svg aria-label="Inner"><text>Drawn</text></svg>"#,
                !self.0,
                visible_text_options(),
            )
            .unwrap();
            assert_eq!(
                inner.content.as_deref(),
                Some(if self.0 { "Drawn\n" } else { "Inner Drawn\n" })
            );
            VisitResult::Continue
        }
    }
    for enabled in [false, true] {
        let visitor: VisitorHandle = Arc::new(Mutex::new(ReentrantVisitor(enabled)));
        let options = ConversionOptions {
            visitor: Some(visitor),
            ..visible_text_options()
        };
        let outer = convert_with_accessible_names(
            r#"<p>Before <svg aria-label="Outer"><text>Drawn</text></svg> after</p>"#,
            enabled,
            options,
        )
        .unwrap();
        assert_eq!(
            outer.content.as_deref(),
            Some(if enabled {
                "Before Outer Drawn after\n"
            } else {
                "Before Drawn after\n"
            })
        );
    }
}

#[test]
fn should_apply_explicit_and_default_policies_to_plain_text_rewalk() {
    use html_to_markdown_rs::OutputFormat;
    let html = r#"<p><svg aria-label="Name"><title>Tooltip</title><text>Drawn</text><svg><title>Nested</title><text>Child</text></svg></svg></p>"#;
    for tier_strategy in [TierStrategy::Auto, TierStrategy::Tier2] {
        let options = ConversionOptions {
            output_format: OutputFormat::Plain,
            tier_strategy,
            ..visible_text_options()
        };
        let enabled = convert_with_accessible_names(html, true, options.clone()).unwrap();
        let default = convert(html, options.clone()).unwrap();
        let disabled = convert_with_accessible_names(html, false, options).unwrap();
        assert_eq!(enabled.content.as_deref(), Some("Name Tooltip Drawn Nested Child\n"));
        assert_eq!(default.content, enabled.content);
        assert_eq!(disabled.content.as_deref(), Some("Drawn Child\n"));
    }
}
