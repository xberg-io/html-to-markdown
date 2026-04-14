# Visitor Pattern

The visitor system is the library's main extensibility point. Implement `HtmlVisitor` and you can replace, skip, or augment how any HTML element becomes Markdown. No fork required.

Rust users must opt in with `features = ["visitor"]`. The other bindings expose the visitor through their native idiom (`Visitor` interface in Java, callback object in Python, etc.) and link against a Rust core built with the feature enabled.

## Execution Order

Traversal is pre-order. For `<div><p>text</p></div>`:

1. `visit_element_start` fires for `<div>`
2. `visit_element_start` fires for `<p>`
3. `visit_text` fires for `"text"`
4. `visit_element_end` fires for `<p>` with the rendered output
5. `visit_element_end` fires for `<div>` with the rendered output

`visit_text` is hot. It runs for every text node in the document, often 100+ times on a single page. Return `Continue` fast when you don't care about the node, and avoid allocations in the method body.

## VisitResult

Every callback returns a `VisitResult`.

| Variant | Effect |
|---------|--------|
| `Continue` | Use the default rendering. |
| `Custom(String)` | Replace the default output with the supplied Markdown. The visitor owns the rendering for this node and its children. |
| `Skip` | Drop the element and all of its children. |
| `PreserveHtml` | Emit the raw HTML for this element verbatim. |
| `Error(String)` | Halt conversion. The message surfaces as `ConversionError::Visitor` in Rust (behind `features = ["visitor"]`). |

## NodeContext

Every callback receives a `NodeContext` describing the current node.

| Field | Type | Meaning |
|-------|------|---------|
| `node_type` | `NodeType` | Coarse-grained classification (heading, list, link, form, …). 87 variants. |
| `tag_name` | `String` | Raw HTML tag name. Lowercased. |
| `attributes` | `BTreeMap<String, String>` | All attributes on the element. |
| `depth` | `usize` | Depth in the DOM tree. Root is 0. |
| `index_in_parent` | `usize` | 0-based position among siblings. |
| `parent_tag` | `Option<String>` | Parent element's tag, or `None` at the root. |
| `is_inline` | `bool` | `true` when the element is rendered inline (inside a paragraph, link text, cell, …). |

## Method Reference

All 40 methods have default implementations that return `Continue`. Override only the ones you care about.

### Generic element callbacks

| Method | When it fires |
|--------|---------------|
| `visit_element_start(ctx)` | Before any element. First callback for every node. |
| `visit_element_end(ctx, output)` | After an element, with the rendered Markdown. |
| `visit_text(ctx, text)` | Every text node. HTML entities already decoded. |
| `visit_custom_element(ctx, tag_name, html)` | Unknown tags and web components. |

### Links and images

| Method | Arguments |
|--------|-----------|
| `visit_link(ctx, href, text, title)` | `<a>` anchor with href, rendered text, and optional title. |
| `visit_image(ctx, src, alt, title)` | `<img>` with src, alt text, and optional title. |

### Headings, rules, breaks

| Method | Arguments |
|--------|-----------|
| `visit_heading(ctx, level, text, id)` | `<h1>`–`<h6>` with level (1-6), text, and optional id. |
| `visit_horizontal_rule(ctx)` | `<hr>`. |
| `visit_line_break(ctx)` | `<br>`. |

### Code

| Method | Arguments |
|--------|-----------|
| `visit_code_block(ctx, lang, code)` | `<pre><code>` with language tag and raw code. |
| `visit_code_inline(ctx, code)` | Inline `<code>`. |

### Lists

| Method | Arguments |
|--------|-----------|
| `visit_list_start(ctx, ordered)` | Before `<ul>` or `<ol>`. |
| `visit_list_item(ctx, ordered, marker, text)` | Each `<li>` with marker and rendered text. |
| `visit_list_end(ctx, ordered, output)` | After the list, with the rendered block. |

### Definition lists

| Method | Arguments |
|--------|-----------|
| `visit_definition_list_start(ctx)` | Before `<dl>`. |
| `visit_definition_term(ctx, text)` | `<dt>`. |
| `visit_definition_description(ctx, text)` | `<dd>`. |
| `visit_definition_list_end(ctx, output)` | After `<dl>`. |

### Tables

