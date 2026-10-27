---
type: Reference
title: Scoop Manifest Layout
paths:
  - "scripts/publish/html-to-markdown.json.tmpl"
  - "scripts/publish/scoop.json"
  - ".github/workflows/publish.yaml"
x-ai-rulez:
  kind: rule
  id: scoop-manifest
  metadata:
    priority: medium
---

- The Windows zip nests the binary in a directory named after the asset without the product prefix, so `extract_dir` is `cli-x86_64-pc-windows-msvc`. The layout comes from `reusable-cli-release.yml` (`asset-prefix: cli-` in .github/workflows/publish.yaml).
