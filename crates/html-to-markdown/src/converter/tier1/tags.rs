//! Compile-time table mapping tag-name bytes → semantic spec consulted by the
//! Tier-1 byte scanner (M3c).  All lookups go through `lookup(tag_name_bytes)`;
//! callers are expected to lowercase their input first.

use phf::phf_map;

/// Coarse semantic category of an HTML element.
///
/// `Block` and `Inline` are the catch-all defaults; specific kinds get their
/// own variant so the scanner / emit dispatch can branch cleanly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagKind {
    // Block-level structural
    Block,
    Paragraph,
    /// Heading level 1–6.
    Heading(u8),
    Blockquote,
    Hr,
    /// Code-block container.
    Pre,

    // Inline formatting
    Inline,
    Strong,
    Emphasis,
    /// Inline `code`.
    Code,
    /// `<br>` — forces a line break.
    LineBreak,
    /// `<a>` — hyperlink.
    Link,
    /// `<img>` — embedded image.
    Image,

    // List structures
    List(ListKind),
    ListItem,
    DefinitionTerm,
    DefinitionDescription,

    // Tables
    Table,
    TableHead,
    TableBody,
    TableFoot,
    TableRow,
    /// Table cell; `is_header` distinguishes `<th>` from `<td>`.
    TableCell {
        is_header: bool,
    },
    TableCaption,

    // Raw-text containers (scanner must skip their contents)
    RawText(RawKind),

    // Special / ignored
    /// Explicitly drop (e.g. `<script>`, `<style>`).
    Ignored,
}

/// List flavour for `TagKind::List`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListKind {
    Unordered,
    Ordered,
    Definition,
}

/// Raw-text sub-type for `TagKind::RawText`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawKind {
    Script,
    Style,
    Textarea,
    Title,
    Xmp,
    Iframe,
    Noscript,
    NoEmbed,
    NoFrames,
}

/// One row of the static tag spec table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TagSpec {
    /// Coarse semantic category.
    pub kind: TagKind,
    /// HTML void element — no closing tag, no children.
    pub is_void: bool,
    /// `<br/>` normalised to `<br>`, etc.
    pub is_self_closing_xhtml: bool,
    /// True for block-level elements (HTML5 flow content that establishes a
    /// block formatting context); false for inline/text-level elements.
    pub is_block: bool,
    /// HTML5 optional-close-tag rule, if any.
    pub optional_close: Option<OptionalCloseRule>,
    /// If true, the scanner enters rawtext mode and skips contents until a
    /// matching close tag.  Set for `script`, `style`, `textarea`, `title`,
    /// `xmp`.
    pub is_rawtext: bool,
}

/// HTML5 optional-close-tag rules.
///
/// The scanner consults this when it sees a new open tag — if the new tag's
/// rule matches the currently-open tag, the open tag is implicitly closed.
/// See spec: <https://html.spec.whatwg.org/multipage/syntax.html#optional-tags>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionalCloseRule {
    /// `<li>` closes any open `<li>`.
    CloseSameKind,
    /// `<p>` closes when a block-level child opens.
    CloseOnBlockChild,
    /// `<dt>`/`<dd>` close any open `<dt>`/`<dd>`.
    CloseSiblingDtDd,
    /// `<tr>` closes any open `<tr>` (after closing any open `<td>`/`<th>`).
    CloseTableRow,
    /// `<td>`/`<th>` close any open cell.
    CloseTableCell,
    /// `<option>` closes any open `<option>`.
    CloseOption,
    /// First `<tr>` inside `<table>` without `<tbody>`/`<thead>`/`<tfoot>` →
    /// the scanner emits an implicit `<tbody>` open.
    ImplicitTbody,
}

/// Look up a tag by its lowercased name bytes.
///
/// Returns `None` for unknown tags.  Callers **must** lowercase the tag name
/// before calling; the table keys are all ASCII-lowercase.
#[inline]
pub fn lookup(tag_name_lower: &[u8]) -> Option<&'static TagSpec> {
    TAGS.get(tag_name_lower)
}

// ── Convenience constructors (reduce repetition in the static table) ──────────

const fn block(kind: TagKind) -> TagSpec {
    TagSpec {
        kind,
        is_void: false,
        is_self_closing_xhtml: false,
        is_block: true,
        optional_close: None,
        is_rawtext: false,
    }
}

const fn block_opt(kind: TagKind, rule: OptionalCloseRule) -> TagSpec {
    TagSpec {
        kind,
        is_void: false,
        is_self_closing_xhtml: false,
        is_block: true,
        optional_close: Some(rule),
        is_rawtext: false,
    }
}

const fn void_block(kind: TagKind) -> TagSpec {
    TagSpec {
        kind,
        is_void: true,
        is_self_closing_xhtml: false,
        is_block: true,
        optional_close: None,
        is_rawtext: false,
    }
}

