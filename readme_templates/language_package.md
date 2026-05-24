# html-to-markdown

{% include 'partials/_badges.md' %}

{{ description }}

## What This Package Provides

- **Same renderer as every binding** — output matches Rust, Python, Node.js, Ruby, PHP, Go, Java, .NET, Elixir, R, Dart, Swift, Zig, C FFI, and WASM.
- **Structured conversion result** — Markdown plus metadata, links, headings, images, tables, and warnings where the binding exposes them.
- **Production defaults** — HTML is parsed with the Rust core, sanitized by default, and rendered without runtime-specific Markdown drift.
  {% if language == "typescript" %}
- **Node-first TypeScript API** — native NAPI package, typed options/results, async-friendly integration for Node.js and Bun.
  {% elif language == "python" %}
- **Python-native packaging** — wheels for supported desktop/server platforms with a small, typed Python surface.
  {% elif language == "ruby" %}
- **Ruby extension** — Magnus-backed native extension with idiomatic Ruby objects and no separate service process.
  {% elif language == "php" %}
- **PHP extension** — typed PHP 8.2+ API with stubs suitable for PHPStan and production Composer projects.
  {% elif language == "go" %}
- **Go module** — cgo-backed binding with the Rust renderer behind ordinary Go functions and errors.
  {% elif language == "java" %}
- **Java package** — Panama FFM binding for direct native calls without a sidecar process.
  {% elif language == "csharp" %}
- **.NET package** — P/Invoke binding with record-shaped results and nullable-aware types.
  {% elif language == "elixir" %}
- **BEAM package** — Rustler NIF binding that keeps conversion inside the VM boundary.
  {% elif language == "r" %}
- **R package** — extendr binding for scripts, notebooks, and data workflows that need Markdown plus extracted metadata.
  {% elif language == "kotlin_android" %}
- **Android AAR** — bundled JNI libraries for Android targets; server-side Kotlin should use the Java package.
  {% elif language == "swift" %}
- **SwiftPM package** — swift-bridge types for macOS and iOS codebases.
  {% elif language == "dart" %}
- **Dart package** — flutter_rust_bridge API for Dart and Flutter projects.
  {% elif language == "zig" %}
- **Zig package** — thin wrapper over the C FFI with explicit allocation and error handling.
  {% endif %}

## Installation

{% include 'partials/_installation.md' %}

{% if migration_guide %}
{{ migration_guide }}
{% endif %}

{% if performance %}

## Performance Snapshot

{{ performance | render_performance_table(name) }}

{% endif %}

## Quick Start

{% include 'partials/_quick_start.md' %}

## API Reference

{% include 'partials/_api_reference.md' %}

{% include 'partials/_djot_output.md' %}

{% include 'partials/_plain_text_output.md' %}

{% if features.metadata_extraction %}

## Metadata Extraction

{% include 'partials/_metadata_extraction.md' %}
{% endif %}

{% if features.visitor_pattern %}

## Visitor Pattern

{% include 'partials/_visitor_pattern.md' %}
{% endif %}

## Examples

{% include 'partials/_footer.md' %}
