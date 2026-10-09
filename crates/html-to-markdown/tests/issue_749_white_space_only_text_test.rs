#![allow(missing_docs)]

//! Issue #749: text that is only white space must not make the conversion fail.
//!
//! The document structure records the text of each heading, paragraph and list item. Text that
//! is only white space made that record fail, and the caller lost the markdown of the whole
//! document. Each test below takes one kind of element, fills it with each kind of white space,
//! and converts it with the document structure off and on.

use html_to_markdown_rs::options::{OutputFormat, WhitespaceMode};
use html_to_markdown_rs::types::{DocumentNode, DocumentStructure, NodeContent, build_document_structure};
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};
use std::panic::{AssertUnwindSafe, catch_unwind};

/// Each kind of text that a reader sees as empty, by name.
///
/// ~keep Rust's `trim` removes the characters with the Unicode `White_Space` property. The
/// ~keep zero-width characters and the soft hyphen do not have it, so they stay as text; they
/// ~keep are here because a page uses them as "empty" just as often.
const WHITE_SPACE: &[(&str, &str)] = &[
    ("empty", ""),
    ("space", " "),
    ("two spaces", "  "),
    ("tab", "\t"),
    ("line feed", "\n"),
    ("carriage return and line feed", "\r\n"),
    ("form feed", "\u{c}"),
    ("mixed ASCII", " \n\t "),
    ("space entity", "&#32;"),
    ("tab entity", "&#9;"),
    ("line feed entity", "&#10;"),
    ("non-breaking space entity", "&nbsp;"),
    ("numeric non-breaking space entity", "&#160;"),
    ("non-breaking space", "\u{a0}"),
    ("non-breaking space and space", "&nbsp; "),
    ("space between non-breaking spaces", "&nbsp; &nbsp;"),
    ("next line", "\u{85}"),
    ("ogham space mark", "\u{1680}"),
    ("en space", "\u{2002}"),
    ("em space entity", "&emsp;"),
    ("thin space", "\u{2009}"),
    ("line separator", "\u{2028}"),
    ("paragraph separator", "\u{2029}"),
    ("narrow non-breaking space", "\u{202f}"),
    ("medium mathematical space", "\u{205f}"),
    ("ideographic space", "\u{3000}"),
    ("zero-width space", "\u{200b}"),
    ("zero-width space entity", "&#8203;"),
    ("zero-width non-joiner entity", "&zwnj;"),
    ("zero-width joiner", "\u{200d}"),
    ("word joiner", "\u{2060}"),
    ("byte order mark", "\u{feff}"),
    ("soft hyphen entity", "&shy;"),
    ("zero-width space between spaces", " \u{200b} "),
];

const HEADINGS: &[&str] = &[
    "<h1>Title</h1><h2>{}</h2><p>body text</p>",
    "<h1>{}</h1>",
    "<h3>{}</h3><h4>{}</h4><h5>{}</h5><h6>{}</h6>",
    r#"<h2 id="section">{}</h2><p>body text</p>"#,
    "<h2><span><b>{}</b></span></h2>",
    "<h2><em>{}</em>{}<strong>{}</strong></h2>",
    r##"<h1>Title</h1><h2><a href="#x">{}</a></h2><p>body text</p>"##,
    r##"<h2>Section<a class="anchor" href="#section">{}</a></h2>"##,
    r##"<h2><a class="anchor" href="#section">{}</a>Section</h2>"##,
    "<h2>{}<br>{}</h2>",
    "<blockquote><h2>{}</h2></blockquote>",
    "<ul><li><h2>{}</h2></li></ul>",
];

const PARAGRAPHS: &[&str] = &[
    "<p>{}</p>",
    "<p>before</p><p>{}</p><p>after</p>",
    "<p><span>{}</span></p>",
    "<p><b>{}</b><i>{}</i></p>",
    "<p><em>{}</em>{}<strong>{}</strong></p>",
    "<p><code>{}</code></p>",
    "<p>{}<br>{}</p>",
    "<blockquote><p>{}</p></blockquote>",
    "<blockquote>{}</blockquote>",
    "<div><p>{}</p></div>",
    "<div>{}</div>",
    "<section><p>{}</p><p>text</p></section>",
];

const LIST_ITEMS: &[&str] = &[
    "<ul><li>{}</li></ul>",
    "<ol><li>{}</li><li>text</li></ol>",
    "<ul><li>text</li><li>{}</li></ul>",
    "<ul><li><p>{}</p></li></ul>",
    "<ul><li>{}<ul><li>{}</li></ul></li></ul>",
    "<ul><li>text<ul><li>{}</li></ul>{}</li></ul>",
    r##"<ul><li><a href="#x">{}</a></li></ul>"##,
    "<ul><li><b>{}</b></li></ul>",
    r#"<ul><li><input type="checkbox">{}</li></ul>"#,
    "<li>{}</li>",
    "<ul>{}<li>{}</li>{}</ul>",
];