const fn void_inline(kind: TagKind) -> TagSpec {
    TagSpec {
        kind,
        is_void: true,
        is_self_closing_xhtml: false,
        is_block: false,
        optional_close: None,
        is_rawtext: false,
    }
}

const fn inline(kind: TagKind) -> TagSpec {
    TagSpec {
        kind,
        is_void: false,
        is_self_closing_xhtml: false,
        is_block: false,
        optional_close: None,
        is_rawtext: false,
    }
}

const fn rawtext_ignored() -> TagSpec {
    TagSpec {
        kind: TagKind::Ignored,
        is_void: false,
        is_self_closing_xhtml: false,
        is_block: true,
        optional_close: None,
        is_rawtext: true,
    }
}

const fn rawtext_block(kind: TagKind) -> TagSpec {
    TagSpec {
        kind,
        is_void: false,
        is_self_closing_xhtml: false,
        is_block: true,
        optional_close: None,
        is_rawtext: true,
    }
}

const fn ignored_block() -> TagSpec {
    TagSpec {
        kind: TagKind::Ignored,
        is_void: false,
        is_self_closing_xhtml: false,
        is_block: true,
        optional_close: None,
        is_rawtext: false,
    }
}

const fn ignored_void() -> TagSpec {
    TagSpec {
        kind: TagKind::Ignored,
        is_void: true,
        is_self_closing_xhtml: false,
        is_block: false,
        optional_close: None,
        is_rawtext: false,
    }
}

// ── Static lookup table ───────────────────────────────────────────────────────

