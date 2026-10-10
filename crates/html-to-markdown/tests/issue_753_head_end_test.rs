//! Coverage for where the document head ends under `ConversionOptions::hidden_content`
//! (issue #753).
//!
//! ~keep A `<template>` or a `<noscript>` of the head holds metadata, so its content is never
//! ~keep written. The same element after the head holds text for a reader, and
//! ~keep `HiddenContent::All` writes it. The end tag of the head is optional: a browser ends the
//! ~keep head at the first text that is not white space and at the first element that the head
//! ~keep cannot hold.

#![allow(missing_docs)]

use html_to_markdown_rs::{ConversionOptions, HiddenContent, TierStrategy, convert};

/// The front matter of a page whose title is `t`.
const TITLE: &str = "---\ntitle: t\n---\n";

/// The tiers that write the body of a page whose head has no end tag.
const DOM_TIERS: [TierStrategy; 2] = [TierStrategy::Auto, TierStrategy::Tier2];

fn tiers() -> Vec<TierStrategy> {
    vec![
        TierStrategy::Auto,
        TierStrategy::Tier2,
        #[cfg(feature = "testkit")]
        TierStrategy::Tier1,
    ]
}

fn convert_in(html: &str, choice: HiddenContent, tier_strategy: TierStrategy) -> String {
    let options = ConversionOptions {
        hidden_content: choice,
        tier_strategy,
        ..ConversionOptions::default()
    };
    convert(html, Some(options))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

/// A page whose `<template>` or `<noscript>` is after a head that ends by implication.
struct Kept {
    /// The page.
    html: &'static str,
    /// The page with the two tags of the element removed.
    unwrapped: &'static str,
    /// The output of [`HiddenContent::Drop`], which is the output of the base.
    drop: String,
    /// The output of [`HiddenContent::All`].
    all: String,
}

/// One line for each output of `row` that is wrong.
///
/// ~keep In every tier, `HiddenContent::Reachable` writes what `HiddenContent::Drop` writes: no
/// ~keep row has content that a reader can reach. `HiddenContent::All` converts the content of
/// ~keep the element where it is written, so it writes what the page with no such tags gives.
/// ~keep The exact outputs hold for the tiers that write the body of such a page.
fn wrong_outputs(row: &Kept) -> Vec<String> {
    let mut wrong = Vec::new();
    let mut check = |name: &str, tier: TierStrategy, output: &str, expected: &str| {
        if output != expected {
            wrong.push(format!(
                "{name} {tier:?} {}\n  wrote    {output:?}\n  expected {expected:?}",
                row.html
            ));
        }
    };
    for tier in tiers() {
        let [drop, reachable, all] =
            [HiddenContent::Drop, HiddenContent::Reachable, HiddenContent::All].map(|c| convert_in(row.html, c, tier));
        check("reachable against drop", tier, &reachable, &drop);
        check(
            "all against the page with no such tags",
            tier,
            &all,
            &convert_in(row.unwrapped, HiddenContent::Drop, tier),
        );
        if DOM_TIERS.contains(&tier) {
            check("drop", tier, &drop, &row.drop);
            check("all", tier, &all, &row.all);
        }
    }
    wrong
}

/// One line for each output of `html` that is not `expected`, for each choice in every tier.
fn wrong_in_the_head(html: &str, expected: &str) -> Vec<String> {
    let mut wrong = Vec::new();
    for tier in tiers() {
        for choice in [HiddenContent::Drop, HiddenContent::Reachable, HiddenContent::All] {
            let output = convert_in(html, choice, tier);
            if output != expected {
                wrong.push(format!(
                    "{choice:?} {tier:?} {html}\n  wrote    {output:?}\n  expected {expected:?}"
                ));
            }
        }
    }
    wrong
}

fn assert_none(wrong: &[String]) {
    assert!(wrong.is_empty(), "{} wrong outputs:\n{}", wrong.len(), wrong.join("\n"));
}

// ~keep For a head with no end tag the base writes the title text into the body, and loses text
// ~keep that is a child of the head. The exact outputs below hold both as the base has them: this
// ~keep file fixes where the content of a kept element goes, not those two.

#[test]
fn should_keep_a_template_after_text_that_ends_the_head() {
    assert_none(&wrong_outputs(&Kept {
        html: "<head><title>t</title>visible<template><p>BODY</p></template>",
        unwrapped: "<head><title>t</title>visible<p>BODY</p>",
        drop: TITLE.to_string(),
        all: format!("{TITLE}tvisible\n\nBODY\n"),
    }));
}

#[test]
fn should_end_the_head_at_text_that_is_not_white_space() {
    let rows = [
        Kept {
            html: "<head><title>t</title>visible<noscript><p>BODY</p></noscript>",
            unwrapped: "<head><title>t</title>visible<p>BODY</p>",
            drop: TITLE.to_string(),
            all: format!("{TITLE}tvisible\n\nBODY\n"),
        },
        Kept {
            html: "<head><title>t</title> <!-- c -->\n visible<template><p>BODY</p></template>",
            unwrapped: "<head><title>t</title> <!-- c -->\n visible<p>BODY</p>",
            drop: TITLE.to_string(),
            all: format!("{TITLE}t visible\n\nBODY\n"),
        },
        Kept {
            html: r#"<head><meta charset="utf-8">visible<template><p>BODY</p></template>"#,
            unwrapped: r#"<head><meta charset="utf-8">visible<p>BODY</p>"#,
            drop: String::new(),
            all: "visible\n\nBODY\n".to_string(),
        },
        // ~keep Text before the `<head>` tag starts the body: a browser ignores the tag.
        Kept {
            html: "visible<head><template><p>BODY</p></template></head>",
            unwrapped: "visible<head><p>BODY</p></head>",
            drop: "visible\n".to_string(),
            all: "visible\n\nBODY\n".to_string(),
        },
        // ~keep A `<` that starts no tag is text.
        Kept {
            html: "<head><title>t</title>1 < 2<template><p>BODY</p></template>",
            unwrapped: "<head><title>t</title>1 < 2<p>BODY</p>",
            drop: TITLE.to_string(),
            all: format!("{TITLE}t1 < 2\n\nBODY\n"),
        },
    ];
    assert_none(&rows.iter().flat_map(wrong_outputs).collect::<Vec<_>>());
}

#[test]
fn should_end_the_head_at_an_element_of_the_body_and_at_the_body_tag() {
    let rows = [
        Kept {
            html: "<head><title>t</title><p>visible</p><template><p>BODY</p></template>",
            unwrapped: "<head><title>t</title><p>visible</p><p>BODY</p>",
            drop: format!("{TITLE}t\n\nvisible\n"),
            all: format!("{TITLE}t\n\nvisible\n\nBODY\n"),
        },
        Kept {
            html: "<head><title>t</title><div>visible</div><template><p>BODY</p></template>",
            unwrapped: "<head><title>t</title><div>visible</div><p>BODY</p>",
            drop: format!("{TITLE}t\n\nvisible\n"),
            all: format!("{TITLE}t\n\nvisible\n\nBODY\n"),
        },
        Kept {
            html: "<head><title>t</title><body><p>visible</p><template><p>BODY</p></template>",
            unwrapped: "<head><title>t</title><body><p>visible</p><p>BODY</p>",
            drop: format!("{TITLE}t\n\nvisible\n"),
            all: format!("{TITLE}t\n\nvisible\n\nBODY\n"),
        },
        Kept {
            html: "<head><title>t</title></head><noscript><p>BODY</p></noscript><p>visible</p>",
            unwrapped: "<head><title>t</title></head><p>BODY</p><p>visible</p>",
            drop: format!("{TITLE}\nvisible\n"),
            all: format!("{TITLE}\nBODY\n\nvisible\n"),
        },
    ];
    assert_none(&rows.iter().flat_map(wrong_outputs).collect::<Vec<_>>());
}

#[test]
fn should_not_end_the_head_at_white_space_a_comment_or_metadata() {
    let mut wrong = Vec::new();
    for (metadata, front_matter) in [
        (" \n\t\r", ""),
        ("<!-- a comment -->", ""),
        (r#"<meta name="a" content="b c">"#, "meta-a: b c\n"),
        (r#"<link rel="icon" href="i.png">"#, ""),
        (r#"<base href="/">"#, "base: /\n"),
        ("<script>var a = 1;</script>", ""),
        ("<style>p { color: red }</style>", ""),
        (r#"<noscript><img src="p.png"></noscript>"#, ""),
        (r#"<template><meta name="a" content="b"></template>"#, ""),
    ] {
        let html =
            format!("<head><title>t</title>{metadata}<template><p>HEAD</p></template></head><body><p>visible</p>");
        let expected = format!("---\n{front_matter}title: t\n---\n\nvisible\n");
        wrong.extend(wrong_in_the_head(&html, &expected));
    }
    let page = "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<title>t</title>\n<template><p>HEAD</p></template>\n</head>\n<body><p>visible</p></body></html>";
    wrong.extend(wrong_in_the_head(page, &format!("{TITLE}\nvisible\n")));
    assert_none(&wrong);
}

#[test]
fn should_not_end_the_head_at_a_tag_that_a_browser_ignores_there() {
    // ~keep A browser ignores a second `<head>` tag and a stray end tag in the head.
    let mut wrong = Vec::new();
    for html in [
        "<head><head><template><p>HEAD</p></template></head><body><p>visible</p>",
        "<head></p><template><p>HEAD</p></template></head><body><p>visible</p>",
    ] {
        wrong.extend(wrong_in_the_head(html, "visible\n"));
    }
    assert_none(&wrong);
}

#[test]
fn should_read_a_tag_in_a_noscript_of_the_head_as_its_content() {
    // ~keep A `<noscript>` of the head ends at its own end tag, not at the first end tag in it,
    // ~keep so the `<template>` after it is still metadata.
    let html = r#"<head><noscript></i><img src="p.png"></noscript><template><p>HEAD</p></template></head><body><p>visible</p>"#;
    assert_none(&wrong_in_the_head(html, "visible\n"));
}
