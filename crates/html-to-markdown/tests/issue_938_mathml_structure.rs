#![allow(missing_docs)]

use html_to_markdown_rs::{ConversionOptions, HiddenContent, TierStrategy, convert};

fn assert_routes(html: &str, expected: &str) {
    for tier_strategy in [
        TierStrategy::Auto,
        TierStrategy::Tier2,
        #[cfg(feature = "testkit")]
        TierStrategy::Tier1,
    ] {
        let result = convert(
            html,
            Some(ConversionOptions {
                tier_strategy,
                ..ConversionOptions::default()
            }),
        )
        .expect("MathML conversion should succeed");
        assert_eq!(result.content.as_deref(), Some(expected), "{tier_strategy:?}: {html}");
    }
}

#[test]
fn should_skip_math_annotations_and_keep_only_the_presentation() {
    for annotation in [
        "<annotation encoding=\"application/x-tex\">x</annotation>",
        "<annotation-xml encoding=\"application/xhtml+xml\"><span>duplicate</span></annotation-xml>",
    ] {
        assert_routes(
            &format!("<p>so <math><semantics><mi>x</mi>{annotation}</semantics></math> holds</p>"),
            "so x holds\n",
        );
    }
}

#[test]
fn should_keep_fraction_and_power_structure() {
    for (formula, expected) in [
        ("<mi>π</mi><mo>=</mo><mfrac><mi>C</mi><mi>d</mi></mfrac>", "π=C/d"),
        ("<mi>π</mi><msup><mi>r</mi><mn>2</mn></msup>", "πr^2"),
        (
            "<mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mrow><mi>c</mi><mo>+</mo><mi>d</mi></mrow></mfrac>",
            "(a+b)/(c+d)",
        ),
        (
            "<msup><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msup>",
            "(a+b)^(n+1)",
        ),
    ] {
        assert_routes(
            &format!("<p>The value is <math>{formula}</math> here.</p>"),
            &format!("The value is {expected} here.\n"),
        );
    }
}

#[test]
fn should_preserve_nested_fraction_grouping() {
    assert_routes(
        "<math><mfrac><mi>a</mi><mfrac><mi>b</mi><mi>c</mi></mfrac></mfrac></math>",
        "a/(b/c)\n",
    );
}

#[test]
fn should_group_tokens_that_contain_operators() {
    for (formula, expected) in [
        ("<msup><mn>-1</mn><mn>2</mn></msup>", "(-1)^2\n"),
        ("<mfrac><mtext>a+b</mtext><mi>c</mi></mfrac>", "(a+b)/c\n"),
    ] {
        assert_routes(&format!("<math>{formula}</math>"), expected);
    }
}

#[test]
fn should_render_the_first_semantics_child_even_when_it_is_an_annotation() {
    assert_routes(
        "<math><semantics><annotation>first</annotation><mi>second</mi></semantics></math>",
        "first\n",
    );
}

#[test]
fn should_not_promote_an_alternative_when_the_presentation_is_hidden() {
    assert_routes("<math><semantics><mi hidden>x</mi><mi>y</mi></semantics></math>", "");
}

#[test]
fn should_preserve_text_in_malformed_fraction_operands() {
    assert_routes("<math><mfrac><mi>a</mi>STRAY<mi>b</mi></mfrac></math>", "aSTRAYb\n");
}

#[test]
fn should_keep_annotations_when_hidden_content_is_requested() {
    let result = convert(
        "<math><semantics><mi>x</mi><annotation>alternate</annotation></semantics></math>",
        Some(ConversionOptions {
            hidden_content: HiddenContent::All,
            ..ConversionOptions::default()
        }),
    )
    .expect("MathML conversion should succeed");
    assert_eq!(result.content.as_deref(), Some("xalternate\n"));
}

#[test]
fn should_keep_html_semantics_children_at_annotation_integration_points() {
    for encoding in ["text/html", "application/xhtml+xml", "TEXT/HTML"] {
        assert_routes(
            &format!(
                "<math><annotation-xml encoding=\"{encoding}\"><semantics><span hidden>x</span><span>y</span></semantics></annotation-xml></math>"
            ),
            "y\n",
        );
    }
}
