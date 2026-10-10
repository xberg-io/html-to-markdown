use super::preprocess_repaired_html;
use crate::converter::DomContext;
use crate::converter::main_helpers::{repair_with_html5ever, repair_with_html5ever_within_depth};
use crate::converter::preprocessing_helpers::{has_inline_block_misnest, has_omitted_end_tag};
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
    if *attempted_misnest_repair {
        return Ok(ParseOutcome::Ready { dom, dom_ctx });
    }
    let omitted_end_tag = has_omitted_end_tag(&dom_ctx, parser);
    if !omitted_end_tag && !has_inline_block_misnest(&dom_ctx, parser) {
        return Ok(ParseOutcome::Ready { dom, dom_ctx });
    }
    *attempted_misnest_repair = true;
    // ~keep The depth limit is for the second parse that only an omitted end tag asks for. A
    // ~keep misnest got the second parse before issue #772 and keeps it with no limit: its first
    // ~keep parse can hide content, so it is not a tree to fall back on.
    let within_depth = omitted_end_tag
        .then(|| repair_with_html5ever_within_depth(input))
        .flatten();
    let repaired_as_omitted_end_tag = within_depth.is_some();
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
        return Ok(ParseOutcome::Ready { dom, dom_ctx });
    };
    if repaired_as_omitted_end_tag {
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

        assert_eq!(content.as_deref(), Some("|   |\n| --- |\n\n|  |\n| --- |\n"));
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
