---
type: Decision
title: Alef Generated Bindings
paths:
  - "alef.toml"
  - "packages/**"
  - "crates/html-to-markdown-ffi/**"
  - "crates/html-to-markdown-py/**"
  - "crates/html-to-markdown-node/**"
  - "crates/html-to-markdown-php/**"
  - "crates/html-to-markdown-wasm/**"
  - "crates/html-to-markdown-rs-jni/**"
  - "fixtures/**"
  - "e2e/**"
x-ai-rulez:
  kind: rule
  id: alef-generated-bindings
  metadata:
    priority: critical
---

- Files in `packages/*/` and the binding crates are generated or managed by Alef; check `alef.toml` before editing.
- `alef.toml` defines output paths, module names, rename mappings, e2e call overrides and README templates.
- Do not hand-edit generated files; change `alef.toml` or the Rust source, then run `task alef:generate` (runs `alef all --clean`).
- Formatting is poly only (`task format` / `poly fmt --fix .`); there is no separate Alef formatting task.
- Build bindings explicitly with `task build:bindings` or `task build:all`.
- `fixtures/` is the input to `alef e2e generate` (`task e2e:generate`), which writes `e2e/`; add tests as fixtures, not in `e2e/`. There is no `tools/e2e-generator`.
- e2e tasks: `task e2e:generate`, `task e2e:build`, `task e2e:test`, `task e2e:all`; do not add legacy aliases.
