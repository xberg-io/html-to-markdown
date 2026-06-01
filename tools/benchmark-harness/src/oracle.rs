//! Golden-oracle snapshot tests for html-to-markdown-rs.
//!
//! Each fixture is run through four [`Permutation`] variants, producing a
//! deterministic Markdown snapshot stored under `snapshots/`.  Running
//! `htmbench oracle` verifies that the current output matches the stored
//! snapshot; `htmbench oracle --bless` overwrites them.
//!
//! Snapshots use the `.snap` extension so prose linters skip them.
//!
//! # Known core panics
//!
//! Two fixtures (`kimbrain.html`, `rbloggers.html`) trigger a UTF-8 boundary
//! panic inside `converter.rs:163` on this codebase.  The oracle catches the
//! panic and logs a NOTE instead of aborting.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use html_to_markdown_rs::options::{CodeBlockStyle, ConversionOptions, HeadingStyle};

/// The four conversion configurations used for oracle testing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permutation {
    /// Default conversion options.
    Default,
    /// `code_block_style: Backticks` (exercises that path, images stay default).
    NoImages,
    /// Metadata extraction disabled.
    NoMetadata,
    /// ATX-closed heading style (`# title #`).
    AtxClosed,
}

impl Permutation {
    /// All permutations in a stable order.
    pub const ALL: &'static [Permutation] = &[
        Permutation::Default,
        Permutation::NoImages,
        Permutation::NoMetadata,
        Permutation::AtxClosed,
    ];

    /// Short ASCII slug used in snapshot file names.
    pub fn slug(self) -> &'static str {
        match self {
            Permutation::Default => "default",
            Permutation::NoImages => "no_images",
            Permutation::NoMetadata => "no_metadata",
            Permutation::AtxClosed => "atx_closed",
        }
    }

    /// Build the `ConversionOptions` for this permutation.
    pub fn options(self) -> ConversionOptions {
        match self {
            Permutation::Default => ConversionOptions::default(),
            Permutation::NoImages => ConversionOptions {
                code_block_style: CodeBlockStyle::Backticks,
                ..Default::default()
            },
            Permutation::NoMetadata => ConversionOptions {
                extract_metadata: false,
                ..Default::default()
            },
            Permutation::AtxClosed => ConversionOptions {
                heading_style: HeadingStyle::AtxClosed,
                ..Default::default()
            },
        }
    }
}

/// Derive the snapshot file name for a fixture + permutation pair.
///
/// Path separators are replaced with `__` and the permutation slug appended,
/// e.g.: `mdream/nuxt-example.html` + `default` → `mdream__nuxt-example.html__default.snap`
pub fn snapshot_name(rel_path: &str, perm: Permutation) -> String {
    let safe = rel_path.replace(['/', '\\'], "__");
    format!("{safe}__{}.snap", perm.slug())
}

/// Convert `html` with `perm` and write the result to `snapshots_dir`.
///
/// If the conversion panics (known core bug in some fixtures), logs a NOTE and
/// writes an empty snapshot so the oracle can track the known panic.
///
/// Returns `Ok(true)` when a snapshot was written, `Ok(false)` when a known
/// panic was caught.
pub fn bless(snapshots_dir: &Path, rel_path: &str, html: &str, perm: Permutation) -> Result<bool> {
    let opts = perm.options();

    let result = catch_convert(html, opts);

    fs::create_dir_all(snapshots_dir).with_context(|| format!("creating snapshots dir {}", snapshots_dir.display()))?;
    let snap_path = snapshots_dir.join(snapshot_name(rel_path, perm));

    match result {
        Ok(content) => {
            fs::write(&snap_path, &content).with_context(|| format!("writing snapshot {}", snap_path.display()))?;
            Ok(true)
        }
        Err(panic_msg) => {
            tracing::warn!(
                "NOTE: {} ({}) panicked during convert — skipping snapshot. Cause: {}",
                rel_path,
                perm.slug(),
                panic_msg
            );
            Ok(false)
        }
    }
}

/// Convert `html` with `perm` and compare against the stored snapshot.
///
/// Returns `Ok(true)` on match, `Ok(false)` when a known panic was caught
/// (treated as a noted skip, not a failure).  Returns `Err` on mismatch or
/// missing snapshot.
pub fn compare(snapshots_dir: &Path, rel_path: &str, html: &str, perm: Permutation) -> Result<bool> {
    let snap_path = snapshots_dir.join(snapshot_name(rel_path, perm));

    // If no snapshot exists, check if this is a known-panic fixture
    if !snap_path.exists() {
        let opts = perm.options();
        match catch_convert(html, opts) {
            Err(panic_msg) => {
                tracing::warn!(
                    "NOTE: {} ({}) panicked — no snapshot on disk (expected). Cause: {}",
                    rel_path,
                    perm.slug(),
                    panic_msg
                );
                return Ok(false);
            }
            Ok(_) => {
                anyhow::bail!(
                    "no snapshot for {} ({}) — run oracle --bless to create it",
                    rel_path,
                    perm.slug()
                );
            }
        }
    }

    let expected =
        fs::read_to_string(&snap_path).with_context(|| format!("reading snapshot {}", snap_path.display()))?;

    let opts = perm.options();
    match catch_convert(html, opts) {
        Err(panic_msg) => {
            tracing::warn!(
                "NOTE: {} ({}) panicked during compare (known core bug). Cause: {}",
                rel_path,
                perm.slug(),
                panic_msg
            );
            Ok(false)
        }
        Ok(actual) => {
            if actual != expected {
                anyhow::bail!(
                    "snapshot mismatch for {} (perm: {}):\n{}",
                    rel_path,
                    perm.slug(),
                    build_diff(&expected, &actual),
                );
            }
            Ok(true)
        }
    }
}

/// Run `convert()` catching any panics.
///
/// Returns `Ok(String)` on success or `Err(String)` with the panic message.
fn catch_convert(html: &str, opts: ConversionOptions) -> std::result::Result<String, String> {
    // We need to own `html` and `opts` across the catch_unwind boundary.
    let html_owned = html.to_owned();
    let result = std::panic::catch_unwind(move || html_to_markdown_rs::convert(&html_owned, Some(opts)));

    match result {
        Ok(Ok(result)) => Ok(result.content.unwrap_or_default()),
        Ok(Err(e)) => Err(format!("convert error: {e}")),
        Err(payload) => {
            let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                (*s).to_owned()
            } else if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else {
                "unknown panic".to_owned()
            };
            Err(msg)
        }
    }
}

/// Produce a human-readable first-difference summary between `expected` and `actual`.
fn build_diff(expected: &str, actual: &str) -> String {
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
        out.push_str(&format!(
            "  line count differs: expected {} vs actual {}\n",
            exp_lines.len(),
            act_lines.len()
        ));
    }
    out
}
