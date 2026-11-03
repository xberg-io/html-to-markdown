---
status: accepted
date: 2026-11-02
decision-makers: ["Na'aman Hirschfeld"]
---

# Parse with a DOM built on astral-html

## Context and Problem Statement

The `astral-tl` tag scanner ends a tag name at any byte outside a fixed set. A tag such as `<x.y>` never closes, and every later link nests inside it (#965). The crate also reads the body of a raw-text element as markup, which the converter corrects in byte passes before the parse.

## Considered Options

* Keep `astral-tl` and rename dotted tags in a byte pass
* Own a fork of `tl`
* Build the DOM in this crate on the `astral_html` tokenizer

## Decision Outcome

Chosen option: "Build the DOM in this crate on the `astral_html` tokenizer", because the tokenizer follows the HTML rules for names and attributes, it builds for wasm32, and the crate stays free of unsafe code.

### Consequences

* The `dom` module owns the stack rule of the parser: an end tag closes the element on top when it spells that name, and closes nothing otherwise. The repair gates of the converter depend on this rule.
* `astral-html` is pinned exactly. It is pre-1.0, so each bump runs the document acceptance comparison.
* `astral-html` needs Rust 1.97. The workspace minimum of 1.99 covers it.
* The parse takes longer than `astral-tl` and uses less peak memory. End to end, the difference stays inside the noise of the benchmark.

## More Information

* Supersedes ADR 0001.
* The selector and stream code in `crates/html-to-markdown/src/dom/` comes from `tl` under the MIT license. See `ATTRIBUTIONS.md`.
