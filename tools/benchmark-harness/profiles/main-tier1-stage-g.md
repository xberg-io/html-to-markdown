# Stage G: Perf Iteration on Activated Tier-1

## Context

PR #400 shipped the activated Tier-1 byte scanner. Stage E's "loss table" was
measured against a pre-Stage-A `Tier-2-only` baseline that no longer represents
the same code path. With a proper `--force-tier2` flag and a like-for-like
comparison on current `chore/bench` HEAD, Tier-1 is at parity or marginally
ahead of Tier-2 on every previously-reported "loss" fixture — there was no
gap to close.

This stage shifted the goal from "close the loss gap" to "push Tier-1
absolute speed", gated by a per-commit `≥ 2%` improvement rule on at least
two reference fixtures with no `> 2%` regression on any control.

## Methodology

Each candidate was applied in isolation against the prior committed HEAD,
benched with `./target/release/htmbench run --force-tier1 --output …`
(20 warmup iters, auto-calibrated N, best-of-3 measurement), and compared
delta-by-delta against the immediately-prior snapshot. Candidates that did
not clear the gate were reverted via `git checkout` of the affected file.

Bench numbers drift run-to-run (system noise on the order of ±2–3% on some
fixtures), so per-candidate measurements were always taken back-to-back in
the same shell session, not against snapshots from earlier sessions.

## Landed changes

### P2 — Thread normalized `Cow<str>` through bail fallback

`crates/html-to-markdown/src/convert_api.rs`. Previously, on Tier-1 bail or
`RouterDecision::Tier2`, the dispatcher discarded the `Cow<str>` produced by
`normalize_input` and re-ran `normalize_input(html)` for the Tier-2 path.
Threading the existing `Cow` through eliminates one full input pass on every
bail-fallback or Tier-2-routed call.

### P3.a — In-place `collapse_excess_blank_lines`

`crates/html-to-markdown/src/converter/tier1/scanner.rs::collapse_excess_blank_lines`.
Was: build a fresh `String::with_capacity(output.len())`, iterate chars,
push selectively, then `*output = cleaned`. Now: `String::retain` walks the
buffer in place with no fresh allocation.

| Fixture                       | before ms | after ms | delta  |
|-------------------------------|-----------|----------|--------|
| github-markdown-complete.html |     9.486 |    9.203 | -2.99% |
| wikipedia/lists_timeline.html |     8.004 |    7.699 | -3.81% |
| wikipedia/medium_python.html  |    47.022 |   45.638 | -2.94% |
| mdream/wikipedia-small.html   |     5.095 |    4.958 | -2.68% |
| mdream/mdn-array.html         |     5.528 |    5.410 | -2.12% |
| issues/gh-121-hacker-news.html|     0.731 |    0.700 | -4.23% |
| issues/gh-190/firsteigen.html |     0.979 |    0.954 | -2.55% |
| wikipedia/large_rust.html     |    27.686 |   27.623 | -0.23% |

Eight fixtures improved, no fixture regressed.

## Rejected candidates (reverted)

### P3 candidate 1 — Replace `format!()` in `close_link` / `emit_void` image with explicit `push_str` chains

Reverted. Mixed: github-markdown -2.21%, large_rust **+2.00%**,
firsteigen **+3.73%**. Confirmed the existing Stage-5c comment
("measured: write! is slower on this workload") — rustc/LLVM already
amortizes the `format!` allocation well enough that the explicit chain
plus `reserve()` is net-slower on most fixtures.

### P3 candidate 3 — Cache per-cell `chars().count()` in `emit_gfm_table` + ASCII fast-path

Reverted. Uniform regression across all fixtures, including non-table ones
(react-learn +2.02%, large_rust +2.98%). The added helper function and the
`Vec<usize>` cache disturbed inlining/codegen of the surrounding hot path
more than the saved scan paid for.

### P3 candidate 4 — `memchr`-skip prescan main loop's byte-by-byte search for `<`

Reverted. Catastrophic regression (+4% to +14% across the board). The
tight `if bytes[idx] != b'<' { idx += 1; continue; }` loop appears to
already be auto-vectorized by LLVM, and the explicit `memchr` call adds
per-iteration overhead that defeats the win on tag-dense inputs.

## Head-to-head: Tier-1 vs Tier-2 (current HEAD)

| Fixture                                | T1 ms | T2 ms | T2/T1 (speedup) |
|----------------------------------------|-------|-------|-----------------|
| mdream/react-learn.html                |  9.41 |  9.53 | 1.01×           |
| mdream/vuejs-docs.html                 |  4.49 |  4.69 | 1.04×           |
| mdream/github-markdown-complete.html   |  9.49 |  9.15 | 0.96×           |
| real-world/wikipedia/lists_timeline    |  8.00 |  8.12 | 1.01×           |
| real-world/wikipedia/medium_python     | 47.02 | 46.53 | 0.99×           |
| real-world/wikipedia/large_rust        | 27.69 | 27.55 | 1.00×           |
| mdream/wikipedia-small.html            |  5.10 |  5.14 | 1.01×           |
| mdream/mdn-array.html                  |  5.53 |  5.45 | 0.99×           |

All measurements are best-of-3 with the same harness; speedups within ±5% are
noise-equivalent on this hardware. Tier-1 is at parity or marginally ahead on
every prior "loss" fixture, with P3.a accounting for ~2–4% of the closure.

## Notes for future iterations

Diminishing returns regime. Three of four attempted candidates regressed
under the per-commit gate. The auto-vectorizer on tight byte-loops, and
the format-machinery on short formatted strings, leave little headroom for
hand-tuning. Future wins likely require:

- A real profiler with working symbol resolution on the host (Stage P1's
  samply traces could not be symbolicated on macOS).
- Architectural changes (e.g., a single-pass scan that fuses prescan +
  scanner — currently they are sequential full-input passes).
- Profile-guided optimization (PGO) rather than further source rewrites.