const TABLES: &[&str] = &[
    "<table><tr><td>{}</td></tr></table>",
    "<table><tr><th>{}</th><th>b</th></tr><tr><td>{}</td><td>d</td></tr></table>",
    "<table><caption>{}</caption><tr><td>a</td></tr></table>",
    "<table><tr><td><p>{}</p></td><td><h2>{}</h2></td></tr></table>",
    "<table><tr><td><ul><li>{}</li></ul></td></tr></table>",
    r##"<table><tr><td><a href="#x">{}</a></td><td><b>{}</b></td></tr></table>"##,
    "<table>{}<tr>{}<td>a</td>{}</tr>{}</table>",
];

const LINKS_AND_IMAGES: &[&str] = &[
    r#"<p><a href="/x">{}</a></p>"#,
    r#"<a href="/x">{}</a>"#,
    r#"<p>text <a href="/x">{}</a> text</p>"#,
    r#"<p><a href="/x"><b>{}</b></a></p>"#,
    r#"<p><img src="/a.png" alt="{}"></p>"#,
    r#"<img src="/a.png" alt="{}">"#,
    r#"<h2><img src="/a.png" alt="{}"></h2>"#,
    r#"<ul><li><img src="/a.png" alt="{}"></li></ul>"#,
    r#"<p><a href="/x"><img src="/a.png" alt="{}"></a></p>"#,
    r#"<p><a href="/x" title="{}">{}</a></p>"#,
    r#"<figure><img src="/a.png" alt="{}"><figcaption>{}</figcaption></figure>"#,
];

/// White space beside images and inline elements, in each block that records its text.
const INLINE_RUNS: &[&str] = &[
    r#"<img src="/a.png" alt="a">{}<img src="/b.png" alt="b">"#,
    r#"<img src="/a.png" alt="a">{}"#,
    r#"{}<img src="/a.png" alt="a">"#,
    r#"<img src="/a.png">{}<img src="/b.png">"#,
    r#"<span>{}</span><img src="/a.png" alt="a">"#,
    r#"<b>{}</b><img src="/a.png" alt="a">{}<i>{}</i>"#,
    r##"<a href="#x"><img src="/a.png" alt="a"></a>{}<a href="#y"><img src="/b.png" alt="b"></a>"##,
    r#"<img src="/a.png" alt="a">{}<img src="/b.png" alt="b">{}<img src="/c.png" alt="c">"#,
    "<b>{}</b>{}<i>{}</i>",
    "<svg></svg>{}<svg></svg>",
];

const INLINE_RUN_BLOCKS: &[&str] = &[
    "<p>{}</p>",
    "<h2>{}</h2>",
    "<ul><li>{}</li></ul>",
    "<div>{}</div>",
    "<table><tr><td>{}</td></tr></table>",
    "<blockquote><p>{}</p></blockquote>",
];

const OTHER_BLOCKS: &[&str] = &[
    "{}",
    "<pre>{}</pre>",
    "<pre><code>{}</code></pre>",
    "<dl><dt>{}</dt><dd>{}</dd></dl>",
    "<details><summary>{}</summary>{}</details>",
    "<html><head><title>{}</title></head><body>{}</body></html>",
    "<script>{}</script><style>{}</style><p>text</p>",
    "<button>{}</button><label>{}</label>",
    "<p>text</p>{}<p>text</p>",
];

struct Case {
    label: String,
    html: String,
}

fn fill(templates: &[&str]) -> Vec<Case> {
    let mut cases = Vec::new();
    for template in templates {
        for (name, white_space) in WHITE_SPACE {
            cases.push(Case {
                label: format!("{template} with {name}"),
                html: template.replace("{}", white_space),
            });
        }
    }
    cases
}

fn inline_run_templates() -> Vec<String> {
    let mut templates = Vec::new();
    for block in INLINE_RUN_BLOCKS {
        for run in INLINE_RUNS {
            templates.push(block.replace("{}", run));
        }
    }
    templates
}

fn every_template() -> Vec<String> {
    let mut templates: Vec<String> = [HEADINGS, PARAGRAPHS, LIST_ITEMS, TABLES, LINKS_AND_IMAGES, OTHER_BLOCKS]
        .iter()
        .flat_map(|class| class.iter().map(|template| (*template).to_owned()))
        .collect();
    templates.extend(inline_run_templates());
    templates
}

fn with_structure(options: &ConversionOptions, include_document_structure: bool) -> ConversionOptions {
    ConversionOptions {
        include_document_structure,
        ..options.clone()
    }
}

