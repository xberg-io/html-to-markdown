//! Forty-run fixture-floor calibration and rollback-safe baseline promotion.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, ensure};

use crate::schema::{
    BenchRecord, CALIBRATION_RUNS, CalibratedBaseline, CalibratedBenchRecord, FixtureFloor, GroupThreshold, Guardrails,
    LegacyGuardrails, LegacyRunResults, Provenance, RunResults, SAMPLES_PER_RUN, SCHEMA_VERSION,
};
use crate::stats;

type Inventory = HashMap<String, (String, u64, u64)>;
type Thresholds = HashMap<String, GroupThreshold>;

/// Validate a campaign, then promote its baseline and guardrails as one rollback-safe pair.
///
/// `accept_output_change` relaxes **only** the converted-output size comparison, for the case where
/// a deliberate conversion fix legitimately changes what the corpus renders to. Without it there is
/// no way to re-promote after such a fix, because the inventory is checked against the very baseline
/// being replaced. The fixture set, its groups and its input sizes are still compared exactly, so a
/// changed or missing fixture is still rejected. ~keep
pub fn calibrate(
    runs_dir: &Path,
    baseline_path: &Path,
    guardrails_path: &Path,
    accept_output_change: bool,
) -> Result<()> {
    let run_paths = run_paths(runs_dir)?;
    let runs: Vec<RunResults> = run_paths
        .iter()
        .map(|path| load_schema_v2(path, "calibration result"))
        .collect::<Result<_>>()?;
    let (expected_inventory, thresholds) = load_existing_policy(baseline_path, guardrails_path)?;
    validate_campaign(&runs, &expected_inventory, accept_output_change)?;
    let (baseline, guardrails) = build_outputs(&runs, thresholds)?;
    let baseline_json = to_json_line_terminated(&baseline).context("serializing calibrated baseline")?;
    let guardrails_json = to_json_line_terminated(&guardrails).context("serializing calibrated guardrails")?;
    promote_pair(baseline_path, &baseline_json, guardrails_path, &guardrails_json)
}

/// Serialize pretty JSON with the trailing newline every formatter here expects.
///
/// `to_vec_pretty` omits it, so a promoted pair failed the repository's format check and any
/// reformat was undone by the next promotion. ~keep
fn to_json_line_terminated<T: serde::Serialize>(value: &T) -> serde_json::Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn run_paths(runs_dir: &Path) -> Result<Vec<PathBuf>> {
    let paths: Vec<PathBuf> = (1..=CALIBRATION_RUNS)
        .map(|index| runs_dir.join(format!("{index:04}.json")))
        .collect();
    for path in &paths {
        ensure!(path.is_file(), "missing ordered calibration capture {}", path.display());
    }
    let json_count = std::fs::read_dir(runs_dir)
        .with_context(|| format!("reading calibration directory {}", runs_dir.display()))?
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.path().extension().is_some_and(|extension| extension == "json"))
        .count();
    ensure!(
        json_count == CALIBRATION_RUNS,
        "calibration directory must contain only 0001.json through 0040.json"
    );
    Ok(paths)
}

fn load_schema_v2(path: &Path, kind: &str) -> Result<RunResults> {
    let raw = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let value: serde_json::Value = serde_json::from_str(&raw).with_context(|| format!("parsing {}", path.display()))?;
    let schema = schema_of(&value);
    ensure!(
        schema == SCHEMA_VERSION,
        "unsupported {kind} schema {schema} in {}; expected {SCHEMA_VERSION}",
        path.display()
    );
    serde_json::from_value(value).with_context(|| format!("decoding schema-v2 {kind} {}", path.display()))
}

fn load_existing_policy(baseline_path: &Path, guardrails_path: &Path) -> Result<(Inventory, Thresholds)> {
    let baseline_value = load_value(baseline_path, "baseline")?;
    let inventory = match schema_of(&baseline_value) {
        1 => legacy_inventory(serde_json::from_value::<LegacyRunResults>(baseline_value)?.runs),
        SCHEMA_VERSION => calibrated_inventory(serde_json::from_value::<CalibratedBaseline>(baseline_value)?.runs),
        schema => anyhow::bail!("unsupported baseline schema {schema}; expected 1 or {SCHEMA_VERSION}"),
    }?;
    let guardrails_value = load_value(guardrails_path, "guardrails")?;
    let thresholds = match schema_of(&guardrails_value) {
        1 => serde_json::from_value::<LegacyGuardrails>(guardrails_value)?.thresholds,
        SCHEMA_VERSION => serde_json::from_value::<Guardrails>(guardrails_value)?.thresholds,
        schema => anyhow::bail!("unsupported guardrails schema {schema}; expected 1 or {SCHEMA_VERSION}"),
    };
    validate_thresholds(&thresholds, inventory.values().map(|metadata| metadata.0.as_str()))?;
    Ok((inventory, thresholds))
}

