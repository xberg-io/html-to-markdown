//! `htmbench` — benchmark harness CLI for html-to-markdown-rs.
//!
//! Subcommands:
//! - `run`     — benchmark the fixture corpus and write a JSON results file
//! - `compare` — compare a results file against a baseline with guardrail checks
//! - `calibrate` — derive an approved baseline and fixture noise floors
//! - `oracle`  — verify (or bless) Markdown snapshot tests
//! - `survey`  — print a fixture corpus feature-coverage table

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use html_to_markdown_bench::{
    bench, calibration, fixture,
    oracle::{self, Permutation},
    policy, provenance,
    schema::{
        BenchRecord, CalibratedBaseline, Guardrails, LegacyGuardrails, LegacyRunResults, Provenance, RunResults,
        SCHEMA_VERSION,
    },
    survey,
};
use html_to_markdown_rs::TierStrategy;
use html_to_markdown_rs::options::ConversionOptions;

/// Benchmark harness for html-to-markdown-rs.
#[derive(Debug, Parser)]
#[command(name = "htmbench", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run benchmark over fixture corpus and write results JSON.
    Run(RunArgs),
    /// Compare a results file against a baseline with guardrail enforcement.
    Compare(CompareArgs),
    /// Calibrate a baseline and fixture floors from forty full-corpus captures.
    Calibrate(CalibrateArgs),
    /// Run (or bless) Markdown snapshot oracle tests.
    Oracle(OracleArgs),
    /// Print a fixture corpus feature-coverage survey.
    Survey(SurveyArgs),
}

#[derive(Debug, Parser)]
struct RunArgs {
    /// Path to the fixtures directory (contains groups.toml).
    #[arg(long, default_value = "tools/benchmark-harness/fixtures")]
    fixtures: PathBuf,

    /// Write results JSON to this path.
    #[arg(long, default_value = "tools/benchmark-harness/results/latest.json")]
    output: PathBuf,

    /// Only benchmark fixtures belonging to this group.
    #[arg(long)]
    filter: Option<String>,

    /// Override iteration count (default: auto-calibrated).
    #[arg(long, value_parser = clap::value_parser!(u32).range(1..))]
    iters: Option<u32>,

    /// Also benchmark against mdream (requires `compare-mdream` feature).
    #[arg(long)]
    mdream: bool,

    /// Force Tier-1 conversion path, bypassing the classifier (requires `testkit` feature).
    /// Falls back to Tier-2 on bail. Useful for bench isolation; `auto` is the production path.
    #[arg(long, conflicts_with = "force_tier2")]
    force_tier1: bool,

    /// Force Tier-2 conversion path, skipping Tier-1 entirely. Useful for bench isolation;
    /// `auto` is the production path.
    #[arg(long)]
    force_tier2: bool,

    /// Attach a no-op visitor to every conversion. Used to measure the
    /// `NodeContext` build cost and `visit_*` dispatch overhead in isolation.
    #[arg(long)]
    with_visitor: bool,
}

#[expect(clippy::print_stdout, reason = "CLI result output, not diagnostics")]
fn cmd_run(args: RunArgs) -> Result<()> {
    let fixtures = load_run_fixtures(&args)?;
    let sha = git_sha();
    let hostname = hostname();
    let created_at = humantime::format_rfc3339(std::time::SystemTime::now()).to_string();
    let provenance = capture_run_provenance(&args)?;
    let runs = benchmark_fixtures(&fixtures, &args)?;
    let results = RunResults {
        schema: SCHEMA_VERSION,
        sha,
        hostname,
        created_at,
        provenance,
        runs,
    };
    write_run_results(&args.output, &results)?;
    println!("Results written to {}", args.output.display());
    Ok(())
}

