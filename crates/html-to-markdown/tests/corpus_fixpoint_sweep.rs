// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![allow(clippy::print_stdout, clippy::print_stderr, clippy::dbg_macro)] // ~keep: tests print by design

//! Conversion-fixpoint oracle, swept across the real-world document corpus.
//!
//! `commonmark_spec_fixpoint.rs` runs this same oracle -- convert, render back to HTML with
//! `comrak`, convert again, compare -- over the 652 `CommonMark` spec examples. That corpus is
//! clean-room synthetic Markdown-shaped HTML; it cannot exercise browser tag-soup recovery,
//! real generator markup, or the option combinations real documents actually hit.
//! `roundtrip_fixpoint.rs` already runs the same oracle over `tools/benchmark-harness/fixtures`
//! (29 files) with an exhaustive, per-file `KNOWN_DIVERGENCES` allow-list. This file adds the
//! 70-document, 23-generator-family corpus at `test_documents/html/` to the sweep (99 files
//! combined with the benchmark fixtures) using a different technique on purpose: a *ratchet*
//! (a floor on how many documents must be stable, per `commonmark_spec_fixpoint.rs`'s own
//! `MIN_STABLE_ESCAPED` pattern) rather than an exhaustive per-file allow-list. A 70-document,
//! organically-growing corpus is exactly the case an exhaustive allow-list handles badly: every
//! new fixture would need a manual entry-or-pass decision before it could be added at all.
//!
//! ## Three populations, not one pass/fail count
//!
//! A prior, uncommitted run of this same oracle over a larger corpus (this one, plus the
//! benchmark fixtures, plus a since-discarded generated corpus) found that a raw stable/
//! unstable split conflates three populations that need different handling:
//!
//! 1. **Genuinely stable**: `md1 == md2`. The converter's choices survive an HTML round trip.
//! 2. **Stable after one extra pass**: `md1 != md2` but `md2 == md3`. The same class as spec
//!    examples 301/302 (documented on `commonmark_spec_fixpoint::MIN_STABLE_ESCAPED`): two
//!    adjacent blocks that use the same marker (two lists with the same bullet, two block
//!    quotes) have no separator in Markdown that survives a compliant reparse, so they merge
//!    into one block on the first render. That merge is itself stable once it has happened.
//!    Forcing a one-cycle fixpoint here would mean inserting a separator real documents never
//!    asked for, corrupting the overwhelmingly more common case where no merge was intended.
//! 3. **Harness artifacts**: inputs where instability is *this test's* blind spot, not the
//!    converter's. The dominant instance found previously -- 567 documents in that larger,
//!    since-discarded run -- was this crate's own `HighlightStyle::DoubleEqual` output for
//!    `<mark>`: `==text==` is Pandoc-compatible, not `CommonMark`, so a compliant renderer
//!    (`comrak` included) has no notion of it and passes it through as inert literal text.
//!    That is fine on its own -- literal `==text==` federates back to literal `==text==` -- but
//!    it collides with `escape_misc`, which must defensively escape a leading `=` run as
//!    *plain text* (a bare `=` sequence can be mistaken for a setext-heading underline) with no
//!    way to tell that the same bytes were emitted deliberately, as syntax, one pass earlier.
//!    Measured directly below in `mark_highlight_syntax_is_a_known_non_commonmark_artifact`.
//!    This corpus has exactly one `<mark>`-bearing document, and it happens not to trigger the
//!    collision in its current shape, but the detection stays in the sweep on purpose: a corpus
//!    that keeps growing (as this one is designed to) will eventually add one that does, and
//!    silently mis-filing that document as a new converter bug would be worse than a few lines
//!    of bookkeeping that currently subtract zero from the stable count.
//!
//! Populations 2 and 3 are excluded from the floor in [`MIN_STABLE_CORPUS`] on purpose --
//! neither is a converter defect -- but neither is silently discarded either: both are counted
//! and printed, and population 3 is verified non-empty-*capable* by its own unit test so a
//! regression in the detection itself cannot go unnoticed.
//!
//! ## What is left over: genuinely unexplained instability
//!
//! Whatever remains after removing populations 2 and 3 is capped by [`MAX_UNEXPLAINED`], a
//! ratchet in the other direction (this number must not grow). Every document in that bucket,
//! measured against this corpus today, was root-caused by minimizing the actual diff (see the
//! module comment on `KNOWN_DIVERGENCES` in `roundtrip_fixpoint.rs` for the full writeup of each
//! bucket letter) rather than assumed from a shared symptom:
//!
//! - `issues/gh-190/kimbrain.html` (here and in `tools/benchmark-harness/fixtures`, a literal
//!   duplicate): Bucket D, `known_issue_unescaped_angle_bracket_text_becomes_a_phantom_tag` --
//!   literal `<word>`-shaped text is unescaped by default and is consumed as a real tag on
//!   reparse.
//! - `issues/gh-190/ozonekorea.html` (ditto, duplicated): Bucket D,
//!   `known_issue_unescaped_lone_tilde_pairs_into_phantom_strikethrough`.
//! - `wikipedia/small_html.html` (ditto, duplicated): the table-row-count-shrinks cause already
//!   allow-listed for `real-world/wikipedia/small_html.html` in `roundtrip_fixpoint.rs`.
//! - `mdream/mdn-array.html` is only present via the benchmark-fixtures half of this sweep, and
//!   is the "soft-break whitespace normalisation, plus a nesting-depth-driven bullet-cycle
//!   rotation" cause already allow-listed there.
//! - `stackoverflow/regex-html-parsing.html` is new to this corpus, but not to the underlying
//!   cause: minimizing its diff shows the same Bucket D phantom-tag mechanism (a literal,
//!   unescaped `<center>` in body text is consumed as a real tag on reparse) plus the
//!   `<hr>`-adjacent-to-text setext collision that `hr_adjacent_to_text_is_now_a_rule` pins as
//!   fixed; this document's output did not change with that fix. Not a new bug class.
//!
//! No document in this corpus surfaced a bug outside those two already-tracked buckets.
//!
//! ## Runtime
//!
//! Measured at ~5s for 99 files x one conversion pass (plus a second/third pass only for the
//! ~45% of files that do not stabilize on the first pass), on par with `roundtrip_fixpoint.rs`
//! (~4.3s for 29 files x 2 newline-style variants) and well under `corpus_robustness.rs`
//! (~13s). Kept in the default suite rather than behind `#[ignore]`: at that cost relative to
//! tests already run unconditionally today, gating it behind a flag nobody passes would cost
//! more coverage than the runtime it would save.

