//! Tier-1: single-pass byte-scanner conversion path.
//!
//! `run` delegates to the byte scanner in `scanner.rs`.  When the scanner
//! encounters a construct it cannot handle, it returns `Err(BailReason::*)`
//! and the dispatcher in `lib.rs::convert` falls back to the Tier-2 path.
//!
//! When `options.extract_metadata` is true, `run` additionally re-parses the
//! head slice captured by the prescan and prepends YAML frontmatter to the
//! scanner's output (matching Tier-2 behaviour byte-for-byte).

pub mod bail;
pub mod metadata;
pub mod parse;
pub mod router;
pub mod scanner;
pub mod spec_rules;
pub mod state;
pub mod tags;

// These re-exports are used by testkit/bench consumers; they look unused in
// normal builds where `tier1` itself is not re-exported from the crate root.
#[allow(unused_imports)]
pub use tags::{ListKind, OptionalCloseRule, RawKind, TagKind, TagSpec, lookup};

#[allow(unused_imports)]
pub use bail::BailReason;
#[allow(unused_imports)]
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
pub fn run(html: &str, report: &PrescanReport, options: &ConversionOptions) -> Result<String, BailReason> {
    let body = scanner::scan(html, report, options)?;

    // Prepend YAML frontmatter when metadata extraction is requested.
    // `metadata::extract_frontmatter` re-parses only the head slice (cheap).
    if let Some(frontmatter) = metadata::extract_frontmatter(html, report, options) {
        let mut output = String::with_capacity(frontmatter.len() + body.len());
        output.push_str(&frontmatter);
        output.push_str(&body);
        return Ok(output);
    }

    Ok(body)
}
