# Table Extraction

Every call to `convert()` populates `result.tables` with one entry per `<table>` found in the input. Each entry has both a rendered Markdown string and a structured cell grid, so you can embed the Markdown in downstream documents or walk the grid for analysis without re-parsing.

Table extraction runs on every call. There is no opt-in flag. Set `output_format` to `"none"` if you only want the table data and not the rendered content.

## TableData

`result.tables` is a `Vec<TableData>` (or the equivalent list in each binding).

| Field | Type | Description |
|-------|------|-------------|
| `grid` | `TableGrid` | Structured cell grid. |
| `markdown` | `String` | The Markdown rendering of this table, identical to what appears in `result.content`. |

## TableGrid

| Field | Type | Description |
|-------|------|-------------|
| `rows` | `u32` | Number of rows in the table. |
| `cols` | `u32` | Number of columns in the table. |
| `cells` | `Vec<GridCell>` | Flat list of cells. May be shorter than `rows * cols` when cells span multiple rows or columns. |

## GridCell

| Field | Type | Description |
|-------|------|-------------|
| `content` | `String` | Cell text. Inline formatting is flattened to plain text. |
| `row` | `u32` | 0-indexed row position. |
| `col` | `u32` | 0-indexed column position. |
| `row_span` | `u32` | How many rows the cell occupies. Defaults to `1`. |
| `col_span` | `u32` | How many columns the cell occupies. Defaults to `1`. |
| `is_header` | `bool` | `true` for `<th>`, `false` for `<td>`. |

## Basic Extraction

=== "Rust"
    --8<-- "snippets/rust/table-extraction/basic_extraction.md"

=== "Python"
    --8<-- "snippets/python/table-extraction/basic_extraction.md"

=== "TypeScript"
    --8<-- "snippets/typescript/table-extraction/basic_extraction.md"

=== "Go"
    --8<-- "snippets/go/table-extraction/basic_extraction.md"

=== "Ruby"
    --8<-- "snippets/ruby/table-extraction/basic_extraction.md"

=== "PHP"
    --8<-- "snippets/php/table-extraction/basic_extraction.md"

=== "Java"
    --8<-- "snippets/java/table-extraction/basic_extraction.md"

=== "C#"
    --8<-- "snippets/csharp/table-extraction/basic_extraction.md"

=== "Elixir"
    --8<-- "snippets/elixir/table-extraction/basic_extraction.md"

=== "R"
    --8<-- "snippets/r/table-extraction/basic_extraction.md"

=== "C"
    --8<-- "snippets/c/table-extraction/basic_extraction.md"

=== "WASM"
    --8<-- "snippets/wasm/table-extraction/basic_extraction.md"

## Relationship to `result.content`

The Markdown in `TableData.markdown` is the same Markdown that appears inline inside `result.content`. The grid exists for code that needs cell-level access: headers vs body rows, span detection, or programmatic lookup by `(row, col)`.

If the input has no tables, `result.tables` is an empty list. If the output format is `"plain"` or `"none"`, tables are still extracted and their grids are still populated; only the Markdown rendering in `result.content` changes.

## Spans

A cell with `row_span > 1` or `col_span > 1` appears once in `cells`, positioned at its top-left coordinates. Downstream code that iterates by `(row, col)` should respect the span or use the spans to reconstruct a dense grid.

--8<-- "snippets/feedback.md"
