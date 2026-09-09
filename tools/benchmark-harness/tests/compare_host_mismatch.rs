//! End-to-end coverage for `htmbench compare` host-identity handling.
//!
//! The provenance contract (toolchain, profile, build flags, features, measurement settings,
//! runner class) must hard-fail on any drift, while the host identity fields (`cpu_model`,
//! `cpu_count`) must not: GitHub's `ubuntu-24.04` label spans AMD and Intel hosts, so a capture
//! routinely runs on a CPU the calibration campaign never drew. These tests pin both halves plus
//! the `--allow-host-mismatch` opt-in that turns timing violations advisory on foreign hardware
//! and — critically — leaves them fatal on the calibrated hardware.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use html_to_markdown_bench::bench;
use html_to_markdown_bench::schema::{
    BenchRecord, CalibratedBaseline, CalibratedBenchRecord, FixtureFloor, Guardrails, Provenance, RunResults,
    SCHEMA_VERSION, default_thresholds,
};

/// Baseline median in milliseconds; with a 10% `clean_small` threshold and a 0.10 ms floor the
/// effective allowance is 0.10 ms.
const BASELINE_MS: f64 = 1.0;
/// Sample value that lands inside the allowance.
const WITHIN_ALLOWANCE_MS: f64 = 1.08;
/// Sample value that exceeds the allowance and is therefore a timing violation.
const BEYOND_ALLOWANCE_MS: f64 = 1.11;
/// Floor derived from the calibration campaign for the single fixture used here.
const FLOOR_MS: f64 = 0.10;

#[test]
fn should_pass_host_only_mismatch_as_advisory_when_opted_in() {
    let case = Case::new("advisory-under-flag");
    let output = case.run(foreign_host(), BEYOND_ALLOWANCE_MS, &["--allow-host-mismatch"]);
    let stderr = stderr_of(&output);
    assert!(output.status.success(), "expected success, stderr:\n{stderr}");
    assert!(stderr.contains("ADVISORY:"), "missing advisory notice:\n{stderr}");
    assert!(
        stderr.contains("host CPU differs from the calibrated baseline host"),
        "missing host warning:\n{stderr}"
    );
    assert!(
        !stderr.contains("benchmark provenance mismatch"),
        "host identity must not abort the contract check:\n{stderr}"
    );
}

#[test]
fn should_fail_host_only_mismatch_without_the_opt_in() {
    let case = Case::new("fatal-without-flag");
    let output = case.run(foreign_host(), BEYOND_ALLOWANCE_MS, &[]);
    let stderr = stderr_of(&output);
    assert!(!output.status.success(), "expected failure, stderr:\n{stderr}");
    assert!(
        stderr.contains("1 guardrail(s) violated"),
        "unexpected failure:\n{stderr}"
    );
    assert!(
        !stderr.contains("ADVISORY:"),
        "violations must stay fatal without the flag:\n{stderr}"
    );
}

#[test]
fn should_pass_host_only_mismatch_with_no_timing_violation() {
    let case = Case::new("clean-foreign-host");
    let output = case.run(foreign_host(), WITHIN_ALLOWANCE_MS, &[]);
    let stderr = stderr_of(&output);
    assert!(output.status.success(), "expected success, stderr:\n{stderr}");
    assert!(
        stderr.contains("host CPU differs from the calibrated baseline host"),
        "missing host warning:\n{stderr}"
    );
}

#[test]
fn should_fail_non_host_provenance_drift_with_and_without_the_opt_in() {
    for (name, flags) in [
        ("drift-without-flag", &[][..]),
        ("drift-with-flag", &["--allow-host-mismatch"][..]),
    ] {
        let case = Case::new(name);
        let output = case.run(drifted_toolchain(), WITHIN_ALLOWANCE_MS, flags);
        let stderr = stderr_of(&output);
        assert!(!output.status.success(), "{name}: expected failure, stderr:\n{stderr}");
        assert!(
            stderr.contains("benchmark provenance mismatch"),
            "{name}: expected a provenance contract failure:\n{stderr}"
        );
        assert!(
            !stderr.contains("ADVISORY:"),
            "{name}: contract drift must never be advisory:\n{stderr}"
        );
    }
}

#[test]
fn should_fail_timing_regression_on_matching_hardware_even_with_the_opt_in() {
    let case = Case::new("matched-host-regression");
    let output = case.run(calibrated_host(), BEYOND_ALLOWANCE_MS, &["--allow-host-mismatch"]);
    let stderr = stderr_of(&output);
    assert!(!output.status.success(), "expected failure, stderr:\n{stderr}");
    assert!(
        stderr.contains("1 guardrail(s) violated"),
        "unexpected failure:\n{stderr}"
    );
    assert!(
        !stderr.contains("ADVISORY:"),
        "a regression on the calibrated CPU must stay fatal:\n{stderr}"
    );
    assert!(
        !stderr.contains("host CPU differs"),
        "hardware matches, so no host warning is expected:\n{stderr}"
    );
}

#[test]
fn should_compare_timings_with_adjacent_duplicate_warning_flags() {
    for (name, sample, expected_success) in [
        ("duplicate-flags-within-threshold", WITHIN_ALLOWANCE_MS, true),
        ("duplicate-flags-regression", BEYOND_ALLOWANCE_MS, false),
    ] {
        let case = Case::new(name);
        let mut provenance = calibrated_host();
        provenance.build_flags = "-D warnings -D warnings".to_owned();
        let output = case.run(provenance, sample, &[]);
        let stderr = stderr_of(&output);
        assert_eq!(output.status.success(), expected_success, "{name}: {stderr}");
        assert!(!stderr.contains("benchmark provenance mismatch"), "{name}: {stderr}");
        if !expected_success {
            assert!(stderr.contains("1 guardrail(s) violated"), "{name}: {stderr}");
        }
    }
}

