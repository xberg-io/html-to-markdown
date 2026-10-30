---
status: accepted
date: 2026-07-30
decision-makers: ["Na'aman Hirschfeld"]
---

# Tracing as the observability surface

## Context and Problem Statement

The core emitted no structured diagnostics, and the CLI reported warnings with raw `eprintln!` calls.
A caller could not see which tier a conversion took, when the parser repair ran or why a conversion
failed, and nothing stopped new raw prints.

## Considered Options

* `tracing` spans and events in the core, with the subscriber installed only by consumers such as the
  CLI, and raw prints denied by clippy
* Raw `eprintln!` diagnostics in the CLI (the behaviour before c4be45193b)

## Decision Outcome

Chosen option: "`tracing` spans and events in the core, with the subscriber installed only by
consumers such as the CLI, and raw prints denied by clippy", because c4be45193b (2026-07-30) records
the spans as "a product observability surface" and leaves the subscriber to the application.

* Every conversion runs in an `html_to_markdown::convert` INFO span with the fields `input_len`,
  `output_format`, `wrap`, `extract_metadata`, `extract_images` and `tier_strategy`. The rustdoc
  declares these names part of the public observability contract.
* Stage boundaries emit DEBUG events, recovered failures such as a parser repair or a Tier-1 bail emit
  WARN, and fatal errors emit ERROR.
* The library never installs a subscriber. The CLI installs `tracing-subscriber`, driven by `RUST_LOG`
  and `--debug`, and writes to stderr.
* 963c5973de (2026-07-30) denies `print_stdout`, `print_stderr` and `dbg_macro` workspace-wide. CLI
  result output opts back in with `#[expect(clippy::print_stdout)]`.

### Consequences

* Good, because applications that embed the library see conversions in their own tracing pipeline.
* Bad, because renaming a span or field is a breaking change to the observability contract.

### Confirmation

* `crates/html-to-markdown/src/convert_api.rs:72` instruments the conversion, and
  `crates/html-to-markdown/src/convert_api.rs:53` documents the span and its stable field names.
* `crates/html-to-markdown/Cargo.toml` has no `tracing-subscriber` dependency, and
  `crates/html-to-markdown-cli/src/main.rs:43` installs the subscriber in the CLI.
* `Cargo.toml:81` and `Cargo.toml:82` deny `print_stdout` and `print_stderr`.