fn load_value(path: &Path, kind: &str) -> Result<serde_json::Value> {
    let raw = std::fs::read_to_string(path).with_context(|| format!("reading {kind} {}", path.display()))?;
    serde_json::from_str(&raw).with_context(|| format!("parsing {kind} {}", path.display()))
}

fn schema_of(value: &serde_json::Value) -> u32 {
    value
        .get("schema")
        .and_then(serde_json::Value::as_u64)
        .and_then(|schema| u32::try_from(schema).ok())
        .unwrap_or(0)
}

fn legacy_inventory(records: Vec<crate::schema::LegacyBenchRecord>) -> Result<Inventory> {
    inventory(
        records
            .into_iter()
            .map(|record| (record.fixture, record.group, record.bytes, record.output_bytes)),
    )
}

fn calibrated_inventory(records: Vec<CalibratedBenchRecord>) -> Result<Inventory> {
    inventory(
        records
            .into_iter()
            .map(|record| (record.fixture, record.group, record.bytes, record.output_bytes)),
    )
}

fn inventory(records: impl Iterator<Item = (String, String, u64, u64)>) -> Result<Inventory> {
    let mut inventory = HashMap::new();
    for (fixture, group, bytes, output_bytes) in records {
        ensure!(
            inventory
                .insert(fixture.clone(), (group, bytes, output_bytes))
                .is_none(),
            "duplicate fixture {fixture}"
        );
    }
    Ok(inventory)
}

fn validate_campaign(runs: &[RunResults], expected: &Inventory, accept_output_change: bool) -> Result<()> {
    ensure!(
        runs.len() == CALIBRATION_RUNS,
        "calibration requires exactly 40 captures"
    );
    let first = &runs[0];
    validate_sha(&first.sha)?;
    validate_provenance(&first.provenance)?;
    let mut previous_timestamp = None;
    for (index, run) in runs.iter().enumerate() {
        ensure!(
            run.schema == SCHEMA_VERSION,
            "run {index} has unsupported schema {}",
            run.schema
        );
        ensure!(run.sha == first.sha, "run {index} commit differs from the first run");
        ensure!(
            run.provenance == first.provenance,
            "run {index} provenance differs from the first run"
        );
        ensure!(
            !run.hostname.is_empty() && run.hostname != "unknown",
            "run {index} hostname is a placeholder"
        );
        let timestamp = humantime::parse_rfc3339(&run.created_at)
            .with_context(|| format!("run {index} timestamp is not RFC3339"))?;
        if let Some(previous) = previous_timestamp {
            ensure!(timestamp > previous, "run {index} timestamp is not strictly monotonic");
        }
        previous_timestamp = Some(timestamp);
        validate_inventory(run, expected, index, accept_output_change)?;
    }
    Ok(())
}

fn validate_sha(sha: &str) -> Result<()> {
    ensure!(
        sha.len() == 40 && sha.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "calibration commit must be a full SHA"
    );
    ensure!(
        !sha.bytes().all(|byte| byte == b'0'),
        "calibration commit cannot be a placeholder"
    );
    Ok(())
}

