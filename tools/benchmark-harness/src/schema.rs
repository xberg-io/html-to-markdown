//! JSON schema types for benchmark results, baselines, and guardrails.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Schema version for newly captured result files and calibrated guardrails.
pub const SCHEMA_VERSION: u32 = 2;
/// Number of independently timed samples retained for every fixture run.
pub const SAMPLES_PER_RUN: usize = 9;
/// Number of full-corpus runs required to approve calibration data.
pub const CALIBRATION_RUNS: usize = 40;

/// Hardware, toolchain, build, and runner identity for a benchmark capture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provenance {
    /// Operating-system family reported by Rust.
    pub os: String,
    /// CPU architecture reported by Rust.
    pub arch: String,
    /// Processor model reported by the operating system.
    pub cpu_model: String,
    /// Logical processor count available to the process.
    pub cpu_count: usize,
    /// Full `rustc -Vv` output.
    pub rustc_verbose: String,
    /// Rust target host parsed from `rustc -Vv`.
    pub rustc_host: String,
    /// `cargo -V` output.
    pub cargo_version: String,
    /// Cargo profile used to build the harness.
    pub profile: String,
    /// Rust compiler flags that can affect generated code.
    pub build_flags: String,
    /// Timing/statistic contract used by this capture.
    pub measurement_mode: String,
    /// Conversion tier selection used for every measured call.
    pub tier_strategy: String,
    /// Visitor configuration used for every measured call.
    pub visitor_mode: String,
    /// Fixed iteration count, or `None` when each fixture was calibrated.
    pub iteration_override: Option<u32>,
    /// Warmup calls made before measuring each fixture.
    pub warmup_iterations: u32,
    /// Target duration used to calibrate an automatic iteration count.
    pub calibration_target_ms: u64,
    /// Maximum time allowed for automatic iteration calibration.
    pub calibration_timeout_ms: u64,
    /// Enabled html-to-markdown core features, sorted for stable comparison.
    pub core_features: Vec<String>,
    /// GitHub runner image family, when the runner exposes it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner_image: Option<String>,
    /// Runner class, such as `github-hosted`, when exposed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner_class: Option<String>,
}

impl Provenance {
    /// Compare every field that defines the measurement *contract*, ignoring host identity.
    ///
    /// The contract is everything a capture and its calibrated baseline must share for their
    /// numbers to be comparable at all: toolchain, profile, build flags, enabled core features,
    /// measurement mode, tier and visitor selection, iteration/warmup settings, and runner class.
    /// A difference in any of them is a configuration error and must hard-fail.
    ///
    /// `cpu_model` and `cpu_count` are deliberately excluded — see [`Provenance::host_matches`].
    #[must_use]
    pub fn contract_matches(&self, other: &Self) -> bool {
        // ~keep Destructured field by field on purpose: adding a field to `Provenance` breaks
        // this function to compile, forcing an explicit contract-vs-host-identity decision
        // instead of silently inheriting whichever side of the split the author never considered.
        let Self {
            os,
            arch,
            cpu_model: _,
            cpu_count: _,
            rustc_verbose,
            rustc_host,
            cargo_version,
            profile,
            build_flags,
            measurement_mode,
            tier_strategy,
            visitor_mode,
            iteration_override,
            warmup_iterations,
            calibration_target_ms,
            calibration_timeout_ms,
            core_features,
            runner_image,
            runner_class,
        } = self;
        *os == other.os
            && *arch == other.arch
            && *rustc_verbose == other.rustc_verbose
            && *rustc_host == other.rustc_host
            && *cargo_version == other.cargo_version
            && *profile == other.profile
            && normalized_build_flags(build_flags) == normalized_build_flags(&other.build_flags)
            && *measurement_mode == other.measurement_mode
            && *tier_strategy == other.tier_strategy
            && *visitor_mode == other.visitor_mode
            && *iteration_override == other.iteration_override
            && *warmup_iterations == other.warmup_iterations
            && *calibration_target_ms == other.calibration_target_ms
            && *calibration_timeout_ms == other.calibration_timeout_ms
            && *core_features == other.core_features
            && *runner_image == other.runner_image
            && *runner_class == other.runner_class
    }

    /// Compare the host-identity fields that are excluded from the measurement contract.
    ///
    /// `ubuntu-24.04` is one label over a heterogeneous pool: the same workflow draws AMD EPYC
    /// hosts on one run and Intel Xeon hosts on the next, and the calibration campaign
    /// (`workflow_dispatch`, 360-minute budget) can only ever be captured on whichever host it
    /// happened to draw. Treating the CPU as part of the contract therefore turned an unavoidable
    /// pool difference into a coin-flip hard failure that aborted before a single timing was
    /// evaluated. Host identity still gates *how* timing violations are reported — matched
    /// hardware means violations are real and fatal, mismatched hardware means they may be
    /// hardware noise and can be downgraded to advisory by an explicit opt-in.
    #[must_use]
    pub fn host_matches(&self, other: &Self) -> bool {
        self.cpu_model == other.cpu_model && self.cpu_count == other.cpu_count
    }
}

/// Remove only adjacent repetitions of identical two-token Rust lint settings.
/// Their repetition is idempotent; other flags and lint-setting order remain significant.
fn normalized_build_flags(flags: &str) -> Vec<&str> {
    let mut normalized = Vec::new();
    let mut tokens = flags.split_whitespace().peekable();
    while let Some(flag) = tokens.next() {
        if matches!(flag, "-A" | "-W" | "-D" | "-F")
            && let Some(&lint) = tokens.peek()
        {
            tokens.next();
            if !normalized.ends_with(&[flag, lint]) {
                normalized.extend([flag, lint]);
            }
        } else {
            normalized.push(flag);
        }
    }
    normalized
}

