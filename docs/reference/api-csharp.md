---
title: "C# API Reference"
---

## C# API Reference <span class="version-badge">v3.5.0</span>

### Functions

#### Convert()

Convert HTML to Markdown, returning a `ConversionResult` with content, metadata, images,
and warnings.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```csharp
public static ConversionResult Convert(string html, ConversionOptions? options = null)
```

**Parameters:**

| Name      | Type                 | Required | Description        |
| --------- | -------------------- | -------- | ------------------ |
| `Html`    | `string`             | Yes      | The html           |
| `Options` | `ConversionOptions?` | No       | The options to use |

**Returns:** `ConversionResult`
**Errors:** Throws `Error`.

---

### Types

#### ConversionOptions

Main conversion options for HTML to Markdown conversion.

Use `ConversionOptions.builder()` to construct, or `the default constructor` for defaults.

| Field                      | Type                   | Default                      | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| -------------------------- | ---------------------- | ---------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `HeadingStyle`             | `HeadingStyle`         | `HeadingStyle.Atx`           | Heading style to use in Markdown output (ATX `#` or Setext underline).                                                                                                                                                                                                                                                                                                                                                                                                              |
| `ListIndentType`           | `ListIndentType`       | `ListIndentType.Spaces`      | How to indent nested list items (spaces or tab).                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `ListIndentWidth`          | `nuint`                | `2`                          | Number of spaces (or tabs) to use for each level of list indentation.                                                                                                                                                                                                                                                                                                                                                                                                               |
| `Bullets`                  | `string`               | `"-*+"`                      | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`).                                                                                                                                                                                                                                                                                                                                                                                                            |
| `StrongEmSymbol`           | `string`               | `"*"`                        | Character used for bold/italic emphasis markers (`*` or `_`).                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `EscapeAsterisks`          | `bool`                 | `false`                      | Escape `*` characters in plain text to avoid unintended bold/italic.                                                                                                                                                                                                                                                                                                                                                                                                                |
| `EscapeUnderscores`        | `bool`                 | `false`                      | Escape `_` characters in plain text to avoid unintended bold/italic.                                                                                                                                                                                                                                                                                                                                                                                                                |
| `EscapeMisc`               | `bool`                 | `false`                      | Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text.                                                                                                                                                                                                                                                                                                                                                                                                          |
| `EscapeAscii`              | `bool`                 | `false`                      | Escape ASCII characters that have special meaning in certain Markdown dialects.                                                                                                                                                                                                                                                                                                                                                                                                     |
| `CodeLanguage`             | `string`               | `""`                         | Default language annotation for fenced code blocks that have no language hint.                                                                                                                                                                                                                                                                                                                                                                                                      |
| `Autolinks`                | `bool`                 | `true`                       | Automatically convert bare URLs into Markdown autolinks.                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `DefaultTitle`             | `bool`                 | `false`                      | Emit a default title when no `<title>` tag is present.                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `BrInTables`               | `bool`                 | `false`                      | Render `<br>` elements inside table cells as literal line breaks.                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `HighlightStyle`           | `HighlightStyle`       | `HighlightStyle.DoubleEqual` | Style used for `<mark>` / highlighted text (e.g. `==text==`).                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `ExtractMetadata`          | `bool`                 | `true`                       | Populate `result.metadata` with `<head>` / `<meta>` extraction (title, description, Open Graph, Twitter Card, JSON-LD, …). Default `true`. Disabling skips the metadata pass only — table extraction into `result.tables` runs unconditionally.                                                                                                                                                                                                                                     |
| `WhitespaceMode`           | `WhitespaceMode`       | `WhitespaceMode.Normalized`  | Controls how whitespace is normalised during conversion.                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `StripNewlines`            | `bool`                 | `false`                      | Strip all newlines from the output, producing a single-line result.                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `Wrap`                     | `bool`                 | `false`                      | Wrap long lines at `wrap_width` characters.                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `WrapWidth`                | `nuint`                | `80`                         | Maximum line width when `wrap` is enabled (default `80`).                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `ConvertAsInline`          | `bool`                 | `false`                      | Treat the entire document as inline content (no block-level wrappers).                                                                                                                                                                                                                                                                                                                                                                                                              |
| `SubSymbol`                | `string`               | `""`                         | Markdown notation for subscript text (e.g. `"~"`).                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `SupSymbol`                | `string`               | `""`                         | Markdown notation for superscript text (e.g. `"^"`).                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `NewlineStyle`             | `NewlineStyle`         | `NewlineStyle.Spaces`        | How to encode hard line breaks (`<br>`) in Markdown.                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `CodeBlockStyle`           | `CodeBlockStyle`       | `CodeBlockStyle.Backticks`   | Style used for fenced code blocks (backticks or tilde).                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `KeepInlineImagesIn`       | `List<string>`         | `new List<string>()`         | HTML tag names whose `<img>` children are kept inline instead of block.                                                                                                                                                                                                                                                                                                                                                                                                             |
| `Preprocessing`            | `PreprocessingOptions` | —                            | Pre-processing options applied to the HTML before conversion.                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `Encoding`                 | `string`               | `"utf-8"`                    | Expected character encoding of the input HTML (default `"utf-8"`).                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `Debug`                    | `bool`                 | `false`                      | Emit debug information during conversion.                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `StripTags`                | `List<string>`         | `new List<string>()`         | HTML tag names whose content is stripped from the output entirely.                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `PreserveTags`             | `List<string>`         | `new List<string>()`         | HTML tag names that are preserved verbatim in the output.                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `SkipImages`               | `bool`                 | `false`                      | Skip conversion of `<img>` elements (omit images from output).                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `LinkStyle`                | `LinkStyle`            | `LinkStyle.Inline`           | Link rendering style (inline or reference).                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `OutputFormat`             | `OutputFormat`         | `OutputFormat.Markdown`      | Target output format (Markdown, plain text, etc.).                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `IncludeDocumentStructure` | `bool`                 | `false`                      | Include structured document tree in result.                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `ExtractImages`            | `bool`                 | `false`                      | Extract inline images from data URIs and SVGs.                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `MaxImageSize`             | `ulong`                | `5242880`                    | Maximum decoded image size in bytes (default 5MB).                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `CaptureSvg`               | `bool`                 | `false`                      | Capture SVG elements as images.                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `InferDimensions`          | `bool`                 | `true`                       | Infer image dimensions from data.                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `MaxDepth`                 | `nuint?`               | `null`                       | Maximum DOM traversal depth. `null` means unlimited. When set, subtrees beyond this depth are silently truncated.                                                                                                                                                                                                                                                                                                                                                                   |
| `ExcludeSelectors`         | `List<string>`         | `new List<string>()`         | CSS selectors for elements to exclude entirely (element + all content). Unlike `strip_tags` (which removes the tag wrapper but keeps children), excluded elements and all their descendants are dropped from the output. Supports any CSS selector that `tl` supports: tag names, `.class`, `#id`, `[attribute]`, etc. Invalid selectors are silently skipped at conversion time. Example: `vec![".cookie-banner".into(), "#ad-container".into(), "[role='complementary']".into()]` |
| `Visitor`                  | `VisitorHandle?`       | `null`                       | Optional visitor for custom traversal logic. When set, the visitor's callbacks are invoked for matching HTML elements during conversion, allowing custom output, skipping, or HTML preservation. See `HtmlVisitor`.                                                                                                                                                                                                                                                                 |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public ConversionOptions CreateDefault()
```

###### From()

**Signature:**

```csharp
public ConversionOptions From(ConversionOptionsUpdate update)
```

---

#### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field      | Type                      | Default                         | Description                                                                                                                                        |
| ---------- | ------------------------- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Content`  | `string?`                 | `null`                          | Converted text output (markdown, djot, or plain text). `null` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode. |
| `Document` | `DocumentStructure?`      | `null`                          | Structured document tree with semantic elements. Populated when `include_document_structure` is `true` in options.                                 |
| `Metadata` | `HtmlMetadata`            | —                               | Extracted HTML metadata (title, OG, links, images, structured data).                                                                               |
| `Tables`   | `List<TableData>`         | `new List<TableData>()`         | Extracted tables with structured cell data and markdown representation.                                                                            |
| `Images`   | `List<string>`            | `new List<string>()`            | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `true` in options.                                                |
| `Warnings` | `List<ProcessingWarning>` | `new List<ProcessingWarning>()` | Non-fatal processing warnings.                                                                                                                     |

