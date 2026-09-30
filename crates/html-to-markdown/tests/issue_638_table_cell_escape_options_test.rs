#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! Regression tests for issue #638: the full converter escaped `*` and `_` in a table cell
//! whatever `escape_asterisks` and `escape_underscores` said, so `<td>sample_value</td>`
//! came out as `sample\_value` while `<p>sample_value</p>` stayed `sample_value`. Every
//! escape option now acts the same in a cell as in a paragraph; a `|` in a cell is still
//! escaped, because it is table syntax.

use html_to_markdown_rs::prescan;
use html_to_markdown_rs::tier1;
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

const TEXT: &str = "sample_value and 2*3 [x] a~b";

const MISC: u8 = 1;
const ASTERISKS: u8 = 2;
const UNDERSCORES: u8 = 4;
const ASCII: u8 = 8;

/// One combination of the four escape options, one bit per option.
#[derive(Clone, Copy)]
struct Flags(u8);

impl Flags {
    const fn has(self, flag: u8) -> bool {
        self.0 & flag != 0
    }
}

impl std::fmt::Debug for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Flags")
            .field("misc", &self.has(MISC))
            .field("asterisks", &self.has(ASTERISKS))
            .field("underscores", &self.has(UNDERSCORES))
            .field("ascii", &self.has(ASCII))
            .finish()
    }
}

fn all_flags() -> Vec<Flags> {
    (0u8..16).map(Flags).collect()
}

fn options(flags: Flags) -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        escape_misc: flags.has(MISC),
        escape_asterisks: flags.has(ASTERISKS),
        escape_underscores: flags.has(UNDERSCORES),
        escape_ascii: flags.has(ASCII),
        ..ConversionOptions::default()
    }
}

fn tier2(html: &str, options: &ConversionOptions) -> String {
    let options = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        ..options.clone()
    };
    convert(html, Some(options))
        .expect("conversion must succeed")
        .content
        .unwrap_or_default()
}

fn auto(html: &str, options: &ConversionOptions) -> String {
    convert(html, Some(options.clone()))
        .expect("conversion must succeed")
        .content
        .unwrap_or_default()
}

fn tier1_run(html: &str, options: &ConversionOptions) -> Result<String, tier1::BailReason> {
    let (cleaned, report) = prescan::run(html);
    let options = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        ..options.clone()
    };
    tier1::run(cleaned.as_ref(), &report, &options)
}

fn render(markdown: &str) -> String {
    let mut options = comrak::Options::default();
    options.extension.table = true;
    comrak::markdown_to_html(markdown, &options)
}

/// The text of the first cell on the first line of a pipe table.
fn first_cell(markdown: &str) -> String {
    let line = markdown
        .lines()
        .find(|line| line.trim_start().starts_with('|'))
        .unwrap_or_else(|| panic!("no table row in {markdown:?}"));
    let inner = line.trim().strip_prefix('|').expect("row starts with a pipe");
    let inner = inner.strip_suffix('|').expect("row ends with a pipe");
    inner.trim().to_string()
}

/// The paragraph text the same options write for `text`: the oracle for a cell.
fn paragraph(text: &str, options: &ConversionOptions) -> String {
    tier2(&format!("<p>{text}</p>"), options).trim().to_string()
}

/// Cell shapes: a text-only cell and a cell whose text sits under a tag take different
/// escape paths in the full converter, and a header cell is written by the same code.
fn cell_shapes(text: &str) -> Vec<(&'static str, String)> {
    vec![
        ("td text", format!("<table><tr><td>{text}</td></tr></table>")),
        (
            "td span",
            format!("<table><tr><td><span>{text}</span></td></tr></table>"),
        ),
        ("th text", format!("<table><tr><th>{text}</th></tr></table>")),
        (
            "th strong",
            format!("<table><tr><th><strong>x</strong> {text}</th></tr></table>"),
        ),
    ]
}