#[test]
fn should_reject_differing_or_reordered_warning_flags() {
    let mut baseline = calibrated_host();
    baseline.build_flags = "-D warnings -A dead_code".to_owned();
    for flags in [
        "-A warnings -A dead_code",
        "-A dead_code -D warnings",
        "-D warnings -A dead_code -D warnings",
        "-D warnings -A dead_code -C opt-level=2",
    ] {
        let mut results = baseline.clone();
        results.build_flags = flags.to_owned();
        assert!(!results.contract_matches(&baseline), "unexpected equivalence: {flags}");
    }
}

/// One isolated temporary workspace holding a results/baseline/guardrails triple.
struct Case {
    dir: PathBuf,
}

impl Case {
    fn new(name: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("htmbench-compare-{}-{name}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("creating case directory");
        Self { dir }
    }

    fn run(&self, results_provenance: Provenance, sample_ms: f64, flags: &[&str]) -> Output {
        let results = self.dir.join("results.json");
        let baseline = self.dir.join("baseline.json");
        let guardrails = self.dir.join("guardrails.json");
        write_json(&results, &run_results(results_provenance, sample_ms));
        write_json(&baseline, &calibrated_baseline());
        write_json(&guardrails, &guardrails_document());
        Command::new(env!("CARGO_BIN_EXE_htmbench"))
            .args(["compare", "--results"])
            .arg(&results)
            .arg("--baseline")
            .arg(&baseline)
            .arg("--guardrails")
            .arg(&guardrails)
            .args(flags)
            .output()
            .expect("running htmbench compare")
    }
}

impl Drop for Case {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

fn stderr_of(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn write_json<T: serde::Serialize>(path: &Path, value: &T) {
    std::fs::write(path, serde_json::to_vec_pretty(value).expect("serializing document")).expect("writing document");
}

fn run_results(provenance: Provenance, sample_ms: f64) -> RunResults {
    let samples_ms = vec![sample_ms; 9];
    RunResults {
        schema: SCHEMA_VERSION,
        sha: "a".repeat(40),
        hostname: "host".to_owned(),
        created_at: "2026-01-01T00:00:01Z".to_owned(),
        provenance,
        runs: vec![BenchRecord {
            fixture: "fixture.html".to_owned(),
            group: "clean_small".to_owned(),
            bytes: 10,
            median_ms: sample_ms,
            mad_ms: 0.0,
            legacy_ms_best: sample_ms,
            mb_per_s: 1.0,
            output_bytes: 5,
            samples_ms,
        }],
    }
}

fn calibrated_baseline() -> CalibratedBaseline {
    CalibratedBaseline {
        schema: SCHEMA_VERSION,
        campaign_id: "campaign".to_owned(),
        sha: "a".repeat(40),
        hostname: "host".to_owned(),
        created_at: "2026-01-01T00:00:00Z".to_owned(),
        provenance: calibrated_host(),
        runs: vec![CalibratedBenchRecord {
            fixture: "fixture.html".to_owned(),
            group: "clean_small".to_owned(),
            bytes: 10,
            median_ms: BASELINE_MS,
            mad_ms: 0.01,
            mb_per_s: 1.0,
            output_bytes: 5,
        }],
    }
}

fn guardrails_document() -> Guardrails {
    Guardrails {
        schema: SCHEMA_VERSION,
        campaign_id: "campaign".to_owned(),
        thresholds: default_thresholds(),
        calibration_provenance: calibrated_host(),
        fixture_floors: HashMap::from([(
            "fixture.html".to_owned(),
            FixtureFloor {
                sample_count: 40,
                median_ms: BASELINE_MS,
                mad_ms: 0.01,
                floor_ms: FLOOR_MS,
            },
        )]),
    }
}

fn calibrated_host() -> Provenance {
    Provenance {
        os: "linux".to_owned(),
        arch: "x86_64".to_owned(),
        cpu_model: "AMD EPYC 7763 64-Core Processor".to_owned(),
        cpu_count: 4,
        rustc_verbose: "rustc 1.95.0".to_owned(),
        rustc_host: "x86_64-unknown-linux-gnu".to_owned(),
        cargo_version: "cargo 1.95.0".to_owned(),
        profile: "release".to_owned(),
        build_flags: "-D warnings".to_owned(),
        measurement_mode: "nine-batch-median-mad-v2".to_owned(),
        tier_strategy: "auto".to_owned(),
        visitor_mode: "disabled".to_owned(),
        iteration_override: None,
        warmup_iterations: bench::WARMUP_ITERATIONS,
        calibration_target_ms: bench::CALIBRATION_TARGET_MS,
        calibration_timeout_ms: bench::CALIBRATION_TIMEOUT_MS,
        core_features: vec!["metadata".to_owned()],
        runner_image: Some("ubuntu24".to_owned()),
        runner_class: Some("github-hosted".to_owned()),
    }
}

/// The calibrated contract measured on the other CPU vendor in the same runner pool.
fn foreign_host() -> Provenance {
    Provenance {
        cpu_model: "INTEL(R) XEON(R) PLATINUM 8573C".to_owned(),
        cpu_count: 8,
        ..calibrated_host()
    }
}

/// Real configuration drift: same host, different toolchain.
fn drifted_toolchain() -> Provenance {
    Provenance {
        rustc_verbose: "rustc 1.94.0".to_owned(),
        ..calibrated_host()
    }
}
