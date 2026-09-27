//! End-to-end coverage for `htmbench compare` host-identity handling.
//!
//! The provenance contract (toolchain, profile, build flags, features, measurement settings,
//! runner class) must hard-fail on any drift, while the host identity fields (`cpu_model`,
//! `cpu_count`) must not: GitHub's `ubuntu-24.04` label spans AMD and Intel hosts, so a capture
//! routinely runs on a CPU the calibration campaign never drew. These tests pin both halves: on a
//! foreign CPU no timing is scored and `--allow-host-mismatch` only decides whether that unscored
//! run exits successfully, while on the calibrated CPU a regression stays fatal.

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
/// Converted output size the baseline records for the fixture.
const BASELINE_OUTPUT_BYTES: u64 = 5;

/// Header of the GitHub Actions annotation an unscored run prints under `--allow-host-mismatch`.
const UNSCORED_ANNOTATION: &str = "::warning title=Benchmark timings not scored::";

#[test]
fn should_report_timings_unscored_on_a_foreign_host_when_opted_in() {
    for (name, sample) in [
        ("unscored-flag-within", WITHIN_ALLOWANCE_MS),
        ("unscored-flag-beyond", BEYOND_ALLOWANCE_MS),
    ] {
        let case = Case::new(name);
        let output = case.run(foreign_host(), sample, &["--allow-host-mismatch"]);
        let stderr = stderr_of(&output);
        let stdout = stdout_of(&output);
        assert!(output.status.success(), "{name}: expected success, stderr:\n{stderr}");
        assert!(
            stderr.contains("TIMINGS NOT SCORED: this run measured on INTEL(R) XEON(R) PLATINUM 8573C (8 cores)"),
            "{name}: missing not-scored notice:\n{stderr}"
        );
        assert!(
            stdout.contains(UNSCORED_ANNOTATION),
            "{name}: an unscored run must not pass silently:\n{stdout}"
        );
        assert!(
            stdout.contains("fixture.html"),
            "{name}: timing table missing:\n{stdout}"
        );
        assert!(
            !stderr.contains("FAIL:"),
            "{name}: a foreign-host timing was scored:\n{stderr}"
        );
        assert!(
            !stdout.contains("All guardrails passed"),
            "{name}: an unscored run passed:\n{stdout}"
        );
        assert!(
            !stderr.contains("benchmark provenance mismatch"),
            "{name}: host identity must not abort the contract check:\n{stderr}"
        );
    }
}

#[test]
fn should_leave_timings_unscored_when_only_the_cpu_model_differs() {
    // The #514 pair: both hosts have 4 cores, only the model string differs.
    assert_unscored_with_the_opt_in(
        "model-only",
        Provenance {
            cpu_model: "AMD EPYC 9V74 80-Core Processor".to_owned(),
            ..calibrated_host()
        },
        "AMD EPYC 9V74 80-Core Processor (4 cores)",
    );
}

#[test]
fn should_leave_timings_unscored_when_only_the_cpu_count_differs() {
    assert_unscored_with_the_opt_in(
        "count-only",
        Provenance {
            cpu_count: 8,
            ..calibrated_host()
        },
        "AMD EPYC 7763 64-Core Processor (8 cores)",
    );
}

/// Run a regression-sized sample on `host` under the flag and require an unscored, not failed, run.
fn assert_unscored_with_the_opt_in(name: &str, host: Provenance, reported_host: &str) {
    let case = Case::new(name);
    let output = case.run(host, BEYOND_ALLOWANCE_MS, &["--allow-host-mismatch"]);
    let stderr = stderr_of(&output);
    assert!(output.status.success(), "{name}: expected success, stderr:\n{stderr}");
    assert!(
        stderr.contains(&format!("TIMINGS NOT SCORED: this run measured on {reported_host}")),
        "{name}: this host must not count as the calibrated one:\n{stderr}"
    );
    assert!(
        !stderr.contains("FAIL:"),
        "{name}: a foreign-host timing was scored:\n{stderr}"
    );
}

