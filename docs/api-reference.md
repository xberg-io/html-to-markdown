# API Reference

Every type listed here is part of the public API re-exported at the crate root. For the full Rust surface, see [`lib.rs`](https://github.com/kreuzberg-dev/html-to-markdown/blob/main/crates/html-to-markdown/src/lib.rs). The other bindings translate these types into language-native shapes but keep the field names.

## Result Types

### `ConversionResult`

Returned from every `convert()` call.

| Field | Type | Description |
|-------|------|-------------|
| `content` | `Option<String>` | Rendered output. `None` when `output_format` is `"none"`. |
| `document` | `Option<DocumentStructure>` | Parsed semantic tree, populated only when `include_document_structure` is enabled. |
| `metadata` | `HtmlMetadata` | Document metadata, headers, links, images, structured data. Only compiled with `features = ["metadata"]` (default). |
| `tables` | `Vec<TableData>` | Every `<table>` in the input, with grid and rendered Markdown. Always populated. |
| `images` | `Vec<InlineImage>` | Extracted inline images. Only compiled with `features = ["inline-images"]`. |
| `warnings` | `Vec<ProcessingWarning>` | Non-fatal issues raised during conversion. |

### `ProcessingWarning`

| Field | Type | Description |
|-------|------|-------------|
| `message` | `String` | Human-readable warning text. |
| `kind` | `WarningKind` | One of `image_extraction_failed`, `encoding_fallback`, `truncated_input`, `malformed_html`, `sanitization_applied`. |

## Document Types

### `DocumentStructure`

| Field | Type | Description |
|-------|------|-------------|
| `nodes` | `Vec<DocumentNode>` | All nodes in reading order. |
| `source_format` | `Option<String>` | Always `"html"` for this crate. |

### `DocumentNode`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `String` | Deterministic node identifier. |
| `content` | `NodeContent` | Semantic payload. See below. |
| `parent` | `Option<u32>` | Index of the parent node, or `None` for roots. |
| `children` | `Vec<u32>` | Indices of child nodes in reading order. |
| `annotations` | `Vec<TextAnnotation>` | Inline formatting annotations with byte offsets into the node's text. |
| `attributes` | `Option<HashMap<String, String>>` | Original HTML attributes for this node. |

### `NodeContent`

Tagged enum (`node_type` discriminator). Thirteen variants.

| Variant | Payload |
|---------|---------|
| `Heading` | `level: u8`, `text: String` |
| `Paragraph` | `text: String` |
| `List` | `ordered: bool` (children hold the items) |
| `ListItem` | `text: String` |
| `Table` | `grid: TableGrid` |
| `Image` | `description`, `src`, `image_index` (all optional) |
| `Code` | `text: String`, `language: Option<String>` |
| `Quote` | (none) |
| `DefinitionList` | (none) |
| `DefinitionItem` | `term: String`, `definition: String` |
| `RawBlock` | `format: String`, `content: String` |
| `MetadataBlock` | `entries: Vec<(String, String)>` |
| `Group` | `label`, `heading_level`, `heading_text` (all optional) |

### `TextAnnotation`

| Field | Type | Description |
|-------|------|-------------|
| `start` | `u32` | Start byte offset into the parent node's text. |
| `end` | `u32` | End byte offset (exclusive). |
| `kind` | `AnnotationKind` | See below. |

### `AnnotationKind`

Tagged enum (`annotation_type` discriminator). Nine variants.

| Variant | Payload |
|---------|---------|
| `Bold` | (none) |
| `Italic` | (none) |
| `Underline` | (none) |
| `Strikethrough` | (none) |
| `Code` | (none) |
| `Subscript` | (none) |
| `Superscript` | (none) |
| `Highlight` | (none) |
| `Link` | `url: String`, `title: Option<String>` |

## Table Types

See [Tables](tables.md) for a fuller discussion. Shapes:

### `TableData`

| Field | Type |
|-------|------|
| `grid` | `TableGrid` |
| `markdown` | `String` |

### `TableGrid`

| Field | Type |
|-------|------|
| `rows` | `u32` |
| `cols` | `u32` |
| `cells` | `Vec<GridCell>` |

### `GridCell`

| Field | Type | Default |
|-------|------|---------|
| `content` | `String` | — |
| `row` | `u32` | — |
| `col` | `u32` | — |
| `row_span` | `u32` | `1` |
| `col_span` | `u32` | `1` |
| `is_header` | `bool` | `false` |

## Metadata Types

### `HtmlMetadata`

| Field | Type | Description |
|-------|------|-------------|
| `document` | `DocumentMetadata` | Head-level data (see below). |
| `headers` | `Vec<HeaderMetadata>` | Every `<h1>`–`<h6>`. |
| `links` | `Vec<LinkMetadata>` | Every `<a>`. |
| `images` | `Vec<ImageMetadata>` | Every `<img>`. |
| `structured_data` | `Vec<StructuredData>` | JSON-LD, Microdata, RDFa blocks. |

### `DocumentMetadata`

| Field | Type | Source |
|-------|------|--------|
| `title` | `Option<String>` | `<title>` element. |
| `description` | `Option<String>` | `<meta name="description">`. |
| `keywords` | `Vec<String>` | `<meta name="keywords">`, split on commas. |
| `author` | `Option<String>` | `<meta name="author">`. |
| `canonical_url` | `Option<String>` | `<link rel="canonical">`. |
| `base_href` | `Option<String>` | `<base href="…">`. |
| `language` | `Option<String>` | `<html lang="…">`. |
| `text_direction` | `Option<TextDirection>` | `<html dir="…">`. Values: `left_to_right`, `right_to_left`, `auto`. |
| `open_graph` | `BTreeMap<String, String>` | Every `og:*` meta tag (without the `og:` prefix). |
| `twitter_card` | `BTreeMap<String, String>` | Every `twitter:*` meta tag (without the prefix). |
| `meta_tags` | `BTreeMap<String, String>` | Every other `<meta name>` tag. |

### `HeaderMetadata`

| Field | Type |
|-------|------|
| `level` | `u8` (1-6) |
| `text` | `String` |
| `id` | `Option<String>` |
| `depth` | `usize` (DOM depth) |
| `html_offset` | `usize` (byte offset in original HTML) |

### `LinkMetadata`

| Field | Type |
|-------|------|
| `href` | `String` |
| `text` | `String` |
| `title` | `Option<String>` |
| `link_type` | `LinkType` |
| `rel` | `Vec<String>` (e.g. `["nofollow"]`) |
| `attributes` | `BTreeMap<String, String>` |

### `ImageMetadata`

| Field | Type |
|-------|------|
| `src` | `String` |
| `alt` | `Option<String>` |
| `title` | `Option<String>` |
| `dimensions` | `Option<(u32, u32)>` |
| `image_type` | `ImageType` |
| `attributes` | `BTreeMap<String, String>` |

### `StructuredData`

| Field | Type |
|-------|------|
| `data_type` | `StructuredDataType` (`json_ld`, `microdata`, `rdfa`) |
| `raw_json` | `String` |
| `schema_type` | `Option<String>` (e.g. `"Article"`) |

### `MetadataConfig`

Controls which metadata categories are populated. See [Configuration: Metadata Extraction](configuration.md#metadata-extraction) for the user-facing flags that map onto this type.

## Inline Image Types

Available when `features = ["inline-images"]` is enabled.

### `InlineImageConfig`

| Field | Type | Default |
|-------|------|---------|
| `max_decoded_size_bytes` | `u64` | `5_242_880` (5 MB) |
| `filename_prefix` | `Option<String>` | `"embedded_image"` |
| `capture_svg` | `bool` | `true` |
| `infer_dimensions` | `bool` | `false` |

### `InlineImage`

| Field | Type | Description |
|-------|------|-------------|
| `data` | `Vec<u8>` | Raw bytes in the original encoding. |
| `format` | `InlineImageFormat` | Detected or inferred format. |
| `filename` | `Option<String>` | Generated or extracted filename. |
| `description` | `Option<String>` | Alt text. |
| `dimensions` | `Option<(u32, u32)>` | Present only if `infer_dimensions` is on. |
| `source` | `InlineImageSource` | Either a data URI or an inline SVG. |
| `attributes` | `BTreeMap<String, String>` | Additional HTML attributes from the source element. |

## Option Types

See [Configuration](configuration.md) for the full user-facing options. The Rust types are re-exported from the crate root: `ConversionOptions`, `ConversionOptionsUpdate`, `HeadingStyle`, `ListIndentType`, `CodeBlockStyle`, `NewlineStyle`, `HighlightStyle`, `LinkStyle`, `WhitespaceMode`, `OutputFormat`, `PreprocessingOptions`, `PreprocessingOptionsUpdate`, `PreprocessingPreset`.

## Error Types

See [Error Handling](errors.md) for the full variant list. The type is `ConversionError`, and `Result<T>` is re-exported from `html_to_markdown_rs`.

## Visitor Types

See [Visitor Pattern](visitor.md) for the trait and execution model. The types are `HtmlVisitor` (the trait), `NodeContext`, `NodeType`, and `VisitResult`, all gated behind `features = ["visitor"]`.

## Supported HTML Elements

`NodeType` enumerates every element the visitor and converter recognise. 87 variants covering text, block containers, inline formatting, tables, lists, forms, media, ruby, interactive elements, and document-level tags. See [`visitor/types.rs`](https://github.com/kreuzberg-dev/html-to-markdown/blob/main/crates/html-to-markdown/src/visitor/types.rs) for the full list. Unknown or custom elements fall through to `NodeType::Custom` and reach the visitor as `visit_custom_element`.

## Generated Docs (future work)

This page is hand-written for voice control and completeness in a single place. A later iteration could generate much of it from `cargo doc` comments; the type-level docstrings in the source are already detailed enough to feed a generator. Contributions welcome.

--8<-- "snippets/feedback.md"
