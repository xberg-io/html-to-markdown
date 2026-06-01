//! Tier-1: single-pass byte-scanner conversion path.
//!
//! `run` delegates to the byte scanner in `scanner.rs`.  When the scanner
//! encounters a construct it cannot handle, it returns `Err(BailReason::*)`
//! and the dispatcher in `lib.rs::convert` falls back to the Tier-2 path.
//!
//! When `options.extract_metadata` is true, `run` additionally re-parses the
//! head slice captured by the prescan and prepends YAML frontmatter to the
//! scanner's output (matching Tier-2 behaviour byte-for-byte).

// All submodules are declared `pub` here.  The `tier1` module itself lives
// inside `pub(crate) mod converter`, so the effective visibility is already
// crate-internal; `pub(crate)` would be redundant and triggers the
// `clippy::redundant_pub_crate` lint.  The `tier1` module is only re-exported
// from `lib.rs` under `#[cfg(any(test, feature = "testkit"))]`, so these
// submodules remain invisible outside the crate in normal builds.
pub mod bail;
pub mod parse;
pub mod router;
pub mod scanner;
pub mod spec_rules;
pub mod state;
pub mod tags;

// `lookup` is called by scanner.rs as `tier1::lookup(...)`.
pub use tags::lookup;

// `BailReason` re-export for testkit/bench consumers who pattern-match on it.
// Not needed by production code (convert_api.rs discards the bail value), so
// gate it to avoid widening the non-testkit API surface.
#[cfg(any(test, feature = "testkit"))]
pub use bail::BailReason;

// Convenience re-exports for testkit consumers that import via
// `html_to_markdown_rs::tier1::{ListKind, TagKind, …}` rather than the full
// module path.
#[cfg(any(test, feature = "testkit"))]
pub use tags::{ListKind, OptionalCloseRule, RawKind, TagKind, TagSpec};

// `RouterDecision` is compared in production code (convert_api.rs line 82)
// via the path `tier1::RouterDecision::Tier1`, so this re-export is ungated.
pub use router::RouterDecision;

use crate::converter::prescan::PrescanReport;
use crate::options::ConversionOptions;

/// Attempt a Tier-1 conversion.
///
/// Returns the complete output string (optional YAML frontmatter followed by
/// the markdown body) on success, or `Err(BailReason::*)` when the scanner
/// encounters a construct it cannot handle.  The dispatcher falls back to
/// Tier-2 transparently.
///
/// # Errors
///
/// Returns `Err(BailReason::*)` when the scanner encounters a construct it
/// cannot handle.  The dispatcher falls back to Tier-2 transparently.
pub fn run(html: &str, report: &PrescanReport, options: &ConversionOptions) -> Result<String, bail::BailReason> {
    let body = scanner::scan(html, options)?;

    // Prepend YAML frontmatter when metadata extraction is requested.
    // `head_metadata::extract_frontmatter` re-parses only the head slice (cheap).
    if let Some(frontmatter) = crate::converter::head_metadata::extract_frontmatter(html, report, options) {
        let mut output = String::with_capacity(frontmatter.len() + body.len());
        output.push_str(&frontmatter);
        output.push_str(&body);
        return Ok(output);
    }

    Ok(body)
}
