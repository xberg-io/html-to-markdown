//! Regression policy evaluation for legacy and calibrated configurations.

use std::collections::{HashMap, HashSet};

use anyhow::{Result, bail, ensure};

use crate::schema::{
    BenchRecord, CalibratedBaseline, CalibratedBenchRecord, GroupThreshold, Guardrails, LegacyGuardrails,
    LegacyRunResults, RunResults, SAMPLES_PER_RUN, SCHEMA_VERSION,
};
use crate::stats;

/// One fixture comparison and its effective allowance.
#[derive(Debug, Clone, PartialEq)]
pub struct Comparison {
    /// Fixture key.
    pub fixture: String,
    /// Baseline timing.
    pub baseline_ms: f64,
    /// Current method-compatible timing.
    pub current_ms: f64,
    /// Policy percent for the fixture group.
    pub threshold_pct: f64,
    /// Effective absolute allowance.
    pub allowed_delta_ms: f64,
    /// Whether the positive delta exceeds the allowance.
    pub failed: bool,
}

/// Outcome of a strict evaluation.
///
/// ~keep Carries the timing comparisons and the inventory differences *together* rather than
/// letting the first inventory difference abort the run. `compare` previously bailed on inventory,
/// so any release that legitimately changed a fixture's output reported only "fixture metadata
/// differs" and never evaluated a single timing -- a genuine regression riding along with an
/// accepted output change could not be seen. Both halves are computed in one pass; both are fatal.
#[derive(Debug, Clone)]
pub struct Evaluation {
    /// Per-fixture timing comparisons, for every fixture present in both documents.
    pub comparisons: Vec<Comparison>,
    /// Human-readable inventory differences; empty when the inventories match exactly.
    pub inventory_mismatches: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Metadata<'a> {
    group: &'a str,
    bytes: u64,
    output_bytes: u64,
}

/// Host-identity difference between a capture and the host the baseline was calibrated on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostMismatch {
    /// CPU model recorded by the calibration campaign.
    pub calibrated_cpu_model: String,
    /// CPU model of the host that produced the capture.
    pub current_cpu_model: String,
    /// Logical CPU count recorded by the calibration campaign.
    pub calibrated_cpu_count: usize,
    /// Logical CPU count of the host that produced the capture.
    pub current_cpu_count: usize,
}

/// Report the host-identity difference between a capture and its calibrated guardrails.
///
/// Host identity is excluded from the provenance contract (see [`Provenance::contract_matches`]),
/// so it can never abort the comparison. It is reported separately because it is the only
/// legitimate reason to downgrade a timing violation from fatal to advisory: on hardware the
/// baseline was never measured on, a positive delta is not evidence of a code regression.
///
/// [`Provenance::contract_matches`]: crate::schema::Provenance::contract_matches
#[must_use]
pub fn host_mismatch(results: &RunResults, guardrails: &Guardrails) -> Option<HostMismatch> {
    let calibrated = &guardrails.calibration_provenance;
    let current = &results.provenance;
    if current.host_matches(calibrated) {
        return None;
    }
    Some(HostMismatch {
        calibrated_cpu_model: calibrated.cpu_model.clone(),
        current_cpu_model: current.cpu_model.clone(),
        calibrated_cpu_count: calibrated.cpu_count,
        current_cpu_count: current.cpu_count,
    })
}

