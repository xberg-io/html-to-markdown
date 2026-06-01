//! Fixture loading and group classification.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

/// A single fixture entry from `groups.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct FixtureEntry {
    /// Relative path from the fixtures directory (e.g. `"mdream/nuxt-example.html"`).
    pub path: String,
    /// Group tag (e.g. `"clean_small"`).
    pub group: String,
}

/// Parsed `groups.toml`.
#[derive(Debug, Deserialize)]
struct GroupsFile {
    fixtures: Vec<FixtureEntry>,
}

/// Loaded fixture with resolved path and metadata.
#[derive(Debug, Clone)]
pub struct Fixture {
    /// Relative path key (from fixtures root).
    pub rel_path: String,
    /// Absolute path to the `.html` file.
    pub path: PathBuf,
    /// Group assignment.
    pub group: String,
    /// File size in bytes.
    pub bytes: u64,
}

/// Loads fixtures from a `groups.toml` manifest file.
pub struct Loader {
    fixtures_dir: PathBuf,
}

impl Loader {
    /// Create a loader rooted at `fixtures_dir`.
    pub fn new(fixtures_dir: PathBuf) -> Self {
        Self { fixtures_dir }
    }

    /// Load all fixtures defined in `groups.toml`, applying an optional group filter.
    pub fn load(&self, filter: Option<&str>) -> Result<Vec<Fixture>> {
        let groups_path = self.fixtures_dir.join("groups.toml");
        let raw =
            std::fs::read_to_string(&groups_path).with_context(|| format!("reading {}", groups_path.display()))?;
        let parsed: GroupsFile = toml::from_str(&raw).with_context(|| format!("parsing {}", groups_path.display()))?;

        let mut fixtures = Vec::new();
        for entry in parsed.fixtures {
            if let Some(f) = filter {
                if entry.group != f {
                    continue;
                }
            }
            let abs = self.fixtures_dir.join(&entry.path);
            if !abs.exists() {
                tracing::warn!("fixture not found, skipping: {}", abs.display());
                continue;
            }
            let bytes = abs.metadata().with_context(|| format!("stat {}", abs.display()))?.len();
            fixtures.push(Fixture {
                rel_path: entry.path,
                path: abs,
                group: entry.group,
                bytes,
            });
        }
        Ok(fixtures)
    }

    /// Load all fixtures without group filtering and return a group-to-fixtures map.
    pub fn load_by_group(&self) -> Result<HashMap<String, Vec<Fixture>>> {
        let all = self.load(None)?;
        let mut map: HashMap<String, Vec<Fixture>> = HashMap::new();
        for f in all {
            map.entry(f.group.clone()).or_default().push(f);
        }
        Ok(map)
    }
}

/// Discover all HTML files under `dir` that are not listed in `groups.toml`.
pub fn find_ungrouped(fixtures_dir: &Path) -> Result<Vec<PathBuf>> {
    let groups_path = fixtures_dir.join("groups.toml");
    let raw = std::fs::read_to_string(&groups_path).with_context(|| format!("reading {}", groups_path.display()))?;
    let parsed: GroupsFile = toml::from_str(&raw).with_context(|| format!("parsing {}", groups_path.display()))?;
    let known: std::collections::HashSet<PathBuf> =
        parsed.fixtures.iter().map(|e| fixtures_dir.join(&e.path)).collect();

    let mut ungrouped = Vec::new();
    for entry in walkdir::WalkDir::new(fixtures_dir).into_iter().filter_map(|e| e.ok()) {
        let p = entry.path().to_path_buf();
        if p.extension().and_then(|e| e.to_str()) == Some("html") && !known.contains(&p) {
            ungrouped.push(p);
        }
    }
    Ok(ungrouped)
}