| Method | Arguments |
|--------|-----------|
| `visit_table_start(ctx)` | Before `<table>`. |
| `visit_table_row(ctx, cells, is_header)` | Each `<tr>`. Cells are pre-rendered Markdown. `is_header` is true for rows inside `<thead>`. |
| `visit_table_end(ctx, output)` | After `<table>`. |

### Blockquote

| Method | Arguments |
|--------|-----------|
| `visit_blockquote(ctx, content, depth)` | `<blockquote>` with rendered content and nesting depth. |

### Inline formatting

| Method | Covers |
|--------|--------|
| `visit_strong(ctx, text)` | `<strong>`, `<b>`. |
| `visit_emphasis(ctx, text)` | `<em>`, `<i>`. |
| `visit_strikethrough(ctx, text)` | `<s>`, `<del>`, `<strike>`. |
| `visit_underline(ctx, text)` | `<u>`, `<ins>`. |
| `visit_subscript(ctx, text)` | `<sub>`. |
| `visit_superscript(ctx, text)` | `<sup>`. |
| `visit_mark(ctx, text)` | `<mark>`. |

### Forms

| Method | Arguments |
|--------|-----------|
| `visit_form(ctx, action, method)` | `<form>` with optional action URL and method. |
| `visit_input(ctx, input_type, name, value)` | `<input>`. |
| `visit_button(ctx, text)` | `<button>`. |

### Media

| Method | Arguments |
|--------|-----------|
| `visit_audio(ctx, src)` | `<audio>`. |
| `visit_video(ctx, src)` | `<video>`. |
| `visit_iframe(ctx, src)` | `<iframe>`. |

### Interactive

| Method | Arguments |
|--------|-----------|
| `visit_details(ctx, open)` | `<details>` with the `open` attribute. |
| `visit_summary(ctx, text)` | `<summary>`. |

### Figures

| Method | Arguments |
|--------|-----------|
| `visit_figure_start(ctx)` | Before `<figure>`. |
| `visit_figcaption(ctx, text)` | `<figcaption>`. |
| `visit_figure_end(ctx, output)` | After `<figure>`. |

## Basic Visitor

=== "Rust"
    --8<-- "snippets/rust/visitor/basic_visitor.md"

=== "Python"
    --8<-- "snippets/python/visitor/basic_visitor.md"

=== "TypeScript"
    --8<-- "snippets/typescript/visitor/basic_visitor.md"

=== "Go"
    --8<-- "snippets/go/visitor/basic_visitor.md"

=== "Ruby"
    --8<-- "snippets/ruby/visitor/basic_visitor.md"

=== "PHP"
    --8<-- "snippets/php/visitor/basic_visitor.md"

=== "Java"
    --8<-- "snippets/java/visitor/basic_visitor.md"

=== "C#"
    --8<-- "snippets/csharp/visitor/basic_visitor.md"

=== "Elixir"
    --8<-- "snippets/elixir/visitor/basic_visitor.md"

=== "R"
    --8<-- "snippets/r/visitor/basic_visitor.md"

=== "C"
    --8<-- "snippets/c/visitor/basic_visitor.md"

=== "WASM"
    --8<-- "snippets/wasm/visitor/basic_visitor.md"

## Common Patterns

### Link rewriting

Override `visit_link`, return `VisitResult::Custom(...)` with the new URL baked in. Useful for rewriting relative links to absolute, stripping tracking parameters, or converting internal links to anchor references.

### Element filtering

Override `visit_element_start` and return `VisitResult::Skip` when `ctx.tag_name` matches an unwanted tag. The element and every descendant is dropped. A class filter works too: check `ctx.attributes.get("class")` and skip on match.

### Content extraction

Override `visit_text` and push each text fragment into an external buffer. The visitor becomes a simple text-extraction pass that bypasses Markdown rendering. Combine with `Skip` on unwanted elements to exclude code blocks, navigation, or footers.

## Performance

`visit_text` fires on every text node. Keep the handler small. Match the few element kinds you care about in `visit_element_start` and return `Continue` for everything else. Allocations inside the handler multiply by the number of text nodes in the input.

The visitor trait is synchronous. The core walker calls each method in place during the single-pass DOM traversal. There is no queuing or batching.

--8<-- "snippets/feedback.md"
