//! `htmbench` — benchmark harness CLI for html-to-markdown-rs.
//!
//! Subcommands:
//! - `run`     — benchmark the fixture corpus and write a JSON results file
//! - `compare` — compare a results file against a baseline with guardrail checks
//! - `oracle`  — verify (or bless) Markdown snapshot tests
//! - `survey`  — print a fixture corpus feature-coverage table

use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use html_to_markdown_bench::{
    bench, fixture,
    oracle::{self, Permutation},
    schema::{BenchRecord, Guardrails, RunResults, SCHEMA_VERSION},
    survey,
};
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
    /// Run (or bless) Markdown snapshot oracle tests.
    Oracle(OracleArgs),
    /// Print a fixture corpus feature-coverage survey.
    Survey(SurveyArgs),
}

// ---------------------------------------------------------------------------
// Run subcommand
// ---------------------------------------------------------------------------

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
    group: Option<String>,

    /// Also benchmark against mdream (requires `compare-mdream` feature).
    #[arg(long)]
    mdream: bool,
}

fn cmd_run(args: RunArgs) -> Result<()> {
    tracing::info!("loading fixtures from {}", args.fixtures.display());
    let loader = fixture::Loader::new(args.fixtures.clone());
    let fixtures = loader.load(args.group.as_deref())?;

    if fixtures.is_empty() {
        anyhow::bail!("no fixtures found (check --group filter and groups.toml)");
    }

    let sha = git_sha();
    let host = hostname();
    let created_at = humantime::format_rfc3339(std::time::SystemTime::now()).to_string();

    let mut runs: Vec<BenchRecord> = Vec::with_capacity(fixtures.len());
    for fix in &fixtures {
        let html = std::fs::read_to_string(&fix.path).with_context(|| format!("reading {}", fix.path.display()))?;

        let opts: Option<ConversionOptions> = None;
        let (ms_best, output_bytes) = bench::run_one(&html, opts);
        let mb_per_s = if ms_best > 0.0 {
            (fix.bytes as f64 / 1_048_576.0) / (ms_best / 1_000.0)
        } else {
            0.0
        };

        #[cfg(feature = "compare-mdream")]
        let mdream_ms_best = if args.mdream {
            Some(bench::run_one_mdream(&html))
        } else {
            None
        };
        #[cfg(not(feature = "compare-mdream"))]
        let mdream_ms_best: Option<f64> = None;

        if args.mdream {
            #[cfg(not(feature = "compare-mdream"))]
            tracing::warn!("--mdream flag has no effect without the compare-mdream feature");
        }

        let record = BenchRecord {
            fixture: fix.rel_path.clone(),
            group: fix.group.clone(),
            bytes: fix.bytes,
            ms_best,
            mb_per_s,
            output_bytes: output_bytes as u64,
            mdream_ms_best,
        };

        tracing::info!("{:<50}  {:.4} ms  {:.1} MB/s", fix.rel_path, ms_best, mb_per_s,);
        runs.push(record);
    }

    let results = RunResults {
        schema: SCHEMA_VERSION,
        sha,
        host,
        created_at,
        runs,
    };

    if let Some(parent) = args.output.parent() {
        std::fs::create_dir_all(parent).with_context(|| format!("creating output dir {}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(&results)?;
    std::fs::write(&args.output, json).with_context(|| format!("writing {}", args.output.display()))?;
    println!("Results written to {}", args.output.display());
    Ok(())
}

// ---------------------------------------------------------------------------
// Compare subcommand
// ---------------------------------------------------------------------------

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
}

fn cmd_compare(args: CompareArgs) -> Result<()> {
    let results: RunResults = load_json(&args.results)?;
    let baseline: RunResults = load_json(&args.baseline)?;
    let guardrails: Guardrails = load_json(&args.guardrails)?;

    // Build baseline lookup by fixture path
    let baseline_map: HashMap<&str, f64> = baseline.runs.iter().map(|r| (r.fixture.as_str(), r.ms_best)).collect();

    let mut failures = Vec::new();
    for record in &results.runs {
        let Some(&base_ms) = baseline_map.get(record.fixture.as_str()) else {
            tracing::warn!("no baseline for fixture {}, skipping", record.fixture);
            continue;
        };

        let threshold = guardrails
            .thresholds
            .get(&record.group)
            .map_or(10.0, |t| t.max_regression_pct);

        if base_ms > 0.0 {
            let pct_change = (record.ms_best - base_ms) / base_ms * 100.0;
            let symbol = if pct_change > 0.0 { "+" } else { "" };
            println!(
                "{:<50}  base={:.4}ms  new={:.4}ms  {}{:.1}%  (limit +{:.0}%)",
                record.fixture, base_ms, record.ms_best, symbol, pct_change, threshold
            );
            if pct_change > threshold {
                failures.push(format!(
                    "{}: regression {:.1}% exceeds limit {:.0}%",
                    record.fixture, pct_change, threshold
                ));
            }
        }
    }

    if failures.is_empty() {
        println!("\nAll guardrails passed.");
        Ok(())
    } else {
        for f in &failures {
            eprintln!("FAIL: {f}");
        }
        anyhow::bail!("{} guardrail(s) violated", failures.len())
    }
}

// ---------------------------------------------------------------------------
// Oracle subcommand
// ---------------------------------------------------------------------------

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
    group: Option<String>,

    /// Overwrite stored snapshots instead of comparing.
    #[arg(long)]
    bless: bool,
}

fn cmd_oracle(args: OracleArgs) -> Result<()> {
    let loader = fixture::Loader::new(args.fixtures.clone());
    let fixtures = loader.load(args.group.as_deref())?;

    let mut failures = Vec::new();
    for fix in &fixtures {
        let html = std::fs::read_to_string(&fix.path).with_context(|| format!("reading {}", fix.path.display()))?;

        for &perm in Permutation::ALL {
            if args.bless {
                oracle::bless(&args.snapshots, &fix.rel_path, &html, perm)
                    .with_context(|| format!("blessing {} ({:?})", fix.rel_path, perm))?;
                tracing::info!("blessed {} ({})", fix.rel_path, perm.slug());
            } else {
                match oracle::compare(&args.snapshots, &fix.rel_path, &html, perm) {
                    Ok(()) => tracing::info!("ok {} ({})", fix.rel_path, perm.slug()),
                    Err(e) => {
                        eprintln!("FAIL: {e}");
                        failures.push(format!("{} ({}): {e}", fix.rel_path, perm.slug()));
                    }
                }
            }
        }
    }

    if args.bless {
        println!("Snapshots blessed for {} fixture(s).", fixtures.len());
        Ok(())
    } else if failures.is_empty() {
        println!("All oracle snapshots match ({} fixture(s)).", fixtures.len());
        Ok(())
    } else {
        anyhow::bail!("{} oracle failure(s)", failures.len())
    }
}

// ---------------------------------------------------------------------------
// Survey subcommand
// ---------------------------------------------------------------------------

#[derive(Debug, Parser)]
struct SurveyArgs {
    /// Path to the fixtures directory.
    #[arg(long, default_value = "tools/benchmark-harness/fixtures")]
    fixtures: PathBuf,

    /// Only survey fixtures belonging to this group.
    #[arg(long)]
    group: Option<String>,
}

fn cmd_survey(args: SurveyArgs) -> Result<()> {
    let stats = survey::run_survey(&args.fixtures, args.group.as_deref())?;
    survey::print_survey(&stats);
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn load_json<T: serde::de::DeserializeOwned>(path: &PathBuf) -> Result<T> {
    let raw = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    serde_json::from_str(&raw).with_context(|| format!("parsing {}", path.display()))
}

fn git_sha() -> String {
    std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
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
        .unwrap_or_else(|_| "unknown".to_owned())
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

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
        Commands::Oracle(args) => cmd_oracle(args),
        Commands::Survey(args) => cmd_survey(args),
    }
}
