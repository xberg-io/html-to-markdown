---
title: API reference
description: "Generated, binding-accurate API pages from the Rust source (alef docs)."
---

## API reference

These pages are **generated** from the public Rust API surface. You can run `task alef:generate` (or `alef docs`) after changing types, options, or `alef.toml` so the Markdown under `docs/reference/` stays in sync.

| Topic                               | Link                                                    |
| ----------------------------------- | ------------------------------------------------------- |
| All types and result shapes         | [Types](reference/types.md)                             |
| Options, metadata, and field tables | [Configuration (generated)](reference/configuration.md) |
| Error / enum reference              | [Error types (generated)](reference/errors.md)          |

### Language APIs

| Language          | Page                                                  |
| ----------------- | ----------------------------------------------------- |
| Rust              | [api-rust](reference/api-rust.md)                     |
| Python            | [api-python](reference/api-python.md)                 |
| TypeScript / Node | [api-typescript](reference/api-typescript.md)         |
| Go                | [api-go](reference/api-go.md)                         |
| Ruby              | [api-ruby](reference/api-ruby.md)                     |
| PHP               | [api-php](reference/api-php.md)                       |
| Java              | [api-java](reference/api-java.md)                     |
| C#                | [api-csharp](reference/api-csharp.md)                 |
| Elixir            | [api-elixir](reference/api-elixir.md)                 |
| R                 | [api-r](reference/api-r.md)                           |
| Kotlin (Android)  | [api-kotlin-android](reference/api-kotlin-android.md) |
| Swift             | [api-swift](reference/api-swift.md)                   |
| Dart              | [api-dart](reference/api-dart.md)                     |
| Zig               | [api-zig](reference/api-zig.md)                       |
| C (FFI)           | [api-c](reference/api-c.md)                           |
| WebAssembly       | [api-wasm](reference/api-wasm.md)                     |

For how to _use_ the API in your language, start with [Language guides](language-guides.md) and the narrative [Configuration](configuration.md) and [Error handling](errors.md) pages; this section is the exhaustive field-level reference.
