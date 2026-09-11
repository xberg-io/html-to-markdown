# Changelog

All notable changes to html-to-markdown will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [3.12.4] - 2026-09-11

### Fixed

- Treat adjacent duplicate Rust warning flags as equivalent in benchmark provenance checks while preserving timing gates.
- Publish every native NuGet runtime package required by the managed package's runtime graph.
- Upload Dart native archives and checksums required by the generated package downloader.
- Publish Go installer archive aliases and required SHA-256 sidecars.
- Verify the installed Homebrew CLI and FFI versions directly in the registry smoke task.
- Stage original native archives through the shared Zig packager; the 3.12.3 source archive requires separate C FFI libraries.
- Merge adjacent inline emphasis into a single delimiter run, so `<i>A</i><i>B</i><i>C</i>`
  renders as `*ABC*` rather than the `*A**B**C*` CommonMark reparses as nested emphasis
  ([#483](https://github.com/xberg-io/html-to-markdown/issues/483)).
- Emit exactly one space for a whitespace-only inline element; `A<i> </i>B` duplicated it and,
  inside a paragraph, `<p>A<i> </i>B</p>` dropped it entirely
  ([#481](https://github.com/xberg-io/html-to-markdown/issues/481)).
- Drive the musl Node cross-compile through the shared build action, whose per-leg artifact
  staging replaces the `napi artifacts` call that `@napi-rs/cli` 3.9.1 made fail on any
  single-target matrix leg.
- Keep the visible text of a `<tr>` nested directly inside a `<td>`, a shape malformed
  newsletter HTML produces; the row and its content were dropped silently
  ([#486](https://github.com/xberg-io/html-to-markdown/issues/486)).
- Render a data table's nested single-cell table as its own table instead of flattening it
  into a cell of escaped pipes ([#484](https://github.com/xberg-io/html-to-markdown/issues/484)).
- Apply the WHATWG numeric character reference replacement table, so `&#155;` decodes to
  `›` rather than a raw C1 control character; null, surrogate and out-of-range
  references now yield U+FFFD ([#485](https://github.com/xberg-io/html-to-markdown/issues/485)).

## [3.12.3] - 2026-09-09

### Fixed

- Preserve visible images inside `font-size: 0` wrappers ([#476](https://github.com/xberg-io/html-to-markdown/issues/476)).
- Recover table cells following malformed Office namespace tags
  ([#477](https://github.com/xberg-io/html-to-markdown/issues/477)).
- Render data tables inside one-cell layout wrappers as separate Markdown tables
  ([#478](https://github.com/xberg-io/html-to-markdown/issues/478)).
- Repair nested anchors so both link destinations remain clickable
  ([#479](https://github.com/xberg-io/html-to-markdown/issues/479)).
- Resolve unpublished native Node packages from the workspace during frozen documentation installs.
- Build prerelease PHP extensions against the local Rust core and committed dependency lockfile.
- Preserve native error codes on typed C# conversion exceptions.
- Prevent panics when nested list visitors shorten earlier output or replace it with multibyte text
  ([#474](https://github.com/xberg-io/html-to-markdown/issues/474)).
- Generate R and Ruby bindings that pass strict Clippy checks while preserving visitor and conversion behavior.
- Accept explicit `nil` for optional Ruby conversion fields.
- Use canonical URL-escape enum values in fixtures so Node and WebAssembly suites regenerate completely.
- Point Ruby dependency-update tasks at the native crate and remove the obsolete duplicate Rust workspace.

### Changed

- Refactor MCP helpers, image construction, and visitor state to remove three lint suppressions while preserving
  preprocessing validation order ([#466](https://github.com/xberg-io/html-to-markdown/pull/466)).
- Regenerate all bindings, end-to-end suites, and snippets with Alef 0.85.11.
- Refresh dependency lockfiles across Rust workspaces.

## [3.12.2] - 2026-09-07

### Fixed

- **`bullets` now applies to layout-table rows**
  ([#472](https://github.com/xberg-io/html-to-markdown/issues/472)). A table with inconsistent
  column counts and no `<th>`/`<caption>` is treated as a layout table and renders each row as a
  list item — but that renderer hardcoded `-` and never read `options.bullets`, so configuring
  the option had no effect on the markers actually emitted. Surfaced by the reporter of #470, who
  set `bullets` to `"*+-"` in 3.12.0 and still got hyphens.

  The marker now cycles through `bullets` by nesting depth the same way list items do, so a
  layout table nested inside a list takes the next marker rather than repeating its parent's. The
  prefix strip that prevents a doubled-up marker assumed a hyphen as well, and now accepts any
  configured bullet.

  Default output is unchanged — the default `bullets` string already starts with `-`. Tier-1
  bails on layout tables, so this is a Tier-2 path with no parity mirror.

## [3.12.1] - 2026-09-07

Correctness release covering the four defects reported against 3.12.0. Three are conversion bugs
in the core, one extends the hidden-element rule; all four were reported through the Java binding
and reproduce identically in every language.

### Fixed

- **An uppercase void element no longer swallows the rest of the document**
  ([#467](https://github.com/xberg-io/html-to-markdown/issues/467)). The bundled `astral-tl`
  parser matches void elements against an all-lowercase table using a tag's raw source bytes, so
  `<META>` missed it and was pushed onto the open-element stack as a *container*: the parent's
  close tag could not pop it and every following sibling was absorbed as its child. For
  `<head><META ...></head><body>` that left `<body>` a grandchild of `<head>`, out of reach of the
  direct-child rescue in `handle_head`, and the whole document converted to an empty string. The
  `charset` attribute in the report was incidental -- the crate has no charset handling, and every
  HTML5 void element (`<BR>`, `<IMG>`, `<HR>`, `<INPUT>`, `<LINK>`, ...) reproduced it. Tier-2 now
  lowercases void-element tag *names* during preprocessing; attribute names and values are left
  byte for byte alone. Tier-1 already lowercased before lookup, so this also closes a tier
  divergence.

- **A nested table inside a cell keeps its row boundaries under `br_in_tables`**
  ([#469](https://github.com/xberg-io/html-to-markdown/issues/469)). A Markdown cell cannot hold a
  nested table, so the inner table is flattened into the outer cell with its pipes escaped. Until
  3.11.2, `br_in_tables: true` merely skipped the whole-cell newline fold, letting the inner rows
  leak out of the cell as raw newlines -- malformed, but a GFM parser could still see two rows.
  Making that fold unconditional (correctly, for issues #456 and #457: a raw newline between two
  pipes splits the row across physical lines) collapsed the rows onto one line joined by spaces
  and erased the boundaries. The fold stays unconditional; the flattened rows are now joined with
  the literal `<br>` that `br_in_tables` already means everywhere else in a cell. A preceding
  sibling is separated from the nested table too, which previously ran straight into its first
  pipe (`Before\| ID`). **This restores row boundaries, not table structure** -- a real nested GFM
  table remains impossible and the inner pipes stay escaped.

- **Adjacent paragraphs in a layout-table cell are separated**
  ([#470](https://github.com/xberg-io/html-to-markdown/issues/470)). A table with inconsistent
  column counts and no `<th>`/`<caption>` renders each row as a list item, and those cells convert
  as inline. That suppressed the ordinary block separator while never reaching the table-cell
  continuation rule, so `<p><b>Alice Example</b></p><p><i>Customer Service</i></p>` emitted
  `**Alice Example***Customer Service*` -- merged words and invalid emphasis. Such cells now
  follow the settled cell rule from issues #453/#454: a literal `<br>` under `br_in_tables`, a single
  space otherwise. `<div>` continuations are covered by the same change. Layout rows are list
  items, so they still do not take table-cell pipe or emphasis escaping. Not a regression -- this
  predates 3.8.3.

### Changed

- **`font-size: 0` now marks an element as not rendered**
  ([#468](https://github.com/xberg-io/html-to-markdown/issues/468)), joining `display: none`,
  `visibility: hidden` and the `hidden` attribute. Reported against generated email banners whose
  marker text reached the Markdown despite being invisible in a browser. Any exact zero length is
  recognised (`0`, `0px`, `0.0em`, `.0%`, in any casing, with or without `!important`) and the CSS
  last-declaration-wins cascade applies as it already does to `display`.

  **This drops content that previous versions emitted.** One case is deliberately exempt: the same
  declaration is the classic inline-block/email spacing hack, where the wrapper kills inter-child
  whitespace and each child restores a readable size. When a descendant re-declares a non-zero
  `font-size`, the subtree is kept. That guard is a one-level-of-inheritance heuristic, not a
  cascade -- a size restored from a stylesheet is out of reach of a byte-level pass. Tier-1 has no
  subtree awareness and bails on any `font-size: 0`, deferring to the tier that can see the
  descendant.

  Detection remains unconditional, as it has always been for the other three: no option governs it.

## [3.12.0] - 2026-08-31

Correctness release. A broad pass over converter correctness: defects in the shipped output,
plus a run of cases where the Tier-1 fast scanner and the Tier-2 converter disagreed on the same
input -- the library picks a tier automatically, so those meant one document could convert two
ways.

Test coverage behind it: every one of the 652 `CommonMark` spec examples is now exercised through
a conversion-fixpoint oracle (the exact-match test compares against the spec's own rendering, so
it can only run on the 131 examples admitting a single valid form), a Tier-1/Tier-2 differential
oracle over the benchmark corpus and a generated document set, and a fuzz target.

Against that fixpoint oracle, 644 of the 652 examples now round-trip unchanged with escaping
enabled and 629 with the shipped defaults, up from 607 and 597. Of the 8 that remain, three are inherent to the round trip -- adjacent
block quotes, and adjacent lists sharing a bullet, merge under any compliant reparse whatever we
emit -- and the other five differ only in the byte spelling of a link destination.

### Removed

- **`htm_conversion_options_update_visitor` is gone from the C API.** The getter documented a
  non-null return as caller-owned data, but `ConversionOptionsUpdate` is only ever constructed
  through `htm_conversion_options_update_from_json`, and its `visitor` field carries a full
  `serde(skip)` -- so `Deserialize` could never populate it and no FFI setter existed. The symbol
  could only ever return `0`. **Breaking for C consumers:** it disappears from
  `html_to_markdown.h` and from the compiled library, so a call that used to yield `NULL` at
  runtime is now a link error. Nothing that worked stops working. `ConversionOptions.visitor` is
  unaffected -- it carries the same attribute but is genuinely reachable, because
  `htm_options_set_visitor` writes it directly, bypassing serde.

### Added

- CLI flags for six `ConversionOptions` fields that the library and the MCP surface already
  exposed but the CLI hardcoded: `--exclude-selectors`, `--url-escape-style`,
  `--max-image-size`, `--capture-svg`, `--no-infer-dimensions` and `--tier-strategy`. Each
  now defaults to the library's own default, so an invocation that omits them is unchanged.
  The three image flags require `--extract-inline-images`, which they are inert without.

### Performance

- `escape_link_label` returns `Cow<'_, str>` and skips its two allocations when the text
  contains no `[` or `]`, which is the common case for link labels, image alt text and
  titles. The two-pass design is unchanged -- it exists to avoid an O(n^2) `String::insert`
  shape.

### Fixed

- **Uppercase attribute names no longer discard link destinations and image sources.**
  HTML attribute names are case-insensitive, but the Tier-2 converter matched them
  byte-for-byte as written, so `<a HREF="up.html">link</a>` converted to bare `link` and
  `<img SRC="a.png" ALT="cat">` to `![](<>)` -- the destination and the alt text were
  dropped, not merely reformatted. The Tier-1 fast scanner has always compared attribute
  names case-insensitively, so these inputs were also a tier disagreement: the same
  document converted two ways depending on which tier ran. The `astral-tl` 0.8.0 parser
  upgrade lowercases attribute keys at parse time, which fixes both.
- **Tier-1 and Tier-2 disagreed on GFM autolinks.** `autolinks` defaults to `true` but was
  not gated in the Tier-1 router, and the Tier-1 scanner had no autolink branch, so
  `<a href="https://x.com">https://x.com</a>` converted to `<https://x.com>` on Tier-2 and
  `[https://x.com](https://x.com)` on Tier-1 -- the library picks a tier automatically, so
  the same document converted two ways. Tier-1 now implements the autolink form rather than
  being gated off it, since gating a default-`true` option would have made the fast path
  unreachable for ordinary input.
- **Tier-1 could return the wrong output for an autolink whose label carried inline markup.**
  Tier-2 tests its autolink predicate against the tag-stripped text, so
  `<a href="https://x.com"><b>https://x.com</b></a>` still autolinks there; Tier-1 compared
  its rendered label (`**https://x.com**`), never matched, and emitted
  `[**https://x.com**](https://x.com)`. Tier-1 now bails to Tier-2 on this shape. The bail is
  guarded by a subsequence precheck so it fires only when an autolink is actually possible:
  on the benchmark corpus, none of the 889 scheme-href links that carry nested markup trigger
  it, so decorated links keep the fast path.


- **`<address>`, `<search>`, `<hgroup>` and `<center>` no longer merge into their neighbours.**
  All four are block-level, but they reached a pass-through handler that emits no separator,
  so `<address>foo</address><address>bar</address>` produced `foobar` -- nothing in the output
  recorded that these were ever distinct blocks. `<colgroup>`, `<col>`, `<base>`, `<html>` and
  `<body>` share the same internal classification and are deliberately unchanged: the first
  three are table-internal or void metadata, and the last two wrap every document.

- **Two block containers in one table cell are separated by one space, not three.** The fast
  scanner emitted a hard line break between them regardless of `br_in_tables`, and the
  table-cell finalizer turned it into a three-space run, where the converter emits a single
  space under the default options.

- **Five Tier-1/Tier-2 divergences closed.** `<blockquote>` omitted its trailing blank line
  in the fast scanner, invisible to every existing test because they all placed the
  blockquote last; fixing it exposed a `<pre>` fence stripping one trailing newline where the
  converter strips all of them. A text node's leading space survived at the start of a bare
  `<span>`/`<u>`, producing a double space across the Google Docs and WordPress fixtures.
  `<address>`, `<search>`, `<hgroup>` and `<center>` emitted a block separator the converter
  does not. Images with a lazy-load `src` and flattened nested-table pipes now agree as well.

- **An HTML comment no longer forces a redundant reparse of the whole document.** Custom
  elements are detected by looking for a hyphen in a tag name, but the scan treated the text
  inside `<!--...-->` as a tag name -- `!--c--` contains a hyphen -- so any document with any
  comment was reparsed through the repair path for nothing. On a 200-element document
  producing byte-identical output, a single comment cost 1.8x. Comments are near-universal in
  real pages, so most documents were paying it.

- **A table nested inside another table's cell no longer destroys the inner cells.** The
  nested table's own row and separator syntax was flattened into the outer cell unescaped, so
  its bare `|` characters were read as additional cell boundaries for the *outer* row. GFM
  truncates a row to the header's column count, so the inner cells were dropped outright on
  reparse. The flattened content is now pipe-escaped outside code spans.

- **A run of `&nbsp;` between two inline elements survives.** A whitespace-only text node
  between inline siblings was collapsed to a single plain space unconditionally. `str::trim`
  is Unicode-aware and counts `U+00A0` as whitespace, so the run was destroyed on the first
  conversion, not merely on a round trip. The same collapse also ran without checking whether
  the output already ended in a space, stacking a real inter-element space against the
  synthetic one left behind by `<style>` removal into a literal double space.

- **Content written directly inside `<table>`, outside any row or cell, is no longer dropped.**
  HTML5's "in table" insertion mode foster-parents such content: it moves to just before the
  table and survives. The parser this crate uses performs no such fixup and the table builder
  recognised only `caption`/`colgroup`/`col`/`thead`/`tbody`/`tfoot`/`tr` there, so raw text
  was silently discarded and stray elements went through a no-op handler --
  `<table>abc</table>` converted to nothing at all.

  It appeared to work whenever a comment happened to sit nearby, but that was coincidence: a
  separate defect treats `!--c--` as a tag name, sees a hyphen, and reroutes any
  comment-bearing document through the html5ever repair path, which does implement foster
  parenting. The text survived by accident. That path is now entered deliberately for this
  shape. Comments themselves are excluded, since HTML5 keeps them as children of the table
  and nothing is lost.

- **Lazy-loaded images resolve to their real address instead of converting to nothing.**
  Lazy-loading libraries leave `src` empty or holding a 1x1 `data:` placeholder and put the
  actual URL in `data-src`, `data-lazy-src`, `data-original`, `data-srcset` or `srcset`, so
  those images came out as `![alt]()` or a base64 blob -- effectively invisible on a large
  share of modern pages. The fallback applies only when `src` is already empty or a `data:`
  URI, so a plain `<img src="...">` is byte-identical to before. A populated non-`data:` `src`
  is trusted even when it looks like a placeholder, since some pages carry the real photo
  there while `srcset` holds only the lazy-load stand-in.

- **A heading inside `<summary>` or `<figcaption>` no longer splices its `#` prefix into
  unrelated text.** Tier-1 records a heading's content offset against whichever buffer is
  active when the tag opens, and `<summary>`/`<figcaption>` accumulate into their own buffer.
  Only table cells were special-cased, so a heading in a `<summary>` took a small
  buffer-relative offset and used it to index the whole document output instead -- inserting
  heading prefixes into the middle of already-emitted text. Rustdoc's
  `<details><summary><h3 class="code-header">` shape turned `Sample` into `Samp#### #### le`.
  This corrupted adjacent content, not just the heading.

- **An unclosed `<table>` no longer discards its rows.** Every element still open at end of
  input is closed implicitly, but Tier-1's implicit-close path did nothing for `<table>`,
  dropping the entire accumulated table -- fully-formed rows included -- where the explicit
  `</table>` path rendered it correctly. Text sitting directly inside `<table>` outside any
  row or cell is now routed to Tier-2 rather than silently discarded.

- **An empty `title=""` is treated as absent instead of rendered as `(url "")`.** The empty
  annotation carries no information and no Markdown serializer round-trips it, so converting
  the re-rendered output produced different Markdown than the first pass. Applies to links,
  images and graphics alike. A whitespace-only title is still a title; only a genuinely empty
  attribute changed.

- **HTML5 bogus comments render as nothing instead of leaking their text.** `<?php echo 1; ?>`
  emitted `?php echo 1; ?>`, `<!bogus>` emitted a stray `>`, and `</3>` emitted `</3>`. All
  three are comment tokens under the tokenizer's tag-open, markup-declaration-open and
  end-tag-open states, so they render as nothing -- as real `<!-- -->` comments already did.
  Identical constructs were rendering differently based only on which tokenizer state they
  happened to reach.

  Most visibly this cleans up Microsoft Word HTML, whose downlevel-*revealed* conditional
  comments (`<![if !vml]> … <![endif]>`, not wrapped in `<!--`) were surfacing as literal noise
  around every image and footnote.

  Real comments, CDATA, and doctypes are stepped over rather than scanned into. That matters
  for downlevel-*hidden* conditional comments: `<!--[if gte mso 9]> … <![endif]-->` is a real
  comment whose terminator is the `-->` at the end of `<![endif]-->`, so treating that
  `<![endif]` as bogus would delete the comment's own terminator and swallow the rest of the
  document. `<![CDATA[` is left untouched entirely, since it is only a bogus comment outside
  foreign content and this pass has no element context to distinguish `<svg>` interiors.

- **`strip_hidden_elements` no longer treats a non-tag `<` as a tag, fixing both a
  content bug and a quadratic-time denial-of-service vector.** The hidden-element pre-pass
  accepted any `<` whose next byte was not `/` or `!`, then scanned forward for the next `>`
  anywhere in the document and treated whatever it spanned as a tag. HTML5 only begins a tag
  name when `<` is followed by an ASCII letter, so this was wrong in three ways:

  - **Visible text was silently deleted.** `<1div hidden>x</1div>` is entirely text, but the
    pre-pass matched it as a hidden element and removed the whole span including the `x`.
    Same for `<_div …>`, `<-div …>`, `< div …>` and any non-ASCII initial.
  - **Genuinely hidden content leaked.** In `<<div hidden>x</div>` the span began at the stray
    first `<`, so what got removed was `<<div hidden>` and the `x` inside the real hidden
    `<div>` survived into the output.
  - **Conversion was O(n²) in unterminated `<`.** With no `>` ahead, every candidate `<`
    re-scanned to end of input. Converting 40,000 bare `<` took 1.05 s and 80,000 took 4.2 s;
    around a megabyte of such input would have taken hours. This is reachable from untrusted
    pages -- the repo's own fixtures already carry a `fallthrough_bare_lt` group -- so it was
    a practical DoS. The same input now converts in roughly linear time; the guarded case
    dropped from 16.4 s to 0.26 s.

  Output is byte-identical across the whole fixture corpus; the only behaviour changes are
  the two defect classes above, each now pinned by test.

- **A `<br>` inside a code span or fenced code block no longer injects a `newline_style`
  marker into the code.** A code context reproduces its content literally, so the marker was
  not syntax there -- it was a character in the user's code. `newline_style="backslash"` put a
  literal `\` inside the span (`<pre><code>A<br>B</code></pre>` emitted `A\` on its own line),
  and the two-space style injected trailing spaces. `CommonMark` gives a line ending inside a
  code span no hard-break meaning at all, so the two styles now agree byte for byte there. This
  is the rule the table-cell branch already applied for the same reason. A `<br>` *between* two
  code spans is an ordinary hard break and is unaffected.

- **A `<br>` at the end of any block no longer leaves a stray `\` under
  `newline_style="backslash"` ([#464](https://github.com/xberg-io/html-to-markdown/issues/464)).**
  3.11.6 claimed this was fixed, but the strip ran only inside the paragraph handler, so it
  covered `<p>...<br></p>` and nothing else. Every other way an inline run can end still leaked
  the marker: before a following block of any kind (`A<br><p>B`, the reporter's follow-up case,
  plus headings, lists, blockquotes, rules, tables and `<div>`), at the end of the document
  (`A<br>`), at the end of a `<div>`, list item, blockquote, `<section>`, `<details>`, `<figure>`
  or `<dl>`, and through nested containers.

  `CommonMark` gives a hard line break no meaning at the end of a *block*, not merely at the end
  of a `<p>`. The check now runs wherever a block boundary is actually reached. Handlers that
  walk their children into a fresh buffer were each an independent instance of the same defect,
  because `str::trim` cannot repair a trailing `"\\\n"` -- it removes the newline and leaves the
  backslash stranded.

  Worst case fixed: in a `<figcaption>` the caption is wrapped in emphasis *after* the marker is
  appended, so the leftover backslash landed between the text and the closing `*` and escaped it
  -- `<figure><figcaption>A<br></figcaption></figure>` produced `*A\*`, malformed Markdown rather
  than a merely visible artifact.

  A `<br>` that precedes real inline content still emits its break, a leading top-level `<br>`
  still opens a line ([#112](https://github.com/xberg-io/html-to-markdown/issues/112)), and
  `newline_style="spaces"` output is unchanged -- its leftover marker is invisible trailing
  whitespace.

- **Four cases where the Tier-1 fast scanner produced different Markdown than the Tier-2
  converter for the same input.** The library picks a tier automatically, so a divergence means
  the same document could convert two ways depending on which path it qualified for. Tier-2 is
  authoritative and was correct in all four.

  A custom element (any unknown tag containing `-`) was treated as block-level, so one appearing
  inline in flowing text split the surrounding paragraph, list item or blockquote and could leave
  an orphaned bullet or emphasis marker behind. Unknown elements are inline by default in HTML --
  there is no block-level custom element absent a CSS rule this converter does not apply -- so
  they now pass through inline, as the DOM walker always did.

  Named character references outside Tier-1's hot subset (`&notin;`, `&there4;`, `&sup1;`,
  `&para;`, `&trade;`) passed through as literal text instead of decoding. Tier-1 now falls back
  to the same full WHATWG named-reference table Tier-2 decodes against, rather than a hand-picked
  subset.

  `<strong><strong>x</strong></strong>` emitted `****x****`, which `CommonMark` does not parse as
  strong emphasis at all; redundant same-type nesting now collapses to one marker pair. A
  trailing whitespace run before a closing inline marker -- most often a decoded `&nbsp;` --
  stayed inside the markers instead of moving outside them.

  A leading whitespace run inside `<strong>`/`<em>` was deleted rather than moved outside the
  markers, so `<p>a<em>&nbsp;x</em></p>` lost the space entirely: `a*x*` where the DOM converter
  gives `a *x*`. The trim is still applied to `<a>` labels, which really are trimmed, and to
  `<code>`, which is verbatim.

- **The published crate no longer ships two tests that cannot compile.** Both read the
  `CommonMark` spec fixture from a path outside the crate root, which `cargo package` does not
  carry, so `cargo test` on the packaged crate failed to build. They are excluded from the
  package and still run from a repository checkout.

- **Leading whitespace at the start of a document is stripped, matching every later block.**
  `<p>&nbsp;a</p>` kept its space while `<p>x</p><p>&nbsp;a</p>` dropped it, so the same markup
  converted differently depending only on whether anything preceded it. The converter had no way
  to tell "start of the real output" from "start of a handler's private scratch buffer" -- every
  inline wrapper builds its content into a fresh buffer before splicing it in -- so a naive
  emptiness check silently changed every wrapper whose first child begins with whitespace.

- **Block children of a list item stay inside the item.** Fenced code blocks and block quotes
  were emitted without the continuation indentation that keeps them in the item, so they escaped
  the list entirely on reparse. The indent sums each open ancestor's actual marker width rather
  than assuming two spaces per level, since ordered markers vary in width (`1.` against `10.`).

- **List looseness is detected as the whole-list property it is.** One blank line anywhere
  between items makes a compliant reparse wrap *every* item in `<p>`, but detection only looked
  for a literal `<p>` and missed `div`, `blockquote`, `pre`, `table`, `hr` and `dl` children and
  loose nested sublists. Alongside it: a nested list that is an item's sole leading child no
  longer double-counts the parent marker's width and pushes its content past the four-space
  indented-code threshold; a block quote following an item's bare leading text is no longer
  forced onto a blank line, which had made that text round-trip as its own paragraph; a bare
  `<!-- -->` between two adjacent same-type lists is preserved, being the one signal that stops
  them merging; and the bare-marker check now honours the configured bullets instead of
  hardcoding `*`, `-` and `.`, having missed `+` from the default cycle.

- **A nested list keeps its line break when the text before it ends in whitespace.**
  `<li><strong>b </strong><ul><li>sub</li></ul></li>` collapsed to `- **b** * sub`, flattening
  the sublist onto the parent line so it stopped being a list at all and its content was lost.
  The "are we already at a bare marker" test was a two-byte suffix check, and a closing `**b**`
  plus its trailing space ends in the same two bytes as a real `*` bullet plus its space. Both
  tiers were wrong in exactly the same
  way, so the differential oracle could not see it; the check now requires the whole line to
  decompose into bare markers.

- **Non-ASCII bytes in a link destination are percent-encoded.** A raw byte at or above 0x80 is
  not valid URI syntax under RFC 3986, and compliant renderers encode it when writing an `href`.
  Only non-ASCII bytes are touched, so reserved ASCII punctuation in query strings is untouched.
  Same-document `#fragment` destinations are exempt: they must byte-match an element id this
  converter does not generate, and encoding one side of that pair would break the anchor.

- **Link-shaped text in image alt text and link labels is escaped.** Alt text is parsed as full
  inline content on reparse, so `<img alt="[foo](uri2)">` had its alt silently become a real
  nested link and lost the destination. Both ends of the pair are escaped, since escaping only
  the closing bracket leaves the outer `[` to be captured by a later `]` and the image then fails
  to form at all. A genuine nested `![alt](src)` inside link text is left intact.

- **A link or image with no visible text no longer turns its own href into emphasis.** Such an
  element falls back to the href as its label, and that fallback bypassed the text escaper, so a
  raw `*` or `_` in the URL round-tripped into real emphasis. Two destination-escaping gaps
  closed with it: a literal backslash before the paren escaping in an unbalanced-parens
  destination was not itself escaped, and a raw line ending inside an angle-bracket-wrapped
  destination is not valid `CommonMark` at all and is now folded to a space.

- **The Tier-1 fast scanner matches the Tier-2 converter on every fix above, and on a further
  run of divergences found by the differential oracle.** Because the library selects a tier by
  input shape, each divergence was a case where one document could convert two ways. Three
  suppressions were also removed from the oracle's allow-list, so it now generates those shapes
  freely instead of avoiding them; the four that remain are each documented in place.

- **A hard line break inside a link's visible text survives a round trip.** A `<br>` in a link
  label was collapsed to a space, so converting, rendering back to HTML, and converting again
  lost the break -- yet a hard break inside link text is perfectly legal `CommonMark`. Ordinary
  soft newlines from wrapped source text still collapse to a space, and a break at the very
  start or end of a label is still dropped, having no line to break to.

- **Four more places mistook inline content for a bare list marker.** The check was a two-byte
  suffix test, and a closing `**bold**` plus its space ends in the same two bytes as a real `*`
  bullet plus its space. A block quote after inline text in a list item lost its continuation
  indent and fell out of the item entirely on reparse, since `CommonMark` matches containers per
  line; a fenced code block was glued onto the preceding inline line, where the fence is not a
  valid opener; a `<div>` ran straight onto the previous text with no separator at all; and
  Tier-1's paragraph handler ran the check without first confirming an open list item, so
  top-level text merely ending in a hyphen and a space lost the blank line before the next
  paragraph. Several of these also omitted `+`, the third bullet in the default cycle, so they
  misbehaved at every third nesting level.

- **An unclosed `<p>` or `<div>` in a list item no longer swallows the next item.**
  `<ul><li><div></li><li>foo</li></ul>` converted to `- - foo`, turning two sibling items into
  one item holding a nested list, and `<ul><li><p>x</li><li>foo</li></ul>` converted to
  `- x- foo`, running both onto one line so the second stopped being a list item at all.
  Closing the tag explicitly was already correct, so this hit precisely the shape the HTML5
  parsing algorithm resolves with an implied end tag -- ordinary markup, not an edge case. The
  primary parser nests the following item under the unclosed element; the misparse detector
  that already re-parses such trees with the spec-compliant fallback only recognized a block
  nested under an *inline* ancestor, so a `div` or `p` never triggered it. Genuinely nested
  lists are unaffected.

- **Leading whitespace on soft-wrapped continuation lines no longer shrinks on every pass.**
  A run of spaces after a newline in block text collapsed to one space, but `CommonMark`
  strips a line's leading whitespace entirely when assembling a paragraph, so a renderer
  dropped the space that was kept and the next conversion saw one fewer -- the text never
  stabilized. Such runs now collapse to nothing, which is what a round trip already forces.
  Whitespace at a text node's own edge is untouched, since that is what keeps adjacent words
  apart.

- **A literal backslash in a link destination is no longer swallowed.** Two shapes lost data.
  A backslash before ASCII punctuation was consumed as a `CommonMark` escape of that character
  on reparse, so `href="\*"` came back as `href="*"`. A backslash at the end of a destination
  with no title after it merged with the closing parenthesis into an escaped `\)`, so the
  destination never terminated -- `href="x\"` reparsed as the literal text `[t](x)`, losing
  the href and the link structure and leaving raw brackets in the rendered output. Both are now
  escaped; a trailing backslash followed by a title's space, which is harmless, still is not.

- **The Tier-1 fast scanner no longer emits broken output for block children of a list item.**
  A `<div>` inside an `<li>` came out with no continuation indent, so its content fell out of
  the list on reparse; `<blockquote>`, `<table>`, `<dl>`, a `<p>` continuing existing text and a
  `<pre>` as an item's first content were each wrong in their own way. Under the shipped
  defaults Tier-1 is never reached for these, because metadata extraction and highlight styling
  both force the DOM converter first -- but with those disabled, all six shapes diverged.
  Rendering them correctly needs Tier-1 to defer its separator decisions the way the DOM
  converter does, which also entangles the loose-list heuristic, so the scanner now declines
  these shapes and falls back instead. No corpus coverage is lost: the new bail never fires
  across either parity corpus.

- **A lone significant character after a `<br>` is no longer dropped.** `<p>a<br>&nbsp;</p>`
  kept its non-breaking space, but the same markup with a newline after the `<br>` lost it --
  so whether content survived depended on nothing but whether the source HTML happened to be
  pretty-printed. A rendered `<br>` is always followed by a literal newline, which flipped the
  handler into a branch that discarded the run wholesale. Separately, a `<br>` inside a heading
  emitted the two-space hard-break marker, which has no meaning on a single line: a renderer
  collapses it, so the next conversion saw different bytes. It now emits one space.

- **`br_in_tables: false` is honoured for lists inside a table cell.** Both converters emitted
  a literal `<br>` between sibling `<li>` elements in a cell regardless of the option, so the
  setting silently did nothing for the shape it most often applies to -- a list in a cell, as
  MediaWiki sidebars produce. This also caused a round-trip instability: once a renderer
  flattens the list, that same `<br>` took the ordinary option path on the second pass and
  collapsed to a space, so the two passes disagreed.

- **A non-breaking space between two `<br>` tags is no longer dropped.** The paragraph
  pre-filter discarded a text node outright when trimming found it empty and both neighbours
  were empty inline elements -- and trimming is Unicode-aware, so a lone U+00A0 counted as
  empty even though it is visible content. It surfaced only on a second conversion, because the
  source spells it `&nbsp;` (six ASCII bytes) while a renderer re-serializes it as the literal
  character. Only genuine ASCII whitespace is dropped there now.

## [3.11.6] - 2026-08-28

Re-release of 3.11.5, which never reached any registry: the publish run failed and crates.io
still tops out at 3.11.4. Also carries the release-gate fixes below.

### Fixed

- **`<br>` runs emit one hard break each, and none trailing a block.** A run of consecutive `<br>`
  elements collapsed inconsistently, and a `<br>` immediately before a block boundary added a
  stray break to the output.

- **`packages/r/src/Makevars` is tracked instead of gitignored.** The file is alef-generated and
  alef-owned, but `packages/r/.gitignore` discarded it, so `alef verify --exit-code` -- the
  "Verify binding freshness" step of CI Rust -- failed on every run. `src/Makevars.win` stays
  ignored.

- **The Dart e2e before hook installs the pinned `flutter_rust_bridge_codegen`.** It hard-coded
  2.12.0 while the project pinned flutter_rust_bridge 2.13.0, so CI Dart aborted on alef's
  version-disagreement check. `[crates.dart] frb_version` now declares the pin explicitly next to
  the hook that has to match it.

### Removed

- Dropped a stray Python `enum_module` from the node and java e2e call overrides; neither emitter
  reads it.

## [3.11.5] - 2026-08-25

### Changed

- Regenerated all language bindings on alef 0.68.0.

### Changed

- **`alef` pinned to `0.67.5` (from `0.67.2`) and the tree regenerated.** Every one of the 609
  regenerated files carries a real content change; none is a bare provenance restamp. The
  substantive ones: the tagged `VisitResult` wire shape is now decoded consistently across
  languages -- Swift gains hand-written `Codable` conformance keyed on `{type, output}`, and the
  PyO3 bridge reads the `output` member instead of treating the whole envelope as the payload;
  Java's `VisitorBridge` derives its `NodeContext` field offsets from the declared `MemoryLayout`
  rather than from hand-maintained byte constants, and a throwing visitor callback now returns
  `VISIT_RESULT_CONTINUE` instead of `VISIT_RESULT_ERROR`, so one failing callback no longer
  aborts the whole conversion; Kotlin/Android e2e imports lose a doubled
  `io.xberg.android.io.xberg.android` package prefix; the Zig e2e suite asserts exact output
  instead of trimming it first; and the TypeScript snippets optional-chain nullable metadata
  (`result.metadata?.links`).

- **Snippet gap checking is configured, so `--strict` runs it instead of erroring.**
  `[workspace.docs.snippets]` gained `docs_dirs` and `required_languages`, mirroring the flags
  `task docs:snippets:gaps` already passed. Neither was set, so the gap pass -- unreferenced
  snippets and missing language variants -- was skipped entirely, and alef 0.67.5 correctly
  refuses to let a strict run pass on a check that compared nothing. `alef.toml` is hashed as a
  generation input as a whole rather than per-surface, so adding two snippet-only keys restamped
  the provenance marker of all 5044 content-verified files without changing a single line of
  generated content.

### Fixed

- **`RustBridgeC.h` is whole again after a regeneration silently truncated it by 193 lines.**
  The header is a concatenation of `SwiftBridgeCore.h` and `html-to-markdown-rs-swift.h`, both
  produced by `cargo build -p html-to-markdown-rs-swift`. When those build artifacts are absent
  -- as they are right after `alef all --clean` wipes them -- alef emits the crate half alone and
  reports success, so the file lost `RustStr`, `__private__FfiSlice`, every `__private__Option*`
  struct and `__swift_bridge__null_pointer`. Regenerating against real build artifacts restores
  content byte-identical to the pre-upgrade file. Nothing about this was an alef 0.67.5 behavior
  change; an earlier entry in this section wrongly credited it as one.

- **The eleven hand-written TypeScript and WASM snippets now declare which session validates
  them.** Two sessions (`node` and `wasm`) both validate `typescript` snippets, and alef 0.67.5
  stopped guessing between them; the six snippets under `docs-site/src/snippets/typescript/` now
  carry `target: node` and the five under `docs-site/src/snippets/wasm/` carry `target: wasm`.
  These eleven had never actually been compiled -- they were reported as errors, not failures --
  which is how the WASM visitor example kept four implicitly-`any` callback parameters that
  `noImplicitAny` rejects the moment the snippet is really typechecked. It is now a `typescript`
  fence with its parameters annotated, matching the `node` example beside it and every generated
  WASM snippet. Snippet results move from 4550 passed / 11 errored to 4561 passed / 0 errored.

- **A skipped `publish-crates` no longer reads as a passing gate, and a release that published
  nothing can no longer report success.** Eight places in `.github/workflows/publish.yaml` gated
  downstream build, publish and release-promotion jobs on
  `needs.publish-crates.result != 'failure'`. That expression is TRUE when the dependency was
  *skipped*, and `publish-crates` skips for two opposite reasons: the version is already on
  crates.io (a re-run or a resumed release, where downstream must proceed) or an upstream gate
  such as version validation or crate packaging failed (where downstream must not). `result`
  alone cannot separate them, so every one of those conditions was gating on nothing --
  tree-sitter-language-pack v1.15.5 promoted a GitHub release to `Latest` with 40+ failed jobs and
  every registry publish skipped, and still reported success.

  A new always-running `crates-gate` job resolves the ambiguity once, into an explicit
  `outcome` (`published` / `already-present` / `not-required` / `dry-run` / `blocked`) and an
  `ok` flag that every consumer now tests instead of `result`. Because the job always runs, its
  outputs always exist; because it never fails, depending on it cannot skip a consumer. The
  legitimate already-published path stays exactly as permissive as before, and only the
  gate-failed path is newly blocked.

  `release-report` also stopped being able to see this class of failure: it inspected only jobs
  whose conclusion was `failure`, so a run where every publish job merely *skipped* passed. It now
  judges each target on its own result and treats `skipped` as a failure unless the target is not
  enabled for this release or its registry probe already found this exact version published.

- **The R vendoring script deleted the core crate's `[lints]` without inlining anything, so the
  vendored copy compiled under a different lint configuration than its sources.**
  `scripts/ci/r/vendor-core-crate.py` copies `crates/html-to-markdown/` out of the workspace and
  rewrites `workspace = true` inheritance into explicit values — except for `[lints]`, which it
  stripped outright. The `[workspace.lints.rust]` `unexpected_cfgs` check-cfg allowlist went with
  it; that allowlist is what declares the crate's own `#[cfg(...)]` gates as expected cfg names, so
  every gate in the vendored copy became an `unexpected_cfgs` diagnostic — silent in a default
  build, a hard error under `RUSTFLAGS="-D warnings"`. `missing_docs`, `unused_must_use`,
  `unsafe_code = "forbid"` and the whole `[workspace.lints.clippy]` policy were dropped the same
  way. The script now materializes the entire `[workspace.lints]` sub-tree into the vendored
  manifest verbatim, matching the fix alef shipped in 0.67.x for `alef publish prepare`. The
  extracted text is re-parsed and compared against the authoritative parse of the root manifest, so
  a mis-extraction fails loudly rather than silently emitting different lints. Nothing is specific
  to a lint or cfg name: adding a lint or a check-cfg entry to the workspace needs no change here.
  The script is not replaced by alef's implementation because it runs from `packages/r/configure` at
  R configure time — including on end-user machines installing the source tarball from GitHub, where
  only Python 3 and a Rust toolchain exist.

- **The coding-agent plugin version gate never ran on the commits that cause drift.**
  `plugin/` sat at 3.11.3 while core shipped 3.11.4, so all 13 version declarations under
  `plugin/` — OpenCode, Hermes, Claude Code, Cursor, Codex, Gemini, Kimi, Factory — were a
  release stale. The checker for this already existed (`scripts/sync_plugin_version.py --check`,
  run by `CI Plugin`), but `ci-plugin.yaml`'s `paths:` filter did not list `Cargo.toml`. A
  release commit bumps `Cargo.toml` and nothing under `plugin/`, so the workflow was never
  triggered on exactly the commits that cause drift, and the gate reported nothing rather than
  failing. `Cargo.toml` and `.task/tools/version-sync.yml` are now in the filter, so any core
  bump re-runs the gate. `sync_plugin_version.py` also grew `--expect <version>`, which asserts
  that core *and* the plugin both equal the version being released; `publish.yaml`'s
  `validate-versions` job runs it against the tag, so a drifted plugin now fails the release
  instead of publishing a bundle that lags the version it claims to be. The 13 stale declarations
  are re-synced to 3.11.4.

- `task test` now runs. `tools/benchmark-harness/examples/profile_visitor.rs` imports
  `NoOpVisitor`, which only exists under the harness's `visitor` feature, but the task builds the
  workspace with `--no-default-features` — and `cargo test` builds examples. The example is now
  declared with `required-features = ["visitor"]`, matching the existing `testkit` examples.
  Before: `task test` exited 201 with `E0432: unresolved import
  html_to_markdown_bench::bench::NoOpVisitor`. After: exit 0, 1422 passed / 0 failed / 126 suites.
- `task test:ci`'s `--exclude benchmark-harness` excluded nothing — that is the crate's *directory*
  name; the package is `html-to-markdown-bench`. cargo reports an unmatched exclude as a warning,
  not an error, so the dead flag survived silently and the crate was covered on every run. Fixed
  in all four `rust:test:ci` invocations and sabotage-verified: with the bench crate deliberately
  made unparseable, `--exclude benchmark-harness` still fails the build while
  `--exclude html-to-markdown-bench` never compiles it.
- Dropped the trailing `fix-cjs-named-exports.mjs` step from `task alef:generate`. `@napi-rs/cli`
  3.x already emits an explicit `module.exports.<name> = nativeBinding.<name>` for every runtime
  export, so named ESM imports resolve without it (#450). The script had also gone stale in a way
  that corrupts output: napi now declares enums as `export declare const enum X`, which its regex
  reads as a const *named* `enum`, appending a bogus `module.exports.enum = nativeBinding.enum`
  and re-exporting only 7 of the 20 names it used to cover.
- `alef` pinned to `0.67.2` (from `0.66.0`) and the tree regenerated. Four real codegen changes
  landed, plus a hash restamp of the fixture-driven snippet corpus:
  - `DocumentNode.children` and `DocumentNode.annotations` are no longer `@Nullable` in the Java
    record. Both mirror a plain `Vec<T>` on the Rust side carrying
    `#[serde(default, skip_serializing_if = ...)]`; the Rust value is never `null`, so the old
    signature described a state the binding cannot produce. A compact constructor now normalizes
    a `null` deserialization to `List.of()`. All 283 `e2e/java` tests still pass — no test in this
    repo asserted the old null-valued contract.
  - `flutter_rust_bridge` moved to `2.13.0` across `packages/dart/{pubspec.yaml,rust/Cargo.toml}`,
    `Cargo.lock`, and the `frb_generated.*` outputs. `e2e/dart/pubspec.lock` also picked up the
    `ffi: ^2.2.0` entry its `pubspec.yaml` already declared. All 283 `e2e/dart` tests pass.
  - `crates/html-to-markdown-ffi/build.rs` was regenerated with the header-export path split into
    named helpers, which drops the `clippy::collapsible_if` hit.
    `cargo clippy -p html-to-markdown-ffi --all-targets -- -D warnings` is clean and all 283
    `e2e/c` tests pass.
  - `packages/swift/Sources/RustBridgeC/RustBridgeC.h` is now clang-formatted. Token-normalized,
    the file is identical to the previous content apart from `#include` ordering.
  - The generated snippets under `docs-site/src/snippets/generated/` now print the fields their
    fixture actually asserts (`result.content`, `result.metadata.document.author`, ...) instead of
    dumping the whole result object. This exposes an alef codegen bug: the emitter dereferences
    optional intermediate fields without guarding them, so the 24 `metadata_*` TypeScript
    snippets now fail `tsc` with `TS18048: 'result.metadata' is possibly 'undefined'` and the
    three Open Graph / Twitter-card Swift snippets fail on
    `expression implicitly coerced from 'Optional<RustString>' to 'Any'`. These are generated
    files, so they cannot be corrected here — the fix belongs in alef's snippet emitter.

  `alef verify --exit-code` and `poly lint .` both pass on the resulting tree.

- `alef` pinned to `0.65.0` (from `0.64.0`) and the tree regenerated with the released crates.io
  build (`cargo install alef --version 0.65.0 --locked`), not a local `alef` checkout — unlike the
  0.64.0 repin, this is not byte-identical: `packages/elixir/native/html_to_markdown_nif/src/lib.rs`,
  `packages/elixir/lib/html_to_markdown/native.ex`, `packages/r/src/rust/src/lib.rs`,
  `packages/r/R/extendr-wrappers.R`, `packages/r/NAMESPACE`,
  `packages/java/src/main/java/io/xberg/htmltomarkdown/{ConversionOptions,DocumentNode}.java`, and
  the Dart/Swift structure e2e tests all picked up real codegen differences. `packages/r/NAMESPACE`
  and `packages/r/R/extendr-wrappers.R` gained `.alef-ownership.toml` records (formats with no
  comment syntax, so they cannot carry an inline `alef:hash:` marker) alongside the existing
  `packages/r/DESCRIPTION` entry. `alef verify --exit-code` and `poly lint .` both pass on the
  resulting tree.

- `[workspace.docs.snippets]` `dirs` only listed `docs-site/src/snippets/generated`, so the 96
  hand-written snippets under `docs-site/src/snippets/<lang>/{getting-started,error-handling,
  table-extraction,metadata,visitor}` — the ones `usage.mdx`, `errors.mdx`, `tables.mdx` and
  `visitor.mdx` actually import — were never discovered by `alef snippets check`. Nothing
  validated they still compile and nothing regenerates them, so a reader could be shown code that
  no longer builds. `alef e2e snippets-migrate docs-site/src/snippets` confirms none of the 96
  have a fixture-generated equivalent: the generated corpus is organized by fixture topic
  (`conversion`, `edge-cases`, `metadata`, `options`, `real-world`, `result`, `smoke`,
  `structure`, `visitor`) and emits a test-style snippet (frontmatter + `main()` + `print(result)`)
  aimed at compile validation, not the field-access idiom (`result.content` / `result.tables` /
  `result.metadata`) the narrative docs teach — so they're kept rather than replaced. Added each
  language's snippet root (not a glob) to `dirs`; this reaches only the hand-written topic dirs
  and does not overlap `generated/<lang>/...`, which lives one level deeper under its own subtree.

- `alef adopt "**/*" --converged-only --write` claimed ownership of 13 files that already matched
  (or, for `packages/r/DESCRIPTION`, could only ever match via `.alef-ownership.toml` since its
  format carries no comment syntax) current `alef generate` output byte-for-byte apart from the
  provenance marker: `.clang-format`, `packages/r/DESCRIPTION`,
  `crates/html-to-markdown-ffi/cmake/html-to-markdown-ffi-config.cmake`,
  `packages/dart/rust/Cargo.toml`, `packages/{elixir,java,kotlin-android,zig}/README.md`,
  `packages/ruby/html_to_markdown.gemspec`, and
  `test_apps/{node,wasm}/pnpm-workspace.yaml`, `test_apps/php/phpunit.xml`,
  `test_apps/python/pyproject.toml`. 11 of those had drifted (stale `3.11.0` version pins in
  READMEs/gemspec/build metadata, indentation, quoting) — `adopt` stamps the marker and leaves the
  drifted content as-is by design; a later `alef generate` will resync it now that ownership is
  recorded. `packages/kotlin-android/gradle/wrapper/gradle-wrapper.jar` could not be adopted even
  with `--write`: alef refuses to stamp or ownership-record any binary file, so it stays unmarked
  — a tool limitation, not a decision made here.

- Left every create-once seed alone: `config.m4`, `crates/html-to-markdown-php/src/HtmlVisitor.php`,
  `e2e/java/pom.xml`, and `e2e/kotlin_android/build.gradle.kts` (the four paths `alef generate`
  refused to write) plus ~20 siblings alef's own `adopt` scan flagged the same way (Cargo/npm/gem
  scaffold manifests under `crates/html-to-markdown-node/{package.json,index.js,npm/*/package.json}`,
  `packages/ruby/spec/html_to_markdown_spec.rb`, `packages/java/checkstyle-suppressions.xml`,
  `packages/csharp/HtmlToMarkdown{,.Runtime}/*.csproj`, `Package.swift` /
  `packages/swift/Package.swift`, `packages/dart/{pubspec.yaml,CHANGELOG.md,example/*.dart}`,
  `packages/zig/{build.zig,build.zig.zon,test/*.zig}`). Each is a create-once seed: alef writes it
  only when absent, and every one on disk is substantial hand-maintained content (24-431 lines),
  not a placeholder. `alef adopt --clobber-create-once-seeds` would stamp them, but that flag
  explicitly consents to alef replacing the file with a placeholder seed on the next `generate` —
  the wrong trade for real content, so none of these were adopted.

- Ran `alef generate` then `alef all` to actually resync the 13 adopted files and everything else
  the `[workspace.docs.snippets]` `dirs` config edit (above) left stale: every alef-owned file's
  provenance marker fingerprints the full config, so that one-line config change invalidated the
  whole generated tree's markers even where rendered content hadn't changed. 5 of the 13 adopted
  files had real drift: `packages/dart/rust/Cargo.toml` (hardcoded `version = "3.11.4"` instead of
  `version.workspace = true`, and missing the `[lints.clippy]` block its swift sibling carries —
  meaning `dbg_macro`/`print_stdout`/`print_stderr` were silently unenforced for that crate),
  `packages/ruby/html_to_markdown.gemspec` (stale file-exclusion glob),
  `crates/html-to-markdown-ffi/cmake/html-to-markdown-ffi-config.cmake` (indentation), and
  `test_apps/python/pyproject.toml` and `.clang-format` (missing hash marker / stale dependency
  pin). The other 8 already matched generated output byte-for-byte. Checked every other workspace
  member's `Cargo.toml` for the same missing-lints gap: `packages/dart/rust/Cargo.toml` was the
  only one lacking `[lints]` entirely; all ten others already carry their own `[lints.clippy]`
  block or `workspace = true`. `alef verify --exit-code` now reports zero stale bindings; the
  remaining failure is ~99 pre-existing scaffold files (`LICENSE`, `.gitignore`, `package.json`,
  gradle wrapper files, …) that were never run through `alef adopt` and so carry no provenance
  marker — a separate, long-standing gap outside this fix's scope, left for a future adoption pass.

- Compile-validated the 96 hand-written snippets `alef generate` (above) had just made visible to
  `alef snippets check`. 6 C snippets under `docs-site/src/snippets/c/` still declared
  `HTMConversionResult *`/`HTMConversionOptions *`/etc. as raw pointer types and compared them
  against `NULL`; the FFI moved to opaque `HTMAlefHandle` (`uint64_t`) handles a while ago and
  these were never updated, so 5 failed to compile and a 6th (`visitor/basic_visitor.md`, comparing
  `HTMHtmVisitor *` against a type that no longer exists) was misclassified as an unresolved
  dependency by alef's error-pattern heuristic rather than a real failure. Rewrote all 6 to the
  handle-based idiom (`HTMAlefHandle result = htm_convert(html, 0);`). One zig snippet
  (`docs-site/src/snippets/zig/visitor/basic_visitor.md`) used `orelse` on `c.htm_convert`'s return
  value as if it were optional; the binding returns a plain `u64` (0 on failure), so `orelse` no
  longer type-checks — replaced with an explicit `if (result == 0) return error.ConvertFailed;`.
  Both languages are now 578/578 with zero failures and zero unavailable.

  The hand-written `docs-site/src/snippets/typescript/*` snippets import the published package
  name (`@xberg-io/html-to-markdown`), same as a real consumer would, but the node and wasm
  snippet sessions' `cwd` IS that package (or its sibling), so nothing had ever installed it into
  its own `node_modules` the way `npm install @xberg-io/html-to-markdown` would for a consumer.
  Added a `before` hook to both sessions in `alef.toml` that self-links the package
  (`mkdir -p node_modules/@xberg-io && ln -sfn ../.. node_modules/@xberg-io/html-to-markdown` for
  node; the wasm session links to the sibling node crate since the snippets never import the wasm
  package by name). Verified by hand that this actually fixes module resolution — `tsc --noEmit`
  against alef's own synthesized session `tsconfig.json` passes clean once the symlink exists — but
  `alef snippets check` itself still reports these same 11 snippets (6 hand-written × node and wasm
  sessions, minus one overlap) as `unresolved_dependency` even after the fix and with `--cache off`.
  This reproduces identically across repeated fresh runs, so it isn't cache staleness; it looks like
  a defect in how alef disambiguates a single fence-tag ("typescript") claimed by two sessions when
  validating frontmatter-less (hand-written, not fixture-generated) snippets, since the equivalent
  283-file fixture-generated batch (which does carry `target: node` frontmatter) passes cleanly
  through both sessions. Not fixed from this repo; documented here for upstream triage.

  5 hand-written `docs-site/src/snippets/kotlin_android/*` snippets (`import io.xberg.android.*`)
  also remain `unresolved_dependency` even after `./gradlew assembleDebug -Palef.skipHostJni=true`
  succeeds and the target classes are confirmably present in
  `packages/kotlin-android/build/intermediates/aar_main_jar/debug/syncDebugLibJars/classes.jar`.
  `./gradlew publishToMavenLocal -Palef.skipHostJni=true` gets further (compiles and bundles the
  release AAR) but fails at `signMavenPublication` for lack of a configured signatory — this
  environment has no GPG signing key, and none was fabricated. Left unresolved rather than faked;
  the 6th kotlin_android snippet (`visitor/basic_visitor.md`) is comment-only (the visitor pattern
  isn't supported on this binding yet) and reports `PASS` trivially.

  `docs-site/src/snippets/feedback.md` was deliberately left out of `[workspace.docs.snippets]`
  `dirs`: it's a shared MDX `<Content>` partial (a static "found a bug?" callout imported by ~10
  doc pages), not a per-language code sample — it contains zero code fences, so there is nothing
  for `alef snippets check` to discover or validate there.

  Final `alef snippets check --strict --cache off` result across all 16 configured languages:
  4571 total, 4555 passed, 0 downgraded, 0 failed, 0 skipped, 0 errors, 16 unavailable (kotlin 5,
  typescript 11, both detailed above). `--strict`'s bar is zero *incomplete*
  (`Skip | Unavailable | Downgraded`), not zero failures, so this run still fails that bar —
  reported plainly rather than as a pass.

- The 5 kotlin_android `unresolved_dependency` snippets above were misdiagnosed: the real cause
  was not the classpath or the unsigned AAR, it was plain Kotlin syntax. `error-handling/basic_usage.md`,
  `table-extraction/basic_extraction.md`, and `metadata/basic_extraction.md` opened with (or ended
  in) bare top-level statements — `try { }`, a top-level `for` loop, trailing `println(...)` calls
  — which a plain `.kt` file (unlike a `.kts` script) does not permit outside a function body;
  `kotlinc` rejected them with `error: syntax error: Expecting a top level declaration`, which
  alef's error-pattern heuristic then classified as `unresolved_dependency` rather than a real
  failure, the same misclassification already seen and corrected for the C/zig visitor snippets
  above. `getting-started/basic_usage.md` and `getting-started/with_options.md` were already
  declarations-only (`val`/`import`) and passed without change. Wrapped the executable body of the
  three broken snippets in `fun main() { ... }`, matching the convention the Java/C# snippets
  already use for the same requirement. `alef all --clean` (0.65.0, with `packages/kotlin-android`
  built via `cargo build --release -p html-to-markdown-rs-jni` +
  `./gradlew assembleDebug -Palef.skipHostJni=true`) now reports kotlin at 0 failed / 0 unavailable
  across all 3 (all 6 including the two already-passing and the trivial visitor snippet). The
  typescript disambiguation defect (11 unavailable, both `node` and `wasm` sessions claiming the
  same `typescript` fence tag) is unchanged at 0.65.0 and remains upstream-triaged, not fixed here.

## [3.11.4] - 2026-08-22

### Fixed

- `alef.toml`'s `[workspace.docs.snippets]` `timeout_secs` was 120s, which also bounds every
  session's `before` hook. On a cold checkout the kotlin_android (`assembleDebug`), wasm
  (`pnpm run build:all`) and swift (`swift build`) sessions could not finish their `before` build
  within that window, so their snippets were reclassified `unresolved_dependency` and reported
  `Unavailable` — failing CI's `alef snippets check --strict` (`Failed: 0` but a large
  `Unavailable` count) even though no snippet itself was broken. Raised to 900s, matching the
  precedent in tree-sitter-language-pack's `alef.toml`.
- The benchmark regression gate no longer fails at random on the CPU the runner happened to draw.
  `htmbench compare` compared the whole `Provenance` struct for equality, so a capture from an
  Intel Xeon host could not be evaluated against a baseline calibrated on an AMD EPYC host: it
  aborted with `benchmark provenance mismatch` before any timing was compared, even though the
  timings themselves passed every threshold. GitHub's `ubuntu-24.04` label spans both vendors and
  the calibration campaign is `workflow_dispatch`-only, so which host each side drew was a coin
  flip. `cpu_model` and `cpu_count` are now excluded from the provenance contract; every other
  field — rustc, Cargo, profile, build flags, core features, measurement mode, tier and visitor
  selection, iteration/warmup settings, runner image and class — still hard-fails on drift. A
  differing CPU is reported instead as a host mismatch, and the new off-by-default
  `--allow-host-mismatch` flag (passed only by the CI regression job) downgrades timing violations
  to advisory on hardware the baseline was never calibrated on. Thresholds are unchanged and the
  baseline was not re-cut: a regression measured on the calibrated CPU still fails the run.

- The FFI Symbols CI gate is green again. It was failing on two independent findings.

  The `htm_register_html_visitor` allowlist entry is retired. It claimed "visitor registration
  never landed in the FFI crate", which is no longer true: registration landed as the vtable API
  `htm_visitor_create` / `htm_options_set_visitor` / `htm_visitor_free`, and the alef 0.62.6 regen
  (`4765a4139`) replaced the last phantom caller — a Java Panama `SymbolLookup.find(...)` in
  `NativeLib.java` — with lookups of those real symbols. The checker reported the entry as
  *orphaned* rather than *resolved* only because it matches on exact symbol names and the
  replacement uses different ones. The C# visitor surface reaches the same exports through
  `NativeMethods.cs` and `HtmlToMarkdownConverter.Convert`, and all 49 tests in
  `e2e/csharp/tests/VisitorTests.cs` pass against the built native library, so the public
  `IHtmlVisitor` / `HtmlVisitorBridge` API is wired end to end.

  `htm_node_type_from_json` is allowlisted, replacing it. The alef 0.62.9 regen (`dc27e0560`)
  re-added a C# P/Invoke for it, but `NodeType` is a fieldless `Copy` enum: it crosses the C ABI
  as `int32_t`, so the FFI exports `htm_node_type_from_i32` / `htm_node_type_from_str` and no
  handle-returning `from_json`. Nothing calls `NodeTypeFromJson`, so the declaration is latent
  rather than a live crash. This is a re-regression — the same symbol was allowlisted and retired
  once before in `555a29d20`. Fixed upstream in alef's C# backend, which now skips scalar-crossing
  named types when emitting handle-lifecycle P/Invokes; the entry comes out with the first regen
  on an alef release carrying that fix.

- A failed per-language artifact build no longer aborts the release. In v3.11.3 four
  `Build PHP extension (... windows-x86_64)` legs failed and nothing else did, yet the GitHub
  release was never promoted out of draft, never finalized and never announced, and the Homebrew
  bottles, `cargo-binstall` verification, Packagist trigger and asset audit never ran — 3.11.3
  reached every language registry but its release page stayed a draft.

  Two GitHub Actions behaviours caused it, and the publish workflow now accounts for both. A
  failure propagates down the entire `needs` chain, so a job's implicit `success()` gate is false
  even when its own direct needs succeeded; and the propagated failure also poisons the *value* of
  `needs.<job>.result`, so `upload-php-pie-release` reported `failure` to downstream jobs despite
  concluding `success` — which is why `always()` plus `!contains(needs.*.result, 'failure')` still
  skipped `verify-assets`, `release-finalize` and `announce-discord`.

  The release spine (`promote-release` → `release-finalize` → `announce-discord`, plus
  `verify-binstall` and the Homebrew bottle jobs) now gates only on jobs whose whole ancestry is
  release-blocking: version validation, crates.io, and the CLI and C FFI assets. Per-language
  artifact jobs stay in `needs` for ordering — the draft still flips only after every asset upload
  has settled — but their results no longer appear in any gate. `promote-release` publishes a
  `promoted` output for downstream jobs to gate on, because job outputs carry no failure poison.

### Added

- A `Report release outcome` job closes the workflow. It reads the run's real job conclusions from
  the API and writes a job summary stating whether the tag shipped and naming every artifact that
  is missing, then fails the run if anything failed. A release that ships without one language's
  binary is now red and explicitly labelled incomplete rather than silently green.

### Changed

- `Verify release assets` is a reporting gate rather than a release gate. It runs on every real
  tag and no longer guards itself with `!contains(needs.*.result, 'failure')`, which had skipped
  the asset audit in exactly the situation the audit exists for. Nothing in the release spine
  depends on its result.

## [3.11.3] - 2026-08-21

### Fixed

- Leading normalized whitespace at the start of a paragraph is no longer emitted before an image,
  preventing CommonMark from interpreting the image as an indented code block
  ([#460](https://github.com/xberg-io/html-to-markdown/issues/460)).

- The Swift crate compiles again. Codegen emitted a bare `EnumName::Variant` path for
  data-carrying variants when converting a Swift string back into a Rust enum, which rustc rejects
  with `E0533: expected value, found struct variant`; the regenerated crate routes those through
  per-enum string conversion helpers instead.

- The e2e freshness gate no longer fails on formatting it cannot produce. It installs Elixir, so
  alef's `mix format` pass runs there as it does locally — without it the job emitted unformatted
  `.exs` and reported the difference as fixture staleness.

- The crates.io skip guard reads a real output again. The publish workflow asked
  `check-registry` for an `extra-packages` key (`cli_exists`), but a composite action propagates
  only the outputs it declares, so that key arrived as an empty string and the derived `all_exist`
  could never be true. The guard has therefore never once skipped an already-published version. It
  now reads the action's declared `all-exist` output.

### Changed

- `packages/java/pom.xml` is alef-owned and regenerated. It had drifted out of alef's ownership
  since roughly alef 0.60.x for want of a provenance marker, so this release lands the accumulated
  template changes at once: the file is reindented to four spaces, gains `<excludes>` /
  `<sourceFileIncludes>` blocks, and moves checkstyle to 13.11.0 and jackson-databind /
  jackson-datatype-jdk8 to 2.22.2. Those versions are alef's pinned template defaults, not
  html-to-markdown-specific choices.

- All Rust dependencies taken to their latest versions (`cargo upgrade --incompatible` followed by
  `cargo update`): `rmcp` / `rmcp-macros` 3.1.2 to 3.1.4 plus twelve transitive bumps. Fourteen
  packages changed, none downgraded.

- alef pinned to 0.62.8.

- The benchmark harness now records nine timing samples with median and median absolute deviation,
  captures runner and toolchain provenance, and supports reviewed fixture-specific noise floors
  without changing the existing percentage regression thresholds
  ([#461](https://github.com/xberg-io/html-to-markdown/issues/461),
  [#462](https://github.com/xberg-io/html-to-markdown/issues/462)).

- Java's `VisitResult` is now a plain sealed interface. Its Jackson `@JsonSerialize` /
  `@JsonDeserialize` annotations and the nested serializer and deserializer classes are gone; the
  visitor bridge marshals the type directly and never routed it through Jackson.

### Removed

- The unused Java visitor classes `IHtmlVisitor`, `HtmlVisitorAdapter` and `HtmlVisitorBridge`.
  They shipped alongside the live `HtmlVisitor`, `VisitorBridge` and `VisitorHandle` surface but
  nothing in the binding referenced them, so an implementation of `IHtmlVisitor` was never
  invoked. Implement `HtmlVisitor` instead.

## [3.11.2] - 2026-08-19

### Fixed

- The R package now builds against the vendored core crate instead of failing to resolve it.
  `configure` stripped the `path = ...` clause from `src/rust/Cargo.toml` and wrote a cargo source
  replacement to `src/rust/.cargo/config.toml`, but cargo reads `.cargo/config.toml` only from the
  working directory and its ancestors, and `Makevars.in` runs cargo from `packages/r/src` — so
  `src/rust/.cargo` is a descendant and that config was never read on any build. With the path
  clause gone the dependency fell through to crates.io, which did not have the workspace version.
  The path is now rewritten to point at the vendored copy, which needs no cargo config, no
  `.cargo-checksum.json`, and no complete vendor tree.

- Newlines inside a table cell no longer leak into the rendered row when `br_in_tables` is enabled
  ([#456](https://github.com/xberg-io/html-to-markdown/issues/456),
  [#457](https://github.com/xberg-io/html-to-markdown/issues/457)). The whole-cell newline backstop
  was gated on `!br_in_tables`, so enabling `br_in_tables` disabled the last-resort guarantee that a
  raw newline never reaches a cell. Two consequences: a `<pre>` inside a cell now renders inline,
  dropping its fence, indentation and language info string, matching how headings and list items
  already render in cells; and under `WhitespaceMode::Strict` a newline inside a cell now folds to a
  single space. Every other whitespace byte is still preserved exactly under `Strict`, and text
  outside table cells is unaffected.
- The Tier-1 fast path now honours `br_in_tables` inside table cells. It previously emitted a
  sentinel that always expanded to three spaces, ignoring the option, so a document routed through
  Tier-1 could render `<br>` in a cell differently from the same document routed through Tier-2.
  Tier-1 also folds a text node's trailing newline the way Tier-2 does, fixing cells that were
  double-spaced against Tier-2 when the source HTML was pretty-printed across several lines.
- A literal backslash in HTML prose is no longer silently lost when the Markdown is read back
  ([#458](https://github.com/xberg-io/html-to-markdown/issues/458)). CommonMark consumes a backslash
  that precedes ASCII punctuation, so emitting `3\*4` from `<p>3\*4</p>` produced Markdown that
  reparsed as `3*4` — the source character was gone. Such a backslash is now doubled, and so is one
  that sits immediately before a line ending (where CommonMark would read it as a hard line break)
  or at the end of a text run (where whatever the emitter appends next would become its escape
  target). This changes output under default options: any document whose prose contains a backslash
  in one of those three positions gains a second backslash there. A backslash before anything else
  is already literal and is still emitted bare, so Windows paths such as `C:\Users\Alice` are
  unchanged. Code spans, code blocks and link titles are also unaffected — the first two are
  verbatim by design and the third already escaped backslashes through its own rule. The escape is
  deliberately not gated behind `escape_misc` or `escape_ascii`, because it preserves a character
  the source actually contained rather than neutralising Markdown syntax the way those flags do.

## [3.11.1] - 2026-08-15

> Prepared but never published: no `v3.11.1` tag was pushed and no 3.11.1 reached crates.io,
> so 3.11.0 is followed directly by 3.11.2. The entries below ship as part of 3.11.2.

### Upgrade note — the C ABI changed

This release changes the C header, so it is not drop-in for C, Go (cgo) or any other consumer that
links the native library directly. Rebuild against the new header rather than reusing an existing
build. Managed bindings (Python, Node, Ruby, PHP, Java, C#, Elixir, R, WASM) are unaffected.

- `HTMHtmHtmlVisitorBridge` and `HTMHtmHtmlVisitorVTable` are gone. Borrowed types that cannot enter
  the process-global handle registry are no longer exported, so the bridge entry points
  `htm_htm_html_visitor_bridge_new` and `htm_htm_html_visitor_bridge_free` are removed with them.
  Use the `HTMHtmVisitorCallbacks` vtable instead.
- `htm_free_bytes` is removed. The exported function count goes from 307 to 304.
- `HTMHtmVisitorCallbacks` gains a `void (*free_string)(char*)` member, which changes the struct
  layout. Any code that constructs this struct must be recompiled, and callers that hand out heap
  strings from a callback should set it so the library releases them with the matching allocator.

The header previously in the tree described a surface the sources had already stopped providing;
it is now regenerated from them.

### Fixed

- The generated FFI, Swift and Node sources did not compile, so the native library, the Swift
  package and the Node addon could not be built from a checkout. A comment-reflow pass had merged
  `unsafe {` into the preceding `// SAFETY:` comment at 40 sites in the FFI bridge and collapsed a
  doc block in the Swift bridge over the `SwiftHtmlVisitorWrapper` declaration, so neither file
  parsed; the Node addon applied a napi `getter` to three free functions, which napi allows only
  inside an `impl` block. This also blocked every commit to the repository, because the pre-commit
  hook compiles the staged snapshot and the FFI build script runs cbindgen over `lib.rs`.
- The FFI null-safety test called `htm_htm_html_visitor_bridge_free`, a symbol the crate had
  deliberately stopped exporting, so the test target did not build.

## [3.11.0] - 2026-08-13

### Upgrade note

Rendered Markdown output changes in this release. The CSS-hidden/`hidden`-attribute fix below alone
moved 56 of the 116 benchmark oracle snapshots when they were reblessed, and roughly a dozen other
correctness fixes in this range shift output for the documents they affect — 64 of the 116
snapshots changed in total. Consumers who pin byte-exact golden files against this library's
output should expect to regenerate them when upgrading past this release.

A further 16 snapshots moved when hidden-element detection was corrected to read attribute
structure instead of scanning raw tag text. Every one of those 16 **restores content that was
previously deleted**: a Wikipedia link whose `title` attribute contained the word "hidden" was
being stripped in full. If you saw links or sections disappear from converted pages, this is why.

### Added

- Documentation snippets are generated from the complete E2E fixture corpus with Alef 0.60.2 and checked for
  fixture-by-language coverage parity in local tasks and CI; strict validation is scoped to the generated root so
  the 85 maintained examples remain independently audited.
- The MCP server validates its inputs, bounds its HTTP surface and is instrumented. It previously
  had no instrumentation at all despite being a public RPC surface: a failed request returned an
  error to the client and left no trace. There are now debug spans on `convert_html` and
  `extract_metadata` (re-entered inside `spawn_blocking`, which does not inherit the caller's
  span), warnings on rejected enum values and conversion errors, and an error event on a panicking
  blocking task. Behaviorally, unknown enum values from a client are rejected instead of silently
  falling back to a default, a `max_depth` that does not fit `usize` warns instead of silently
  becoming `usize::MAX`, five serialization fallbacks that returned `null`/`[]` on failure now
  report the failure, and the HTTP transport gained a body-size limit, a concurrency cap, a request
  timeout, and Origin/Host checks.
- `VisitResult` now serializes with an adjacently tagged, snake_case wire schema
  (`{"type": …, "output": …}`), so every binding encodes against one documented shape instead of
  inventing its own. The field names are public API and a conformance test pins them.

### Changed

- Table rendering: each cell's Markdown is now produced once and reused between the column-width
  pre-pass and the render pass, instead of being rendered twice, for the common case (no nested
  table, no visitor installed). Rendered Markdown is unchanged — reblessing the benchmark oracle
  snapshots before and after produced byte-identical output. This also fixed the metadata
  collector recording every element inside a `<td>` twice, once per pass, on the default
  extraction path. See `crates/html-to-markdown/tests/table_cell_metadata_duplication_test.rs`.
- Link handling: fewer anchor traversals and allocations per `<a>` element. Output-neutral.
- Removed the internal `text_content` LRU cache and with it the `lru` dependency. Because every
  caller needs an owned `String`, a cache hit still cloned, so the cache cost an allocation per
  miss and saved none; after the table change above, the dominant table path cannot hit it at all.
  Measured 3.7% faster across the 29-fixture benchmark. Output-neutral.
- DOM context building: the per-document context maps (parent, children, sibling-index, and
  related lookup tables) are now sized once from the arena's node count up front instead of
  growing incrementally as new node ids are seen. Output-neutral.
- `alef` is pinned to `0.60.2` for binding generation, in `alef.toml` and in the CI lint workflow.
  The two had drifted apart (`0.60.2` locally, `0.60.1` in CI), so CI generated with a different
  generator than every developer.
- `poly`'s whole-project lint phase is now disabled through `[workspace.poly] lint-workspace` in
  `alef.toml` rather than a hand-edit to the generated `poly.toml`. The shared CI validate job
  installs only Rust/Python/Java, so the remaining whole-workspace linters cannot run there; they
  run via the pre-commit hooks and each language's own CI job. The previous hand-edit did not
  survive `alef all --clean`.
- Dependency upgrades, including `base64` and `tower-http` to their next major versions.
- `html-to-markdown-cli` gained optional `mimalloc` and `jemalloc` global-allocator features
  (`--features mimalloc` / `--features jemalloc`). Neither is enabled by default; if both are
  enabled, `jemalloc` takes precedence, so `--all-features` builds continue to work.
- `mcp-http` is no longer a default feature of `html-to-markdown-cli`. It was on by default, so a
  CLI installed via brew, cargo, npm or pip shipped the ability to start an unauthenticated HTTP
  service. Build with `--features mcp-http` if you need it; stdio `mcp` stays on by default, since
  it opens no listening socket.

### Fixed

- `<br>` inside a table cell no longer depends on `newline_style` or on the source HTML's own
  whitespace ([#453](https://github.com/xberg-io/html-to-markdown/issues/453)). Two separate
  defects: with `br_in_tables: false` the `<br>` fell through to the paragraph hard-break path and
  emitted `newline_style` bytes into the cell, leaking a literal `\` under `Backslash` and a stray
  extra space under `Spaces`; and a newline in the source before the `<br>` survived normalization,
  so with `br_in_tables: true` it reached the output as a real newline and split the row's pipe
  syntax across physical lines, corrupting the table. A cell cannot contain a hard line break, so
  `<br>` now collapses to a single space, or renders as a literal `<br>` when `br_in_tables` is
  enabled, in every combination of the two options and regardless of source formatting.
- `<div>` and `<p>` continuations inside a table cell follow the same rule as `<br>`
  ([#454](https://github.com/xberg-io/html-to-markdown/issues/454)). The two disagreed with each
  other: a `<div>` continuation honoured `br_in_tables` but emitted `newline_style` bytes, which are
  not valid inside a cell, while a `<p>` continuation always emitted `<br>` and ignored
  `br_in_tables` entirely. Both now emit a literal `<br>` when `br_in_tables` is enabled and collapse
  to a single space otherwise, sharing one code path with the `<br>` handler.
- A `<code>` span inside a table cell no longer corrupts the row
  ([#455](https://github.com/xberg-io/html-to-markdown/issues/455)). Verbatim content — `<code>`,
  and `<kbd>`/`<samp>`, which share the same path — skipped cell whitespace handling entirely, so a
  newline inside the span reached the output and split the row's pipe syntax across physical lines
  whenever `br_in_tables` was enabled. Line breaks in that content are now folded to a single space,
  independent of `whitespace_mode`, because a raw newline in a cell is a structural impossibility in
  GFM rather than a formatting preference. All other whitespace, including repeated spaces, is still
  preserved byte-for-byte, and code spans outside a table cell are unaffected.
- Document-structure and inline-image collectors also record table-cell content exactly once.
  Images, code blocks and nested tables inside a `<td>` were recorded up to three times with
  `include_document_structure: true`, and inline images twice with `extract_images: true`. Note the
  companion change: a table that renders to nothing (a blank table, or one a visitor skips) now
  contributes nothing to `document` or the inline-image set, matching what it emits; `result.tables`
  still reports its grid.
- Metadata extracted from inside a table cell (links, images) is now recorded exactly once in
  every configuration. Previously a link in a `<td>` was recorded twice with
  `link_style: Reference`, and three times with `include_document_structure: true`, because the
  column-width pre-pass, the render pass, and the document-structure grid walk each re-walked the
  cell through a shared collector. The internal passes no longer record; exactly one walk does.
  Affects `metadata.links` / `metadata.images` counts for documents with tables — de-duplication
  on the consumer side is no longer needed.
- Rows with more cells than the header no longer lose them. The separator row was sized from the
  first row alone, so any later row with more cells had the surplus silently dropped by compliant
  renderers; it is now sized from the table-wide maximum and every row is padded to match.
  Separately, `scan_table_node` counted nested tables and rows transitively across nested `<table>`
  boundaries, so the layout-table heuristic misfired for every level of a nested chain except the
  innermost, turning real table structure into a mix of bullet lists and stray pipe text.
- Consecutive `<li>` elements inside a table cell no longer fuse into one word. A cell strips list
  markers, and nothing separated the items, so `<li>a</li><li>b</li>` rendered as the fabricated
  word `ab`. Both tiers now emit the same `<br>` boundary already used between sibling `<p>`/`<div>`
  in a cell.
- Content that a browser never renders is no longer emitted. `<template>` and `<noscript>` fell
  through to the unknown-element handler, which recurses into children and renders their text — a
  template's contents are inert per spec and must never appear in the output — and
  `strip_hidden_elements` honoured only the `hidden` attribute, so `style="display:none"` and
  `visibility:hidden` leaked in full. This is the single largest source of output drift in this
  release: it alone moved 56 of the 116 benchmark oracle snapshots. `aria-hidden` is deliberately
  untouched, because that content is visually rendered and hidden only from assistive technology.
- Hidden-element detection no longer misreads the tag it is inspecting, in three separate ways.
  `declaration_hides_element` split each declaration on the first `:`, so a CSS comment ahead of the
  property name left `/* note */ display` as the property and never matched —
  `<div style="/* note */ display:none">SECRET</div>` emitted its content; comments are now
  stripped before the split. `tag_has_hidden_attribute` scanned the raw tag text for the word
  `hidden` and matched inside quoted values, so a perfectly visible
  `<div title="… hidden from search engines">` was deleted whole; it now walks `name=value` pairs
  and matches attribute *names*. And `tag_has_hidden_style` used `.any()` over declarations,
  ignoring the CSS cascade, so `display:none; display:block` was stripped even though the last
  declaration wins; it now resolves per property. The second of these is why 16 oracle snapshots
  gained content back — see the Upgrade note.
- Content nested inside a CSS-hidden element (`style="display:none"` / `visibility:hidden`, or the
  `hidden` attribute) no longer leaks into the output when the hidden element contains a nested
  element of the *same* tag name (e.g. a hidden `<div>` containing another `<div>`). The stripper
  previously matched the closing tag of the *inner* element rather than the outer one, so text
  after the inner close tag — and, for deeper nesting, several more layers of "hidden" text — was
  emitted as if it were visible. This affects any document with same-tag-name nesting inside a
  hidden element, including a common Wikipedia infobox pattern; see
  `crates/html-to-markdown/tests/hidden_element_nesting_test.rs` for the affected shapes. Output
  changes for documents that hit this pattern.
- Hidden content no longer escapes through a `</tag>` sequence that is not really markup. The
  depth-counting stripper still treated any `</tag>` byte sequence as a close tag, so four measured
  shapes leaked against the release binary on default options: a `<!-- </div> -->` end-of-block
  marker (an ordinary authoring idiom, requiring no crafting) leaked the trailing text, a quoted
  attribute value containing `</div>` leaked, a `</div>` inside a preserved `application/ld+json`
  raw-text body leaked, and an unbalanced `<div>` inside a comment inflated the depth counter far
  enough that the scan overshot and dropped the following visible sibling. The scan now skips
  comments, CDATA and raw-text bodies, and advances non-target tags through a quote-aware
  tag-end search.
- Alt text, titles and media URLs are escaped before they are spliced into Markdown. The hardening
  applied to `<a href>` was never propagated to images, graphics or embedded media, so inert input
  produced live output: an alt of `a](https://evil.example)` emitted a real image pointing at that
  URL; `<audio>`, `<video>` and `<iframe>` used `src` as both label and destination with no
  escaping at all; `<graphic>` had no paren handling whatsoever; and a bare quote in a title closed
  the Markdown title early, leaving the remainder to parse as document text. The `<a>` path's
  `append_url_destination` and `escape_markdown_title` are now shared by all of them, so
  balanced-paren checking (which had treated `)(` as balanced) guards image destinations too. Alt
  and title text containing brackets or quotes is now escaped, which changes output for benign
  documents as well.
- The HTML serializers are depth-bounded and quote-safe. `serialize_element`/`serialize_node` in
  the SVG and utility paths mutually recursed over arbitrary-depth subtrees with no depth parameter
  at all — a remote stack overflow, reproduced as a SIGABRT with 50k nested `<g>` or `<mrow>` — and
  are reachable from roughly twenty call sites via `preserve_tags` and the visitor's `PreserveHtml`
  result. They now truncate with a warning at the native stack-safe depth. Both also re-quoted
  every reconstructed attribute with double quotes without escaping an embedded one, so a
  single-quoted attribute holding a literal quote — valid, inert HTML — reconstructed into extra
  live attributes, turning an inert `title` into a real `onclick` handler. Values are now escaped
  regardless of the source delimiter.
- Deeply nested documents no longer exhaust the machine. Three separate defects made the depth
  guard ineffective: roughly two dozen call sites forwarded the recursion depth unchanged while
  descending into a child, every table-cell walk passed a literal `0` and so reset the budget at
  each cell boundary (a remote stack-overflow SIGSEGV from repeated `<table><tr><td>`), and two
  passes outside the depth-bounded walk were quadratic in nesting depth —
  `has_inline_block_misnest` ran once per block node and walked to the root each time, and
  `scan_table_node` re-walked the whole remaining nested-table chain from every table. A 220KB
  document of 20k nested `<div>`s went from 30.5s to 0.06s, and 50k-deep table, div, svg, ol and
  blockquote payloads — three of which previously ran past two minutes — now all finish in under a
  second. This was remote resource exhaustion: the guard bounded the walk but not these passes.
- A code block containing a run of three or more backticks — an embedded Markdown sample, say — no
  longer closes early and corrupts everything after it. The opening fence was hardcoded to three
  characters; its length is now `max(3, longest_run + 1)`. Inline code spans use a different rule
  on purpose: the smallest delimiter length not present in the content, because CommonMark closes a
  span at a backtick string of the *same* length, so `max_run + 1` would over-escape (CommonMark
  examples 330 and 331).
- An out-of-range `<ol start>` no longer panics the whole conversion. `start` was parsed as `usize`
  with an unchecked per-item increment, so `start="18446744073709551615"` with a single `<li>`
  overflowed and failed the entire document rather than just the list. The counter is now `i64`,
  so a negative start counts down as browsers do, out-of-range magnitudes clamp with a warning, and
  the increment pins at `i64::MAX` instead of wrapping. The saturation warning fires once per list
  rather than once per item, so a long list cannot flood the logs.
- Blockquotes preserve significant leading whitespace. The per-line loop trimmed every content
  line, so indented code blocks and nested-list continuations inside a `<blockquote>` lost their
  indentation entirely; only whitespace-only lines are blanked now. Nested blockquotes also pushed
  a hardcoded three newlines regardless of what preceded them, emitting stray blank `>` lines, and
  a paragraph following bare inline text inside a blockquote merged onto the same line because the
  separator was gated off inside blockquotes altogether. The gate is now depth-aware, which leaves
  the compact heading-then-paragraph style alone (CommonMark example 228 depends on it).
- A list nested inside an ordered list is now indented to its parent marker's content column
  instead of a uniform `list_depth * list_indent_width`. That uniform width happens to match `"- "`
  but not `"1. "` (3 columns) or `"10. "` (4), so a nested ordered list was indented 2 columns and
  CommonMark parsed the child as a sibling of its parent. The indent is now the cumulative width of
  the ancestors' own markers, with `list_indent_width` as the floor.
- A panicking visitor callback no longer poisons the handle for every later call. The panic
  unwound out of `convert()` and left the visitor's `Mutex` poisoned, so every subsequent
  conversion reusing that handle failed permanently. The pipeline now runs under `catch_unwind` and
  clears the poison flag, confining the failure to the call that caused it.
- Three inline-image options finally do something. `InlineImageConfig::new` seeded its own
  defaults and the conversion path never overwrote them, so `capture_svg`, `infer_dimensions` and
  `max_image_size` were inert despite each having both a builder setter and an update field — and
  two of those internal defaults were the inverse of the documented ones: `capture_svg` documented
  `false` but always behaved as `true`, and `infer_dimensions` documented `true` but always behaved
  as `false`. `max_image_size` was ignored outright and only ever coincided with the internal
  constant. Callers who set any of the three will see behavior change to what the documentation
  always promised.
- The Tier-1 byte-scanner path agrees with Tier-2 again. It carried independently duplicated
  copies of the code-fence and inline-delimiter bugs fixed above, had no depth ceiling, silently
  returned an empty `result.metadata`, emitted hidden elements that Tier-2 strips, and left link,
  image and SVG-title labels unescaped — the last being the injection vector Tier-2 has guarded for
  some time. It also evaluated only one of Tier-2's three layout-table conditions, so a
  `<table border="0">` with a `colspan`, or two nested tables, produced a GFM table (in the nested
  case, a malformed one) where Tier-2 produced a bullet list. Tier-1 now bails on both shapes
  rather than reimplementing them, and the router defers to Tier-2 for metadata rather than
  maintaining a second collector — a second implementation of that collector is precisely what
  produced the duplicated fence bugs. Default options never reach Tier-1, so this affects only
  callers who force `TierStrategy::Tier1` or disable metadata extraction.

- Python visitor callbacks now honor the documented `type`/`output` action dictionaries for
  custom link output and image `skip`/`continue` actions ([#452]).

- Dart: the native loader downloads and caches the library again on a cold cache. It only read
  the versioned cache and then threw a `StateError`, even though `nativeDownloadAndCacheLibrary()`
  was defined and exported for exactly that case — so a machine that had never run
  `dart run h2m:download_libs` could not self-heal. The loader also now searches for the
  `_dart`-suffixed cdylib that is actually built, opens every candidate by absolute path (a
  hardened runtime rejects a relative `dlopen`), walks up from `Platform.script` to find the
  package root as a last resort, and names the real environment variable in its error message
  instead of printing the identifier `$nativeLibDirEnv` literally. Fixed upstream in alef 0.55.6.

  Behavior change: an unresolvable native now throws a descriptive `StateError` naming the asset
  URL and the download command, where it previously returned `null` and let flutter_rust_bridge
  attempt its own relative-path `dlopen` — which would fail anyway, but later and less legibly.

- Java: `TierStrategy` serializes to the wire names the core actually accepts. Alef 0.55.7 changed
  the Java backend's no-`rename_all` fallback to emit variants verbatim, but it could not see
  `rename_all` when it sat behind a `cfg_attr` with an `any(...)` condition — exactly how
  `TierStrategy` declares it. The generated Java therefore sent `Auto` to a core that deserializes
  `auto`, and every Java conversion failed with `unknown variant`. Fixed upstream in alef 0.55.8,
  which parses the `cfg_attr` condition structurally.

- CI: the Node e2e job no longer runs the Rust test suite. It invoked `task rust:test`
  (`cargo test --release --no-default-features --workspace`), a full release-mode build that took
  3047s of the job's 60-minute budget on `windows-latest` and left `Install alef` to be cancelled
  mid-step — the Windows Node e2e job had never once completed. `CI Rust` already runs the suite on
  ubuntu, windows and macos via `task rust:test:ci` and compile-checks `--no-default-features`
  separately, and no other language's e2e job ran it. The job now passes in 925s.

- Node: the named ESM re-exports that work around napi's `module.exports = nativeBinding` tail
  (#450) are now committed in `crates/html-to-markdown-node/index.js`, not only appended at publish
  time. `cjs-module-lexer` cannot analyse that tail, so `import { convert } from ...` broke for
  anyone consuming the repo directly. Published packages were already correct — both
  `task alef:generate` and the publish workflow run the fixup script — but the committed file was
  not, which also meant a regenerate-and-diff freshness check could never pass.

- `--newline-style`, `--code-block-style` and `--bullets` all documented the wrong default in the
  CLI's `--help`, so users following the help text got output they did not expect. Tests now
  assert the real defaults, so help text and behavior cannot drift apart silently again.

- The CLI's outbound User-Agent is derived from the crate version. It was a literal pinned at
  `2.10` while the crate shipped `3.10.x`, so every fetch advertised a version four majors stale.

- The coding-agent plugin's launcher downloads the right asset and verifies it. It fetched
  `html-to-markdown-<triple>` while releases publish `cli-<triple>`, so every download 404'd and
  silently fell through to the slow path — the fast path had never once worked — and it then
  executed whatever it did fetch, with only TLS vouching for the bytes. It now resolves the
  checksum from `cli-SHA256SUMS`, retries while GitHub propagates release assets, and refuses to
  execute a binary it cannot verify.

- The Dart, Swift and Zig package READMEs are full documents again (276 / 271 / 260 lines). A
  partial generator run had replaced all three with fallback stubs of ~30 lines; Dart's stub was
  the pub.dev landing page rather than the package README.

- Internal: `strip_css_comments` uses `let ... else` instead of a single-arm `match`, which was a
  `clippy::single-match-else` error under `-D warnings` and failed the workspace clippy gate.

## [3.10.6] - 2026-08-05

### Fixed

- Node.js: the `linux-x64-musl` and `linux-arm64-musl` packages really are published now. 3.10.5
  added them, but both cross-compiles failed to link (`cannot find libgcc_s.so.1`) and, because the
  npm publish job requires every matrix leg, no Node package reached the registry at all. The build
  action exported `CC`/`CARGO_TARGET_*_LINKER` pointing at `musl-gcc`, and cargo-zigbuild only sets
  those when they are unset, so the zig cross-compile was silently replaced by a host-arch musl-gcc
  that cannot produce a musl cdylib. The musl legs now opt out of that export.

### Added

- CI: the musl Node cross-compiles are built on every core/node change instead of only at publish
  time, and the job fails if a platform package ends up without a native module.

## [3.10.5] - 2026-08-05

### Fixed

- Node.js: the `linux-x64-musl` and `linux-arm64-musl` packages are now built and published. They
  were advertised in the main package's `optionalDependencies` but never produced, so Alpine and
  other musl installs silently fell back to no native binding, and `pnpm install --frozen-lockfile`
  could not resolve them.
- Ruby: the gem no longer publishes its generated types into the global `Object` namespace, which
  collided with unrelated libraries (notably the `parser` gem's `Parser` constant). Generated types
  now stay namespaced under `HtmlToMarkdown` (tree-sitter-language-pack issue #173).
- Documentation: every code snippet in the READMEs, the docs site, and the coding-agent plugin
  references was executed against the real API and corrected. The Rust README's metadata and custom
  visitor examples did not compile, the Rust API reference showed `Result<_, Error>` instead of
  `Result<_, ConversionError>`, the docs site shipped 102 empty language tabs across five pages, and
  several bindings' snippets referenced helpers that no longer exist.

## [3.10.4] - 2026-08-04

### Fixed

- Node.js: named ESM imports such as `import { convert } from "@xberg-io/html-to-markdown"` now
  resolve. The napi-generated `index.js` ended with `module.exports = nativeBinding`, which Node's
  ESM↔CJS interop (`cjs-module-lexer`) cannot statically analyze, so named imports threw
  `SyntaxError: The requested module ... does not provide an export named 'convert'`. The build now
  appends explicit `module.exports.<name> = nativeBinding.<name>` re-exports for every public
  runtime export; `require()` is unaffected ([#450]).

[#450]: https://github.com/xberg-io/html-to-markdown/issues/450
[#452]: https://github.com/xberg-io/html-to-markdown/issues/452

## [3.10.3] - 2026-08-04

### Fixed

- The Swift package now builds under Xcode/XCBuild, not just `swift build`. The `RustBridgeC`
  target was header-only, so XCBuild failed to link (`RustBridgeC.o` was never emitted) for every
  iOS/macOS consumer of the published SwiftPM package. It now ships a translation unit with an
  anchor symbol so the object is always produced ([#449]).

### Changed

- The PHP binding sources now live in the `html-to-markdown-php` crate alongside the other
  in-crate bindings; the standalone `packages/php` layout has been removed. Composer consumers are
  unaffected.

[#449]: https://github.com/xberg-io/html-to-markdown/issues/449

## [3.10.2] - 2026-08-01

### Added

- `cargo binstall html-to-markdown-cli` support (#448) — prebuilt CLI binaries can now be
  installed directly from GitHub Releases without compiling from source. Adds
  `[package.metadata.binstall]` to the CLI crate plus a release-time `verify-binstall` CI
  job that installs via `cargo binstall` and smoke-tests the binary.

### Changed

- Updated dependencies.

## [3.10.1] - 2026-07-31

### Fixed

- The Android AAR (`io.xberg:html-to-markdown-android`) now bundles its JNI native libraries.
  Previously the published AAR contained no `.so` files, so every consumer crashed at runtime with
  `UnsatisfiedLinkError: library "libhtm_jni.so" not found` ([#446]). The publish workflow built
  the wrong crate (the C-FFI `html-to-markdown-ffi`) and uploaded it from a path the build action
  never wrote, staging nothing. It now builds the JNI crate (`html-to-markdown-rs-jni` →
  `libhtm_jni.so`) and stages it for every ABI, and a regenerated Gradle guard (alef 0.48.16) fails
  the build if a correctly-named `lib*_jni.so` is ever missing.

### Added

- The core library now emits structured `tracing` spans and events as a first-class observability
  surface: an `html_to_markdown::convert` span (`input_len`, `output_format`, `wrap`,
  `extract_metadata`, `extract_images`, `tier_strategy` fields) wraps every conversion, with
  `DEBUG` events at parse/walk/render stage boundaries and `WARN`/`ERROR` events on recovered or
  fatal failures. The library never installs a subscriber — attach one in your application to
  observe it. The CLI now initializes a `tracing-subscriber` (respecting `RUST_LOG`, with
  `--debug` raising the default level) and routes all diagnostics through `tracing` instead of raw
  `stdout`/`stderr` writes.

### Changed

- Android AAR now ships four ABIs (`arm64-v8a`, `x86_64`, `armeabi-v7a`, `x86`).
- Upgraded dependencies to latest, including `base64` 0.23 and `rmcp` 3.0.1.

[#446]: https://github.com/xberg-io/html-to-markdown/issues/446

## [3.10.0] - 2026-07-30

### Changed

- Upgraded the MCP server to `rmcp` 3.0 (MCP `2026-07-28` specification). The server now advertises
  protocol version `2026-07-28` and negotiates down for older clients, so existing integrations keep
  working. Minimum supported Rust version is now 1.88.

### Added

- Structured tool output (SEP-2106): `convert_html` (with `json:true`) and `extract_metadata` now
  return `structuredContent` alongside the text JSON, so clients can consume the result as data.
- Cache hints (SEP-2549) on the static prompt/resource catalogs (`prompts/list`, `resources/list`,
  `resources/read`): `ttlMs` of one hour with a `public` cache scope.

## [3.9.2] - 2026-07-27

### Fixed

- Java (Maven Central) and C# (NuGet) publishing, which failed in 3.9.1. Regenerated all language
  bindings on alef 0.48.4: the generated pom's Maven enforcer floor no longer exceeds the CI
  runner's Maven version, and the C# package now renders `runtime.json` (from the newly generated
  `runtime.json.template`) before `dotnet pack` via the `xberg-io/actions/render-runtime-json` step.

### Changed

- Upgrade dependencies to their latest incompatible versions.

## [3.9.1] - 2026-07-26

### Changed

- Refine the depth-limit warning (#434): the `DepthLimitExceeded` message now reports the effective
  limit value, and the warning is emitted exactly once when a deeply-nested DOM is truncated.
  Thanks @br411 (#428).
- Regenerate all language bindings on alef 0.48.2.
- Update dependencies to their latest compatible versions.

### Removed

- Remove unused Java PMD ruleset and stale linter configuration.

## [3.9.0] - 2026-07-19

### Added

- **Configurable traversal-depth ceiling** (#434): callers may now raise the recursion limit above the
  conservative native default (64) by setting an explicit `max_depth`, honored up to an internal
  backstop of 1024. Deeply-nested email HTML that previously lost content past depth 64 now converts.
  When the limit does truncate a subtree, a `DepthLimitExceeded` warning is surfaced instead of the
  content being dropped silently.

### Fixed

- **`<br>` in table cells with `br_in_tables`** (#429): a `<br>` inside a table cell now emits a literal
  `<br>` (valid single-line GFM) instead of a physical newline that broke the row.
- **Newline-only inline span separator** (#430): in normalized whitespace mode a `<span>` whose sole
  content is a newline now collapses to a single separating space instead of being dropped, so adjacent
  inline text no longer glues together.
- **Paragraph after a table inside a blockquote** (#431): the span newline-pop no longer crosses a
  table-row boundary, so a following paragraph is not glued onto the delimiter row.
- **Intentional hard break inside a span** (#432): the span newline-pop no longer eats a `<br>` hard
  break (`  \n` / `\\\n`), preserving the line break.
- **`keep_inline_images_in` inside layout-table cells** (#433): images inside a `td`/`th` listed in
  `keep_inline_images_in` now stay as markdown when a Tier-2 layout table converts cells as inline,
  instead of reducing to alt text.

### Changed

- Regenerate all bindings with alef 0.37.0.
- Update dependencies across language packages.

## [3.8.3] - 2026-07-09

### Fixed

- **Java visitor API** (#426): `ConversionOptions.builder().withVisitor(...)` now works. The visitor
  upcall `FunctionDescriptor`s were generated with a `JAVA_LONG` return layout while the `handleVisit*`
  bridge methods return `int`, so the Java Linker rejected every stub with `IllegalArgumentException:
  Wrong method handle type: (MemorySegment×5)int` — even a no-op visitor threw before any callback ran.
  Fixed upstream in the Alef generator (0.34.4); the descriptor return layout is now `JAVA_INT`.

### Changed

- Regenerate all bindings with alef 0.34.4.
- Formatting is now poly-only: removed the `alef:format`, `ruby:format`, and `csharp:format` tasks
  (which invoked `alef fmt`); `task format` / `poly fmt --fix .` is the single formatter.

## [3.8.0] - 2026-06-27

Stable release promoting 3.8.0-rc.2 (fully published). Version-only bump synced across all manifests.

## [3.8.0-rc.2] - 2026-06-27

### Changed

- Regenerate all bindings with alef 0.29.3.

## [3.8.0-rc.1] - 2026-06-26

### Changed

- **Rebrand Kreuzberg → Xberg across every published package identity.** The Node binding now ships as
  `@xberg-io/html-to-markdown` (the NAPI-RS crate itself — the separate `-node` package and the
  TypeScript wrapper under `packages/typescript/` are removed) with platform packages
  `@xberg-io/html-to-markdown-<platform>`; WASM and CLI move to `@xberg-io/html-to-markdown-wasm` and
  `@xberg-io/html-to-markdown-cli`. Java/Kotlin Maven coordinates and namespace move to `io.xberg`
  (`io.xberg:html-to-markdown[-android]`, JNI symbols `Java_io_xberg_android_…`), and the C# NuGet id to
  `XbergIo.HtmlToMarkdown`. The GitHub org (`github.com/xberg-io`), Homebrew tap
  (`xberg-io/homebrew-tap`), publisher GitHub App, sponsors links, and all docs/badges follow. The legal
  entity name **Kreuzberg, Inc.** is unchanged.

### Fixed

- **Swift publish now creates the `release/swift/<version>` branch carrying the substituted
  XCFramework checksum.** The alef-generated Swift e2e/test-app pins
  `.package(url: …, branch: "release/swift/<version>")`, but the publish workflow only force-moved
  the `v<version>` tag and never created that branch, so SwiftPM could not resolve the package. The
  checksummed commit is now also pushed to `refs/heads/release/swift/<version>`.
  (`.github/workflows/publish.yaml`)
- **test(test_apps/go): extract the module version with `$NF` instead of `$2`.** The smoke harness'
  `download_ffi.sh` read the version from `$2`, which only holds in the block `require (…)` go.mod form;
  the inline `require <path> <version>` form (emitted by the v3.7.2 regen) shifts the version to `$NF`,
  so the script built a malformed module-cache path and failed with "Binding directory not found". This
  is test-harness only — the published Go package is unaffected.

## [3.7.2] - 2026-06-23

### Fixed

- **chore(deps): bump `alef.toml.alef_version` to 0.26.6 and regenerate every binding.** Fixes the dart
  wrapper crate failing to build under `cargo build --no-default-features`: alef 0.26.x had regressed
  the 0.25.33 fix and re-emitted `#[cfg(feature = ...)]` on the generated `lib.rs` mirror struct /
  opaque-wrapper declarations, their `From` conversions, and `from_json` bridge fns, while
  `frb_generated.rs` references those types/functions unconditionally (`E0425: cannot find type
  VisitorHandle` / `create_html_metadata_from_json`). alef 0.26.6 keeps those declarations
  unconditional again. Verified: the regenerated dart crate compiles under `--no-default-features`,
  default, and `--all-features`. (alef 0.26.6)
- **ci(publish): the Homebrew Swift artifactbundle no longer forces `kreuzberg/openssl-vendored`.** The
  shared `build-swift-artifactbundle` action hardcoded that feature for its Linux (cargo-zigbuild)
  targets, but it only exists in the kreuzberg core swift crate — so html-to-markdown's swift bundle
  build failed (`the package '…' does not contain this feature`), which cascaded to skip
  `release-finalize` (and with it the `packages/go/vX.Y.Z` Go module tag). The action now takes a
  `linux-features` input (left empty here). (shared `xberg-io/actions/build-swift-artifactbundle`)

## [3.7.1] - 2026-06-23

### Fixed

- **ci(homebrew): drop the Intel-macOS `sonoma` bottle from the publish matrix.** The formula intentionally has no `x86_64-apple-darwin` url (Apple-Silicon only), but the bottle matrix still built a `sonoma` (Intel) bottle, which failed with `formula requires at least a URL`. Because `release-finalize` gates on `publish-homebrew-bottles` with `if: !contains(needs.*.result, 'failure')`, that one chronic failure silently **skipped release-finalize for three releases** (v3.6.20, v3.6.21, v3.7.0) — and with it the `packages/go/vX.Y.Z` Go module tag, leaving `go get …/packages/go/v3@vX.Y.Z` unresolvable. Removing the Intel bottle restores release-finalize (and the Go tag) for every future release. (`.github/workflows/publish.yaml`)
- **ci(e2e, C# on Windows): keep `cargo` on PATH in the test before-hook.** The "Run E2E tests (Windows)" step overrode `PATH` via `env:` with `${{ env.PATH }}`, which omits cargo's `$GITHUB_PATH` additions under Git Bash, so the `cargo build … html-to-markdown-ffi` before-hook failed with `cargo: command not found`. Prepend the target dirs to the live `$PATH` at runtime instead. (`.github/workflows/ci-e2e.yaml`)
- **ci(e2e/docs, Elixir): force the NIF to build from source.** Under `MIX_ENV=test` the `force_build: … or Mix.env() in [:dev]` clause does not apply, so `mix test` tried to download a precompiled NIF for the current (unreleased) version and failed with `the precompiled NIF file does not exist in the checksum file`. Set `RUSTLER_PRECOMPILED_FORCE_BUILD_ALL=1` so the test/doc compile builds the NIF locally. (`scripts/ci/elixir/run-tests.sh`)
- **ci(e2e, Swift): build the FFI crate the generated `Package.swift` links.** The swift e2e before-hook built only `html-to-markdown-rs-swift`, but the generated manifest links `html_to_markdown_ffi`, so `swift test` failed with `library 'html_to_markdown_ffi' not found`. Build `html-to-markdown-ffi` in the before-hook too (matching Go/C#/C). (`alef.toml`)
- **ci(e2e, PHP): drop `--locked` from the PHP test before-hook.** The PHP e2e job's extension build rewrites the native package deps to the published registry version and runs `cargo update`, mutating the workspace `Cargo.lock`; the subsequent `cargo build --locked -p html-to-markdown-php` then failed with `cannot update the lock file … --locked`. (`alef.toml`)
- **ci(node/wasm): use `--no-frozen-lockfile` for the NAPI binding install.** `build-node-napi` ran a frozen `pnpm install`, but the napi platform packages in `optionalDependencies` are pinned to the unpublished release version and can never be in `pnpm-lock.yaml`, so the install failed with `ERR_PNPM_OUTDATED_LOCKFILE` across every Node build/e2e. (shared `xberg-io/actions/build-node-napi`, `alef.toml`)
- **docs(php): recommend `pie install` over `composer require`.** The PHP package is a native `ext-php-rs` extension that `composer require` cannot load (the cause of #420); every PHP install snippet now leads with `pie install xberg-io/html-to-markdown`. (`docs/`, `readme_templates/`)

### Changed

- **ci(publish): require all 16 PHP PIE cells by explicit per-cell pattern in `verify-release-assets`.** A dropped cell now fails the release instead of silently shipping a partial PIE matrix (#333). (`.github/workflows/publish.yaml`)
- **chore(deps): re-pin `alef.toml.alef_version` to 0.26.5 and regenerate every binding, e2e suite, README, and API doc.** (alef 0.26.5)

## [3.7.0] - 2026-06-22

### Added

- **feat(mcp): expand the MCP server to full API parity with typed, discoverable options and complete tool annotations.** The `convert_html` tool now accepts a typed `config` object covering every settable `ConversionOptions` field (heading/list/escaping/whitespace/wrapping, preprocessing, image extraction, output format, tier strategy, …) instead of an opaque untyped JSON blob, so MCP clients discover all options through the tool's generated `inputSchema`; enum options are accepted as case-insensitive strings parsed by the core parsers. A new `extract_metadata` tool returns structured `<head>`/`<meta>` metadata (title, Open Graph, Twitter Card, JSON-LD/microdata, headers, links, images) as JSON. Both tools carry the full MCP annotation set (`title`, `read_only_hint=true`, `idempotent_hint=true`, `destructive_hint=false`, `open_world_hint=false`). The typed `ConvertConfig` mirror is implemented MCP-side (no changes to the alef-tracked core option types) and guarded by a drift test that fails if a core option is added without being mirrored. The `mcp` feature now implies `metadata`. (`crates/html-to-markdown/src/mcp/`)
- **feat(mcp): add prompts, resources, and completions capabilities.** Beyond tools, the server now advertises three more MCP capabilities: **prompts** — ready-made workflow templates (`convert_to_markdown`, `extract_main_content`, `inspect_metadata`) that drive the tools, with arguments; **resources** — `htmltomarkdown://options-schema` (the JSON Schema of every conversion option) and `htmltomarkdown://output-formats` (the markdown/djot/plain guide); and **completions** — argument autocompletion for prompt arguments (e.g. `output_format` → markdown/djot/plain). (`crates/html-to-markdown/src/mcp/catalog.rs`)

## [3.6.21] - 2026-06-22

### Changed

- **chore(deps): re-pin `alef.toml.alef_version` to 0.25.60 and regenerate every binding, e2e suite, README, and API doc.** Folds in the 0.25.59–0.25.60 generator fixes: the R/extendr by-reference DTO rework now emits a non-optional Named param following an optional param as `&T` (passed by reference via an owned `name_core` binding) instead of the non-compiling `Nullable<&T>::into_option()` path; Kotlin/Kotlin-Android content-union `text()` accessors reference the actual data-class payload property (`value`) instead of the non-existent `field0`; generated binding rustdoc de-links core intra-doc references (e.g. ``[`Error::LanguageNotFound`]``) to plain code spans so `rustdoc -D rustdoc::broken-intra-doc-links` passes; and `sync-versions` now runs the same `format_generated` pass as `alef all`, so version-bumped manifests (`package.json`, `composer.json`, `Package.swift`) are byte-identical to the generate path and no longer trip freshness gates. (alef 0.25.60)

## [3.6.20] - 2026-06-21

### Fixed

- **chore(precommit): scope the v2.3.0 strict doc/format/lint hooks away from the generated trees.** The canonical-hookset bump (v2.3.0, commit `8a5203b7d`) enabled several strict hooks that ran against alef-generated code and failed on it. All per-language code under `packages/<lang>/`, `e2e/`, and `test_apps/` is generated and ships as alef emits it, so each hook now carries the same generated-tree exclusion the other formatters already use: `yard-coverage` excludes `packages/ruby/` (its `native.rb` holds undocumentable Sorbet-sig RBI defs); `air-check`/`air-format` exclude `packages/` (alef formats R with styler, not air); `palantir-java-format` excludes `packages/`, `e2e/`, `test_apps/`, and the vendored `.mvn/` wrapper. The `lintr` hook runs `lintr::lint_dir(".")` and ignores pre-commit file scoping, so its exclusion lives in a new root `.lintr` (`exclusions: list("e2e", "packages", "test_apps")`); `tsc-typecheck` runs `tsc --noEmit` from the repo root, so a root `tsconfig.json` scopes type-checking to `packages/typescript/src`. `.yardoc/` (yard's cache side-effect) is gitignored. (`.pre-commit-config.yaml`, `.lintr`, `tsconfig.json`, `.gitignore`)
- **chore(precommit): exclude alef-canonical generated files from the formatters that diverge from alef's own output.** alef 0.25.58 made `alef verify` output-sensitive and the v2.3.0 hookset added `alef-docs-fresh` (which runs `alef verify`), so any prek formatter that rewrites a generated file differently than alef emits it now breaks verify. Fourteen generated files hit this because the hooks' pinned tools differ from alef's: `ruff`/`ruff-format` (line-wrapping generated Python), `pyproject-fmt` (alef runs no pyproject-fmt pass), `oxfmt` (alef formats `composer.json` via npm oxfmt; the hook uses the oxc binary with a different style), `shfmt` (generated `download_ffi.sh`/`install.sh`/`gradlew`), `cargo-sort` (the workspace-excluded R and Elixir NIF `Cargo.toml`), and `end-of-file-fixer` (the Dart `frb_generated.dart` and `test_apps` `__init__.py`). Each now carries the generated-tree exclusion the other tools already use, so the files ship exactly as alef emits them and `alef verify` stays clean. (`.pre-commit-config.yaml`)

### Changed

- **chore(deps): re-pin `alef.toml.alef_version` to 0.25.58 and regenerate every binding, e2e suite, README, and API doc.** Folds in the 0.25.55–0.25.58 generator fixes that apply here: the generated Go `cmd/download_ffi/main.go` tool now carries the standard `auto-generated by alef` marker so golangci-lint's `generated: lax` skips it (0.25.x had dropped its `//go:build ignore` guard, exposing inherent-to-a-downloader gosec/errcheck findings); the `generate`/`all` up-to-date skip now also compares against on-disk output (not just the side cache), so out-of-band drift (a `git restore`, a hand-edit, an interrupted write) is regenerated rather than silently retained; **extract** OR-merges cfg gates for same-named types with disjoint gates; **R/extendr** forwards cfg-gated core features into the generated crate's `[features]` table so it builds under `-D warnings`. The visible churn is the generator's `.d.ts`/TS-bridge reformatting (tabs→2-space, double→single quotes). Verified locally: `alef verify` is clean and the 3.6.20 bump is synced across all manifests. (alef 0.25.58)

## [3.6.19] - 2026-06-20

### Fixed

- **test_apps(php): install the PHP extension via PIE's `pkg:version` syntax.** Both generated PHP test_app installers (`test_apps/php/install.sh`, `test_apps/php_ext/run_tests.sh`) ran `pie install --version "$VERSION" <pkg>`, but PIE parses `--version`/`-V` as "print PIE's own version" and exits **without installing** — the pinned extension version was never fetched, so the registry-mode PHP smoke validated whatever stale build happened to be present (or nothing). Use `pie install "<pkg>:$VERSION"` so the targeted release is actually installed. (alef 0.25.54)
- **ci(publish): unblock the npm/Node publish under pnpm 11.8.0.** The `node-bindings` job runs `setup-node-workspace` (which installs with `--no-frozen-lockfile`) and then `build-node-napi`, whose own `pnpm install --filter` re-enforces CI's default frozen-lockfile. The napi platform `optionalDependencies` are pinned to the not-yet-published release version, so they cannot be in the lockfile — pnpm 11.6.0 treated the redundant install as a no-op, but the 11.8.0 bump made it fail (`ERR_PNPM_OUTDATED_LOCKFILE`), dropping every Node build and the npm publish. Pass `install-deps: false` so the already-installed workspace is reused. (`.github/workflows/publish.yaml`)
- **ci(homebrew): drop the macOS-Intel CLI block from the formula template.** The CLI matrix no longer builds `x86_64-apple-darwin` (Apple Silicon only), but `scripts/publish/html-to-markdown.rb.tmpl` + `homebrew.json` still referenced `cli-x86_64-apple-darwin.tar.gz`, so "Update Homebrew formulas" failed (`no assets match the file pattern`) and the tap stayed on the prior version. Remove the macOS `on_intel` block; macOS is now arm-only. (`libhtml-to-markdown` is unchanged — the C FFI still ships an x86_64-apple-darwin archive.)

### Changed

- **chore(deps): re-pin `alef.toml.alef_version` to 0.25.54 and regenerate every binding, e2e suite, README, and API doc.** Folds in the 0.25.51–0.25.54 generator fixes: the PHP PIE `pkg:version` install fix above; **Go** — unit/newtype-tuple enum constants emit serde wire values, a parameter named `result` no longer collides with the codegen variable, and `Option<&[u8]>` returns generate correctly; **R/extendr** — cfg-variant dedup, registration entries drop the stray `#[cfg(...)]`, and opaque-method/`Option`/`Vec`/error returns convert properly; **C#** — corrected host-capsule native method name; **Swift** — `Package.swift` dependency argument order and e2e `harness_extras` product override; **test_apps runner** — declared `[crates.e2e.env]` vars are exported to the run command. Verified locally: both PIE fixes are present in the regenerated test_apps and the 3.6.19 bump is synced across all manifests.
- **chore(precommit): scope `oxfmt`/`oxlint` away from generated `e2e/`+`test_apps/`.** alef has no e2e/test_apps JS/TS formatter (`alef.toml [crates.e2e.format]` covers go/python/rust/c only), so `oxfmt` was the lone tool reformatting generated test JS, producing churn on every regenerate. Add the `^(e2e/|test_apps/)` exclude that `go-fmt`, `ruff-format`, and `golangci-lint` already carry; generated test JS now ships as alef emits it, matching `e2e/ruby`, `e2e/php`, etc. (`.pre-commit-config.yaml`)

## [3.6.18] - 2026-06-20

### Fixed

- **ci(publish): unbreak Python wheels on every platform.** The shared `build-python-wheels` action source-built libheif 1.23.0 (with `libde265`/`x265` codec headers) for *all* consumers because kreuzberg links `libheif-sys`. html-to-markdown does not use libheif, but inherited the build — which fails on the `manylinux2014` (CentOS 7 EOL) base whose yum repos no longer carry `libde265-devel`/`x265-devel` (`Error: Not tolerating missing names on install`). The action now gates the libheif build behind an opt-in `build-libheif` input (default off); h2m no longer builds it, so all Python wheels build again. (`xberg-io/actions@v1`)
- **ci(publish): re-exclude `php8.5` + `macos-arm64`, restoring PHP publishing.** v3.6.17 tried to build the php8.5 Apple-Silicon PIE asset on the `macos-14` runner ([#333](https://github.com/xberg-io/html-to-markdown/issues/333)), but `shivammathur/setup-php` cannot provision PHP 8.5 on arm64 on *any* macOS runner (the install leaves empty paths — `sed: : No such file`, `/php.ini: No such file`). That failing cell skipped `upload-php-pie-release`, dropping **every** PHP PIE asset (a v3.6.15-class regression). Restoring the exclusion republishes the PHP extension for all supported targets (PHP 8.2–8.5 on linux/macos-x86_64/windows, plus arm64-darwin for 8.2–8.4). #333 stays open pending an upstream php@8.5 arm64 formula. (`.github/workflows/publish.yaml`)

### Changed

- **chore(deps): re-pin `alef.toml.alef_version` to 0.25.50 and regenerate every binding, e2e suite, README, and API doc.** Folds in the 0.25.50 cross-language visitor-codegen fixes that turn the chronically-red e2e visitor suites green: **Node** — the bridge now reads the externally-tagged `{ Custom: ... }`/`{ Error: ... }` payload key (was lowercased to `custom`/`error`, yielding `[object Object]`); **Ruby** — visitor callbacks take named params and interpolate `{placeholder}` templates; **PHP** — bare-string returns map to `Custom` for multi-payload result enums; **Elixir** — unit-variant atoms match the snake_case wire name (`:skip` no longer leaks as literal `"Skip"`); **Swift** — `visitBlockquote`'s `depth` is `UInt`, so the override satisfies the protocol. Verified locally: every language e2e suite passes and `alef verify` is clean.

## [3.6.17] - 2026-06-19

### Fixed

- **ci(publish): stage Java natives under the `macos-*` RID the loader expects, fixing Java on all macOS.** The `java-natives` matrix labelled the macOS cells `osx-aarch64` / `osx-x86_64`, which became the `natives/<rid>/` directory baked into the jar, but the loader (`NativeLib.resolveNativesRid`, alef `go_java_platform`) expects `macos-arm64` / `macos-x86_64`. The published jar therefore failed with `UnsatisfiedLinkError` on every macOS. Align the labels with the loader. (`.github/workflows/publish.yaml`)
- **ci(publish): bundle the Dart native libraries into the published package.** `assemble-dart-package` shipped the pub.dev tarball with no native libraries, so the loader fell back to a relative framework path that hardened runtimes reject — the package was unusable for every consumer. Add a `dart-natives` matrix that builds the `html-to-markdown-rs-dart` cdylib per platform and stages it into `lib/src/native/<rid>/` where the loader (`_alefHostRid`) looks. (`.github/workflows/publish.yaml`)
- **ci(publish): build the PHP 8.5 macOS-arm64 extension on the `macos-14` runner ([#333](https://github.com/xberg-io/html-to-markdown/issues/333)).** `shivammathur/setup-php` only provisions PHP 8.5 on Apple Silicon via the `macos-14` runner, not `macos-latest` (15/26). The `php-extension` matrix used `macos-latest` and excluded php8.5/macos-arm64, so that PIE asset was never built. Build the macos-arm64 cell on `macos-14` and drop the exclusion so the `php8.5-arm64-darwin` PIE asset ships. (`.github/workflows/publish.yaml`)
- **fix(task): refresh `go.sum` in the Go smoke test.** `test-apps:smoke:go` failed with "missing go.sum entry" after a version bump because alef edits `go.mod` only. Run the targeted `go mod download <module>` Go suggests — it adds the entry without rewriting `go.mod`'s block form, which `download_ffi.sh` parses for the version. (`Taskfile.yaml`)
- **test(ci): run the Java e2e step on macOS to cover native RID resolution.** The Java e2e ran the e2e step only on ubuntu, so macOS-specific binding/native regressions (such as the RID mismatch above) were never exercised. Run it on macOS too. (`.github/workflows/ci-e2e.yaml`)

### Changed

- **chore(deps): re-pin `alef.toml.alef_version` to 0.25.49 and regenerate every binding, e2e suite, README, and API doc.** Folds in the 0.25.45–0.25.49 generator fixes (incl. kotlin-android AGP 9 toolchain, swift bridge build, content-union accessors). Verified locally: `alef verify` is clean and `task alef:format` succeeds across all languages.
- **chore(precommit,alef): standardize kotlin-android formatting on ktfmt `--kotlinlang-style`.** Drop the conflicting prek `ktlint` hook (its always-format mode fought `ktfmt` over blank-line-after-brace and rewrote alef's `///` doc comments to `// /`, breaking `alef verify`), switch `alef.toml` kotlin-android format/check from gradle-`ktlintFormat` to `ktfmt` so alef and prek agree, and exclude the vendored Gradle wrapper from shellcheck. detekt remains for static analysis. (`.pre-commit-config.yaml`, `alef.toml`)
- **chore(alef): close formatter-coverage gaps for generated Go and Zig.** Add a `gofmt -w` format hook for `packages/go`/`e2e/go` (alef's Go emitter output needs canonical re-indentation), and extend the Zig formatter/check to cover `build.zig` in addition to `src`. (`alef.toml`)

## [3.6.16] - 2026-06-19

### Fixed

- **ci(publish): re-exclude `php8.5` + `macos-arm64` from the PHP extension matrix, restoring PHP publishing.** v3.6.15 removed this exclusion to chase the Apple-Silicon PIE asset for [#333](https://github.com/xberg-io/html-to-markdown/issues/333), but `shivammathur/setup-php` still cannot provision PHP 8.5 on `macos-arm64` — the homebrew tap has no `php@8.5` arm64 formula, so the cell fails at the setup step with "Could not setup PHP 8.5". Because `upload-php-pie-release` does not run when `needs.php-extension.result == 'failure'`, that single failing cell dropped **every** PHP PIE asset from the v3.6.15 release (a regression from v3.6.14's full set). Restoring the exclusion returns the matrix to all-green and republishes the PHP extension for every supported target (PHP 8.2–8.5 on linux/macos-x86_64/windows, plus arm64-darwin for 8.2–8.4). #333 stays open until upstream ships a `php@8.5` arm64 formula. (`.github/workflows/publish.yaml`)
- **ci(publish): re-publish the Ruby gems missing from v3.6.15.** The v3.6.15 run skipped `publish-rubygems` because the `Build Ruby gem (windows-x64)` cell hung ~3h on `setup-rust` and was cancelled, failing the `needs.ruby-gem.result == 'success'` gate. This is a transient runner fault, not a code defect; cutting v3.6.16 re-runs the publish so the gems ship.

## [3.6.15] - 2026-06-18

### Fixed

- **ci(publish): attempted to build the PHP 8.5 + macOS arm64 (Apple Silicon) PIE extension ([#333](https://github.com/xberg-io/html-to-markdown/issues/333)) — ineffective, reverted in 3.6.16.** Removed the build-matrix exclusion for `php8.5` on `macos-latest`, but `shivammathur/setup-php` cannot install PHP 8.5 on arm64 (no `php@8.5` arm64 homebrew formula), so the cell failed at setup and — because `upload-php-pie-release` skips on a failed matrix — dropped all PHP PIE assets from this release. See the 3.6.16 entry. (`.github/workflows/publish.yaml`)
- **docs(php): use the native `HtmlToMarkdownApi` class in all PHP examples.** The README quick-start, API reference, and docs snippets used the `HtmlToMarkdown` userland wrapper, which is only autoloaded via Composer's PSR-4 and is absent on the PIE install path (extension-only) — so `HtmlToMarkdown::convert()` raised "Class not found" for users who installed via `pie`. Examples now call `HtmlToMarkdownApi::convert()`, the native class registered by the extension itself, which works on both the PIE and Composer install paths ([#415](https://github.com/xberg-io/html-to-markdown/issues/415)). (`readme_templates/partials/`, `docs/snippets/php/`, `docs/language-guides.md`)

### Changed

- **chore(deps): bump `alef.toml.alef_version` to 0.25.44 (was 0.25.40) and regenerate every binding, e2e suite, README, and API doc.** Folds in the 0.25.41–0.25.44 cross-language generator fixes. Verified locally: `alef verify` is clean, `cargo check --workspace --all-features` compiles, and `prek run --all-files` (including the alef freshness hook) passes.

## [3.6.14] - 2026-06-18

### Fixed

- **chore(deps): bump `alef.toml.alef_version` to 0.25.40 (was 0.25.36) and regenerate every binding.** Advances the pin past the `[Unreleased]` 0.25.33 note to a tagged, published alef revision and folds in the 0.25.34–0.25.40 cross-language fixes. Binding-visible changes: **Java** drops the throwing `UnsupportedOperationException` DTO/enum-method stubs (`NodeContext.withOwnedAttributes`/`intoOwned`, `ConversionOptions.defaultInstance`, `PreprocessingOptions.defaultInstance`) — there is no JNI symbol for these yet, so the stubs compiled but threw at runtime; absence is safer than a misleading throw until DTO marshaling lands (boxed-`Boolean` serde defaults also restored). **Swift** emits a matching `pub fn visitor_handle_noop` definition for the bridge no-op declaration so the swift rust crate compiles (was `E0425` under 0.25.36's decl-without-def), alongside the upstream streaming-owner declaration rework. **Node (NAPI)** map-returning functions now convert borrowed maps to owned `HashMap` instead of returning them bare (fixes a would-be `E0308` from the new DTO-method emission). **Python (pyo3)** drops a redundant `# type: ignore[arg-type]` on the visitor assignment that `mypy --strict` (`warn_unused_ignores`) rejected, and None-guards the coercion comprehension for optional `Vec<enum>` fields. Verified locally: `task alef:generate` is fresh (`alef verify`), every workspace binding crate compiles, and `prek run --all-files` is clean.
- **ci(publish): `upload-php-pie-release` now hard-errors if zero `php-package-*` artifacts reach the aggregator instead of producing an empty PIE upload.** The download step adds `if-no-files-found: error`, so a future regression that drops all PHP build outputs surfaces immediately instead of silently shipping a release with no PIE assets — the failure mode behind v3.6.11's missing-asset reports (see [#333](https://github.com/xberg-io/html-to-markdown/issues/333)). The matrix-cell tolerance from v3.6.12 (`needs.php-extension.result != 'cancelled' && != 'skipped'`) is unchanged. (`.github/workflows/publish.yaml`)
- **ci(task rust:test): re-include `html-to-markdown-rs-dart` in the `--no-default-features --workspace` sweep.** The provisional exclusion added earlier in this `[Unreleased]` cycle is now unnecessary: alef 0.25.33 + the queued `[Unreleased]` follow-up drop `#[cfg(feature = ...)]` from every dart-wrapper mirror declaration **and** the bidirectional `From` impls, so the wrapper crate compiles cleanly under default, `--no-default-features`, and `--all-features`. Verified locally against a fresh `task alef:generate`. (`.task/languages/rust.yml`)
- **chore(deps): bump `alef.toml.alef_version` to 0.25.33 and regenerate every binding.** Sweeps in the cross-language fixes accumulated through 0.25.30–0.25.33 plus the (still-`[Unreleased]`) dart cfg follow-up that the regen incorporates locally: pyo3 `convert(...)` / `ConversionOptions(__init__)` visitor kwarg widened to `HtmlVisitor | object | None` (resolves [#403](https://github.com/xberg-io/html-to-markdown/issues/403)); swift options-field factory exported as `make{Trait}Handle` (matches docs snippets + e2e generator); java record + builder PMD cleanup (per-instance non-`final` fields, redundant component docs removed); kotlin file-level `@file:Suppress` extended with `ReturnCount` + `NestedBlockDepth` on the shared emitter; codegen `Vec<core::T>` → `Vec<wrapper::T>` conversion on non-opaque method returns (pyo3, magnus, extendr); wasm `Option<Vec<UnitEnum>>` getter/setter emission; php untagged-data-enum delegation via `serde_json::from_value`; extract honors `#[cfg_attr(alef, alef(skip))]` on `impl` blocks (no more duplicate `#[no_mangle]` symbols on builder + field name collisions); cfg-gated public function re-exports treated as binding surface; extendr `extendr_module!` registration entries gated on the source `FunctionDef` cfg; dart mirror enum + struct declarations + bidirectional `From` impls all unconditional now so `frb_generated.rs` resolves against the local crate regardless of the dart wrapper's feature set. Pin will advance again to a tagged alef revision once the `[Unreleased]` dart follow-up + extendr fix ship in 0.25.34.

## [3.6.13] - 2026-06-17

### Fixed

- **style(test_apps/go): `shfmt`-normalize `download_ffi.sh` so CI Lint's `shfmt` hook stops rejecting the v3.6.12 baseline.** The hand-written cgo library stager shipped with v3.6.12 used inline case-arm bodies; CI's `shfmt -i 2 -ci -bn -s` profile splits them onto separate lines. CI Lint failed on every v3.6.12 commit until the file was normalized.
- **(via alef 0.25.26–0.25.28)** Generated bindings pick up: PHP shared lossy and enum-tainted binding-to-core helpers now skip `binding_excluded` fields and emit `..Default::default()` so core types with custom `Default` impls (e.g. anything that pulls config from the environment) are no longer shadowed by zeroed field defaults across Python/Node/Ruby/WASM/extendr/PHP method bodies and From-impls; FFI same-name function dedup moved out of the shared extractor pass into a backend-local pass (`backends::ffi::gen_bindings::functions::cfg_dedup::dedup_same_name_functions`) so every other backend and the e2e call-export validator see the original multi-entry surface untouched (v0.25.26 had over-collapsed the shared surface and stripped `alef(skip)`-tagged siblings from every backend); FRB Dart bridge functions wrapping core functions that return primitives now emit the cross-type cast (e.g. `.map(|v| v as i64)`) instead of the redundant `.map(|v| v)` that v0.25.25–0.25.27 emitted; generated Kotlin Android files add `ReturnCount` to the `@file:Suppress` list so detekt no longer fails sealed-class deserializers with 3+ variants.
- **(via alef 0.25.27)** Generated Swift bindings emit the factory function as `make{Trait}Handle` (matching the documented spec, the docs snippets under `docs/snippets/swift/visitor/`, and the e2e test generator) instead of `make{Trait}{TypeAlias}` (which silently doubled the trait stem to `makeHtmlVisitorVisitorHandle`); generated Swift e2e visitor closures qualify the context type with the host module (`HtmlToMarkdown.NodeContext`) so the unqualified reference is no longer ambiguous against `RustBridge.NodeContext` that the test file also imports.
- **(via alef 0.25.27)** Generated TypeScript WASM e2e configs override `SsrfPolicy.denyPrivate = false` when the binding exposes the field, since WASM has no `std::env::var` and `SsrfPolicy::from_env()` always falls back to `deny_private = true`, which rejected the localhost mock-server requests every WASM e2e fixture targets.

### Changed

- **Bump alef pin to 0.25.28** (was 0.25.25), regenerating all binding outputs, e2e codegen, API reference docs, and test_apps scaffolds.

## [3.6.12] - 2026-06-17

### Fixed

- **test_apps/go: stage cgo FFI library into the binding's module-cache `.lib/<platform>/` so smoke tests link cleanly.** The published Go binding declares `// #cgo LDFLAGS: -L${SRCDIR}/.lib/<platform>/ -lhtml_to_markdown_ffi`, but `${SRCDIR}` resolves to the binding's source directory under `GOMODCACHE`, which is empty after `go mod download`. The package's own `cmd/download_ffi` is `//go:build ignore`-tagged and only invocable via the binding's own `go generate`, which test_apps doesn't run. Fix: `test_apps/go/download_ffi.sh` reads `MODULE_VERSION` from `go.mod`, downloads the matching `html-to-markdown-rs-ffi-v<version>-<rust-triple>.tar.gz` artifact from the GitHub release, caches it under `${XDG_CACHE_HOME:-~/.cache}/html-to-markdown-ffi/`, makes the binding's module-cache subtree writable, and copies the library into `<binding>/.lib/<platform>/`. The smoke task invokes the script before `go test`. Restores Go to the registry-mode smoke matrix.
- **ci(publish): replace single-success gate on `upload-php-pie-release` with cancelled/skipped guards so partial matrix success still publishes the available PIE archives.** The job was gated on `needs.php-extension.result == 'success'`, which meant a single transient matrix-cell failure (e.g. the `ENOTFOUND` artifact-upload flake observed on the v3.6.11 publish run) cascaded into skipping the entire PIE upload step. With the matrix already declared `fail-fast: false` and the in-job `actions/download-artifact@v4` step configured with `pattern: php-package-*` + `merge-multiple: true`, the upload now proceeds whenever at least one matrix cell produced an artifact. Transient flakes no longer block the entire release surface.
- **(via alef 0.25.20–0.25.25)** Generated binding suites pick up: NAPI TypeScript `import { A as B }` → CJS `{ A: B }` rename translation (so `oxfmt` no longer aborts on `Expected ',' or '}' but found 'as'`); Go FFI `copyLibraryToBindingPackage` no-op when source aliases destination (avoids 0-byte `libhtml_to_markdown_ffi.dylib` after `go generate`); Swift `Package.swift` `v__ALEF_SWIFT_VERSION__` placeholder substitution at scaffold-write time so SwiftPM consumers don't 404 against the release asset URL; Homebrew `run_tests.sh` formula-installed CLI preflight using the parameterized binary name; alef post-generation formatter pipeline aligned with downstream prek hooks (`ktfmt` for Kotlin format, `gofmt`+`goimports` for Go, `oxfmt` for TS/JSON, `shfmt` for shell scripts, `php-cs-fixer` for PHP, `cargo sort` for emitted Cargo.toml). Resolves CI Lint formatter divergence that had been red on every commit since v3.6.11.
- **(via alef 0.25.25)** Generated PHP test_apps `install.sh` emits `pie install --version "$VERSION" "<pkg>"` instead of the deprecated `pie install "<pkg>:$VERSION"` form that PIE 1.4.5+ rejects with `Unable to find an installable package <pkg> for version <ver>`. Restores PHP to the registry-mode smoke matrix.
- **(via alef 0.25.25)** Generated `[crates.dart] excluded_default_features` / `[crates.swift] excluded_default_features` keep optional cargo features out of the wrapper's `default = [...]` array while still declaring them as opt-in forwarding entries, preventing target-conditional cross-compile activation of features whose system deps aren't cross-compile-ready.
- **(via alef 0.25.25)** Generated publish/vendor flow retries `cargo update` + `cargo metadata` on crates.io registry-index propagation lag (up to 6 attempts × 30 s) so per-language build jobs don't hard-fail in the first few minutes after `Publish Rust crates` completes.

### Changed

- **Bump alef pin to 0.25.25** (was 0.25.19), regenerating all binding outputs, e2e codegen, API reference docs, and test_apps scaffolds.

## [3.6.11] - 2026-06-16

### Fixed

- **Table layout pre-pass: skip nested-table rendering during column-width measurement (issue #406, residual fix).** The `MAX_CELL_WIDTH = 200` cap shipped in v3.6.10 (`4013a6864`) bounded the discarded *output* but not the measurement CPU. For deeply nested layout HTML (e.g. the reporter's Outlook digest: 393 `<table>` tags, 851 KB), `cell_text_content` still recursively dispatched `handle_table_with_context` on every nested table during the outer measurement pre-pass, triggering its own pre-pass on every descendant cell — combinatorial explosion (`cells × nested_cells × ...`) unbounded at greater nesting depth. The reproducer still ran for tens of minutes on v3.6.10. The fix threads `measure_width_only: bool` through the conversion `Context`; `walk_node`'s `"table"` arm short-circuits to descendant text content when set, keeping the pre-pass linear in descendant character count. The reproducer now converts in 0.04 s. Regression test `nested_layout_tables_convert_within_wall_clock_budget` covers a synthetic 4×4 nested-cell fixture with a 10s wall-clock budget. Tier-1 vs Tier-2 separator-row dash counts now diverge on the nested-table fallback (Tier-1 still measures the rendered cell text); existing tier-divergence tests were updated to assert outer-row content equality instead of byte-equality. Resolves the residual #406 report from `hobofan` on 2026-06-16.

- **(via alef 0.25.19)** Generated Swift app harness migrates fixtures JSON from triple-quote multi-line string literal to chunked-array `[...].joined()`, avoiding Swift's `multi-line string literal content must begin on a new line` error when the Jinja whitespace-trim placed content on the same line as the opening `"""`.

- **(via alef 0.25.19)** Generated FFI service-API codegen clones borrowed opaque-pointer params at the call site so consuming Rust APIs that take `T` by value receive an owned value while the C caller retains the original handle for its own `_free` call.

- **(via alef 0.25.19)** Generated csharp e2e csproj template branches `<RuntimeIdentifier>` on `OSArchitecture`, picking `osx-arm64` / `linux-arm64` / `win-arm64` on arm64 runners instead of hardcoded `*-x64`.

- **(via alef 0.25.19)** Generated magnus binding `Cargo.toml` emits a cfg-feature forwarding `[features]` block so any `#[cfg(feature = "X")]` arms in the binding compile under `-D warnings`.

### Added

- **(via alef 0.25.19)** Generated language doc pages translate Rust type spellings to language-native terminology in prose (Python `list[X]` / `dict`, TypeScript `X[]` / `Record`, Go `[]X` / `map`, etc.), applied via `map_non_code_lines` so code fences stay intact.

## [3.6.10] - 2026-06-16

### Fixed

- **Table column-width: cap per-cell measurement at 200 characters (issue #406).** Converting Microsoft-style HTML emails with hundreds of nested layout tables produced runaway output (73 MB / 2.2 s for the reporter's 851 KB reproducer) because `collect_row_cell_widths` used the *rendered* width of each cell — including any inner table's separator rows — as the outer column width. The outer table's separator was then emitted with that many dashes, which fed back into the grandparent's measurement, doubling at every nesting level. Capping the per-cell measurement at `MAX_CELL_WIDTH = 200` bounds total output to `O(cells × 200)` and keeps both the Tier-1 fast path and the Tier-2 walker numerically consistent. The reproducer now converts in 0.47 s producing 218 KB of markdown. Bisected by the reporter to alef-driven `Arc<Mutex<>>` visitor migration in 3.4.1 (`7f6178f25`); the v3.6 series already fixed the visitor-mutex hot loop in `4863d1ab6`, but the underlying exponential-width feedback remained. Regression test added: `deeply_nested_layout_tables_do_not_produce_runaway_output`.

- **Bump alef to 0.25.18.** Picks up: (a) the **e2e visitor codegen fixes** for Node and C# — Node tests now pass `{ visitor: V as any }` through `options.visitor` rather than as a silently-dropped third positional argument, C# tests merge into the existing `new ConversionOptions()` literal rather than appending a fourth arg, restoring every `Custom`-substitution visitor test. (b) The **Zig e2e build.zig fix** dropping the duplicated `_run` suffix in test-sequencing `dependOn` calls (`conversion_run_run` → `conversion_run`), unblocking Test: Zig. (c) The **Swift box-delegate cast fix** removing the spurious `Int(...)` wrap on `usize`/`isize` args so the bridge call matches the `UInt` protocol declaration, unblocking Test: Swift. (d) The **Ruby Rakefile modifier-`if` fix** for `cross_compile_versions` assignment, removing the `Style/IfUnlessModifier` rubocop offense that broke all 4 Test: Ruby matrix jobs. (e) The **PHP PIE URL `{OSLower}` placeholder fix** so `pie install xberg-io/html-to-markdown` resolves the correct asset name (resolves h2m #333). (f) The **binding-crate feature passthrough block** emission in FFI/Node/PHP/Wasm `Cargo.toml` — each binding crate now declares `metadata`, `visitor`, `inline-images`, `testkit` as passthrough features forwarding to `html-to-markdown-rs/X`, fixing the `unexpected cfg condition value` errors under `RUSTFLAGS="-D warnings"` for code paths gated by core features. (g) The **swift-bridge owner-type extern block + cfg-gating fix** preventing proc-macro expansion failures on cfg-disabled types. (h) The **JNI trait-`use` emission fix** so Tier-B Rust-public trait extension points are reachable from the generated JNI shims.

## [3.6.9] - 2026-06-15

### Fixed

- **Bump alef to 0.25.17.** Picks up the dart `unreachable_patterns` allow attribute for the crate-root, which was missing from the dart `gen_rust_crate` scaffold. v3.6.8's CI Rust failed with `unreachable pattern` errors at `packages/dart/rust/src/lib.rs:1739` and `:2102`: the dart enum-conversion path emits an `_ => unreachable!("cfg-gated variant ... not active in this build")` catch-all so the match remains exhaustive when a `#[cfg(feature = "X")]`-gated variant is compiled out, but the dart binding's `[features]` table forwards `testkit` unconditionally, so the cfg-gated arm IS compiled in and the catch-all is unreachable — `-D warnings` turned the lint into an error. alef 0.25.17 adds `unreachable_patterns` to the existing `#![allow(unused_variables, unreachable_code)]` crate-root attribute, matching the swift backend's already-correct allow list.

## [3.6.8] - 2026-06-15

### Fixed

- **Bump alef to 0.25.16.** Picks up the vendor `manifest_abs` canonicalize fix (resolves Elixir NIF / PHP extension / Ruby gem / Python sdist `alef publish prepare` failures from the v3.6.7 publish run), the Dart `cfg(testkit)` whitespace fix (unblocks `Validate: Rust` and `Test: Dart` on main), the rustler `.clone()` skip for reference parameters, the JNI workspace-target host fix, the extendr enum type-path resolver, the swift `From<core>` cfg-gating for variant arms, the FFI visitor enum-context i32 emission, the visitor_result bare-string `Custom` routing, the e2e csharp SDK-AssemblyInfo suppression, prerelease set-version support (0.25.10–11); the swift cfg-union postprocessing fixes for default-build wrapper-type emission and the `default = [<features>]` Cargo.toml emission (0.25.12–15); and the drop of cfg propagation on enum `From`-impl match arms — binding crates do not declare gated features themselves but pull the core dependency with them enabled, so propagating `#[cfg(feature = "testkit")]` to binding-side arms produced `unexpected cfg condition value: testkit` errors under `-D warnings` on py/node bindings (0.25.16).
- **C# assembly identity now tracks `<Version>`.** Deleted the hand-committed `packages/csharp/HtmlToMarkdown/Properties/AssemblyInfo.cs` (carrying a stale `AssemblyVersion("3.4.0")`) and switched the scaffold to let the .NET SDK derive `AssemblyVersion`, `AssemblyFileVersion`, and `AssemblyInformationalVersion` from `<Version>` plus the new `<Company>` / `<Product>` MSBuild properties. The published 3.4.0 assembly identity was breaking `task test-apps:smoke:csharp` against every NuGet package since the original ship.

### Changed

- **CI E2E PHP/Ruby builds: re-enable `rewrite-native-deps`.** The workaround in 3.6.7 (`a8c7fe40c`) disabled `rewrite-native-deps` on PHP and Ruby `build-*` actions to dodge the alef 0.25.9 canonicalize bug. With 0.25.11's fix the action works correctly again, so the four `rewrite-native-deps: "false"` overrides are removed.

## [3.6.7] - 2026-06-15

### Fixed

- **Restore Dart binding.** v3.6.6 silently dropped Dart from `[workspace].languages` in `alef.toml` and moved `packages/dart/rust` from workspace `members` to `exclude` in the root `Cargo.toml`. The pub.dev publish of `h2m` 3.6.6 succeeded only because the Dart workflow's tag-time checkout predated the exclusion; any subsequent regeneration would have orphaned the binding. Dart is restored to both lists.

- **Bump alef to 0.25.9 — fixes Elixir NIF `[patch.crates-io]` no-op error blocking the v3.6.6 publish.** alef 0.25.7's Elixir scaffold emitted an unconditional `[patch.crates-io]` block in the generated Rustler NIF `Cargo.toml` with entries shaped `alloc-no-stdlib = { version = "=2.0.4" }` — cargo rejects these as `patch for 'alloc-no-stdlib' points to the same source, but patches must point to different sources`, failing every Elixir NIF matrix cell (linux x86_64/aarch64, macos x86_64/arm64) plus the Hex publish job on v3.6.6 publish run 27510348278. alef 0.25.9 (commit `e1e86bbc1`) replaces the broken patch block with direct `[dependencies]` entries pinning `alloc-no-stdlib = "=2.0.4"`, `alloc-stdlib = "=0.2.2"`, `brotli-decompressor = "=5.0.1"` plus matching `cargo-machete` ignores. 0.25.9 also ships the Ruby Rakefile YARD-coverage fix (`55f1eccf6`).

## [3.6.6] - 2026-06-14

### Fixed

- **Bump alef to 0.25.7 — fixes `alef publish prepare` for non-workspace-member NIF crates (Ruby gem, Elixir NIF).** alef 0.25.4–0.25.6 had a bug where `cargo update --locked` and `cargo metadata --locked` would fail for binding crates that are not workspace members (like the Rustler NIF and Magnus gem). The seed lockfile lacked a `[[package]]` entry for the binding crate, causing cargo to reject it. alef 0.25.7 dropped `--locked` from the metadata validation step and set `.env_remove("CARGO_BUILD_LOCKED")` to allow cargo to resolve these crates from the registry. This fix restores Ruby (all platforms) and Elixir NIF (all platforms + Hex package) support.

## [3.6.5] - 2026-06-14

### Fixed

- **Elixir Hex/NIF + Ruby gem publish: bump alef pin to 0.25.4 so `alef publish prepare` produces a valid binding lockfile.** alef 0.25.1's `vendor::scrub_or_regenerate_lock` strict-mode path failed on every v3.6.4 Elixir NIF and macOS/Linux Ruby gem build with `cargo update -p <lockfile> (or final cargo metadata validation) failed (exit code 101) ... cannot update the lock file ... because --locked was passed` — the seed lockfile's workspace-member path entries collided with the registry-source entries the rewrite added, and the final `cargo metadata --locked` validation could not reconcile them. alef 0.25.3 added `strip_workspace_member_entries` to drop the path-source entries from the seed before per-member `cargo update -p` runs, plus a full registry-URL package-id spec (`registry+https://github.com/rust-lang/crates.io-index#NAME@VERSION`) to disambiguate the per-member update. 0.25.4 also escapes Rust reserved keywords in extendr struct fields (relevant to R bindings consuming flat data enums with `serde(tag = "type")`).

- **Python sdist on Alpine/musl: `actions/rewrite-native-deps` v1.8.69 now strips `path = "..."` from `[workspace.dependencies]` entries.** The 3.6.4 sdist's root `Cargo.toml` shipped with `[workspace.dependencies] html-to-markdown-rs = { version = "3.6.4", path = "crates/html-to-markdown" }`, and cargo eagerly validates every workspace-dep entry on `pip install` — bailing with `failed to read .../crates/html-to-markdown/Cargo.toml` because the workspace crate is not bundled in the sdist. v1.8.66 added `[patch.*]` stripping for sdist consumers (resolved #390) but missed `[workspace.dependencies]`. The new step drops `path` from every workspace-dependency entry, leaving the version so the dep resolves from crates.io on consumer install. Resolves #402.

## [3.6.4] - 2026-06-14

### Fixed

- **CLI: drop `reqwest` `brotli` feature to keep `alloc-no-stdlib` on 2.x.** The transitive `brotli` crate (pulled in via `async-compression`) jumped to 8.x which expects `alloc-no-stdlib` 3.x, but the workspace resolves the sibling at 2.x. The resulting trait drift broke `cargo build` on stable rustc with `the trait bound 'StandardAlloc: alloc::Allocator<...>' is not satisfied`, blocking the ubuntu-latest Python wheel build during the v3.6.3 publish run. The CLI's HTTP client now uses gzip+deflate only.

- **Publish workflow: dispatch `publish-pubdev` with the release tag, not the branch ref.** When `publish.yaml` was triggered by `release: published`, `github.ref_name` resolved to the branch where the release was authored (e.g. `main`). The child workflow's OIDC token then carried `refType=branch`, which pub.dev rejects with `publishing is only allowed from 'tag' refType`. Dispatch now uses `needs.prepare.outputs.tag` so the child runs against the tag and the OIDC token is accepted.

- **Taskfile: `test-apps:test:zig` now grep'd the root `Cargo.toml` for the workspace version.** The previous grep targeted `crates/html-to-markdown/Cargo.toml`, which uses `version.workspace = true` and has no literal version line — `H2M_VERSION` resolved to empty and the zig-fetch URL collapsed to `…/download/v/html-to-markdown-rs-zig-v.tar.gz` (404).

### Changed

- **Track `Cargo.lock` for reproducible builds.** `Cargo.lock` was previously gitignored, defeating `cargo build --locked` everywhere it was used. Every CI runner resolved deps fresh, allowing semver-compatible drift to silently introduce broken transitives (the brotli/alloc-no-stdlib mismatch above). The root workspace lockfile plus the per-package nested lockfiles (R, Ruby, Elixir NIF, e2e/rust, test_apps/rust) are now tracked.

- **Pass `--locked` to `cargo build` in CI, publish, scripts, and `alef.toml` hooks.** Now that the lockfile is tracked, every CI/publish cargo build runs with `--locked` so a dirty index can't silently substitute newer transitive deps. Local-dev tasks (`.task/languages/rust.yml`) intentionally remain without `--locked` so contributors can pick up dep updates.

- **Regenerated cross-language bindings with alef 0.25.1.** Pulls in: csharp e2e codegen using `KreuzbergConverter` facade; swift e2e codegen no longer emits `?` chains on non-Optional `RustString` returns; C e2e codegen panics on missing `fields_c_types` keys instead of silently miscompiling; FFI/NAPI/PyO3/Magnus/Rustler test surface cleanup; `vendor::scrub_or_regenerate_lock` preserves workspace lockfile pins via per-member `cargo update` + `cargo metadata --locked` validation; NAPI strips the `readonly` keyword from emitted `service.cjs`.

## [3.6.3] - 2026-06-14

### Changed

- **alef extension API: `[crates.ffi].visitor_callbacks` is now backed by the new alef per-extension
  config mechanism.** alef's `parse_config` hook now receives the real `[extensions.<name>]` section
  from `alef.toml` rather than always `None`. The `visitor_callbacks = true` knob in `[crates.ffi]`
  remains the correct way to enable the visitor/callback FFI pattern — it is a general alef feature
  shared with Go, Java, C#, and other FFI consumers, not h2m-specific. A new
  `transform_emitted_files` hook on the `Extension` trait is also available for downstream post-
  processing of generated files; h2m does not currently use it but can opt in without alef changes.

### Fixed

- **CI: `upload-go-release` now gates on `release_go == 'true'`.** The job's conditional was missing the explicit `needs.prepare.outputs.release_go == 'true'` check, allowing it to run during partial publishes when Go is intentionally skipped. Without this guard, skipped Go builds would still trigger FFI uploads/finalize-release's go-module-path tagging. Now properly skips when Go release is disabled.

## [3.6.2] - 2026-06-13

### Fixed

- **Python `.pyi`: `ConversionOptions.visitor` attribute type now resolves to `HtmlVisitor | None`.** The class-attribute annotation pass in alef's pyo3 stub emitter previously printed the opaque `VisitorHandle` concrete type for `Option<dyn HtmlVisitor>` fields, while `__init__` parameters and `convert(...)` already resolved to the `HtmlVisitor` Protocol. Field-style assignment (`options.visitor = MyVisitor()`) is now type-clean under pyright/pylance. Closes #403.

- **NAPI `convert`: visitor callbacks now fire when passed via `options.visitor`.** The generated `convert(html, options)` shim previously declared a third standalone `visitor` parameter and used it directly while ignoring `options.visitor`. The TypeScript surface advertises a single uniform entry point — `convert(html, { visitor: { … } })` — so visitor callbacks routed through `options` were silently dropped. Fixed in alef v0.24.16; the standalone parameter is gone and `options.visitor` is the sole source. Closes #395.

- **NAPI options-field bridge: drop unused `mut` on the closure binding.** The generated `convert(html, options)` shim's `Option<JsConversionOptions>.map(|o| …)` closure moved `o` straight into `o.into()` without mutating the binding, so `mut o` triggered `warn(unused_mut)` on `crates/html-to-markdown-node/src/lib.rs`. Fixed in alef v0.24.17 by dropping `mut` from the napi-side closure binding; the wasm-side template is unchanged because it does mutate `o.visitor` before the `into()` call.

- **Ruby: precompiled-gem ABI fallback to source-gem.** The `cross_compile_versions` list in the Magnus Rakefile template now targets Ruby 3.5/3.4/3.3/3.2 (dropping 3.1, adding 3.5), and the gemspec `required_ruby_version` bounds the upper edge with `>= 3.2.0, < 4.0`. On Ruby 4.0+ / 4.1.0dev, RubyGems now refuses the precompiled platform gem and falls back to the source gem, eliminating `incompatible ABI version of binary` load failures on prerelease ABIs. Closes #405, #409.

- **C# / Kotlin Android: wrapper class renamed `HtmlToMarkdownRs` → `HtmlToMarkdownConverter`.** The previous `…Rs` suffix was Rust-implementation-bleed in the public binding surface. The new name is idiomatic for both ecosystems and matches the names already used in our hand-written docs. **BREAKING for downstream C# / Kotlin-Android consumers**: apply `s/HtmlToMarkdownRs/HtmlToMarkdownConverter/g` to source code that imported or invoked the wrapper. Closes #408.

- **PHP PIE: macOS install resolves the extension at the archive root.** `pie install xberg-io/html-to-markdown` on macOS arm64 previously failed because the staged extension was named `html_to_markdown.so` while PIE's `UnixBuild` probes for `<extname>.<dylib_ext>` on macOS. The publish workflow now stages the extension as `html_to_markdown.dylib` on macOS targets and `.so` on Linux, restoring PIE installs on Apple Silicon and Intel Macs. Closes #334.

- **Python wheel: `HtmlVisitor` trait-bridge runtime-import resolved.** alef's pyo3 trait-marker-class emitter creates `_Trait{Name}Marker` types in the DTO module (`visitor.rs`) to satisfy TypeScript/Ruby/etc. downstream trait surface generation. The PyO3 bridge uses these markers as import-path sentinels, but h2m's `ConversionOptions` visitor field only became public in v3.6.1 after its prior conditional `#[cfg(feature = "visitor")]` gate was removed. Wheels shipped without the marker import working, causing `ImportError: cannot import name '_TraitHtmlVisitorMarker'` at runtime when user code called `convert(..., visitor)`. Fixed in alef v0.21.0 (marker-import path resolved from the actual DTO module).

- **Java JAR: `NativeLib` RID alignment with publish-workflow classifier names.** The publish workflow (`publish.yaml`) uploads native libraries with RID classifiers: `osx-aarch64`, `osx-x86_64`, `linux-x86_64`, `linux-aarch64`, `windows-x64`. The `NativeLib.scala.kt` loader looked for `osx-arm64` and `osx-x86-64` (mismatched case and infix), breaking native library resolution at JAR initialization. Fixed in alef v0.20.5; RID names now match the publish assets exactly.

- **R tarball: GitHub-release install fixed.** The CRAN tarball sources from GitHub release tags when users install from `.tar.gz` source (non-CRAN edge case). The `configure` script ran `cargo` with `CARGO_HOME=...` override to redirect vendor offline access, but `cargo vendor` itself does not honor `CARGO_HOME` (only cargo build does), so the offline build failed with "could not find `html-to-markdown` in the registry". extendr 0.18.1 correctly passes `--registry crates-io --offline` through to `cargo vendor`, restoring offline builds. The configure script also fixed inbound `str_as_str` imports from extendr that the v0.18.0 landing initially broke.

- **Homebrew: `brew trust` prerequisite documented.** Homebrew 6.0+ (released 2026-05) requires `brew trust xberg-io/tap` before installing from the third-party tap. Updated `docs/installation.md`, `README.md`, and `test_apps/homebrew/run_tests.sh` with the trust step and explanatory note. Smoke test now calls `brew trust "$TAP" || true` before `brew bundle install`, making CI-tested on Homebrew 6.0+ environments.

### Added

- **CITATION.cff `date-released:` auto-stamped.** `alef sync-versions --release-date YYYY-MM-DD` now stamps the release date directly into the workspace-generated `CITATION.cff`, eliminating the prior hand-edit step at release-cut time. The release date for v3.6.2 is `2026-06-13`. Closes #327.

## [3.6.1] - 2026-06-12

### Fixed

- **Ruby gem: `Rakefile` reverted to v3.5.5 working pattern.** v3.6.0's alef 0.21.0 regen introduced `RbSys::ExtensionTask` with manual `Dir.chdir` nesting that broke nested `file "Cargo.lock"` task declarations. Reverts to the v3.5.5 `Rake::ExtensionTask` pattern for all 5 platforms (macOS arm64/x86_64, Linux x86_64/aarch64, Windows x64). Resolves bundle exec compilation failures.

- **Zig: curl HTTP/2 support.** Zig's vendored curl bindings required `+h2` feature flag for HTTPS URL fetch operations. Updated `packages/zig/build.zig.zon` to enable the feature, restoring GitHub-release downloads on Zig consumers.

- **Homebrew bottle ordering stabilized.** alef v0.20.5 sort order for bottle entries now matches `brew audit` expectations; formulae no longer fail the Homebrew official-tap checklist.

## [3.6.0] - 2026-06-12

### Added

- **Tiered HTML-to-Markdown conversion architecture.** Clean HTML inputs now have an opt-in
  fast path through a Tier-1 single-pass byte scanner (`converter/tier1/`); on anything the
  scanner cannot prove byte-equivalent to the existing Tier-2 DOM walker, it returns a
  structured bail and the dispatcher falls back to Tier-2 (`tl::parse` + `walk_node`)
  transparently. Output is always byte-equal to what Tier-2 would have produced for the
  same input — verified by 116 oracle snapshots and a per-fixture byte-equality integration
  test (`tests/tier1_byte_equality_test.rs`). Tier-3 (`html5ever` repair) remains the
  fallback for truly malformed HTML.

- **`ConversionOptions::tier_strategy`** (`TierStrategy::Auto` | `Tier2` | `Tier1`)
  for runtime tier selection. `Auto` (default) lets the classifier decide based on the
  prescan signals and options shape; `Tier2` forces the existing DOM-walk path; `Tier1` is
  testkit-only (`#[cfg(any(test, feature = "testkit"))]`) for debugging and benchmarking.
  Exposed to Node (`tierStrategy: "auto" | "tier2"`) and Wasm (`WasmTierStrategy` enum).
  CLI and Python pick up the field via the standard options-mapping flow.

- **Tier-1 router style-option gate.** The classifier forces Tier-2 when any of these
  deviate from Tier-1's hardcoded value: `heading_style`, `code_block_style`,
  `strong_em_symbol`, `bullets`, `list_indent_*`, `whitespace_mode`, `newline_style`,
  `escape_*` flags, `output_format`, `link_style`, `url_escape_style`, `compact_tables`,
  `default_title`, `sub_symbol`, `sup_symbol`, `highlight_style`. 47 integration tests
  enforce the gate.

- **Tier-1 conservative bail set.** Tier-1 returns `Err(BailReason::*)` rather than emit
  potentially-wrong output on: custom elements, CDATA, unescaped `<`, inline SVG, HTML
  5 optional-close edge cases, `<table>` with rowspan/colspan/block-children-in-cells/
  caption/mixed-section-order, nested lists (Tier-2 cycles bullets by depth), `<pre>`
  with non-Indented `code_block_style`, named HTML entities outside the 45-entry
  zero-alloc table, table cells containing `|`, and `<br>` inside table cells. Tests
  cover every variant.

- **Benchmark harness** (`tools/benchmark-harness/`, binary `htmbench`) with
  `run` / `compare` / `oracle` / `oracle:bless` / `survey` / `mdream` subcommands.
  Per-group regression guardrails (`baselines/baseline.json`, `guardrails.json`).
  Wired under `task bench:*` namespace.

- **Bench regression gate in CI** — every PR that touches the Rust core or bench harness now
  runs `task bench:oracle && task bench:run && task bench:compare` on `ubuntu-24.04-arm`,
  failing on any fixture that exceeds its per-group threshold in
  `tools/benchmark-harness/guardrails.json` (5% clean_large, 8% clean_medium, 10% other groups,
  30% adversarial). Baselines are blessed deliberately by humans, not automatically by CI.

### Changed

- **Publish workflow now authenticates via the `kreuzberg-dev-publisher` GitHub App** (org-level
  `BOT_APP_ID` / `BOT_APP_PRIVATE_KEY`) for all writes (own-repo pushes, tag updates, release-asset
  uploads, homebrew-tap commits, Discord dedup marker). OIDC publisher jobs (PyPI/npm/hex/Maven/NuGet/crates)
  are unaffected. `HOMEBREW_TOKEN` is no longer used.

- **BREAKING: `NodeContext::attributes` is no longer a public field.** Access attributes via
  the new `attributes()` method (`fn attributes(&self) -> &BTreeMap<String, String>`).
  Struct-literal construction of `NodeContext` is no longer possible; use the provided
  constructors: `with_borrowed_attributes`, `with_owned_attributes` (public), or
  `with_lazy_attributes` (pub(crate), for internal use only). Attributes are now
  lazily materialized from the DOM on first access when constructed via the internal path,
  eliminating the per-element `BTreeMap` allocation on the visitor hot path. Measured
  throughput improvement with visitor enabled (100-iteration harness, Apple Silicon):
  small_html −29%, medium_python −10%, large_rust −24%, tables_countries −24%.

- **`DomContext::parent_tag_name` now returns `Option<&str>` instead of `Option<String>`.**
  This is an internal API (`pub(crate)`); external consumers are unaffected.

### Performance

- **Tier-1 byte scanner activated in production** (`tier_strategy = Auto`). The classifier
  decides per-input whether Tier-1's single-pass byte scanner runs; on bail, the dispatcher
  falls back to Tier-2 transparently. Measured throughput on the harness corpus (29 fixtures,
  6.4 MB total; Apple Silicon, `cargo build --release`):

  | Fixture                                  |   Size | ms (best) | Throughput |
  |------------------------------------------|-------:|----------:|-----------:|
  | `real-world/wikipedia/medium_python.html` | 1.24 MB | 62.58 ms  | 19.0 MB/s  |
  | `real-world/wikipedia/large_rust.html`   | 1.07 MB | 37.17 ms  | 27.3 MB/s  |
  | `mdream/github-markdown-complete.html`   | 430 KB | 10.57 ms  | 38.7 MB/s  |
  | `mdream/react-learn.html`                | 265 KB | 12.11 ms  | 20.9 MB/s  |
  | `mdream/wikipedia-small.html`            | 166 KB |  5.63 ms  | 28.1 MB/s  |
  | `real-world/issues/gh-121-hacker-news.html` |  57 KB |  1.08 ms  | 50.3 MB/s  |
  | `mdream/nuxt-example.html`               | 3.6 KB |  0.029 ms | 116.1 MB/s |

  Per-group regression thresholds (5–30%) are enforced on every PR via `task bench:compare` (see Added section).

- **memchr-driven text scan**: `decode_and_collapse_into` and `decode_entities_into` use
  `memchr::memchr3` / `memchr::memchr` to skip ahead to the next special byte (`<`, `&`,
  whitespace boundary) and bulk-copy plain text runs in a single `push_str`. Replaces a
  byte-by-byte conditional inner loop and closes a substantial portion of the gap to main's
  heavily-optimized Tier-2 path on Wikipedia-scale documents (e.g., +32% on `wikipedia-small`).

- **Tier-1 dispatcher reuses normalized input on Tier-2 fallback.** When the Tier-1 scanner
  bails or the classifier routes to Tier-2, the dispatcher threads the already-computed
  `normalize_input` `Cow<str>` through to the Tier-2 path instead of recomputing it. Eliminates
  one full-input pass on every bail-fallback or Tier-2-routed call.

- **`collapse_excess_blank_lines` in-place.** Replaced the fresh-`String` rewrite with
  `String::retain`, eliminating an output-sized allocation on every Tier-1 success whose
  output contains `\n\n\n`. Measured wins (best-of-3, `--force-tier1`, vs prior commit):
  `wikipedia/lists_timeline` -3.81%, `gh-121-hacker-news` -4.23%, `github-markdown-complete`
  -2.99%, `wikipedia/medium_python` -2.94%, `mdream/wikipedia-small` -2.68%, `mdn-array`
  -2.12%, `gh-190/firsteigen` -2.55%. No fixture regressed.

- **`htmbench --force-tier2`** flag mirroring `--force-tier1`, for clean head-to-head
  benchmarks now that the Auto router activates Tier-1 on most fixtures and so cannot be
  used as a Tier-2 control.

- **`convert()` accepts options as a bare `ConversionOptions`** in addition to
  `Option<ConversionOptions>` (resolves #398). The second parameter now bounds
  `impl Into<Option<ConversionOptions>>`, so `convert(html, opts)`,
  `convert(html, Some(opts))`, `convert(html, None)`, and
  `convert(html, ConversionOptions::default())` are all valid Rust call shapes.
  Existing callers continue to compile unchanged — this is purely additive
  flexibility, not a breaking change. The ~250-line conversion body is held in a
  private non-generic `convert_inner` so the generic wrapper monomorphises exactly
  once per call site rather than per `Into` impl chosen.

- **`url_escape_style` option** (`UrlEscapeStyle::Angle` | `UrlEscapeStyle::Percent`). When set
  to `Percent`, link and image destinations are percent-encoded instead of wrapped in angle
  brackets, producing output that all Markdown parsers handle correctly even when the URL
  contains `<`, `>`, spaces, or parentheses (resolves #392).

### Fixed

- **Non-deterministic SVG attribute serialization** in `converter/media/svg.rs`:
  `serialize_element` iterated `tag.attributes()` over astral-tl's internal `HashMap`,
  which has non-deterministic iteration order. SVG `data:` URIs therefore differed across
  runs. Fixed by sorting attributes by name before emission, restoring determinism.

- **spurious blank lines after frontmatter and lists (MD012)** (resolves #399). Block-level
  emission now collapses runs of three or more consecutive newlines into exactly two, so
  the frontmatter→body and list→next-block transitions no longer produce extra blank lines
  that violate markdownlint MD012.

- **autolinks: bare paths and filenames are no longer wrapped as autolinks** (resolves #397).
  Per GFM §6.5, autolinks require an absolute URI with a scheme — but the previous check only
  compared the link text to the `href`, so `<a href="foobar.png">foobar.png</a>` became the
  invalid `<foobar.png>` (which parsers read as a literal HTML tag). Added `has_uri_scheme`
  helper that validates the RFC 3986 scheme grammar (`ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )`
  followed by `:`). Bare paths, fragments, and filenames now render as `[text](href)`. `https://`,
  `mailto:`, `ftp://`, `data:`, and other schemed URLs continue to autolink as before.

- **node binding: visitor callbacks (`visitText`, `visitLink`, `visitHeading`, …) now fire**
  (resolves #395). `JsHtmlVisitorBridge` now stores a persistent `napi::bindgen_prelude::ObjectRef<false>`
  obtained via `Object::create_ref()` and materializes a fresh local `Object` handle per callback
  through `obj_ref.get_value(&env)`. The previous bridge stored raw `napi_value` pointers extracted
  via `transmute_copy` and reconstructed via `Object::from_raw` — but a `napi_value` is a local handle
  tied to the HandleScope active at construction time, and by the time visitor methods fired deep
  inside `convert()` the scope was no longer active, so `get_named_property("visitText")` silently
  returned `Err` and the bridge fell through to `VisitResult::Continue` without dispatching. A
  `Drop` impl on the bridge calls `obj_ref.unref(&env)` so the JS object can be GC'd after
  conversion. Verified against the issue repro: `visitText`/`visitLink`/`visitHeading` all fire.
  (alef commit `1ffdaafe4`)

- **code block: `CodeBlockStyle::Backticks` and `Tildes` no longer emit a trailing blank
  line inside the fence and now insert a blank line after the closing fence** (resolves #396).
  The fenced emitter pushed `content` verbatim (which already ends in `\n` for any trailing-newline
  source) plus an extra `\n` before the closing fence, producing `…\n\n```\n`. It also closed with a
  single `\n`, so following block content butted up against the closing fence with no blank line
  separator — `Indented` style was unaffected because its content path strips trailing newlines
  via `.lines().join("\n")` and already emits `\n\n` after. The Backticks/Tildes path now trims
  trailing newlines from inner content (`content.trim_end_matches('\n')`) before re-emitting a
  single `\n`, and pushes `\n\n` after the closing fence to match Indented.

- **php test_app: PIE `install.sh` now installs `xberg-io/html-to-markdown`** instead of
  the non-existent `xberg-io/html-to-markdown-rs` (resolves #98 / smoke regression). The
  Packagist project only publishes the un-suffixed name; `alef.toml` was renamed and the regen
  picks it up in both `install.sh` and the inline comment.

- **r binding: `conversion_options()` helper exported in `NAMESPACE`.** alef's R emitter
  generated the `R/options.R` helper for ergonomic ConversionOptions construction but never
  exported it, so `htmltomarkdown::conversion_options(...)` raised `could not find function`
  at runtime — which downstream `convert(..., options=)` callers then surfaced as the
  extendr-api 0.9.0 `options must be a named list` validation error. Resolves #99 / smoke
  regression. (alef commit `160d504f3`)

- **java binding: `NativeLib.<clinit>` no longer throws `NoSuchElementException`** when an
  optional FFI symbol is missing. The error-context handle lookups (`LAST_ERROR_CODE`,
  `LAST_ERROR_CONTEXT`) used `Optional.orElseThrow()` mid-static-init, escalating any partial
  symbol set into a hard `NoClassDefFoundError`; they now fall back to `null` and the rest of
  the loader's null-checks handle the absence gracefully. Resolves #100 / smoke regression.
  (alef commit `d296bfdeb`)

- **c FFI test_app `Makefile` now embeds an `LC_RPATH`** so the smoke binary can find the
  dylib at runtime on macOS without `DYLD_LIBRARY_PATH`. Adds `-Wl,-rpath,$(FFI_LIB_DIR)`
  to the LDFLAGS path. Resolves #101 / smoke regression.

- **csharp binding: trait-bridge catch blocks no longer emit unused `Exception ex` variables.**
  The generated `TraitBridges.cs` had 25+ `catch (Exception ex)` blocks where `ex` was never
  referenced; with `<TreatWarningsAsErrors>true</TreatWarningsAsErrors>` the build failed
  with CS0168, which in turn blocked NuGet publish for 3 releases (root cause of #104). alef
  now emits bare `catch (Exception)` when the exception variable isn't actually used.
  (alef commit `16e81b20d`)

### CI/Publish

- **`.npmrc` at workspace root: `minimum-release-age=0`** so the npm `Build Node bindings` and
  `Build WASM package` jobs stop failing at `pnpm install` time with
  `ERR_PNPM_MINIMUM_RELEASE_AGE_VIOLATION` for transitively-recent dep pins. The v3.5.5 fix
  only patched the test command; this covers every `pnpm install` call repo-wide. Resolves
  #96.

- **Go subtag publish** (`packages/go/v3.5.x`) is already wired through the
  `xberg-io/actions` `finalize-release` job (publish.yaml:2418 passes
  `go-module-path: packages/go/v3`); no action needed here. The reason v3.5.2–v3.5.7 didn't
  push subtags is upstream main-release-job failures cascading. Resolves #97.

### Chore

- **alef pin: 0.20.12 → 0.21.0** (local unreleased head; no alef tag). Bindings + e2e suites
  regenerated against the freshly-rebuilt local alef binary, picking up the cohort fixes
  above in a single regen.

## [3.5.7] - 2026-05-29

### Fixed

- **release: sync workspace `Cargo.toml` version.** v3.5.6's release commit (`2c37942f8`) claimed to bump the workspace version 3.5.5 → 3.5.6 but the change never actually landed in the commit. Every binding manifest (Cargo.toml, pyproject.toml, gemspec, mix.exs, pom.xml, .csproj, package.json, …) stayed pinned at 3.5.5, so the v3.5.6 publish workflow built v3.5.5 binaries against the v3.5.6 tag and uploaded them as `*-v3.5.5-*.tar.gz` assets to the v3.5.6 GitHub Release. crates.io / PyPI / Hex / RubyGems / Maven publishes were either no-ops (3.5.5 already exists) or rejected; npm + WASM + NuGet + Homebrew steps also failed. v3.5.6 is skipped; v3.5.7 is the corrected ship of every fix originally rolled into v3.5.6.
- **alef pin: 0.20.11 → 0.20.12** (auto-synced by `alef sync-versions`).

### Chore

- **bindings: regenerated against alef v0.20.12** with workspace version correctly bumped to 3.5.7 first, so every manifest now matches the release tag.

## [3.5.5] - 2026-05-28

### Fixed

- **ci(ruby): cross-compile `windows-x64` gem via `bundle install` inside the `rb-sys-dock` container.** The previous matrix entry ran `rb-sys-dock --platform ... -- bundle exec rake "native[$target]" gem`, which invoked `bundle exec` inside the container without first materialising the lockfile against the container's Ruby version. The rb-sys-dock container ships its own Ruby toolchain (4.0.2 preview) while the host `Gemfile.lock` pins 3.3.11, so bundler failed with "Could not find …" and hung for 47 minutes. The command now wraps the container invocation in `bash -c "bundle install --jobs=4 --retry=3 && bundle exec rake …"` so the lockfile materialises against the container's Ruby before `rake` runs. (`.github/workflows/publish.yaml`)
- **ci(elixir): Hex publish no longer 404s on darwin tarball downloads.** alef v0.20.2 changed darwin NIF tarballs to `.dylib.tar.gz` to "match the platform-native extension," but `rustler_precompiled 0.9.0` (the latest version on Hex; no `.dylib`-aware version exists) hardcodes `.so` for every non-Windows consumer download URL in `lib_name_with_ext/2` and ignores all caller overrides. h2m's hand-maintained publish.yaml had already normalised darwin uploads to `.so`, so h2m's own Hex publish kept working; the bug only surfaced in sibling polyglot repos. The alef v0.20.5 revert (`.dylib` → `.so`) restores polyrepo alignment and is picked up by this regen. (`alef.toml` pin → `v0.20.5`, `xberg-io/actions/generate-elixir-checksums@v1`)
- **bindings(c): registry test_app `download_ffi.sh` now hits the correct release asset prefix.** The `[crates.e2e.registry.packages.c] name` was `html-to-markdown-ffi`, but `publish.yaml` uploads C FFI tarballs with `asset-prefix: html-to-markdown-rs-ffi-`. Drift caused `task test-apps:smoke:c` to 404 on the GH release tarball. (`alef.toml`)
- **bindings(java): registry test_app now ships the Maven wrapper (`mvnw`, `mvnw.cmd`, `.mvn/wrapper/maven-wrapper.properties`).** Previously missing because mvnw emission landed in alef v0.20.2 and h2m was pinned at v0.20.1. (`alef.toml` pin → `v0.20.5`)
- **bindings(homebrew, c, …): every alef-emitted `*.sh` (e.g. `run_tests.sh`, `download_ffi.sh`) is now created with the `+x` bit set.** alef's `write_scaffold_files_with_overwrite()` skipped the shebang-chmod helper that `write_files()` applied, so every alef-generated shell script in `e2e/` and `test_apps/` landed as `-rw-r--r--`, breaking `task test-apps:smoke:homebrew` with "permission denied" on `./run_tests.sh`. Fixed by alef v0.20.3's shared `apply_shebang_chmod()` helper called from both writers. (`alef.toml` pin → `v0.20.5`)
- **bindings(go, php, swift, csharp, dart, kotlin_android, zig, …): trait-bridge codegen cohort fixes.** PHP refcount safety (inc_count/dec_count via PhpRc), Go duplicate //export removal + cgo.Handle.Delete defer/recover, Swift async/throws + try await order + JSON-encode conditionally, C# callback method naming + bool→int + IntPtr userData + usize/isize mapping, Dart trait-import + factory wrapper required-only methods + RID-aware published-loader, Kotlin Android useJUnitPlatform() in registry mode, Zig error-union removal from test-backend stubs, Java trait-method emission incl. default-impl methods + `ffi_skip_methods` honouring in `I{Trait}` interface emission. (`alef.toml` pin → `v0.20.5`)
- **bindings(elixir): rustler upgraded 0.37 → 0.38.** (`packages/elixir/native/html_to_markdown_nif/Cargo.toml`)
- **bindings(php): PHP trait-bridge async no longer panics on "Cannot start a runtime from within a runtime."** Generated async method bodies now prefer `Handle::try_current()` before `WORKER_RUNTIME.block_on(...)` so the bridge is safe to call from within an outer tokio runtime. (`alef.toml` pin → `v0.20.5`)
- **bindings(wasm, node): test_apps pass `--config.minimumReleaseAge=0` to `pnpm test` so freshly-published RC packages clear pnpm's supply-chain policy check.** (`alef.toml` pin → `v0.20.5`)

### Chore

- **bindings: regenerated against alef v0.20.5.** Folds in every fix from v0.20.2 through v0.20.5 — see alef CHANGELOG for the full backend cohort.
- **deps(java): jackson-databind + jackson-datatype-jdk8 2.21.2 → 2.21.3.** (`packages/java/pom.xml`)
- **deps(rust): reqwest 0.13.3 → 0.13.4 in CLI binary.** (`crates/html-to-markdown-cli/Cargo.toml`)
- **deps(js): pnpm packageManager 11.3.0 → 11.4.0.** (`package.json`)
- **taskfile(elixir): tolerate `mix hex.outdated` non-zero exits before `mix deps.update --all`.** (`.task/languages/elixir.yml`)

## [3.5.4] - 2026-05-28

### Fixed

- **ci(node): restore alef-generated `index.d.ts` after `napi build` in the `node-typescript-defs` publish job.** The napi-rs macros deliberately filter out the `*Update` DTO types (`ConversionOptionsUpdate`, `PreprocessingOptionsUpdate`), so the d.ts regenerated by `napi build` lacks them; the wrapper at `packages/typescript/src/index.ts` re-exports those names and `tsc` fails with TS2724 on every publish. The publish job now `git checkout HEAD -- crates/html-to-markdown-node/index.d.ts` between `napi build` and the artifact copy, preserving alef's augmented d.ts.
- **ci(ruby): cross-compile `windows-x64` gem via `rake-compiler-dock` from the Linux runner.** The previous matrix entry's `cargo build --target x86_64-pc-windows-gnu` against the ucrt64 MSYS2 toolchain corrupted a linker argument and failed every release with `cannot find -l■`. Linux-host docker-based `rake-compiler-dock` is rb-sys's recommended cross-build path and matches what every other rb-sys gem uses for Windows.
- **bindings(elixir): `RustlerPrecompiled` now resolves the correct platform extension on macOS.** v0.20.0's rustler emit lets RustlerPrecompiled's built-in mapping pick `.dylib` for `*-apple-darwin` targets and `.so` for `*-linux-*`, so `mix deps.get` no longer 404s against `*.so.tar.gz` on darwin consumers.
- **alef hash: per-file `alef:hash:` now computed over generation inputs, not emitted content.** Post-format whitespace drift no longer invalidates `alef verify`, so CI Lint stops reporting bindings as stale after every prek `--all-files` cycle.
- **bindings(java): test_app pom.xml drops `-Djava.library.path=.../target/release` in registry `dep_mode`.** The Maven-Central JAR bundles natives under `/natives/{rid}/` and alef's loader extracts them at startup; overriding the library path to a non-existent local directory broke `UnsatisfiedLinkError` on smoke runs. Override is now emitted only in local `dep_mode`.
- **bindings(c): test_app Makefile no longer requires a local `target/release/` build to resolve the FFI.** `download_ffi.sh` is skipped only when both the FFI header and shared library are present locally; otherwise the published tarball is fetched. Smoke runs from a clean checkout work without a prior `cargo build`.
- **bindings(php): registry test_app installs the PIE-distributed extension via `install.sh` before `composer install`.** Removes the implicit "ext-html_to_markdown must be pre-installed" requirement.

### Chore

- **bindings: regenerated against alef v0.20.1.** Folds in B6 hash semantics, java/c e2e build-path fixes, php install.sh, vendored Cargo workspace marker, scaffold persistence (rustc job cap + Node memory cap), wasm e2e import dedupe, service_api emitter family for csharp/dart/go/swift/zig/jni/ffi/extendr/napi/magnus/php/rustler/pyo3 with real dispatch (not yet wired in h2m but available for downstream features), zig platform-suffixed URLs in build.zig.zon, kotlin nullable bare result assertions, and php anonymous-class super_trait `name()` dedup.

## [3.5.3] - 2026-05-27

### Fixed

- **core: prevent silent truncation on XHTML-style self-closing tags (`<td/>`, `<br/>`, etc.) embedded in EPUB-derived HTML.** The bundled astral-tl parser treats `/` as an identifier character, so `<td/>` was parsed as a tag literally named `"td/"` and subsequent siblings nested under it — table rows collapsed into a single cell and any content after the broken table was dropped. `convert_api::normalize_input` now preprocesses `<tag/>` → `<tag />` before tokenization so the trailing slash is read as a self-closing marker. Regression covered by `tests/issue_391_xhtml_self_closing.rs`. Closes #391.
- **docs(python): visitor API reference rewritten to match the actual duck-typed binding.** The generated `docs/reference/api-python.md` documented a `class MyVisitor(VisitorHandle):` subclass pattern with `VisitResult.Continue` enum returns, but `VisitorHandle` is `#[pyclass(unsendable, from_py_object)]` (sealed; cannot be subclassed) and `VisitResult` exposes only property getters (`VisitResult.Continue` raises `AttributeError`). The trait rustdoc on `HtmlVisitor` now leads with the plain-class + string/dict return idiom that the magnus-style bridge actually accepts. Closes #389.
- **packaging(python): unblock PyPI sdist build from Alpine/musl and other no-precompiled-wheel platforms.** PyPI 3.5.1's sdist was missing the core `crates/html-to-markdown/` directory because a stale `.gitmodules` entry pointing at a deleted `homebrew-tap` submodule caused `cargo package --list` to fail silently during the CI maturin run, so maturin only bundled the PyO3 crate and shipped a sdist that could not build. Removed the stale `.gitmodules`; maturin's path-dependency auto-bundling now works again and the resulting sdist contains all 177 core-crate files. Closes #390.
- **ci(swift): Swift Artifact Bundle publish job uses the renamed crate `html-to-markdown-rs-swift`.** Stale `html-to-markdown-swift` reference broke the Swift bundle step on every release since v3.5.0.

### Chore

- **bindings: regenerated against alef v0.19.22.** Pulls in the e2e green-up cohort (csharp/php/extendr/rustler/swift/zig/dart/go/java/ruby/elixir trait-bridge emitter fixes) and drops alef's broken `[tool.maturin] include` directive (maturin rejects archive paths with `..`).

## [3.5.2] - 2026-05-26

### Fixed

- **bindings(csharp): trait-bridge facade methods now throw the per-binding `HtmlToMarkdownRsException` instead of an undefined `KreuzbergException`.** The alef csharp backend's register/unregister facade method emitter was hardcoded to `KreuzbergException` (the kreuzberg core lib's exception class), causing every C# build to fail with `CS0246: KreuzbergException could not be found`. Fixed in alef 0.19.13 and pulled in by this regen.

- **bindings(swift): `RustBridgeC.h` placeholder now declares the `RustStr` C struct that `SwiftBridgeCore.swift` depends on.** Without the typedef the Swift compiler reported `cannot find type 'RustStr' in scope` for every `extension RustStr` block before the full `cargo build` populated the real header. Fixed in alef 0.19.13.

- **bindings(swift): resolve Swift Package Manager "unsafe build flags" rejection for v3.5.2+.** Swift 6.0+ strictly rejects packages with `unsafeFlags` in public products to prevent supply-chain code injection. The root `Package.swift` used by external consumers (via `.package(url: "...", from: "...")`) is now removed from git; it will be regenerated at release time with `.binaryTarget` pointing to pre-built xcframework/artifactbundle assets. For v3.5.2+, consumers will receive binary distribution (no Rust compilation required). For source builds, developers use `cd packages/swift && swift build` after `cargo build -p html-to-markdown-rs-swift`. The in-tree `packages/swift/Package.swift` retains `unsafeFlags` for local development. This fixes the blocking error for v3.5.1 external consumers attempting to depend on the package.

## [3.5.1] - 2026-05-25

### Fixed

- **bindings(ruby): expose `ConversionOptions.visitor` and wire the magnus visitor bridge end-to-end.** The Ruby gem previously dropped `VisitorHandle` from codegen via `[crates.ruby] exclude_types`, so the documented `HtmlToMarkdown.convert(html, MyVisitor.new)` example silently ignored the visitor and emitted default markdown. Removed the exclusion — alef's magnus backend already implements the full visitor trait bridge, so the regen produces a working `RbHtmlVisitorBridge` that dispatches each `visit_*` callback via `respond_to?` + `funcall` and translates Ruby return values (`:continue` / `:skip` / `:preserve_html` / `{custom: "..."}`) into `VisitResult` variants. Closes #388.

- **bindings(elixir): expose `ConversionOptions.visitor` and wire the rustler visitor bridge end-to-end.** Same fix as Ruby: removed `VisitorHandle` from `[crates.elixir] exclude_types`. The alef rustler backend already ships the bridge — a system thread runs the conversion, sends `{:visitor_callback, ref_id, callback_name, args_json}` to the caller, and blocks on `HtmlToMarkdown.Native.visitor_reply/2` until the receive loop in `HtmlToMarkdown.convert/2` dispatches the user's visitor map and replies. Visitor maps are keyed by callback name (`"visit_link"`, `"visit_text"`, …) with one-arity function values.

- **list**: nested-list duplication in Markdown output and the document structure collector (PR #385, fixes [kreuzberg#1004](https://github.com/xberg-io/kreuzberg/issues/1004)). `push_list_item` was previously called with `output[item_start_pos..]`, which included the rendered Markdown of nested `<ul>`/`<ol>` children. Inner items' text was triple-counted: once as the item itself, once inside the parent item's text, and once as free text from the walker. A `text_end_pos` cursor now advances only past non-list children. Affects any `ul > li > ul` (or arbitrarily nested) shape.

- **alef.toml**: dropped stale Ruby/Elixir `vendor_mode = "core-only"` overrides — alef now defaults source-build languages to `vendor_mode = "registry"` (the migrated install-from-crates.io flow), and the shared `build-ruby-gem` / `build-elixir-hex` actions enforce the same rewrite at workflow time. Overriding to `core-only` would silently vendor the core crate into `packages/{ruby,elixir}/vendor/` and leave the manifest pointing at the workspace path, which fails to resolve on consumer machines.

- **crates/html-to-markdown-py/src/pyproject.toml**: maturin `manifest-path` corrected to `../Cargo.toml` (was `../../crates/html-to-markdown-py/Cargo.toml`, which resolved to `crates/crates/…` from the pyproject's own directory). Local `uv sync --upgrade` / `task upgrade` now completes; the publish workflow builds via `maturin build` with explicit paths and was not affected.

- **packages/swift/rust/src/lib.rs**: regenerated against alef 0.19.7 with the swift-bridge inbound trait phantom fix. The previous alef emitted `Vec<Swift{Trait}BoxBox>` in the `extern "Rust"` block (double-`Box` suffix), which swift-bridge-build rejected with "Type must be declared with `type >`". The inbound phantom is now omitted entirely — `Swift{Trait}Box` is an `extern "Swift" type` with no Rust-side struct backing it, and the Vec accessors are not consumed by the bindings we emit.

- **.task/languages/kotlin_android.yml** + **alef.toml `[crates.update.kotlin_android]`**: the `com.github.ben-manes.versions` Gradle plugin (current pin 0.52.0, latest 0.53.0) throws `java.util.ConcurrentModificationException` under Gradle 9.5.1 on every `:dependencyUpdates` invocation; the plugin has not shipped a release since 2024-11. Both `alef update --latest` and `task kotlin_android:upgrade` previously failed the rest of the upgrade chain. Inert-echo the probe in both places until upstream lands a Gradle 9 fix or we swap to `nl.littlerobots.version-catalog-update`.

### Added

- **`alef test-apps generate` first-class subcommand**, bundled into `alef all` as a discrete pipeline stage so registry-mode test_apps regenerate alongside the local-mode e2e suite. Per-stage stale-file sweep prevents either output dir (`e2e/` vs `test_apps/`) from deleting the other's files (the bug that wiped half of `test_apps/` when `alef all` ran in the previous topology).

- **Two new test_app channels** emitted by the new subcommand: `test_apps/homebrew/` (Brewfile + `run_tests.sh` + `ffi_smoke.c` — exercises both Homebrew formulae via `brew bundle install` and a pkg-config-linked C smoke test) and `test_apps/php_ext/` (PIE-installed native ext driver — `pie install xberg-io/html-to-markdown-ext` then `extension_loaded` + convert smoke).

- **alef-generated `build.gradle.kts` for `test_apps/kotlin_android/`** — registry mode consumes the published `dev.kreuzberg:html-to-markdown-android:3.5.x` Maven artifact instead of the local workspace AAR.

- **Taskfile**: `e2e:smoke:homebrew` and `e2e:smoke:php_ext` entries wired into the `e2e:smoke:all` aggregator.

- **`.gitattributes`**: marks `test_apps/**` as `linguist-generated=true` (via the alef-scaffold change that now reads `[e2e.registry].output`).

### Docs

- **`docs/snippets/ruby/visitor/basic_visitor.md`** rewritten against the magnus-real API: visitor is a positional second argument to `HtmlToMarkdown.convert`, callbacks dispatch via `respond_to?` + `funcall`, return values are `:continue` / `:skip` / `:preserve_html` / `{ custom: "..." }`, the `ctx` argument is a Hash with `:node_type` / `:tag_name` / `:depth` / etc. The previous snippet used a `visitor: MyVisitor.new` kwarg form and `result[:content]` accessor that don't match the gem's actual surface.

- **`docs/snippets/elixir/visitor/basic_visitor.md`** rewritten against the rustler-real API: visitor is a `%{"visit_*" => fn args -> ... end}` map under the `:visitor` key of the options Hash; the bridge sends `{:visitor_callback, ref_id, callback_name, args_json}` messages and blocks on `visitor_reply/2`. The previous snippet used an aspirational `use HtmlToMarkdown.Visitor` behaviour form that alef does not generate.

### CI

- **`publish-swift` job removed** from `.github/workflows/publish.yaml`. Swift Package Index has no central registry — packages are consumed directly from git tags, so the tag push IS the publish event. The previous `check-spi-swift` + `publish-swift` jobs only pinged SPI for fast re-indexing (SPI auto-discovers tags within ~1h without a ping). Cuts runner cost; SPI catches up automatically.

- **`publish-zig` retained** — appends the tarball URL + SHA-256 to the GitHub Release notes via `update-release-notes: true`, which downstream consumers copy directly into `build.zig.zon`.

- **`actions/publish-hex` bumped to v1.6.9**: generates `Cargo.lock` for every `native/**/Cargo.toml` before `mix hex.publish`. The lockfile is gitignored, so the publish action's fresh checkout had nothing to publish; this mirrors the `build-elixir-hex` fix from earlier in the v3.5.0 cycle.

- **`task upgrade` works end-to-end again** after the Python and Kotlin Android fixes above.

## [3.5.0] - 2026-05-25

### Fixed

- **bindings: regenerated with alef 0.19.6.** Node optional-dep package names now carry the `@kreuzberg/` scope (`@kreuzberg/html-to-markdown-node-<target>`) so `requireOptionalDependency()` resolves the published per-platform packages instead of an unscoped name that does not exist on npm. `test_apps/` restructured by alef to the new layout (per-language runners under `test_apps/<lang>/{ffi,htm_test,run_tests}` for C; legacy in-tree test files removed). Additional alef-emitter, swift-bridge, FFI param handling, and ahash-scaffold fixes carried through from the 0.19.x line.

- **ci(publish-hex): bump to xberg-io/actions@v1.6.9 — generate `Cargo.lock` before `mix hex.publish`.** `publish-hex@v1` runs a fresh `actions/checkout`, so the gitignored `packages/elixir/native/html_to_markdown_nif/Cargo.lock` is absent and `mix hex.publish` fails with `Missing files: native/html_to_markdown_nif/Cargo.lock`. v1.6.9 mirrors the `build-elixir-hex` fix and runs `cargo generate-lockfile` for every `native/**/Cargo.toml` before publishing. (`v1` floating tag retagged.)

- **ci(publish): rename Go FFI tarballs to use `-go-` infix instead of `-ffi-`.** alef 0.19.6's Go packager (`alef publish package --lang go`) emitted `{crate}-ffi-v{version}-{platform}.tar.gz`, colliding with the C FFI packager's prefix. `check-registry` asset-prefix probes and `verify-release-assets` pattern lists could not distinguish Go from C FFI tarballs, so `verify-release-assets` failed on the missing `html-to-markdown-rs-go-*.tar.gz` pattern. Workaround: rename `-ffi-v` → `-go-v` immediately after `alef publish package --lang go` in the workflow. The alef-side fix is already on alef main; the workaround can be dropped once the local alef pin moves to a release that ships it.

- **ci(actions/build-elixir-hex): generate `Cargo.lock` unconditionally after `rewrite-native-deps`.** The fallback `cargo generate-lockfile` only ran on dry-run, but the lockfile is gitignored — real-release runs hit `Missing files: native/html_to_markdown_nif/Cargo.lock` at `mix hex.build`. Now runs in both modes. (xberg-io/actions v1.6.6, floating `v1` retagged.)

- **ci(publish): split Dart pub.dev publishing into a workflow_dispatch flow so OIDC trusted publishing succeeds.** pub.dev's OIDC verifier rejects tokens minted by `release` events (`Authentication failed!`) and only accepts `push` / `workflow_dispatch`. The publish workflow now assembles the Dart package as an artifact under the `release`-triggered run, then dispatches a separate `publish-pubdev.yaml` workflow (`workflow_dispatch`) that downloads the artifact and runs `dart-lang/publish-pub@v1`. The dispatched job inherits a token GitHub's OIDC provider mints with `event_name == workflow_dispatch`, which pub.dev's audit accepts.

- **ci(actions/homebrew-build-bottles): suppress `brew config` SIGPIPE under `pipefail` on arm64 Linux runners.** The arm64 runner's `/usr/bin/ldd` writes more output than `head -20` consumes; under `set -o pipefail` the broken-pipe propagated out of the early diagnostic block and aborted the script before any bottle work. Temporarily disables `pipefail` for the diagnostic stanza only — strict mode is restored before the build phase. (xberg-io/actions v1.6.7, floating `v1` retagged.)

- **bindings: regenerated with alef 0.19.5.** Picks up the Kotlin Android trait-bridge codegen fixes that caused the v3.5.0-rc.2 `:compileReleaseKotlin` failure (`Unresolved reference 'HtmlToMarkdownRsBridge'`): `bridge_obj` filename is now used for trait-bridge codegen so the file matches the object name; trait-bridge emission is skipped when the bridge function is excluded via `kotlin_android.exclude_functions`. Also picks up the alef 0.19.5 cumulative sweep: WASM emitter JSON-deserializes structured sub-config fields; swift-bridge restores JSON deserialization step in pre-call AHashMap binding; alef-emitter removes stray `>` after `.collect::<Vec<Vec<String>>>()`; PHP/Ruby/Elixir/Swift/Dart bridges emit pre-call `AHashMap` binding + `ahash = "0.8"` scaffold dep for `Cow<'static, str>` key map params; WASM wraps sanitized `Vec<Vec<String>>` fields with `serde_wasm_bindgen::to_value()`; WASM deduplicates input DTO struct generation across functions sharing the same config type; FFI preserves `AHashMap<Cow<'static, str>, _>` param types across the wrapper boundary; setup-defaults-ruby appends `--add-checksums` to default `bundle install`; scaffold-ffi injects workspace version into every internal workspace dependency so `cargo publish` accepts the FFI crate.

- **ci(publish): replace inline Homebrew formula updater with `xberg-io/actions/publish-homebrew-source-formulas@v1`.** The `publish-homebrew-formula` job previously ran a 184-line `scripts/publish/update-homebrew-formula.sh` Bash heredoc that wrote `html-to-markdown.rb` + `libhtml-to-markdown.rb` from scratch (h2m is a dual-formula tap; the shared single-formula `publish-homebrew@v1` doesn't apply). The new shared action does the same job from per-formula `.rb.tmpl` templates + a `scripts/publish/homebrew.json` manifest, downloading release assets via `gh release download` and substituting their SHA256s into `${cli_*_sha}` / `${ffi_*_sha}` placeholders. The script is deleted; the formula generation rules now live in version-controlled Ruby templates rather than a bash heredoc. The job's gate now also passes on `dry_run == 'true'` (was `is_tag == 'true'` only) so dry-run pipelines exercise the bottle pipeline downstream — the new action substitutes a zero-SHA placeholder for missing assets on dry-run.

- **list**: Fix content duplication in Markdown output and the document structure collector when list items contain nested `ul` or `ol` children. `item.rs` previously captured the full rendered output of an `<li>` — including the rendered nested-list Markdown — as the item's text. A `text_end_pos` cursor now advances only past non-list children, so the structure collector records only the item's own text and the Markdown output for outer and mid items is not repeated. Affected: any `ul > li > ul` or `ol > li > ol` (arbitrarily nested) HTML structure. (#385)

- **ci(publish): skip `publish-hex` and `homebrew-bottles` on dry-run.** Both jobs need real GitHub Release assets (`generate-elixir-checksums` downloads NIF tarballs; `brew install --build-bottle` downloads CLI/FFI source tarballs), but `upload-release-assets@v1` only logs on dry-run — the release doesn't exist. Both jobs failed every dry-run after surviving every other stage. Gated their `if:` on `dry_run != 'true'`; real-release runs continue to exercise them.

- **ci(publish): replace inline Elixir Hex packaging with `xberg-io/actions/build-elixir-hex@v1`.** The `elixir-package` job in `.github/workflows/publish.yaml` previously ran `mix deps.get` + `mix hex.build` inline with no path-dep rewrite and no `Cargo.lock` generation. `Cargo.lock` is gitignored, so on a fresh CI checkout `mix hex.build` failed at the `files` list check with `Missing files: native/html_to_markdown_nif/Cargo.lock` — the proximate blocker on v3.5.0-rc.2 dry-run after the Go FFI fix landed. The new shared action wraps `rewrite-native-deps@v1` (default-on, dry-run-guarded) and falls back to `cargo generate-lockfile` on dry-run so the lockfile exists for `mix hex.build` even when the rewrite is skipped. Hex source-package builds now match the python-sdist / ruby-gem pattern (rewrite baked into the shared action; cannot be omitted).

- **ci(publish): build the Go FFI matrix on dry-runs too.** `.github/workflows/publish.yaml` job `go-ffi-libraries` gated only on `release_go == 'true'` and the registry existence check, so on `workflow_dispatch` dry-runs it skipped entirely. Its downstream sibling `upload-go-release` already gates on `is_tag || dry_run` but waits for `go-ffi-libraries.result == 'success'`, so no Go assets were ever uploaded to the dry-run release, and the terminal `verify-assets` gate failed with `✗ pattern NOT matched: html-to-markdown-rs-go-*.tar.gz` on every recent retry (the immediate blocker on v3.5.0-rc.1 publish). Added the `(is_tag == 'true' || dry_run == 'true')` clause to `go-ffi-libraries`'s `if:`, mirroring the `kotlin-android-natives` pattern (precedent: commit `e00a56e1` "ci(publish): build Kotlin Android natives on dry_run too").

- **ci(e2e): pin `erlef/setup-beam@v1.24.0`** (was `@v1.24` floating minor) to avoid silent action upgrades during the rc cycle.

- **ci(e2e/ruby): drop the explicit `python3 scripts/ci/ruby/vendor-core-crate.py` step from both ruby build jobs in `.github/workflows/ci-e2e.yaml` (committed earlier today as `f23e6d458`); the dead script itself is removed in `f440af4fa`.** The shared `xberg-io/actions/build-ruby-gem@v1` action now invokes `rewrite-native-deps@v1` internally and vendors the core crate into `packages/ruby/vendor/html-to-markdown/` (no `-rs` suffix). Running the local script first wrote `packages/ruby/vendor/Cargo.toml` with `members = ["html-to-markdown-rs"]` and copied the crate into `vendor/html-to-markdown-rs/`; the subsequent action then created a sibling `vendor/html-to-markdown/` outside that members list, and cargo refused to build it with `error inheriting 'lints' from workspace root manifest's 'workspace.lints'` / `'workspace.lints' was not defined`. The action is now the single source of truth for ruby vendoring.

### Changed

- **ci(publish): pin `actions/checkout@v5` on the `homebrew-bottles` matrix job.** v6 hits an includeIf credential regression on the `macos-15-intel` runner that this matrix includes (same workaround liter-llm applies on its Homebrew matrix). Other jobs stay on `@v6`.

- **release: cut v3.5.0.** Promoted from v3.5.0-rc.3 after the dry-run/republish cycle (publish run 26393110006) reached fully green (31 success / 0 failed / 56 skipped). Aligned every workspace manifest on `3.5.0` via `alef sync-versions --set 3.5.0` + full `alef generate` regen.

- **release: cut v3.5.0-rc.1 release candidate.** Aligned every workspace manifest (Cargo + npm + PyPI + Maven + Composer + Gemfile + Hex + pub.dev + Zig + R + Cocoa + nuget + Cargo.lock entries) on `3.5.0-rc.1` via `alef sync-versions --set`. Refreshed every per-language dependency tree to its current upstream pin (`alef update --latest`) and re-generated all bindings, READMEs, docs reference pages, and e2e suites against alef 0.18.1 so the regen artefacts on disk match the version pin baked into every binding manifest.

- **bindings: regenerated with alef 0.19.2.** Picks up the cumulative v0.19.1 → v0.19.2 sweep: Swift codegen handles `serde_rename_all = "lowercase"` / `"UPPERCASE"` in unit enums and tagged Codable shapes (was silently emitting wrong casing); Swift now emits a custom Codable for serde-untagged data enums (the auto-derive previously used the wrong shape, surfacing as 19 runtime e2e failures); tagged-data enums route through `JSONDecoder` and `Vec<Codable-enum>` closure signatures are fixed. The trait-bridge codegen pipeline is rewired across all 14 language backends (rust/python/ts/node/wasm/ruby/php/go/c#/r/zig/elixir/dart/swift/java/kotlin-android) — super-trait lifecycle methods (`name` / `version` / `initialize` / `shutdown`) are driven by the IR instead of hardcoded literals, and canonical bridge-name helpers in Swift/Kotlin de-duplicate the prior near-misses. JNI no longer special-cases trait impl-name extraction; Kotlin Android accepts a `bridge_class_name` parameter so non-kreuzberg consumers (e.g. liter-llm) can opt out of the hardcoded `KreuzbergBridge` / `kreuzberg::` references. Java's `package_dir` output_paths override now excludes setup/test/lint runs from `.../src/main/java/` so they execute from `packages/java/` as expected; JNI 0.22 compatibility was tightened (`RuntimeMethodSignature` parsing, borrow semantics, unsafe-block scoping, `JString` lifetime binding). Inherited from the v0.19.1 fix landed earlier today: magnus (Ruby) and rustler (Elixir) `NodeContent::MetadataBlock` reverse conversion now reconstructs `Vec<(String, String)>` from the sanitized `Vec<Vec<String>>` binding shape (the prior code didn't type-check and broke every Ruby build).

- **bindings: regenerated with alef 0.18.1.** Picks up the v0.18.0 → v0.18.1 sweep: Java Builder `#[serde(default)]` non-optional fields use boxed nullable types so omitted JSON keys survive Jackson round-trip; PHP enum-variant accessor paths skipped before field validation; Python pyproject TOML arrays normalised to canonical `pyproject-fmt` shape; the Windows e2e runner no longer clobbers the inherited `Path` (case-insensitive env-key collision on `std::process::Command::env`); Ruby scaffold emits a `Steepfile` that ignores `lib/<gem>/native.rb` so Steep stops tripping on the Sorbet sigs the magnus backend deliberately emits; `alef readme` markdown normalisation matches `rumdl-fmt` MD012 (one blank max) so cold-regen READMEs no longer diverge from the `alef all` output; rubocop double-quote / `%w[...]` defaults emitted directly; Kotlin Android `.gitkeep` is empty (matches `end-of-file-fixer`); WASM optionalised non-Duration fields preserve core `Default` via the if-let wrapper; `Box<str>` round-trips through binding `String` via the new `CoreWrapper::Box` classifier; bare `str` resolves to `TypeRef::String` instead of falling through `sanitize_type_ref`; Zig test preamble installs `SIG_IGN` on `SIGABRT` so C++ destructor `abort()`s don't break the test-runner IPC; Java sealed-interface tagged-enum field defaults emit `new EnumName.Variant()` (records can't be statically referenced); PHP binding structs with custom core `Default` suppress the auto `#[derive(Default)]` and emit a delegating `impl Default`; doc-test import paths use the published crate name `alef` (post workspace-collapse); kotlin-android snapshot updated for vanniktech 0.36 (`SourcesJar.Sources()`, no-arg `publishToMavenCentral()`); Java e2e resolves per-fixture mock URLs from system properties; Java `Vec<T>`/`Map<K,V>` marshaling uses `MAPPER.writerFor(constructCollectionType(...))` to preserve `@JsonTypeInfo` discriminators; PyO3 `_to_rust_*_config` aliases serde-renamed keys + final pyo3 calls use serde-renamed param names.

- **bindings: regenerated with alef 0.18.0.** Workspace collapse + 0.17.36 → 0.18.0 cumulative codegen sweep. Notable fixes consumed in this regen: PyO3 `replace_constructor_with_serde_rename` skips the trait-bridge options field (`visitor`) from `sorted_fields` to avoid emitting it twice on `has_default` types with cfg-gated bridge fields (h2m all-features `ConversionOptions::new` previously failed `rustc E0415: identifier 'visitor' is bound more than once`); PHP e2e codegen sources the JSON key rename strategy from the language-effective `serde_rename_all` (camelCase by default) rather than the Rust core type's (which is `None` for h2m's `ConversionOptions`) so `from_json` reads the keys correctly into the binding struct's `#[serde(rename_all = "camelCase")]` — fixes 32 / 7-error PHP e2e failures; `alef all` repopulates `current_gen_paths` from the e2e cache manifest on a cache hit so the orphan-cleanup pass does not delete every previously-generated e2e file (157 deletes observed on first warm run); `[crates.{node,wasm}.crate_dir]` per-language override lets h2m point alef at its actual `crates/html-to-markdown-{node,wasm}` Rust crate dirs (without the `-rs` infix that the default formula assumed).

- **package READMEs: regenerated with cold `alef readme`.** The 0.18.0 regen sweep used `alef all` whose hot-cache README pass omits the blank lines between consecutive `##` section headers that the standalone `alef readme` pass emits cold. CI's `Validate READMEs` step (cold `alef readme`) caught the divergence; this commits the cold output. Same cosmetic alef bug as the prior `dcd072a58` patch; tracked separately upstream.

- **bindings: regenerated with alef 0.17.36.** Picks up the v0.17.18→v0.17.36 cumulative codegen sweep, including: Rustler `from_json` NIF shims gated on types with NIF wrappers (fixes Elixir NIF compile errors for `ConversionOptionsUpdate`, `PreprocessingOptionsUpdate`, `NodeContext`); Kotlin Android `= PreprocessingOptions()` synthesized defaults for non-nullable nested struct fields with Rust `Default` (fixes Jackson `MissingKotlinParameterException` on partial-options JSON, 98 → 0 failures); Kotlin sealed-class `@field:JsonSerialize(as = …)`/`contentAs` annotations; Kotlin `@JsonIgnoreProperties(ignoreUnknown = true)` + nullable default for `#[serde(flatten)]` fields; JNI `crate_suffix = "-jni"` build fix; Zig test_apps build.zig references binding `module_name` (not registry `pkg_name`) for `root_source_file`; Zig local-path dependencies in registry-mode test_apps; Dart pubspec single-caret version constraint; wasm `package.json` filenames use underscores (`html_to_markdown_wasm.js`); plus inherited 0.17.18-0.17.23 fixes (Swift visitor `case continue`, C# enum converters + nullable options, PHP `with_visitor` wither for trait-bridge fields, PyO3 streaming wrapper type identity, Go unresolved-Named fallback, Java sealed-interface display helpers, Ruby array literals, Dart positional-vs-named heuristic, R `.alef_format_value` wrapping, WASM camelCase input DTOs). Includes C visitor test suite (208 → 262 tests). The v0.17.25→v0.17.27 increment additionally brings: Rustler opaque-type NIF resource wrappers and enum-variant-field type collection (fixes `cannot find type` errors for types reachable only through enum variants); Kotlin Android `LongMethod` added to the generated `@file:Suppress` list, trait-interface emission skipped (no `IDocumentExtractor`/`IRenderer` redeclaration), `Vec<u8>` return types mapped to `ByteArray`, and integer-like float literals normalized; Java `null` (not `""`) builder default for `Path` fields and streaming adapters skipped; PHP `#[php(name = …)]` on facade static methods; Swift `unwrap_or` instead of `unwrap_or_else` in serde fallbacks (clippy `-D warnings`); WASM stops emitting the broken `*Input` config DTO; and the extractor skips underscore-prefixed `pub fn`s (test-only helpers no longer leak into bindings or docs). The v0.17.28→v0.17.32 increment additionally brings: C FFI struct-field getters for nested struct and `Option<T>` fields now return a cloned, boxed value instead of `null` (`htm_html_metadata_document`, `htm_conversion_result_document`, `htm_conversion_result_metadata`); the Node `index.d.ts`/`index.js` are emitted by NAPI-RS at build time and no longer carry an alef generated-header; plus Swift visitor `VisitResult`, Java sealed-interface, and Ruby scaffold corrections. The v0.17.32→v0.17.34 increment additionally brings: a `LICENSE` file synced into every per-language package directory (pub.dev, RubyGems, and other registries require one); per-language READMEs that render only their own binding section instead of leaking every language's bullet; the PHP e2e harness no longer re-execs PHP with `-n` (PHPUnit keeps shared modules); the Ruby scaffold's `.rubocop.yml` excludes generated `lib/**/*.rb`; PHP streaming facade methods + snake_case adapter names; PyO3 keyword-escaping and signature defaults in serde-rename constructors; Java `clear_fn` `(int, String)` error constructor; and C/Zig/C# e2e codegen compile fixes. The v0.17.34→v0.17.35 increment additionally brings: PyO3 trait-bridge constructor signatures and dict-coercion exclude the synthetic bridge field (e.g. `visitor`) — fixes spurious `visitor=...` kwargs on generated Python constructors and stale dict round-trips; magnus RBS type aliases use lowercase identifiers (`json_value`) — invalid uppercase `JsonValue` was breaking `steep check`; ext-php-rs getters for `Option<NonOpaqueNamed>` explicitly `.map(Into::into)` instead of relying on the blanket `IntoZval` impl that did not unwrap the inner type; Dart e2e fixture codegen emits a default `ConversionOptions()` instance for absent required-positional options; Java `options_type` inherits from sibling language overrides; CLI `--version` syncs `packages/zig/build.zig.zon`; kotlin-android publishing switched to the vanniktech maven-publish plugin for Maven Central Portal. The v0.17.35→v0.17.36 increment additionally brings: the `/* serde(default) */` placeholder marker (used as a `required`-suppression flag for C#) is filtered before emission in `alef-codegen` and `alef-backend-java` — generated Java now contains `List.of()` / `false` / `0` instead of `/* serde(default) */`, and generated magnus Rust uses the type's actual zero value (`vec![]` / `false`); kotlin-android `.editorconfig` disables ktlint's `trailing-comma-on-call-site` and `-on-declaration-site` rules (ktfmt strips them, ktlint demands them — the two were fighting on `build.gradle.kts` after the vanniktech migration); brew unsupported-call `unsupported_in` configuration; csharp variant-struct payload type fix; alef-e2e elixir Rustler NifTaggedEnum tuple emission for tagged-enum array args; publish workflow orders `alef-scaffold` ahead of backends.

### Tests

- **Taskfile: add `dart:e2e`, `swift:e2e`, `zig:e2e`, `kotlin_android:e2e` convenience tasks** so all 16 language bindings have a uniform per-language local e2e entry point matching the existing `c:e2e` / `python:e2e` / `node:e2e` / etc. pattern. Each delegates to `alef test --e2e --lang <name>`; the previously-existing `e2e:test:<lang>` tasks (which run against the published-package `test_apps/` registry tree) stay alongside. Verified all 16 suites pass locally: c (262), csharp (262), dart (262), elixir, go, java, kotlin_android, node (262), php (262 — after the alef PHP rename fix), python (262), r (639), ruby (262), rust, swift (262), wasm (263), zig.

- **e2e/php: skip visitor fixtures in `fixtures/edge-cases/visitor_errors.json`** — five visitor fixtures (`visitor_custom_element_with_nesting`, `visitor_unknown_tag_preservation`, `visitor_deeply_nested_skip`, `visitor_element_start_skip_entire_subtree`, `visitor_element_end_modification`) now carry `"skip": { "languages": ["php"] }` matching the rest of the visitor fixtures already in `fixtures/visitor/*.json`. The PHP `ext-php-rs` binding does not yet support callable visitor handles (no `VisitorHandle::from_php_object()` method), so these tests were emitting unbuildable test code. Generated PHP tests dropped from 262 to 208; suite is fully green.

- **e2e/kotlin_android: build the JNI crate in the e2e before-hook** — the Kotlin/Android host-JVM e2e loads `libhtm_jni` (the `html-to-markdown-rs-jni` crate's `[lib] name = "htm_jni"` cdylib), not the C FFI dylib. The `[crates.test.kotlin_android]` before-hook in `alef.toml` was building `html-to-markdown-ffi`, so all 208 tests failed with `UnsatisfiedLinkError: no htm_jni in java.library.path`. It now builds `html-to-markdown-rs-jni`; the suite is fully green.

- **e2e/node: invoke the napi build via `pnpm run build` in the e2e before-hook** — the `[crates.test.node]` before-hook ran a bare `napi build`, which is not on `PATH` in CI (the `@napi-rs/cli` binary lives in `node_modules/.bin`). The before-hook now runs `pnpm install && pnpm run build` so napi resolves from the crate's own `package.json` `build` script, fixing `sh: 1: napi: not found`.

### Fixed

- **core: fix panic slicing output at a non-UTF-8 char boundary** — `paragraph.rs` captured `content_start_pos = output.len()` before appending a separator; a subsequent `output.pop()` in the whitespace-normalisation path of `handle_span` could shift the effective boundary one byte back, landing it mid-codepoint (e.g. inside U+25A0 ■). The structure-collector slice `output[content_start_pos..]` then panicked. Fixed by clamping with `floor_char_boundary` before slicing; the same clamp is now applied at the two analogous sites in `figure.rs`. Triggered by `include_document_structure = true` with a `<pre>` preceding block and a `<span>` whose first character is multibyte. (#380)

- **core: avoid stack overflow on documents with many unclosed list items** — preprocessing now applies the HTML5 implicit-close rule for `<li>`, `<dt>`, and `<dd>` before the `tl` parser sees the document, so a 15k-item changelog with bare `<li>` tags (e.g. `https://curl.se/changes.html`) is parsed as siblings instead of a 448-deep chain. Eliminates a process abort (`fatal runtime error: stack overflow`) that bypassed `Result::Err` and `std::panic::catch_unwind`. (#379)

### Documentation

- **install: Windows MSVC linker / PATH troubleshooting** — `docs/installation.md` and the per-package Python READMEs now document `pip install --only-binary=:all: html-to-markdown` and how to keep MSVC's `link.exe` ahead of GNU/Cygwin `link` on `PATH` when sdist fallback is unavoidable. (#378)
- **wasm: `NodeContent::MetadataBlock { entries }` round-trips correctly as `[[k,v],...]`** — the `entries: Vec<(String, String)>` field is now stored as `JsValue` in the wasm binding struct and serialized/deserialized via `serde_wasm_bindgen` instead of `Vec<String>`, preserving the nested-array wire format that serde produces for tuple vecs.

### Added

- **`compact_tables` option** — set `compact_tables: true` on `ConversionOptions` to emit GFM tables with no column padding. Cells are flushed to content width and separator rows use exactly `---` per column, producing token-efficient output for RAG / LLM pipelines. Default `false`; existing output is unchanged.

- **Kotlin Android binding** — `dev.kreuzberg:html-to-markdown-android` on Maven Central. Standalone Android library (AAR) with bundled `libhtml_to_markdown_ffi.so` for `arm64-v8a` and `x86_64` ABIs; minSdk 21, compileSdk 35. JVM Kotlin users continue to consume the existing Java package (`dev.kreuzberg:html-to-markdown`) directly — Kotlin/JVM treats Java classes as native and Panama FFM is unavailable on Android, which is why Android needs its own package.
- **Swift binding** — `HtmlToMarkdown` Swift Package on Swift Package Index. SPM-only (no CocoaPods); macOS 13+, iOS 16+; powered by `swift-bridge`.
- **Dart binding** — `h2m` on pub.dev. Built with `flutter_rust_bridge` 2.12; supports Flutter Android/iOS targets plus server Dart on Linux/macOS/Windows. Package name `h2m` because `html_to_markdown` and `html-to-markdown` are taken on pub.dev.
- **Zig binding** — published via GitHub Releases (`build.zig.zon` + tarball SHA-256 in release notes). Requires Zig 0.16+; links the existing `html_to_markdown_ffi` C library — no separate Rust bridge crate.
- **ffi**: `htm_visitor_handle_from_callbacks` exports a vtable-style visitor-handle constructor for zig and other C consumers. Wraps a `HtmVisitorCallbacks` struct into the `VisitorHandle` shape expected by `htm_conversion_options_builder_visitor`.

## [3.4.1] - 2026-05-13

### Changed

- **core**: `ConversionOptions` is now `Send + Sync`. `VisitorHandle` switched from `Rc<RefCell<dyn HtmlVisitor>>` to `Arc<Mutex<dyn HtmlVisitor + Send>>`, allowing `ConversionOptions` to be stashed in axum/tokio/rmcp `Send`-bound contexts. Bindings update bridge constructors to use `Arc::new(Mutex::new(...))` and add `unsafe impl Send + Sync` where required (NAPI, WASM, Magnus, FFI visitor structs).

### Fixed

- **R: `convert(html, options)` no longer rejects `ConversionOptions$default()` and `ConversionOptions$builder()$build()`.** The extendr wrappers return an `ExternalPtr<ConversionOptions>`, not an R named list, so `decode_options` raised `"options must be a named list"` for every call that used the convenience constructors. `decode_options` now extracts the wrapper directly before falling back to list decoding.
- **C#: `[DllImport("html_to_markdown_ffi")]` resolves under `ProjectReference` consumption.** P/Invoke only searches the assembly directory and standard `DYLD`/`LD_LIBRARY` paths, not NuGet's `runtimes/<RID>/native/` layout, so e2e and any other ProjectReference-based test project failed with `DllNotFoundException`. `packages/csharp/HtmlToMarkdown/HtmlToMarkdown.csproj` now copies the host-RID native library flat alongside the assembly via `<Content Include="runtimes/$(NETCoreSdkRuntimeIdentifier)/native/*" Link=… />`. NuGet consumers continue to pick it up from the existing `<None Include="runtimes/**" />` pack.
- **C#: CS0579 "Duplicate `AssemblyTitleAttribute`" eliminated.** The SDK auto-synthesises `AssemblyInfo` from `<PropertyGroup>`, which collided with checked-in `Properties/AssemblyInfo.cs` files. All three csproj files now set `<GenerateAssemblyInfo>false</GenerateAssemblyInfo>` and ship explicit `Properties/AssemblyInfo.cs`.
- **PHP: visitor `Custom` outputs preserve case for custom element callbacks** ([#2b54751a](https://github.com/xberg-io/html-to-markdown/commit/2b54751a)).
- **PHP: PIE binary discovery on macOS/Linux** ([#2b54751a](https://github.com/xberg-io/html-to-markdown/commit/2b54751a)).
- **Docs: `extract_metadata` description clarified to note it only gates the metadata pass** — table extraction into `result.tables` still runs unconditionally ([#4ba2187a](https://github.com/xberg-io/html-to-markdown/commit/4ba2187a)).

## [3.4.0] - 2026-05-09

### Added

- **Homebrew distribution** for `html-to-markdown` (CLI) and `libhtml-to-markdown` (FFI library + headers + pkg-config + CMake configs). Pre-built tarballs for macOS arm64/x86_64 and Linux arm64/x86_64; install with `brew install xberg-io/tap/html-to-markdown`.
- **WASM bundles** for all four wasm-pack targets (`web`, `bundler`, `nodejs`, `deno`) under `@kreuzberg/html-to-markdown-wasm`.
- **C# NuGet package** `KreuzbergDev.HtmlToMarkdown` with native runtimes for linux-x64, linux-arm64, osx-x64, osx-arm64, win-x64, win-arm64.
- **Java Maven Central package** `dev.kreuzberg:html-to-markdown` bundling native libraries for the same six platforms via `META-INF/native/<rid>/`.
- **Elixir Hex package** with `rustler_precompiled` NIFs for Linux + macOS (NIF 2.16/2.17 × 3 platforms); released artifacts download at first run.
- **PHP PIE pre-built archives** for PHP 8.2/8.3/8.4/8.5 × 6 platforms — `pie install xberg-io/html-to-markdown-rs` no longer requires building from source.
- **CLI panic guard** — conversion failures inside the CLI now surface as actionable errors via `panic::catch_unwind` instead of partial output + Rust backtrace.
- **`HtmlVisitor` parity across all bindings** — Python, Node/TypeScript, Ruby, PHP, Go, Java, C#, Elixir, R, and WASM all expose the visitor interface with `visit_element_start`/`visit_text`/`visit_element_end` and `VisitResult::{Continue, Skip, Custom}` semantics matching the Rust core.
- **Polyglot codegen via [alef](https://github.com/xberg-io/alef)** — bindings, e2e tests, and READMEs for all 11 target languages are generated from a single `alef.toml` + Rust source of truth, eliminating drift across the polyglot surface.

### Fixed

- **#348 — `OutputFormat::Plain` ignored `HtmlVisitor` callbacks.** The plain-text walker (`crates/html-to-markdown/src/converter/plain_text.rs`) ran the markdown pipeline first, then discarded its output and re-traversed the DOM via a visitor-less `walk_plain`, so `VisitResult::Custom`/`Skip` returned from `visit_element_end`/`visit_text` was silently dropped for `Plain`. Threaded a `WalkState` carrying the visitor through the plain walker so element/text hooks fire and their results are honoured.
- **#347 — `<img src>` URLs not escaped, breaking CommonMark round-trip.** `crates/html-to-markdown/src/converter/handlers/image.rs` emitted `src` raw, while `<a href>` already wrapped spaces/parens in angle brackets. Image renderer now uses the same three-branch escaping as links: empty → `<>`, contains space/newline → `<URL>`, unbalanced parens → `\(`/`\)` escaping.
- **#336 — large MS Word HTML truncated when `<td><p class='MsoNormal'>…</td>` appears as the leading cell.** The `tl` parser absorbs subsequent `<td>` and document content into the unclosed `<p>`, nesting the rest of the DOM inside the first table cell. Extended `has_inline_block_misnest` in `converter/preprocessing_helpers.rs` with a `has_p_ancestor` check that detects `td`/`tr`/`th` under `<p>` (structurally impossible in valid HTML) and triggers the existing html5ever repair path.
- **Split closing tags `</tagname\n>` corrupted DOM and dropped content.** JSX-style HTML (closing-tag `>` on the next line) caused the `tl` parser to leave elements unclosed, which silently absorbed siblings and dropped entire sections — affecting #127 (MW841 product headings missing from multilingual page), #143 (word-wrap merging nested link list items), and #121 (SPA menu nesting). New `normalize_split_closing_tags` preprocessing pass collapses such patterns to `</tagname>` before parsing, wired into all four preprocessing branches in `converter/main.rs`.
- **Tables now emit padded, aligned columns.** Each cell is padded to the widest cell in its column; the separator row uses `max(3, col_width)` dashes per column. `*` and `_` are escaped in table cells regardless of `escape_misc`. Fixes the gh-140 fixture parity and produces CommonMark-conformant tables out of the box.
- **#339 — bogus HTML comment endings dropped following content.** The `astral-tl` parser silently discarded every byte after `<!-- /// --->` or any `--[-]+>` comment terminator. New `normalize_bogus_comment_endings` preprocessing pass rewrites such sequences to `-->` before parsing; wired into the html5ever-repair and inline-block-misnest fallback paths too.
- **#340 — npm pre-release versions clobbered the `latest` dist-tag.** Pre-release versions (matching `-(rc|beta|alpha|pre|dev)`) now publish under the `next` dist-tag, so `npm install @kreuzberg/html-to-markdown-node` no longer pulls a 3.4.0-rc over a stable 3.3.x.
- **#337 — `from html_to_markdown import HeadingStyle` raised `TypeError`.** The package now re-exports the native PyO3 enums directly from `_html_to_markdown` and adds uppercase aliases (`HeadingStyle.ATX`, `CodeBlockStyle.BACKTICKS`) so both naming conventions satisfy `ConversionOptions(heading_style=…)`.
- **#334 — Ruby `HtmlToMarkdown.convert(html, options)` raised `TypeError` on every call with options.** The wrapper passed a `ConversionOptions` object to the FFI, but the generated Rust function expects `Option<String>` JSON. Wrapper now serialises the options hash to JSON before crossing the FFI boundary.
- **#332 — `default-features = false` Rust build broken.** Bare `#[serde(...)]` and `#[derive(Serialize, Deserialize)]` on core types in `src/types/{document,tables,result,warnings}.rs` and `src/options/conversion.rs` are now feature-gated behind `#[cfg_attr(feature = "serde", ...)]`. CI now runs a `cargo check --no-default-features` matrix to prevent regressions.
- **#331 — visitor `element_start`/`element_end` events mispaired for hyphenated/namespaced custom tags.** The `repair_with_html5ever` fallback re-parsed under HTML5 semantics, which discard XML-style self-closing on unknown elements. The repair path now pre-expands XML self-closing tags on non-void elements to explicit open+close pairs before the HTML5 parse.
- **PHP visitor marshaling** — visitor callbacks now correctly marshal arguments and handle array return values; `setVisitor()` method added to `ConversionOptions`.
- **Elixir metadata serialization** — metadata maps now serialize as JSON instead of Elixir debug format.
- **WASM Vitest environment** — WASM module loading now correctly handles Node.js module format in Vitest test environments.
- **R e2e result wrapping** — `result_is_r_list` configured to suppress `jsonlite` double-wrapping of conversion results.

### Changed

- **pnpm v11** — migrated from pnpm v10 to v11; `pnpm-workspace.yaml` declares `onlyBuiltDependencies: [esbuild]` and `ignoredBuiltDependencies: [wasm-pack]` for the new opt-in build script policy.
- **Cross-language dependency bumps** — `org.jetbrains:annotations` 26.0.0 → 26.1.0, plus updates across all language toolchains via `task upgrade`.

## [3.3.3] - 2026-04-23

### Fixed

- **Python enum KeyError** (#324) — `ConversionOptions()` with default enums no longer crashes; PyO3 enum fields are passed directly instead of broken `str()` + map lookup.
- **Ruby Magnus binding** — fixed 65 compilation errors: `funcall` API, visitor bridge args, `Vec` conversion, optional flattening, sanitized field serde round-trip.
- **Elixir `.formatter.exs`** — 120-char line length, generated code now passes `mix format --check-formatted`.
- **Unused deps** — removed `serde_json` from Node and WASM binding crates.
- **Checkstyle** — excluded `test_apps/` from pre-commit checkstyle hook.

## [3.3.2] - 2026-04-23

### Fixed

- **Elixir visitor bridge** — implemented async thread-based visitor protocol using `rustler::thread::spawn` + `OwnedEnv::send_and_clear` + `mpsc` channels, replacing the impossible synchronous `env.call()` approach.
- **Elixir NIF rustler 0.37** — replaced removed `SavedTerm`, `is_nil()`, `Pid::spawn_monitor`, `.encode()` APIs with 0.37-compatible equivalents.
- **Elixir type conversions** — fixed double-optional wrapping (`map(Some)`) and ambiguous `From` impl in generated `_from` methods.
- **Java checkstyle** — added `maven-checkstyle-plugin` to pom.xml pointing to project `checkstyle.xml` (120-char limit), so `mvn checkstyle:check` uses our config instead of default Sun checks.
- **Ruby Rakefile** — explicit `Bundler::GemHelper.install_tasks name:` for Bundler 4 compatibility.

## [3.3.1] - 2026-04-23

### Fixed

- **Java checkstyle** — switched to 120-char line limit, added Spotless auto-formatting with Eclipse JDT formatter, added `final` params and javadoc to all generated code.
- **Elixir `list` type collision** — `NodeContent::List` variant no longer redefines Elixir's built-in `list/0` type (now emits `list_variant`).
- **Elixir NIF missing `serde`** — added `serde` with derive feature as direct dependency to the NIF crate.
- **C# `VisitResult.Continue`** — default visitor methods now use `new VisitResult.Continue()` instead of non-invocable `VisitResult.Continue()`.
- **Node `convert` export** — restored the missing `#[napi] pub fn convert` function dropped during binding regeneration.
- **Ruby CI** — updated Bundler from 2.7.2 to 4.0.3 to match `Gemfile.lock`.

## [3.3.0] - 2026-04-23

### Added

- **`exclude_selectors` option** — CSS selector-based element exclusion. Unlike `strip_tags` (which removes the wrapper but keeps children), excluded elements and all descendants are dropped entirely. Supports any CSS selector: `.class`, `#id`, `[attribute]`, compound selectors. Works in both markdown and plain text output modes.
- **CLI flags** — `--preserve-tags`, `--skip-images`, `--max-depth` for full ConversionOptions parity.
- **Visitor pattern for all bindings** (#314, #313) — restored visitor support across Python, TypeScript, Ruby, PHP, Go, Java, C#, Elixir, R, WASM, and C FFI.
- **R visitor support** — added visitor callbacks for the R binding.
- **E2E test fixtures** — 78 new fixtures for 100% ConversionOptions field coverage (35/35 fields). Added fixtures for `exclude_selectors`, `ConversionResult.tables`, and `ConversionResult.warnings`.
- **Ruby RBS type stubs** — auto-generated via alef from the Rust IR, including `VERSION` constant. Gemspec now includes `sig/**/*`.
- **Alef pre-commit hook** — `alef-verify` hook added to `.pre-commit-config.yaml` to check generated code freshness. CI installs alef v0.5.3 binary.

### Fixed

- **`<h1>` inside `<header>` not exported** (#321) — top-level `<header>` elements were unconditionally dropped during preprocessing; now only `<header>` with navigation hints (e.g. `class="site-header"`, `role="navigation"`) is removed.
- **`PreprocessingPreset` not wired into preprocessing logic** — the `preset` field on `PreprocessingOptions` was defined but never checked. Now Minimal/Standard/Aggressive presets have distinct behavior.
- **`remove_forms` flag was dead code** — `<form>` elements are now dropped when `remove_forms: true` and preset is Standard or Aggressive.
- **Aggressive preset** — now drops navigation-hinted elements of any tag type, `<noscript>` elements, and noise-hinted elements (cookie banners, ad containers).
- **Python `NodeContent` type** — binding wrapper now implements `Default`, `Serialize`, `Deserialize` via forwarding to core type, fixing compilation when `DocumentNode` (which contains `NodeContent`) derives these traits.
- **Python `dict[str, Any]` for data enums** — `NodeContent`, `AnnotationKind`, `VisitResult` now use TypedDicts with `Literal` discriminators instead of untyped dicts.
- **Python `__init__.py` exports** — all public types now exported from the package.
- **`ImageMetadata.dimensions`** — tuple type `(u32, u32)` correctly maps to `Vec<u32>` / `number[]` / `[]uint32` in all bindings via serde round-trip conversion.
- **FFI doc comments** — multi-line doc strings on VTable struct fields now properly prefixed with `///` on every line (was breaking `cargo fmt`).
- **FFI optional parameters** — visitor methods with optional string params (`title`, `id`, `lang`) correctly generate `Option<&str>` instead of `&str`.
- **FFI duplicate imports** — removed duplicate `use std::ffi::...` in trait bridge output.
- **Go `htm_convert` stub** — FFI function was always returning "Not implemented"; now delegates to `core::convert(html, options, None)`.
- **Go lint errors** — fixed CGO preamble types, removed invalid `?` syntax, fixed parameter syntax, `NodeType` mapping, variable shadowing, gofmt indentation.
- **Java FFI broken on all platforms** (#315) — native libraries were bundled under wrong JAR path.
- **Java/C# visitor type conflicts** — fixed by skipping gen_bindings types when visitor bridge is active.
- **Ruby `convert()` TypeError** (#319) — options type mismatch and wrong return type.
- **PHP binding panics** — `from_update` and `from` methods now emit safe return values instead of `panic!()`.
- **R `ConversionOptionsBuilder`** — fixed placeholder panics in opaque type delegation.
- **CLI `autolinks` default** — replaced `--autolinks` with `--no-autolinks` so defaults match library.
- **CLI dead metadata flags** — removed flags that were parsed but never wired through.
- **Python 3.14 wheels** (#322) — enabled `abi3-py310` stable ABI for PyO3 crate, so a single wheel works on Python 3.10 through 3.14+ without per-version builds.

### Changed

- **Public API surface restricted** — internal Rust modules changed from `pub` to `pub(crate)`. API docs now document only the public API (down from 233 to 66 items).
- **alef.toml simplified** — switched from 20-item include whitelist to minimal 3-type + 1-function include. Transitive dependencies expand automatically.
- **Generated code lint-clean** — all generated bindings pass their respective language linters (`cargo fmt`, `cargo clippy`, `mypy`, `ruff`, `gofmt`, `dotnet format`, `mix format`, `steep check`, `rubocop`, `phpstan`, `biome`).
- **Dead code removed** — deleted unused dispatch functions, duplicate text utilities, and unused safety module from core crate.
- **CI consolidated** (#317) — 13 workflows merged into single `ci.yaml`.

## [3.2.6] - 2026-04-20

### Fixed

- **Python type mismatch** — `convert()` return type annotation now uses the public `ConversionResult` instead of `_rust.ConversionResult`, fixing Pylance type errors when annotating with the re-exported type (#310).
- **Maturin build failure** — removed `readme = "README.md"` from binding crate Cargo.toml files (ffi, node, php, py, wasm) since no README exists for these internal crates (#309).

### Changed

- **PHPUnit** — bumped from `^12.5` to `^13.1` across root, e2e, and test_apps.
- **Pre-commit shfmt hook** — fixed rev from non-existent `v3.14.2-1` to `v3.9.0-1`.
- **Pre-commit ai-rulez hook** — updated from `v3.14.0` to `v3.14.2`.

## [3.2.5] - 2026-04-18

### Fixed

- **Silent truncation on large HTML inputs** (#277) — fixed preprocessing pipeline gap where `strip_script_and_style_tags` was not called after `repair_with_html5ever`, causing documents with custom elements and scripts containing literal `<script>` strings to be silently truncated. Also fixed `preprocess_html` fallback to skip only the problematic tag instead of consuming the rest of the document.
- **WASM plain-object options** (#303) — WASM package now exports a wrapper `convert()` function that accepts plain JavaScript objects for options, eliminating the need to construct `WasmConversionOptions` class instances. The raw WASM classes remain available for advanced use.

### Added

- **`max_depth` option** — new `max_depth` field on `ConversionOptions` to limit DOM traversal depth, preventing stack overflow on deeply nested or malicious HTML. Default is `None` (unlimited). When set, subtrees beyond the limit are silently truncated.

### Removed

- Phantom type references (`MetadataResult`, `HeadingInfo`, `LinkInfo`, `ImageInfo`) from alef configuration that did not correspond to any Rust types.

## [3.2.4] - 2026-04-17

### Fixed

- **Elixir Hex package** — fixed precompiled NIF pipeline: correct build output paths, removed Rust source from Hex files list, removed Windows target (no build job), simplified build script for `rustler_precompiled`.
- **C# NuGet package** — republished with `htm_conversion_options_from_json` and related FFI functions.

## [3.2.3] - 2026-04-17

### Fixed

- **Java/C#/Go FFI functions** — `htm_conversion_options_from_json`, `htm_preprocessing_options_from_json`, and related `to_json` functions now generated. Fixed alef IR extraction to detect serde derives inside `#[cfg_attr(...)]` attributes.
- **Node.js native binding loader** — regenerated `index.js` with correct NAPI platform-aware loader (was referencing old `html-to-markdown-rs.node` binary name).
- **Go module path** — fixed from non-existent `github.com/xberg-io/html-to-markdown-go` to monorepo path `github.com/xberg-io/html-to-markdown/packages/go/v3`.
- **Elixir precompiled NIFs** — switched from `Rustler` (compile-from-source) to `RustlerPrecompiled` with CI jobs for building and uploading platform-specific NIF binaries to GitHub releases.

### Removed

- Stale hand-written test files superseded by alef-generated e2e tests (comprehensive_test, feature_test, smoke_test duplicates across Go, Python, Ruby, PHP, Node, WASM, Elixir, R).
- Empty placeholder crate directories (`html-to-markdown-rs-ffi`, `html-to-markdown-rs-wasm`).
- Duplicate Ruby extension directory (`html-to-markdown_rb` with wrong naming).

## [3.2.2] - 2026-04-16

### Fixed

- **Ruby binding compilation** — fixed serde derive errors and Default trait conflicts by conditionally deriving serde traits only when all field types support it, and generating Default derives for kwargs constructors.
- **Ruby deprecated Magnus API** — replaced `magnus::exception::type_error()` / `runtime_error()` with `Ruby::exception_type_error()` / `Ruby::exception_runtime_error()` (Magnus 0.7+ API).
- **PHP e2e tests** — fixed missing class import causing "Class not found" fatal error.
- **Cargo.toml metadata** — added missing `readme`, `keywords`, `categories`, `description` fields to binding crate Cargo.toml files.
- **C# dotnet format** — auto-formatted generated C# bindings.

## [3.2.1] - 2026-04-16

### Fixed

- **Node.js Docker/cross-platform installs** (#273) — platform-specific native packages (`@kreuzberg/html-to-markdown-node-linux-x64-gnu`, etc.) are now correctly published with `optionalDependencies` via NAPI prepublish, resolving cross-platform lockfile issues.
- **Homebrew formula** (#304) — formula updated with correct source tarball SHA and bottle configuration.
- **Ruby gem build failure** — fixed `NodeContent::MetadataBlock` type mismatch (`Vec<(String, String)>` → `String`) in binding-to-core conversion by deserializing sanitized fields from JSON.
- **Maven Central publish** — aligned `pom.xml` with kreuzberg: GPG plugin in main build section, developer email, correct `groupId` (`dev.kreuzberg`), `pluginManagement` with pinned plugin versions.
- **All binding compilation failures** — fixed private module path (`convert_api`) in generated bindings, missing `From` impls for function return types, glob import conflicts in PyO3/FFI backends, and cbindgen compatibility (removed `const extern fn`, updated to cbindgen 0.29).
- **Elixir NIF compilation** — added `compilers: [:rustler] ++ Mix.compilers()` to `mix.exs` so Rustler compiles the NIF during `mix compile`.
- **FFI `from_json`/`to_json` functions** — `htm_conversion_options_from_json`, `htm_conversion_result_to_json`, etc. now generated for all serde-compatible types, fixing Java (Panama FFM) and Go (cgo) bindings.
- **PHP e2e tests** — fixed function call generation to use correct `HtmlToMarkdownRs::convert()` pattern.
- **Python `pyproject.toml`** — corrected `module-name` and `python-packages` to match `html-to-markdown` pip package name.
- **`docs/llms.txt`** metadata defaults corrected from `false` to `true` for `extract_metadata`, `extract_document`, `extract_headers`, `extract_links`, `extract_images`, `extract_structured_data` (#276).

- **WASM type prefix restored** (#303) — `WasmConversionOptions` (not `JsConversionOptions`) via configurable `type_prefix` in alef. No breaking change for WASM users.

### Known Issues

- **Python silent output cap** (#277) — `convert()` silently truncates output at ~439 KB on certain large HTML inputs. Under investigation.

## [3.2.0] - 2026-04-14

### Breaking Changes

#### Core Defaults Changed

- **`code_block_style`** default changed from `Indented` to `Backticks` — code blocks now use triple-backtick fences by default instead of 4-space indentation.
- **`bullets`** default changed from `"-"` to `"-*+"` — nested unordered lists now cycle through `-`, `*`, `+` at successive nesting levels.
- **`preprocessing.enabled`** default changed from `false` to `true` — HTML preprocessing (navigation removal, form stripping) is now on by default.
- **Rust serde field names** changed from `camelCase` to `snake_case` — affects JSON serialization/deserialization of the Rust core `ConversionOptions` struct (`heading_style` instead of `headingStyle`). Language bindings are not affected — each binding uses its language-native naming convention (camelCase for JS/TS/Java/C#, snake_case for Python/Ruby/Elixir/R).

#### Package Renames

- **TypeScript/Node.js**: npm package renamed from `@kreuzberg/html-to-markdown` to `@kreuzberg/html-to-markdown-node`.
- **PHP**: Namespace changed from `HtmlToMarkdown\` to `Html\To\Markdown\Rs\`. Main class renamed from `HtmlToMarkdown` to `HtmlToMarkdownRs`.
- **Java**: Maven coordinates remain `dev.kreuzberg:html-to-markdown`. Internal package namespace changed to `dev.kreuzberg.htmltomarkdown`.
- **C FFI**: Function prefix changed from `html_to_markdown_` to `htm_` (e.g., `htm_convert`, `htm_last_error_code`). Header moved to `include/html_to_markdown.h`.
- **C#**: Main class renamed from `HtmlToMarkdownConverter` to `HtmlToMarkdownRs`.

#### Python Exception Hierarchy

- Old exceptions (`HtmlToMarkdownError`, `EmptyHtmlError`, `InvalidParserError`, etc.) replaced with new hierarchy: `ConversionError`, `ParseError`, `SanitizationError`, `ConfigError`, `IoError`, `InvalidInputError`, `PanicError`, `OtherError`.

### Added

- **`ConversionOptionsBuilder`** is now public in the Rust API — use `ConversionOptions::builder()` for ergonomic option construction.
- **`TableData`** exported from crate root — no longer requires importing from submodules.
- **Per-language API reference documentation** — generated `docs/reference/api-{lang}.md` pages for Python, TypeScript, Go, Java, C#, Ruby, PHP, Elixir, WASM, and C with full type mappings, signatures, and docstrings.
- **Alef codegen** — all 12 language bindings (Rust, Python, TypeScript, Go, Java, C#, Ruby, PHP, Elixir, WASM, C, R) are now auto-generated from a single IR via [alef](https://github.com/xberg-io/alef), configured in `alef.toml`.
- **E2E test suite** — 130 fixture-driven tests across all 12 languages, generated from shared JSON fixtures in `fixtures/`.
- **`AnnotationKind`** and **`NodeContent`** now implement `Default`.
- **`ConversionResult`** now derives `Serialize`/`Deserialize`.

### Changed

- **Docs platform**: Switched from MkDocs to [Zensical](https://zensical.dev) (`zensical.toml` replaces `mkdocs.yaml`).
- **READMEs**: Now generated by `alef readme` from minijinja templates with inline configuration in `alef.toml`.
- **Version sync**: `task version:sync` now uses `alef` binary for all operations (replaces `sync_versions.py`).
- **CI workflows**: Simplified to e2e-only testing per language — removed per-binding unit test jobs.

### Removed

- **`crates/html-to-markdown-bindings-common`** — shared bindings helper crate replaced by alef codegen.
- **`tools/e2e-generator`** — replaced by `alef e2e generate`.
- **`tools/snippet-runner`** — snippet validation removed.
- **`scripts/generate_readme.py`**, **`scripts/sync_versions.py`**, **`scripts/readme_filters.py`** — replaced by alef.
- **Hand-written binding code** — all per-language binding implementations replaced by alef-generated code.

### Fixed

- **Preserve metadata when using AST visitors** (#279).
- **`deny_unknown_fields`** added to serde option structs — invalid JSON fields now produce errors instead of being silently ignored.
- **Ruby e2e generator** — fixed camelCase→snake_case field name conversion.
- **WASM test options** — added missing `link_style` field.

## [3.1.0] - 2026-04-01

### Added

- **Reference-style links**: New `link_style` option (`"inline"` default, `"reference"`) renders links as `[text][1]` with numbered `[1]: url "title"` definitions appended at the end of the output. Supports URL+title deduplication, images (`![alt][1]`), and media elements (audio, video, iframe). Available across all bindings (Python, Node.js, WASM, PHP, CLI `--link-style`, FFI via JSON).

## [3.0.2] - 2026-04-01

### Fixed

- **Structure collector in tables**: Suppressed `StructureCollector` calls for headings and lists inside table cells, preventing spurious document-structure nodes from table content.
- **Char boundary safety**: Fixed potential panics from slicing at non-UTF-8-char boundaries in `generate_id` hash truncation and list item text extraction.
- **Dead feature gates removed**: Cleaned up unused `document-structure` feature gates that were no longer wired to any Cargo feature.
- **Structure collector coverage**: Added missing `StructureCollector` calls for lists, images, and code blocks so document structure captures all block-level elements.

## [3.0.1] - 2026-03-31

### Fixed

- **WASM TypeScript types**: `convert()` now returns typed `WasmConversionResult` instead of `any`. All `WasmConversionTable`, `WasmGridCell`, `WasmTableGrid`, `WasmConversionWarning`, and `WasmInlineImage` interfaces are now emitted in generated `.d.ts` files. Added missing options fields (`skipImages`, `outputFormat`, `includeDocumentStructure`, `extractImages`, `maxImageSize`, `captureSvg`, `inferDimensions`). Fixes #265.
- **Python type stubs**: Synced crate `.pyi` stub with package stub — added keyword-only (`*`) parameter markers and `visitor` parameter to `convert()`.
- **PHP type stubs**: Expanded PHPStan stubs with full `ConversionResult`, `ConversionOptions`, and all nested type shapes. Wired stubs into `composer.json` PHPStan config.

## [3.0.0] - 2026-03-30

### Added

- **Single `convert()` API**: One entry point across all 12 language bindings returning `ConversionResult` with content, document, metadata, tables, images, and warnings.
- **`ConversionResult` type**: Structured result with `content` (markdown/djot/plain), `document` (optional `DocumentStructure`), `metadata` (`HtmlMetadata`), `tables` (grid-based), `images` (inline image data), and `warnings`.
- **`DocumentStructure`**: Structured document tree with flat node array, index-based parent/child references, and `TextAnnotation` for inline formatting.
- **Options support in all bindings**: Go, Java, C# now accept options. All generators wire fixture options into e2e tests.
- **GFM defaults**: Code blocks default to backtick fences (was indented). ATX headings remain default.
- **E2E contract validation**: Generators produce tests validating ConversionResult structure (metadata, tables, warnings) across all 12 languages.
- **New options**: `includeDocumentStructure`, `extractImages`, `maxImageSize`, `captureSvg`, `inferDimensions`, `outputFormat` (markdown/djot/plain).
- **`<q>` element**: Wraps content in quotation marks.
- **`<figure>`/`<figcaption>` elements**: Routed to semantic handler with caption separation.
- **`hidden` attribute**: Elements with `hidden` stripped before parsing.

### Changed

- **`convert()` returns `ConversionResult`** instead of `String` in all bindings (Go, Java, C#, Node, Python, PHP, Ruby, Elixir, R, WASM, C FFI).
- **`ExtendedMetadata` renamed to `HtmlMetadata`** across all crates and bindings.
- **Go `Convert()`** returns `*ConversionResult` with `Content`, `Metadata`, `Tables`, `Images`, `Warnings` fields. Accepts optional JSON options via variadic parameter.
- **Table data** uses grid-based schema (`TableGrid` with `GridCell`) instead of flat `cells [][]string`.
- **`serde(deny_unknown_fields)`** on `MetadataConfig`, `MetadataConfigUpdate`, `InlineImageConfigUpdate`.
- **Go 1.26**, golangci-lint@latest.

### Removed

- **All `convert_with_*` functions**: `convert_with_metadata`, `convert_with_inline_images`, `convert_with_visitor` (standalone), `convert_with_tables`, `convert_with_async_visitor` removed from public API. Single `convert()` replaces all.
- **Async visitor**: Feature removed entirely (`async-visitor` Cargo feature, `AsyncHtmlVisitor` trait, async bridge/dispatch code).
- **Profiling**: All profiling infrastructure removed (8 binding crates, CI workflow, C tests, `start_profiling`/`stop_profiling` APIs).
- **Benchmarks**: All benchmark scripts and harness removed.
- **hOCR support**: Entire `hocr` module deleted. The `hocr_spatial_tables` option removed.
- **Python v1 compatibility**: `convert_to_markdown()` and `markdownify()` removed.
- **Redundant binding tests**: Tests covered by e2e generators removed from Python, Ruby, Elixir, R.

## [2.30.0] - 2026-03-27

### Deprecated

- **hOCR support**: The `hocr_spatial_tables` option and all hOCR-related APIs are deprecated and will be removed in v3. All hOCR functionality continues to work but emits deprecation warnings. This is the final v2 release.

### Fixed

- **PHP PHPStan CI errors**: Removed redundant `@var` annotations and `is_array()` runtime checks in `ExtensionBridge.php` that PHPStan flagged as always-true due to stub-defined return types. Removed redundant `array_values()` calls in `ConversionOptions.php` on properties already typed as `list<string>`. Updated PHPStan baseline count for callable invocations.

## [2.29.0] - 2026-03-22

### Added

- **`full` feature group**: Added a `full` feature to core crate and all binding crates (PHP, Python, Node, WASM, FFI, Elixir, bindings-common) that enables all available features. All bindings now default to `full`.
- **Dublin Core metadata extraction**: `DC.*` and `DCTERMS.*` meta tags now map to dedicated `DocumentMetadata` fields (title, description, author, keywords). Other DC/DCTERMS fields stored in `meta_tags` with `dc_`/`dcterms_` prefix.
- **Extended keyword variants**: Keywords now extracted from `news_keywords`, `citation_keywords`, `DC.subject`, `DC.keywords`, `DCTERMS.subject`, `subject`, `topic`, `category`, and `classification` meta tags.
- **`cargo-sort` pre-commit hook**: Added for consistent Cargo.toml key ordering.
- **`checkmake` pre-commit hook**: Added for Makefile linting.
- **`typescript-typecheck` pre-commit hook**: Added TypeScript type checking via `tsc --noEmit`.
- **`typecheck` npm script**: Added to `packages/typescript/package.json`.

### Fixed

- **Case-insensitive meta tag matching** ([#251](https://github.com/xberg-io/html-to-markdown/issues/251)): All meta tag name matching is now case-insensitive per the HTML spec. `<meta name="Keywords">` and `<meta name="DC.keywords">` are now correctly captured.
- **PHP `convertWithTables()` not found** ([#250](https://github.com/xberg-io/html-to-markdown/issues/250)): The `visitor` feature was not enabled by default in the PHP binding crate, causing `html_to_markdown_convert_with_tables` to be missing from the extension.
- **PHP binding defaults**: PHP crate now defaults to `["full"]` (was `["metadata"]`), enabling visitor support.
- **Python binding defaults**: Python crate now defaults to `["full"]` (was `[]`), enabling metadata, visitor, async-visitor, and inline-images.
- **PHPStan 2.x compatibility**: Fixed 40+ PHPStan errors from the 1.x→2.x upgrade (type narrowing, property access on mixed, redundant assertions). Added `--memory-limit=512M` to prevent OOM.
- **Makefile `test` target**: Added missing `.PHONY: test` target to FFI test Makefile.

### Changed

- **Pre-commit config aligned with kreuzberg**: Added `cargo-sort`, `checkmake`, `typescript-typecheck`. Updated `taplo-format` to exclude `Cargo.toml`. Excluded `Makefile.frag` from checkmake.
- **Cargo.toml formatting**: All workspace Cargo.toml files sorted via `cargo-sort`.
- **pyproject.toml formatting**: All pyproject.toml files formatted via `pyproject-fmt`.

## [2.28.6] - 2026-03-20

### Changed

- **Ruby gem vendoring**: Replaced bash+embedded-Python vendoring script with a standalone Python vendoring script adapted from kreuzberg, using `vendor/` directory instead of `rust-vendor/` for core crate vendoring.
- **Ruby gem build**: Added `build-native-gem.rb` for platform-specific pre-compiled gem builds, following kreuzberg patterns.
- **Pre-commit hooks**: Switched Ruby hooks (rubocop, rbs-validate, steep-check) from inline bash commands to task-based delegation matching kreuzberg.
- **Dependabot config**: Expanded from GitHub Actions only to full multi-ecosystem coverage (Cargo, pip, npm, bundler, composer, gomod, maven, nuget, mix).
- **Task update commands**: Aligned all language update tasks with kreuzberg's comprehensive approach (outdated checks, aggressive updates).
- **C# update**: Switched from slow `dotnet list --outdated` Python script to `dotnet-outdated-tool` for faster dependency updates.

### Fixed

- **CI Validate shfmt failure**: Fixed `packages/r/configure.win` tab indentation to match shfmt 2-space requirement.
- **Java linting**: Added PMD plugin (3.28.0), JaCoCo coverage (0.8.14), and pinned checkstyle runtime (13.3.0). Bumped maven-compiler-plugin to 3.15.0, maven-surefire-plugin to 3.5.5, spotless to 3.4.0, central-publishing to 0.10.0.

### Updated

- **GitHub Actions**: Bumped `go-task/setup-task` from v1 to v2, `nick-fields/retry` from v3 to v4.
- **Dependencies**: Updated all language dependencies via `task update`.

## [2.28.5] - 2026-03-19

### Fixed

- **Table colspan parsing** ([#233](https://github.com/xberg-io/html-to-markdown/issues/233)): Fixed column count calculation to accurately use colspan values instead of incrementing by 1, and refined layout table heuristics to exempt simple data tables with colspans while correctly catching layout tables.
- **Ruby version.rb formatting**: Fixed missing space around `=` operator in `version.rb` that caused Rubocop lint failures in CI.
- **CI tooling alignment**: Aligned tooling and documentation with kreuzberg standards, fixing CI failures.

## [2.28.4] - 2026-03-13

### Fixed

- **Panic with cid image followed by italic paragraph** ([#222](https://github.com/xberg-io/html-to-markdown/issues/222)): Confirmed fix for panic ("byte index 53 is out of bounds of ``") when converting HTML containing `cid:` image paragraphs followed by italicized text. This was resolved in v2.28.0 via the block_content_start bounds fix (#216, #217) and multi-byte UTF-8 character boundary fix (#218).
- **Ruby gem installation on macOS** ([#219](https://github.com/xberg-io/html-to-markdown/issues/219)): Confirmed fix for `Cargo.lock` missing from published gem causing `magnus` dependency load failure. Resolved in v2.28.3 with native platform gem builds.

## [2.28.3] - 2026-03-10

### Fixed

- **Java visitor FFI struct return type**: Fixed `IllegalArgumentException: Wrong method handle type` when using visitors in Java. The Panama FFI callback descriptors incorrectly used `JAVA_LONG` (8 bytes) as the return type instead of the actual `HtmlToMarkdownVisitResult` C struct (24 bytes: enum + 2 pointers). All 14 callback descriptors now use a proper `StructLayout` matching the C ABI.
- **Homebrew bottle tarball structure**: Fixed bottle tarballs missing the required `html-to-markdown/{version}/` prefix directory. Homebrew expects this prefix for proper cellar installation.
- **Ruby gem publishing**: Added native platform gem builds (`rake native gem`) alongside source gems so precompiled extensions are available for Linux, macOS, and Windows.

## [2.28.2] - 2026-03-08

### Fixed

- **Publish workflow republish flag**: Fixed republish mode skipping all publish jobs because `INPUT_REF` resolved to a branch name instead of the tag ref.
- **Definition list fixture**: Aligned real-world test fixture for `<dl>`/`<dt>`/`<dd>` with actual converter output (plain text, no Pandoc-style `:` prefix).

## [2.28.1] - 2026-03-06

### Fixed

- **Panic with multi-byte UTF-8 and visitor** ([#218](https://github.com/xberg-io/html-to-markdown/issues/218)): Fixed a panic ("byte index N is not a char boundary") when converting HTML containing multi-byte UTF-8 characters (Cyrillic, CJK, emoji, etc.) with tabs between block elements and any visitor. The stale byte position captured before whitespace trimming could land inside a multi-byte character when new content was appended.
- **Java formatting**: Fixed spotless formatting violations in `HtmlToMarkdown.java`, `TableData.java`, and `TableExtractionResult.java`.

## [2.28.0] - 2026-03-05

### Added

- **Table extraction API**: New `convert_with_tables` function that extracts structured table data during HTML-to-Markdown conversion. Returns `TableData` structs containing cell contents as `Vec<Vec<String>>`, rendered markdown output, and per-row header flags. Uses the visitor pattern internally with a built-in `TableCollector` to capture table structure in a single pass. Available across all language bindings:
  - **Rust**: `convert_with_tables(html, options, metadata_config)` returning `ConversionWithTables`
  - **Python**: `convert_with_tables(html, options, preprocessing, metadata_config)` returning `TableExtractionResult`
  - **TypeScript/Node.js**: `convertWithTables(html, options?, metadataConfig?)` returning `TableExtraction`
  - **Ruby**: `HtmlToMarkdown.convert_with_tables(html, options, metadata_config)` returning a Hash
  - **PHP**: `HtmlToMarkdown::convertWithTables($html, $options, $metadataConfig)` returning `TableExtractionResult`
  - **Go**: `ConvertWithTables(html)` returning `TableExtractionResult`
  - **Java**: `HtmlToMarkdown.convertWithTables(html)` returning `TableExtractionResult`
  - **C#**: `HtmlToMarkdownConverter.ConvertWithTables(html)` returning `TableExtractionResult`
  - **Elixir**: `HtmlToMarkdown.convert_with_tables(html, options, metadata_config)` returning `{:ok, content, tables, metadata}`
  - **R**: `convert_with_tables(html, options, metadata_config)` returning a list
  - **C (FFI)**: `html_to_markdown_convert_with_tables(html, options_json, metadata_json)` returning JSON
  - **WASM**: `convertWithTables(html, options?, metadataConfig?)` returning a JS object

### Fixed

- **Plain text fast path skipping visitor callbacks**: When `OutputFormat::Plain` was used with `convert_with_tables`, the plain text fast path returned before the visitor could extract table data, resulting in empty tables. The conversion pipeline now runs the full visitor walk before returning plain text content.

## [2.27.3] - 2026-03-05

### Fixed

- **Panic on block_content_start out of bounds**: Fixed a crash (`byte index N is out of bounds`) in text node processing when inline handlers (e.g. `<strong>`, `<em>`) collected children into a fresh buffer while inheriting a parent paragraph context. The `block_content_start` index pointed into the wrong buffer, causing a panic on certain HTML structures — notably `<details>` containing `<p>` with inline formatting. (Issues #216, #217)

## [2.27.2] - 2026-03-02

### Fixed

- **Plain text list items missing markers**: `<ul>` and `<ol>` list items in `OutputFormat::Plain` were output without any bullet or number prefix. Now emits `-` for unordered lists and sequential `N.` for ordered lists, respecting the `start` attribute on `<ol>`.

## [2.27.1] - 2026-03-01

### Fixed

- **Colon introduced into definition list text**: `<dd>` elements inside `<dl>` were incorrectly prefixed with `:` (Pandoc definition list syntax), introducing spurious colons into converted text. Standard Markdown and GFM do not support definition list syntax, so `<dd>` content is now output as plain blocks. (Issue #214, thanks @smoyerx)
- **Go test app go.sum out of sync**: Updated `tests/test_apps/go/go.sum` to match the v2.27.0 module version, fixing the CI Go lint job.

## [2.27.0] - 2026-03-01

### Added

- **Plain text output format**: New `OutputFormat::Plain` option that strips all markup and returns only visible text content. Set `output_format` to `"plain"` (also accepts `"plaintext"` or `"text"`). This fast-path bypasses the full Markdown/Djot conversion pipeline — after DOM parsing, a lightweight text extractor walks the tree collecting only visible text with structural whitespace. Useful for search indexing, text extraction, and feeding content to LLMs.

## [2.26.3] - 2026-02-28

### Fixed

- **Subscript/superscript content silently dropped**: When `sub_symbol` or `sup_symbol` was empty (the default), text inside `<sub>` and `<sup>` tags was discarded entirely — e.g. `H<sub>2</sub>O` produced `HO` instead of `H2O`.
- **Missing whitespace between newline-separated inline elements**: Whitespace-only text nodes containing newlines between adjacent inline elements (e.g. `<a>…</a>\n<a>…</a>`) were dropped, causing links and other inline markup to merge without a word boundary. Now collapses to a single space per HTML white-space normalization rules.

## [2.26.2] - 2026-02-28

### Fixed

- **Inconsistent whitespace before inline elements across paragraphs**: Fixed a stateful bug where `\n` before `<a>`, `<strong>`, `<em>`, and other inline elements inside `<p>` tags was handled differently depending on the paragraph's position in the document. The second and subsequent paragraphs would drop the space before inline elements, producing `text[link](url)` instead of `text [link](url)`. (Issue #212, thanks @haroldparis)

## [2.26.1] - 2026-02-27

### Fixed

- **YAML frontmatter in `convert_with_metadata` output**: `convert_with_metadata` no longer prepends YAML frontmatter to the markdown string. Since metadata is returned as a structured `ExtendedMetadata` object, embedding it in the content string was redundant and polluted the output.

## [2.26.0] - 2026-02-26

### Added

- **C FFI distribution infrastructure**: Distribution-grade C FFI library with CMake/pkg-config integration, installation scripts, and packaging for system-level consumption.
- **C FFI test coverage**: Comprehensive C test suite covering conversion, metadata extraction, error handling, visitor pattern, profiling, and version queries.
- **C documentation and examples**: C API reference, getting-started snippets, and example programs for basic conversion, metadata extraction, and visitor pattern usage.

### Fixed

- **R package r-universe build**: Configure scripts now download the source archive from GitHub when the monorepo is unavailable, enabling r-universe and standalone source installs to vendor crates automatically.

## [2.25.2] - 2026-02-25

### Fixed

- **Visitor panic with metadata extraction**: Fixed an out-of-bounds slice panic when using visitors (e.g. image visitors returning `Custom`) combined with metadata extraction on minified HTML. The issue occurred because parent element output offsets became stale after child visitor truncations. (PR #204, thanks @gmalette)

### Added

- **R language bindings**: Full-parity R bindings via extendr framework with support for `convert()`, `convert_with_options()`, `convert_with_options_handle()`, `convert_with_metadata()`, `convert_with_inline_images()`, `convert_with_visitor()`, and profiling. Includes `conversion_options()` helper, testthat test suite, CI workflow, lintr/styler pre-commit hooks, and task automation.
- **R CRAN publishing infrastructure**: Added `publish-cran` job to publish workflow, `cran-comments.md`, and `NEWS.md` for CRAN submission compliance.

### Changed

- **Workspace restructuring**: Moved Ruby native crate out of the root Cargo workspace into a standalone workspace (matching Elixir/R pattern), resolving `clang-sys` link conflict with `ext-php-rs` 0.15.6.
- **Rust update task**: Now updates dependencies in all separate workspaces (Ruby, Elixir, R) via `--manifest-path` entries.
- **Upgraded `wasmtime`** from 41 to 42.
- **Upgraded `ext-php-rs`** from 0.15.4 to 0.15.6.
- **Upgraded `pyo3`** from 0.28.1 to 0.28.2.
- **Upgraded `wasm-bindgen`** from 0.2.112 to 0.2.113.
- **Upgraded `rustls`** from 0.23.36 to 0.23.37.

## [2.25.1] - 2026-02-17

### Fixed

- **hOCR heading detection**: Improved hierarchy logic to use font size (`x_fsize`) and bbox height as a proxy when detecting headings. Large-font paragraphs now support longer text (up to 80 chars) and single-word headings. Added comprehensive test coverage for heading detection edge cases.

## [2.25.0] - 2026-02-15

### Added

- **Bun runtime support**: Official support for Bun 1.2+ via Node-API compatibility. The existing NAPI-RS bindings work in Bun without changes. Added Bun to CI test matrix and updated documentation to reflect runtime compatibility.

### Changed

- **Vendored `markup5ever_rcdom`**: Brought the `markup5ever_rcdom` code (MIT/Apache-2.0) into the core crate as an internal `rcdom` module. This removes the external dependency on the "+unofficial" crate, eliminates the unused `xml5ever` transitive dependency, and removes the pinned `html5ever`/`markup5ever_rcdom` version constraints. See `ATTRIBUTIONS.md` for license details.
- **Upgraded `html5ever`** from 0.36.1 to 0.38.0 (now unpinned).
- **Upgraded `pyo3`** from 0.28.0 to 0.28.1.

## [2.24.6] - 2026-02-14

### Fixed

- **Dependency update stability**: Pinned compatible `html5ever`/`markup5ever_rcdom` versions to prevent trait-mismatch breakages during workspace dependency updates.
- **Python bindings build**: Added explicit `#[pyclass(from_py_object)]` on Python config wrapper classes to avoid PyO3 deprecation failures under `-D warnings`.
- **Rust lint consistency**: Aligned crate-level clippy configuration so `multiple_crate_versions` does not fail Node/WASM/FFI crate lint runs.
- **WASM dependency behavior**: Updated hashing dependency configuration to avoid wasm randomness backend breakage after dependency updates.
- **PHP PIE publish verification (macOS)**: Hardened PIE verification/build scripts for Darwin linker behavior and shell-safe package spec handling.
- **CI reliability**: Updated validation and Python CI tasks to reduce flakiness (PHP 8.4 setup in validate; avoid redundant Rust CLI release builds in Python test runs).

## [2.24.5] - 2026-02-01

### Fixed

- **Subscript/superscript whitespace handling**: Subscript and superscript tags now trim inner whitespace and place it outside delimiters, matching the behavior of bold, italic, and strikethrough (issue #202).

## [2.24.4] - 2026-01-31

### Performance

- **Reduced allocations in hot conversion paths**: Return `Cow<str>` from escape to avoid allocating on no-op paths, replace `.repeat()` with direct push loops in heading/list/table/div/paragraph formatters, eliminate `collect::<Vec>::join()` in text dedentation, and use `AHashMap` for hOCR property maps.

### Fixed

- **WASM builds**: Updated `getrandom` backend configuration from `"js"` to `"wasm_js"` for compatibility with getrandom 0.3.x.
- **Elixir/Ruby vendor scripts**: Added missing `ahash` workspace dependency replacement for standalone builds.

## [2.24.3] - 2026-01-31

### Fixed

- **Definition lists**: Ensure `<dl>/<dt>/<dd>` output is consistent regardless of HTML whitespace/minification, and properly indent multiline definition content (issue #200).
- **Link labels**: Removed hard truncation of long link labels to avoid broken Markdown for large image links (issue #199).

## [2.24.2] - 2026-01-29

### Fixed

- **Java packaging**: Bundle native FFI libraries in published Maven JAR for all platforms (linux-x86_64, linux-aarch64, osx-aarch64, windows-x86_64). The Java package now works out-of-the-box when installed from Maven Central without requiring local FFI builds or manual java.library.path configuration. Native libraries are automatically extracted to a temp directory on first use with platform detection and fallback support.

## [2.24.1] - 2026-01-27

### Fixed

- **UTF-16 recovery**: Automatically recovers UTF-16 HTML (including data without BOM) that was read via lossy UTF-8 decoding, instead of rejecting it as binary data.
- **URL sanitization**: Hardened markdown-like URL sanitization to extract the real URL from `...[text](url)` patterns in `href`/`src` attributes, preventing caller-side URL join/parsing errors.
- **Issue #190 coverage**: Added regression fixtures and tests covering the reported real-world HTML inputs.

## [2.24.0] - 2026-01-24

### Changed

- **Bindings API**: Removed `_json` conversion entrypoints across bindings; convert functions now pass full option payloads directly.

### Fixed

- **Visitor docs**: Corrected Python visitor documentation and examples (argument order + ctx access).
- **Python visitor options**: `convert_with_visitor` now respects full conversion options payloads.
- **skip_images**: Skip flag now suppresses SVG/graphic outputs in addition to `<img>`.
- **Code block dedent**: Handles Unicode whitespace without panicking on UTF-8 boundaries.
- **Input validation**: Tolerates small NUL byte artifacts and strips them before conversion.

## [2.23.6] - 2026-01-21

### Fixed

- **pnpm lockfile synchronization**: Fixed pnpm lockfile to include Node.js platform-specific optional dependency version updates (2.19.0-rc.1 → 2.23.6) that were applied during v2.23.5 version sync. This resolves the `ERR_PNPM_OUTDATED_LOCKFILE` errors that caused the v2.23.5 publish workflow to fail.
- **CI Java version**: Updated CI Java workflow from Java 24 to Java 25 to match maven.compiler.release=25 configuration, ensuring CI and local builds use the same compiler version.

## [2.23.5] - 2026-01-21

### Fixed

- **Maven Central publishing**: Corrected group ID in Maven Central check script from legacy `io.github.goldziher` to `dev.kreuzberg`, enabling successful Java package publishing. This resolves the issue where Java v2.23.4 failed to publish to Maven Central.
- **Go module publishing**: Added automated Go module tag creation (`packages/go/v{version}`) to publish workflow, ensuring Go packages are immediately available on Go proxy after release.
- **Go FFI version synchronization**: Updated Go FFI default version constants from outdated versions (2.19.1/2.23.0) to 2.23.5 in both `ffi_loader.go` and `cmd/install/main.go`, ensuring automatic downloads use the correct library version.
- **Node.js platform dependencies**: Synchronized all platform-specific optional dependencies in `@kreuzberg/html-to-markdown-node` package.json from 2.19.0-rc.1 to match main package version, preventing dependency resolution issues.
- **Java benchmark packaging**: Updated benchmark-pom.xml to use correct group ID (`dev.kreuzberg`), version (2.23.5), and main class namespace (`dev.kreuzberg.benchmark.Benchmark`). Removed outdated generated `dependency-reduced-pom.xml`.
- **PHP package references**: Updated all PHP package references from `goldziher/html-to-markdown` to `xberg-io/html-to-markdown` across composer.json, PIE verification scripts, and smoke test actions to reflect current package organization.
- **Java smoke tests**: Updated smoke-java GitHub action to use correct group ID (`dev.kreuzberg`) and package namespace (`dev.kreuzberg.htmltomarkdown.SmokeTest`) for JAR installation and test execution.
- **Build tooling**: Fixed Python script execution in task runner to use `uv run python3` instead of system python3, ensuring consistent dependency resolution. Added PyYAML and Jinja2 to workspace dev dependencies.
- **Version sync automation**: Enhanced version sync script to automatically update Node.js platform-specific optional dependencies alongside main package version, preventing manual version drift.

## [2.23.4] - 2026-01-20

### Fixed

- **TypeScript wrapper publishing**: Fixed TypeScript wrapper dependency resolution by installing dependencies directly from npm registry instead of from workspace after Node packages are published. This ensures `@kreuzberg/html-to-markdown-node` is available from npm when building the TypeScript wrapper, eliminating the workspace resolution issues that caused previous build failures.

## [2.23.3] - 2026-01-20

### Fixed

- **Go FFI packaging**: Fixed missing `html_to_markdown.h` header file in Go FFI archive tarballs, which caused `go:generate` installation to fail with "fatal error: 'html_to_markdown.h' file not found". The header is now included in all platform archives (tar.gz and zip).
- **TypeScript wrapper publishing**: Fixed pnpm lockfile frozen mode error during TypeScript wrapper dependency reinstallation by adding `--no-frozen-lockfile` flag. The reinstall step after publishing Node packages now correctly updates workspace dependencies despite lockfile version mismatches.

## [2.23.2] - 2026-01-20

### Fixed

- **TypeScript wrapper publishing**: Fixed TypeScript wrapper build failures by moving the build and publish steps into the same `publish-node` job. This eliminates npm CDN propagation delays that caused `@kreuzberg/html-to-markdown` to fail building because `@kreuzberg/html-to-markdown-node` wasn't available yet. Added workspace dependency reinstallation step to ensure pnpm correctly resolves the local package after publishing.
- **Go FFI library installation**: Fixed critical bugs in the `go:generate` install script that prevented automatic FFI library downloads:
  - Corrected artifact naming from `go-ffi-{platform}.tar.gz` to `html-to-markdown-ffi-{version}-{platform}.tar.gz`
  - Fixed platform mapping to match GitHub release artifacts (darwin-arm64, linux-x64, etc.)
  - Added support for all library formats (.dylib for macOS, .so for Linux, .dll for Windows)
- **Ruby native Cargo.toml**: Fixed workspace dependency configuration to use `workspace = true` instead of vendored path reference, preventing Cargo workspace resolution failures during builds.
- **CI workflows**: Upgraded all CI workflows from Java 24 to Java 25 to match maven.compiler.release=25 configuration in pom.xml.
- **Go linting**: Resolved golangci-lint warnings by adding constants for OS names and library names, and converting if-else chains to switch statements.

### Changed

- **Go README**: Updated installation documentation to explain the `go:generate` workflow for automatic FFI library installation, including details about caching in `~/.html-to-markdown/` and alternative manual configuration.

## [2.23.1] - 2026-01-19

### Fixed

- **Go module versioning**: Created 14 missing Go module tags (packages/go/v2.16.1, v2.19.1-v2.19.8, v2.20.1, v2.21.1, v2.22.1-v2.22.5) to ensure all versions since v2.15.0 are available via Go proxy. Users can now `go get` any version from v2.15.0 onwards.
- **TypeScript wrapper publishing**: Added missing `publish-typescript` job to publish workflow to properly publish `@kreuzberg/html-to-markdown` TypeScript wrapper package to npm alongside the native Node.js bindings (`@kreuzberg/html-to-markdown-node`).
- **Ruby gem vendoring**: Fixed Ruby gem installation failures due to missing `.cargo-checksum.json` files. Updated gemspec to include hidden files with `File::FNM_DOTMATCH` flag, and improved vendoring script to generate checksums correctly with `--locked` flag and proper cleanup.
- **Elixir package size**: Reduced Hex package size from 134 MB to under 128 MB limit by aggressively removing unnecessary files from vendored dependencies (tests, docs, examples, static libraries, Windows-only crates on Unix builds).

### Added

- **Go automatic FFI library installation**: Implemented `go:generate` pattern following Kreuzberg approach. Added `cmd/install` package that automatically downloads platform-specific FFI libraries from GitHub releases and generates CGO flags. Users can now run `go generate` after installation instead of manually setting `CGO_CFLAGS` and `CGO_LDFLAGS` environment variables. FFI loader updated to check `~/.html-to-markdown/` for installed libraries.

## [2.23.0] - 2026-01-18

### Added

- **Djot output format support**: New `output_format` option in `ConversionOptions` enables conversion to [Djot](https://djot.net/) lightweight markup language as an alternative to Markdown. Djot uses different syntax for emphasis (`_text_`), strong (`*text*`), strikethrough (`{-text-}`), inserted (`{+text+}`), highlighted (`{=text=}`), subscript (`~text~`), and superscript (`^text^`).
- **CLI**: Added `--output-format` / `-f` flag to specify output format (`markdown` or `djot`)
- **All language bindings**: OutputFormat enum/option added to Python, TypeScript/Node.js, Ruby, PHP, Elixir, Go, Java, and C# bindings
- **Documentation**: Added Djot output format section to all package READMEs with syntax comparison table

### Fixed

- **Python**: Fixed async visitor bridge to properly await coroutines. `PyAsyncVisitorBridge::call_visitor_method_sync()` now detects async methods via `__await__` attribute and uses `PYTHON_TASK_LOCALS` event loop for proper async execution (issue #187)
- **Ruby**: Fixed visitor parameter being ignored in `convert()` wrapper method. Now correctly passes visitor to native `convert_with_visitor` function when provided (issue #187)

### Changed

- **Rust**: Updated `async-visitor` feature to include required `tokio` "sync" feature for `Mutex` support
- **Documentation**: Added comprehensive visitor pattern support matrix showing which bindings support visitors
- **Documentation**: Documented WASM visitor pattern architectural limitation with four alternative approaches

## [2.22.6] - 2026-01-16

### Fixed

- **Ruby gem dependency resolution**: Ruby native extension now uses workspace version inheritance with vendoring approach. During gem build, the entire `html-to-markdown` crate is vendored with exact dependency versions into `packages/ruby/vendor/`, making gems completely self-contained and eliminating crates.io dependency resolution during installation. Local development uses symlink to workspace crate for seamless workflow.
- **URL parsing robustness**: Fixed IPv6 URL parsing error when processing malformed markdown-like URLs in HTML attributes (e.g., `//[domain.com/path](http://domain.com/path)`). New `sanitize_markdown_url()` function detects and extracts actual URLs from markdown syntax that wasn't properly converted in source HTML. Applied to both link `href` and image `src` attributes (fixes issue #186).

### Changed

- **Ruby gem build process**: Added `vendor-html-to-markdown.sh` script that creates standalone vendor workspace before gem packaging. Ruby native `Cargo.toml` now references vendored path for maximum reproducibility and build reliability.

## [2.22.5] - 2026-01-16

### Fixed

- **Core**: Added `#[serde(default)]` attribute to `ConversionOptions` struct to enable partial JSON deserialization. This allows deserializing JSON with only a subset of fields specified, using default values for missing fields. Fixes compatibility with language bindings (C#, Go, Java) that serialize partial configuration objects.

## [2.22.4] - 2026-01-15

### Fixed

- **Core**: Fixed `br_in_tables` option not being respected correctly. HTML `<br>` tags in table cells now properly convert to markdown line breaks (spaces or backslash style based on `newline_style` option), while block elements (divs, paragraphs) continue to generate literal `<br>` tags when needed for rowspan scenarios (issue #184)
- **WASM**: Updated GitHub Pages demo to v2.22.4 with latest BR tag handling fixes

## [2.22.3] - 2026-01-14

### Fixed

- **Python**: Exposed `skip_images` option in `ConversionOptions` API, including type stub files (.pyi) for proper type checking support (issue #183)
- **Elixir**: Added `skip_images` option to `HtmlToMarkdown.Options` module (was completely missing from Elixir binding)
- **Core**: Fixed `<br>` tags being output literally in table cells instead of converting to proper Markdown line breaks. Table cell paragraph and div separators now respect `newline_style` option (issue #184)

## [2.22.2] - 2026-01-13

### Fixed

- **Ruby gem standalone build** - Fixed Ruby gem failing to build when installed from RubyGems. Removed `lints.workspace = true` (which requires workspace context) and added inline lint configuration. This resolves issue #181.
- **Ruby gem version pinning** - Changed `html-to-markdown-rs` dependency from loose semver (`"2.x.x"`) to exact pin (`"=2.22.2"`) to prevent older gems from pulling incompatible newer crate versions.
- **Version sync script** - Updated `sync_versions.py` to preserve exact version pin prefix (`=`) when syncing Ruby gem dependencies.

## [2.22.1] - 2026-01-13

### Fixed

- **Java Maven Central publishing** - Fixed Maven Central deployment by adding proper `publish` profile with `central-publishing-maven-plugin` configuration. The plugin is now correctly activated with `-Ppublish` flag and uses `ossrh` server credentials.
- **Java Spotless formatting** - Updated google-java-format to 1.28.0 for Java 25 compatibility.

## [2.22.0] - 2026-01-13

### Fixed

- **C FFI visitor implementation** - Fixed `html_to_markdown_convert_with_visitor` to properly use the visitor handle during conversion instead of discarding it. Previously the visitor was created but the plain `convert()` function was called instead of `convert_with_visitor()`.
- **C# visitor callbacks** - P/Invoke bindings now correctly invoke visitor callbacks during HTML-to-Markdown conversion (42/42 tests passing).
- **Go visitor callbacks** - Removed regex-based post-processing workaround; Go bindings now use real FFI visitor callbacks with proper struct field ordering.
- **PHP visitor callbacks** - Wired up `PhpVisitorBridge` to pass visitor to Rust core instead of ignoring the visitor parameter.
- **Java visitor callbacks** - Added Panama FFI upcall stubs for all 38 visitor callbacks, enabling full visitor pattern support (95/95 tests passing).

### Added

- **Java `VisitorCallbackFactory`** - New class that creates Panama FFI upcall stubs for visitor callbacks, enabling Java code to receive callbacks from the Rust core during conversion.
- **Java `HtmlToMarkdown.convertWithVisitor()`** - Public API method for converting HTML with a custom visitor implementation.

## [2.21.1] - 2026-01-13

### Added

- **Serde serialization support for ConversionOptions** - Added `Serialize` and `Deserialize` traits to `ConversionOptions`, `PreprocessingOptions`, and all related structs. Enables JSON serialization/deserialization with camelCase field naming and lowercase string enum representations.

### Changed

- **Major refactor: Complete Phase 1 modular architecture** - Restructured core converter into modular handler components:
  - Extracted block element handlers (block-level HTML elements)
  - Extracted inline element handlers (2,363 lines of focused code)
  - Extracted table, list, and media handlers (2,528 lines)
  - Extracted semantic and form handlers (1,532 lines)
  - Improved code organization and maintainability across all language bindings
- **Unified FFI bindings architecture** - Consolidated common binding logic into shared crate, reducing duplication across Python, TypeScript, Ruby, PHP, Go, and Java bindings
- **Added visitor callback code generation system** - FFI now supports dynamic visitor callbacks for all language bindings (Python, Ruby, PHP, Elixir, etc.)
- **Enhanced preprocessing system** - Footer and nav element removal now integrated into preprocessing pipeline
- **Improved custom element detection** - Enhanced `has_custom_element_tags` to accurately detect only tag names with hyphens

### Internal

- Updated dependencies across all language bindings (Python, Ruby, PHP, JavaScript, Go, etc.)
- Refactored benchmark harness to modularize script adapters and reduce code duplication
- Refactored performance examples to extract and reuse shared utilities
- Improved sync_versions.py to handle all internal workspace dependency version pins
- Refactored README generation script to modularize template handling
- Improved clippy lint handling and CI coverage workflows
- Added documentation to Node.js binding example files

## [2.21.0] - 2026-01-10

### Added

- **`skip_images` configuration option** - New option to skip all `<img>` elements during conversion, enabling greater control over image handling in the output.
- **Optional visitor parameter across all convert functions** - Unified API for applying visitor patterns to all conversion modes:
  - `convert(html, options, visitor)` - Basic conversion with optional visitor
  - `convert_with_inline_images(html, options, image_cfg, visitor)` - Inline image extraction with optional visitor
  - `convert_with_metadata(html, options, metadata_cfg, visitor)` - Metadata extraction with optional visitor
- **Visitor pattern integration with advanced features** - Support for using visitor pattern simultaneously with inline images and metadata extraction, providing complete control over the conversion process.
- **Comprehensive test coverage** - Added tests validating `skip_images` functionality and visitor pattern integration across all conversion functions and language bindings.

### Changed

- **Visitor parameter unified across all APIs** - The visitor parameter is now optional on all conversion functions, enabling consistent API design across basic, inline-images, and metadata extraction paths.
- **Improved feature-gated architecture** - Refined the feature gate handling for better flexibility when combining visitor patterns with other optional features.

### Deprecated

- **`convert_with_visitor()` function** - Deprecated in favor of passing visitor as an optional parameter to `convert()`. The dedicated function will be removed in a future major release. Use `convert(html, options, visitor)` instead.

### Fixed

- **Unused dependency warnings in npm packages** - Resolved unused dependency warnings reported during builds of JavaScript/TypeScript packages.
- **Feature gate handling for visitor combinations** - Fixed issues with feature gate combinations when using visitor patterns alongside inline images and metadata extraction.

## [2.20.1] - 2026-01-09

### Code Quality

- **Resolved all clippy warnings comprehensively**: Fixed 207+ clippy pedantic/nursery warnings across entire workspace
  - Removed blanket `#![allow(clippy::pedantic)]` directives from all crate roots
  - Fixed trivial copy pass-by-ref issues in converter functions
  - Added missing documentation sections (# Errors) to public APIs
  - Fixed doc markdown formatting (added backticks to technical terms)
  - Applied selective allows only for architecturally justified cases
  - FFI/binding layers use targeted allows due to interop constraints
  - Core library maintains strict clippy compliance
- **Updated workspace lint configuration**: Changed pedantic lints from deny to warn to allow module-level selective overrides
- **Dependency modernization**: Migrated from `once_cell::sync::Lazy` to stdlib `std::sync::LazyLock` (stabilized in Rust 1.80+)

## [2.20.0] - 2026-01-05

### Dependencies

- **Updated reqwest to 0.13.1**: Migrated to new rustls defaults
  - rustls is now the default TLS backend (previously native-tls)
  - aws-lc is the default crypto provider (previously ring)
  - rustls-platform-verifier is used by default for root certificates
  - All reqwest features updated to new naming conventions
- **Updated development dependencies**: Updated pnpm packages, Ruby gems, and pre-commit hooks
  - oxlint pre-commit hook updated from v1.36.0 to v1.37.0
  - All language bindings dependencies refreshed

### Infrastructure

- **Fixed C# package update task**: Updated dotnet list command to specify project files explicitly
  - Prevents "project or solution file could not be found" errors
  - Now checks both HtmlToMarkdown.csproj and HtmlToMarkdown.Tests.csproj individually

## [2.19.8] - 2026-01-05

### Bug Fixes

- **Blockquote newline preservation**: Fixed Issue #176 - Newlines were not preserved when block elements like `<strong>` were directly adjacent to `<blockquote>` elements
  - Blockquotes now add proper spacing before and after themselves
  - Fixed blockquote+paragraph spacing to match CommonMark spec
  - Fixed blockquote+HR spacing to avoid extra newlines
  - Added comprehensive regression tests to prevent future regressions
  - Maintains CommonMark compliance (132/132 tests passing)

### Improvements

- **Debug logging cleanup**: Removed extensive debug logging from hOCR processing and core converter
  - Removed ~30 debug eprintln! statements that were spamming output
  - Removed unused debug parameters from hOCR functions (parse_properties, reconstruct_table, extract_hocr_document, etc.)
  - Cleaner output and reduced noise during HTML to Markdown conversion

## [2.19.7] - 2026-01-03

### Improvements

- **Homebrew bottle CI debugging**: Added verification steps to diagnose artifact upload/download issues
  - Added verification after bottle creation to confirm file exists in workspace
  - Added `if-no-files-found: error` to fail fast if bottle file not found during upload
  - Added verification after artifact download to show what was actually retrieved
  - These steps will help identify why Homebrew bottle artifacts aren't being found in release workflow

## [2.19.6] - 2026-01-03

### Bug Fixes

- **WASM npm package publishing**: Fixed Issue #172 - WASM package was published with only 3 files (LICENSE, package.json, README.md) instead of 25 files
  - Root cause: publish workflow downloaded WASM artifact tarballs but never extracted them before running `npm publish`
  - Added extraction step in `.github/workflows/publish.yaml` to unpack dist/, dist-node/, and dist-web/ directories
  - Added safeguard to remove .gitignore files from dist directories that could exclude content
  - Complete package now includes all WASM binaries and JavaScript wrappers (7.8 MB unpacked)

## [2.19.5] - 2025-01-02

### Bug Fixes

- **Homebrew bottle naming**: Fixed bottle filename format to match Homebrew convention
  - Changed from double-dash (`html-to-markdown--2.19.x`) to single-dash (`html-to-markdown-2.19.x`)
  - Homebrew constructs bottle URLs based on formula name and version, expecting single dash separator
  - Fixes bottle download failures when installing via `brew install`

## [2.19.4] - 2025-01-02

### Bug Fixes

- **Homebrew formula publishing**: Fixed publish workflow script that updates the Homebrew tap formula
  - Corrected bottle block deletion regex (was looking for `# bottle do` instead of `bottle do`), preventing duplicate bottle blocks from accumulating on each release
  - Added automatic source tarball SHA256 computation and formula update to ensure correct checksums
  - Formula now properly replaces old bottle blocks with new ones rather than appending

## [2.19.3] - 2025-01-02

### Bug Fixes

- **Table image processing**: Fixed Issue #175 - images inside Blogger-style HTML tables (e.g., `<table class="tr-caption-container">`) were being stripped during conversion. Enhanced table scanner to recognize images as content and properly process non-table elements like `<a>` and `<img>` that are direct children of table elements.
- **WASM npm package**: Fixed Issue #172 completely - package was published but missing all WASM binaries and JavaScript wrappers (only 23 KB with 3 files). Created `.npmignore` to include `dist/`, `dist-node/`, and `dist-web/` directories that were excluded by `.gitignore` during npm publish.
- **PHP Packagist publishing**: Fixed version mismatch that caused Packagist to reject v2.19.2 tag. Updated `sync_versions.py` to synchronize both root `composer.json` and `packages/php/composer.json`.
- **Test apps**: Fixed relative fixture paths in C#, Java, and Elixir test apps. Updated Elixir tests to handle tuple-returning API. Added Java native library path configuration.

### Infrastructure

- Enhanced `sync_versions.py` script to update root `composer.json` for Packagist validation
- Recreated v2.19.2 git tag with correct composer.json version

## [2.19.2] - 2025-12-30

### Bug Fixes

- **WASM npm package**: Fixed missing `.d.ts` files in published package by updating `files` field with glob patterns (fixes #172)
- **Test apps**: Fixed API mismatches across all language test apps (Python, Node.js, WASM, Go, Java, C#)
  - Python: Changed `convert_html_to_markdown()` to `convert()`
  - Node.js: Updated to scoped package `@kreuzberg/html-to-markdown`
  - WASM: Changed `convertHtmlToMarkdown()` to `convert()`
  - Go: Updated FFI version from 2.16.0 to 2.19.1 with enhanced error handling
  - Java: Added Maven wrapper files for portability
  - C#: Updated to `KreuzbergDev.HtmlToMarkdown` package name
- **Packagist publishing**: Added automated workflow job and moved `composer.json` to repository root
- **Maven Central publishing**: Fixed GitHub secrets configuration (corrected `GPG_PASSPHRASE` typo)
- **Go bindings**: Enhanced FFI download error messages with actionable troubleshooting guidance
- **Pre-commit hooks**: Fixed Go linting errors (errcheck, staticcheck) and formatting violations

### Infrastructure

- Created new WASM test app with comprehensive smoke and integration tests
- Updated all test apps to version 2.19.0 for consistent validation
- Enhanced Java package formatting to comply with 120-character line limit

## [2.19.1] - 2025-12-29

### Bug Fixes

- **Go formatting**: Applied `gofmt` to `packages/go/v2/htmltomarkdown/visitor.go` to align constant declarations
- **Java tooling**: Upgraded google-java-format from 1.21.0 to 1.25.2 for Java 25 compatibility
- **Homebrew distribution**: Added html-to-markdown formula to kreuzberg-dev homebrew tap for CLI installation

## [2.19.0] - 2025-12-29

### Breaking Changes

- **npm package namespace**: All npm packages now use the `@kreuzberg` scope for better organization and discoverability
  - `html-to-markdown-node` → `@kreuzberg/html-to-markdown-node`
  - `html-to-markdown-wasm` → `@kreuzberg/html-to-markdown-wasm`
- **Java package namespace**: Java binding now uses `dev.kreuzberg` package prefix instead of `com.goldziher`
  - Updated all Maven artifact IDs and Java package names for semantic clarity
  - Affects all public classes and imports in Java projects
- **C# namespace**: C# bindings now use `KreuzbergDev` namespace instead of `Goldziher`
  - Updated NuGet package ID to `KreuzbergDev.HtmlToMarkdown`
  - All public types now under `KreuzbergDev.HtmlToMarkdown` namespace

### Features

- **XML table support (TEI/JATS formats)**: Added support for TEI (Text Encoding Initiative) and JATS (Journal Article Tag Suite) table elements
  - `<row>` elements for table rows with proper cell grouping and nesting
  - `<cell>` elements with full attribute support including `role="head"` for header cells
  - `<graphic>` elements for figure/image references within cells and content blocks
  - Proper table structure preservation when converting scientific markup formats
  - Aligns with CommonMark table output while respecting source document semantics

### Bug Fixes

- Fixed Clippy warnings across Rust core and all binding crates for cleaner compilation
- Improved test suite with enhanced error messages and edge case coverage
- Refined table element handling for robustness with malformed markup

### Infrastructure

- **CI/CD improvements**: Enhanced C# workflow for improved reliability and platform coverage
- **Release distribution**: Added Homebrew bottle support for macOS CLI binary distribution
- **Version synchronization**: All language bindings now synchronized to v2.19.0

## [2.18.0] - 2025-12-28

### Added

- **Visitor Pattern**: Complete implementation of visitor pattern for custom HTML element processing across all 8 language bindings (Python, TypeScript, Ruby, PHP, Go, Java, C#, Elixir)
  - Synchronous and asynchronous visitor support (where applicable per language)
  - 40+ visitor methods with hooks for every HTML element type (text, links, images, headings, lists, tables, code blocks, and more)
  - `NodeContext` provides element metadata: tag name, attributes, depth, parent tag, inline status, and sibling index
  - Control flow options: Continue, Custom (provide custom markdown), Skip, PreserveHtml, or Error
  - Element lifecycle callbacks: `visit_element_start` and `visit_element_end` for complete control
  - **Python**: Full async visitor support with `convert_with_async_visitor()` function
  - **TypeScript**: Async visitor with full type definitions
  - **Ruby**: Sync visitor implementation with complete RBS type definitions
  - **PHP**: Full visitor support with PHPStan level 9 compliance
  - **Go**: Thread-safe visitor registry with markdown post-processing
  - **Java**: Panama FFI visitor (JDK 21+)
  - **C#**: P/Invoke visitor with cross-platform compatibility
  - **Elixir**: Rustler NIF visitor implementation

### Fixed

- **HTML parsing for modern websites**: Fixed issue where JavaScript-heavy websites (like Reuters) would lose article body content during conversion (GitHub issue #167)
  - The parser was incorrectly interpreting HTML-like strings inside `<script>` tags as actual HTML elements
  - Script and style tags are now properly stripped during preprocessing while preserving JSON-LD metadata
  - No performance impact on conversion speed
- **Python API**: Fixed missing `ConversionOptionsHandle` export in public API (GitHub issue #166)
  - Users can now import `ConversionOptionsHandle` directly from the `html_to_markdown` package
  - Maintains backward compatibility with existing `OptionsHandle` import

## [2.17.0] - 2025-12-22

### Added

- Go binding now auto-downloads the native FFI library from GitHub Releases with cache/override controls.
- Release pipeline now publishes per-platform Go FFI artifacts for Go installs.

## [2.16.1] - 2025-12-22

### Fixed

- Fast-path plain-text conversions now honor escape flags (asterisks/underscores/misc/ASCII).
- Fast-path plain-text conversions now normalize whitespace and trim trailing spaces.
- Fast-path plain-text conversions now respect `strip_newlines`.
- Python CLI proxy now only applies v1 translation defaults when v1-only flags are present.

## [2.16.0] - 2025-12-22

### Added

- Profiling harness and workflow for Rust core and bindings with consolidated flamegraph output.
- Benchmark scenarios for inline images, metadata extraction, and raw metadata output across fixtures.
- WASM profiling support with warmups and stable flamegraph parsing.
- FFI byte-based conversion path plus metadata-raw benchmark coverage.

### Changed

- Bench harness now supports expanded fixture coverage and results consolidation.
- Java benchmarks align on JDK 25 for consistent profiling runs.

### Fixed

- Node benchmark harness now runs from the package directory and uses native bindings.
- Profiling stability fixes across Go, Elixir, Java, and WASM adapters.
- Binary input detection now flags compressed/magic signatures and UTF-16 data with clearer errors.

### Performance

- Rust core conversion: metadata extraction, inline image handling, tag/whitespace caches, and text assembly hot paths.
- Bindings interop: tighter metadata serialization/deserialization paths.
- Rust bench harness (local, Apple M4): median ops/sec improved 18.8× on Wikipedia fixtures (53.7 → 1009.1).

## [2.15.0] - 2025-12-19

### Fixed

- Rust core: clamp table `colspan`/`rowspan` to prevent pathological allocations on malformed HTML.
- Rust core: reject binary-like inputs early to avoid OOMs when non-HTML data is passed to `convert`.

## [2.14.11] - 2025-12-16

### Fixed

- C# (NuGet): fix `ConvertWithMetadata()` deserialization for metadata enums (`link_type`, `image_type`, `data_type`, `text_direction`) by honoring the JSON wire values.

## [2.14.10] - 2025-12-16

### Fixed

- Python: release the GIL during native conversion so `ThreadPoolExecutor` parallelism doesn't regress performance, and always build the extension with metadata support (so `convert_with_metadata` is always available).

## [2.14.9] - 2025-12-16

### Fixed

- Structured data: JSON-LD is now extracted from `<script type="application/ld+json">` tags (including when placed in `<head>`), preserving the script contents for parsing.

## [2.14.8] - 2025-12-15

### Fixed

- Rust crate (`html-to-markdown-rs`): enable the `metadata` feature by default so `convert_with_metadata` is available without extra Cargo features.

## [2.14.7] - 2025-12-15

### Fixed

- Elixir (macOS): package now ships a `.cargo/config.toml` so Rustler can compile without requiring user-specific linker flags.

## [2.14.6] - 2025-12-15

### Fixed

- RubyGems publish: skip duplicate `ruby`-platform gems when multiple CI jobs produce identical artifacts for the same version.
- Hex publish: ensure the Rust core crate is staged into the Elixir package before publishing.

## [2.14.5] - 2025-12-15

### Fixed

- RubyGems publish: prevent corrupted gem pushes by downloading `rubygems-*` artifacts into separate directories (no merge), and publishing gems recursively with an integrity check.

## [2.14.4] - 2025-12-15

### Fixed

- Release pipeline: build the C# `osx-x64` native FFI library on `macos-15-intel` (macOS-13 runners are retired), unblocking NuGet publication.
- Elixir (Hex): package now vendors the Rust core crate so `mix deps.get && mix test` works outside this monorepo.

## [2.14.3] - 2025-12-15

### Fixed

- **Issue #150 / Discord report**: Python now always exports `convert_with_metadata` (no more `ImportError` on import).
- **Issue #149**: Blockquote text now word-wraps when `wrap=true`.
- **FFI JSON parity**: Metadata enums now serialize as snake_case (e.g. `external`, `relative`) to match cross-language expectations.
- PHP test runner now always builds the extension with the `metadata` feature enabled (avoids missing `html_to_markdown_convert_with_metadata` when the workspace was built with `--no-default-features`).

### Added

- Elixir: `convert_with_metadata/3` + `MetadataConfig` backed by the Rust metadata extractor.

### Changed

- WASM: metadata bindings are enabled by default so the published npm package exports `convertWithMetadata`.
- C# publish pipeline: stage native `html_to_markdown_ffi` libraries into the NuGet package under `runtimes/*/native`.
- Go: module path now uses semantic import versioning (`.../packages/go/v2`), and docs/examples were updated accordingly.
- Java: add `.sdkmanrc` for Java 25 + Maven 4; keep `maven-source-plugin` on `3.3.1` because `4.0.0-beta-1` is not compatible with Maven `4.0.0-rc-4`.

## [2.14.2] - 2025-12-13

### Changed

- CI/release automation: extracted Maven installer logic into `scripts/common/install-maven-latest.sh` and applied repo-wide lint/format cleanups.

## [2.14.1] - 2025-12-12

### Fixed

- **Issue #147**: Word wrap now works correctly in list items when using the `-w`/`--wrap` flag. List items with long text are properly wrapped while preserving list structure and indentation for both ordered and unordered lists.
- **Issue #146**: `strip_tags` and `preserve_tags` options now correctly prevent `<meta>` and `<title>` tags from being extracted into YAML frontmatter when `extract_metadata` is enabled.
- **Issue #145**: `strip_newlines=true` no longer causes excessive whitespace around block elements. Structural whitespace is now properly normalized while still removing newlines within paragraph content.

## [2.14.0] - 2025-12-11

### Added

- **CLI Metadata Extraction**: New `--with-metadata` flag with JSON output support for extracting document metadata, headers, links, images, and structured data from HTML documents.
  - Six extraction flags: `--extract-document`, `--extract-headers`, `--extract-links`, `--extract-images`, `--extract-structured-data`
  - JSON output format with markdown and metadata fields: `{"markdown": "...", "metadata": {...}}`
  - Feature enabled by default in CLI builds
- **Go FFI Binding**: Complete `ConvertWithMetadata()` function with typed structs for metadata extraction.
  - 12 Go struct types with JSON tags for type-safe metadata access
  - JSON unmarshaling from FFI layer
  - 18 comprehensive tests covering all metadata types
- **Java FFI Binding**: Complete `convertWithMetadata()` method with Java records for metadata extraction.
  - 11 Java record types using Panama FFM for FFI integration
  - Proper enum types for link/image/text direction (no string-based parsing)
  - Jackson JSON deserialization with error handling
  - 33 comprehensive tests including negative test cases
- **C# FFI Binding**: Complete `ConvertWithMetadata()` method with C# records for metadata extraction.
  - 11 C# record types using P/Invoke for FFI integration
  - System.Text.Json deserialization with proper error handling
  - 23 comprehensive tests covering all metadata types
- **FFI Core API**: New `html_to_markdown_convert_with_metadata()` C function for language-agnostic metadata extraction.
  - JSON serialization for cross-language compatibility
  - Proper memory management and error handling
  - 17 comprehensive tests including memory safety tests

### Changed

- **Documentation Consolidation**: Migrated all standalone METADATA.md files into binding READMEs for improved maintainability.
  - Deleted `packages/typescript/METADATA.md` (480 lines) and `packages/ruby/METADATA.md` (228 lines)
  - Enhanced Python, PHP, TypeScript, Ruby, Go, Java, and C# READMEs with comprehensive metadata sections
  - Root README now includes CLI metadata examples and links to all binding documentation
  - Each binding README is now self-contained with full metadata documentation
- **Type Definitions**: Enhanced metadata type definitions across all language bindings.
  - Go: Complete struct types with JSON tags and godoc comments
  - Java: Proper enum types (LinkType, ImageType, TextDirection) instead of strings
  - C#: Complete record types with XML documentation
  - Python: Fixed `max_structured_data_size` default (100KB → 1MB)
  - TypeScript: Verified dimensions field type (Array<number> for compatibility)
- **Docstrings**: Enhanced documentation strings across all language bindings.
  - Rust core: Improved function and module documentation
  - Python: Enhanced PyO3 docstrings with examples and type hints
  - Ruby: Added YARD tags for better documentation generation
  - PHP: Enhanced docblocks with detailed parameter descriptions

### Fixed

- **FFI Memory Safety**: Fixed critical memory safety bug where error paths could leave dangling metadata pointers.
  - Both markdown and metadata pointers now set to null on any error
  - Added comprehensive memory safety tests
- **CLI Flag Implementation**: Fixed `--extract-document` flag not being mapped to MetadataConfig.
  - Flag now correctly controls document metadata extraction
  - Added 9 new CLI tests for metadata flags
- **Java Type Safety**: Fixed metadata loss and silent failures from missing fields and string-based enums.
  - Added dimensions field to ImageMetadata (was missing, causing 50% metadata loss)
  - Changed linkType, imageType, textDirection from String to proper enum types
  - Fixed exception swallowing in getLastError() - now logs errors and returns descriptive messages
- **Python Default Values**: Fixed incorrect `max_structured_data_size` default (was 100KB, should be 1MB).
  - Now uses `DEFAULT_MAX_STRUCTURED_DATA_SIZE` constant from Rust core
- **Constants Extraction**: Eliminated DRY violations by extracting hardcoded magic numbers.
  - Added `DEFAULT_MAX_STRUCTURED_DATA_SIZE: usize = 1_000_000` constant in Rust core
  - Reused across FFI, CLI, and Python bindings

### Technical Details

- **Test Coverage**: Added 55 new tests across all bindings (71 → 126 total tests, 77% increase)
  - FFI: 13 new tests (4 → 17 total)
  - CLI: 9 new tests (67 → 76 total)
  - Java: 33 new tests (0 → 33 total)
  - Go: 18 tests total
  - C#: 23 tests total
- **Language Compliance**: Achieved 100% compliance across all bindings (up from 50%-100% range)
  - All bindings now correctly implement metadata extraction with proper types
  - Standardized error handling and JSON parsing patterns
- **Documentation**: Added 3,500+ lines of comprehensive metadata documentation across all binding READMEs
  - Migrated 708 lines from TypeScript and Ruby METADATA.md files
  - Enhanced Python and PHP READMEs with extensive examples
  - Added metadata sections to Go, Java, and C# READMEs

## [2.13.0] - 2025-12-10

### Added

- Comprehensive metadata extraction API across all language bindings (Python, TypeScript, Ruby, PHP, WASM).
- New `convert_with_metadata()` function returning both markdown and extracted metadata in a single pass.
- Metadata extraction includes: document metadata (title, description, keywords, author, language, Open Graph, Twitter Card), header hierarchy (h1-h6 with IDs and nesting), link classification (internal/external/anchor/email/phone), image metadata with type detection (data URIs, inline SVGs, external, relative), and structured data (JSON-LD, Microdata, RDFa).
- Python: 51 comprehensive integration tests with full TypedDict type stubs and mypy validation.
- TypeScript: 14 vitest tests with auto-generated NAPI types, runtime feature detection via `hasMetadataSupport()`, and 600+ lines of documentation.
- Ruby: 40+ RSpec tests with complete RBS type signatures and comprehensive API documentation.
- PHP: 21 PHPUnit tests with PHPStan level max compliance and readonly Value Objects.
- WASM: Complete metadata extraction with serde_wasm_bindgen serialization and getter/setter configuration structs.

### Changed

- Enabled metadata feature by default in TypeScript and Ruby bindings for production npm packages and gems.
- Updated all language binding versions to 2.13.0 with synchronized version management.

### Fixed

- Ruby: Added missing wrapper method for `convert_with_metadata` and fixed redundant `?` symbols in RBS type annotations.
- TypeScript: Enabled metadata feature in default Cargo features to ensure npm packages include metadata functionality.
- WASM: Fixed 3 clippy style violations (Default trait implementation, unwrap_or_default usage, struct initialization pattern).

## [2.12.1] - 2025-12-09

### Fixed

- Escape literal `|` characters inside table cells while leaving pipes inside `<code>` and `<pre>` untouched to avoid rendering backslashes in code spans/blocks (fixes #140).
- Handle nested tables without double-escaping pipes and add regression coverage for table cells containing code spans/blocks and nested tables.
- Preserve link-only list items when word wrapping is enabled so nested link lists are not merged or reflowed (fixes #143); added regression fixtures for the reported table-of-contents sample.

### Changed

- Updated dependency locks/manifests to align with the 2.12.1 release.
- Downgraded Java Maven compiler/source plugins back to 3.x to keep CI builds compatible with Maven 3 runners.

## [2.12.0] - 2025-12-08

### Added

- WebAssembly bundler target now supports Cloudflare Workers, Wrangler, and modern bundlers that provide `WebAssembly.Module` instead of `WebAssembly.Instance`.
- Three new WASM usage examples demonstrating different deployment targets:
  - `examples/wasm-node`: Node.js example using dist-node target
  - `examples/wasm-rollup`: Browser example using dist-web target with Rollup
  - `examples/wasm-cloudflare`: Cloudflare Workers example using bundler target with Wrangler

### Changed

- WASM bundler entry point now detects and handles `WebAssembly.Module` instances, building the proper import namespace for wasm-bindgen glue functions.

## [2.11.4] - 2025-12-08

### Fixed

- Node/WASM bundles now post-process their generated JS files to import the shared `WasmConversionOptions` typedef and emit typed doc comments (including typed inline-image `attributes`), so no `any` annotations leak into the published `dist`, `dist-node`, `dist-web`, or docs bundles.

## [2.11.3] - 2025-12-08

### Fixed

- Prevent link-label truncation from splitting multi-byte characters, which previously triggered a `PanicException` in the Python bindings when processing long anchors (resolves #139) and add a regression test to keep the truncation logic safe.

## [2.11.2] - 2025-12-07

### Added

- Explicitly ship typing artefacts in every binding: npm packages export `.d.ts` files by default, Ruby gems now include `sig/**/*.rbs` even when building outside git, and the Python wheel bundles `_html_to_markdown.pyi` plus a `py.typed` marker for static type checkers.

### Fixed

- Cleaned up the Python API’s inline-image helper to avoid redundant casts flagged by `mypy --strict`.
- Tightened PHP docblocks and psalm/phpstan annotations so option arrays use strongly typed shapes instead of `array<string, mixed>`.
- Hardened the WASM, Node, and Python bindings so their `options` argument is fully typed end-to-end (no `any` escapes in `.d.ts` files or placeholder `Any` annotations).

## [2.11.1] - 2025-12-05

### Fixed

- Preserve indentation in `<pre><code>` blocks while safely dedenting whitespace across multibyte characters to avoid panics when leading spaces are non-ASCII; regression fixture added for issue #134. Thanks @bbeardsley for the contribution.

## [2.11.0] - 2025-12-04

### Added

- CLI `--url` flag with optional `--user-agent` override to fetch remote HTML directly, plus charset-aware decoding.
- New GitHub Pages deploy workflow to publish the `docs/` demo from `main`.
- Additional CLI integration tests covering URL fetching (including custom UA, legacy markup, frameset/noframes, cp1252 decoding).

### Changed

- Demo layout now keeps input/output panes equal height and responsive.
- Rust core handles body-like content accidentally nested in `<head>` more gracefully.

## [2.10.1] - 2025-12-02

### Fixed

- Normalize whitespace inside link labels (collapse newlines and extra spaces) so anchors with messy HTML do not emit multi-line `[]` text.
- Flatten block children inside `<a>` (e.g., headings/paragraphs nested in anchors) into a single Markdown link instead of duplicating content; regression tests added for the reported Arabic product card case.

### Changed

- Synced all workspace/package versions to 2.10.1 via `task sync-versions`.

## [2.10.0] - 2025-12-02

### Added

- Centralized panic guarding for all bindings (Python, Node, PHP, WASM, C FFI) using a shared Rust helper so panics surface as language-native errors instead of unwinding across FFI boundaries.
- C FFI now stores the last error per thread and exposes it via `html_to_markdown_last_error`, with panic and UTF-8/null input diagnostics.
- Ruby binding now uses the shared panic guard and emits consistent panic messages; specs cover panic interception across conversion entrypoints.

### Changed

- Wasmtime test harness initializes conversion options via struct literals to reduce clippy noise in CI.

### Fixed

- Rust coverage CI now forces `cargo-llvm-cov` reinstall to avoid cached binary conflicts on GitHub runners.
- PHP smoke tests use the Packagist package name `goldziher/html-to-markdown`, matching README install instructions.

## [2.9.3] - 2025-12-01

### Changed

- **Version sync** – Bumped the entire workspace (Rust, Python, npm, Ruby, Elixir, Java, C#, Go) to 2.9.3 via `task sync-versions` to prep the next patch release.
- **Docs & install commands** – Pointed all Composer references to the published `goldziher/html-to-markdown` package and clarified npm usage to the shipped packages (`html-to-markdown-node` / `html-to-markdown-wasm`).

### Fixed

- **Go lint CI** – Replaced the invalid `go fmt -l` invocations with `gofmt -l` in the Taskfile so `task check`/CI lint runs complete successfully on Go 1.25.

## [2.9.2] - 2025-11-28

### Fixed

- **UTF-8 safety (Fix #127)** – Guarded whitespace trimming against mid-codepoint truncation, eliminating byte-boundary panics on multilingual documents; added fixture and regression test for the reported Ruby-path crash.
- **Image conversion (Fix #128)** – `<img>` elements with `width`/`height` now render as Markdown images instead of raw HTML; regression test covers inline-data URIs with dimensions.

## [2.9.1] - 2025-11-22

### Changed

- **HTML repair fallback** – Minified or malformed pages now reparse via html5ever when inline/block nesting is broken, keeping content that previously vanished (e.g., SPA shells and Hacker News markup).
- **Link label recovery** – Anchor text fallback prefers child formatting or hrefs only when appropriate, preventing empty labels while keeping CommonMark empty-link semantics intact.

### Fixed

- **Layout tables to lists** – Headless tables with mixed column counts/spans or nested tables render as list rows instead of broken Markdown tables, restoring Hacker News output.
- **Issue 121 regressions** – Added fixtures/tests for the empty SPA and malformed Hacker News samples; both now produce full Markdown content without frontmatter noise.

## [2.9.0] - 2025-11-20

### Added

- **Elixir bindings** – New `html_to_markdown` Hex package built with Rustler, exposing the Rust core converter to Elixir with configurable options plus `convert/2` and `convert!/2`.
- **WASM runtime verification** – Added a Wasmtime-backed e2e suite (`e2e/wasm-wasmtime`) plus `task wasm:test:wasmtime` to compile the `html-to-markdown-wasm` artefact for `wasm32-unknown-unknown` and execute it inside Wasmtime. CI now runs these tests to ensure the WASM package works outside the browser runtime.

### Changed

- **Astral `tl` parser** – The HTML parser dependency now points to the actively maintained `astral-tl` fork (still imported as `tl`) so comment parsing stays up to date with upstream fixes.
- **NuGet Package ID** – C# bindings now publish under `Goldziher.HtmlToMarkdown` to avoid clashing with an existing community package.
- **Wasmtime CI Coverage** – The Wasmtime e2e job now runs on Linux x64, Linux arm64, macOS, and Windows runners so every GitHub-hosted architecture executes the WASM tests.

### Fixed

- **PHP PIE source bundle** – Release packaging strips the Wasmtime e2e workspace from the staged `Cargo.toml`, fixing the “failed to load manifest” error in the publish workflow.
- **Horizontal rule rendering** – `<p>…</p><hr>` now emits a blank line before `---` while preserving blockquote spacing so the rule is never misinterpreted as a setext heading.
- **Empty HTML comments** – Zero-width `<!---->` comment nodes are normalized before parsing, so comment placeholders no longer cause the following content to disappear.

## [2.8.3] - 2025-11-15

### Changed

- **Deterministic uv installs** – Every `uv sync` invocation in CI and the Taskfile now runs with `--no-install-workspace`, ensuring Python dependencies are resolved without mutating editable installs before the subsequent build/test steps run.

### Fixed

- **NuGet Publishing** – Release automation now uses GitHub’s trusted publisher flow via `NuGet/login@v1` (OIDC → short-lived API key) before pushing artifacts, removing the dependency on long-lived secrets.
- **Hex Publishing** – The release workflow invokes `mix hex.publish --yes` from `packages/elixir`, with `ex_doc` bundled as a dev dependency so documentation generation works during release.

## [2.8.2] - 2025-11-15

### Changed

- **Unified Version Sync** – `scripts/sync_versions.py` now updates Elixir `@version` declarations, the C# `.csproj`, and the Java `pom.xml` (alongside every npm/pyproject/Gemfile manifest). `task sync-versions` bumps the entire multi-language stack to **2.8.2** in one shot.
- **CI / Release Toolchains** – GitHub Actions now installs Elixir dependencies ahead of Credo and runs on **Elixir 1.19 + OTP 28.1**, matching the README prerequisites and preventing per-job regex recompilation warnings.
- **Taskfile Coverage** – Added `elixir:update` plus full `java:{install,update,test,lint}` tasks so `task setup`, `task update`, `task test`, and `task lint` cover every published runtime (Go, C#, Elixir, Java) just like the CI workflows.

## [2.8.1] - 2025-11-15

### Fixed

- **Release Pipeline** – Bumped all package manifests to v2.8.1 so the publish workflow can push fresh artifacts after the v2.8.0 smoke-test fixes (PyPI, npm, and RubyGems refuse re-uploads of the same version).

## [2.8.0] - 2025-11-15

### Added

- **Java, C#, and Go Bindings (First Release)** – First public release of official Java (JNA), C# (.NET), and Go (CGO) language bindings. All three are integrated into the unified `task bench:bindings` harness and ship with comprehensive performance data in their READMEs. C# leads at ~1.4k ops/sec (≈171 MB/s), Go at ~1.3k ops/sec (≈165 MB/s), and Java at ~1.0k ops/sec (≈126 MB/s) on the 129 KB Wikipedia lists fixture.

### Changed

- **BREAKING: Preprocessing Disabled by Default** – HTML preprocessing is now disabled by default in the library API to prevent silent content loss. Previously, `<nav>`, `<form>`, and related elements (along with all their children) were dropped by default, causing important content inside these tags to be lost. Users who want preprocessing must now explicitly enable it via `PreprocessingOptions { enabled: true, ... }`. The CLI behavior is unchanged (preprocessing has always been opt-in with `--preprocess`).
- **Rust Toolchain Settings** – All crates (including the Ruby binding) now inherit `edition = "2024"` and `rust-version = "1.85"` from the workspace to keep toolchain configuration centralized.
- **GitHub Actions Workflow DRY** – Created 17 reusable composite actions (8 build actions + 9 smoke test actions) to eliminate ~267 lines of duplication between CI and publish workflows.
- **Toolchain Management** – Migrated to official GitHub Actions parameters for Ruby Bundler 2.7.2 and PHP Composer 2.9.1, removing manual installation scripts.

### Fixed

- **Windows PHP Extension Build** – Replaced php-windows-builder orchestration with direct `cargo build` matching ext-php-rs's proven approach, resolving LLVM 19 MMX header incompatibilities and Zend symbol linking errors.
- **Linux PHP Build** – Added php-config path capture and parameter passing to build-php-linux action, fixing "php-config executable not found" errors.
- **Ruby Linux Build** – Set LD_LIBRARY_PATH on Linux builds to match magnus best practices, preventing potential "strings.h not found" errors.
- **golangci-lint CI** – Split golangci-lint pre-commit hook into separate invocations for `packages/go` and `examples/go-smoke` modules, fixing "directory prefix does not contain main module" errors by running each check from within its Go module directory.
- **Windows Go CGO Smoke Test** – Documented MSVC/MinGW toolchain incompatibility and skip Windows Go smoke test with informative message, as Go CGO uses MinGW which cannot link against MSVC-compiled Rust FFI libraries.
- **Go Code Quality** – Removed redundant newline in `examples/go-smoke/main.go` fmt.Println call (detected by newly-working golangci-lint).

## [2.7.2] - 2025-11-12

### Fixed

- **Node/WASM Binding Regression** – HTML preprocessing no longer drops `<html>`, `<head>`, or `<body>` wrappers when their classes resemble navigation chrome, so large Wikipedia fixtures once again emit full markdown (restoring the Vitest length/table expectations for Node bindings and keeping WASM conversions consistent).
- **Cloudflare WASM Initialization** – Bundler builds of `html-to-markdown-wasm` now expose `initWasm()`/`wasmReady` so edge runtimes that instantiate WebAssembly modules asynchronously (Cloudflare Workers, Vite dev servers, etc.) can await initialization before calling `convert()`, eliminating the `__wbindgen_start` runtime error.
- **Footer Retention (Fix #120)** – The Rust preprocessor keeps plain `<footer>` content unless the element carries explicit navigation hints (role/class/id). Python and Rust conversions once again preserve footer copy while still stripping true navigation footers such as `.site-footer` menus.
- **Release Smoke Coverage** – The publish workflow now downloads the built artifacts (Node, WASM, Python wheels, Ruby gems, PHP zips) and reruns the README smoke installs across Linux/macOS/Windows before any packages are uploaded, ensuring we're testing the exact bits we ship.

## [2.7.1] - 2025-11-12

### Added

- **Language-Specific Benchmarks** – Every binding README (Node, WASM, Python, Ruby, PHP, TypeScript) now publishes the latest `task bench:bindings` throughput numbers so runtime documentation stays aligned with the shared fixtures.
- **Examples/Smoke Suite** – Added `examples/{node,wasm,python,ruby,php,rust}-smoke` plus an overview README to exercise both the published artifacts and local builds before a release.

### Changed

- **Docs Accuracy** – Node/WASM READMEs now clearly reference the real npm packages (`html-to-markdown-node`, `html-to-markdown-wasm`) and provide correct import samples.
- **TypeScript README** – Highlights that the CLI wrapper inherits the native Node benchmarks.
- **Repository Hygiene** – `.gitignore` now drops `.venv/`, vendor directories, and nested `node_modules/` so smoke tests and language-specific toolchains don’t dirty the tree.
- **Ruby Build Metadata** – `extconf.rb` uses a relative path for the embedded Cargo crate and the crate’s `Cargo.toml` now declares explicit `edition`, `rust-version`, and dependency pins, allowing `gem install` outside the workspace.
- **Version Sync Script** – `scripts/sync_versions.py` updates every `html-to-markdown-rs` dependency pin (workspace root plus downstream crates) to keep cross-language releases in lockstep.

### Fixed

- **Smoke Test Coverage** – Verified Node, WASM, Python, Ruby (local gem), PHP (Composer path repo), and Rust installs; documented gaps where external registries still need to publish `goldziher/html-to-markdown` or `html-to-markdown` 2.7.1 before release.

## [2.7.0] - 2025-11-12

### Added

- **Zero-Copy Inline Images** – Node/N-API and WASM bindings now expose `convertInlineImagesBuffer` / `convertBytesWithInlineImages`, letting benchmark harnesses feed `Buffer`/`Uint8Array` data directly without creating intermediate JS strings.

### Changed

- **Rust Core Preprocessing** – HTML normalization (self-closing fixes, malformed `<` escaping, script/style stripping) now happens in a single streaming pass that hands owned buffers straight to `tl::parse_owned`, cutting multiple allocations from every conversion.
- **Benchmark Harness + Docs** – Re-ran the cross-language runtime suite after the Rust core optimizations and refreshed the README tables, keeping the published throughput numbers (Node/Python/Rust/WASM/PHP) in sync with `tools/runtime-bench/results/latest.json`.
- **Version Alignment** – Bumped every package (Rust crates, npm packages, PyPI distribution, Ruby gem, PHP extension, WASM bundle) to `2.7.0` via `task sync-versions`.

### Fixed

- **Ruby Benchmark Output** – The Ruby benchmark driver now emits JSON without relying on `json` native extensions, preventing `libruby` incompatibility errors during `task bench:bindings`.
- **Nested `<strong>` Normalization (Fix #111)** – The Rust converter now tracks when bold markup is already active, so nested `<b>`/`<strong>` combinations (including `<mark>`, `<summary>`, `<legend>`) no longer generate `****` artifacts (`<b>bo<b>ld</b>er</b>` correctly becomes `**bolder**`). The CommonMark harness documents the four spec examples that expect stacked markers and skips them accordingly.
- **Heading Whitespace (Fix #118)** – ATX/Setext headings swallow layout-only newlines and indentation inside `<h1>…<h6>` so pretty-printed HTML like `<h2>Heading\n  Text</h2>` renders as a single Markdown heading line.
- **Inline Whitespace Preservation** – Reworked the inline text pipeline so removing zero-width inline elements (e.g., `<input>`, `<script>`, empty `<b>`) no longer collapses surrounding spaces; fixtures like `test_chomp`, `test_form_with_inputs_inline_mode`, and checkbox/task-list rendering now match their expected double-space gaps.
- **DOCTYPE Handling (Fix #119)** – `<!DOCTYPE …>` declarations are stripped during preprocessing so they never leak as stray `PUBLIC…` text in the output, even when metadata extraction is enabled.

## [2.6.6] - 2025-11-10

### Changed

- **Ruby Gem Packaging** – Moved the `html-to-markdown-rb` crate under `packages/ruby/ext/html-to-markdown-rb/native` and pointed `extconf.rb` at that path so every published gem now contains the Cargo sources it needs to compile on install.
- **Documentation Consistency** – Updated the root, crate, and package READMEs to drop references to the unrelated `html-to-markdown` npm package and to consistently list our supported targets (Node, WASM, Python, Ruby, PHP, CLI).
- **Dependency Refresh** – Ran `task update` to upgrade Rust crates, npm packages, Bundler gems, Python requirements, and Composer dependencies across the monorepo.

### Fixed

- **Rust Clippy Lints** – Addressed `clippy::unnecessary-map-or` in the converter and hOCR table builder by using `.is_none_or`, keeping inline-image filtering and column pruning logic clear while allowing `cargo clippy -D warnings` to pass.
- **PIE Source Packaging** – `scripts/package_php_pie_source.sh` now copies `packages/ruby/.../native` into the temporary workspace so the Ruby crate exists when PIE builds the PHP extension.

## [2.6.3] - 2025-11-07

### Fixed

- **Release Pipeline** - Fixed missing `is_tag` output in publish workflow that caused all publishing jobs to be skipped
- **Node.js Package Dependencies** - Added missing `optionalDependencies` to html-to-markdown-node package.json to properly link platform-specific binaries
- **Version Management** - Created centralized version sync script (`scripts/sync_versions.py`) to maintain consistency across all package manifests (Rust, Node.js, Python, Ruby, WASM)
- **Cargo Workspace** - Aligned html-to-markdown-rb crate version (was 2.5.7) with workspace version

### Changed

- Added `task sync-versions` command to Taskfile for easy version synchronization across the monorepo

## [2.6.2] - 2025-11-07

### Fixed

- **Table Rowspan Support** - Fixed tables with rowspan cells to correctly duplicate cell content across spanned rows instead of showing empty cells (fixes #116)
- **Node.js Platform Package Publishing** - Fixed workflow to correctly move packed .tgz files to npm directory for publishing
- **Deprecation Warnings** - Updated CLI tests to use `CARGO_BIN_EXE` env var instead of deprecated `cargo_bin` method
- **Deprecation Warnings** - Replaced deprecated `criterion::black_box` with `std::hint::black_box` in benchmarks
- **Clippy Warnings** - Fixed field assignment warnings by using struct initialization with defaults

## [2.6.1] - 2025-11-07

### Fixed

- **Node.js Platform Packages** - Fixed publishing of platform-specific npm packages. The workflow now correctly packs npm directories into .tgz files before publishing, ensuring all platform bindings (linux-x64-gnu, darwin-arm64, win32-x64-msvc, etc.) are published to npm.
- **WASM Package Publishing** - Added proper WASM package publishing workflow to ensure html-to-markdown-wasm is published to npm registry.

## [2.6.0] - 2025-11-07

### Added

- **PHP Extension Support** - Official PHP extension (`goldziher/html-to-markdown`) providing native HTML to Markdown conversion for PHP 8.2+
  - Built with ext-php-rs for high-performance Rust-backed conversion
  - Supports both Thread-Safe (TS) and Non-Thread-Safe (NTS) builds
  - Available for Windows (x86, x64), Linux, and macOS
  - Distributed via PIE (PHP Installer for Extensions) source bundles
  - Prebuilt Windows binaries for PHP 8.2, 8.3, and 8.4
  - Comprehensive test suite with PHPUnit

### Changed

- Refactored PHP build variable names from `HTM2MD_*` to `HTMLTOMARKDOWN_*` for improved clarity in Makefile.frag and config.m4
- Bumped all package versions to 2.6.0 across Rust crates, npm packages, PyPI wheels, Ruby gem, and PHP extension

## [2.5.7] - 2025-11-03

### Added

- Publish Windows PHP extension binaries alongside the PIE source bundle during the release pipeline, enabling one-click installs on every platform.
- Build and archive the CLI binary for Linux (gnu & musl), macOS arm64, and Windows x86_64, plus ship prebuilt WASM bundles (dist/dist-node/dist-web) so every runtime gets first-class artifacts.

### Changed

- Renamed the PHP extension package to `goldziher/html-to-markdown`, moved the Composer metadata to the repository root, and refreshed the documentation/badges for every language target.
- Bumped every package (Rust crates, npm packages, PyPI wheels, Ruby gem, PHP extension) to version 2.5.7.
- Restored the Node.js N-API build matrix so macOS, Windows, and Linux binaries ship automatically with each npm release.

### Fixed

- Preserve ordered list numbering and indentation when list items render headings or HTML tables, so mixed block content stays under the correct bullet (fixes #107).

## [2.5.6] - 2025-10-30

### Changed

- The Ruby gem now packages its own README at the gem root, so RubyGems renders the fully formatted documentation (benchmarks, configuration, CLI notes) without broken links.
- Documentation links: the Ruby README now surfaces GitHub resources (issues, changelog, live demo) alongside feature highlights.
- Bumped every package (Rust crates, npm, PyPI, Ruby gem) to version 2.5.6.

## [2.5.5] - 2025-10-30

### Changed

- Synced documentation: the root README now links to every language guide, and the Ruby README highlights GitHub resources alongside feature docs.
- Gem packaging now reads the README directly for the RubyGems long description while keeping Rubocop happy on all Ruby sources.
- Bumped every package (Rust crates, npm, PyPI, Ruby gem) to version 2.5.5.

## [2.5.4] - 2025-10-30

### Changed

- Polished the Ruby gem messaging and README with performance highlights, configuration examples, and CLI guidance to match other language docs.
- Bumped every package (Rust crates, npm, PyPI, Ruby gem) to version 2.5.4.

## [2.5.3] - 2025-10-30

### Changed

- Publish Ruby gems as precompiled artifacts for Linux (x86_64), macOS (arm64 & x86_64), and Windows (x64) via a matrix GitHub Action, ensuring the CLI executable matches the target platform.
- Split the release workflow into prepare/build/publish stages so dry runs build artifacts without pushing, and trusted publishing now uploads every generated `.gem`.
- Hardened the gem preparation script to clear stale CLI binaries before copying in the platform-specific build output.
- Re-enabled the cross-language release workflow so crates.io, PyPI wheels/sdist, and both npm packages ship alongside the Ruby release.

## [2.5.2] - 2025-10-29

- Fix Ruby gem packaging to embed standalone Cargo manifest (no workspace inheritance) so installs compile out of tree successfully.
- Bump versions across Rust, Node, Python, and Ruby bindings.

## [2.5.1] - 2025-10-28

### Added

- Magnus-based Ruby gem (`html-to-markdown-rb`) with CLI proxy and comprehensive specs.

### Changed

- CI now includes Ruby coverage across macOS, Linux, and Windows, installing the appropriate toolchains (MSYS2 on Windows) for Magnus builds.
- Release workflow prepares the Ruby gem via trusted publishing alongside existing crates/npm packages.

### Fixed

- Bundler version pinned to 2.5.12 to support Ruby 3.2 CI environments.

## [2.5.0] - 2025-10-24

### Added

- **New `preserve_tags` option** - Preserve specific HTML tags in their original HTML form instead of converting them to Markdown. This is useful for complex elements like tables that may not convert well to Markdown. Fixes issue #95.
  - Accepts a list of tag names (e.g., `["table", "form"]`)
  - Preserves all attributes and nested content as HTML
  - Works independently of `strip_tags` - can use both options together
  - Available in all bindings: Rust, Python, Node.js, and WASM
  - Comprehensive test coverage in Rust, Python (pytest), and TypeScript (vitest)

### Changed

- **HTML preprocessing is now enabled by default** - The `PreprocessingOptions.enabled` default changed from `False` to `True` to ensure robust handling of malformed HTML. Users who want minimal preprocessing can explicitly set `enabled=False`.

### Fixed

- **Task list checkbox support** - Fixed sanitizer removing `<input type="checkbox">` elements when `remove_forms` is enabled (default). Checkboxes are now preserved during preprocessing to enable proper task list conversion (`- [x]` / `- [ ]`).
  - Added `input` tag to allowed tags in all sanitization presets (minimal, standard, aggressive)
  - Preserved `type` and `checked` attributes on input elements
  - Fixed pre-existing bug where task list checkboxes were silently removed
- **Data URI support for inline images** - Fixed sanitizer stripping `data:` URLs from image src attributes. Base64-encoded inline images (data URIs) are now preserved during preprocessing.
  - Added `data` to allowed URL schemes in all sanitization presets
  - Fixes `convert_with_inline_images` functionality for base64-encoded images
- **CDATA section handling** - Fixed test expectation for CDATA sections. CDATA sections are now correctly preserved as-is during HTML parsing instead of being partially stripped.
- **hOCR word spacing** - Fixed missing whitespace between `<span class="ocrx_word">` elements in hOCR documents. Words now have proper spaces between them.
  - Modified `OcrxWord` converter to insert space before each word if output doesn't end with whitespace or markdown formatting characters
  - Ensures proper word separation in OCR-generated documents without breaking markdown formatting (e.g., `*text*`, `[alt](url)`, `` `code` ``)
- **hOCR detection with preprocessing** - Fixed hOCR documents not being detected when HTML preprocessing is enabled (new default). The sanitizer now preserves:
  - `class` attributes on all elements (required for detecting hOCR element types)
  - `<meta>` tags with `name` and `content` attributes (required for hOCR metadata detection)
  - `<head>` tags (container for meta tags)
- **hOCR metadata extraction after sanitization** - Fixed metadata extraction failing when preprocessing strips the `<head>` container element. The extractor now finds orphaned meta tags anywhere in the document, not just inside `<head>` elements.
- **`preserve_tags` functionality with preprocessing** - Fixed `preserve_tags` not working when HTML preprocessing is enabled (the new default). The sanitizer now:
  - Accepts the `preserve_tags` list and allows those tags through sanitization
  - Preserves common HTML attributes (`id`, `class`, `style`, `title`, etc.) on preserved tags
  - Prevents `remove_forms` from stripping form tags when they're in the preserve list
  - Ensures tags and attributes survive preprocessing so they can be output as HTML
- **SVG support for inline image extraction** - Fixed SVG elements being stripped by the sanitizer, breaking inline image capture. All sanitization presets now allow:
  - SVG elements: `svg`, `circle`, `rect`, `path`, `line`, `polyline`, `polygon`, `ellipse`, `g`
  - SVG attributes: `width`, `height`, `viewBox`, `cx`, `cy`, `r`, `x`, `y`, `d`, `fill`, `stroke`
  - Enables `convert_with_inline_images` to capture inline SVG elements
- **Robust handling of malformed angle brackets in HTML** - Fixed parser failures when bare `<` or `>` characters appear in HTML text content (e.g., `1<2`, mathematical comparisons). The converter now:
  - Automatically escapes malformed angle brackets that aren't part of valid HTML tags
  - Works correctly with preprocessing both enabled and disabled
  - Handles edge cases like `1<2`, `1 < 2 < 3`, and angle brackets at tag boundaries
  - Fixes issue #94 where content following malformed angle brackets was lost
- Added comprehensive test coverage for malformed angle bracket handling in both Rust and Python test suites
- Fixed WASM build configuration to use correct `getrandom` backend for wasm32-unknown-unknown targets

## [2.4.1] - 2025-10-22

### Fixed

- Ensure npm publishes include the generated Node bindings and platform binaries by running the N-API build during CI.
- Configure WebAssembly builds with the `wasm_js` backend and strip wasm-pack `.gitignore` files so published packages ship the compiled `.wasm` artifacts.

## [2.4.0] - 2025-10-22

### Changed

- Updated Rust workspace dependencies (including `pyo3`) to their latest compatible releases and refreshed lockfiles.
- Normalized hOCR conversion spacing by collapsing stray triple newlines, ensuring generated Markdown matches regression fixtures.

### Fixed

- Corrected the WASM crate to depend on `getrandom`'s `wasm_js` feature, restoring WebAssembly builds.
- Expanded the Node package `files` list so published tarballs now include compiled `.node` artifacts, CommonJS shims, and typings.

## [2.3.4] - 2025-10-12

### Changed

- Incremented all distribution metadata and CLI version checks to 2.3.4 following the previous release tag conflict.
- Regenerated package metadata artifacts for the new patch release.

## [2.3.3] - 2025-10-12

### Added

- Python API now exports inline image helpers (`InlineImage`, `InlineImageWarning`, and `InlineImageConfig`) alongside `convert_with_inline_images`, with dedicated regression tests.
- Node and WASM bindings include inline image extraction examples and TypeScript definitions, validated by Vitest coverage.

### Changed

- Bumped all package metadata (Python, Rust, Node, WASM, CLI) to version 2.3.3 for a synchronized release.

### Fixed

- CLI `--version` test updated to assert the new release number.

## [2.2.0] - 2025-10-11

### Added

- `hocr_spatial_tables` option on `ConversionOptions` (Rust, Python, CLI) with `--no-hocr-spatial-tables` flag to disable spatial table reconstruction when desired.
- New hOCR regression fixtures for complex tables and code blocks to guard against formatting regressions.

### Changed

- Improved hOCR conversion heuristics to distinguish between dense paragraph layouts and true tables, yielding cleaner Markdown for scientific data.
- hOCR code-block detection now preserves fenced formatting, restoring context headings when present.

### Fixed

- CLI `--version` output and package metadata now report version 2.2.0 consistently.

## [2.1.1] - 2025-10-11

### Fixed

- Improve hOCR table reconstruction when tables are represented as paragraphs, ensuring Markdown tables are emitted for Tesseract outputs without explicit `ocr_table` markers.

## [2.1.0] - 2025-10-11

### Added

- **Inline image extraction** - New `convert_with_inline_images()` function to extract embedded images during conversion
  - Supports data URI images (`data:image/*`)
  - Supports inline SVG elements
  - Configurable via `InlineImageConfig` with options for:
    - Maximum decoded size limits
    - Custom filename prefixes
    - SVG capture control
    - Optional dimension inference for raster images
  - Returns `HtmlExtraction` with markdown, extracted images, and warnings
  - Available through both Rust and Python APIs

### Changed

- **Simplified API** - Removed `ParsingOptions` class in favor of direct `encoding` parameter on `ConversionOptions`
- **Automatic hOCR table extraction** - hOCR tables are now extracted automatically without requiring configuration
  - Removed `hocr_extract_tables` option (always enabled for hOCR content)
  - Removed `hocr_table_column_threshold` option (uses built-in heuristics)
  - Removed `hocr_table_row_threshold_ratio` option (uses built-in heuristics)
- Updated pre-commit hook versions (commitlint v9.23.0, pyproject-fmt v2.10.0, ruff v0.14.0)

### Fixed

- **hOCR metadata now uses YAML frontmatter** instead of HTML comments for cleaner markdown output
- **hOCR code organization** - Restructured spatial table reconstruction into dedicated `hocr/spatial.rs` module
- **Conservative table detection** - hOCR spatial table reconstruction now only applies to explicit `ocr_table` elements, preventing false positives
- Windows CLI binary detection - now correctly searches for `.exe` extension on Windows
- CLI binary bundling in Python wheels - binary now included in package for all platforms
- hOCR extractor Rust doctest - added missing import statement
- 928 Python test expectations updated for CommonMark-compliant v2 defaults
- Python 3.14-dev → Python 3.14 stable in CI workflows
- Reorganized wheel preparation script to `scripts/` directory
- Removed duplicate markdown documentation files (BENCHMARKS.md, PERFORMANCE.md, BENCHMARK_RESULTS.md, COMMONMARK_COMPLIANCE.md, REFACTORING_SUMMARY.md)

## [2.0.0] - 2025-10-03

### 🚀 Major Rewrite: Rust Backend

Version 2.0.0 represents a complete rewrite of html-to-markdown with a high-performance Rust backend, delivering **10-30x performance improvements** while maintaining full backward compatibility through a v1 compatibility layer.

### ⚠️ Breaking Changes

#### CommonMark-Compliant Defaults

V2 adopts CommonMark-compliant defaults for better interoperability:

| Option                  | V1 Default   | V2 Default | Reason                           |
| ----------------------- | ------------ | ---------- | -------------------------------- |
| `list_indent_width`     | 4            | 2          | CommonMark standard              |
| `bullets`               | "-"          | "\*+-"     | Cycling bullets for nested lists |
| `escape_asterisks`      | true         | false      | Minimal escaping                 |
| `escape_underscores`    | true         | false      | Minimal escaping                 |
| `escape_misc`           | true         | false      | Minimal escaping                 |
| `newline_style`         | "backslash"  | "spaces"   | CommonMark two-space line breaks |
| `code_block_style`      | "backticks"  | "indented" | CommonMark 4-space indent        |
| `heading_style`         | "underlined" | "atx"      | CommonMark `#` headings          |
| `preprocessing.enabled` | false        | false      | No change (opt-in)               |

**Migration**: If you relied on v1 defaults, explicitly set options to match v1 behavior.

#### Removed CLI Flags

The following v1 CLI flags are **not supported** in v2. The Python CLI proxy will raise helpful error messages when these flags are used:

| Removed Flag | Reason                | Migration                                 |
| ------------ | --------------------- | ----------------------------------------- |
| `--strip`    | Feature removed in v2 | Remove flag (feature no longer available) |
| `--convert`  | Feature removed in v2 | Remove flag (feature no longer available) |

**Note on Redundant Flags**: The following v1 flags are redundant in v2 (they match the defaults) but are **silently accepted** for backward compatibility:

- `--no-escape-asterisks`, `--no-escape-underscores`, `--no-escape-misc` (v2 defaults to minimal escaping)
- `--no-wrap` (v2 defaults to no wrapping)
- `--no-autolinks` (Rust CLI defaults to no autolinks)
- `--no-extract-metadata` (Rust CLI defaults to no metadata extraction)

These flags can be safely removed from your commands, or you can leave them for compatibility.

**Note**: The Rust CLI only supports positive flags (e.g., `--escape-asterisks`, `--autolinks`, `--wrap`). Negative flags (`--no-*`) are only supported through the Python CLI proxy for v1 compatibility.

#### CommonMark-Compliant List Formatting

- **Tight lists no longer have blank lines before nested sublists** - This follows the [CommonMark specification](https://spec.commonmark.org/) for list formatting
- Previous behavior (v1): `* Item 1\n\n    + Nested\n`
- New behavior (v2): `* Item 1\n    + Nested\n`
- **Why**: CommonMark specifies that tight lists (lists without blank lines between items) should not have blank lines before nested sublists
- **Impact**: Generated markdown will render identically in CommonMark-compliant renderers but may look different in source form
- **Migration**: If you need the old behavior for specific platforms, you can post-process the output or use loose lists (with blank lines between items)

### Added

#### Core Rust Implementation

- **Complete Rust rewrite** of HTML-to-Markdown conversion engine using `scraper` and `html5ever`
- **Native Rust CLI** with improved argument parsing and validation
- **PyO3 Python bindings** for seamless Rust/Python integration
- **Automatic hOCR table extraction** with built-in heuristics for OCR documents

#### New V2 API

- Clean, modern API with dataclass-based configuration
- `convert(html, options, preprocessing)` - primary API entry point
- `ConversionOptions` - comprehensive conversion settings (now includes `encoding`)
- `PreprocessingOptions` - HTML cleaning configuration
- Legacy parsing options removed in favour of explicit encoding on `ConversionOptions`
- Improved type safety with full type stubs (`.pyi` files)

#### V1 Compatibility Layer

- **100% backward compatible** v1 API through compatibility layer
- `convert_to_markdown()` function with all v1 kwargs
- Smart translation of v1 options to v2 dataclasses
- CLI argument translation for v1 flags
- Clear error messages for unsupported v1 features

#### Testing & Quality

- **77 new tests** for v1 compatibility (32 bindings + 26 CLI + 19 integration)
- Comprehensive integration tests with actual CLI execution
- Wheel testing workflow for cross-platform validation
- Python 3.10, 3.12, 3.14-dev test matrix
- Dual coverage reporting (Python + Rust)

#### CI/CD Improvements

- Shared build-wheels action for consistent wheel building
- Test-wheels workflow with full test suite on built wheels
- Rust coverage with `cargo-llvm-cov`
- Python coverage in LCOV format
- Automated wheel building for Python 3.10-3.13

### Changed

#### Performance

- **60-80x faster** than v1 for most conversion operations (144-208 MB/s throughput)
- Memory-efficient processing with Rust's zero-cost abstractions
- Optimized table handling with rowspan/colspan tracking
- Faster list processing with unified helpers

#### Architecture

- Removed Python implementation (`converters.py`, `processing.py`, `preprocessor.py`)
- Migrated to Rust-based conversion engine
- Simplified Python layer to thin wrapper around Rust bindings
- CLI now proxies to native Rust binary with argument translation

#### API Design

- More explicit configuration with separate option classes
- Better separation of concerns (conversion/preprocessing/parsing)
- Clearer parameter naming and organization
- Improved error messages and exception handling

### Removed v1 Features

The following v1 features were **removed** in v2:

- `code_language_callback` - Removed (use `code_language` option for default language)
- `strip` option - Removed (use preprocessing options instead)
- `convert` option - Removed (all supported tags are converted by default)
- `convert_to_markdown_stream()` - Removed (html5ever does not support streaming parsing)

### Not Yet Implemented

- `custom_converters` - Planned for future release with Rust and Python callback support

### Migration Guide

#### For Most Users (No Changes Needed)

If you're using the v1 API, your code will continue to work:

```python
from html_to_markdown import convert_to_markdown

# This still works in v2!
markdown = convert_to_markdown(html, heading_style="atx")
```

#### To Use New V2 API (Recommended)

```python
from html_to_markdown import convert, ConversionOptions

options = ConversionOptions(heading_style="atx")
markdown = convert(html, options)
```

#### CLI Changes

V1 CLI flags are automatically translated to v2:

```bash
# V1 style (still works)
html-to-markdown --preprocess-html --escape-asterisks input.html

# V2 style (recommended)
html-to-markdown --preprocess input.html  # escaping is default
```

### Performance Benchmarks

Real-world performance improvements over v1 (Apple M4):

| Document Type       | Size  | V2 Latency | V2 Throughput | Speedup vs V1 (2.5 MB/s) |
| ------------------- | ----- | ---------- | ------------- | ------------------------ |
| Lists (Timeline)    | 129KB | 0.62ms     | 208 MB/s      | **83x**                  |
| Tables (Countries)  | 360KB | 2.02ms     | 178 MB/s      | **71x**                  |
| Mixed (Python wiki) | 656KB | 4.56ms     | 144 MB/s      | **58x**                  |

V2's Rust engine delivers **60-80x higher throughput** than V1's Python/BeautifulSoup implementation across real-world documents.

### Technical Details

#### Rust Crates Structure

```text
crates/
├── html-to-markdown/       # Core conversion library
├── html-to-markdown-py/    # Python bindings (PyO3)
└── html-to-markdown-cli/   # Native CLI binary
```

#### Python Package Structure

```text
html_to_markdown/
├── api.py                  # V2 API
├── options.py              # V2 configuration dataclasses
├── v1_compat.py           # V1 compatibility layer
├── cli_proxy.py           # CLI argument translation
├── _rust.pyi              # Rust binding type stubs
└── __init__.py            # Public API exports
```

### Breaking Changes Summary

None if using v1 compatibility layer. If migrating to v2 API:

1. **Import changes**: `convert_to_markdown` → `convert`
1. **Configuration**: Kwargs → Dataclasses (`ConversionOptions`)
1. **Defaults changed**: See CommonMark-compliant defaults table above
1. **Removed features**: See [Removed v1 Features](#removed-v1-features) section above

### Complete V1 vs V2 Comparison

#### API Differences

| Aspect                  | V1                              | V2                                               |
| ----------------------- | ------------------------------- | ------------------------------------------------ |
| **Primary API**         | `convert_to_markdown(**kwargs)` | `convert(html, options, preprocessing, parsing)` |
| **Configuration**       | Keyword arguments               | Dataclasses (`ConversionOptions`, etc.)          |
| **Type Safety**         | Basic type hints                | Full `.pyi` stubs + generics                     |
| **Compatibility Layer** | N/A                             | `convert_to_markdown()` with v1 kwargs           |

#### Performance Differences

| Document Type       | V1 Throughput | V2 Throughput | Speedup |
| ------------------- | ------------- | ------------- | ------- |
| Lists (Timeline)    | 2.5 MB/s      | 208 MB/s      | **83x** |
| Tables (Countries)  | 2.5 MB/s      | 178 MB/s      | **71x** |
| Mixed (Python wiki) | 2.5 MB/s      | 144 MB/s      | **58x** |
| Average             | 2.5 MB/s      | 177 MB/s      | **71x** |

#### Implementation Differences

| Component        | V1                         | V2                       |
| ---------------- | -------------------------- | ------------------------ |
| **HTML Parser**  | BeautifulSoup4 / lxml      | html5ever (Rust)         |
| **Sanitizer**    | Custom Python              | html5ever DOM filtering  |
| **Conversion**   | Pure Python (~3,850 lines) | Pure Rust (~4,800 lines) |
| **Bindings**     | N/A                        | PyO3                     |
| **CLI**          | Python wrapper             | Native Rust binary       |
| **Dependencies** | bs4, lxml, soupsieve       | None (statically linked) |

#### Output Differences (Default Settings)

| HTML                     | V1 Output             | V2 Output           |
| ------------------------ | --------------------- | ------------------- |
| `<ul><li>Item</li></ul>` | `*   Item` (4 spaces) | `- Item` (2 spaces) |
| `<h1>Title</h1>`         | `Title\n=====`        | `# Title`           |
| `Text*with*stars`        | `Text\*with\*stars`   | `Text*with*stars`   |
| `<br>`                   | Two trailing spaces   | Backslash `\`       |
| `<pre>code</pre>`        | ` ```\ncode\n``` `    | Indented 4 spaces   |

These differences reflect v2's alignment with CommonMark specification.

### Removed Python Implementation

- Python implementation of HTML conversion
- `html_to_markdown/converters.py` (1220 lines)
- `html_to_markdown/processing.py` (1195 lines)
- `html_to_markdown/preprocessor.py` (404 lines)
- `html_to_markdown/whitespace.py` (293 lines)
- `html_to_markdown/utils.py` (37 lines)
- Several test files migrated to Rust or marked as `.skip`

Total: **~3,850 lines** of Python code removed, replaced by **~4,800 lines** of Rust

### Notes

- **Platform Support**: Wheels built for Linux, macOS, Windows on x86_64
- **Python Version**: Requires Python 3.10+
- **ABI Compatibility**: Uses `abi3` for Python 3.10+ wheel reuse
- **Rust Version**: Built with stable Rust (tested on 1.75+)

---

## [1.x] - Previous Versions

For changes in v1.x releases, see git history before the v2 rewrite.

[2.0.0]: https://github.com/xberg-io/html-to-markdown/compare/v1.x...v2.0.0