fn load_run_fixtures(args: &RunArgs) -> Result<Vec<fixture::Fixture>> {
    tracing::info!("loading fixtures from {}", args.fixtures.display());
    let loader = fixture::Loader::new(args.fixtures.clone());
    let fixtures = loader.load(args.filter.as_deref())?;
    if fixtures.is_empty() {
        anyhow::bail!("no fixtures found (check --filter and groups.toml)");
    }
    Ok(fixtures)
}

fn capture_run_provenance(args: &RunArgs) -> Result<Provenance> {
    let tier_strategy = if args.force_tier1 {
        "tier1"
    } else if args.force_tier2 {
        "tier2"
    } else {
        "auto"
    };
    let visitor_mode = if args.with_visitor { "noop" } else { "disabled" };
    provenance::collect(&provenance::CaptureSettings {
        tier_strategy,
        visitor_mode,
        iteration_override: args.iters,
    })
}

fn benchmark_fixtures(fixtures: &[fixture::Fixture], args: &RunArgs) -> Result<Vec<BenchRecord>> {
    let mut runs: Vec<BenchRecord> = Vec::with_capacity(fixtures.len());
    for fix in fixtures {
        runs.push(benchmark_fixture(fix, args)?);
    }
    Ok(runs)
}

fn benchmark_fixture(fix: &fixture::Fixture, args: &RunArgs) -> Result<BenchRecord> {
    let html = std::fs::read_to_string(&fix.path).with_context(|| format!("reading {}", fix.path.display()))?;
    let options = conversion_options(args)?;
    let measurement = bench::run_one(&html, options, args.iters);
    if measurement.median_ms == 0.0 {
        tracing::warn!(
            "NOTE: {} panicked during bench (known core bug) — recording 0 ms",
            fix.rel_path
        );
    }
    if args.mdream {
        tracing::warn!("--mdream flag has no effect (compare-mdream feature removed)");
    }
    let mb_per_s = throughput(fix.bytes, measurement.median_ms);
    let record = BenchRecord {
        fixture: fix.rel_path.clone(),
        group: fix.group.clone(),
        bytes: fix.bytes,
        samples_ms: measurement.samples_ms,
        median_ms: measurement.median_ms,
        mad_ms: measurement.mad_ms,
        legacy_ms_best: measurement.legacy_ms_best,
        mb_per_s,
        output_bytes: measurement.output_bytes as u64,
    };
    tracing::info!(
        "{:<55}  median={:.4} ms  MAD={:.4} ms  {:.1} MB/s",
        fix.rel_path,
        record.median_ms,
        record.mad_ms,
        mb_per_s,
    );
    Ok(record)
}

fn conversion_options(args: &RunArgs) -> Result<Option<ConversionOptions>> {
    let options = tier_options(args)?;
    if args.with_visitor {
        return with_noop_visitor(options);
    }
    Ok(options)
}

fn tier_options(args: &RunArgs) -> Result<Option<ConversionOptions>> {
    if args.force_tier1 {
        return tier_one_options();
    }
    if args.force_tier2 {
        return Ok(Some(ConversionOptions {
            tier_strategy: TierStrategy::Tier2,
            ..ConversionOptions::default()
        }));
    }
    Ok(None)
}

#[cfg(feature = "testkit")]
fn tier_one_options() -> Result<Option<ConversionOptions>> {
    Ok(Some(ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        ..ConversionOptions::default()
    }))
}

#[cfg(not(feature = "testkit"))]
fn tier_one_options() -> Result<Option<ConversionOptions>> {
    anyhow::bail!(
        "--force-tier1 requires building with the testkit feature: cargo run --features testkit -- run --force-tier1"
    )
}

#[cfg(feature = "visitor")]
fn with_noop_visitor(options: Option<ConversionOptions>) -> Result<Option<ConversionOptions>> {
    Ok(Some(ConversionOptions {
        visitor: Some(bench::new_noop_visitor_handle()),
        ..options.unwrap_or_default()
    }))
}

#[cfg(not(feature = "visitor"))]
fn with_noop_visitor(_options: Option<ConversionOptions>) -> Result<Option<ConversionOptions>> {
    anyhow::bail!("--with-visitor requires building with the visitor feature")
}

