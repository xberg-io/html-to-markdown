#![allow(missing_docs)]

//! ~keep A tag name ends at whitespace, a slash or a greater-than sign, and at nothing else. An element
//! ~keep with a dot, a colon, an underscore or a digit in its name closes at its end tag, as an element
//! ~keep with a hyphen in its name does, and a link after it is not inside the link before it.

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

/// ~keep Each way the converter reads a page: by default, on the fast path that asks for no metadata,
/// ~keep on the tree path, and on the byte scanner.
fn routes() -> Vec<(&'static str, ConversionOptions)> {
    let with = |tier_strategy, extract_metadata| ConversionOptions {
        tier_strategy,
        extract_metadata,
        ..ConversionOptions::default()
    };
    vec![
        ("auto", with(TierStrategy::Auto, true)),
        ("auto without metadata", with(TierStrategy::Auto, false)),
        ("tier2", with(TierStrategy::Tier2, true)),
        #[cfg(feature = "testkit")]
        ("tier1", with(TierStrategy::Tier1, false)),
    ]
}

fn markdown(html: &str, options: &ConversionOptions) -> String {
    convert(html, options.clone())
        .expect("conversion succeeds")
        .content
        .unwrap_or_default()
}

/// ~keep The links of the page as (href, text), in order, with the metadata of the default route and
/// ~keep of the tree path.
#[cfg(feature = "metadata")]
fn link_texts(html: &str, tier_strategy: TierStrategy) -> Vec<(String, String)> {
    let options = ConversionOptions {
        tier_strategy,
        extract_metadata: true,
        ..ConversionOptions::default()
    };
    convert(html, options)
        .expect("conversion succeeds")
        .metadata
        .links
        .into_iter()
        .map(|link| (link.href, link.text))
        .collect()
}

const NAMES: [&str; 12] = [
    "x.y",
    "x:y",
    "x_y",
    "x1",
    "x.1",
    "a.b.c",
    "X.Y",
    "x.y_z:1",
    "produits.promopct",
    "x..y",
    "x.y-z",
    "x$y",
];