mod support;

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use html_to_markdown_rs::{ConversionOptions, convert};

// ---------------------------------------------------------------------------
// Corpus plumbing
// ---------------------------------------------------------------------------

/// `tools/benchmark-harness/fixtures`, resolved the same way `roundtrip_fixpoint.rs` does.
///
/// ~keep Not reused from that file: integration tests are separate compiled crates, and
/// ~keep duplicating this ~3-line path helper is cheaper than introducing a cross-file
/// ~keep dependency between two independently-evolving oracles.
fn benchmark_fixtures_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tools/benchmark-harness/fixtures")
}

/// Every HTML file in the combined corpus: `test_documents/html/` (all 70 files, walked
/// directly rather than filtered through `MANIFEST.toml`, since `wikipedia/`, `issues/`, and
/// `visitor/` predate the manifest and are out of its scope -- see `manifest_consistency_test`)
/// plus `tools/benchmark-harness/fixtures/` (29 files). Sorted for deterministic iteration.
fn all_corpus_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    support::collect_html_files(&support::corpus_root(), &mut files);
    support::collect_html_files(&benchmark_fixtures_root(), &mut files);
    files.sort();
    files
}

fn render_markdown_to_html(md: &str) -> String {
    let mut options = comrak::Options::default();
    // ~keep Same rationale as `commonmark_spec_fixpoint.rs` and `roundtrip_fixpoint.rs`: GFM
    // ~keep tables/strikethrough are enabled because this crate emits them; GFM autolinking is
    // ~keep deliberately NOT enabled, since it would manufacture links this crate never asked
    // ~keep for and produce phantom instability that is the renderer's choice, not ours.
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.render.r#unsafe = true;
    comrak::markdown_to_html(md, &options)
}

