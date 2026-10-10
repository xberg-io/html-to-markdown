//! Where the document head is, for a scan that reads the tags of a page before the parse.

use super::markup::matches_tag_start;
use crate::converter::main_helpers::{end_tag_starts_body, is_ignorable_before_head, starts_body};

/// Where a scan over the tags of a page is, relative to the document head.
///
/// ~keep The rule for where the head ends is the one that the search for the document head uses:
/// ~keep [`starts_body`] for a start tag, [`end_tag_starts_body`] for an end tag and
/// ~keep [`is_ignorable_before_head`] for text. The `<head>` tag, its end tag and the body tag
/// ~keep are optional: `<head><title>t</title><p>` has its paragraph in the body, and
/// ~keep `<html><meta charset="utf-8"><noscript>` has its `<noscript>` in the head.
/// ~keep
/// ~keep What a `<template>` or a `<noscript>` of the head holds is not content of the head: a
/// ~keep browser with scripting reads the first as a separate fragment and the second as text.
/// ~keep A tracking pixel, `<noscript><img></noscript>`, does not end the head.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct HeadScan {
    place: Place,
    inside: Inside,
}

/// The part of the page that the scan is in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Place {
    /// Before a `<head>` tag and before the body. `in_document` says that the page is a
    /// document, whose head a browser opens by implication: the scan has read a doctype, an
    /// `<html>` tag or an element that only the head holds. Without one, the input is a
    /// fragment, which has no head.
    Before { in_document: bool },
    /// After the `<head>` tag.
    In,
    /// After the end tag of the head, where a browser still puts metadata in the head.
    AfterEndTag,
    /// In the body.
    Body,
}

/// The element of the head whose content the scan is in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Inside {
    /// No element.
    Nothing,
    /// The `<title>`, where a tag is text.
    Title,
    /// A `<noscript>`, up to its end tag.
    Noscript,
    /// A `<template>`, with the count of the `<template>` elements that are open.
    Template(usize),
}

impl HeadScan {
    /// The state at the start of the input.
    pub(super) const fn new() -> Self {
        Self {
            place: Place::Before { in_document: false },
            inside: Inside::Nothing,
        }
    }

    /// Whether the body has started.
    pub(super) const fn in_body(self) -> bool {
        matches!(self.place, Place::Body)
    }

    /// The state after the tag whose name starts at `name_start`.
    ///
    /// `closes_itself` says that the start tag is written as self-closing, which the parser
    /// reads as an element with no content.
    pub(super) fn after_tag(self, bytes: &[u8], name_start: usize, is_end_tag: bool, closes_itself: bool) -> Self {
        if self.in_body() {
            return self;
        }
        if !is_end_tag && matches_tag_start(bytes, name_start, b"!doctype") {
            return self.in_document();
        }
        let name = lower_case_name(bytes, name_start);
        if name.is_empty() {
            return self;
        }
        match self.inside {
            Inside::Nothing if is_end_tag => self.after_end_tag(&name),
            Inside::Nothing => self.after_start_tag(&name, closes_itself),
            inside => Self {
                inside: inside.after_tag(&name, is_end_tag, closes_itself),
                ..self
            },
        }
    }

    /// The state after `text`, which is written between two tags. `next_tag_is_html` says that
    /// the `<html>` tag follows the text.
    ///
    /// ~keep White space stays in the head and other text starts the body. The text of a
    /// ~keep `<title>`, a `<noscript>` and a `<template>` of the head is their content.
    pub(super) fn after_text(self, text: &str, next_tag_is_html: bool) -> Self {
        if self.in_body() || self.inside != Inside::Nothing {
            return self;
        }
        let before_the_head_tag = matches!(self.place, Place::Before { .. });
        if is_ignorable_before_head(text, before_the_head_tag && next_tag_is_html) {
            self
        } else {
            self.body()
        }
    }

    fn after_start_tag(self, name: &[u8], closes_itself: bool) -> Self {
        let holds_fragment = matches!(name, b"template" | b"noscript");
        match (self.place, name) {
            (Place::Before { .. }, b"head") => self.at(Place::In),
            (_, b"head") => self,
            (_, b"html") => self.in_document(),
            (_, name) if starts_body(name) => self.body(),
            // ~keep A fragment has no head: its first `<template>` is content. After the end
            // ~keep tag of the head, a browser puts a `<noscript>` in the body.
            (Place::Before { in_document: false }, _) | (Place::AfterEndTag, b"noscript") if holds_fragment => {
                self.body()
            }
            _ => Self {
                inside: Inside::opened_by(name, closes_itself),
                ..self.in_document()
            },
        }
    }

    fn after_end_tag(self, name: &[u8]) -> Self {
        match name {
            b"head" => self.at(Place::AfterEndTag),
            name if end_tag_starts_body(name) => self.body(),
            _ => self,
        }
    }

    /// The same state in a page that is a document.
    const fn in_document(self) -> Self {
        match self.place {
            Place::Before { .. } => self.at(Place::Before { in_document: true }),
            _ => self,
        }
    }

    const fn body(self) -> Self {
        self.at(Place::Body)
    }

    const fn at(self, place: Place) -> Self {
        Self { place, ..self }
    }
}

impl Inside {
    /// The element that the start tag named `name` opens in the head.
    fn opened_by(name: &[u8], closes_itself: bool) -> Self {
        match name {
            _ if closes_itself => Self::Nothing,
            b"title" => Self::Title,
            b"noscript" => Self::Noscript,
            b"template" => Self::Template(1),
            _ => Self::Nothing,
        }
    }

