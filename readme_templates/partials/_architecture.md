## Architecture

The converter routes each input through one of three tiers based on a fast prescan of the byte stream:

1. **Tier-1 — single-pass byte scanner.** Handles 110+ HTML tags directly. Bails on any construct it cannot prove byte-equivalent to Tier-2.
2. **Tier-2 — DOM walker.** Picks up Tier-1 bails and inputs the classifier rejected up front.
3. **Tier-3 — standards-conformant parser.** Engaged for malformed HTML requiring full HTML5 repair.

The dispatcher is invisible to the caller. Output is byte-identical across tiers — enforced by a 116-snapshot oracle.

## Capabilities

- **16 languages, one Rust core.** Rust, Python, Node.js, WASM, Java, Go, C#, PHP, Ruby, Elixir, R, Dart, Kotlin (Android), Swift, Zig, C ABI.
- **CommonMark-compatible Markdown** with GFM-style tables.
- **Djot output**: set `output_format = "djot"` (see Djot Output Format section below).
- **Real-HTML robust**: unclosed tags, CDATA, custom elements, malformed entities, nested tables, mixed encodings handled without losing content.
- **Metadata extraction**, **visitor API**, **inline images**, **configurable preprocessing presets**.
- **Per-group regression gates in CI**: every PR runs the bench harness against per-group thresholds.