---

#### DocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field           | Type                         | Default                            | Description                                                                                                              |
| --------------- | ---------------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `Title`         | `string?`                    | `null`                             | Document title from `<title>` tag                                                                                        |
| `Description`   | `string?`                    | `null`                             | Document description from `<meta name="description">` tag                                                                |
| `Keywords`      | `List<string>`               | `new List<string>()`               | Document keywords from `<meta name="keywords">` tag, split on commas                                                     |
| `Author`        | `string?`                    | `null`                             | Document author from `<meta name="author">` tag                                                                          |
| `CanonicalUrl`  | `string?`                    | `null`                             | Canonical URL from `<link rel="canonical">` tag                                                                          |
| `BaseHref`      | `string?`                    | `null`                             | Base URL from `<base href="">` tag for resolving relative URLs                                                           |
| `Language`      | `string?`                    | `null`                             | Document language from `lang` attribute                                                                                  |
| `TextDirection` | `TextDirection?`             | `null`                             | Document text direction from `dir` attribute                                                                             |
| `OpenGraph`     | `Dictionary<string, string>` | `new Dictionary<string, string>()` | Open Graph metadata (og:\* properties) for social media Keys like "title", "description", "image", "url", etc.           |
| `TwitterCard`   | `Dictionary<string, string>` | `new Dictionary<string, string>()` | Twitter Card metadata (twitter:\* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `MetaTags`      | `Dictionary<string, string>` | `new Dictionary<string, string>()` | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content           |

---

#### DocumentNode

A single node in the document tree.

| Field         | Type                          | Default | Description                                                                                |
| ------------- | ----------------------------- | ------- | ------------------------------------------------------------------------------------------ |
| `Id`          | `string`                      | —       | Deterministic node identifier.                                                             |
| `Content`     | `NodeContent`                 | —       | The semantic content of this node.                                                         |
| `Parent`      | `uint?`                       | `null`  | Index of the parent node (None for root nodes).                                            |
| `Children`    | `List<uint>`                  | —       | Indices of child nodes in reading order.                                                   |
| `Annotations` | `List<TextAnnotation>`        | —       | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text. |
| `Attributes`  | `Dictionary<string, string>?` | `null`  | Format-specific attributes (e.g. class, id, data-\* attributes).                           |

---

#### DocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field          | Type                 | Default | Description                                         |
| -------------- | -------------------- | ------- | --------------------------------------------------- |
| `Nodes`        | `List<DocumentNode>` | —       | All nodes in document reading order.                |
| `SourceFormat` | `string?`            | `null`  | The source format (always "html" for this library). |

---

#### GridCell

A single cell in a table grid.

| Field      | Type     | Default | Description                                    |
| ---------- | -------- | ------- | ---------------------------------------------- |
| `Content`  | `string` | —       | The text content of the cell.                  |
| `Row`      | `uint`   | —       | 0-indexed row position.                        |
| `Col`      | `uint`   | —       | 0-indexed column position.                     |
| `RowSpan`  | `uint`   | —       | Number of rows this cell spans (default 1).    |
| `ColSpan`  | `uint`   | —       | Number of columns this cell spans (default 1). |
| `IsHeader` | `bool`   | —       | Whether this is a header cell (`<th>`).        |

---

#### HeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field        | Type      | Default | Description                               |
| ------------ | --------- | ------- | ----------------------------------------- |
| `Level`      | `byte`    | —       | Header level: 1 (h1) through 6 (h6)       |
| `Text`       | `string`  | —       | Normalized text content of the header     |
| `Id`         | `string?` | `null`  | HTML id attribute if present              |
| `Depth`      | `nuint`   | —       | Document tree depth at the header element |
| `HtmlOffset` | `nuint`   | —       | Byte offset in original HTML document     |

##### Methods

###### IsValid()

Validate that the header level is within valid range (1-6).

**Returns:**

`true` if level is 1-6, `false` otherwise.

**Signature:**

```csharp
public bool IsValid()
```

---

#### HtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field            | Type                   | Default                      | Description                                                   |
| ---------------- | ---------------------- | ---------------------------- | ------------------------------------------------------------- |
| `Document`       | `DocumentMetadata`     | —                            | Document-level metadata (title, description, canonical, etc.) |
| `Headers`        | `List<HeaderMetadata>` | `new List<HeaderMetadata>()` | Extracted header elements with hierarchy                      |
| `Links`          | `List<LinkMetadata>`   | `new List<LinkMetadata>()`   | Extracted hyperlinks with type classification                 |
| `Images`         | `List<ImageMetadata>`  | `new List<ImageMetadata>()`  | Extracted images with source and dimensions                   |
| `StructuredData` | `List<StructuredData>` | `new List<StructuredData>()` | Extracted structured data blocks                              |

---

#### HtmlVisitor

Visitor trait for HTML→Markdown conversion.

Implement this trait to customize the conversion behavior for any HTML element type.
All methods have default implementations that return `VisitResult.Continue`, allowing
selective override of only the elements you care about.

## Method Naming Convention

- `visit_*_start`: Called before entering an element (pre-order traversal)
- `visit_*_end`: Called after exiting an element (post-order traversal)
- `visit_*`: Called for specific element types (e.g., `visit_link`, `visit_image`)

## Execution Order

For a typical element like `<div><p>text</p></div>`:

1. `visit_element_start` for `<div>`
2. `visit_element_start` for `<p>`
3. `visit_text` for "text"
4. `visit_element_end` for `<p>`
5. `visit_element_end` for `</div>`

## Performance Notes

- `visit_text` is the most frequently called method (~100+ times per document)
- Return `VisitResult.Continue` quickly for elements you don't need to customize
- Avoid heavy computation in visitor methods; consider caching if needed

### Methods

#### VisitText()

Visit text nodes (most frequent callback - ~100+ per document).

**Signature:**

```csharp
public VisitResult VisitText(NodeContext ctx, string text)
```

##### VisitElementStart()

Called before entering any element.

This is the first callback invoked for every HTML element, allowing
visitors to implement generic element handling before tag-specific logic.

**Signature:**

```csharp
public VisitResult VisitElementStart(NodeContext ctx)
```

###### VisitElementEnd()

Called after exiting any element.

Receives the default markdown output that would be generated.
Visitors can inspect or replace this output.

**Signature:**

```csharp
public VisitResult VisitElementEnd(NodeContext ctx, string output)
```

###### VisitLink()

Visit anchor links `<a href="...">`.

**Signature:**

```csharp
public VisitResult VisitLink(NodeContext ctx, string href, string text, string title)
```

###### VisitImage()

Visit images `<img src="...">`.

**Signature:**

```csharp
public VisitResult VisitImage(NodeContext ctx, string src, string alt, string title)
```

###### VisitHeading()

Visit heading elements `<h1>` through `<h6>`.

**Signature:**

```csharp
public VisitResult VisitHeading(NodeContext ctx, uint level, string text, string id)
```

###### VisitCodeBlock()

Visit code blocks `<pre><code>`.

**Signature:**

```csharp
public VisitResult VisitCodeBlock(NodeContext ctx, string lang, string code)
```

###### VisitCodeInline()

Visit inline code `<code>`.

**Signature:**

```csharp
public VisitResult VisitCodeInline(NodeContext ctx, string code)
```

###### VisitListItem()

Visit list items `<li>`.

**Signature:**

```csharp
public VisitResult VisitListItem(NodeContext ctx, bool ordered, string marker, string text)
```

###### VisitListStart()

Called before processing a list `<ul>` or `<ol>`.

**Signature:**

```csharp
public VisitResult VisitListStart(NodeContext ctx, bool ordered)
```

###### VisitListEnd()

Called after processing a list `</ul>` or `</ol>`.

**Signature:**

```csharp
public VisitResult VisitListEnd(NodeContext ctx, bool ordered, string output)
```

###### VisitTableStart()

Called before processing a table `<table>`.

**Signature:**

```csharp
public VisitResult VisitTableStart(NodeContext ctx)
```

###### VisitTableRow()

Visit table rows `<tr>`.

**Signature:**

```csharp
public VisitResult VisitTableRow(NodeContext ctx, List<string> cells, bool isHeader)
```

###### VisitTableEnd()

Called after processing a table `</table>`.

**Signature:**

```csharp
public VisitResult VisitTableEnd(NodeContext ctx, string output)
```

###### VisitBlockquote()

Visit blockquote elements `<blockquote>`.

**Signature:**

```csharp
public VisitResult VisitBlockquote(NodeContext ctx, string content, nuint depth)
```

###### VisitStrong()

Visit strong/bold elements `<strong>`, `<b>`.

**Signature:**

```csharp
public VisitResult VisitStrong(NodeContext ctx, string text)
```

###### VisitEmphasis()

Visit emphasis/italic elements `<em>`, `<i>`.

**Signature:**

```csharp
public VisitResult VisitEmphasis(NodeContext ctx, string text)
```

###### VisitStrikethrough()

Visit strikethrough elements `<s>`, `<del>`, `<strike>`.

**Signature:**

```csharp
public VisitResult VisitStrikethrough(NodeContext ctx, string text)
```

###### VisitUnderline()

Visit underline elements `<u>`, `<ins>`.

**Signature:**

```csharp
public VisitResult VisitUnderline(NodeContext ctx, string text)
```

###### VisitSubscript()

Visit subscript elements `<sub>`.

**Signature:**

```csharp
public VisitResult VisitSubscript(NodeContext ctx, string text)
```

###### VisitSuperscript()

Visit superscript elements `<sup>`.

**Signature:**

```csharp
public VisitResult VisitSuperscript(NodeContext ctx, string text)
```

###### VisitMark()

Visit mark/highlight elements `<mark>`.

**Signature:**

```csharp
public VisitResult VisitMark(NodeContext ctx, string text)
```

###### VisitLineBreak()

Visit line break elements `<br>`.

**Signature:**

```csharp
public VisitResult VisitLineBreak(NodeContext ctx)
```

###### VisitHorizontalRule()

Visit horizontal rule elements `<hr>`.

**Signature:**

```csharp
public VisitResult VisitHorizontalRule(NodeContext ctx)
```

###### VisitCustomElement()

Visit custom elements (web components) or unknown tags.

**Signature:**

```csharp
public VisitResult VisitCustomElement(NodeContext ctx, string tagName, string html)
```

###### VisitDefinitionListStart()

Visit definition list `<dl>`.

**Signature:**

```csharp
public VisitResult VisitDefinitionListStart(NodeContext ctx)
```

###### VisitDefinitionTerm()

Visit definition term `<dt>`.

**Signature:**

```csharp
public VisitResult VisitDefinitionTerm(NodeContext ctx, string text)
```

###### VisitDefinitionDescription()

Visit definition description `<dd>`.

**Signature:**

```csharp
public VisitResult VisitDefinitionDescription(NodeContext ctx, string text)
```

###### VisitDefinitionListEnd()

Called after processing a definition list `</dl>`.

**Signature:**

```csharp
public VisitResult VisitDefinitionListEnd(NodeContext ctx, string output)
```

###### VisitForm()

Visit form elements `<form>`.

**Signature:**

```csharp
public VisitResult VisitForm(NodeContext ctx, string action, string method)
```

###### VisitInput()

Visit input elements `<input>`.

**Signature:**

```csharp
public VisitResult VisitInput(NodeContext ctx, string inputType, string name, string value)
```

###### VisitButton()

Visit button elements `<button>`.

**Signature:**

```csharp
public VisitResult VisitButton(NodeContext ctx, string text)
```

###### VisitAudio()

Visit audio elements `<audio>`.

**Signature:**

```csharp
public VisitResult VisitAudio(NodeContext ctx, string src)
```

###### VisitVideo()

Visit video elements `<video>`.

**Signature:**

```csharp
public VisitResult VisitVideo(NodeContext ctx, string src)
```

###### VisitIframe()

Visit iframe elements `<iframe>`.

**Signature:**

```csharp
public VisitResult VisitIframe(NodeContext ctx, string src)
```

###### VisitDetails()

Visit details elements `<details>`.

**Signature:**

```csharp
public VisitResult VisitDetails(NodeContext ctx, bool open)
```

###### VisitSummary()

Visit summary elements `<summary>`.

**Signature:**

```csharp
public VisitResult VisitSummary(NodeContext ctx, string text)
```

###### VisitFigureStart()

Visit figure elements `<figure>`.

**Signature:**

```csharp
public VisitResult VisitFigureStart(NodeContext ctx)
```

###### VisitFigcaption()

Visit figcaption elements `<figcaption>`.

**Signature:**

```csharp
public VisitResult VisitFigcaption(NodeContext ctx, string text)
```

###### VisitFigureEnd()

Called after processing a figure `</figure>`.

**Signature:**

```csharp
public VisitResult VisitFigureEnd(NodeContext ctx, string output)
```

---

##### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field        | Type                         | Default | Description                                             |
| ------------ | ---------------------------- | ------- | ------------------------------------------------------- |
| `Src`        | `string`                     | —       | Image source (URL, data URI, or SVG content identifier) |
| `Alt`        | `string?`                    | `null`  | Alternative text from alt attribute (for accessibility) |
| `Title`      | `string?`                    | `null`  | Title attribute (often shown as tooltip)                |
| `Dimensions` | `List<uint>?`                | `null`  | Image dimensions as (width, height) if available        |
| `ImageType`  | `ImageType`                  | —       | Image type classification                               |
| `Attributes` | `Dictionary<string, string>` | —       | Additional HTML attributes                              |

---

##### LinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field        | Type                         | Default | Description                                                         |
| ------------ | ---------------------------- | ------- | ------------------------------------------------------------------- |
| `Href`       | `string`                     | —       | The href URL value                                                  |
| `Text`       | `string`                     | —       | Link text content (normalized, concatenated if mixed with elements) |
| `Title`      | `string?`                    | `null`  | Optional title attribute (often shown as tooltip)                   |
| `LinkType`   | `LinkType`                   | —       | Link type classification                                            |
| `Rel`        | `List<string>`               | —       | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical")  |
| `Attributes` | `Dictionary<string, string>` | —       | Additional HTML attributes                                          |

---

##### NodeContext

Context information passed to all visitor methods.

Provides comprehensive metadata about the current node being visited,
including its type, attributes, position in the DOM tree, and parent context.

| Field           | Type                         | Default | Description                                             |
| --------------- | ---------------------------- | ------- | ------------------------------------------------------- |
| `NodeType`      | `NodeType`                   | —       | Coarse-grained node type classification                 |
| `TagName`       | `string`                     | —       | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `Attributes`    | `Dictionary<string, string>` | —       | All HTML attributes as key-value pairs                  |
| `Depth`         | `nuint`                      | —       | Depth in the DOM tree (0 = root)                        |
| `IndexInParent` | `nuint`                      | —       | Index among siblings (0-based)                          |
| `ParentTag`     | `string?`                    | `null`  | Parent element's tag name (None if root)                |
| `IsInline`      | `bool`                       | —       | Whether this element is treated as inline vs block      |

---

##### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field              | Type                  | Default                        | Description                                                    |
| ------------------ | --------------------- | ------------------------------ | -------------------------------------------------------------- |
| `Enabled`          | `bool`                | `true`                         | Enable HTML preprocessing globally                             |
| `Preset`           | `PreprocessingPreset` | `PreprocessingPreset.Standard` | Preprocessing preset level (Minimal, Standard, Aggressive)     |
| `RemoveNavigation` | `bool`                | `true`                         | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `RemoveForms`      | `bool`                | `true`                         | Remove form elements (forms, inputs, buttons, etc.)            |

###### Methods

###### CreateDefault()

**Signature:**

```csharp
public PreprocessingOptions CreateDefault()
```

###### From()

**Signature:**

```csharp
public PreprocessingOptions From(PreprocessingOptionsUpdate update)
```

---

##### ProcessingWarning

A non-fatal warning generated during HTML processing.

| Field     | Type          | Default | Description                     |
| --------- | ------------- | ------- | ------------------------------- |
| `Message` | `string`      | —       | Human-readable warning message. |
| `Kind`    | `WarningKind` | —       | The category of warning.        |

---

##### StructuredData

Structured data block (JSON-LD, Microdata, or RDFa).

Represents machine-readable structured data found in the document.
JSON-LD blocks are collected as raw JSON strings for flexibility.

| Field        | Type                 | Default | Description                                                     |
| ------------ | -------------------- | ------- | --------------------------------------------------------------- |
| `DataType`   | `StructuredDataType` | —       | Type of structured data (JSON-LD, Microdata, RDFa)              |
| `RawJson`    | `string`             | —       | Raw JSON string (for JSON-LD) or serialized representation      |
| `SchemaType` | `string?`            | `null`  | Schema type if detectable (e.g., "Article", "Event", "Product") |

---

##### TableData

A top-level extracted table with both structured data and markdown representation.

| Field      | Type        | Default | Description                           |
| ---------- | ----------- | ------- | ------------------------------------- |
| `Grid`     | `TableGrid` | —       | The structured table grid.            |
| `Markdown` | `string`    | —       | The markdown rendering of this table. |

---

##### TableGrid

A structured table grid with cell-level data including spans.

| Field   | Type             | Default                | Description                                                         |
| ------- | ---------------- | ---------------------- | ------------------------------------------------------------------- |
| `Rows`  | `uint`           | —                      | Number of rows.                                                     |
| `Cols`  | `uint`           | —                      | Number of columns.                                                  |
| `Cells` | `List<GridCell>` | `new List<GridCell>()` | All cells in the table (may be fewer than rows\*cols due to spans). |

---

##### TextAnnotation

An inline text annotation with byte-range offsets.

Annotations describe formatting (bold, italic, etc.) and links within a node's text content.

| Field   | Type             | Default | Description                                                |
| ------- | ---------------- | ------- | ---------------------------------------------------------- |
| `Start` | `uint`           | —       | Start byte offset (inclusive) into the parent node's text. |
| `End`   | `uint`           | —       | End byte offset (exclusive) into the parent node's text.   |
| `Kind`  | `AnnotationKind` | —       | The type of annotation.                                    |

---

##### VisitorHandle

Type alias for a visitor handle (`Arc`-wrapped `Mutex` for thread-safe shared mutation).

`Send + Sync` so that types embedding a `VisitorHandle` (e.g. `ConversionOptions`)
can be shared across threads — required by callers that stash configs inside
axum/rmcp/tokio Send-bound contexts.

---

#### Enums

##### TextDirection

Text directionality of document content.

Corresponds to the HTML `dir` attribute and `bdi` element directionality.

| Value         | Description                                          |
| ------------- | ---------------------------------------------------- |
| `LeftToRight` | Left-to-right text flow (default for Latin scripts)  |
| `RightToLeft` | Right-to-left text flow (Hebrew, Arabic, Urdu, etc.) |
| `Auto`        | Automatic directionality detection                   |

---

##### LinkType

Link classification based on href value and document context.

Used to categorize links during extraction for filtering and analysis.

| Value      | Description                                           |
| ---------- | ----------------------------------------------------- |
| `Anchor`   | Anchor link within same document (href starts with #) |
| `Internal` | Internal link within same domain                      |
| `External` | External link to different domain                     |
| `Email`    | Email link (mailto:)                                  |
| `Phone`    | Phone link (tel:)                                     |
| `Other`    | Other protocol or unclassifiable                      |

---

##### ImageType

Image source classification for proper handling and processing.

Determines whether an image is embedded (data URI), inline SVG, external, or relative.

| Value       | Description                                        |
| ----------- | -------------------------------------------------- |
| `DataUri`   | Data URI embedded image (base64 or other encoding) |
| `InlineSvg` | Inline SVG element                                 |
| `External`  | External image URL (http/https)                    |
| `Relative`  | Relative image path                                |

---

##### StructuredDataType

Structured data format type.

Identifies the schema/format used for structured data markup.

| Value       | Description                                                |
| ----------- | ---------------------------------------------------------- |
| `JsonLd`    | JSON-LD (JSON for Linking Data) script blocks              |
| `Microdata` | HTML5 Microdata attributes (itemscope, itemtype, itemprop) |
| `RDFa`      | RDF in Attributes (RDFa) markup                            |

---

##### PreprocessingPreset

HTML preprocessing aggressiveness level.

Controls the extent of cleanup performed before conversion. Higher levels remove more elements.

| Value        | Description                                                                        |
| ------------ | ---------------------------------------------------------------------------------- |
| `Minimal`    | Minimal cleanup. Remove only essential noise (scripts, styles).                    |
| `Standard`   | Standard cleanup. Default. Removes navigation, forms, and other auxiliary content. |
| `Aggressive` | Aggressive cleanup. Remove extensive non-content elements and structure.           |

---

##### HeadingStyle

Heading style options for Markdown output.

Controls how headings (h1-h6) are rendered in the output Markdown.

| Value        | Description                                        |
| ------------ | -------------------------------------------------- |
| `Underlined` | Underlined style (=== for h1, --- for h2).         |
| `Atx`        | ATX style (# for h1, ## for h2, etc.). Default.    |
| `AtxClosed`  | ATX closed style (# title #, with closing hashes). |

---

##### ListIndentType

List indentation character type.

Controls whether list items are indented with spaces or tabs.

| Value    | Description                                                                   |
| -------- | ----------------------------------------------------------------------------- |
| `Spaces` | Use spaces for indentation. Default. Width controlled by `list_indent_width`. |
| `Tabs`   | Use tabs for indentation.                                                     |

---

##### WhitespaceMode

Whitespace handling strategy during conversion.

Determines how sequences of whitespace characters (spaces, tabs, newlines) are processed.

| Value        | Description                                                                                  |
| ------------ | -------------------------------------------------------------------------------------------- |
| `Normalized` | Collapse multiple whitespace characters to single spaces. Default. Matches browser behavior. |
| `Strict`     | Preserve all whitespace exactly as it appears in the HTML.                                   |

---

##### NewlineStyle

Line break syntax in Markdown output.

Controls how soft line breaks (from `<br>` or line breaks in source) are rendered.

| Value       | Description                                                            |
| ----------- | ---------------------------------------------------------------------- |
| `Spaces`    | Two trailing spaces at end of line. Default. Standard Markdown syntax. |
| `Backslash` | Backslash at end of line. Alternative Markdown syntax.                 |

---

##### CodeBlockStyle

Code block fence style in Markdown output.

Determines how code blocks (`<pre><code>`) are rendered in Markdown.

| Value       | Description                                                                      |
| ----------- | -------------------------------------------------------------------------------- |
| `Indented`  | Indented code blocks (4 spaces). `CommonMark` standard.                          |
| `Backticks` | Fenced code blocks with backticks (```). Default (GFM). Supports language hints. |
| `Tildes`    | Fenced code blocks with tildes (~~~). Supports language hints.                   |

---

##### HighlightStyle

Highlight rendering style for `<mark>` elements.

Controls how highlighted text is rendered in Markdown output.

| Value         | Description                                                  |
| ------------- | ------------------------------------------------------------ |
| `DoubleEqual` | Double equals syntax (==text==). Default. Pandoc-compatible. |
| `Html`        | Preserve as HTML (==text==). Original HTML tag.              |
| `Bold`        | Render as bold (**text**). Uses strong emphasis.             |
| `None`        | Strip formatting, render as plain text. No markup.           |

---

##### LinkStyle

Link rendering style in Markdown output.

Controls whether links and images use inline `[text](url)` syntax or
reference-style `[text][1]` syntax with definitions collected at the end.

| Value       | Description                                                            |
| ----------- | ---------------------------------------------------------------------- |
| `Inline`    | Inline links: `[text](url)`. Default.                                  |
| `Reference` | Reference-style links: `[text][1]` with `[1]: url` at end of document. |

---

##### OutputFormat

Output format for conversion.

Specifies the target markup language format for the conversion output.

| Value      | Description                                         |
| ---------- | --------------------------------------------------- |
| `Markdown` | Standard Markdown (CommonMark compatible). Default. |
| `Djot`     | Djot lightweight markup language.                   |
| `Plain`    | Plain text output (no markup, visible text only).   |

---

##### NodeContent

The semantic content type of a document node.

Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.

| Value            | Description                                                                                                                                        |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Heading`        | A heading element (h1-h6). — Fields: `Level`: `byte`, `Text`: `string`                                                                             |
| `Paragraph`      | A paragraph of text. — Fields: `Text`: `string`                                                                                                    |
| `List`           | A list container (ordered or unordered). Children are `ListItem` nodes. — Fields: `Ordered`: `bool`                                                |
| `ListItem`       | A single list item. — Fields: `Text`: `string`                                                                                                     |
| `Table`          | A table with structured cell data. — Fields: `Grid`: `TableGrid`                                                                                   |
| `Image`          | An image element. — Fields: `Description`: `string`, `Src`: `string`, `ImageIndex`: `uint`                                                         |
| `Code`           | A code block or inline code. — Fields: `Text`: `string`, `Language`: `string`                                                                      |
| `Quote`          | A block quote container.                                                                                                                           |
| `DefinitionList` | A definition list container.                                                                                                                       |
| `DefinitionItem` | A definition list entry with term and description. — Fields: `Term`: `string`, `Definition`: `string`                                              |
| `RawBlock`       | A raw block preserved as-is (e.g. `<script>`, `<style>` content). — Fields: `Format`: `string`, `Content`: `string`                                |
| `MetadataBlock`  | A block of key-value metadata pairs (from `<head>` meta tags). — Fields: `Entries`: `List<string>`                                                 |
| `Group`          | A section grouping container (auto-generated from heading hierarchy). — Fields: `Label`: `string`, `HeadingLevel`: `byte`, `HeadingText`: `string` |

---

##### AnnotationKind

The type of an inline text annotation.

Uses internally tagged representation (`"annotation_type": "bold"`) for JSON serialization.

| Value           | Description                                               |
| --------------- | --------------------------------------------------------- |
| `Bold`          | Bold / strong emphasis.                                   |
| `Italic`        | Italic / emphasis.                                        |
| `Underline`     | Underline.                                                |
| `Strikethrough` | Strikethrough / deleted text.                             |
| `Code`          | Inline code.                                              |
| `Subscript`     | Subscript text.                                           |
| `Superscript`   | Superscript text.                                         |
| `Highlight`     | Highlighted / marked text.                                |
| `Link`          | A hyperlink. — Fields: `Url`: `string`, `Title`: `string` |

---

##### WarningKind

Categories of processing warnings.

| Value                   | Description                                                                  |
| ----------------------- | ---------------------------------------------------------------------------- |
| `ImageExtractionFailed` | An image could not be extracted (e.g. invalid data URI, unsupported format). |
| `EncodingFallback`      | The input encoding was not recognized; fell back to UTF-8.                   |
| `TruncatedInput`        | The input was truncated due to size limits.                                  |
| `MalformedHtml`         | The HTML was malformed but processing continued with best effort.            |
| `SanitizationApplied`   | Sanitization was applied to remove potentially unsafe content.               |
| `DepthLimitExceeded`    | DOM traversal was truncated because max_depth was exceeded.                  |

---

##### NodeType

Node type enumeration covering all HTML element types.

This enum categorizes all HTML elements that the converter recognizes,
providing a coarse-grained classification for visitor dispatch.

| Value                   | Description                                    |
| ----------------------- | ---------------------------------------------- |
| `Text`                  | Text node (most frequent - 100+ per document)  |
| `Element`               | Generic element node                           |
| `Heading`               | Heading elements (h1-h6)                       |
| `Paragraph`             | Paragraph element                              |
| `Div`                   | Generic div container                          |
| `Blockquote`            | Blockquote element                             |
| `Pre`                   | Preformatted text block                        |
| `Hr`                    | Horizontal rule                                |
| `List`                  | Ordered or unordered list (ul, ol)             |
| `ListItem`              | List item (li)                                 |
| `DefinitionList`        | Definition list (dl)                           |
| `DefinitionTerm`        | Definition term (dt)                           |
| `DefinitionDescription` | Definition description (dd)                    |
| `Table`                 | Table element                                  |
| `TableRow`              | Table row (tr)                                 |
| `TableCell`             | Table cell (td, th)                            |
| `TableHeader`           | Table header cell (th)                         |
| `TableBody`             | Table body (tbody)                             |
| `TableHead`             | Table head (thead)                             |
| `TableFoot`             | Table foot (tfoot)                             |
| `Link`                  | Anchor link (a)                                |
| `Image`                 | Image (img)                                    |
| `Strong`                | Strong/bold (strong, b)                        |
| `Em`                    | Emphasis/italic (em, i)                        |
| `Code`                  | Inline code (code)                             |
| `Strikethrough`         | Strikethrough (s, del, strike)                 |
| `Underline`             | Underline (u, ins)                             |
| `Subscript`             | Subscript (sub)                                |
| `Superscript`           | Superscript (sup)                              |
| `Mark`                  | Mark/highlight (mark)                          |
| `Small`                 | Small text (small)                             |
| `Br`                    | Line break (br)                                |
| `Span`                  | Span element                                   |
| `Article`               | Article element                                |
| `Section`               | Section element                                |
| `Nav`                   | Navigation element                             |
| `Aside`                 | Aside element                                  |
| `Header`                | Header element                                 |
| `Footer`                | Footer element                                 |
| `Main`                  | Main element                                   |
| `Figure`                | Figure element                                 |
| `Figcaption`            | Figure caption                                 |
| `Time`                  | Time element                                   |
| `Details`               | Details element                                |
| `Summary`               | Summary element                                |
| `Form`                  | Form element                                   |
| `Input`                 | Input element                                  |
| `Select`                | Select element                                 |
| `Option`                | Option element                                 |
| `Button`                | Button element                                 |
| `Textarea`              | Textarea element                               |
| `Label`                 | Label element                                  |
| `Fieldset`              | Fieldset element                               |
| `Legend`                | Legend element                                 |
| `Audio`                 | Audio element                                  |
| `Video`                 | Video element                                  |
| `Picture`               | Picture element                                |
| `Source`                | Source element                                 |
| `Iframe`                | Iframe element                                 |
| `Svg`                   | SVG element                                    |
| `Canvas`                | Canvas element                                 |
| `Ruby`                  | Ruby annotation                                |
| `Rt`                    | Ruby text                                      |
| `Rp`                    | Ruby parenthesis                               |
| `Abbr`                  | Abbreviation                                   |
| `Kbd`                   | Keyboard input                                 |
| `Samp`                  | Sample output                                  |
| `Var`                   | Variable                                       |
| `Cite`                  | Citation                                       |
| `Q`                     | Quote                                          |
| `Del`                   | Deleted text                                   |
| `Ins`                   | Inserted text                                  |
| `Data`                  | Data element                                   |
| `Meter`                 | Meter element                                  |
| `Progress`              | Progress element                               |
| `Output`                | Output element                                 |
| `Template`              | Template element                               |
| `Slot`                  | Slot element                                   |
| `Html`                  | HTML root element                              |
| `Head`                  | Head element                                   |
| `Body`                  | Body element                                   |
| `Title`                 | Title element                                  |
| `Meta`                  | Meta element                                   |
| `LinkTag`               | Link element (not anchor)                      |
| `Style`                 | Style element                                  |
| `Script`                | Script element                                 |
| `Base`                  | Base element                                   |
| `Custom`                | Custom element (web components) or unknown tag |

---

##### VisitResult

Result of a visitor callback.

Allows visitors to control the conversion flow by either proceeding
with default behavior, providing custom output, skipping elements,
preserving HTML, or signaling errors.

| Value          | Description                                                                                                                                                      |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Continue`     | Continue with default conversion behavior                                                                                                                        |
| `Custom`       | Replace default output with custom markdown The visitor takes full responsibility for the markdown output of this node and its children. — Fields: `0`: `string` |
| `Skip`         | Skip this element entirely (don't output anything) The element and all its children are ignored in the output.                                                   |
| `PreserveHtml` | Preserve original HTML (don't convert to markdown) The element's raw HTML is included verbatim in the output.                                                    |
| `Error`        | Stop conversion with an error The conversion process halts and returns this error message. — Fields: `0`: `string`                                               |

---

#### Errors

##### ConversionError

Errors that can occur during HTML to Markdown conversion.

| Variant             | Description                             |
| ------------------- | --------------------------------------- |
| `ParseError`        | HTML parsing error                      |
| `SanitizationError` | HTML sanitization error                 |
| `ConfigError`       | Invalid configuration                   |
| `IoError`           | I/O error                               |
| `Panic`             | Internal error caught during conversion |
| `InvalidInput`      | Invalid input data                      |
| `Other`             | Generic conversion error                |

---
