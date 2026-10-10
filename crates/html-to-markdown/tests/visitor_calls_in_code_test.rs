// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![allow(clippy::significant_drop_tightening)]
#![cfg(feature = "visitor")]

//! The calls a visitor gets for a link, a highlight and an image in code.
//!
//! Code writes a link and a highlight as their text. The visitor still gets the calls it gets for
//! the same markup outside code, in the same order: none for a link that is left out (an empty
//! anchor into its own page, which a highlighter writes for each line of code), none for a link in
//! the `<address>` form, none for a link around one heading.

use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult, VisitorHandle};
use html_to_markdown_rs::{CodeBlockStyle, ConversionOptions, convert};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
struct Recorder {
    calls: Vec<String>,
    link_texts: Vec<String>,
    code: Vec<String>,
}

impl HtmlVisitor for Recorder {
    fn visit_link(&mut self, _ctx: &NodeContext<'_>, href: &str, text: &str, _title: Option<&str>) -> VisitResult {
        self.calls.push(format!("link {href}"));
        self.link_texts.push(text.to_string());
        VisitResult::Continue
    }

    fn visit_mark(&mut self, _ctx: &NodeContext<'_>, text: &str) -> VisitResult {
        self.calls.push(format!("mark {text}"));
        VisitResult::Continue
    }

    fn visit_image(&mut self, _ctx: &NodeContext<'_>, src: &str, _alt: &str, _title: Option<&str>) -> VisitResult {
        self.calls.push(format!("image {src}"));
        VisitResult::Continue
    }

    fn visit_code_block(&mut self, _ctx: &NodeContext<'_>, _lang: Option<&str>, code: &str) -> VisitResult {
        self.calls.push("code_block".to_string());
        self.code.push(code.to_string());
        VisitResult::Continue
    }

    fn visit_code_inline(&mut self, _ctx: &NodeContext<'_>, code: &str) -> VisitResult {
        self.calls.push("code_inline".to_string());
        self.code.push(code.to_string());
        VisitResult::Continue
    }
}

fn record(html: &str, style: CodeBlockStyle) -> (Recorder, String) {
    let recorder = Arc::new(Mutex::new(Recorder::default()));
    let handle: VisitorHandle = recorder.clone();
    let options = ConversionOptions {
        extract_metadata: false,
        code_block_style: style,
        visitor: Some(handle),
        ..ConversionOptions::default()
    };
    let content = convert(html, Some(options))
        .expect("the conversion succeeds")
        .content
        .unwrap_or_default();
    let seen = std::mem::take(&mut *recorder.lock().expect("the visitor lock"));
    (seen, content)
}

/// Each row: a name, the page, the calls in order. The calls are those of a run of the same page
/// before code wrote a link as its text.
const CASES: &[(&str, &str, &[&str])] = &[
    (
        "an empty anchor into its own page for each line (MkDocs)",
        "<pre><code><a id=\"l1\" href=\"#l1\"></a>a = 1\n<a id=\"l2\" href=\"#l2\"></a>  b = 2\n</code></pre>",
        &["code_block"],
    ),
    (
        "the same anchors in a line span",
        "<pre><span></span><code><span id=\"s1\"><a id=\"c1\" name=\"c1\" href=\"#c1\"></a><span class=\"k\">a</span>\n</span><span id=\"s2\"><a id=\"c2\" name=\"c2\" href=\"#c2\"></a>  b\n</span></code></pre>",
        &["code_block"],
    ),
    (
        "an empty anchor into its own page that holds an empty element",
        "<pre><span><a href=\"#s\"><span></span></a>x</span></pre>",
        &["code_block"],
    ),
    (
        "an anchor into its own page that holds one space",
        "<pre>a<a href=\"#x\"> </a>b</pre>",
        &["code_block"],
    ),
    (
        "an empty anchor into its own page that has a name",
        "<pre><a href=\"#n\" aria-label=\"N\" title=\"T\"></a>x</pre>",
        &["code_block"],
    ),
    (
        "an empty anchor in a code span",
        "<p>t <code><a href=\"#t\"></a>x</code></p>",
        &["code_inline"],
    ),
    (
        "a link with text in a pre",
        "<pre>see <a href=\"/x\">link</a> end</pre>",
        &["link /x", "code_block"],
    ),
    (
        "a link with text into its own page",
        "<pre>see <a href=\"#x\">link</a> end</pre>",
        &["link #x", "code_block"],
    ),
    (
        "a link in a code span",
        "<p>t <code>b <a href=\"/y\">two</a></code></p>",
        &["link /y", "code_inline"],
    ),
    (
        "a highlight in a pre",
        "<pre>a <mark>hot</mark> b</pre>",
        &["mark hot", "code_block"],
    ),
    (
        "a highlight in a code span",
        "<p>t <code>a <mark>m</mark></code></p>",
        &["mark m", "code_inline"],
    ),
    (
        "an empty highlight",
        "<pre><mark></mark>a</pre>",
        &["mark ", "code_block"],
    ),
    (
        "a link in a highlight",
        "<pre>a <mark>hot <a href=\"/x\">link</a></mark> b</pre>",
        &["mark hot link", "link /x", "code_block"],
    ),
    (
        "a highlight in a link",
        "<pre>a <a href=\"/x\"><mark>hot</mark> link</a> b</pre>",
        &["mark hot", "link /x", "code_block"],
    ),
    (
        "a link and a highlight in a code element in a pre",
        "<pre><code>see <a href=\"/x\">link</a>\n  <mark>m</mark></code></pre>",
        &["link /x", "mark m", "code_block"],
    ),
    (
        "an empty anchor to another page",
        "<pre><a href=\"/x\"></a>a</pre>",
        &["link /x", "code_block"],
    ),
    (
        "an empty anchor to another page with a title",
        "<pre><a href=\"/x\" title=\"T\"></a>a</pre>",
        &["link /x", "code_block"],
    ),
    (
        "three anchors: empty into its own page, with text, empty to another page",
        "<pre><a href=\"#a\"></a>x <a href=\"#b\">y</a> <a href=\"/c\"></a>z</pre>",
        &["link #b", "link /c", "code_block"],
    ),
    (
        "a pre in a list item",
        "<ul><li>t<pre>a <a href=\"/x\">l</a>\n<a href=\"#e\"></a>b</pre></li></ul>",
        &["link /x", "code_block"],
    ),
    (
        "an image alone in an anchor into its own page",
        "<pre><a href=\"#x\"><img src=\"i.png\" alt=\"p\"></a>z</pre>",
        &["image i.png", "link #x", "code_block"],
    ),
    (
        "an image alone in an anchor",
        "<pre><a href=\"/x\"><img src=\"i.png\" alt=\"p\"></a>z</pre>",
        &["image i.png", "link /x", "code_block"],
    ),
    (
        "an anchor with no address",
        "<pre><a id=\"l1\"></a>a</pre>",
        &["code_block"],
    ),
    (
        "a link whose text is its address",
        "<pre>see <a href=\"https://e.org/x\">https://e.org/x</a> end</pre>",
        &["code_block"],
    ),
    (
        "a mail link whose text is its address",
        "<pre>to <a href=\"mailto:a@e.org\">a@e.org</a></pre>",
        &["code_block"],
    ),
    (
        "a link whose text is its address in a code span",
        "<p>t <code><a href=\"https://e.org/x\">https://e.org/x</a></code></p>",
        &["code_inline"],
    ),
    (
        "a link around one heading",
        "<pre><a href=\"/h\"><h2>t</h2></a>x</pre>",
        &["code_block"],
    ),
    (
        "a link around one empty heading",
        "<pre><a href=\"/h\"><h2></h2></a>x</pre>",
        &["link /h", "code_block"],
    ),
];

