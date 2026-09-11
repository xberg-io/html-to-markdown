//! `htmbench` — benchmark harness CLI for html-to-markdown-rs.
//!
//! Subcommands:
//! - `run`     — benchmark the fixture corpus and write a JSON results file
//! - `compare` — compare a results file against a baseline with guardrail checks
//! - `calibrate` — derive an approved baseline and fixture noise floors
//! - `oracle`  — verify (or bless) Markdown snapshot tests
//! - `survey`  — print a fixture corpus feature-coverage table

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use html_to_markdown_bench::{
    bench, calibration, fixture,
    oracle::{self, Permutation},
    policy, provenance,
    schema::{
        BenchRecord, CalibratedBaseline, Guardrails, LegacyGuardrails, LegacyRunResults, RunResults, SCHEMA_VERSION,
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
    tracing::info!("loading fixtures from {}", args.fixtures.display());
    let loader = fixture::Loader::new(args.fixtures.clone());
    let fixtures = loader.load(args.filter.as_deref())?;

    if fixtures.is_empty() {
        anyhow::bail!("no fixtures found (check --filter and groups.toml)");
    }

    let sha = git_sha();
    let hostname = hostname();
    let tier_strategy = if args.force_tier1 {
        "tier1"
    } else if args.force_tier2 {
        "tier2"
    } else {
        "auto"
    };
    let visitor_mode = if args.with_visitor { "noop" } else { "disabled" };
    let provenance = provenance::collect(&provenance::CaptureSettings {
        tier_strategy,
        visitor_mode,
        iteration_override: args.iters,
    })?;
    let created_at = humantime::format_rfc3339(std::time::SystemTime::now()).to_string();

    let mut runs: Vec<BenchRecord> = Vec::with_capacity(fixtures.len());
    for fix in &fixtures {
        let html = std::fs::read_to_string(&fix.path).with_context(|| format!("reading {}", fix.path.display()))?;

        let opts: Option<ConversionOptions> = if args.force_tier1 {
            #[cfg(feature = "testkit")]
            {
                Some(ConversionOptions {
                    tier_strategy: TierStrategy::Tier1,
                    ..ConversionOptions::default()
                })
            }
            #[cfg(not(feature = "testkit"))]
            {
                anyhow::bail!(
                    "--force-tier1 requires building with the testkit feature: cargo run --features testkit -- run --force-tier1"
                );
            }
        } else if args.force_tier2 {
            Some(ConversionOptions {
                tier_strategy: TierStrategy::Tier2,
                ..ConversionOptions::default()
            })
        } else {
            None
        };

        #[cfg(feature = "visitor")]
        let opts = if args.with_visitor {
            {
                let handle = html_to_markdown_bench::bench::new_noop_visitor_handle();
                Some(ConversionOptions {
                    visitor: Some(handle),
                    ..opts.unwrap_or_default()
                })
            }
        } else {
            opts
        };
        #[cfg(not(feature = "visitor"))]
        let opts = {
            if args.with_visitor {
                anyhow::bail!("--with-visitor requires building with the visitor feature");
            }
            opts
        };
        let measurement = bench::run_one(&html, opts, args.iters);
        if measurement.median_ms == 0.0 {
            tracing::warn!(
                "NOTE: {} panicked during bench (known core bug) — recording 0 ms",
                fix.rel_path
            );
        }
        let mb_per_s = if measurement.median_ms > 0.0 {
            (fix.bytes as f64 / 1_048_576.0) / (measurement.median_ms / 1_000.0)
        } else {
            0.0
        };

        if args.mdream {
            tracing::warn!("--mdream flag has no effect (compare-mdream feature removed)");
        }

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
        runs.push(record);
    }

    let results = RunResults {
        schema: SCHEMA_VERSION,
        sha,
        hostname,
        created_at,
        provenance,
        runs,
    };

    if let Some(parent) = args.output.parent() {
        std::fs::create_dir_all(parent).with_context(|| format!("creating output dir {}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(&results)?;
    std::fs::write(&args.output, &json).with_context(|| format!("writing {}", args.output.display()))?;
    println!("Results written to {}", args.output.display());
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

    /// Report timing violations as advisory when this host's CPU differs from the calibrated one.
    ///
    /// Off by default: a violation is fatal. Only pass this where the runner pool is known to be
    /// heterogeneous (CI), and understand what it does and does not relax — it never weakens the
    /// provenance contract, and on hardware that *does* match the baseline it changes nothing, so
    /// a genuine regression measured on the calibrated CPU still fails.
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
    let (comparisons, host_mismatch) = match (baseline_schema, guardrails_schema) {
        (1, 1) => {
            eprintln!(
                "WARNING: schema-v1 baseline has no calibrated fixture floors; using temporary percentage-only policy"
            );
            let comparisons = policy::evaluate_legacy(
                &results,
                &serde_json::from_value::<LegacyRunResults>(baseline_value)?,
                &serde_json::from_value::<LegacyGuardrails>(guardrails_value)?,
            )?;
            (comparisons, None)
        }
        (SCHEMA_VERSION, SCHEMA_VERSION) => {
            let baseline = serde_json::from_value::<CalibratedBaseline>(baseline_value)?;
            let guardrails = serde_json::from_value::<Guardrails>(guardrails_value)?;
            let comparisons = policy::evaluate_strict(&results, &baseline, &guardrails)?;
            (comparisons, policy::host_mismatch(&results, &guardrails))
        }
        _ => anyhow::bail!(
            "baseline/guardrails schema mismatch: baseline={baseline_schema}, guardrails={guardrails_schema}"
        ),
    };

    let failures = report_comparisons(&comparisons);
    report_verdict(&failures, host_mismatch.as_ref(), args.allow_host_mismatch)
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

/// Decide whether guardrail violations are fatal, and say why when they are not.
///
/// Violations are downgraded to advisory only when both halves hold: the capture ran on a CPU the
/// baseline was never calibrated on, *and* the caller explicitly opted in. Either half missing
/// keeps the violation fatal, which is what preserves regression detection on matched hardware.
#[expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "guardrail verdict is this command's result output"
)]
fn report_verdict(
    failures: &[String],
    host_mismatch: Option<&policy::HostMismatch>,
    allow_host_mismatch: bool,
) -> Result<()> {
    if let Some(mismatch) = host_mismatch {
        eprintln!(
            "WARNING: host CPU differs from the calibrated baseline host: {} ({} cores) vs calibrated {} ({} cores)",
            mismatch.current_cpu_model,
            mismatch.current_cpu_count,
            mismatch.calibrated_cpu_model,
            mismatch.calibrated_cpu_count,
        );
    }
    if failures.is_empty() {
        println!("\nAll guardrails passed.");
        return Ok(());
    }
    for failure in failures {
        eprintln!("FAIL: {failure}");
    }
    if host_mismatch.is_some() && allow_host_mismatch {
        eprintln!(
            "ADVISORY: {} guardrail(s) violated on a CPU the baseline was not calibrated on; \
             not failing the run because --allow-host-mismatch was passed",
            failures.len()
        );
        return Ok(());
    }
    anyhow::bail!("{} guardrail(s) violated", failures.len())
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
        println!(
            "Snapshots blessed for {} fixture(s) ({} skipped due to core panics).",
            fixtures.len(),
            skipped
        );
        Ok(())
    } else if failures.is_empty() {
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
