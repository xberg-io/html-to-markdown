---
description: "html-to-markdown plugin system — the HtmlVisitor trait, 42 element-level callbacks, VisitResult variants, and how each binding exposes the same hooks."
---

# Plugin system (visitors)

The visitor system is the library's extensibility point. Implement `HtmlVisitor` and you can replace, skip, or augment how any HTML element becomes Markdown. No fork required.

Rust users opt in with `features = ["visitor"]`. Bindings expose the same hooks through their native idiom — anonymous class in PHP, instance with `handle_*` callbacks in Elixir, Python class with `visit_*` methods, etc. — and link against a Rust core built with the feature enabled.

## The trait

```rust
pub trait HtmlVisitor: std::fmt::Debug {
    fn visit_text(&mut self, ctx: &NodeContext, text: &str) -> VisitResult { VisitResult::Continue }
    fn visit_element_start(&mut self, ctx: &NodeContext) -> VisitResult { VisitResult::Continue }
    fn visit_element_end(&mut self, ctx: &NodeContext, output: &str) -> VisitResult { VisitResult::Continue }
    fn visit_link(&mut self, ctx: &NodeContext, href: &str, text: &str, title: Option<&str>) -> VisitResult { VisitResult::Continue }
    fn visit_image(&mut self, ctx: &NodeContext, src: &str, alt: &str, title: Option<&str>) -> VisitResult { VisitResult::Continue }
    fn visit_heading(&mut self, ctx: &NodeContext, level: u32, text: &str, id: Option<&str>) -> VisitResult { VisitResult::Continue }
    // … 36 more element-specific methods, all with `Continue` defaults
}
```

42 methods in total: text and element pre/post hooks plus a method per HTML element family (links, images, headings, lists, code, tables, definition lists, …). Override only the methods you need.

## VisitResult

Every callback returns a `VisitResult`. There are five variants:

| Variant          | Effect                                                                                |
| ---------------- | ------------------------------------------------------------------------------------- |
| `Continue`       | Use the default rendering. Default for every method.                                  |
| `Custom(String)` | Replace the default output with the supplied string. The visitor owns this subtree.   |
| `Skip`           | Drop the element and all of its children.                                             |
| `PreserveHtml`   | Emit the raw HTML for this element verbatim, without conversion.                      |
| `Error(String)`  | Halt conversion. Surfaces as `ConversionError::Visitor` in Rust.                      |

`visit_text` is hot — it fires for every text node, often 100+ times per page. Return `Continue` fast when you don't care, and avoid allocations in the hot path.

## Registration

In Rust, attach a visitor through the options builder:

```rust
use html_to_markdown_rs::visitor::{HtmlVisitor, VisitorHandle};
use html_to_markdown_rs::ConversionOptions;
use std::cell::RefCell;
use std::rc::Rc;

let visitor: VisitorHandle = Rc::new(RefCell::new(MyVisitor::default()));
let options = ConversionOptions::builder().visitor(Some(visitor)).build();
```

In other languages, the same hook is reached by passing a visitor object to the options builder. For binding-specific examples see [Guides → Visitor pattern](../visitor.md).

## Cost when unused

Without a visitor registered, the dispatch site short-circuits and the default handler runs directly — there is no virtual call, no allocation, no extra branch on the hot path. The visitor feature is opt-in for Rust users specifically so consumers who never need it pay nothing.

## Across the C FFI

When the visitor crosses the FFI boundary (Go, Java, C#, C), the C layer exposes a `HtmHtmVisitorCallbacks` struct of function pointers. The Rust side wraps each callback in a bridge that marshals strings and `NodeContext` fields and translates the returned status code into a `VisitResult`. The status codes are `HTM_VISIT_CONTINUE`, `HTM_VISIT_SKIP`, `HTM_VISIT_PRESERVE_HTML`, `HTM_VISIT_CUSTOM`, and `HTM_VISIT_ERROR`.

--8<-- "snippets/feedback.md"