/// Evaluate a schema-v2 capture against an approved calibrated baseline.
pub fn evaluate_strict(
    results: &RunResults,
    baseline: &CalibratedBaseline,
    guardrails: &Guardrails,
) -> Result<Evaluation> {
    let inventory_mismatches = validate_strict_documents(results, baseline, guardrails)?;
    let baseline_map = calibrated_map(&baseline.runs)?;
    // ~keep Indexing `baseline_map` directly would panic for a fixture the baseline lacks, which
    // was unreachable only while an inventory mismatch aborted first. It no longer does.
    let comparisons = results
        .runs
        .iter()
        .filter_map(|record| {
            baseline_map
                .get(record.fixture.as_str())
                .map(|calibrated| strict_record(record, calibrated, guardrails))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Evaluation {
        comparisons,
        inventory_mismatches,
    })
}

/// Evaluate with the temporary schema-v1 percentage-only migration bridge.
pub fn evaluate_legacy(
    results: &RunResults,
    baseline: &LegacyRunResults,
    guardrails: &LegacyGuardrails,
) -> Result<Vec<Comparison>> {
    ensure_schema(results.schema, "results")?;
    ensure!(
        baseline.schema == 1,
        "unsupported baseline schema {}; expected 1",
        baseline.schema
    );
    ensure!(
        guardrails.schema == 1,
        "unsupported guardrails schema {}; expected 1",
        guardrails.schema
    );
    let result_inventory = validate_results(results)?;
    validate_thresholds(
        &guardrails.thresholds,
        result_inventory.values().map(|metadata| metadata.group),
    )?;
    let baseline_map = legacy_map(baseline)?;
    ensure_inventory_matches(&result_inventory, &legacy_metadata(&baseline_map)?)?;
    results
        .runs
        .iter()
        .map(|record| legacy_record(record, baseline_map[record.fixture.as_str()].ms_best, guardrails))
        .collect()
}

fn validate_strict_documents(
    results: &RunResults,
    baseline: &CalibratedBaseline,
    guardrails: &Guardrails,
) -> Result<Vec<String>> {
    ensure_schema(results.schema, "results")?;
    ensure_schema(baseline.schema, "baseline")?;
    ensure_schema(guardrails.schema, "guardrails")?;
    ensure!(!baseline.campaign_id.is_empty(), "baseline campaign_id is empty");
    ensure!(
        baseline.campaign_id == guardrails.campaign_id,
        "baseline and guardrails campaign_id differ"
    );
    ensure!(
        results.provenance.contract_matches(&guardrails.calibration_provenance),
        "benchmark provenance mismatch"
    );
    // ~keep Stays exact equality, unlike the results check above: baseline and guardrails are
    // written as one pair by a single `calibrate` run on one host, so any difference between them
    // — host identity included — means the checked-in pair was hand-edited or mixed across
    // campaigns, not that CI drew a different machine.
    ensure!(
        baseline.provenance == guardrails.calibration_provenance,
        "baseline provenance mismatch"
    );
    let result_inventory = validate_results(results)?;
    let baseline_inventory = validate_calibrated_baseline(baseline)?;
    let mismatches = inventory_mismatches(&result_inventory, &baseline_inventory);
    validate_guardrails(guardrails, &result_inventory)?;
    Ok(mismatches)
}

fn validate_results(results: &RunResults) -> Result<HashMap<&str, Metadata<'_>>> {
    ensure!(!results.runs.is_empty(), "results fixture inventory is empty");
    let mut inventory = HashMap::with_capacity(results.runs.len());
    for record in &results.runs {
        validate_record_statistics(record)?;
        let metadata = Metadata {
            group: &record.group,
            bytes: record.bytes,
            output_bytes: record.output_bytes,
        };
        ensure!(
            inventory.insert(record.fixture.as_str(), metadata).is_none(),
            "duplicate result fixture {}",
            record.fixture
        );
    }
    Ok(inventory)
}

fn validate_record_statistics(record: &BenchRecord) -> Result<()> {
    ensure!(
        record.samples_ms.len() == SAMPLES_PER_RUN,
        "fixture {} must have nine samples",
        record.fixture
    );
    ensure!(
        record
            .samples_ms
            .iter()
            .all(|sample| sample.is_finite() && *sample >= 0.0),
        "fixture {} has invalid samples",
        record.fixture
    );
    let recomputed_median = stats::median(&record.samples_ms);
    ensure!(
        stats::approximately_equal(record.median_ms, recomputed_median, recomputed_median),
        "fixture {} median is corrupt",
        record.fixture
    );
    let recomputed_mad = stats::mad(&record.samples_ms);
    ensure!(
        stats::approximately_equal(record.mad_ms, recomputed_mad, recomputed_median),
        "fixture {} MAD is corrupt",
        record.fixture
    );
    let legacy_ms_best = record.samples_ms[..3].iter().copied().fold(f64::INFINITY, f64::min);
    ensure!(
        record.legacy_ms_best == legacy_ms_best,
        "fixture {} legacy statistic is corrupt",
        record.fixture
    );
    ensure!(
        record.mb_per_s.is_finite() && record.mb_per_s >= 0.0,
        "fixture {} throughput is invalid",
        record.fixture
    );
    Ok(())
}

