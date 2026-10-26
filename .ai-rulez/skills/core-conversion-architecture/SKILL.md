---
type: Playbook
title: Core Conversion Architecture
description: "How the core crate turns HTML into Markdown: Tier-1 scanner, Tier-2 DOM walk, element converters, input validation and depth limits. Use when changing conversion output, parsing, or input-safety behavior in crates/html-to-markdown."
paths:
  - "crates/html-to-markdown/src/**"
x-ai-rulez:
  kind: skill
  id: core-conversion-architecture
  metadata:
    name: core-conversion-architecture
---

# Core Conversion Architecture

All paths are under `crates/html-to-markdown/src/`.

## Pipeline

- `convert()` in `convert_api.rs` checks `max_input_size`, normalizes the input, tries Tier-1, then falls back to Tier-2.
- Tier-1: byte scanner in `converter/tier1/`.
- Tier-2: astral-tl DOM walk, `walk_node` in `converter/main.rs`; html5ever is used for tree repair.
- `ConversionOptions::tier_strategy` (`Auto` default, `Tier2`, `Tier1` testkit only) picks the path.
- Per-element converters live under `converter/` (`block/`, `inline/`, `list/`, `form/`, `media/`, `semantic/`, `text/`).
- Output is shaped by `ConversionOptions` in `options/conversion.rs` (heading style, list indent, code block style, newline style, whitespace mode).
- Caller traversal hooks: `HtmlVisitor` trait in `visitor/traits.rs` (feature `visitor`). There is no `DomWalker` trait.

## Input safety (not output sanitization)

- Converted Markdown is not safe to render as trusted content; callers sanitize downstream. See `SECURITY.md`.
- `validate_input()` in `validation.rs` rejects binary input: gzip/zstd/zip/PDF magic bytes, excess NUL bytes, high control-character ratio, undeclared UTF-16. It does not inspect HTML elements, attributes or URLs.
- `max_input_size` (`Option<u64>`) returns `ConversionError::InputTooLarge`; default is `None` natively and 2 MiB on wasm32.
- `max_depth` bounds the Tier-2 walk to avoid stack overflow; `effective_max_depth` in `converter/main_helpers.rs` defaults it and clamps caller values to a ceiling.
- `ConversionError::SanitizationError` exists in `error.rs` and in binding error mappings, but the core never constructs it.
- Script and style are stripped (markup fidelity, not security); `javascript:`/`data:` URLs and event-handler attributes pass through.
- There is no `SafetyConfig`, URL scheme allow-list or attribute allow-list.
