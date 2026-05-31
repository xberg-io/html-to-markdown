//! Fixture corpus survey — detects HTML features that stress the parser.
//!
//! Counts occurrences of categories across the fixture corpus and prints a
//! summary table.  Useful for sanity-checking that the fixture set exercises
//! the intended code paths.

use std::collections::HashMap;

use anyhow::{Context, Result};

use crate::fixture::Loader;

/// Per-fixture feature detection results.
#[derive(Debug, Clone, Default)]
pub struct FeatureCounts {
    /// CDATA sections (`<![CDATA[...]]>`).
    pub cdata: usize,
    /// Custom elements (tag names containing a hyphen, e.g. `<my-widget>`).
    pub custom_elements: usize,
    /// Bare `<` characters in text content (potential parse ambiguity).
    pub bare_lt: usize,
    /// `<table>` elements without a `<tbody>` child.
    pub table_no_tbody: usize,
}

/// Aggregate survey statistics over the full fixture corpus.
#[derive(Debug, Clone, Default)]
pub struct SurveyStats {
    /// Map from fixture relative path to detected feature counts.
    pub by_fixture: HashMap<String, FeatureCounts>,
    /// Map from group tag to aggregate feature counts.
    pub by_group: HashMap<String, FeatureCounts>,
}

impl SurveyStats {
    fn add(&mut self, rel_path: &str, group: &str, counts: FeatureCounts) {
        let group_entry = self.by_group.entry(group.to_owned()).or_default();
        group_entry.cdata += counts.cdata;
        group_entry.custom_elements += counts.custom_elements;
        group_entry.bare_lt += counts.bare_lt;
        group_entry.table_no_tbody += counts.table_no_tbody;
        self.by_fixture.insert(rel_path.to_owned(), counts);
    }
}

/// Analyse a raw HTML string and return feature detection counts.
fn detect(html: &str) -> FeatureCounts {
    let cdata = count_occurrences(html, "<![CDATA[");
    let custom_elements = count_custom_elements(html);
    let bare_lt = count_bare_lt(html);
    let table_no_tbody = count_table_no_tbody(html);
    FeatureCounts {
        cdata,
        custom_elements,
        bare_lt,
        table_no_tbody,
    }
}

/// Count opening tags whose name contains a hyphen (custom elements).
fn count_custom_elements(html: &str) -> usize {
    let mut count = 0;
    let mut remaining = html;
    while let Some(pos) = remaining.find('<') {
        let after = &remaining[pos + 1..];
        // Skip closing tags and declarations
        if !after.starts_with('/') && !after.starts_with('!') && !after.starts_with('?') {
            let tag_end = after
                .find(|c: char| c.is_whitespace() || c == '>' || c == '/')
                .unwrap_or(after.len());
            let tag_name = &after[..tag_end];
            if !tag_name.is_empty() && tag_name.contains('-') {
                count += 1;
            }
        }
        remaining = &remaining[pos + 1..];
    }
    count
}

/// Count bare `<` characters not starting a tag/declaration.
fn count_bare_lt(html: &str) -> usize {
    let bytes = html.as_bytes();
    let len = bytes.len();
    let mut count = 0;
    let mut i = 0usize;
    while i < len {
        if bytes[i] == b'<' {
            let next = if i + 1 < len { bytes[i + 1] } else { 0 };
            if next != b'/' && next != b'!' && next != b'?' && !next.is_ascii_alphabetic() {
                count += 1;
            }
        }
        i += 1;
    }
    count
}

/// Count `<table>` elements without a `<tbody>` descendant.
fn count_table_no_tbody(html: &str) -> usize {
    let mut count = 0;
    let lower = html.to_lowercase();
    let mut search = lower.as_str();
    while let Some(table_start) = search.find("<table") {
        let rest = &search[table_start..];
        let table_end = rest.find("</table").map_or(rest.len(), |p| p + 8);
        let table_content = &rest[..table_end];
        if !table_content.contains("<tbody") {
            count += 1;
        }
        search = &search[table_start + 6..];
    }
    count
}

/// Count non-overlapping occurrences of `needle` in `haystack`.
fn count_occurrences(haystack: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut start = 0;
    while let Some(pos) = haystack[start..].find(needle) {
        count += 1;
        start += pos + needle.len();
    }
    count
}

/// Run the survey across all fixtures (optionally filtered by group).
pub fn run_survey(fixtures_dir: &std::path::Path, filter: Option<&str>) -> Result<SurveyStats> {
    let loader = Loader::new(fixtures_dir.to_path_buf());
    let fixtures = loader.load(filter)?;

    let mut stats = SurveyStats::default();
    for fixture in &fixtures {
        let html =
            std::fs::read_to_string(&fixture.path).with_context(|| format!("reading {}", fixture.path.display()))?;
        let counts = detect(&html);
        stats.add(&fixture.rel_path, &fixture.group, counts);
    }
    Ok(stats)
}

/// Print a formatted survey summary to stdout.
pub fn print_survey(stats: &SurveyStats) {
    println!(
        "{:<42}  {:>6}  {:>8}  {:>7}  {:>13}",
        "fixture", "cdata", "custom-el", "bare-lt", "table-no-tbody"
    );
    println!("{}", "-".repeat(90));

    let mut keys: Vec<&String> = stats.by_fixture.keys().collect();
    keys.sort();
    for key in keys {
        let c = &stats.by_fixture[key];
        println!(
            "{:<42}  {:>6}  {:>9}  {:>7}  {:>13}",
            key, c.cdata, c.custom_elements, c.bare_lt, c.table_no_tbody
        );
    }

    println!("{}", "-".repeat(90));
    println!("By group:");
    let mut groups: Vec<&String> = stats.by_group.keys().collect();
    groups.sort();
    for group in groups {
        let c = &stats.by_group[group];
        println!(
            "  {:<40}  {:>6}  {:>9}  {:>7}  {:>13}",
            group, c.cdata, c.custom_elements, c.bare_lt, c.table_no_tbody
        );
    }
}
