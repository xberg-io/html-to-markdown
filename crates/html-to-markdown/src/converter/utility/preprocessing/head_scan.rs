//! Where the document head is, for a scan that reads the tags of a page before the parse.

use super::markup::matches_tag_start;

/// Where a scan over the tags of a page is, relative to the document head.
///
/// ~keep The head starts at a `<head>` tag that no element of the body precedes, and it ends
/// ~keep once: a browser ignores a `<head>` tag after the head. The end tag of the head and the
/// ~keep body tag are optional, so the head also ends at the first start tag of an element that
/// ~keep it cannot hold: `<head><title>t</title><p>` has its paragraph in the body.
/// ~keep
/// ~keep What a `<template>` or a `<noscript>` of the head holds is not content of the head: a
/// ~keep browser with scripting reads the first as a separate fragment and the second as text.
/// ~keep A tracking pixel, `<noscript><img></noscript>`, does not end the head.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum HeadScan {
    /// No `<head>` tag and no element of the body so far.
    Before,
    /// In the head, between its elements.
    In,
    /// In the text of the `<title>`, where a tag is text.
    InTitle,
    /// In a `<noscript>` of the head, up to its end tag.
    InNoscript,
    /// In a `<template>` of the head, with the count of the `<template>` elements that are open.
    InTemplate(usize),
    /// After the head. A document with no `<head>` tag is here from its first element.
    After,
}

impl HeadScan {
    /// The state after the tag whose name starts at `name_start`.
    ///
    /// `closes_itself` says that a `<template>` or `<noscript>` start tag is written as
    /// self-closing, which the parser reads as an element with no content.
    pub(super) fn after_tag(self, bytes: &[u8], name_start: usize, is_end_tag: bool, closes_itself: bool) -> Self {
        let named = |name: &[u8]| matches_tag_start(bytes, name_start, name);
        let is_start_tag = !is_end_tag && bytes.get(name_start).is_some_and(u8::is_ascii_alphabetic);
        let head_holds = || HEAD_CONTENT_NAMES.iter().any(|name| named(name));
        match self {
            Self::Before if is_start_tag && named(b"head") => Self::In,
            // ~keep A `<template>` or a `<noscript>` with no `<head>` tag before it is content of
            // ~keep the document: a fragment has no head.
            Self::Before if is_start_tag && (named(b"template") || named(b"noscript") || !head_holds()) => Self::After,
            Self::In if is_end_tag && named(b"head") => Self::After,
            Self::In if is_start_tag && named(b"title") => Self::InTitle,
            Self::In if is_start_tag && named(b"noscript") && !closes_itself => Self::InNoscript,
            Self::In if is_start_tag && named(b"template") && !closes_itself => Self::InTemplate(1),
            Self::In if is_start_tag && !head_holds() => Self::After,
            Self::InTitle if is_end_tag && named(b"title") => Self::In,
            Self::InNoscript if is_end_tag && named(b"noscript") => Self::In,
            Self::InTemplate(open) if is_start_tag && named(b"template") && !closes_itself => {
                Self::InTemplate(open + 1)
            }
            Self::InTemplate(1) if is_end_tag && named(b"template") => Self::In,
            Self::InTemplate(open) if is_end_tag && named(b"template") => Self::InTemplate(open - 1),
            state => state,
        }
    }
}

/// The elements whose start tag does not end the document head: the ones that a browser keeps
/// there, and `html` and `head`, whose second start tag it ignores.
const HEAD_CONTENT_NAMES: [&[u8]; 13] = [
    b"base",
    b"basefont",
    b"bgsound",
    b"head",
    b"html",
    b"link",
    b"meta",
    b"noframes",
    b"noscript",
    b"script",
    b"style",
    b"template",
    b"title",
];