/// The option set this sweep runs under.
///
/// ~keep Deliberately the weaker, non-escaping configuration `roundtrip_fixpoint.rs` uses for
/// ~keep real-world corpora (escaping is reserved for `commonmark_spec_fixpoint.rs`'s clean
/// ~keep synthetic examples, "the configuration in which round-trip stability is a real
/// ~keep contract"). Two independent reasons real documents specifically should NOT enable
/// ~keep escaping here, both measured directly rather than assumed:
/// ~keep
/// ~keep 1. `extract_metadata` produces a `key: value` frontmatter block with no HTML
/// ~keep    representation to round-trip through; `comrak` parses its closing `---` as a
/// ~keep    setext-heading underline for the lines above it every time, regardless of escaping.
/// ~keep 2. Enabling `escape_misc`/`escape_asterisks`/`escape_underscores` over this specific
/// ~keep    corpus's pathologically mis-nested tag-soup fixture
/// ~keep    (`legacy/broken-tag-soup-unclosed.html`, deliberately synthesized to stress
/// ~keep    adoption-agency recovery) produces a run of colliding emphasis delimiters at an
/// ~keep    unclosed-`<a>`/`<div>` boundary whose escaped-vs-literal state does not settle
/// ~keep    within 4 additional render/convert cycles. It is cosmetic asterisk-count noise, not
/// ~keep    content loss (no word is dropped), but chasing it is exactly the escaping
/// ~keep    interaction this corpus should not need to characterise to do its job.
fn corpus_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        ..Default::default()
    }
}

fn to_markdown(html: &str, options: &ConversionOptions) -> Option<String> {
    convert(html, Some(options.clone())).ok()?.content
}

fn relative_path(path: &Path) -> String {
    for root in [support::corpus_root(), benchmark_fixtures_root()] {
        if let Ok(rel) = path.strip_prefix(&root) {
            return rel.to_str().expect("path is not valid UTF-8").replace('\\', "/");
        }
    }
    panic!("{} is outside both corpus roots", path.display());
}

/// Whether the document *can* emit `==...==` highlight syntax at all, which under the default
/// `HighlightStyle` (unchanged by [`corpus_options`]) is determined entirely by whether
/// `<mark>` appears in the input.
///
/// ~keep Necessary but deliberately NOT sufficient to file a document as a harness artifact:
/// ~keep see `is_mark_escaping_artifact`.
fn contains_mark_tag(html: &str) -> bool {
    html.to_ascii_lowercase().contains("<mark")
}

/// Whether an unstable document's instability is *entirely* the `<mark>` escaping collision
/// described in the module doc comment's "harness artifacts" population.
///
/// ~keep Tag presence alone must not exempt a document from scrutiny. `<mark>` is common in
/// ~keep real search-result and documentation HTML, so a presence-only check would silently
/// ~keep file any future `<mark>`-bearing document as an artifact no matter what its actual
/// ~keep instability was -- turning the one bucket that is supposed to absorb a known blind
/// ~keep spot into a place real converter defects go to hide, on exactly the growing corpus
/// ~keep this sweep exists to watch.
///
/// ~keep So require the mechanism, not the marker: the collision is precisely that the `==`
/// ~keep delimiters this crate emitted as syntax come back as `\=\=` once a compliant
/// ~keep renderer has passed them through as inert literal text (measured exactly, in
/// ~keep `mark_highlight_syntax_is_a_known_non_commonmark_artifact`). Normalising that one
/// ~keep substitution away must therefore account for the WHOLE diff. If anything else also
/// ~keep differs, the document is not an artifact and belongs in `unexplained`, where the
/// ~keep ceiling will make someone look at it.
fn is_mark_escaping_artifact(html: &str, md1: &str, md2: &str) -> bool {
    contains_mark_tag(html) && md1.contains("==") && md1 == md2.replace("\\=", "=")
}

// ---------------------------------------------------------------------------
// Ratchets
// ---------------------------------------------------------------------------

/// Floor for documents that reach a fixpoint on the very first render/convert cycle.
///
/// ~keep Measured value: 55 of 99. Raise it as more documents are fixed into this bucket; a
/// ~keep drop is a regression. Documents that stabilize on a later cycle, or that are excluded
/// ~keep as `<mark>`-syntax artifacts, do not count against this floor -- see the module doc
/// ~keep comment.
const MIN_STABLE_CORPUS: usize = 55;

