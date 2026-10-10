use super::preprocess_repaired_html;
use crate::converter::DomContext;
use crate::converter::main_helpers::{repair_with_html5ever, repair_with_html5ever_within_depth};
use crate::converter::preprocessing_helpers::{has_inline_block_misnest, has_omitted_end_tag, readable_text_len};
use crate::converter::utility::caching::build_dom_context;
use crate::error::{ConversionError, Result};

#[cfg(test)]
thread_local! {
    static PARSE_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static FORCE_INVALID_LENGTH: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn parse_html(input: &str) -> Result<tl::VDom<'_>> {
    #[cfg(test)]
    PARSE_CALLS.with(|calls| calls.set(calls.get() + 1));
    #[cfg(test)]
    if FORCE_INVALID_LENGTH.with(std::cell::Cell::get) {
        return Err(invalid_length(input));
    }
    tl::parse(input, tl::ParserOptions::default()).map_err(|tl::ParseError::InvalidLength| invalid_length(input))
}

fn invalid_length(input: &str) -> ConversionError {
    tracing::error!(
        target: "html_to_markdown::convert",
        input_len = input.len(),
        "failed to parse HTML; input length exceeds parser capacity"
    );
    ConversionError::ParseError("Failed to parse HTML".to_string())
}

/// The tree that the converter reads, and the length of the page that the tree is made of.
pub(super) struct ParsedPage<'a> {
    pub(super) dom: tl::VDom<'a>,
    pub(super) dom_ctx: DomContext,
    pub(super) len: usize,
}