fn throughput(bytes: u64, median_ms: f64) -> f64 {
    if median_ms > 0.0 {
        (bytes as f64 / 1_048_576.0) / (median_ms / 1_000.0)
    } else {
        0.0
    }
}

fn write_run_results(output: &Path, results: &RunResults) -> Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent).with_context(|| format!("creating output dir {}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(&results)?;
    std::fs::write(output, &json).with_context(|| format!("writing {}", output.display()))?;
    Ok(())
}

#[derive(Debug, Parser)]
struct CompareArgs {
    /// Results file to evaluate.
    #[arg(long, default_value = "tools/benchmark-harness/results/latest.json")]
    results: PathBuf,

    /// Baseline file to compare against.
    #[arg(long, default_value = "tools/benchmark-harness/baselines/baseline.json")]
    baseline: PathBuf,

    /// Guardrails file.
    #[arg(long, default_value = "tools/benchmark-harness/guardrails.json")]
    guardrails: PathBuf,

    /// Exit successfully when this host's CPU differs from the calibrated one, so the timings
    /// cannot be scored.
    ///
    /// Timings are never scored on a CPU the baseline was not calibrated on, with or without this
    /// flag. Off by default: such a run fails with "timings not scored". With the flag it succeeds
    /// and prints a GitHub Actions warning instead. It never weakens the provenance contract or
    /// the fixture inventory check, and on hardware that *does* match the baseline it changes
    /// nothing, so a genuine regression measured on the calibrated CPU still fails.
    #[arg(long)]
    allow_host_mismatch: bool,
}

#[expect(clippy::print_stderr, reason = "guardrail report is this command's result output")]
fn cmd_compare(args: CompareArgs) -> Result<()> {
    let results: RunResults = load_schema_v2(&args.results, "results")?;
    let baseline_value = load_value(&args.baseline)?;
    let guardrails_value = load_value(&args.guardrails)?;
    let baseline_schema = schema_of(&baseline_value);
    let guardrails_schema = schema_of(&guardrails_value);
    let (evaluation, host_mismatch) = match (baseline_schema, guardrails_schema) {
        (1, 1) => {
            eprintln!(
                "WARNING: schema-v1 baseline has no calibrated fixture floors; using temporary percentage-only policy"
            );
            let comparisons = policy::evaluate_legacy(
                &results,
                &serde_json::from_value::<LegacyRunResults>(baseline_value)?,
                &serde_json::from_value::<LegacyGuardrails>(guardrails_value)?,
            )?;
            (
                policy::Evaluation {
                    comparisons,
                    inventory_mismatches: Vec::new(),
                },
                None,
            )
        }
        (SCHEMA_VERSION, SCHEMA_VERSION) => {
            let baseline = serde_json::from_value::<CalibratedBaseline>(baseline_value)?;
            let guardrails = serde_json::from_value::<Guardrails>(guardrails_value)?;
            let evaluation = policy::evaluate_strict(&results, &baseline, &guardrails)?;
            (evaluation, policy::host_mismatch(&results, &guardrails))
        }
        _ => anyhow::bail!(
            "baseline/guardrails schema mismatch: baseline={baseline_schema}, guardrails={guardrails_schema}"
        ),
    };

    let failures = report_comparisons(&evaluation.comparisons);
    report_inventory_mismatches(&evaluation.inventory_mismatches);
    report_verdict(
        &failures,
        &evaluation.inventory_mismatches,
        host_mismatch.as_ref(),
        args.allow_host_mismatch,
    )
}

/// Print inventory differences beneath the timing table.
///
/// ~keep Printed after the comparisons, not instead of them: an inventory difference used to abort
/// `compare` before a single timing was evaluated, so an accepted output change hid whatever the
/// timings were doing. Both are shown, and `report_verdict` fails on an inventory difference on
/// any host, and on a timing violation on the calibrated one.
#[expect(
    clippy::print_stderr,
    reason = "inventory diagnostics belong on stderr with the verdict"
)]
fn report_inventory_mismatches(mismatches: &[String]) {
    if mismatches.is_empty() {
        return;
    }
    eprintln!(
        "\nFIXTURE INVENTORY DIFFERS FROM THE BASELINE ({} difference(s)):",
        mismatches.len()
    );
    for mismatch in mismatches {
        eprintln!("  {mismatch}");
    }
    eprintln!(
        "Re-calibrate with `mise run bench:calibrate --runs-dir <dir>` (--accept-output-change when the output change is intended and reviewed)."
    );
}

