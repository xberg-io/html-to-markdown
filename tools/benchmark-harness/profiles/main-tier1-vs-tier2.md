# Tier-1 vs Tier-2: Head-to-Head on Main (post Stage C)

Measurement context:
- HEAD: `d47c5e21f` (Stage C: metadata relocation + constants + dead code).
- Bench harness: `htmbench run` (Tier-1-forced via `--force-tier1`) and the prior Tier-2-only baseline at `4cc3fb3c8` (pre Stage A classifier activation).
- Methodology: 20 warmup iters, auto-calibrated N (~50 ms target), best-of-3 measurement. Single fixture per `bench:run` invocation.

## Group medians

| Group | N | Tier-2 (MB/s) | Tier-1 (MB/s) | Median speedup |
|---|---|---|---|---|
| `adversarial` | 5 | 28.1 | 58.9 | **2.08×** |
| `spec_rules` | 2 | 16.6 | 79.2 | **7.47×** |
| `clean_small` | 3 | 55.2 | 93.7 | **1.70×** |
| `clean_medium` | 12 | 42.2 | 42.6 | 0.80× (median loss) |
| `clean_large` | 5 | 44.2 | 33.0 | 0.76× (median loss) |
| `fallthrough_bare_lt` | 1 | — | 73.7 | n/a (Tier-2 baseline was 0) |
| `fallthrough_custom_elements` | 1 | 51.9 | 48.7 | 0.94× |

**The big picture:** Tier-1 reliably wins on small, structurally simple inputs (spec rules, adversarial synthetic, small real-world). It reliably *loses* on Wikipedia/MDN/React-scale clean documents, where main's Tier-2 has been heavily optimized over the v3.6 series.

## Top wins

| Speedup | Fixture | Group | Input |
|---|---|---|---|
| 7.47× | `synthetic/optional_li.html` | spec_rules | 33 B |
| 4.27× | `synthetic/unclosed_p.html` | adversarial | 26 B |
| 2.85× | `synthetic/bare_fragment.html` | adversarial | 5 B |
| 2.62× | `real-world/issues/gh-121-hacker-news.html` | clean_medium | 55 KB |
| 2.46× | `real-world/issues/gh-190/firsteigen.html` | clean_medium | 68 KB |
| 2.27× | `real-world/issues/gh-190/insight.html` | clean_medium | 12 KB |
| 2.08× | `synthetic/unescaped_lt.html` | adversarial | 18 B |
| 1.81× | `real-world/issues/gh-190/mitrade.html` | clean_small | 2.3 KB |
| 1.70× | `real-world/issues/gh-190/flex2025.html` | clean_small | 6.2 KB |
| 1.52× | `real-world/wikipedia/large_rust.html` | clean_large | 1.04 MB |

## Top losses

| Speedup | Fixture | Group | Input |
|---|---|---|---|
| 0.49× | `real-world/wikipedia/lists_timeline.html` | clean_large | 246 KB |
| 0.53× | `mdream/react-learn.html` | clean_medium | 259 KB |
| 0.55× | `mdream/github-markdown-complete.html` | clean_medium | 420 KB |
| 0.56× | `mdream/vuejs-docs.html` | clean_medium | 110 KB |
| 0.56× | `real-world/wikipedia/medium_python.html` | clean_large | 1.22 MB |
| 0.61× | `synthetic/unclosed_at_eof.html` | adversarial | 16 B |
| 0.74× | `mdream/nuxt-example.html` | clean_small | 3.5 KB |
| 0.76× | `real-world/wikipedia/small_html.html` | clean_large | 950 KB |
| 0.80× | `mdream/wikipedia-small.html` | clean_medium | 162 KB |
| 0.80× | `real-world/wikipedia/tables_countries.html` | clean_large | 738 KB |
| 0.89× | `mdream/mdn-array.html` | clean_medium | 226 KB |

## Hypothesis on the loss pattern

Tier-1 wins on inputs where Tier-2's fixed overhead (preprocessing, `tl::parse`, walk_node setup) dominates the conversion cost. As fixture size grows past ~100 KB, that fixed overhead amortizes and Tier-2's per-byte path becomes competitive or faster than Tier-1's. main has had v3.x worth of perf work on `walk_node`, `process_text_node`, handler dispatch.

**Suspected Tier-1 hotspots needing investigation** (Stage E candidates):

1. **`decode_and_collapse_into`** runs over every text node; the entity-decode + whitespace-collapse inner loop may be doing redundant work vs Tier-2's specialized `process_text_node`.
2. **`scan` main loop** still has `format!()` calls for ordered-list markers / links / images (clippy flagged 7 sites; the Stage 5c attempt to replace with `write!` measured WORSE, but those measurements were against ForceTier1 which has different overhead than today's Auto path — worth reassessing).
3. **`String::with_capacity` cap** at 256 KB — Wikipedia-small produces 44 KB output (no realloc), but `medium_python` produces 246 KB (within cap), and `react-learn` produces 67 KB (within cap). Capacity isn't the issue for these losses; the per-byte loop is.
4. **Allocation on hot path** — `state.stack` Vec push per tag; the cell text buffer in table emit; `OpenTag` `link_href` / `link_title` Options each carrying a `String`. Profile required.
5. **`apply_open_escape_ctx`** is called per tag open — bit manipulation should be cheap but worth checking it inlines.

## Reproducibility

```bash
cargo build --release --features testkit -p html-to-markdown-bench
./target/release/htmbench run --output /tmp/tier1.json --force-tier1
./target/release/htmbench run --output /tmp/auto.json
```

Compare against `tools/benchmark-harness/baselines/baseline.json` (the pre-Stage-A Tier-2-only baseline).