/// Parse `input`, and parse it a second time through the HTML tree builder when it has an
/// omitted end tag or a misnest. `repaired_page` holds the page of the second parse.
///
/// ~keep The repair must never lose text. The tree builder drops text that the first parse
/// ~keep keeps (all that follows `</frameset>` in a frameset document), and no list of such
/// ~keep cases is kept here: both pages are read by `tl` and measured by
/// ~keep [`readable_text_len`], and a repaired page with a smaller count is not used. The
/// ~keep page then converts as it did before issue #772.
pub(super) fn parse_for_conversion<'a>(
    input: &'a str,
    repaired_page: &'a std::cell::OnceCell<String>,
    preserve_menu: bool,
) -> Result<ParsedPage<'a>> {
    let dom = parse_html(input)?;
    let parser = dom.parser();
    let dom_ctx = build_dom_context(&dom, parser, input.len());
    let len = input.len();
    let omitted_end_tag = has_omitted_end_tag(&dom_ctx, parser);
    if !omitted_end_tag && !has_inline_block_misnest(&dom_ctx, parser) {
        return Ok(ParsedPage { dom, dom_ctx, len });
    }
    // ~keep The depth limit and the text rule are for the second parse that only an omitted end
    // ~keep tag asks for. A misnest got the second parse before issue #772 and keeps it with no
    // ~keep limit and no rule: its first parse can hide content, so it is not a tree to fall
    // ~keep back on.
    let within_depth = omitted_end_tag
        .then(|| repair_with_html5ever_within_depth(input))
        .flatten();
    let for_omitted_end_tag = within_depth.is_some();
    let repaired = within_depth.or_else(|| {
        (!omitted_end_tag || has_inline_block_misnest(&dom_ctx, parser))
            .then(|| repair_with_html5ever(input))
            .flatten()
    });
    let Some(repaired) = repaired else {
        tracing::warn!(
            target: "html_to_markdown::convert",
            "html5ever repair gave no tree (the document nests too deep or cannot be written back); proceeding with original structure"
        );
        return Ok(ParsedPage { dom, dom_ctx, len });
    };
    let page = repaired_page.get_or_init(|| preprocess_repaired_html(&repaired, preserve_menu));
    let second = parse_html(page)?;
    let loses_text = for_omitted_end_tag && readable_text_len(&second) < readable_text_len(&dom);
    if loses_text && !has_inline_block_misnest(&dom_ctx, parser) {
        // ~keep A frameset page with every end tag written comes here, so this is no warning.
        tracing::debug!(
            target: "html_to_markdown::convert",
            "html5ever repair would lose text; proceeding with original structure"
        );
        return Ok(ParsedPage { dom, dom_ctx, len });
    }
    if for_omitted_end_tag && !loses_text {
        // ~keep An omitted end tag is HTML that the standard permits, so this route is logged
        // ~keep at debug, like the choice of a tier. A misnest is an authoring error and warns.
        tracing::debug!(
            target: "html_to_markdown::convert",
            "element with no end tag; re-parsed with html5ever repair"
        );
    } else {
        tracing::warn!(
            target: "html_to_markdown::convert",
            "misnested HTML elements detected; re-parsed with html5ever repair"
        );
    }
    let dom_ctx = build_dom_context(&second, second.parser(), page.len());
    Ok(ParsedPage {
        dom: second,
        dom_ctx,
        len: page.len(),
    })
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
        for html in ["<html><body><p>one</p><p>two</p>", "<HTML><BODY><p>one</p><p>two</p>"] {
            let (calls, content) = parse_calls_for(html);

            assert_eq!(content.as_deref(), Some("one\n\ntwo\n"), "{html:?}");
            assert_eq!(calls, 1, "{html:?}");
        }
    }

    #[test]
    fn should_keep_the_first_parse_when_the_second_parse_nests_past_its_limit() {
        let html = format!("<p>kept</p>{}deep", "<div>".repeat(600));
        let (calls, content) = parse_calls_for(&html);

        assert_eq!(content.as_deref(), Some("kept\n"));
        assert_eq!(calls, 1);
    }

    #[test]
    fn should_parse_twice_when_a_misnest_nests_past_the_limit_for_an_omitted_end_tag() {
        // ~keep A cell with no row is a misnest and has no end tag. The first parse hides it.
        let html = format!("{}x", "<table><td>".repeat(300));
        let (calls, content) = parse_calls_for(&html);

        assert_eq!(content.as_deref(), Some("|  |\n| --- |\n"));
        assert_eq!(calls, 2);
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
    fn should_convert_the_first_parse_when_the_repaired_page_holds_less_text() {
        // ~keep `tl` leaves a `<frame>` open, so the first page looks like one with no end tag.
        // ~keep The second call is the parse of the repaired page, which is measured and left.
        for (html, expected) in [
            (
                "<frameset><frame></frameset><div><p>one</p></div><p>tail</p>",
                "one\n\ntail\n",
            ),
            ("<FRAMESET></FRAMESET><div><p>one</div>tail", "onetail\n"),
            ("<noframes><div></noframes><div><p>one</p></div>tail", "one\n\ntail\n"),
        ] {
            let (calls, content) = parse_calls_for(html);

            assert_eq!(content.as_deref(), Some(expected), "{html:?}");
            assert_eq!(calls, 2, "{html:?}");
        }
    }

    #[test]
    fn should_log_at_debug_that_a_repair_that_loses_text_is_not_used() {
        assert_eq!(
            repair_events("<frameset><frame></frameset><div><p>one</p></div><p>tail</p>"),
            [(
                tracing::Level::DEBUG,
                "html5ever repair would lose text; proceeding with original structure".to_string()
            )]
        );
    }

    type Events = std::sync::Arc<std::sync::Mutex<Vec<(tracing::Level, String)>>>;

    struct EventLog(Events);

    struct Message<'a>(&'a mut String);

    impl tracing::field::Visit for Message<'_> {
        fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
            if field.name() == "message" {
                *self.0 = format!("{value:?}");
            }
        }
    }

    impl tracing::Subscriber for EventLog {
        fn enabled(&self, _metadata: &tracing::Metadata<'_>) -> bool {
            true
        }
        fn new_span(&self, _attributes: &tracing::span::Attributes<'_>) -> tracing::span::Id {
            tracing::span::Id::from_u64(1)
        }
        fn record(&self, _span: &tracing::span::Id, _values: &tracing::span::Record<'_>) {}
        fn record_follows_from(&self, _span: &tracing::span::Id, _follows: &tracing::span::Id) {}
        fn event(&self, event: &tracing::Event<'_>) {
            let mut message = String::new();
            event.record(&mut Message(&mut message));
            self.0
                .lock()
                .expect("event log")
                .push((*event.metadata().level(), message));
        }
        fn enter(&self, _span: &tracing::span::Id) {}
        fn exit(&self, _span: &tracing::span::Id) {}
    }

    fn repair_events(html: &str) -> Vec<(tracing::Level, String)> {
        let events = Events::default();
        tracing::subscriber::with_default(EventLog(Events::clone(&events)), || {
            // ~keep A callsite keeps the interest of the thread that reaches it first, and a
            // ~keep test on another thread reaches these with no subscriber. So the first
            // ~keep conversion only reaches them, and the interest is then asked again.
            parse_calls_for(html);
            tracing::callsite::rebuild_interest_cache();
            events.lock().expect("event log").clear();
            parse_calls_for(html);
        });
        let events = events.lock().expect("event log");
        events
            .iter()
            .filter(|(_, message)| message.contains("html5ever repair"))
            .cloned()
            .collect()
    }

    #[test]
    fn should_log_the_second_parse_at_debug_for_an_omitted_end_tag_and_warn_for_a_misnest() {
        assert_eq!(
            repair_events("<div><p>one</div>tail"),
            [(
                tracing::Level::DEBUG,
                "element with no end tag; re-parsed with html5ever repair".to_string()
            )]
        );
        assert_eq!(
            repair_events("<b><p>one</p></b>"),
            [(
                tracing::Level::WARN,
                "misnested HTML elements detected; re-parsed with html5ever repair".to_string()
            )]
        );
    }

    #[test]
    fn should_warn_of_a_misnest_when_a_page_with_an_omitted_end_tag_takes_the_repair_with_no_limit() {
        // ~keep No cell has an end tag, and the tree builder nests past the limit for an
        // ~keep omitted end tag. The repair that the page then takes is the one for a misnest.
        let html = format!("{}x", "<table><td>".repeat(300));

        assert_eq!(
            repair_events(&html),
            [(
                tracing::Level::WARN,
                "misnested HTML elements detected; re-parsed with html5ever repair".to_string()
            )]
        );
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