    /// The element that the scan is in after a tag named `name` in the content of `self`.
    fn after_tag(self, name: &[u8], is_end_tag: bool, closes_itself: bool) -> Self {
        match (self, name) {
            (Self::Title, b"title") | (Self::Noscript, b"noscript") | (Self::Template(1), b"template")
                if is_end_tag =>
            {
                Self::Nothing
            }
            (Self::Template(open), b"template") if is_end_tag => Self::Template(open - 1),
            (Self::Template(open), b"template") if !closes_itself => Self::Template(open + 1),
            (inside, _) => inside,
        }
    }
}

/// The name of the tag whose name starts at `name_start`, in lower case. It is empty where no
/// name starts: in a comment, and at a `<` that starts no tag.
fn lower_case_name(bytes: &[u8], name_start: usize) -> Vec<u8> {
    let rest = bytes.get(name_start..).unwrap_or_default();
    if !rest.first().is_some_and(u8::is_ascii_alphabetic) {
        return Vec::new();
    }
    let end = rest
        .iter()
        .position(|byte| byte.is_ascii_whitespace() || matches!(byte, b'/' | b'>'))
        .unwrap_or(rest.len());
    rest[..end].to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::HeadScan;

    /// The state after the tags and the text of `markup`, which has no comment.
    fn scan(markup: &str) -> HeadScan {
        let bytes = markup.as_bytes();
        let mut state = HeadScan::new();
        let mut text_start = 0;
        while let Some(start) = markup[text_start..].find('<').map(|offset| text_start + offset) {
            let end = markup[start..]
                .find('>')
                .map_or(markup.len(), |offset| start + offset + 1);
            let is_end_tag = markup[start..].starts_with("</");
            let next_tag_is_html = markup[start..].to_ascii_lowercase().starts_with("<html");
            state = state.after_text(&markup[text_start..start], next_tag_is_html);
            state = state.after_tag(
                bytes,
                start + 1 + usize::from(is_end_tag),
                is_end_tag,
                markup[start..end].ends_with("/>"),
            );
            text_start = end;
        }
        state.after_text(&markup[text_start..], false)
    }

    fn assert_in_body(expected: bool, pages: &[&str]) {
        for page in pages {
            assert_eq!(scan(page).in_body(), expected, "{page:?}");
        }
    }

    #[test]
    fn should_end_the_head_at_a_tag_that_starts_the_body() {
        assert_in_body(
            true,
            &[
                "<head><p>",
                "<head></head><body>",
                "<HEAD><P>",
                "<head><metadata>",
                "<head><titles>",
                "<head><templates>",
                "<head></body>",
                "<head></html>",
                "<head></br>",
                "<head></BODY>",
                "<html><meta></br>",
                "</head><p>",
                "<head></head><noscript>",
            ],
        );
        assert_in_body(
            false,
            &[
                "<head><head><html><meta><link><base>",
                "<head></p></div></bodys></brx>",
                "<head></head>",
                "<head></head><meta><template>",
                "<head></head><template><p></template>",
                "<html><title>",
                "<head><",
                "<head><3>",
            ],
        );
    }

    #[test]
    fn should_read_the_content_of_an_element_of_the_head_as_its_own() {
        assert_in_body(
            false,
            &[
                "<head><title><p>a</body>",
                "<head><title><p>a</title>",
                "<head><noscript><img>a</body></noscript>",
                "<head><template><p>a<template><p></template><p></body>",
                "<head><template><p>a<template><p></template><p></template>",
                "<head><template><template/><p></template>",
                "<html><title><p>a</title><noscript><img></noscript><template><p></template>",
                "<!DOCTYPE html><noscript><img></noscript>",
                "<meta><template><p></template>",
            ],
        );
        assert_in_body(
            true,
            &[
                "<head><title><p>a</title><p>",
                "<head><title/><p>",
                "<head><noscript/><p>",
                "<head><template/><p>",
                "<head><template><template></template></template><p>",
                "<head><noscript></noscript>a",
            ],
        );
    }

    #[test]
    fn should_read_the_first_template_of_a_fragment_as_content() {
        assert_in_body(true, &["<template>", "<noscript>", " \n<TEMPLATE>", "</p><template>"]);
        assert_in_body(
            false,
            &[
                "<html><template>",
                "<!doctype html><template>",
                "<title></title><noscript>",
                "<link><template>",
            ],
        );
    }

    #[test]
    fn should_end_the_head_at_text_that_the_search_for_the_head_does_not_ignore() {
        for start in ["", "<html>", "<head>", "<head></head>", "<head><meta>"] {
            let in_body = |text: &str| scan(&format!("{start}{text}")).in_body();
            assert!(in_body("a"), "{start}");
            assert!(in_body(" \n a"), "{start}");
            assert!(in_body("&amp;"), "{start}");
            assert!(in_body(" &zzz; "), "{start}");
            assert!(in_body("&Tab"), "{start}");
            assert!(in_body("&"), "{start}");
            assert!(in_body("&#0;"), "{start}");
            assert!(!in_body(" \t\n\x0c\r"), "{start}");
            assert!(!in_body(""), "{start}");
            assert!(
                !in_body(" &#32;&#x20;&#X20;&#10;&#9;&Tab;&NewLine;&#13;&#12;&#32\n"),
                "{start}"
            );
            assert!(!in_body("&nbsp;&#160;\u{a0}"), "{start}");
        }
    }

    #[test]
    fn should_ignore_text_before_the_html_tag_and_not_after_the_head_tag() {
        assert_in_body(false, &["a<html>", "a<HTML lang=en><head>", "<meta>a<html>"]);
        assert_in_body(true, &["a<htmls>", "a<head>", "<head>a<html>", "</head>a<html>"]);
    }
}