fn validate_calibrated_baseline(baseline: &CalibratedBaseline) -> Result<HashMap<&str, Metadata<'_>>> {
    ensure!(!baseline.runs.is_empty(), "baseline fixture inventory is empty");
    let mut inventory = HashMap::with_capacity(baseline.runs.len());
    for record in &baseline.runs {
        ensure!(
            record.median_ms.is_finite() && record.median_ms > 0.0,
            "baseline {} median must be positive",
            record.fixture
        );
        ensure!(
            record.mad_ms.is_finite() && record.mad_ms >= 0.0,
            "baseline {} MAD is invalid",
            record.fixture
        );
        let metadata = Metadata {
            group: &record.group,
            bytes: record.bytes,
            output_bytes: record.output_bytes,
        };
        ensure!(
            inventory.insert(record.fixture.as_str(), metadata).is_none(),
            "duplicate baseline fixture {}",
            record.fixture
        );
    }
    Ok(inventory)
}

fn validate_guardrails(guardrails: &Guardrails, inventory: &HashMap<&str, Metadata<'_>>) -> Result<()> {
    validate_thresholds(
        &guardrails.thresholds,
        inventory.values().map(|metadata| metadata.group),
    )?;
    ensure!(
        guardrails.fixture_floors.len() == inventory.len(),
        "guardrail fixture inventory is not exact"
    );
    for fixture in inventory.keys() {
        let floor = guardrails
            .fixture_floors
            .get(*fixture)
            .ok_or_else(|| anyhow::anyhow!("missing floor for {fixture}"))?;
        ensure!(
            floor.sample_count == 40,
            "floor for {fixture} was not derived from 40 runs"
        );
        ensure!(
            floor.median_ms.is_finite() && floor.median_ms > 0.0,
            "floor median for {fixture} must be positive"
        );
        ensure!(
            floor.mad_ms.is_finite() && floor.mad_ms >= 0.0,
            "floor MAD for {fixture} is invalid"
        );
        ensure!(
            floor.floor_ms.is_finite() && floor.floor_ms > 0.0,
            "floor for {fixture} must be positive"
        );
    }
    Ok(())
}

fn validate_thresholds<'a>(
    thresholds: &HashMap<String, GroupThreshold>,
    groups: impl Iterator<Item = &'a str>,
) -> Result<()> {
    for (group, value) in thresholds {
        ensure!(
            value.max_regression_pct.is_finite() && value.max_regression_pct > 0.0,
            "threshold for group {group} must be positive and finite"
        );
    }
    let groups: HashSet<&str> = groups.collect();
    for group in groups {
        ensure!(
            thresholds.contains_key(group),
            "no threshold configured for group {group}"
        );
    }
    Ok(())
}

/// Collect every inventory difference instead of stopping at the first.
///
/// Returned messages are sorted so a CI log diffs cleanly between runs.
fn inventory_mismatches(current: &HashMap<&str, Metadata<'_>>, baseline: &HashMap<&str, Metadata<'_>>) -> Vec<String> {
    let mut mismatches = Vec::new();
    if current.len() != baseline.len() {
        mismatches.push(format!(
            "fixture inventories differ in size: {} in results, {} in baseline",
            current.len(),
            baseline.len()
        ));
    }
    for (fixture, metadata) in current {
        match baseline.get(fixture) {
            Some(baseline_metadata) if baseline_metadata == metadata => {}
            Some(_) => mismatches.push(format!("fixture metadata differs for {fixture}")),
            None => mismatches.push(format!("fixture {fixture} is absent from the baseline")),
        }
    }
    for fixture in baseline.keys() {
        if !current.contains_key(fixture) {
            mismatches.push(format!("baseline fixture {fixture} is absent from the results"));
        }
    }
    mismatches.sort();
    mismatches
}

/// Inventory equality as a hard error, for the schema-v1 bridge that has no richer reporting.
fn ensure_inventory_matches(
    current: &HashMap<&str, Metadata<'_>>,
    baseline: &HashMap<&str, Metadata<'_>>,
) -> Result<()> {
    let mismatches = inventory_mismatches(current, baseline);
    ensure!(mismatches.is_empty(), "{}", mismatches.join("; "));
    Ok(())
}

fn calibrated_map(records: &[CalibratedBenchRecord]) -> Result<HashMap<&str, &CalibratedBenchRecord>> {
    let mut map = HashMap::with_capacity(records.len());
    for record in records {
        ensure!(
            map.insert(record.fixture.as_str(), record).is_none(),
            "duplicate baseline fixture {}",
            record.fixture
        );
    }
    Ok(map)
}

