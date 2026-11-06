---
type: Decision
title: Output Escaping and Assembly
paths:
  - "crates/html-to-markdown/src/**"
x-ai-rulez:
  kind: rule
  id: output-escaping
  metadata:
    priority: high
---

- Escape or quote each output value for the target syntax, so a Markdown or Djot reader parses back the structure and the values of the source HTML. Test the round trip: parse the output and compare it with the source.
- Assemble output so each piece appears once, with its separator. Compute derived values, such as table column widths and offsets, after every transformation that changes the text has run.
