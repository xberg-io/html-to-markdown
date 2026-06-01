# benchmark-harness

Benchmark harness and golden oracle for `html-to-markdown-rs`.

## Purpose

- **`run`** — measure throughput across 29 HTML fixtures, write a JSON results file.
- **`compare`** — diff results against a baseline, fail on per-group regressions.
- **`oracle`** — byte-exact snapshot tests across 4 option permutations per fixture.
- **`survey`** — corpus feature-coverage table (CDATA, custom elements, bare `<`, no-tbody tables).

## Layout

```
tools/benchmark-harness/
  fixtures/
    groups.toml              # group assignments for all 29 fixtures
    mdream/                  # 6 fixtures from harlan-zw/mdream (MIT)
    real-world/
      wikipedia/             # 5 Wikipedia-sourced fixtures
      issues/
        gh-190/              # 9 real-world regression fixtures
    synthetic/               # 7 hand-crafted edge-case fixtures
  baselines/
    baseline.json            # reference run results for compare
  snapshots/
    <group>__<file>__<perm>.snap  # 108 active oracle snapshots
  guardrails.json            # per-group max regression % thresholds
  src/
    lib.rs  bench.rs  fixture.rs  main.rs  oracle.rs  schema.rs  survey.rs
```

## Usage

Run from the repo root. Tasks are under the `bench:` namespace:

```
task bench:run            # benchmark and write results/latest.json
task bench:compare        # compare latest against baselines/baseline.json
task bench:oracle         # verify snapshots match current output
task bench:oracle:bless   # regenerate snapshots from current output
task bench:survey         # print corpus feature-coverage table
task bench:mdream         # side-by-side run with mdream (requires compare-mdream feature)
```

Or invoke directly:

```
cargo run --release -p html-to-markdown-bench -- run
cargo run --release -p html-to-markdown-bench -- compare
cargo run --release -p html-to-markdown-bench -- oracle
cargo run --release -p html-to-markdown-bench -- oracle --bless
cargo run --release -p html-to-markdown-bench -- survey
```

## Adding fixtures

1. Place the `.html` file under `fixtures/<subdir>/`.
2. Add an entry to `fixtures/groups.toml`:
   ```toml
   [[fixtures]]
   path = "subdir/my-fixture.html"
   group = "clean_medium"
   ```
3. Run `task bench:oracle:bless` to generate snapshots.
4. Run `task bench:oracle` to verify.
5. Run `task bench:run` and copy `results/latest.json` to `baselines/baseline.json`.

## Blessing snapshots

After a behavior change that intentionally alters output:

```
task bench:oracle:bless
task bench:oracle        # should pass immediately
```

Commit the new `.snap` files together with the code change.

## Interpreting guardrail failures

`task bench:compare` exits non-zero when any fixture's `ms_best` increases by
more than the group's `max_regression_pct` threshold.  Thresholds are in
`guardrails.json`.  To update the baseline after an intentional speedup or
after adding new fixtures:

```
cp tools/benchmark-harness/results/latest.json tools/benchmark-harness/baselines/baseline.json
```

## compare-mdream caveat

The `compare-mdream` feature requires a local clone of
[harlan-zw/mdream](https://github.com/harlan-zw/mdream) at `/tmp/mdream/`.
It is opt-in (`task bench:mdream`) and never required for CI.

## Known panics

`kimbrain.html` and `rbloggers.html` trigger a UTF-8 boundary panic in the
core converter (`converter.rs:163`) on this codebase.  The oracle catches
these panics, logs a NOTE, and skips them (8 of 116 pairs).  `task bench:run`
records 0 ms for these fixtures.