#[test]
fn should_fail_a_foreign_host_as_unscored_without_the_opt_in() {
    for (name, sample) in [
        ("unscored-within", WITHIN_ALLOWANCE_MS),
        ("unscored-beyond", BEYOND_ALLOWANCE_MS),
    ] {
        let case = Case::new(name);
        let output = case.run(foreign_host(), sample, &[]);
        let stderr = stderr_of(&output);
        assert!(!output.status.success(), "{name}: expected failure, stderr:\n{stderr}");
        assert!(
            stderr.contains("timings not scored: this run measured on INTEL(R) XEON(R) PLATINUM 8573C"),
            "{name}: unexpected failure:\n{stderr}"
        );
        assert!(
            !stderr.contains("guardrail(s) violated") && !stderr.contains("FAIL:"),
            "{name}: a foreign-host timing is not a regression:\n{stderr}"
        );
        assert!(
            !stdout_of(&output).contains(UNSCORED_ANNOTATION),
            "{name}: the annotation belongs to the opted-in run only"
        );
    }
}

#[test]
fn should_fail_an_inventory_difference_on_a_foreign_host_even_with_the_opt_in() {
    let case = Case::new("unscored-inventory");
    let output = case.run_with_output_bytes(foreign_host(), BEYOND_ALLOWANCE_MS, 6, &["--allow-host-mismatch"]);
    let stderr = stderr_of(&output);
    assert!(!output.status.success(), "expected failure, stderr:\n{stderr}");
    assert!(
        stderr.contains("fixture metadata differs for fixture.html"),
        "missing inventory difference:\n{stderr}"
    );
    assert!(
        stderr.contains("1 fixture inventory difference(s); timings not scored on this host"),
        "unexpected failure:\n{stderr}"
    );
    assert!(
        !stderr.contains("guardrail violation"),
        "a foreign-host timing must not be counted as a violation:\n{stderr}"
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
            !stderr.contains("TIMINGS NOT SCORED"),
            "{name}: contract drift is a configuration error, not an unscored run:\n{stderr}"
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
        !stderr.contains("TIMINGS NOT SCORED"),
        "hardware matches, so the timings are scored:\n{stderr}"
    );
}

#[test]
fn should_pass_a_clean_run_on_matching_hardware_with_the_opt_in() {
    let case = Case::new("matched-host-clean");
    let output = case.run(calibrated_host(), WITHIN_ALLOWANCE_MS, &["--allow-host-mismatch"]);
    let stdout = stdout_of(&output);
    assert!(
        output.status.success(),
        "expected success, stderr:\n{}",
        stderr_of(&output)
    );
    assert!(
        stdout.contains("All guardrails passed."),
        "unexpected verdict:\n{stdout}"
    );
    assert!(
        !stdout.contains(UNSCORED_ANNOTATION),
        "a scored run is not unscored:\n{stdout}"
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
        self.run_with_output_bytes(results_provenance, sample_ms, BASELINE_OUTPUT_BYTES, flags)
    }

    fn run_with_output_bytes(
        &self,
        results_provenance: Provenance,
        sample_ms: f64,
        output_bytes: u64,
        flags: &[&str],
    ) -> Output {
        let results = self.dir.join("results.json");
        let baseline = self.dir.join("baseline.json");
        let guardrails = self.dir.join("guardrails.json");
        write_json(&results, &run_results(results_provenance, sample_ms, output_bytes));
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

fn stdout_of(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn write_json<T: serde::Serialize>(path: &Path, value: &T) {
    std::fs::write(path, serde_json::to_vec_pretty(value).expect("serializing document")).expect("writing document");
}

fn run_results(provenance: Provenance, sample_ms: f64, output_bytes: u64) -> RunResults {
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
            output_bytes,
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
            output_bytes: BASELINE_OUTPUT_BYTES,
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