/// The text that the structure records for a heading, a paragraph or a list item.
fn recorded_text(node: &DocumentNode) -> Option<&str> {
    match &node.content {
        NodeContent::Heading { text, .. } | NodeContent::Paragraph { text } | NodeContent::ListItem { text } => {
            Some(text)
        }
        _ => None,
    }
}

/// Each recorded text is not empty and has no white space at its ends, and each annotation is
/// a range of that text.
fn check_document(document: &DocumentStructure) -> Result<(), String> {
    for node in &document.nodes {
        let Some(text) = recorded_text(node) else {
            continue;
        };
        if text.is_empty() || text.trim() != text {
            return Err(format!("the structure records the text {text:?}"));
        }
        for annotation in &node.annotations {
            let (start, end) = (annotation.start as usize, annotation.end as usize);
            if start >= end || !text.is_char_boundary(start) || !text.is_char_boundary(end) || end > text.len() {
                return Err(format!(
                    "the annotation {start}..{end} is not a range of the text {text:?}"
                ));
            }
        }
    }
    Ok(())
}

/// The conversion returns a result with the structure off and on, and the markdown is the same.
fn check_conversion(html: &str, options: &ConversionOptions) -> Result<(), String> {
    let without = convert(html, Some(with_structure(options, false)))
        .map_err(|error| format!("the conversion fails with the structure off: {error}"))?;
    let with = convert(html, Some(with_structure(options, true)))
        .map_err(|error| format!("the conversion fails with the structure on: {error}"))?;
    if with.content != without.content {
        return Err(format!(
            "the markdown is {:?} with the structure on and {:?} with the structure off",
            with.content, without.content
        ));
    }
    with.document.as_ref().map_or(Ok(()), check_document)
}

/// Run `check` on each case and fail once, with the count and the first failures.
///
/// ~keep A loop that stops at the first failure hides the others, and the count of failures is
/// ~keep the measure of how much of a class an edit breaks.
fn assert_each(cases: &[Case], check: impl Fn(&Case) -> Result<(), String>) {
    assert_none(&failures_of(cases, check), cases.len());
}

fn failures_of(cases: &[Case], check: impl Fn(&Case) -> Result<(), String>) -> Vec<String> {
    cases
        .iter()
        .filter_map(|case| {
            check(case)
                .err()
                .map(|reason| format!("{} ({:?}): {reason}", case.label, case.html))
        })
        .collect()
}

fn assert_none(failures: &[String], case_count: usize) {
    assert!(case_count > 0, "the class has no case");
    assert!(
        failures.is_empty(),
        "{} of {case_count} inputs fail; the first are:\n{}",
        failures.len(),
        failures.iter().take(8).cloned().collect::<Vec<_>>().join("\n")
    );
}

fn assert_class_converts(templates: &[&str]) {
    let options = ConversionOptions::default();
    assert_each(&fill(templates), |case| check_conversion(&case.html, &options));
}

#[test]
fn should_convert_the_inputs_of_the_issue_to_the_markdown_of_the_structure_off() {
    let inputs = [
        (
            "<h1>Title</h1><h2>Sub</h2><p>body text</p>",
            "# Title\n\n## Sub\n\nbody text\n",
            5,
        ),
        ("<h1>Title</h1><h2> </h2><p>body text</p>", "# Title\n\nbody text\n", 3),
        (
            "<h1>Title</h1><h2>&nbsp;</h2><p>body text</p>",
            "# Title\n\nbody text\n",
            3,
        ),
        (
            r##"<h1>Title</h1><h2><a href="#x"> </a></h2><p>body text</p>"##,
            "# Title\n\n## [#x](#x)\n\nbody text\n",
            3,
        ),
        (
            r#"<p><img src="/a.png" alt="a"> <img src="/b.png" alt="b"></p>"#,
            "![a](/a.png) ![b](/b.png)\n",
            2,
        ),
        ("<h1>Title</h1><h2></h2><p>body text</p>", "# Title\n\nbody text\n", 3),
    ];
    for (html, markdown, node_count) in inputs {
        for include_document_structure in [false, true] {
            let result = convert(
                html,
                Some(with_structure(
                    &ConversionOptions::default(),
                    include_document_structure,
                )),
            )
            .unwrap_or_else(|error| panic!("{html:?} with the structure {include_document_structure}: {error}"));
            assert_eq!(result.content.as_deref(), Some(markdown), "{html:?}");
            assert_eq!(
                result.document.map(|document| document.nodes.len()),
                include_document_structure.then_some(node_count),
                "{html:?}"
            );
        }
    }
}

#[test]
fn should_convert_a_heading_that_holds_only_white_space() {
    assert_class_converts(HEADINGS);
}

#[test]
fn should_convert_a_paragraph_that_holds_only_white_space() {
    assert_class_converts(PARAGRAPHS);
}