fn legacy_map(baseline: &LegacyRunResults) -> Result<HashMap<&str, &crate::schema::LegacyBenchRecord>> {
    let mut map = HashMap::with_capacity(baseline.runs.len());
    for record in &baseline.runs {
        ensure!(
            record.ms_best.is_finite() && record.ms_best > 0.0,
            "legacy baseline {} must be positive",
            record.fixture
        );
        ensure!(
            map.insert(record.fixture.as_str(), record).is_none(),
            "duplicate legacy baseline fixture {}",
            record.fixture
        );
    }
    Ok(map)
}

fn legacy_metadata<'a>(
    records: &HashMap<&'a str, &'a crate::schema::LegacyBenchRecord>,
) -> Result<HashMap<&'a str, Metadata<'a>>> {
    Ok(records
        .iter()
        .map(|(fixture, record)| {
            (
                *fixture,
                Metadata {
                    group: &record.group,
                    bytes: record.bytes,
                    output_bytes: record.output_bytes,
                },
            )
        })
        .collect())
}

fn strict_record(
    record: &BenchRecord,
    baseline: &CalibratedBenchRecord,
    guardrails: &Guardrails,
) -> Result<Comparison> {
    let floor = &guardrails.fixture_floors[&record.fixture];
    let threshold_pct = guardrails.thresholds[&record.group].max_regression_pct;
    let allowance = (baseline.median_ms * threshold_pct / 100.0).max(floor.floor_ms);
    Ok(comparison(
        record.fixture.clone(),
        baseline.median_ms,
        record.median_ms,
        threshold_pct,
        allowance,
    ))
}

fn legacy_record(record: &BenchRecord, baseline_ms: f64, guardrails: &LegacyGuardrails) -> Result<Comparison> {
    let threshold_pct = guardrails.thresholds[&record.group].max_regression_pct;
    Ok(comparison(
        record.fixture.clone(),
        baseline_ms,
        record.legacy_ms_best,
        threshold_pct,
        baseline_ms * threshold_pct / 100.0,
    ))
}

fn comparison(fixture: String, baseline_ms: f64, current_ms: f64, threshold_pct: f64, allowance: f64) -> Comparison {
    Comparison {
        fixture,
        baseline_ms,
        current_ms,
        threshold_pct,
        allowed_delta_ms: allowance,
        failed: current_ms - baseline_ms > allowance,
    }
}