/// Result of a single fixture benchmark run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchRecord {
    /// Relative path of the fixture file (from the fixtures root).
    pub fixture: String,
    /// Group tag from `groups.toml`.
    pub group: String,
    /// Input HTML size in bytes.
    pub bytes: u64,
    /// All independently timed per-call samples in milliseconds.
    pub samples_ms: Vec<f64>,
    /// Median of `samples_ms`.
    pub median_ms: f64,
    /// Median absolute deviation of `samples_ms`.
    pub mad_ms: f64,
    /// Best of the first three batches for comparison with schema-v1 baselines.
    pub legacy_ms_best: f64,
    /// Throughput in MB/s derived from `bytes / median_ms`.
    pub mb_per_s: f64,
    /// Output Markdown byte length.
    pub output_bytes: u64,
}

/// Top-level JSON document written by `htmbench run`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunResults {
    /// Schema version; always [`SCHEMA_VERSION`].
    pub schema: u32,
    /// Full git SHA of the HEAD commit at capture time.
    pub sha: String,
    /// Hostname, retained for diagnostics but excluded from compatibility checks.
    pub hostname: String,
    /// ISO-8601 timestamp of when the run completed.
    pub created_at: String,
    /// Structured measurement provenance.
    pub provenance: Provenance,
    /// Individual fixture measurements.
    pub runs: Vec<BenchRecord>,
}

/// Per-group regression threshold.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupThreshold {
    /// Maximum allowed regression percentage.
    pub max_regression_pct: f64,
}

/// Fixture-scale variability derived from a calibration campaign.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureFloor {
    /// Full-corpus samples used in calibration.
    pub sample_count: usize,
    /// Median of per-run medians.
    pub median_ms: f64,
    /// MAD of per-run medians.
    pub mad_ms: f64,
    /// Nearest-rank p95 of absolute adjacent-pair deltas.
    pub floor_ms: f64,
}

/// Fixture record stored in a calibrated baseline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibratedBenchRecord {
    /// Relative fixture path.
    pub fixture: String,
    /// Fixture group.
    pub group: String,
    /// Input size.
    pub bytes: u64,
    /// Median across forty run medians.
    pub median_ms: f64,
    /// MAD across forty run medians.
    pub mad_ms: f64,
    /// Throughput derived from the calibrated median.
    pub mb_per_s: f64,
    /// Output Markdown size.
    pub output_bytes: u64,
}

/// Schema-v2 calibrated baseline, distinct from a nine-sample run capture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibratedBaseline {
    /// Schema version.
    pub schema: u32,
    /// Shared identifier tying this baseline to its guardrails.
    pub campaign_id: String,
    /// Full calibration commit SHA.
    pub sha: String,
    /// Informational capture hostname.
    pub hostname: String,
    /// Timestamp of the final campaign capture.
    pub created_at: String,
    /// Approved calibration match key.
    pub provenance: Provenance,
    /// Calibrated fixture summaries.
    pub runs: Vec<CalibratedBenchRecord>,
}

/// Top-level schema-v2 `guardrails.json` document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guardrails {
    /// Schema version; always [`SCHEMA_VERSION`].
    pub schema: u32,
    /// Shared identifier tying these floors to the calibrated baseline.
    pub campaign_id: String,
    /// Per-group policy thresholds.
    pub thresholds: HashMap<String, GroupThreshold>,
    /// Approved calibration match key.
    pub calibration_provenance: Provenance,
    /// Fixture-keyed measured noise floors.
    pub fixture_floors: HashMap<String, FixtureFloor>,
}

/// Legacy schema-v1 result record used only by the checked-in migration bridge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyBenchRecord {
    /// Relative fixture path.
    pub fixture: String,
    /// Fixture group.
    pub group: String,
    /// Input size.
    pub bytes: u64,
    /// Historic best-of-three timing.
    pub ms_best: f64,
    /// Historic throughput.
    pub mb_per_s: f64,
    /// Output size.
    pub output_bytes: u64,
}

/// Legacy schema-v1 results used only until the first approved calibration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyRunResults {
    /// Schema version.
    pub schema: u32,
    /// Captured commit.
    pub sha: String,
    /// Historic hostname field.
    pub host: String,
    /// Capture timestamp.
    pub created_at: String,
    /// Fixture records.
    pub runs: Vec<LegacyBenchRecord>,
}

/// Legacy schema-v1 guardrails used only until the first approved calibration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyGuardrails {
    /// Schema version.
    pub schema: u32,
    /// Per-group policy thresholds.
    pub thresholds: HashMap<String, GroupThreshold>,
}

/// Canonical threshold policy. These values are intentionally unchanged from schema v1.
pub fn default_thresholds() -> HashMap<String, GroupThreshold> {
    [
        ("clean_small", 10.0),
        ("clean_medium", 8.0),
        ("clean_large", 5.0),
        ("spec_rules", 10.0),
        ("fallthrough_custom_elements", 10.0),
        ("fallthrough_bare_lt", 10.0),
        ("adversarial", 30.0),
    ]
    .into_iter()
    .map(|(group, max_regression_pct)| (group.to_owned(), GroupThreshold { max_regression_pct }))
    .collect()
}