#[test]
fn a_visitor_gets_the_same_calls_for_links_and_highlights_in_code_as_before() {
    let mut failures = Vec::new();
    for (name, html, expected) in CASES {
        for style in [
            CodeBlockStyle::Backticks,
            CodeBlockStyle::Tildes,
            CodeBlockStyle::Indented,
        ] {
            let (seen, content) = record(html, style);
            if seen.calls != *expected {
                failures.push(format!(
                    "{name} ({style:?}): calls {:?}, expected {expected:?}; output {content:?}",
                    seen.calls
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} rows differ:\n{}",
        failures.len(),
        CASES.len() * 3,
        failures.join("\n")
    );
}

#[test]
fn an_anchor_that_gets_no_call_still_writes_its_text_in_code() {
    for (html, code, output) in [
        ("<pre>a<a href=\"#x\"> </a>b</pre>", "a b", "```\na b\n```\n"),
        (
            "<pre>see <a href=\"https://e.org/x\">https://e.org/x</a> end</pre>",
            "see https://e.org/x end",
            "```\nsee https://e.org/x end\n```\n",
        ),
        (
            "<pre><a id=\"l1\" href=\"#l1\"></a>a = 1\n<a id=\"l2\" href=\"#l2\"></a>  b = 2\n</pre>",
            "a = 1\n  b = 2\n",
            "```\na = 1\n  b = 2\n```\n",
        ),
    ] {
        let (seen, content) = record(html, CodeBlockStyle::Backticks);
        assert_eq!(seen.code, [code], "the code argument for {html:?}");
        assert_eq!(content, output, "the output for {html:?}");
    }
}

#[test]
fn the_text_argument_of_a_link_in_code_is_the_text_the_code_shows() {
    let (seen, content) = record(
        "<pre>a <a href=\"/x\"><mark>hot</mark> link</a> b</pre>",
        CodeBlockStyle::Backticks,
    );
    assert_eq!(seen.link_texts, ["hot link"]);
    assert_eq!(seen.code, ["a hot link b"], "the code argument holds no link marks");
    assert_eq!(content, "```\na hot link b\n```\n");
}

#[test]
fn a_link_in_code_that_gets_no_call_is_written_the_same_without_a_visitor() {
    for (name, html, _) in CASES {
        let (_, with_visitor) = record(html, CodeBlockStyle::Backticks);
        let without = convert(
            html,
            Some(ConversionOptions {
                extract_metadata: false,
                ..ConversionOptions::default()
            }),
        )
        .expect("the conversion succeeds")
        .content
        .unwrap_or_default();
        assert_eq!(
            with_visitor, without,
            "{name}: a visitor that continues changes no output"
        );
    }
}