fn ensure_schema(schema: u32, kind: &str) -> Result<()> {
    if schema != SCHEMA_VERSION {
        bail!("unsupported {kind} schema {schema}; expected {SCHEMA_VERSION}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{HostMismatch, evaluate_legacy, evaluate_strict, host_mismatch, validate_record_statistics};
    use crate::schema::{
        BenchRecord, CalibratedBaseline, CalibratedBenchRecord, FixtureFloor, Guardrails, LegacyBenchRecord,
        LegacyGuardrails, LegacyRunResults, Provenance, RunResults, SCHEMA_VERSION, default_thresholds,
    };

    #[test]
    fn should_allow_tiny_fixture_delta_within_measured_floor() {
        let (results, baseline, guardrails) = scenario(vec![1.08; 9], 0.10);
        let comparison = evaluate_strict(&results, &baseline, &guardrails)
            .unwrap()
            .comparisons
            .remove(0);
        assert!(!comparison.failed);
        assert_eq!(comparison.allowed_delta_ms, 0.10);
    }

    #[test]
    fn should_fail_material_delta_beyond_floor_and_percentage() {
        let (results, baseline, guardrails) = scenario(vec![1.11; 9], 0.10);
        assert!(evaluate_strict(&results, &baseline, &guardrails).unwrap().comparisons[0].failed);
    }

    #[test]
    fn should_reject_partial_inventory() {
        let (mut results, baseline, guardrails) = scenario(vec![1.0; 9], 0.10);
        results.runs.clear();
        assert_eq!(
            evaluate_strict(&results, &baseline, &guardrails)
                .unwrap_err()
                .to_string(),
            "results fixture inventory is empty"
        );
    }

    #[test]
    fn should_reject_duplicate_fixture() {
        let (mut results, baseline, guardrails) = scenario(vec![1.0; 9], 0.10);
        results.runs.push(results.runs[0].clone());
        assert!(
            evaluate_strict(&results, &baseline, &guardrails)
                .unwrap_err()
                .to_string()
                .contains("duplicate result fixture")
        );
    }

    /// ~keep An inventory mismatch must not hide the timing verdict. `compare` used to bail on
    /// the first differing fixture, so a release whose output legitimately changed reported
    /// "fixture metadata differs" and never evaluated timings at all -- a real regression shipping
    /// alongside an accepted output change would have been invisible. Both verdicts are now
    /// produced in one pass and both are fatal.
    #[test]
    fn should_report_timing_regression_alongside_inventory_mismatch() {
        let (mut results, baseline, guardrails) = scenario(vec![5.0; 9], 0.10);
        results.runs[0].output_bytes = 6;

        let evaluation = evaluate_strict(&results, &baseline, &guardrails)
            .expect("an inventory mismatch must not abort the evaluation");

        assert_eq!(
            evaluation.inventory_mismatches,
            vec!["fixture metadata differs for fixture.html".to_owned()],
            "the inventory mismatch must still be reported"
        );
        assert_eq!(
            evaluation.comparisons.len(),
            1,
            "the timing comparison must still be computed"
        );
        assert!(
            evaluation.comparisons[0].failed,
            "a 1.0ms -> 5.0ms regression must be reported even though inventory differs"
        );
    }

    #[test]
    fn should_report_altered_group_metadata() {
        let (mut results, baseline, guardrails) = scenario(vec![1.0; 9], 0.10);
        results.runs[0].group = "clean_medium".to_owned();
        assert_eq!(
            evaluate_strict(&results, &baseline, &guardrails)
                .unwrap()
                .inventory_mismatches,
            vec!["fixture metadata differs for fixture.html".to_owned()]
        );
    }

    #[test]
    fn should_report_altered_input_and_output_metadata() {
        let (mut results, baseline, guardrails) = scenario(vec![1.0; 9], 0.10);
        results.runs[0].bytes = 11;
        assert_eq!(
            evaluate_strict(&results, &baseline, &guardrails)
                .unwrap()
                .inventory_mismatches,
            vec!["fixture metadata differs for fixture.html".to_owned()]
        );
        results.runs[0].bytes = 10;
        results.runs[0].output_bytes = 6;
        assert_eq!(
            evaluate_strict(&results, &baseline, &guardrails)
                .unwrap()
                .inventory_mismatches,
            vec!["fixture metadata differs for fixture.html".to_owned()]
        );
    }

    /// A clean capture must report no inventory differences at all -- the negative control for
    /// the three tests above, so an always-empty mismatch list cannot masquerade as a pass. ~keep
    #[test]
    fn should_report_no_inventory_mismatch_for_a_matching_capture() {
        let (results, baseline, guardrails) = scenario(vec![1.0; 9], 0.10);
        let evaluation = evaluate_strict(&results, &baseline, &guardrails).unwrap();
        assert!(evaluation.inventory_mismatches.is_empty());
        assert_eq!(evaluation.comparisons.len(), 1);
    }

    #[test]
    fn should_reject_corrupt_and_nan_statistics() {
        let (mut results, baseline, guardrails) = scenario(vec![1.0; 9], 0.10);
        results.runs[0].median_ms = f64::NAN;
        assert!(
            evaluate_strict(&results, &baseline, &guardrails)
                .unwrap_err()
                .to_string()
                .contains("median is corrupt")
        );
        results.runs[0].samples_ms[0] = f64::NAN;
        assert!(
            evaluate_strict(&results, &baseline, &guardrails)
                .unwrap_err()
                .to_string()
                .contains("invalid samples")
        );
    }

    #[test]
    fn should_accept_ci_round_trip_mad_for_vuejs_docs() {
        let record: BenchRecord = serde_json::from_str(
            r#"{
                "fixture":"mdream/vuejs-docs.html","group":"clean_medium","bytes":112232,
                "samples_ms":[10.3192925,10.148774166666666,10.023908833333333,
                    10.064827833333334,10.046842833333335,10.026310666666665,
                    10.018591,10.080160666666666,10.033060666666666],
                "median_ms":10.046842833333335,"mad_ms":0.02293400000000112,
                "legacy_ms_best":10.023908833333333,"mb_per_s":10.653374164846468,
                "output_bytes":26014
            }"#,
        )
        .unwrap();
        validate_record_statistics(&record).unwrap();
    }

    #[test]
    fn should_reject_missing_floor_nonpositive_baseline_and_unconfigured_threshold() {
        let (results, mut baseline, mut guardrails) = scenario(vec![1.0; 9], 0.10);
        guardrails.fixture_floors.clear();
        assert!(
            evaluate_strict(&results, &baseline, &guardrails)
                .unwrap_err()
                .to_string()
                .contains("inventory is not exact")
        );
        guardrails = scenario(vec![1.0; 9], 0.10).2;
        baseline.runs[0].median_ms = 0.0;
        assert!(
            evaluate_strict(&results, &baseline, &guardrails)
                .unwrap_err()
                .to_string()
                .contains("must be positive")
        );
        baseline = scenario(vec![1.0; 9], 0.10).1;
        guardrails.thresholds.remove("clean_small");
        assert!(
            evaluate_strict(&results, &baseline, &guardrails)
                .unwrap_err()
                .to_string()
                .contains("no threshold configured")
        );
    }

    #[test]
    fn should_reject_campaign_mismatch_and_nonpositive_floor() {
        let (results, baseline, mut guardrails) = scenario(vec![1.0; 9], 0.10);
        guardrails.campaign_id = "different".to_owned();
        assert_eq!(
            evaluate_strict(&results, &baseline, &guardrails)
                .unwrap_err()
                .to_string(),
            "baseline and guardrails campaign_id differ"
        );
        guardrails = scenario(vec![1.0; 9], 0.10).2;
        guardrails.fixture_floors.get_mut("fixture.html").unwrap().floor_ms = 0.0;
        assert!(
            evaluate_strict(&results, &baseline, &guardrails)
                .unwrap_err()
                .to_string()
                .contains("floor for fixture.html must be positive")
        );
    }

    #[test]
    fn should_reject_measurement_setting_mismatch() {
        let (mut results, baseline, guardrails) = scenario(vec![1.0; 9], 0.10);
        results.provenance.iteration_override = Some(100);
        assert_eq!(
            evaluate_strict(&results, &baseline, &guardrails)
                .unwrap_err()
                .to_string(),
            "benchmark provenance mismatch"
        );
    }

    #[test]
    fn should_evaluate_timings_when_only_host_identity_differs() {
        let (mut results, baseline, guardrails) = scenario(vec![1.11; 9], 0.10);
        results.provenance.cpu_model = "Other Vendor CPU".to_owned();
        results.provenance.cpu_count = 8;
        let comparisons = evaluate_strict(&results, &baseline, &guardrails).unwrap().comparisons;
        assert_eq!(comparisons.len(), 1);
        assert!(comparisons[0].failed);
        assert_eq!(
            host_mismatch(&results, &guardrails),
            Some(HostMismatch {
                calibrated_cpu_model: "cpu".to_owned(),
                current_cpu_model: "Other Vendor CPU".to_owned(),
                calibrated_cpu_count: 2,
                current_cpu_count: 8,
            })
        );
    }

    #[test]
    fn should_report_no_host_mismatch_on_the_calibrated_host() {
        let (results, _baseline, guardrails) = scenario(vec![1.11; 9], 0.10);
        assert_eq!(host_mismatch(&results, &guardrails), None);
    }

    #[test]
    fn should_reject_toolchain_and_build_configuration_drift() {
        for mutate in [
            (|provenance: &mut Provenance| provenance.rustc_verbose = "rustc 1.0.0".to_owned()) as fn(&mut Provenance),
            |provenance: &mut Provenance| provenance.cargo_version = "cargo 1.0.0".to_owned(),
            |provenance: &mut Provenance| provenance.profile = "debug".to_owned(),
            |provenance: &mut Provenance| provenance.build_flags = "-C target-cpu=native".to_owned(),
            |provenance: &mut Provenance| provenance.core_features = vec!["serde".to_owned()],
            |provenance: &mut Provenance| provenance.measurement_mode = "eight-batch".to_owned(),
            |provenance: &mut Provenance| provenance.tier_strategy = "tier1".to_owned(),
            |provenance: &mut Provenance| provenance.visitor_mode = "noop".to_owned(),
            |provenance: &mut Provenance| provenance.runner_image = Some("ubuntu22".to_owned()),
            |provenance: &mut Provenance| provenance.runner_class = Some("self-hosted".to_owned()),
            |provenance: &mut Provenance| provenance.os = "macos".to_owned(),
            |provenance: &mut Provenance| provenance.arch = "aarch64".to_owned(),
        ] {
            let (mut results, baseline, guardrails) = scenario(vec![1.0; 9], 0.10);
            mutate(&mut results.provenance);
            assert_eq!(
                evaluate_strict(&results, &baseline, &guardrails)
                    .unwrap_err()
                    .to_string(),
                "benchmark provenance mismatch"
            );
            assert_eq!(host_mismatch(&results, &guardrails), None);
        }
    }

    #[test]
    fn should_use_best_of_first_three_for_legacy_equivalence() {
        let samples = vec![1.2, 1.0, 1.1, 9.0, 9.0, 9.0, 9.0, 9.0, 9.0];
        let current = result(samples);
        let baseline = legacy_baseline();
        let guardrails = LegacyGuardrails {
            schema: 1,
            thresholds: default_thresholds(),
        };
        let comparison = evaluate_legacy(&current, &baseline, &guardrails).unwrap().remove(0);
        assert_eq!(comparison.current_ms, 1.0);
        assert!(!comparison.failed);
    }

    #[test]
    fn should_preserve_policy_thresholds() {
        let thresholds = default_thresholds();
        assert_eq!(thresholds["clean_large"].max_regression_pct, 5.0);
        assert_eq!(thresholds["clean_medium"].max_regression_pct, 8.0);
        assert_eq!(thresholds["clean_small"].max_regression_pct, 10.0);
        assert_eq!(thresholds["adversarial"].max_regression_pct, 30.0);
    }

    fn scenario(samples: Vec<f64>, floor_ms: f64) -> (RunResults, CalibratedBaseline, Guardrails) {
        let provenance = provenance();
        let results = result(samples);
        let baseline = CalibratedBaseline {
            schema: SCHEMA_VERSION,
            campaign_id: "campaign".to_owned(),
            sha: "a".repeat(40),
            hostname: "host".to_owned(),
            created_at: "2026-01-01T00:00:00Z".to_owned(),
            provenance: provenance.clone(),
            runs: vec![CalibratedBenchRecord {
                fixture: "fixture.html".to_owned(),
                group: "clean_small".to_owned(),
                bytes: 10,
                median_ms: 1.0,
                mad_ms: 0.01,
                mb_per_s: 1.0,
                output_bytes: 5,
            }],
        };
        let guardrails = Guardrails {
            schema: SCHEMA_VERSION,
            campaign_id: "campaign".to_owned(),
            thresholds: default_thresholds(),
            calibration_provenance: provenance,
            fixture_floors: HashMap::from([(
                "fixture.html".to_owned(),
                FixtureFloor {
                    sample_count: 40,
                    median_ms: 1.0,
                    mad_ms: 0.01,
                    floor_ms,
                },
            )]),
        };
        (results, baseline, guardrails)
    }

    fn result(samples_ms: Vec<f64>) -> RunResults {
        let median_ms = crate::stats::median(&samples_ms);
        let mad_ms = crate::stats::mad(&samples_ms);
        let legacy_ms_best = samples_ms[..3].iter().copied().fold(f64::INFINITY, f64::min);
        RunResults {
            schema: SCHEMA_VERSION,
            sha: "a".repeat(40),
            hostname: "host".to_owned(),
            created_at: "2026-01-01T00:00:01Z".to_owned(),
            provenance: provenance(),
            runs: vec![BenchRecord {
                fixture: "fixture.html".to_owned(),
                group: "clean_small".to_owned(),
                bytes: 10,
                samples_ms,
                median_ms,
                mad_ms,
                legacy_ms_best,
                mb_per_s: 1.0,
                output_bytes: 5,
            }],
        }
    }

    fn legacy_baseline() -> LegacyRunResults {
        LegacyRunResults {
            schema: 1,
            sha: "old".to_owned(),
            host: "unknown".to_owned(),
            created_at: "then".to_owned(),
            runs: vec![LegacyBenchRecord {
                fixture: "fixture.html".to_owned(),
                group: "clean_small".to_owned(),
                bytes: 10,
                ms_best: 1.0,
                mb_per_s: 1.0,
                output_bytes: 5,
            }],
        }
    }

    fn provenance() -> Provenance {
        Provenance {
            os: "linux".to_owned(),
            arch: "x86_64".to_owned(),
            cpu_model: "cpu".to_owned(),
            cpu_count: 2,
            rustc_verbose: "rustc".to_owned(),
            rustc_host: "x86_64-unknown-linux-gnu".to_owned(),
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
            runner_image: Some("ubuntu24".to_owned()),
            runner_class: Some("github-hosted".to_owned()),
        }
    }
}
