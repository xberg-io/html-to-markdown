---
description: "html-to-markdown architecture — Rust core, polyglot bindings, single-pass DOM walk, FFI surface, feature flags. Where the conversion logic lives and how each language reaches it."
---

# Architecture

html-to-markdown has a Rust core plus 15 generated packages. All language surfaces call the same `convert()` function — there is no per-language conversion logic.

## Crate layout

The repository is a Cargo workspace. The core library and every native binding live under `crates/`; thin distribution wrappers live under `packages/`.

```text
crates/
├── html-to-markdown/          # Core library (html-to-markdown-rs on crates.io)
├── html-to-markdown-cli/      # CLI binary
├── html-to-markdown-ffi/      # C FFI shared library (libhtml_to_markdown, headers via cbindgen)
├── html-to-markdown-node/     # Node.js / TypeScript binding (NAPI-RS)
├── html-to-markdown-py/       # Python binding (PyO3)
├── html-to-markdown-php/      # PHP extension (ext-php-rs)
├── html-to-markdown-wasm/     # WebAssembly binding (wasm-bindgen)
└── html-to-markdown-rs-jni/   # JNI bridge used by Kotlin Android
```

Go, Ruby, Java, C#, Elixir, R, Swift, Dart, Kotlin Android, and Zig live under `packages/` and call Rust through their generated backend or `libhtml_to_markdown`. The Rust workspace version in the root `Cargo.toml` is the single source of truth — all binding manifests inherit it.

## Core library

`crates/html-to-markdown/` is the only place where conversion logic lives. Its public API is one function:

```rust
pub fn convert(html: &str, options: Option<ConversionOptions>) -> Result<ConversionResult>
```

See [Conversion pipeline](pipeline.md) for the per-stage breakdown.

## Binding mechanisms

Each language reaches Rust via one of two routes: direct embedding (the Rust code compiles into the host language's native extension module) or the C FFI shared library (the host language loads `libhtml_to_markdown` at runtime).

### Direct embedding

| Language             | Mechanism                                                               | Crate                   |
| -------------------- | ----------------------------------------------------------------------- | ----------------------- |
| Python               | [PyO3](https://pyo3.rs) extension module                                | `html-to-markdown-py`   |
| TypeScript / Node.js | [NAPI-RS](https://napi.rs) Node-API addon                               | `html-to-markdown-node` |
| PHP                  | [ext-php-rs](https://davidcole1340.github.io/ext-php-rs/) PHP extension | `html-to-markdown-php`  |
| Ruby                 | [Magnus](https://github.com/matsadler/magnus) gem extension             | `packages/ruby`         |
| Elixir               | [Rustler](https://github.com/rusterlium/rustler) NIF                    | `packages/elixir`       |
| R                    | [extendr](https://extendr.github.io) R extension                        | `packages/r`            |
| WebAssembly          | [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)                | `html-to-markdown-wasm` |
| Swift                | [swift-bridge](https://github.com/chinedufn/swift-bridge) package       | `packages/swift`        |
| Dart                 | [flutter_rust_bridge](https://cjycode.com/flutter_rust_bridge/) package | `packages/dart`         |

For these, the conversion function compiles directly into the native extension. The host language never sees the C ABI.

### Via libhtml_to_markdown (C FFI)

| Language | Mechanism                                | Notes                                                |
| -------- | ---------------------------------------- | ---------------------------------------------------- |
| Go       | cgo                                      | Bundles `libhtml_to_markdown.a` in the Go module.    |
| Java     | Foreign Function & Memory API (Java 25+) | Extracts the shared library from the JAR at startup. |
| C#       | P/Invoke (`DllImport`)                   | Loads `libhtml_to_markdown` via `NativeLibrary`.     |
| Kotlin Android | JNI bridge                         | Android AAR; JVM Kotlin users consume the Java package. |
| Zig      | C ABI                                   | Zig package wraps the C FFI symbols.                 |
| C        | Direct link                              | Header `html_to_markdown.h`, prefix `htm_*`/`HTM*`.  |

The FFI crate exposes a minimal, null-safe C API (`htm_convert`, `htm_conversion_result_*`, `htm_free_string`, …) and catches any Rust panics at the boundary, converting them to `ConversionError::Panic` rather than letting them unwind across the ABI.

## Feature flags

The core library compiles with five Cargo features. Bindings enable the subset they expose.

| Feature         | What it gates                                                |
| --------------- | ------------------------------------------------------------ |
| `metadata`      | Metadata extraction and `HtmlMetadata` types (on by default) |
| `visitor`       | `HtmlVisitor` trait and `ConversionError::Visitor`           |
| `inline-images` | Data-URI decoder and inline SVG extractor                    |
| `serde`         | `Serialize`/`Deserialize` on option and result types         |
| `full`          | All four optional features combined                          |

Rust and WASM expose inline image extraction when built with `inline-images`. Generated native bindings may omit the Rust-only `InlineImage` payload even when they expose metadata for ordinary images.

## Thread safety and allocation

The core library is stateless and does not use global state. `convert()` is safe to call from multiple threads simultaneously. Each call allocates and frees its own buffers independently.

Bindings that cross the C FFI boundary use opaque handles returned by `htm_*` constructors and freed by matching `htm_*_free` functions. Strings returned across the FFI must be released with `htm_free_string`. The Go, Java, and C# bindings handle this internally.

## Ecosystem fit

html-to-markdown is consumed by [Kreuzberg](https://docs.xberg.io) (as the HTML extractor in the document intelligence pipeline) and [kreuzcrawl](https://docs.kreuzcrawl.xberg.io) (as the conversion stage after page fetch). The output `ConversionResult` is intentionally close in shape to Kreuzberg's `ExtractionResult` so the two compose without translation.

--8<-- "snippets/feedback.md"
