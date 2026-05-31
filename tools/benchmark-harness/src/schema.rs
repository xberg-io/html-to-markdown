//! JSON schema types for benchmark results, baselines, and guardrails.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Schema version for result files.
pub const SCHEMA_VERSION: u32 = 1;

/// Result of a single fixture benchmark run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchRecord {
    /// Relative path of the fixture file (from the fixtures root).
    pub fixture: String,
    /// Group tag from `groups.toml` (e.g. `clean_small`).
    pub group: String,
    /// Input HTML size in bytes.
    pub bytes: u64,
    /// Best observed wall-clock milliseconds per call.
    pub ms_best: f64,
    /// Throughput in MB/s derived from `bytes / ms_best`.
    pub mb_per_s: f64,
    /// Output Markdown byte length.
    pub output_bytes: u64,
    /// Optional mdream best-of-3 ms (populated with `--mdream`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdream_ms_best: Option<f64>,
}

/// Top-level JSON document written by `htmbench run`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunResults {
    /// Schema version; always [`SCHEMA_VERSION`].
    pub schema: u32,
    /// Short git SHA of the HEAD commit at run time (`"unknown"` when unavailable).
    pub sha: String,
    /// Hostname of the machine that produced the results.
    pub host: String,
    /// ISO-8601 timestamp of when the run completed.
    pub created_at: String,
    /// Individual fixture measurements.
    pub runs: Vec<BenchRecord>,
}

/// Per-group regression threshold.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupThreshold {
    /// Maximum allowed performance regression as a percentage (e.g. `10` = 10 %).
    pub max_regression_pct: f64,
}

/// Top-level `guardrails.json` document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guardrails {
    /// Schema version; always 1.
    pub schema: u32,
    /// Per-group threshold map.
    pub thresholds: HashMap<String, GroupThreshold>,
}

impl Default for Guardrails {
    fn default() -> Self {
        let mut thresholds = HashMap::new();
        let groups: &[(&str, f64)] = &[
            ("clean_small", 10.0),
            ("clean_medium", 8.0),
            ("clean_large", 5.0),
            ("spec_rules", 10.0),
            ("fallthrough_custom_elements", 10.0),
            ("fallthrough_bare_lt", 10.0),
            ("adversarial", 30.0),
        ];
        for (group, max_pct) in groups {
            thresholds.insert(
                (*group).to_owned(),
                GroupThreshold {
                    max_regression_pct: *max_pct,
                },
            );
        }
        Self {
            schema: SCHEMA_VERSION,
            thresholds,
        }
    }
}
