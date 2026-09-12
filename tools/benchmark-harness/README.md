# Benchmark harness

`htmbench run` captures nine independently timed samples per fixture and records their median and median absolute
deviation (MAD). It also retains the best of the first three batches solely for method-compatible comparisons with the
temporary schema-v1 baseline. Result schema v2 records the OS, architecture, CPU model/count, full Rust compiler
identity, Cargo version, compile-time profile and flags, compiled core features, measurement mode/settings, forced tier,
visitor mode, iteration override, and runner image/class
when available. Hostname is diagnostic only and is never part of the compatibility key.

Run a normal capture and comparison with:

```sh
task bench:run
task bench:compare
```

The checked-in schema-v1 baseline remains a temporary percentage-only compatibility bridge. Comparisons print a
warning while that bridge is active. Once an approved calibration promotes the baseline and guardrails to schema v2,
comparison becomes strict: the capture's provenance *contract* must match the approved calibration provenance exactly,
and every fixture must have a measured floor. A mismatch or missing floor is a configuration error, not a skip.

## Provenance contract versus host identity

The contract is everything that must match for two captures to be comparable at all: OS, architecture, Rust compiler
identity and host triple, Cargo version, profile, build flags, compiled core features, measurement mode and settings,
forced tier, visitor mode, iteration override, and runner image/class. Any difference there is a configuration error
and aborts the comparison with `benchmark provenance mismatch` before a single timing is evaluated.

`cpu_model` and `cpu_count` are recorded but deliberately excluded from that contract. GitHub's `ubuntu-24.04` label
is one name over a mixed pool — the same workflow draws AMD EPYC hosts on one run and Intel Xeon hosts on the next —
while a calibration campaign is `workflow_dispatch`-only and can be captured on whichever host it happened to draw.
Making the CPU part of the contract therefore turned an unavoidable pool difference into a coin-flip hard failure.

Host identity instead decides how a *timing* violation is reported. When the CPU differs from the calibrated one,
comparison prints a warning, and `--allow-host-mismatch` (`ALLOW_HOST_MISMATCH=true task bench:compare`) downgrades
violations to advisory, because a positive delta on hardware the baseline was never measured on is not evidence of a
code regression. The flag is off by default and is passed only by the `ci-rust.yaml` regression job. It never relaxes
the contract, and when the hardware *does* match the baseline it changes nothing: a real regression on the calibrated
CPU still fails the run. Never reach for it to silence a violation observed on matching hardware — re-measure, or fix
the regression.

## Calibration and baseline promotion

A calibration campaign requires exactly 40 schema-v2, full-corpus captures from one commit on one quiet, pinned
runner. Name captures in acquisition order (`0001.json` through `0040.json`) because adjacent captures form the 20
pairs used by calibration. Then run:

```sh
RUNS_DIR=/path/to/campaign task bench:calibrate
```

Calibration accepts only `0001.json` through `0040.json` with strictly increasing RFC3339 capture timestamps. It
validates the full commit SHA, fixture inventory, input/output sizes, nine-sample records, OS, architecture, CPU
model/count, Rust compiler version/host, Cargo version, release profile, core features, and runner image/class before
writing either file. The promoted baseline uses dedicated calibrated records rather than pretending the 40 run medians
are one run's samples, and a shared campaign ID binds that baseline to its guardrails. For each fixture it stores the
median and MAD across the 40 run medians. The floor is the
nearest-rank p95 of the 20 absolute adjacent-pair deltas. Comparison fails only when the positive delta exceeds the
larger of the unchanged group-policy percentage (5%, 8%, 10%, or 30%) and that fixture floor.

Both output files are staged and backed up before promotion; if either promotion fails, both originals are restored.
Baseline promotion requires retained raw artifacts, comparable hardware, a quiet-runner record, and reviewer approval.
Never promote a baseline or populate floors merely to make CI green.

### Inventory differences never hide the timing verdict

`compare` evaluates the fixture inventory and the timing guardrails in one pass and reports both.
It used to abort on the first inventory difference, which meant a release that legitimately changed
a fixture's output printed `fixture metadata differs` and **never evaluated a single timing** — a
real regression riding along with an accepted output change was invisible, and the green/red signal
said nothing about performance either way. Observed in 3.12.4: one fixture's output moved by one
byte, and three guardrail violations underneath it went unreported.

Both halves are fatal. `--allow-host-mismatch` downgrades **timing** violations only: a fixture
inventory is a property of the corpus and the converter, not of the CPU that measured it, so a
heterogeneous runner pool is never a reason to accept an inventory change.

### Re-promoting after a deliberate conversion change

The fixture inventory is validated against the baseline being replaced, so a conversion fix that
legitimately changes what the corpus renders to blocks calibration on `fixture metadata differs`.
Re-promote with the explicit opt-in:

```sh
RUNS_DIR=/path/to/campaign ACCEPT_OUTPUT_CHANGE=1 task bench:calibrate
```

That relaxes **only** the converted-output size comparison. The fixture set, its groups, and its
input sizes are still compared exactly, so a renamed, added, removed, or edited fixture is still
rejected. Confirm the output change is an improvement — diff the before/after markdown, do not
infer it from the byte count — and record which fixtures moved and why. Every other promotion
requirement above still applies; the flag is not an approval.

## Quiet-runner gate

Capture calibration or A/B evidence only when the runner is pinned, on external power, thermally stable, and has no
competing compiler, test, indexing, or benchmark workload. The five-minute load average must stay below 25% of the
logical CPU count before the first capture and throughout the campaign. Abort and retain no floors if any condition
fails.

Issue #461 A/B evidence belongs under `evidence/issue-461/<campaign>/`. Build both revisions first with separate target
directories and identical toolchain/features, then collect at least 20 full-corpus rounds in A/B/B/A order. Retain every
raw result and report per-fixture median/MAD, paired ratios, bootstrap confidence intervals, and aggregate geometric
mean. The evidence README defines the on-disk layout.
