---
status: accepted
date: 2026-05-13
decision-makers: ["Na'aman Hirschfeld"]
---

# Visitor as a Send field of ConversionOptions

## Context and Problem Statement

The `HtmlVisitor` trait (bd50fcc1cf, 2025-12-26) lets a caller observe or override the conversion of
each element. It was first passed to dedicated functions, then to `convert` as a positional parameter,
and held in an `Rc<RefCell<dyn HtmlVisitor>>` handle. A handle that is not `Send` makes
`ConversionOptions` not `Send`, so callers could not keep options in axum, tokio or rmcp contexts that
require `Send`.

## Considered Options

* An optional `ConversionOptions.visitor` field holding an `Arc<Mutex<dyn HtmlVisitor + Send>>` handle
* An optional `ConversionOptions.visitor` field holding an `Rc<RefCell<dyn HtmlVisitor>>` handle
  (13b936e08a, 2026-05-02)
* A positional visitor parameter on `convert` (removed in 13b936e08a)
* A separate async visitor trait (removed with the 3.0.0 API in a7dde300d1, 2026-03-29)

## Decision Outcome

Chosen option: "An optional `ConversionOptions.visitor` field holding an
`Arc<Mutex<dyn HtmlVisitor + Send>>` handle", because 7f6178f252 records that it lets callers "stash
ConversionOptions inside axum/tokio/rmcp Send-bound contexts".

* 13b936e08a moved the visitor into `ConversionOptions` behind the `visitor` feature, so `convert`
  takes only `html` and `options`. It kept `Rc<RefCell<…>>` to avoid forcing `Send` on host visitors.
* 7f6178f252 (2026-05-13) switched the handle to `Arc<Mutex<…>>` and added the `Send` bound to the
  trait. The binding bridges (NAPI, WASM, Magnus, FFI) assert `Send` and `Sync` with SAFETY notes.
* The visitor is synchronous. The async visitor was removed in 3.0.0.

### Consequences

* Good, because `ConversionOptions` is `Send` and `Sync`, with or without a visitor.
* Good, because the visitor needs no extra entry point (ADR 0003).
* Bad, because every visitor callback takes a mutex lock.
* Bad, because a host visitor that is not thread-safe relies on the binding bridge's `unsafe impl` and
  its SAFETY argument.

### Confirmation

* `crates/html-to-markdown/src/options/conversion.rs:287` declares `visitor: Option<VisitorHandle>`,
  gated on the `visitor` feature at line 279.
* `crates/html-to-markdown/src/visitor/default_impl.rs:13` defines `VisitorHandle` as
  `Arc<Mutex<dyn HtmlVisitor + Send>>`.
* `crates/html-to-markdown/src/visitor/traits.rs:48` declares `HtmlVisitor: std::fmt::Debug + Send`.
* `crates/html-to-markdown/Cargo.toml:33` defines the `visitor` feature.
