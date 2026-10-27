---
type: Reference
title: E2E Fixtures
description: "html-to-markdown e2e fixture schema: fixture arrays per fixtures/<topic>/ file, html and options input, the convert call and assertion types. Use when adding or editing an html-to-markdown e2e fixture."
paths:
  - "fixtures/**"
  - "e2e/**"
x-ai-rulez:
  kind: skill
  id: e2e-fixtures
  metadata:
    name: e2e-fixtures
---

# html-to-markdown E2E Fixtures

- Each file under `fixtures/<topic>/` holds a JSON array of fixtures; the directory is the topic and matches `docs.topic`. Topics: `conversion`, `options`, `metadata`, `visitor`, `edge-cases`, `structure`, `result`, `real-world`, `smoke`. There is no `category` or `skip` field in practice.
- Fields: `id`, `description`, `docs {topic, stem, title}`, `input`, `assertions`, and `visitor` for visitor fixtures.
- Every fixture runs the single call `convert(html, options)` (`[crates.e2e.call]` in `alef.toml`): `input.html` is the document, `input.options` an optional `ConversionOptions` object.
- Assertion types in use: `contains_all`, `contains`, `not_contains`, `not_empty`, `equals`, `count_min`, `count_equals`. Each assertion is `{type, field, value}` (`values` for `contains_all`); `field` is a dotted path into the call result. Most assert on `content`.