fn validate_provenance(provenance: &Provenance) -> Result<()> {
    for (field, value) in [
        ("os", provenance.os.as_str()),
        ("arch", provenance.arch.as_str()),
        ("cpu_model", provenance.cpu_model.as_str()),
        ("rustc_verbose", provenance.rustc_verbose.as_str()),
        ("rustc_host", provenance.rustc_host.as_str()),
        ("cargo_version", provenance.cargo_version.as_str()),
        ("profile", provenance.profile.as_str()),
        ("measurement_mode", provenance.measurement_mode.as_str()),
        ("tier_strategy", provenance.tier_strategy.as_str()),
        ("visitor_mode", provenance.visitor_mode.as_str()),
    ] {
        ensure!(
            !value.is_empty() && value != "unknown",
            "provenance {field} is a placeholder"
        );
    }
    ensure!(provenance.cpu_count > 0, "provenance cpu_count must be positive");
    ensure!(
        !provenance.core_features.is_empty(),
        "provenance core_features is empty"
    );
    ensure!(
        provenance.profile == "release",
        "calibration requires the release profile"
    );
    ensure!(
        provenance.measurement_mode == "nine-batch-median-mad-v2",
        "unsupported measurement mode"
    );
    ensure!(
        matches!(provenance.tier_strategy.as_str(), "auto" | "tier1" | "tier2"),
        "unsupported tier strategy"
    );
    ensure!(
        matches!(provenance.visitor_mode.as_str(), "disabled" | "noop"),
        "unsupported visitor mode"
    );
    ensure!(
        provenance.iteration_override.is_none_or(|iterations| iterations > 0),
        "iteration override must be positive"
    );
    ensure!(
        provenance.warmup_iterations == crate::bench::WARMUP_ITERATIONS
            && provenance.calibration_target_ms == crate::bench::CALIBRATION_TARGET_MS
            && provenance.calibration_timeout_ms == crate::bench::CALIBRATION_TIMEOUT_MS,
        "unsupported measurement settings"
    );
    let mut features = provenance.core_features.clone();
    features.sort();
    features.dedup();
    ensure!(
        features == provenance.core_features,
        "provenance core_features must be sorted and unique"
    );
    for (field, value) in [
        ("runner_image", provenance.runner_image.as_deref()),
        ("runner_class", provenance.runner_class.as_deref()),
    ] {
        if let Some(value) = value {
            ensure!(
                !value.is_empty() && value != "unknown",
                "provenance {field} is a placeholder"
            );
        }
    }
    Ok(())
}

fn validate_inventory(run: &RunResults, expected: &Inventory, index: usize, accept_output_change: bool) -> Result<()> {
    ensure!(
        run.runs.len() == expected.len(),
        "run {index} is not a full-corpus capture"
    );
    let mut seen = HashSet::new();
    for record in &run.runs {
        validate_record(record, index)?;
        ensure!(
            seen.insert(record.fixture.as_str()),
            "run {index} contains duplicate fixture {}",
            record.fixture
        );
        let Some(expected_metadata) = expected.get(&record.fixture) else {
            anyhow::bail!("run {index} fixture metadata differs for {}", record.fixture);
        };
        let (expected_group, expected_bytes, expected_output_bytes) = expected_metadata;
        ensure!(
            *expected_group == record.group && *expected_bytes == record.bytes,
            "run {index} fixture metadata differs for {}",
            record.fixture
        );
        ensure!(
            accept_output_change || *expected_output_bytes == record.output_bytes,
            "run {index} fixture metadata differs for {}",
            record.fixture
        );
    }
    Ok(())
}

fn validate_record(record: &BenchRecord, index: usize) -> Result<()> {
    ensure!(
        record.samples_ms.len() == SAMPLES_PER_RUN,
        "run {index} fixture {} does not have nine samples",
        record.fixture
    );
    ensure!(
        record.samples_ms.iter().all(|value| value.is_finite() && *value >= 0.0),
        "run {index} fixture {} has invalid samples",
        record.fixture
    );
    let recomputed_median = stats::median(&record.samples_ms);
    ensure!(
        stats::approximately_equal(record.median_ms, recomputed_median, recomputed_median),
        "run {index} fixture {} median is corrupt",
        record.fixture
    );
    let recomputed_mad = stats::mad(&record.samples_ms);
    ensure!(
        stats::approximately_equal(record.mad_ms, recomputed_mad, recomputed_median),
        "run {index} fixture {} MAD is corrupt",
        record.fixture
    );
    let legacy = record.samples_ms[..3].iter().copied().fold(f64::INFINITY, f64::min);
    ensure!(
        record.legacy_ms_best == legacy,
        "run {index} fixture {} legacy statistic is corrupt",
        record.fixture
    );
    ensure!(
        record.median_ms > 0.0,
        "run {index} fixture {} median must be positive",
        record.fixture
    );
    Ok(())
}

