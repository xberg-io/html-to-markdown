---
type: Playbook
title: Crate Structure
description: "Map of the html-to-markdown workspace: crates/, packages/ and the convert() API with its result shape and feature flags. Use when navigating crates or packages, deciding where code belongs, or calling convert()."
x-ai-rulez:
  kind: skill
  id: crate-structure
  metadata:
    name: crate-structure
---

# Crate & Package Structure

## Workspace members (root `Cargo.toml`)

- `crates/html-to-markdown` — core library and primary Rust API
- `crates/html-to-markdown-cli` — CLI binary (clap)
- `crates/html-to-markdown-ffi` — C FFI bridge with cbindgen headers
- `crates/html-to-markdown-py` — PyO3 Python binding
- `crates/html-to-markdown-node` — NAPI-RS Node/TypeScript binding
- `crates/html-to-markdown-php` — ext-php-rs PHP binding
- `crates/html-to-markdown-rs-jni` — JNI binding
- `crates/html-to-markdown-wasm` — wasm-bindgen WebAssembly binding
- `packages/dart/rust`, `packages/swift/rust`, `tools/benchmark-harness`
- Workspace lints set `unsafe_code = "forbid"`; only the core and CLI crates inherit them (`[lints] workspace = true`).

## Language packages (`packages/`)

- `csharp/`, `dart/`, `elixir/`, `go/`, `java/`, `kotlin-android/`, `php/`, `python/`, `r/`, `ruby/`, `swift/`, `zig/`

## Primary API

- `convert(html: &str, options: impl Into<Option<ConversionOptions>>) -> Result<ConversionResult>` in `crates/html-to-markdown/src/convert_api.rs`
- `ConversionResult`: `content`, `document`, `tables`, `warnings`; `metadata` with the `metadata` feature, `images` with `inline-images`
- Feature flags: `metadata` (default), `inline-images`, `visitor` (caller traversal hooks), `serde`, `mcp`, `mcp-http`, `full`