#[expect(clippy::print_stdout, reason = "per-fixture comparison table is CLI result output")]
fn report_comparisons(comparisons: &[policy::Comparison]) -> Vec<String> {
    let mut failures = Vec::new();
    for comparison in comparisons {
        let delta_ms = comparison.current_ms - comparison.baseline_ms;
        let pct_change = delta_ms / comparison.baseline_ms * 100.0;
        println!(
            "{:<55} base={:.4}ms new={:.4}ms {:+.1}% allowed={:.4}ms (+{:.0}%)",
            comparison.fixture,
            comparison.baseline_ms,
            comparison.current_ms,
            pct_change,
            comparison.allowed_delta_ms,
            comparison.threshold_pct,
        );
        if comparison.failed {
            failures.push(format!(
                "{}: delta {:.4}ms exceeds effective allowance {:.4}ms",
                comparison.fixture, delta_ms, comparison.allowed_delta_ms
            ));
        }
    }
    failures
}

/// Decide the run's outcome: passed, failed, or timings not scored.
///
/// Timings are scored only on the CPU the baseline was calibrated on. On any other CPU they are
/// neither a pass nor a regression, so the run is reported as unscored; `--allow-host-mismatch`
/// decides only whether an unscored run exits successfully.
#[expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "guardrail verdict is this command's result output"
)]
fn report_verdict(
    failures: &[String],
    inventory_mismatches: &[String],
    host_mismatch: Option<&policy::HostMismatch>,
    allow_host_mismatch: bool,
) -> Result<()> {
    if let Some(mismatch) = host_mismatch {
        return report_unscored(mismatch, inventory_mismatches, allow_host_mismatch);
    }
    if failures.is_empty() && inventory_mismatches.is_empty() {
        println!("\nAll guardrails passed.");
        return Ok(());
    }
    for failure in failures {
        eprintln!("FAIL: {failure}");
    }
    if !inventory_mismatches.is_empty() {
        anyhow::bail!(
            "{} fixture inventory difference(s) and {} guardrail violation(s)",
            inventory_mismatches.len(),
            failures.len()
        );
    }
    anyhow::bail!("{} guardrail(s) violated", failures.len())
}

/// Report a run whose timings cannot be scored because it ran on a CPU the baseline never saw.
///
/// ~keep No timing is scored here, in either direction: a delta measured on a CPU the baseline was
/// not calibrated on reflects the host, not the code (#514). Counting such deltas as violations and
/// then waiving them printed FAIL lines under a green run. A fixture inventory is a property of the
/// corpus and the converter, not of the CPU, so it stays fatal on any host.
#[expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "guardrail verdict is this command's result output"
)]
fn report_unscored(
    mismatch: &policy::HostMismatch,
    inventory_mismatches: &[String],
    allow_host_mismatch: bool,
) -> Result<()> {
    let reason = format!(
        "this run measured on {} ({} cores), but the baseline was calibrated on {} ({} cores)",
        mismatch.current_cpu_model,
        mismatch.current_cpu_count,
        mismatch.calibrated_cpu_model,
        mismatch.calibrated_cpu_count,
    );
    eprintln!("TIMINGS NOT SCORED: {reason}; a timing on other hardware is neither a pass nor a regression.");
    if !inventory_mismatches.is_empty() {
        anyhow::bail!(
            "{} fixture inventory difference(s); timings not scored on this host",
            inventory_mismatches.len()
        );
    }
    if !allow_host_mismatch {
        anyhow::bail!("timings not scored: {reason}");
    }
    // ~keep A workflow command, so the unscored run shows on the Actions run page instead of
    // passing as silently as a scored one. Outside Actions it is one plain line.
    println!("::warning title=Benchmark timings not scored::{reason}");
    Ok(())
}