fn validate_thresholds<'a>(thresholds: &Thresholds, groups: impl Iterator<Item = &'a str>) -> Result<()> {
    for (group, threshold) in thresholds {
        ensure!(
            threshold.max_regression_pct.is_finite() && threshold.max_regression_pct > 0.0,
            "invalid threshold for group {group}"
        );
    }
    for group in groups.collect::<HashSet<_>>() {
        ensure!(
            thresholds.contains_key(group),
            "no threshold configured for group {group}"
        );
    }
    Ok(())
}

fn build_outputs(runs: &[RunResults], thresholds: Thresholds) -> Result<(CalibratedBaseline, Guardrails)> {
    let first = &runs[0];
    let last = &runs[CALIBRATION_RUNS - 1];
    let campaign_id = format!("{}:{}", first.sha, last.created_at);
    let mut baseline_records = Vec::with_capacity(first.runs.len());
    let mut fixture_floors = HashMap::with_capacity(first.runs.len());
    for template in &first.runs {
        let medians = campaign_medians(runs, &template.fixture)?;
        let record = calibrated_record(template, &medians)?;
        fixture_floors.insert(template.fixture.clone(), fixture_floor(&medians)?);
        baseline_records.push(record);
    }
    let baseline = CalibratedBaseline {
        schema: SCHEMA_VERSION,
        campaign_id: campaign_id.clone(),
        sha: first.sha.clone(),
        hostname: last.hostname.clone(),
        created_at: last.created_at.clone(),
        provenance: first.provenance.clone(),
        runs: baseline_records,
    };
    let guardrails = Guardrails {
        schema: SCHEMA_VERSION,
        campaign_id,
        thresholds,
        calibration_provenance: first.provenance.clone(),
        fixture_floors,
    };
    Ok((baseline, guardrails))
}

fn campaign_medians(runs: &[RunResults], fixture: &str) -> Result<Vec<f64>> {
    runs.iter()
        .map(|run| {
            run.runs
                .iter()
                .find(|record| record.fixture == fixture)
                .map(|record| record.median_ms)
                .ok_or_else(|| anyhow::anyhow!("calibration run is missing fixture {fixture}"))
        })
        .collect()
}

fn calibrated_record(template: &BenchRecord, medians: &[f64]) -> Result<CalibratedBenchRecord> {
    let median_ms = stats::median(medians);
    let mad_ms = stats::mad(medians);
    ensure!(
        median_ms.is_finite() && median_ms > 0.0,
        "calibrated median must be positive"
    );
    Ok(CalibratedBenchRecord {
        fixture: template.fixture.clone(),
        group: template.group.clone(),
        bytes: template.bytes,
        median_ms,
        mad_ms,
        mb_per_s: (template.bytes as f64 / 1_048_576.0) / (median_ms / 1_000.0),
        output_bytes: template.output_bytes,
    })
}

fn fixture_floor(medians: &[f64]) -> Result<FixtureFloor> {
    let median_ms = stats::median(medians);
    let mad_ms = stats::mad(medians);
    let pair_deltas: Vec<f64> = medians.chunks_exact(2).map(|pair| (pair[0] - pair[1]).abs()).collect();
    let floor_ms = stats::nearest_rank(&pair_deltas, 0.95);
    ensure!(
        floor_ms.is_finite() && floor_ms > 0.0,
        "calibration produced a non-positive fixture floor"
    );
    Ok(FixtureFloor {
        sample_count: CALIBRATION_RUNS,
        median_ms,
        mad_ms,
        floor_ms,
    })
}

fn promote_pair(baseline: &Path, baseline_json: &[u8], guardrails: &Path, guardrails_json: &[u8]) -> Result<()> {
    let mut rename = |from: &Path, to: &Path| std::fs::rename(from, to);
    promote_pair_with(baseline, baseline_json, guardrails, guardrails_json, &mut rename)
}