#[test]
fn should_convert_a_list_item_that_holds_only_white_space() {
    assert_class_converts(LIST_ITEMS);
}

#[test]
fn should_convert_a_table_cell_or_caption_that_holds_only_white_space() {
    assert_class_converts(TABLES);
}

#[test]
fn should_convert_link_text_or_image_alt_that_is_only_white_space() {
    assert_class_converts(LINKS_AND_IMAGES);
}

#[test]
fn should_convert_white_space_beside_images_and_inline_elements() {
    let templates = inline_run_templates();
    let templates: Vec<&str> = templates.iter().map(String::as_str).collect();
    assert_class_converts(&templates);
}

#[test]
fn should_convert_other_blocks_that_hold_only_white_space() {
    assert_class_converts(OTHER_BLOCKS);
}

#[test]
fn should_convert_white_space_only_text_under_each_output_option() {
    let option_sets = [
        (
            "tier 2",
            ConversionOptions {
                tier_strategy: TierStrategy::Tier2,
                ..ConversionOptions::default()
            },
        ),
        (
            "strict white space",
            ConversionOptions {
                whitespace_mode: WhitespaceMode::Strict,
                ..ConversionOptions::default()
            },
        ),
        (
            "djot",
            ConversionOptions {
                output_format: OutputFormat::Djot,
                ..ConversionOptions::default()
            },
        ),
        (
            "plain text",
            ConversionOptions {
                output_format: OutputFormat::Plain,
                ..ConversionOptions::default()
            },
        ),
        (
            "wrap",
            ConversionOptions {
                wrap: true,
                ..ConversionOptions::default()
            },
        ),
        (
            "strip newlines",
            ConversionOptions {
                strip_newlines: true,
                ..ConversionOptions::default()
            },
        ),
        (
            "metadata",
            ConversionOptions {
                extract_metadata: true,
                ..ConversionOptions::default()
            },
        ),
    ];
    let templates = every_template();
    let templates: Vec<&str> = templates.iter().map(String::as_str).collect();
    let inputs = fill(&templates);
    let mut failures = Vec::new();
    for (name, options) in &option_sets {
        let found = failures_of(&inputs, |case| check_conversion(&case.html, options));
        failures.extend(found.into_iter().map(|failure| format!("{name}: {failure}")));
    }
    assert_none(&failures, inputs.len() * option_sets.len());
}

/// The structure that `build_document_structure` returns for `html`, or the reason it panics.
fn built_structure(html: &str) -> Result<DocumentStructure, String> {
    let dom = tl::parse(html, tl::ParserOptions::default()).map_err(|error| error.to_string())?;
    catch_unwind(AssertUnwindSafe(|| build_document_structure(&dom)))
        .map_err(|_| "the structure builder panics".to_owned())
}

/// The kind and the text of each heading, paragraph and list item, in document order.
fn recorded_texts(document: &DocumentStructure) -> Vec<(&'static str, String)> {
    document
        .nodes
        .iter()
        .filter_map(|node| {
            let kind = match &node.content {
                NodeContent::Heading { .. } => "heading",
                NodeContent::Paragraph { .. } => "paragraph",
                NodeContent::ListItem { .. } => "list item",
                _ => return None,
            };
            recorded_text(node).map(|text| (kind, text.to_owned()))
        })
        .collect()
}

#[test]
fn should_build_a_structure_from_a_parsed_document_that_holds_only_white_space() {
    let templates = every_template();
    let templates: Vec<&str> = templates.iter().map(String::as_str).collect();
    assert_each(&fill(&templates), |case| check_document(&built_structure(&case.html)?));
}

/// Blocks that hold one text each and nothing nested, so the conversion and the builder record
/// the same nodes for them.
const PLAIN_BLOCKS: &[&str] = &[
    "<h1>Title</h1><h2>{}</h2><p>body text</p>",
    "<h3>{}</h3>",
    "<p>{}</p><p>body text</p>",
    "<ul><li>{}</li><li>item</li></ul>",
    "<ol><li>item</li><li>{}</li></ol>",
];

#[test]
fn should_record_the_same_texts_in_the_builder_and_in_the_conversion() {
    let templates = PLAIN_BLOCKS;
    let options = with_structure(&ConversionOptions::default(), true);
    assert_each(&fill(templates), |case| {
        let converted = convert(&case.html, Some(options.clone())).map_err(|error| error.to_string())?;
        let converted = recorded_texts(&converted.document.ok_or("the conversion has no structure")?);
        let built = recorded_texts(&built_structure(&case.html)?);
        if converted == built {
            Ok(())
        } else {
            Err(format!(
                "the conversion records {converted:?} and the builder records {built:?}"
            ))
        }
    });
}
