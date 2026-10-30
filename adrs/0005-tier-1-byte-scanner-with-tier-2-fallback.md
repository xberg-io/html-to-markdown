---
status: accepted
date: 2026-06-01
decision-makers: ["Na'aman Hirschfeld"]
---

# Tier-1 byte scanner with Tier-2 fallback

## Context and Problem Statement

Every conversion parsed the input with `tl` and walked the DOM. The profile captured for PR #400
attributed about 35% of self-time to `walk_node`, about 20% to text-node processing and 30 to 40% to
preprocessing and the two parsers. A survey of the benchmark corpus found about 30% of the fixtures
clean enough to convert without building a DOM.

## Considered Options

* Route eligible input to a single-pass Tier-1 byte scanner that must match Tier-2 byte for byte, and
  fall back to the Tier-2 `tl` parse and walk when it bails
* Convert every input through the Tier-2 parse and walk (the behaviour before 3.6.0)

## Decision Outcome

Chosen option: "Route eligible input to a single-pass Tier-1 byte scanner that must match Tier-2 byte
for byte, and fall back to the Tier-2 `tl` parse and walk when it bails", because it skips the DOM
build and walk on input the scanner can handle, without changing any output.

* 19e6e081d4 (PR #400, 2026-06-01) added `TierStrategy`, the prescan, the router and the Tier-1
  scanner.
* 3e317334df (2026-06-09) removed the prescan from the conversion path: the scanner now skips the
  constructs the prescan used to strip, and the router is handed an empty `PrescanReport`.
* `TierStrategy::Auto` is the default. The router now decides from the options alone and routes any
  option that Tier-1 cannot honour to Tier-2.
* Tier-1 is not attempted when tags must be preserved, when inert bodies were found or when the head
  repair reached a limit. Otherwise it bails on anything it cannot convert exactly as Tier-2 would,
  and the conversion falls back to Tier-2.
* `TierStrategy::Tier2` forces the DOM path. Forcing Tier-1 exists only for tests.

### Consequences

* Good, because clean input is converted without building a DOM.
* Bad, because two converters must produce identical output, and every Tier-2 change can need a
  matching Tier-1 change or a new bail.
* Bad, because a bailed input pays for the Tier-1 attempt before Tier-2 runs.

### Confirmation

* `crates/html-to-markdown/src/options/conversion.rs:38` defines `TierStrategy`, with `Auto` as the
  default.
* `crates/html-to-markdown/src/convert_api.rs:365-372` holds the guards that skip Tier-1, `:373` passes
  `PrescanReport::default()`, and `crates/html-to-markdown/src/converter/tier1/router.rs:213` classifies
  by the options.
* `crates/html-to-markdown/tests/tier1_byte_equality_test.rs:107` fails when Tier-1 output differs from
  Tier-2 output on any benchmark fixture, and `crates/html-to-markdown/tests/tier_parity_corpus.rs`
  checks parity on a corpus.