fn promote_pair_with(
    baseline: &Path,
    baseline_json: &[u8],
    guardrails: &Path,
    guardrails_json: &[u8],
    rename: &mut impl FnMut(&Path, &Path) -> std::io::Result<()>,
) -> Result<()> {
    let baseline_temp = sibling(baseline, "tmp")?;
    let guardrails_temp = sibling(guardrails, "tmp")?;
    let baseline_backup = sibling(baseline, "bak")?;
    let guardrails_backup = sibling(guardrails, "bak")?;
    cleanup(&[&baseline_temp, &guardrails_temp, &baseline_backup, &guardrails_backup]);
    std::fs::write(&baseline_temp, baseline_json).context("writing baseline temporary")?;
    std::fs::write(&guardrails_temp, guardrails_json).context("writing guardrails temporary")?;
    std::fs::copy(baseline, &baseline_backup).context("backing up baseline")?;
    std::fs::copy(guardrails, &guardrails_backup).context("backing up guardrails")?;
    let promotion = (|| -> std::io::Result<()> {
        std::fs::remove_file(baseline)?;
        rename(&baseline_temp, baseline)?;
        std::fs::remove_file(guardrails)?;
        rename(&guardrails_temp, guardrails)
    })();
    if let Err(error) = promotion {
        cleanup(&[baseline, guardrails]);
        let baseline_restore = rename(&baseline_backup, baseline);
        let guardrails_restore = rename(&guardrails_backup, guardrails);
        cleanup(&[&baseline_temp, &guardrails_temp]);
        baseline_restore.context("restoring baseline after promotion failure")?;
        guardrails_restore.context("restoring guardrails after promotion failure")?;
        return Err(error).context("promoting calibrated baseline and guardrails");
    }
    cleanup(&[&baseline_backup, &guardrails_backup]);
    Ok(())
}

fn sibling(path: &Path, suffix: &str) -> Result<PathBuf> {
    let name = path
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .context("promotion path has no file name")?;
    Ok(path.with_file_name(format!(".{name}.{suffix}")))
}