#[derive(Debug, Parser)]
struct CalibrateArgs {
    /// Directory containing exactly forty schema-v2 full-corpus result files.
    #[arg(long)]
    runs_dir: PathBuf,

    /// Baseline file to migrate or update.
    #[arg(long, default_value = "tools/benchmark-harness/baselines/baseline.json")]
    baseline: PathBuf,

    /// Guardrails file to migrate or update.
    #[arg(long, default_value = "tools/benchmark-harness/guardrails.json")]
    guardrails: PathBuf,

    /// Allow fixtures whose converted output size changed since the baseline being replaced.
    ///
    /// Required after a deliberate conversion fix changes what the corpus renders to: the
    /// inventory is otherwise checked against the very baseline this command replaces, so there
    /// would be no way to re-promote. Fixture set, groups and input sizes are still compared
    /// exactly. Promotion still requires retained artifacts, comparable hardware, a quiet-runner
    /// record and reviewer approval -- see the harness README.
    #[arg(long)]
    accept_output_change: bool,
}

#[expect(clippy::print_stdout, reason = "calibration result is CLI output")]
fn cmd_calibrate(args: CalibrateArgs) -> Result<()> {
    calibration::calibrate(
        &args.runs_dir,
        &args.baseline,
        &args.guardrails,
        args.accept_output_change,
    )?;
    println!(
        "Calibrated {} and {} from {}.",
        args.baseline.display(),
        args.guardrails.display(),
        args.runs_dir.display()
    );
    Ok(())
}

#[derive(Debug, Parser)]
struct OracleArgs {
    /// Path to the fixtures directory.
    #[arg(long, default_value = "tools/benchmark-harness/fixtures")]
    fixtures: PathBuf,

    /// Path to the snapshots directory.
    #[arg(long, default_value = "tools/benchmark-harness/snapshots")]
    snapshots: PathBuf,

    /// Calibrated baseline containing expected default output sizes.
    #[arg(long, default_value = "tools/benchmark-harness/baselines/baseline.json")]
    baseline: PathBuf,

    /// Only test fixtures belonging to this group.
    #[arg(long)]
    filter: Option<String>,

    /// Overwrite stored snapshots instead of comparing.
    #[arg(long)]
    bless: bool,
}

#[expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "oracle comparison report is this command's result output"
)]
fn cmd_oracle(args: OracleArgs) -> Result<()> {
    let loader = fixture::Loader::new(args.fixtures.clone());
    let fixtures = loader.load(args.filter.as_deref())?;
    let baseline = load_calibrated_baseline(&args.baseline)?;

    let mut failures = Vec::new();
    let mut skipped = 0usize;
    let mut passed = 0usize;

    for fix in &fixtures {
        let html = std::fs::read_to_string(&fix.path).with_context(|| format!("reading {}", fix.path.display()))?;

        for &perm in Permutation::ALL {
            if args.bless {
                let wrote = oracle::bless(&args.snapshots, &fix.rel_path, &html, perm)
                    .with_context(|| format!("blessing {} ({:?})", fix.rel_path, perm))?;
                if wrote {
                    tracing::info!("blessed {} ({})", fix.rel_path, perm.slug());
                } else {
                    skipped += 1;
                }
            } else {
                match oracle::compare(&args.snapshots, &fix.rel_path, &html, perm) {
                    Ok(true) => {
                        tracing::info!("ok {} ({})", fix.rel_path, perm.slug());
                        passed += 1;
                    }
                    Ok(false) => {
                        skipped += 1;
                    }
                    Err(e) => {
                        eprintln!("FAIL: {e}");
                        failures.push(format!("{} ({}): {e}", fix.rel_path, perm.slug()));
                    }
                }
            }
        }
    }

    if args.bless {
        oracle::validate_output_sizes(&args.snapshots, &fixtures, &baseline.runs)?;
        println!(
            "Snapshots blessed for {} fixture(s) ({} skipped due to core panics).",
            fixtures.len(),
            skipped
        );
        Ok(())
    } else if failures.is_empty() {
        oracle::validate_output_sizes(&args.snapshots, &fixtures, &baseline.runs)?;
        println!(
            "All oracle snapshots match ({} ok, {} skipped due to known core panics).",
            passed, skipped
        );
        Ok(())
    } else {
        anyhow::bail!("{} oracle failure(s)", failures.len())
    }
}

