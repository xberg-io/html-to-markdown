---
priority: critical
---

- Files in `packages/*/` and binding crates are generated or managed by Alef — check `alef.toml` before editing
- `alef.toml` defines: output paths, module names, rename mappings, e2e call overrides, README templates
- Run `alef generate` after changing `alef.toml` — commit both source and generated files
- Never hand-edit generated files; modify `alef.toml` or the Rust source instead
- Fixtures under `fixtures/` feed `tools/e2e-generator/` — never add tests to `e2e/` directly
