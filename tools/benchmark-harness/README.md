# html-to-markdown benchmark harness

Benchmark harness and golden oracle for `html-to-markdown-rs`, providing
fixture-driven timing, snapshot regression testing, and a corpus survey tool.

## Usage

All commands are exposed via the `htmbench` binary and wired into the root
Taskfile under the `bench:h2m:*` namespace.

### Run benchmarks

```sh
task bench:h2m
# or directly:
cargo run --release -p html-to-markdown-bench -- run
```

Writes `tools/benchmark-harness/results/latest.json`.

### Compare against baseline

```sh
task bench:h2m:compare
```

Loads `results/latest.json`, `baselines/baseline.json`, and `guardrails.json`.
Exits non-zero when any fixture regresses beyond its group threshold.

### Oracle snapshot tests

```sh
# Verify snapshots
task bench:h2m:oracle

# Overwrite snapshots (after an intentional output change)
task bench:h2m:oracle:bless
```

Snapshots live under `tools/benchmark-harness/snapshots/`. Each fixture is
run through four [`Permutation`] variants (default, skip\_images, no\_metadata,
reference\_links). All permutations set `skip_images = true` to keep snapshot
output deterministic (html5ever SVG attribute ordering varies at runtime).

### Corpus survey

```sh
task bench:h2m:survey
```

Prints a per-fixture and per-group table counting CDATA sections, custom
elements, bare `<` characters, and tables without `<tbody>`.

### Compare against mdream (optional)

Requires a local clone of [harlan-zw/mdream](https://github.com/harlan-zw/mdream)
at `/tmp/mdream/crates/core`:

```sh
task bench:h2m:mdream
```

## Fixture groups

| Group | Description |
|---|---|
| `clean_small` | Small, well-formed HTML (<10 KB) |
| `clean_medium` | Medium, well-formed HTML (10–100 KB) |
| `clean_large` | Large, well-formed HTML (>100 KB) |
| `spec_rules` | Exercises optional-close-tag and other HTML spec edge cases |
| `fallthrough_custom_elements` | Contains custom elements (tag names with hyphens) |
| `fallthrough_bare_lt` | Contains bare `<` characters in text |
| `adversarial` | Malformed or stress-test HTML |

Fixture assignments live in `fixtures/groups.toml`.

## Guardrails

`guardrails.json` defines per-group maximum regression percentages:

- `clean_small`: 10%
- `clean_medium`: 8%
- `clean_large`: 5%
- `spec_rules`, `fallthrough_*`: 10%
- `adversarial`: 30%

## Timing methodology

Mirrors the methodology used in
`/tmp/bench-h2m-vs-mdream/src/bench.rs`:

1. 20 warmup iterations (discarded)
2. Calibrate: double `N` until `N` iterations take ~50 ms
3. Best-of-3 runs of `N` iterations; report the best observed ms/call

## Adding fixtures

1. Copy the HTML file under `fixtures/<group-slug>/`.
2. Add an entry to `fixtures/groups.toml`.
3. Run `task bench:h2m:oracle:bless` to create initial snapshots.
4. Run `task bench:h2m:survey` to verify the feature counts look reasonable.
5. Run `task bench:h2m` then copy `results/latest.json` over
   `baselines/baseline.json` if the fixture is new.

## Attribution

See [ATTRIBUTIONS.md](../../ATTRIBUTIONS.md) for third-party fixture credits.
