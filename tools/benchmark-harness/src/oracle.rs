//! Golden-oracle snapshot tests for html-to-markdown-rs.
//!
//! Each fixture is run through four [`Permutation`] variants, producing a
//! deterministic Markdown snapshot stored under `snapshots/`.  Running
//! `htmbench oracle` verifies that the current output matches the stored
//! snapshot; `htmbench oracle --bless` overwrites them.
//!
//! # Non-determinism note
//!
//! html5ever serializes SVG attribute ordering non-deterministically across
//! runs (OS-level HashMap seed varies).  To keep snapshots stable, **every**
//! permutation sets `skip_images = true`, which strips `<img>` and inline
//! image tags before parsing.  Oracle tests validate Markdown structure, not
//! image content.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use html_to_markdown_rs::options::ConversionOptions;
use html_to_markdown_rs::options::validation::LinkStyle;

/// The four conversion configurations used for oracle testing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permutation {
    /// Default options with images skipped (for determinism).
    Default,
    /// Explicit `skip_images = true` (same as Default in practice).
    SkipImages,
    /// Metadata extraction disabled.
    NoMetadata,
    /// Reference-style links instead of inline links.
    ReferenceLinks,
}

impl Permutation {
    /// All permutations in a stable order.
    pub const ALL: &'static [Permutation] = &[
        Permutation::Default,
        Permutation::SkipImages,
        Permutation::NoMetadata,
        Permutation::ReferenceLinks,
    ];

    /// Short ASCII slug used in snapshot file names.
    pub fn slug(self) -> &'static str {
        match self {
            Permutation::Default => "default",
            Permutation::SkipImages => "skip_images",
            Permutation::NoMetadata => "no_metadata",
            Permutation::ReferenceLinks => "reference_links",
        }
    }

    /// Build the `ConversionOptions` for this permutation.
    ///
    /// All permutations set `skip_images = true` to avoid html5ever SVG
    /// attribute non-determinism.
    pub fn options(self) -> ConversionOptions {
        let base = ConversionOptions::builder().skip_images(true);
        match self {
            Permutation::Default | Permutation::SkipImages => base.build(),
            Permutation::NoMetadata => base.extract_metadata(false).build(),
            Permutation::ReferenceLinks => base.link_style(LinkStyle::Reference).build(),
        }
    }
}

/// Derive the snapshot file name for a fixture + permutation pair.
///
/// The relative path separators are replaced with `__` and the permutation
/// slug is appended, e.g.:
/// `mdream/nuxt-example.html` + `default` → `mdream__nuxt-example.html__default.md`
fn snapshot_name(rel_path: &str, perm: Permutation) -> String {
    let safe = rel_path.replace(['/', '\\'], "__");
    format!("{safe}__{}.md", perm.slug())
}

/// Convert `html` with `perm` and write the result to `snapshots_dir`.
///
/// Creates parent directories if they do not exist.
pub fn bless(snapshots_dir: &Path, rel_path: &str, html: &str, perm: Permutation) -> Result<()> {
    let opts = perm.options();
    let result = html_to_markdown_rs::convert(html, Some(opts))
        .with_context(|| format!("converting fixture {rel_path} for perm {:?}", perm.slug()))?;
    let content = result.content.as_deref().unwrap_or("");

    fs::create_dir_all(snapshots_dir).with_context(|| format!("creating snapshots dir {}", snapshots_dir.display()))?;

    let snap_path = snapshots_dir.join(snapshot_name(rel_path, perm));
    fs::write(&snap_path, content).with_context(|| format!("writing snapshot {}", snap_path.display()))?;

    Ok(())
}

/// Convert `html` with `perm` and compare against the stored snapshot.
///
/// Returns `Ok(())` on match.  Returns an error with a diff when the output
/// differs from the stored snapshot, and an error if no snapshot exists.
pub fn compare(snapshots_dir: &Path, rel_path: &str, html: &str, perm: Permutation) -> Result<()> {
    let snap_path = snapshots_dir.join(snapshot_name(rel_path, perm));
    let expected = fs::read_to_string(&snap_path)
        .with_context(|| format!("reading snapshot {} (run --bless to create it)", snap_path.display()))?;

    let opts = perm.options();
    let result = html_to_markdown_rs::convert(html, Some(opts))
        .with_context(|| format!("converting fixture {rel_path} for perm {:?}", perm.slug()))?;
    let actual = result.content.as_deref().unwrap_or("");

    if actual != expected {
        anyhow::bail!(
            "snapshot mismatch for {rel_path} (perm: {}):\n{}",
            perm.slug(),
            build_diff(&expected, actual),
        );
    }

    Ok(())
}

/// Produce a human-readable unified diff between `expected` and `actual`.
fn build_diff(expected: &str, actual: &str) -> String {
    // Pretty_assertions provides coloured output; for programmatic use we
    // fall back to a simple first-difference summary.
    let exp_lines: Vec<&str> = expected.lines().collect();
    let act_lines: Vec<&str> = actual.lines().collect();

    let mut out = String::new();
    let limit = exp_lines.len().max(act_lines.len());
    let mut shown = 0;
    for i in 0..limit {
        let e = exp_lines.get(i).copied().unwrap_or("<missing>");
        let a = act_lines.get(i).copied().unwrap_or("<missing>");
        if e != a {
            out.push_str(&format!("  line {}: expected {:?}\n", i + 1, e));
            out.push_str(&format!("  line {}: actual   {:?}\n", i + 1, a));
            shown += 1;
            if shown >= 5 {
                out.push_str("  ... (truncated)\n");
                break;
            }
        }
    }
    if out.is_empty() {
        // Line counts may differ
        out.push_str(&format!(
            "  line count differs: expected {} vs actual {}\n",
            exp_lines.len(),
            act_lines.len()
        ));
    }
    out
}