fn table_variants(base: ConversionOptions) -> Vec<(&'static str, ConversionOptions)> {
    vec![
        ("default", base.clone()),
        (
            "br_in_tables",
            ConversionOptions {
                br_in_tables: true,
                ..base.clone()
            },
        ),
        (
            "compact_tables",
            ConversionOptions {
                compact_tables: true,
                ..base
            },
        ),
    ]
}

#[test]
fn issue_638_escape_underscores_false_leaves_cell_underscore_alone() {
    let options = ConversionOptions {
        escape_underscores: false,
        ..ConversionOptions::default()
    };
    let paragraph = auto("<p>sample_value</p>", &options);
    let table = auto("<table><tr><td>sample_value</td></tr></table>", &options);
    assert!(paragraph.contains("sample_value"), "paragraph: {paragraph:?}");
    assert!(
        table.contains("sample_value") && !table.contains(r"sample\_value"),
        "escape_underscores=false was ignored in a table cell: {table:?}"
    );
}

#[test]
fn issue_638_escape_underscores_true_still_escapes_cell_underscore() {
    let options = ConversionOptions {
        escape_underscores: true,
        ..ConversionOptions::default()
    };
    let table = auto("<table><tr><td>sample_value</td></tr></table>", &options);
    assert!(table.contains(r"sample\_value"), "escape_underscores=true: {table:?}");
}

#[test]
fn issue_638_escape_asterisks_false_leaves_cell_asterisk_alone() {
    let options = ConversionOptions {
        escape_asterisks: false,
        ..ConversionOptions::default()
    };
    let table = auto("<table><tr><td>2*3 = 6</td></tr></table>", &options);
    assert!(
        table.contains("2*3 = 6") && !table.contains(r"2\*3"),
        "escape_asterisks=false was ignored in a table cell: {table:?}"
    );
}

#[test]
fn issue_638_full_converter_cell_escapes_like_paragraph_for_every_flag() {
    for flags in all_flags() {
        let base = options(flags);
        let expected = paragraph(TEXT, &base);
        for (variant, opts) in table_variants(base) {
            for (shape, html) in cell_shapes(TEXT) {
                let cell = first_cell(&tier2(&html, &opts));
                let cell = cell.strip_prefix("**x** ").unwrap_or(&cell);
                assert_eq!(
                    cell, expected,
                    "cell text differs from paragraph text: {flags:?}, {variant}, {shape}, {html}"
                );
            }
        }
    }
}

#[test]
fn issue_638_pipe_in_cell_is_escaped_for_every_flag() {
    for flags in all_flags() {
        for (variant, opts) in table_variants(options(flags)) {
            for (shape, html) in cell_shapes("a|b") {
                let out = tier2(&html, &opts);
                let cell = first_cell(&out);
                let cell = cell.strip_prefix("**x** ").unwrap_or(&cell);
                assert_eq!(
                    cell, r"a\|b",
                    "pipe not escaped: {flags:?}, {variant}, {shape}, {out:?}"
                );
                let html_out = render(&out);
                let cells = html_out.matches("<th>").count() + html_out.matches("<td>").count();
                assert_eq!(
                    cells, 1,
                    "the pipe split the cell: {flags:?}, {variant}, {shape}, {html_out:?}"
                );
            }
        }
    }
}

#[test]
fn issue_638_fast_converter_agrees_with_full_converter_in_cells() {
    // ~keep The fast converter hands every document with an escape flag set to the full
    // ~keep converter, so the all-off options are the only ones it writes itself.
    let base = options(Flags(0));
    // ~keep The router also hands compact_tables to the full converter, so it is left out.
    for (variant, opts) in table_variants(base).into_iter().filter(|(v, _)| *v != "compact_tables") {
        for (shape, html) in cell_shapes(TEXT) {
            let full = tier2(&html, &opts);
            match tier1_run(&html, &opts) {
                Ok(fast) => assert_eq!(fast, full, "fast and full converters differ: {variant}, {shape}"),
                Err(reason) => panic!("fast converter bailed ({reason:?}): {variant}, {shape}"),
            }
        }
    }
}
