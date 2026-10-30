---
status: accepted
date: 2026-06-20
decision-makers: ["Na'aman Hirschfeld"]
---

# MCP server as a feature-gated core module

## Context and Problem Statement

Coding agents call tools over the Model Context Protocol (MCP). Serving the converter over MCP
needs rmcp, tokio, schemars and, for HTTP, axum, while the library and its bindings are synchronous
and had none of those dependencies.

## Considered Options

* An rmcp server in the core crate behind the `mcp` and `mcp-http` features, run by an `mcp`
  subcommand of the existing CLI

No alternatives were recorded.

## Decision Outcome

Chosen option: "An rmcp server in the core crate behind the `mcp` and `mcp-http` features, run by an
`mcp` subcommand of the existing CLI", because e1b39e57c9 (2026-06-20) records that with every MCP
dependency optional, "the default and all binding builds are unchanged (no tokio)".

* e1b39e57c9 shipped one `convert_html` tool that took the `ConversionOptions` surface as an untyped
  JSON blob. c7d4b39a96 (2026-06-22) replaced the blob with a typed `ConvertConfig` that MCP clients
  discover through the tool's input schema, and added a second tool, `extract_metadata`, that returns
  the document metadata as JSON.
* stdio is the default transport, and HTTP is behind `mcp-http`, which implies `mcp`. The flag names
  follow [org ADR-0011 (Shared Cargo feature flag names)](https://github.com/xberg-io/agent-conventions/blob/main/adrs/0011-shared-cargo-feature-flag-names.md).
* The MCP code is outside the alef-generated surface, so no binding exposes it.
* The CLI enables `mcp` by default. 696bffba1b (2026-08-11) made `mcp-http` opt-in, because a CLI
  installed from brew, cargo, npm or pip must not ship an unauthenticated HTTP listener by default.

### Consequences

* Good, because the library and the bindings build without tokio or rmcp.
* Good, because users install one binary for both conversion and the MCP server.
* Bad, because the core crate carries server code that only the CLI uses.

### Confirmation

* `crates/html-to-markdown/Cargo.toml:36` and `crates/html-to-markdown/Cargo.toml:37` define the `mcp`
  and `mcp-http` features, and `crates/html-to-markdown/src/lib.rs:134` gates the `mcp` module on
  `mcp` alone.
* `crates/html-to-markdown/src/mcp/params.rs:447-459` defines the typed `config` parameter of
  `convert_html`, and `crates/html-to-markdown/src/mcp/server.rs:144-160` declares `extract_metadata`.
* `crates/html-to-markdown/src/mcp/server.rs:298` and
  `crates/html-to-markdown/src/mcp/server.rs:329` start the stdio and HTTP servers.
* `crates/html-to-markdown-cli/Cargo.toml:37` sets `default = ["mcp"]`, and
  `crates/html-to-markdown-cli/src/args.rs:26` declares the `mcp` subcommand.