fn pages(name: &str) -> Vec<String> {
    let upper = name.to_ascii_uppercase();
    vec![
        format!(
            r#"<a href="/1"><div>One</div><b>-<{name}>%</{name}></b></a><a href="/2">Two</a><a href="/3">Three</a>"#
        ),
        format!(r#"<a href="/1"><b>-<{name}/>%</b></a><a href="/2">Two</a><a href="/3">Three</a>"#),
        format!(r#"<a href="/1"><b>-<{name}>%</{upper}></b></a><a href="/2">Two</a><a href="/3">Three</a>"#),
        format!(r#"<a href="/1"><b>-<{name}>%</b></a><a href="/2">Two</a><a href="/3">Three</a>"#),
        format!(r#"<a href="/1"><b>-<{name} id="i">%</{name}></b></a><a href="/2">Two</a><a href="/3">Three</a>"#),
    ]
}

#[test]
fn should_close_a_dotted_tag_name_at_its_end_tag() {
    let html = r#"<a href="/1"><div>One</div><b>-<x.y>%</x.y></b></a><a href="/2">Two</a><a href="/3">Three</a>"#;
    for (route, options) in routes() {
        assert_eq!(
            markdown(html, &options),
            "[One **-%**](/1)[Two](/2)[Three](/3)\n",
            "{route}"
        );
    }
}

#[cfg(feature = "metadata")]
#[test]
fn should_not_nest_later_links_in_a_link_that_holds_a_dotted_tag_name() {
    let html = r#"<a href="/1"><div>One</div><b>-<x.y>%</x.y></b></a><a href="/2">Two</a><a href="/3">Three</a>"#;
    for route in [TierStrategy::Auto, TierStrategy::Tier2] {
        let links = link_texts(html, route);
        let expected = [("/1", "One **-%**"), ("/2", "Two"), ("/3", "Three")];
        assert_eq!(links.len(), expected.len(), "{route:?}: {links:?}");
        for ((href, text), (want_href, want_text)) in links.iter().zip(expected) {
            assert_eq!((href.as_str(), text.as_str()), (want_href, want_text), "{route:?}");
        }
    }
}

#[test]
fn should_read_the_trigger_placeholder_like_an_unknown_element() {
    let html = r#"<a href="/1"><span><produits.promopct>%</produits.promopct></span></a><a href="/2">Two</a>"#;
    for (route, options) in routes() {
        assert_eq!(markdown(html, &options), "[%](/1)[Two](/2)\n", "{route}");
    }
}

#[test]
fn should_read_every_tag_name_like_a_name_with_a_hyphen() {
    let default = ConversionOptions::default();
    let mut differing = Vec::new();
    for name in NAMES {
        for (index, page) in pages(name).iter().enumerate() {
            let oracle_page = &pages("x-y")[index];
            let oracle = markdown(oracle_page, &default);
            for (route, options) in routes() {
                if markdown(page, &options) != oracle {
                    differing.push(format!("{route}: {page}"));
                }
            }
            #[cfg(feature = "metadata")]
            {
                let oracle_links = link_texts(oracle_page, TierStrategy::Auto);
                for route in [TierStrategy::Auto, TierStrategy::Tier2] {
                    if link_texts(page, route) != oracle_links {
                        differing.push(format!("links {route:?}: {page}"));
                    }
                }
            }
        }
    }
    assert!(
        differing.is_empty(),
        "{} differ:\n{}",
        differing.len(),
        differing.join("\n")
    );
}

#[test]
fn should_keep_the_markdown_of_a_page_with_no_unusual_tag_name() {
    let html = r#"<a href="/1"><b>One</b></a><a href="/2">Two</a>"#;
    for (route, options) in routes() {
        assert_eq!(markdown(html, &options), "[**One**](/1)[Two](/2)\n", "{route}");
    }
}

/// ~keep Every byte that HTML keeps in a tag name, in the middle of a name, read like a hyphen.
#[test]
fn should_read_each_printable_byte_of_a_name_like_a_hyphen() {
    let default = ConversionOptions::default();
    let mut differing = Vec::new();
    let mut names = 0;
    for byte in (0x21_u8..0x7f).filter(|byte| !matches!(byte, b'/' | b'>')) {
        let name = format!("x{}y", char::from(byte));
        names += 1;
        for (index, page) in pages(&name).iter().enumerate() {
            let oracle = markdown(&pages("x-y")[index], &default);
            for (route, options) in routes() {
                if markdown(page, &options) != oracle {
                    differing.push(format!("{route}: {page}"));
                }
            }
        }
    }
    assert_eq!(names, 92);
    assert!(
        differing.is_empty(),
        "{} differ:\n{}",
        differing.len(),
        differing.join("\n")
    );
}

#[test]
fn should_close_a_dotted_tag_name_at_an_end_tag_with_white_space() {
    let html = r#"<a href="/1"><b>-<x.y>%</x.y ></b></a><a href="/2">Two</a>"#;
    let oracle = r#"<a href="/1"><b>-<x-y>%</x-y ></b></a><a href="/2">Two</a>"#;
    for (route, options) in routes() {
        assert_eq!(markdown(html, &options), markdown(oracle, &options), "{route}");
    }
}

#[test]
fn should_not_read_a_dotted_name_as_the_known_element_before_the_dot() {
    for (html, want) in [
        ("<p><b.c>t</b.c></p>", "t\n"),
        ("<p><script.x>t</script.x></p>", "t\n"),
        ("<p>s<style.y>t</style.y></p>", "st\n"),
        ("<p>s<br.z>t</p>", "st\n"),
    ] {
        // ~keep The byte scanner reads a name up to the dot, as it did before the move. The auto route
        // ~keep never sends these pages to it, so only the routes a caller reaches are checked here.
        for (route, options) in routes().into_iter().filter(|(route, _)| *route != "tier1") {
            assert_eq!(markdown(html, &options), want, "{route}: {html}");
        }
    }
}

#[test]
fn should_keep_a_page_that_holds_a_tag_named_like_an_encoding_apart() {
    let html = "<x_x2e_y>a</x_x2e_y><x.y>b</x.y>";
    let options = ConversionOptions {
        preserve_tags: vec!["x_x2e_y".to_owned()],
        ..ConversionOptions::default()
    };
    assert_eq!(markdown(html, &options), "<x_x2e_y>a</x_x2e_y>b\n");
}

#[test]
fn should_match_a_dotted_tag_name_in_preserve_tags_and_show_it_as_written() {
    let html = r#"<p>a <x.y id="i">b</x.y> c</p>"#;
    let options = ConversionOptions {
        preserve_tags: vec!["x.y".to_owned()],
        ..ConversionOptions::default()
    };
    assert_eq!(markdown(html, &options), "a <x.y id=\"i\">b</x.y> c\n");
    let outer = ConversionOptions {
        preserve_tags: vec!["p".to_owned()],
        ..ConversionOptions::default()
    };
    assert_eq!(markdown(html, &outer), "<p>a <x.y id=\"i\">b</x.y> c</p>\n");
}

#[cfg(feature = "inline-images")]
#[test]
fn should_keep_a_dotted_tag_name_in_an_inline_vector_graphic() {
    use base64::Engine as _;

    let html = r#"<svg xmlns="http://www.w3.org/2000/svg"><x.y/><text>t</text></svg>"#;
    let options = ConversionOptions {
        extract_images: true,
        ..ConversionOptions::default()
    };
    let page = markdown(html, &options);
    let (_, encoded) = page.split_once("base64,").expect("a data URI");
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded.trim_end().trim_end_matches(')'))
        .expect("base64");
    let graphic = String::from_utf8(bytes).expect("UTF-8");
    assert!(graphic.contains("<x.y"), "{graphic}");
}

#[cfg(feature = "visitor")]
mod visitors {
    use super::{ConversionOptions, TierStrategy, markdown};
    use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult};
    use std::sync::{Arc, Mutex};

    #[derive(Debug, Default)]
    struct Names {
        started: Vec<String>,
        custom: Vec<(String, String)>,
    }

    impl HtmlVisitor for Names {
        fn visit_element_start(&mut self, ctx: &NodeContext) -> VisitResult {
            self.started.push(ctx.tag_name.to_string());
            VisitResult::Continue
        }

        fn visit_custom_element(&mut self, _ctx: &NodeContext<'_>, tag_name: &str, html: &str) -> VisitResult {
            self.custom.push((tag_name.to_owned(), html.to_owned()));
            VisitResult::Continue
        }
    }

    #[test]
    fn should_show_a_visitor_the_tag_name_as_written() {
        for tier_strategy in [TierStrategy::Auto, TierStrategy::Tier2] {
            let names = Arc::new(Mutex::new(Names::default()));
            let options = ConversionOptions {
                tier_strategy,
                visitor: Some(names.clone()),
                ..ConversionOptions::default()
            };
            assert_eq!(markdown(r#"<p>a <x.y id="i">b</x.y> c</p>"#, &options), "a b c\n");
            let names = names.lock().expect("visitor lock");
            assert_eq!(names.started, ["p", "x.y"]);
            let written = ("x.y".to_owned(), r#"<x.y id="i">b</x.y>"#.to_owned());
            assert_eq!(names.custom, [written]);
        }
    }
}
