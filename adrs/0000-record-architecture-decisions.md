---
status: accepted
date: 2026-10-30
decision-makers: ["Na'aman Hirschfeld"]
---

# Record architecture decisions

## Context and Problem Statement

html-to-markdown's architectural choices (the parser, the repair fallback, the public API, the
visitor, the tiered converter, the MCP server and the tracing contract) were recorded only in commit
messages, changelog entries and code comments. Nothing marked which of them are still in force, so a
later change could reverse one without saying so. The xberg-io organization keeps its org-wide
decisions as MADR records checked by adrlint, and expects each repository to keep its own decisions in
the same format.

## Considered Options

* A MADR log in `adrs/`, created and checked with adrlint, citing the org log for org-wide decisions

No alternatives were recorded.

## Decision Outcome

Chosen option: "A MADR log in `adrs/`, created and checked with adrlint, citing the org log for
org-wide decisions", because it follows
[org ADR-0000 (Record architecture decisions)](https://github.com/xberg-io/agent-conventions/blob/main/adrs/0000-record-architecture-decisions.md),
which asks each repository to record its own decisions and cite, not restate, the org records.

* Records are MADR 4 as `adrlint new` writes them, numbered from 0000 in `adrs/`.
* ADRs 0001 to 0007 are seed records: decisions already in force, dated to the commit that made them.
* Org-wide decisions are cited, not restated. They include the Rust core with alef-generated bindings
  ([org ADR-0002](https://github.com/xberg-io/agent-conventions/blob/main/adrs/0002-rust-cores-with-alef-generated-bindings.md)),
  the mise baseline and tasks
  ([org ADR-0004](https://github.com/xberg-io/agent-conventions/blob/main/adrs/0004-mise-toolchain-baseline.md),
  [org ADR-0005](https://github.com/xberg-io/agent-conventions/blob/main/adrs/0005-mise-tasks-and-shared-task-sets.md))
  and poly as the lint gate
  ([org ADR-0006](https://github.com/xberg-io/agent-conventions/blob/main/adrs/0006-poly-as-the-single-lint-and-format-gate.md)).
* An accepted record is not edited. A changed decision gets a new record that supersedes or amends it.
* The index lives outside the log, in the docs site's ADR reference page.

### Consequences

* Good, because each decision has a status, a date and evidence, so a later change must supersede it
  explicitly.
* Good, because adrlint checks the shape and numbering, and the lock check will hold accepted records
  once the log is locked.
* Bad, because the seed records were written after the fact, so their context comes from commits and
  changelog entries rather than from a discussion at the time.

### Confirmation

`.adrlint.toml` sets `dir = "adrs"` and `start = 0`. `adrlint lint` and `adrlint format --check` exit 0
on `adrs/`, and the poly hooks `adrlint-lint`, `adrlint-format-check` and `adrlint-lock-check` run
them before each commit.

## More Information

* [MADR](https://adr.github.io/madr/)
* [adrlint](https://github.com/Goldziher/adrlint)
