---
status: accepted
date: 2025-11-20
decision-makers: ["Na'aman Hirschfeld"]
---

# Parse with the astral-tl fork of tl

## Context and Problem Statement

The Rust core has parsed HTML with the `tl` crate since the Rust workspace was added (`tl = "0.7"` in
ca4bc5389c, 2025-10-10). Version 2.9.0 fixed comment parsing defects, for example zero-width
`<!---->` comments that made the following content disappear, and its changelog names `astral-tl` as
the actively maintained fork of `tl`.

## Considered Options

* Upstream `tl` 0.7
* The `astral-tl` fork, imported as `tl`

## Decision Outcome

Chosen option: "The `astral-tl` fork, imported as `tl`", because the 2.9.0 changelog records it as
"the actively maintained `astral-tl` fork (still imported as `tl`) so comment parsing stays up to date
with upstream fixes".

* The workspace dependency became `astral-tl` in 59bbb8b605 and was renamed back to `tl` through
  `package = "astral-tl"` in 8d45cb7880 (both 2025-11-20), so the converter source kept its `tl::`
  paths.
* `tl` builds the DOM that the Tier-2 converter walks (ADR 0005). html5ever only repairs input that
  `tl` gets wrong (ADR 0002).

### Consequences

* Good, because parser fixes come from a maintained fork without rewriting the converter's `tl::` code.
* Bad, because the project depends on a fork's maintainers, and moving off it again means a new parser
  integration.
* Bad, because `tl` is not a WHATWG tree builder, so malformed input needs a separate repair path.

### Confirmation

* `Cargo.toml:62` declares `tl = { package = "astral-tl", version = "0.8.0" }`, and
  `crates/html-to-markdown/Cargo.toml:68` uses it.
* `crates/html-to-markdown/src/converter/main/parse.rs:26` parses every Tier-2 conversion with
  `tl::parse`.
* `changelog-archive-5.md:241` holds the 2.9.0 changelog entry.