fn cleanup(paths: &[&Path]) {
    for path in paths {
        if path.exists() {
            let _ = std::fs::remove_file(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};

    use super::{build_outputs, load_schema_v2, promote_pair_with, run_paths, validate_campaign};
    use crate::schema::{BenchRecord, Provenance, RunResults, SCHEMA_VERSION, default_thresholds};

    #[test]
    fn should_derive_floor_and_shared_campaign_id() {
        let runs = campaign();
        let (baseline, guardrails) = build_outputs(&runs, default_thresholds()).unwrap();
        assert_eq!(guardrails.fixture_floors["fixture.html"].floor_ms, 1.0);
        assert_eq!(baseline.campaign_id, guardrails.campaign_id);
        assert!(baseline.runs[0].median_ms > 0.0);
    }

    #[test]
    fn should_terminate_promoted_json_with_a_newline() {
        let runs = campaign();
        let (baseline, guardrails) = build_outputs(&runs, default_thresholds()).unwrap();
        for bytes in [
            super::to_json_line_terminated(&baseline).unwrap(),
            super::to_json_line_terminated(&guardrails).unwrap(),
        ] {
            assert_eq!(bytes.last(), Some(&b'\n'), "promoted JSON must end with a newline");
        }
    }

    #[test]
    fn should_reject_campaign_with_mismatched_provenance() {
        let mut runs = campaign();
        runs[20].provenance.cpu_count = 4;
        let error = validate_campaign(&runs, &expected_inventory(), false).unwrap_err();
        assert_eq!(error.to_string(), "run 20 provenance differs from the first run");
    }

    #[test]
    fn should_reject_unsupported_measurement_settings() {
        let mut runs = campaign();
        for run in &mut runs {
            run.provenance.calibration_target_ms = 1;
        }
        let error = validate_campaign(&runs, &expected_inventory(), false).unwrap_err();
        assert_eq!(error.to_string(), "unsupported measurement settings");
    }

    #[test]
    fn should_reject_changed_output_bytes_unless_opted_in() {
        let runs = campaign();
        let mut expected = expected_inventory();
        expected.insert("fixture.html".to_owned(), ("clean_small".to_owned(), 10, 9));

        let error = validate_campaign(&runs, &expected, false).unwrap_err();
        assert_eq!(error.to_string(), "run 0 fixture metadata differs for fixture.html");
        assert!(validate_campaign(&runs, &expected, true).is_ok());
    }

    #[test]
    fn should_reject_changed_input_bytes_even_when_accepting_output_change() {
        let runs = campaign();
        let mut expected = expected_inventory();
        expected.insert("fixture.html".to_owned(), ("clean_small".to_owned(), 11, 5));

        let error = validate_campaign(&runs, &expected, true).unwrap_err();
        assert_eq!(error.to_string(), "run 0 fixture metadata differs for fixture.html");
    }

    #[test]
    fn should_reject_unknown_fixture_even_when_accepting_output_change() {
        let runs = campaign();
        let mut expected = expected_inventory();
        expected.remove("fixture.html");
        expected.insert("renamed.html".to_owned(), ("clean_small".to_owned(), 10, 5));

        let error = validate_campaign(&runs, &expected, true).unwrap_err();
        assert_eq!(error.to_string(), "run 0 fixture metadata differs for fixture.html");
    }

    #[test]
    fn should_reject_schema_v1_calibration_capture() {
        let path = unique_path("schema-v1");
        std::fs::write(&path, r#"{"schema":1}"#).unwrap();
        let error = load_schema_v2(&path, "calibration result").unwrap_err();
        std::fs::remove_file(&path).unwrap();
        assert!(error.to_string().contains("unsupported calibration result schema 1"));
    }

    #[test]
    fn should_restore_both_files_when_second_promotion_rename_fails() {
        let directory = unique_path("promotion");
        std::fs::create_dir(&directory).unwrap();
        let baseline = directory.join("baseline.json");
        let guardrails = directory.join("guardrails.json");
        std::fs::write(&baseline, b"old baseline").unwrap();
        std::fs::write(&guardrails, b"old guardrails").unwrap();
        let mut calls = 0;
        let mut rename = |from: &Path, to: &Path| {
            calls += 1;
            if calls == 2 {
                Err(std::io::Error::other("injected second rename failure"))
            } else {
                std::fs::rename(from, to)
            }
        };
        assert!(promote_pair_with(&baseline, b"new baseline", &guardrails, b"new guardrails", &mut rename).is_err());
        assert_eq!(std::fs::read(&baseline).unwrap(), b"old baseline");
        assert_eq!(std::fs::read(&guardrails).unwrap(), b"old guardrails");
        std::fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn should_require_strict_zero_padded_capture_order() {
        let directory = unique_path("ordered-runs");
        std::fs::create_dir(&directory).unwrap();
        for index in 1..=40 {
            std::fs::write(directory.join(format!("{index:04}.json")), b"{}").unwrap();
        }
        assert_eq!(run_paths(&directory).unwrap().len(), 40);
        std::fs::rename(directory.join("0040.json"), directory.join("0041.json")).unwrap();
        assert!(run_paths(&directory).unwrap_err().to_string().contains("0040.json"));
        std::fs::remove_dir_all(directory).unwrap();
    }

    fn campaign() -> Vec<RunResults> {
        (0..40)
            .map(|index| run(if index == 37 { 20.0 } else { index as f64 + 1.0 }, index))
            .collect()
    }

    fn run(median_ms: f64, index: usize) -> RunResults {
        RunResults {
            schema: SCHEMA_VERSION,
            sha: "a".repeat(40),
            hostname: "host".to_owned(),
            created_at: format!("2026-01-01T00:00:{index:02}Z"),
            provenance: provenance(),
            runs: vec![BenchRecord {
                fixture: "fixture.html".to_owned(),
                group: "clean_small".to_owned(),
                bytes: 10,
                samples_ms: vec![median_ms; 9],
                median_ms,
                mad_ms: 0.0,
                legacy_ms_best: median_ms,
                mb_per_s: 1.0,
                output_bytes: 5,
            }],
        }
    }

    fn expected_inventory() -> HashMap<String, (String, u64, u64)> {
        HashMap::from([("fixture.html".to_owned(), ("clean_small".to_owned(), 10, 5))])
    }

    fn provenance() -> Provenance {
        Provenance {
            os: "linux".to_owned(),
            arch: "x86_64".to_owned(),
            cpu_model: "cpu".to_owned(),
            cpu_count: 2,
            rustc_verbose: "rustc".to_owned(),
            rustc_host: "host".to_owned(),
            cargo_version: "cargo".to_owned(),
            profile: "release".to_owned(),
            build_flags: String::new(),
            measurement_mode: "nine-batch-median-mad-v2".to_owned(),
            tier_strategy: "auto".to_owned(),
            visitor_mode: "disabled".to_owned(),
            iteration_override: None,
            warmup_iterations: crate::bench::WARMUP_ITERATIONS,
            calibration_target_ms: crate::bench::CALIBRATION_TARGET_MS,
            calibration_timeout_ms: crate::bench::CALIBRATION_TIMEOUT_MS,
            core_features: vec!["metadata".to_owned()],
            runner_image: None,
            runner_class: None,
        }
    }

    fn unique_path(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("htmbench-{label}-{unique}"))
    }
}
