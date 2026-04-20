---
priority: high
---

# Crate & Package Structure

## Workspace crates (`crates/`)

- `html-to-markdown` — core library, primary Rust API, `unsafe_code = "forbid"` at workspace level
- `html-to-markdown-cli` — CLI binary (clap)
- `html-to-markdown-ffi` — C FFI bridge, cbindgen headers, **only crate that overrides unsafe_code lint**
- `html-to-markdown-py` — PyO3 Python binding
- `html-to-markdown-node` — NAPI-RS Node/TypeScript binding
- `html-to-markdown-php` — ext-php-rs PHP binding
- `html-to-markdown-wasm` — wasm-bindgen WebAssembly binding

## Out-of-workspace packages (`packages/`)

- `csharp/`, `elixir/`, `go/`, `java/`, `r/`, `ruby/` — language-native packages wrapping the FFI crate
- `php/`, `python/`, `typescript/`, `wasm/` — distribution packages

## Primary API

- `convert(&str, Option<ConversionOptions>) -> Result<ConversionResult, ConversionError>`
- `ConversionResult`: `content`, `warnings`, optionally `metadata` and `inline_images` (feature-gated)
- Feature flags: `inline-images`, `metadata`, `visitor` (custom traversal), `serde`
- Dual parser: html5ever (spec-compliance) and astral-tl (performance), selectable via `ConversionOptions`