#[derive(Debug, Parser)]
struct SurveyArgs {
    /// Path to the fixtures directory.
    #[arg(long, default_value = "tools/benchmark-harness/fixtures")]
    fixtures: PathBuf,

    /// Only survey fixtures belonging to this group.
    #[arg(long)]
    filter: Option<String>,
}

fn cmd_survey(args: SurveyArgs) -> Result<()> {
    let stats = survey::run_survey(&args.fixtures, args.filter.as_deref())?;
    survey::print_survey(&stats);
    Ok(())
}

fn load_json<T: serde::de::DeserializeOwned>(path: &PathBuf) -> Result<T> {
    let raw = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    serde_json::from_str(&raw).with_context(|| format!("parsing {}", path.display()))
}

fn load_value(path: &PathBuf) -> Result<serde_json::Value> {
    load_json(path)
}

fn schema_of(value: &serde_json::Value) -> u32 {
    value
        .get("schema")
        .and_then(serde_json::Value::as_u64)
        .and_then(|schema| u32::try_from(schema).ok())
        .unwrap_or(0)
}

fn load_schema_v2(path: &PathBuf, kind: &str) -> Result<RunResults> {
    let value = load_value(path)?;
    let schema = schema_of(&value);
    anyhow::ensure!(
        schema == SCHEMA_VERSION,
        "unsupported {kind} schema {schema}; expected {SCHEMA_VERSION}"
    );
    serde_json::from_value(value).with_context(|| format!("decoding schema-v2 {kind} {}", path.display()))
}

fn load_calibrated_baseline(path: &PathBuf) -> Result<CalibratedBaseline> {
    let value = load_value(path)?;
    let schema = schema_of(&value);
    anyhow::ensure!(
        schema == SCHEMA_VERSION,
        "unsupported baseline schema {schema}; expected {SCHEMA_VERSION}"
    );
    serde_json::from_value(value).with_context(|| format!("decoding schema-v2 baseline {}", path.display()))
}

fn git_sha() -> String {
    std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout).ok().map(|s| s.trim().to_owned())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "unknown".to_owned())
}

fn hostname() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("HOST"))
        .or_else(|_| {
            std::process::Command::new("hostname")
                .output()
                .ok()
                .filter(|output| output.status.success())
                .and_then(|output| String::from_utf8(output.stdout).ok())
                .map(|value| value.trim().to_owned())
                .ok_or(std::env::VarError::NotPresent)
        })
        .unwrap_or_else(|_| "unknown".to_owned())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_env("HTMBENCH_LOG").add_directive(tracing::Level::INFO.into()),
        )
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();
    match cli.command {
        Commands::Run(args) => cmd_run(args),
        Commands::Compare(args) => cmd_compare(args),
        Commands::Calibrate(args) => cmd_calibrate(args),
        Commands::Oracle(args) => cmd_oracle(args),
        Commands::Survey(args) => cmd_survey(args),
    }
}