/// Ceiling on documents whose instability is neither a one-extra-pass stabilisation nor a
/// `<mark>`-syntax artifact.
///
/// ~keep Measured value: 8 of 99, all individually root-caused in the module doc comment as
/// ~keep instances of two already-tracked bucket letters (`roundtrip_fixpoint.rs`'s Bucket A
/// ~keep and Bucket D), not a new defect class. This is a ratchet the other direction from
/// ~keep `MIN_STABLE_CORPUS`: it must not grow. Lower it if a document is fixed out of this
/// ~keep bucket; raising it requires root-causing the new document first, the same bar applied
/// ~keep to the 8 already here.
const MAX_UNEXPLAINED: usize = 8;

#[test]
fn corpus_is_not_empty() {
    // ~keep An empty corpus would make every assertion below vacuously pass.
    let files = all_corpus_files();
    assert!(
        files.len() >= 90,
        "expected the combined corpus (test_documents/html + benchmark fixtures) to hold at \
         least 90 files, found {}",
        files.len()
    );
}

#[test]
fn conversion_fixpoint_sweep_over_the_combined_corpus() {
    let files = all_corpus_files();
    assert!(!files.is_empty(), "corpus must not be empty");

    let options = corpus_options();
    let mut stable = 0usize;
    let mut second_pass_stable = 0usize;
    let mut mark_artifact = 0usize;
    let mut unexplained: Vec<String> = Vec::new();
    let mut conversion_failed: Vec<String> = Vec::new();

    for path in &files {
        let relative = relative_path(path);
        let html = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {relative}: {e}"));

        let Some(md1) = to_markdown(&html, &options) else {
            conversion_failed.push(relative);
            continue;
        };
        let Some(md2) = to_markdown(&render_markdown_to_html(&md1), &options) else {
            conversion_failed.push(relative);
            continue;
        };

        if md1 == md2 {
            stable += 1;
            continue;
        }

        // ~keep Checked before the second-pass-stability probe: a `<mark>`-syntax collision is
        // ~keep an explained artifact regardless of whether it happens to also settle on a
        // ~keep later pass, so it must not silently masquerade as population 2.
        if is_mark_escaping_artifact(&html, &md1, &md2) {
            mark_artifact += 1;
            continue;
        }

        let md3 = to_markdown(&render_markdown_to_html(&md2), &options);
        if md3.as_deref() == Some(md2.as_str()) {
            second_pass_stable += 1;
        } else {
            unexplained.push(relative);
        }
    }

    println!("=== corpus conversion fixpoint sweep ===");
    println!("files:               {}", files.len());
    println!("stable (1st pass):   {stable}");
    println!("stable (2nd pass):   {second_pass_stable}");
    println!("mark-syntax artifact:{mark_artifact}");
    println!("unexplained:         {} {:?}", unexplained.len(), unexplained);
    println!(
        "conversion failed:   {} {:?}",
        conversion_failed.len(),
        conversion_failed
    );

    assert!(
        conversion_failed.is_empty(),
        "every corpus document must convert successfully at least once: {conversion_failed:?}"
    );
    assert_eq!(
        stable + second_pass_stable + mark_artifact + unexplained.len(),
        files.len(),
        "every file must land in exactly one bucket"
    );
    assert!(
        stable >= MIN_STABLE_CORPUS,
        "only {stable} of {} documents reached a fixpoint on the first pass (floor \
         {MIN_STABLE_CORPUS}); see the per-bucket breakdown above",
        files.len()
    );
    assert!(
        unexplained.len() <= MAX_UNEXPLAINED,
        "unexplained instability grew to {} documents (ceiling {MAX_UNEXPLAINED}): {unexplained:?} -- \
         root-cause each new one (see the module doc comment for the format) before raising this ceiling",
        unexplained.len()
    );
}

