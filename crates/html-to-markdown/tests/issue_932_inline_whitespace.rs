#![allow(missing_docs)]

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn converted(html: &str, tier_strategy: TierStrategy) -> String {
    convert(
        html,
        Some(ConversionOptions {
            tier_strategy,
            ..ConversionOptions::default()
        }),
    )
    .expect("conversion should succeed")
    .content
    .unwrap_or_default()
}

#[test]
fn should_keep_bare_whitespace_link_labels_out_of_visible_text() {
    for body in [" ", "&nbsp;", "\t", "\n", "&ensp;", "&thinsp;"] {
        for tier in [
            TierStrategy::Auto,
            #[cfg(feature = "testkit")]
            TierStrategy::Tier1,
            TierStrategy::Tier2,
        ] {
            let html = format!("<p>alpha<a href=\"https://example.com\">{body}</a>omega</p>");
            assert_eq!(
                converted(&html, tier),
                "alpha [](https://example.com)omega\n",
                "body: {body:?}"
            );
        }
    }
}

#[test]
fn should_keep_whitespace_inside_blank_inline_elements() {
    for element in ["label", "code", "abbr", "sup", "sub", "mark", "s", "q", "kbd"] {
        for tier in [
            TierStrategy::Auto,
            #[cfg(feature = "testkit")]
            TierStrategy::Tier1,
            TierStrategy::Tier2,
        ] {
            let html = format!("<p>alpha<{element}>\n</{element}>omega</p>");
            assert_eq!(converted(&html, tier), "alpha omega\n", "element: {element}");
        }
    }
}

#[test]
fn should_keep_nonbreaking_spaces_at_nested_emphasis_edges() {
    for (html, expected) in [
        (
            "<p><strong><em>Cool tubes</em>&nbsp;</strong>are built so</p>",
            "***Cool tubes*** are built so\n",
        ),
        (
            "<p><strong>4 or more</strong><strong>&nbsp;<a href=\"/b\">buckets</a></strong> to buy</p>",
            "**4 or more** **[buckets](/b)** to buy\n",
        ),
    ] {
        for tier in [
            TierStrategy::Auto,
            #[cfg(feature = "testkit")]
            TierStrategy::Tier1,
            TierStrategy::Tier2,
        ] {
            #[cfg(feature = "testkit")]
            let expected = if matches!(tier, TierStrategy::Tier1) && html.contains("Cool tubes") {
                "***Cool tubes***\u{a0}are built so\n"
            } else {
                expected
            };
            assert_eq!(converted(html, tier), expected, "html: {html}");
        }
    }
}

#[test]
fn should_keep_space_after_inline_text_before_a_list_block() {
    for inline in ["<b>Handles</b>", "<i>Handles</i>", "<a href=\"/h\">Handles</a>"] {
        let expected_inline = match inline {
            "<b>Handles</b>" => "**Handles**",
            "<i>Handles</i>" => "*Handles*",
            _ => "[Handles](/h)",
        };
        for block in ["<ul><li>See below</li></ul>", "<p>See below</p>"] {
            let html = format!("<ul><li>Use the {inline} section\n\n{block}</li></ul>");
            let tail = if block.starts_with("<ul>") {
                "  * See below"
            } else {
                "  See below"
            };
            let expected = format!("- Use the {expected_inline} section\n\n{tail}\n");
            assert_eq!(converted(&html, TierStrategy::Auto), expected);
        }
    }
}

#[test]
fn should_separate_adjacent_links_containing_blocks() {
    let html = "<div><a href=\"/p\"><div>Previous</div><div>Getting Started</div></a><a href=\"/n\"><div>Next</div><div>Configuration</div></a></div>";
    for tier in [TierStrategy::Auto, TierStrategy::Tier2] {
        assert_eq!(
            converted(html, tier),
            "[Previous Getting Started](/p) [Next Configuration](/n)\n"
        );
    }
}

#[test]
fn should_keep_edge_breaks_in_code_and_labels() {
    for (html, expected) in [
        ("<p><code>alpha<br></code>omega</p>", "`alpha`  \nomega\n"),
        ("<p>alpha<code><br>omega</code></p>", "alpha  \n`omega`\n"),
        ("<p>alpha<code>\n<br>omega</code></p>", "alpha  \n`omega`\n"),
        ("<p><code>alpha<br>\n</code>omega</p>", "`alpha`  \nomega\n"),
        ("<p><label>alpha<br></label>omega</p>", "alpha  \nomega\n"),
        ("<p>alpha<label><br>omega</label></p>", "alpha  \nomega\n"),
    ] {
        for tier in [
            TierStrategy::Auto,
            #[cfg(feature = "testkit")]
            TierStrategy::Tier1,
            TierStrategy::Tier2,
        ] {
            assert_eq!(converted(html, tier), expected, "html: {html}, tier: {tier:?}");
        }
    }
}

#[test]
fn should_not_promote_source_newlines_to_code_edge_breaks() {
    for (html, expected) in [
        (
            "<p>before<code>\n<span>alpha</span><br>beta</code>after</p>",
            "before`alpha`  \n`beta`after\n",
        ),
        (
            "<p>before<code>alpha<br><span>beta</span>\n</code>after</p>",
            "before`alpha`  \n`beta`after\n",
        ),
    ] {
        for tier in [TierStrategy::Auto, TierStrategy::Tier2] {
            assert_eq!(converted(html, tier), expected, "html: {html}, tier: {tier:?}");
        }
    }
}

#[test]
fn should_keep_code_break_offsets_valid_in_headings_and_cells() {
    for (html, expected) in [
        ("<h1>before<code>a  <br></code>after</h1>", "# before`a `after\n"),
        ("<h1>before<code>λ  <br></code>after</h1>", "# before`λ `after\n"),
        (
            "<table><tr><td>before<code>λ  <br></code>after</td></tr></table>",
            "| before`λ `after |\n| --------------- |\n",
        ),
    ] {
        assert_eq!(converted(html, TierStrategy::Tier2), expected, "html: {html}");
    }
}
