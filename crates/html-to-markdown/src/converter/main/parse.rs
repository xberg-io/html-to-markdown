use super::preprocess_repaired_html;
use crate::converter::DomContext;
use crate::converter::main_helpers::repair_with_html5ever;
use crate::converter::preprocessing_helpers::has_inline_block_misnest;
use crate::converter::utility::caching::build_dom_context;
use crate::error::{ConversionError, Result};

#[cfg(test)]
thread_local! {
    static PARSE_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static FORCE_INVALID_LENGTH: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn parse_html(input: &str) -> std::result::Result<tl::VDom<'_>, tl::ParseError> {
    #[cfg(test)]
    PARSE_CALLS.with(|calls| calls.set(calls.get() + 1));
    #[cfg(test)]
    if FORCE_INVALID_LENGTH.with(std::cell::Cell::get) {
        return Err(tl::ParseError::InvalidLength);
    }
    tl::parse(input, tl::ParserOptions::default())
}

#[expect(
    clippy::large_enum_variant,
    reason = "boxing the hot Ready variant would allocate on every conversion; Retry is cold"
)]
pub(super) enum ParseOutcome<'a> {
    Ready { dom: tl::VDom<'a>, dom_ctx: DomContext },
    Retry(String),
}

pub(super) fn parse_for_conversion<'a>(
    input: &'a str,
    preserve_menu: bool,
    attempted_misnest_repair: &mut bool,
) -> Result<ParseOutcome<'a>> {
    let dom = match parse_html(input) {
        Ok(dom) => dom,
        Err(tl::ParseError::InvalidLength) => {
            tracing::error!(
                target: "html_to_markdown::convert",
                input_len = input.len(),
                "failed to parse HTML; input length exceeds parser capacity"
            );
            return Err(ConversionError::ParseError("Failed to parse HTML".to_string()));
        }
    };
    let parser = dom.parser();
    let dom_ctx = build_dom_context(&dom, parser, input.len());
    let has_misnest = !*attempted_misnest_repair && has_inline_block_misnest(&dom_ctx, parser);
    if !has_misnest {
        return Ok(ParseOutcome::Ready { dom, dom_ctx });
    }
    *attempted_misnest_repair = true;
    let Some(repaired) = repair_with_html5ever(input) else {
        tracing::warn!(
            target: "html_to_markdown::convert",
            "block-level element misnested under an inline ancestor; html5ever repair failed, proceeding with original structure"
        );
        return Ok(ParseOutcome::Ready { dom, dom_ctx });
    };
    tracing::warn!(
        target: "html_to_markdown::convert",
        "misnested HTML elements detected; re-parsed with html5ever repair"
    );
    Ok(ParseOutcome::Retry(preprocess_repaired_html(&repaired, preserve_menu)))
}

#[cfg(test)]
mod tests {
    use super::{FORCE_INVALID_LENGTH, PARSE_CALLS};
    use crate::converter::utility::caching::DOM_CONTEXT_BUILDS;
    use crate::{ConversionError, ConversionOptions, TierStrategy, convert};

    #[test]
    fn should_parse_and_build_dom_context_once_for_well_formed_tier2_input() {
        let options = ConversionOptions {
            tier_strategy: TierStrategy::Tier2,
            ..ConversionOptions::default()
        };
        PARSE_CALLS.with(|calls| calls.set(0));
        DOM_CONTEXT_BUILDS.with(|builds| builds.set(0));

        let result = convert("<p>Hello <strong>world</strong></p>", options).expect("convert HTML");

        assert_eq!(result.content.as_deref(), Some("Hello **world**\n"));
        PARSE_CALLS.with(|calls| assert_eq!(calls.get(), 1));
        DOM_CONTEXT_BUILDS.with(|builds| assert_eq!(builds.get(), 1));
    }

    fn parse_calls_for(html: &str) -> (usize, Option<String>) {
        let options = ConversionOptions {
            tier_strategy: TierStrategy::Tier2,
            ..ConversionOptions::default()
        };
        PARSE_CALLS.with(|calls| calls.set(0));
        let result = convert(html, options).expect("convert HTML");
        (PARSE_CALLS.with(std::cell::Cell::get), result.content)
    }

    #[test]
    fn should_parse_twice_when_an_element_with_content_has_no_end_tag() {
        let (calls, content) = parse_calls_for("<div><p>one</div>tail");

        assert_eq!(content.as_deref(), Some("one\n\ntail\n"));
        assert_eq!(calls, 2);
    }

    #[test]
    fn should_parse_once_when_only_html_and_body_have_no_end_tag() {
        let (calls, content) = parse_calls_for("<html><body><p>one</p><p>two</p>");

        assert_eq!(content.as_deref(), Some("one\n\ntwo\n"));
        assert_eq!(calls, 1);
    }

    #[test]
    fn should_parse_once_when_the_open_element_has_no_content() {
        for html in ["<p>one</p><br>", "<p>one</p><span>", "<html><body><p>one</p><hr>"] {
            let (calls, content) = parse_calls_for(html);

            assert_eq!(calls, 1, "{html:?}");
            assert!(content.is_some_and(|content| content.starts_with("one\n")), "{html:?}");
        }
    }

    #[test]
    fn should_parse_once_when_an_attribute_value_holds_an_end_tag() {
        let (calls, content) = parse_calls_for("<p>one</p><div title='</div>'></div>");

        assert_eq!(content.as_deref(), Some("one\n"));
        assert_eq!(calls, 1);
    }

    #[test]
    fn should_fail_once_when_input_exceeds_parser_capacity() {
        let options = ConversionOptions {
            tier_strategy: TierStrategy::Tier2,
            ..ConversionOptions::default()
        };
        PARSE_CALLS.with(|calls| calls.set(0));
        FORCE_INVALID_LENGTH.with(|forced| forced.set(true));

        let result = convert("<p>oversized</p>", options);

        FORCE_INVALID_LENGTH.with(|forced| forced.set(false));
        assert!(matches!(
            result,
            Err(ConversionError::ParseError(message)) if message == "Failed to parse HTML"
        ));
        PARSE_CALLS.with(|calls| assert_eq!(calls.get(), 1));
    }
}
