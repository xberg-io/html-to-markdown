# Issue 932 output oracle and calibration refresh

PR #951 deliberately separates adjacent links containing block elements. The Vue fixture has three such links;
the four generated snapshots now contain the two required spaces. Default output changes from 12982 to 12984 bytes.
All other 28 fixture output sizes and every input size remain identical to the previous campaign.

[Workflow run 40419519949](https://github.com/xberg-io/html-to-markdown/actions/runs/40419519949) collected exactly
40 ordered schema-v2 captures from `e2cb27c50805182a2f7fa8dcb24d189673f299af` on one
`INTEL(R) XEON(R) PLATINUM 8573C` runner with four logical CPUs, image `ubuntu24` (`20261023.334.3`),
and class `github-hosted`. Every capture has the same provenance, 29 fixtures and nine samples per fixture:
expected/actual counts are 40/40 captures, 1160/1160 fixture records and 10440/10440 samples.
Timestamps increase strictly from `2026-10-27T00:08:08.762222602Z` to `2026-10-27T00:24:22.784214129Z`.

The exact uploaded archive is retained as `raw.zip`; verify it with `shasum -a 256 -c SHA256SUMS` from this directory.
After extracting, verify all 45 original files against `CAPTURE_SHA256SUMS`. The archive includes provenance,
build and capture logs, the quiet gate, all captures and the campaign summary. Git retention preserves the raw
measurements beyond the workflow artifact's 30-day retention.

All five gated pre-capture quiet checks passed: maximum one-minute load 0.62 against a 1.00 limit, minimum idle
99.85%. The first, ungated warmup check reported load 1.70 and 99.85% idle. The existing workflow records no
five-minute load, power, thermal or competing-process checks throughout acquisition; these conditions cannot be
independently established from this archive. Review that limitation before approving baseline promotion.

The previous baseline used an AMD EPYC 7763. New versus old medians range from -39.32% to +159.30%, with a
+27.79% geometric mean. These are different CPUs and revisions, so these differences cannot establish a performance
regression or improvement. The new measured floors range from 0.49% to 7.82% of their medians. All seven group
policy percentages remain unchanged. The table exposes every timing change for review.

The pair was generated with:

```sh
RUNS_DIR=/path/to/extracted/runs ACCEPT_OUTPUT_CHANGE=1 task bench:calibrate
```

Independent recomputation checked all 29 baseline medians/MADs and adjacent-pair nearest-rank p95 floors against
the raw captures. Three scratch negative controls rejected 39 captures, changed output without the opt-in, and a
changed input size with the opt-in; both scratch policy files stayed byte-identical after every rejection.
`task bench:oracle` reports 116 matches, zero skips; `task bench:test` collects and passes 42 tests. Local verification
used `CARGO_PROFILE_RELEASE_LTO=false CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16` to reduce build cost; those overrides
were not used for the remote measurements or to modify capture provenance. This proposed pair requires reviewer
approval before promotion to the main branch.

| Fixture | Previous median ms | New median ms | Cross-CPU delta | New floor ms | Floor / median |
| --- | ---: | ---: | ---: | ---: | ---: |
| `mdream/nuxt-example.html` | 0.051264 | 0.060676 | +18.36% | 0.000985 | 1.62% |
| `mdream/vuejs-docs.html` | 5.485649 | 5.486024 | +0.01% | 0.072371 | 1.32% |
| `mdream/wikipedia-small.html` | 4.201382 | 6.531037 | +55.45% | 0.047280 | 0.72% |
| `mdream/mdn-array.html` | 9.792242 | 10.541756 | +7.65% | 0.154145 | 1.46% |
| `mdream/react-learn.html` | 11.253050 | 11.459678 | +1.84% | 0.457375 | 3.99% |
| `mdream/github-markdown-complete.html` | 10.092585 | 11.007709 | +9.07% | 0.861079 | 7.82% |
| `real-world/wikipedia/small_html.html` | 20.702064 | 31.094832 | +50.20% | 0.503952 | 1.62% |
| `real-world/wikipedia/lists_timeline.html` | 9.131580 | 10.089741 | +10.49% | 0.064792 | 0.64% |
| `real-world/wikipedia/tables_countries.html` | 13.523124 | 19.767412 | +46.17% | 0.385308 | 1.95% |
| `real-world/wikipedia/medium_python.html` | 55.250820 | 62.613916 | +13.33% | 4.037581 | 6.45% |
| `real-world/wikipedia/large_rust.html` | 21.692791 | 32.686468 | +50.68% | 0.704344 | 2.15% |
| `real-world/issues/gh-190/mitrade.html` | 0.079855 | 0.048456 | -39.32% | 0.000813 | 1.68% |
| `real-world/issues/gh-190/flex2025.html` | 0.131529 | 0.198747 | +51.10% | 0.003590 | 1.81% |
| `real-world/issues/gh-190/insight.html` | 0.197283 | 0.362886 | +83.94% | 0.001966 | 0.54% |
| `real-world/issues/gh-190/plusblog.html` | 2.000125 | 2.239241 | +11.96% | 0.010954 | 0.49% |
| `real-world/issues/gh-190/rbloggers.html` | 1.326242 | 1.895770 | +42.94% | 0.009275 | 0.49% |
| `real-world/issues/gh-190/ozonekorea.html` | 2.634802 | 2.872392 | +9.02% | 0.014120 | 0.49% |
| `real-world/issues/gh-190/firsteigen.html` | 2.028436 | 5.259651 | +159.30% | 0.036399 | 0.69% |
| `real-world/issues/gh-190/sjsu.html` | 0.940361 | 0.903280 | -3.94% | 0.006856 | 0.76% |
| `real-world/issues/gh-190/kimbrain.html` | 2.805736 | 3.772609 | +34.46% | 0.043018 | 1.14% |
| `real-world/issues/gh-121-hacker-news.html` | 1.694217 | 2.268913 | +33.92% | 0.015722 | 0.69% |
| `real-world/issues/gh-127-issue.html` | 16.594150 | 17.995988 | +8.45% | 0.224576 | 1.25% |
| `synthetic/optional_li.html` | 0.005353 | 0.008718 | +62.86% | 0.000325 | 3.72% |
| `synthetic/table_no_tbody.html` | 0.018794 | 0.018325 | -2.50% | 0.000297 | 1.62% |
| `synthetic/bare_fragment.html` | 0.000379 | 0.000331 | -12.83% | 0.000004 | 1.14% |
| `synthetic/unescaped_lt.html` | 0.003040 | 0.003829 | +25.97% | 0.000079 | 2.07% |
| `synthetic/unclosed_p.html` | 0.004018 | 0.009617 | +139.35% | 0.000151 | 1.57% |
| `synthetic/unclosed_at_eof.html` | 0.003469 | 0.008239 | +137.48% | 0.000103 | 1.25% |
| `synthetic/cdata_in_svg.html` | 0.012294 | 0.011645 | -5.28% | 0.000204 | 1.75% |