/// Directly measures the mechanism the module doc comment attributes population 3 to: a
/// document whose only content is a `<mark>` span, converted with `escape_misc` enabled,
/// diverges on the very next pass because the literal `==...==` text it produced is
/// indistinguishable, one pass later, from ordinary text that merely looks like the same
/// syntax -- and `escape_misc` must defensively escape the latter.
///
/// ~keep This uses escaping deliberately, unlike [`corpus_options`]: the collision this test
/// ~keep documents does not reproduce under the sweep's own non-escaping configuration (see
/// ~keep `corpus_options`'s doc comment), so proving the mechanism exists at all requires the
/// ~keep configuration that triggers it.
#[test]
fn mark_highlight_syntax_is_a_known_non_commonmark_artifact() {
    let options = ConversionOptions {
        escape_misc: true,
        escape_asterisks: true,
        escape_underscores: true,
        ..Default::default()
    };
    let html = "<p><mark>whole paragraph text</mark></p>";
    let md1 = to_markdown(html, &options).expect("conversion must succeed");
    assert_eq!(md1, "==whole paragraph text==\n");

    let md2 = to_markdown(&render_markdown_to_html(&md1), &options).expect("conversion must succeed");
    assert_ne!(
        md1, md2,
        "expected the documented collision (defensive escaping of a now-literal '==' run) to \
         reproduce; if this now passes, `<mark>` no longer needs special handling in the sweep \
         above and `contains_mark_tag` can be retired"
    );
    assert_eq!(
        md2, "\\=\\=whole paragraph text\\=\\=\n",
        "expected the '==' delimiters to be defensively backslash-escaped on the second pass"
    );
}

/// The combined corpus's one `<mark>`-bearing document does not itself trigger the collision
/// [`mark_highlight_syntax_is_a_known_non_commonmark_artifact`] documents (it is not a whole
/// escaped block in the shape that provokes it), so it is not visible in
/// [`conversion_fixpoint_sweep_over_the_combined_corpus`]'s `mark_artifact` count today. This
/// pins that the file exists and still contains `<mark>`, so the sweep's detection is exercised
/// (not merely present but untested) the day a shape that does trigger it is added.
#[test]
fn corpus_contains_a_mark_bearing_document_for_the_artifact_detector_to_watch() {
    let path = support::corpus_root().join("github-markdown/readme-quick-start-excerpt.html");
    let html = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    assert!(
        contains_mark_tag(&html),
        "expected {} to contain a <mark> tag; if it no longer does, replace it with another \
         corpus fixture that does so `contains_mark_tag` stays exercised",
        path.display()
    );
}

/// The tightened artifact classifier must reject a `<mark>`-bearing document whose instability
/// is something other than the `==` escaping collision.
///
/// ~keep Guards the exemption itself. A presence-only check (`contains_mark_tag` alone, which
/// ~keep is what this originally used) passes this input straight into the artifact bucket and
/// ~keep out of `MAX_UNEXPLAINED`'s reach, hiding a real defect behind an unrelated tag that
/// ~keep merely happens to appear in the same document.
#[test]
fn mark_artifact_classifier_rejects_unrelated_instability() {
    let html = "<p><mark>hi</mark></p>";
    let md1 = "==hi==\n";
    assert!(
        is_mark_escaping_artifact(html, md1, "\\=\\=hi\\=\\=\n"),
        "the pure escaping collision must still classify as an artifact"
    );
    assert!(
        !is_mark_escaping_artifact(html, md1, "\\=\\=hi\\=\\=\n\nan extra block appeared\n"),
        "instability beyond the '==' escaping substitution must NOT be excused as an artifact"
    );
    assert!(
        !is_mark_escaping_artifact("<p>no highlight here</p>", "plain\n", "different\n"),
        "a document with no <mark> at all can never be this artifact"
    );
}

#[test]
fn manifest_and_benchmark_roots_do_not_overlap() {
    // ~keep A path resolving under both roots would silently double-count a file in the sweep
    // ~keep above and make `relative_path` return the wrong corpus's relative form for it.
    let corpus_root = support::corpus_root();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for path in all_corpus_files() {
        let relative = relative_path(&path);
        let prefix = if path.starts_with(&corpus_root) {
            "corpus:"
        } else {
            "bench:"
        };
        let key = format!("{prefix}{relative}");
        assert!(seen.insert(key), "duplicate path resolved for {}", path.display());
    }
}
