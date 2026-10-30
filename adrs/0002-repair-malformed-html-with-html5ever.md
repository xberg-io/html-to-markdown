---
status: accepted
date: 2025-11-21
decision-makers: ["Na'aman Hirschfeld"]
---

# Repair malformed HTML with html5ever

## Context and Problem Statement

`tl` (ADR 0001) does not apply the WHATWG tree-construction rules. On minified or malformed pages with
broken inline and block nesting, such as SPA shells and Hacker News markup, content vanished from the
output (issue 121, recorded in the 2.9.1 changelog).

## Considered Options

* Re-parse such input with html5ever and convert the repaired page
* Convert the `tl` tree as parsed (the behaviour before 2.9.1)

## Decision Outcome

Chosen option: "Re-parse such input with html5ever and convert the repaired page", because the 2.9.1
changelog records that it keeps "content that previously vanished (e.g., SPA shells and Hacker News
markup)".

* html5ever was added as a dependency in the 2.9.1 release commit e237af53a3 (2025-11-21).
* The repair runs for custom element tags before the first parse, and when the
  `tl` parse shows misnested elements, an element with no end tag or a discarded end tag.
* The repaired tree is serialized back to HTML, and that HTML is parsed by `tl` and converted as usual.
* The reference-counted DOM it builds on is vendored from `markup5ever_rcdom` 0.36.0 in
  `crates/html-to-markdown/src/rcdom.rs` (5fb583c058, 2026-02-15).

### Consequences

* Good, because malformed pages keep their text instead of losing it.
* Bad, because the core carries two HTML parsers, and a repaired page is parsed twice.
* Bad, because the repair can amplify input, so it needs a node budget and a depth limit (issues #808
  and #809).

### Confirmation

* `Cargo.toml:51` declares `html5ever`.
* `crates/html-to-markdown/src/converter/main_helpers.rs:564` defines `repair_with_html5ever_bounded`.
* `crates/html-to-markdown/src/converter/main/prepare.rs:27` repairs custom element tags, and
  `crates/html-to-markdown/src/converter/main/parse.rs:102` repairs after the first parse.
* `crates/html-to-markdown/src/converter/main_helpers/tests.rs:6` covers the bounded repair.