static TAGS: phf::Map<&'static [u8], TagSpec> = phf_map! {
    // ── Document structure ────────────────────────────────────────────────────
    b"html"    => block(TagKind::Block),
    b"head"    => ignored_block(),
    b"body"    => block(TagKind::Block),
    b"meta"    => ignored_void(),
    b"link"    => ignored_void(),
    b"base"    => void_block(TagKind::Block),

    // ── Block structural ─────────────────────────────────────────────────────
    b"div"     => block(TagKind::Block),
    b"section" => block(TagKind::Block),
    b"article" => block(TagKind::Block),
    b"header"  => block(TagKind::Block),
    b"footer"  => block(TagKind::Block),
    b"aside"   => block(TagKind::Block),
    b"main"    => block(TagKind::Block),
    b"nav"     => block(TagKind::Block),
    b"address" => block(TagKind::Block),
    b"details" => block(TagKind::Block),
    b"summary" => block(TagKind::Block),
    b"dialog"  => block(TagKind::Block),
    b"figure"  => block(TagKind::Block),
    b"figcaption" => block(TagKind::Block),

    // ── Paragraph ────────────────────────────────────────────────────────────
    b"p" => block_opt(TagKind::Paragraph, OptionalCloseRule::CloseOnBlockChild),

    // ── Headings ─────────────────────────────────────────────────────────────
    b"h1" => block(TagKind::Heading(1)),
    b"h2" => block(TagKind::Heading(2)),
    b"h3" => block(TagKind::Heading(3)),
    b"h4" => block(TagKind::Heading(4)),
    b"h5" => block(TagKind::Heading(5)),
    b"h6" => block(TagKind::Heading(6)),

    // ── Blockquote ───────────────────────────────────────────────────────────
    b"blockquote" => block(TagKind::Blockquote),

    // ── Horizontal rule ──────────────────────────────────────────────────────
    b"hr" => void_block(TagKind::Hr),

    // ── Preformatted / code block ─────────────────────────────────────────────
    b"pre" => block(TagKind::Pre),

    // ── Line break ───────────────────────────────────────────────────────────
    b"br" => TagSpec {
        kind: TagKind::LineBreak,
        is_void: true,
        is_self_closing_xhtml: true,
        is_block: false,
        optional_close: None,
        is_rawtext: false,
    },

    // ── Inline emphasis ───────────────────────────────────────────────────────
    b"strong" => inline(TagKind::Strong),
    b"b"      => inline(TagKind::Strong),
    b"em"     => inline(TagKind::Emphasis),
    b"i"      => inline(TagKind::Emphasis),
    b"s"      => inline(TagKind::Inline),
    b"del"    => inline(TagKind::Inline),
    b"strike" => inline(TagKind::Inline),
    b"u"      => inline(TagKind::Inline),
    b"ins"    => inline(TagKind::Inline),

    // ── Inline marks ──────────────────────────────────────────────────────────
    b"mark"   => inline(TagKind::Inline),
    b"small"  => inline(TagKind::Inline),
    b"sub"    => inline(TagKind::Inline),
    b"sup"    => inline(TagKind::Inline),
    b"kbd"    => inline(TagKind::Inline),
    b"code"   => inline(TagKind::Code),
    b"samp"   => inline(TagKind::Inline),
    b"var"    => inline(TagKind::Inline),
    b"dfn"    => inline(TagKind::Inline),
    b"abbr"   => inline(TagKind::Inline),
    b"span"   => inline(TagKind::Inline),
    b"bdi"    => inline(TagKind::Inline),
    b"bdo"    => inline(TagKind::Inline),
    b"cite"   => inline(TagKind::Inline),
    b"q"      => inline(TagKind::Inline),
    b"time"   => inline(TagKind::Inline),
    b"data"   => inline(TagKind::Inline),

    // ── Links / images ────────────────────────────────────────────────────────
    b"a"   => inline(TagKind::Link),
    b"img" => void_inline(TagKind::Image),

    // ── Lists ─────────────────────────────────────────────────────────────────
    b"ul" => block(TagKind::List(ListKind::Unordered)),
    b"ol" => block(TagKind::List(ListKind::Ordered)),
    b"dl" => block(TagKind::List(ListKind::Definition)),
    b"li" => block_opt(TagKind::ListItem, OptionalCloseRule::CloseSameKind),
    b"dt" => block_opt(TagKind::DefinitionTerm, OptionalCloseRule::CloseSiblingDtDd),
    b"dd" => block_opt(TagKind::DefinitionDescription, OptionalCloseRule::CloseSiblingDtDd),

    // ── Tables ────────────────────────────────────────────────────────────────
    b"table"    => block(TagKind::Table),
    b"thead"    => block(TagKind::TableHead),
    b"tbody"    => block(TagKind::TableBody),
    b"tfoot"    => block(TagKind::TableFoot),
    b"tr"       => block_opt(TagKind::TableRow, OptionalCloseRule::CloseTableRow),
    b"th"       => block_opt(TagKind::TableCell { is_header: true  }, OptionalCloseRule::CloseTableCell),
    b"td"       => block_opt(TagKind::TableCell { is_header: false }, OptionalCloseRule::CloseTableCell),
    b"caption"  => block(TagKind::TableCaption),
    b"colgroup" => block(TagKind::Block),
    b"col"      => void_block(TagKind::Block),

    // ── Raw-text containers ───────────────────────────────────────────────────
    b"script"   => rawtext_ignored(),
    b"style"    => rawtext_ignored(),
    b"title"    => rawtext_block(TagKind::RawText(RawKind::Title)),
    b"xmp"      => rawtext_block(TagKind::RawText(RawKind::Xmp)),
    b"textarea" => rawtext_block(TagKind::RawText(RawKind::Textarea)),
    b"iframe"   => rawtext_block(TagKind::RawText(RawKind::Iframe)),
    b"noscript" => rawtext_block(TagKind::RawText(RawKind::Noscript)),
    b"noembed"  => rawtext_block(TagKind::RawText(RawKind::NoEmbed)),
    b"noframes" => rawtext_block(TagKind::RawText(RawKind::NoFrames)),

    // ── Forms ─────────────────────────────────────────────────────────────────
    b"form"      => block(TagKind::Block),
    b"fieldset"  => block(TagKind::Block),
    b"legend"    => block(TagKind::Block),
    b"label"     => inline(TagKind::Inline),
    b"input"     => void_inline(TagKind::Inline),
    b"select"    => inline(TagKind::Inline),
    b"option"    => TagSpec {
        kind: TagKind::Inline,
        is_void: false,
        is_self_closing_xhtml: false,
        is_block: false,
        optional_close: Some(OptionalCloseRule::CloseOption),
        is_rawtext: false,
    },
    b"optgroup"  => inline(TagKind::Inline),
    b"button"    => inline(TagKind::Inline),
    b"progress"  => inline(TagKind::Inline),
    b"meter"     => inline(TagKind::Inline),
    b"output"    => inline(TagKind::Inline),
    b"datalist"  => inline(TagKind::Inline),

    // ── Media / embedded ──────────────────────────────────────────────────────
    b"audio"   => block(TagKind::Block),
    b"video"   => block(TagKind::Block),
    b"picture" => inline(TagKind::Inline),
    b"canvas"  => inline(TagKind::Inline),
    b"map"     => inline(TagKind::Inline),

    // ── Other void elements (HTML5 spec) ─────────────────────────────────────
    // https://html.spec.whatwg.org/multipage/syntax.html#void-elements
    // Void list: area, base, br, col, embed, hr, img, input, link, meta, source, track, wbr
    // Plus historical: keygen, param
    b"area"   => void_inline(TagKind::Inline),
    b"embed"  => void_inline(TagKind::Inline),
    b"source" => void_inline(TagKind::Inline),
    b"track"  => void_inline(TagKind::Inline),
    b"wbr"    => void_inline(TagKind::Inline),
    b"keygen" => void_inline(TagKind::Inline),
    b"param"  => void_inline(TagKind::Inline),

    // ── Misc block wrappers ───────────────────────────────────────────────────
    b"hgroup"    => block(TagKind::Block),
    b"menu"      => block(TagKind::Block),
    b"search"    => block(TagKind::Block),
};
