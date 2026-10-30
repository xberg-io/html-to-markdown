---
title: Architecture decisions
description: "Index of html-to-markdown's architecture decision records (ADRs) in adrs/, with status and date."
---

html-to-markdown records its architecture decisions as [MADR](https://adr.github.io/madr/) files in
[`adrs/`](https://github.com/xberg-io/html-to-markdown/tree/main/adrs), checked by
[adrlint](https://github.com/Goldziher/adrlint). Org-wide decisions live in the
[agent-conventions log](https://github.com/xberg-io/agent-conventions/tree/main/adrs). Add a row here in
the same change as a new record.

| Number | Title | Status | Date |
| ------ | ----- | ------ | ---- |
| 0000 | [Record architecture decisions](https://github.com/xberg-io/html-to-markdown/blob/main/adrs/0000-record-architecture-decisions.md) | accepted | 2026-10-30 |
| 0001 | [Parse with the astral-tl fork of tl](https://github.com/xberg-io/html-to-markdown/blob/main/adrs/0001-parse-with-the-astral-tl-fork-of-tl.md) | accepted | 2025-11-20 |
| 0002 | [Repair malformed HTML with html5ever](https://github.com/xberg-io/html-to-markdown/blob/main/adrs/0002-repair-malformed-html-with-html5ever.md) | accepted | 2025-11-21 |
| 0003 | [One convert entry point returning ConversionResult](https://github.com/xberg-io/html-to-markdown/blob/main/adrs/0003-one-convert-entry-point-returning-conversionresult.md) | accepted | 2026-03-29 |
| 0004 | [Visitor as a Send field of ConversionOptions](https://github.com/xberg-io/html-to-markdown/blob/main/adrs/0004-visitor-as-a-send-field-of-conversionoptions.md) | accepted | 2026-05-13 |
| 0005 | [Tier-1 byte scanner with Tier-2 fallback](https://github.com/xberg-io/html-to-markdown/blob/main/adrs/0005-tier-1-byte-scanner-with-tier-2-fallback.md) | accepted | 2026-06-01 |
| 0006 | [MCP server as a feature-gated core module](https://github.com/xberg-io/html-to-markdown/blob/main/adrs/0006-mcp-server-as-a-feature-gated-core-module.md) | accepted | 2026-06-20 |
| 0007 | [Tracing as the observability surface](https://github.com/xberg-io/html-to-markdown/blob/main/adrs/0007-tracing-as-the-observability-surface.md) | accepted | 2026-07-30 |
