//! Tests for the Tier-1 compile-time tag spec table (M3b).
//!
//! All lookups use lowercased byte slices — the lookup contract requires
//! callers to lowercase tag names before calling `lookup()`.

use html_to_markdown_rs::tier1::tags::{ListKind, OptionalCloseRule, RawKind, TagKind, lookup};

// ── Void elements ─────────────────────────────────────────────────────────────

#[test]
fn br_is_void() {
    let spec = lookup(b"br").expect("br must be in table");
    assert!(spec.is_void, "br must be void");
}

#[test]
fn hr_is_void() {
    let spec = lookup(b"hr").expect("hr must be in table");
    assert!(spec.is_void, "hr must be void");
}

#[test]
fn img_is_void() {
    let spec = lookup(b"img").expect("img must be in table");
    assert!(spec.is_void, "img must be void");
}

#[test]
fn input_is_void() {
    let spec = lookup(b"input").expect("input must be in table");
    assert!(spec.is_void, "input must be void");
}

#[test]
fn meta_is_void() {
    let spec = lookup(b"meta").expect("meta must be in table");
    assert!(spec.is_void, "meta must be void");
}

#[test]
fn link_is_void() {
    let spec = lookup(b"link").expect("link must be in table");
    assert!(spec.is_void, "link must be void");
}

#[test]
fn area_is_void() {
    let spec = lookup(b"area").expect("area must be in table");
    assert!(spec.is_void, "area must be void");
}

#[test]
fn embed_is_void() {
    let spec = lookup(b"embed").expect("embed must be in table");
    assert!(spec.is_void, "embed must be void");
}

#[test]
fn source_is_void() {
    let spec = lookup(b"source").expect("source must be in table");
    assert!(spec.is_void, "source must be void");
}

#[test]
fn track_is_void() {
    let spec = lookup(b"track").expect("track must be in table");
    assert!(spec.is_void, "track must be void");
}

#[test]
fn wbr_is_void() {
    let spec = lookup(b"wbr").expect("wbr must be in table");
    assert!(spec.is_void, "wbr must be void");
}

#[test]
fn col_is_void() {
    let spec = lookup(b"col").expect("col must be in table");
    assert!(spec.is_void, "col must be void");
}

// ── Block elements ────────────────────────────────────────────────────────────

#[test]
fn div_is_block() {
    let spec = lookup(b"div").expect("div must be in table");
    assert!(spec.is_block, "div must be block");
}

#[test]
fn p_is_block() {
    let spec = lookup(b"p").expect("p must be in table");
    assert!(spec.is_block, "p must be block");
}

#[test]
fn h1_is_block() {
    let spec = lookup(b"h1").expect("h1 must be in table");
    assert!(spec.is_block, "h1 must be block");
}

#[test]
fn blockquote_is_block() {
    let spec = lookup(b"blockquote").expect("blockquote must be in table");
    assert!(spec.is_block, "blockquote must be block");
}

#[test]
fn ul_is_block() {
    let spec = lookup(b"ul").expect("ul must be in table");
    assert!(spec.is_block, "ul must be block");
}

#[test]
fn table_is_block() {
    let spec = lookup(b"table").expect("table must be in table");
    assert!(spec.is_block, "table must be block");
}

// ── Heading levels ────────────────────────────────────────────────────────────

#[test]
fn heading_levels_1_through_6() {
    for n in 1u8..=6 {
        let tag = format!("h{n}");
        let spec = lookup(tag.as_bytes()).unwrap_or_else(|| panic!("{tag} must be in table"));
        assert_eq!(spec.kind, TagKind::Heading(n), "{tag} must be Heading({n})");
    }
}

// ── List kinds ────────────────────────────────────────────────────────────────

#[test]
fn ul_is_unordered_list() {
    let spec = lookup(b"ul").expect("ul must be in table");
    assert_eq!(spec.kind, TagKind::List(ListKind::Unordered));
}

#[test]
fn ol_is_ordered_list() {
    let spec = lookup(b"ol").expect("ol must be in table");
    assert_eq!(spec.kind, TagKind::List(ListKind::Ordered));
}

#[test]
fn dl_is_definition_list() {
    let spec = lookup(b"dl").expect("dl must be in table");
    assert_eq!(spec.kind, TagKind::List(ListKind::Definition));
}

// ── Optional-close rules ──────────────────────────────────────────────────────

#[test]
fn li_has_close_same_kind() {
    let spec = lookup(b"li").expect("li must be in table");
    assert_eq!(
        spec.optional_close,
        Some(OptionalCloseRule::CloseSameKind),
        "li must have CloseSameKind optional-close rule"
    );
}

#[test]
fn p_has_close_on_block_child() {
    let spec = lookup(b"p").expect("p must be in table");
    assert_eq!(
        spec.optional_close,
        Some(OptionalCloseRule::CloseOnBlockChild),
        "p must have CloseOnBlockChild optional-close rule"
    );
}

#[test]
fn tr_has_close_table_row() {
    let spec = lookup(b"tr").expect("tr must be in table");
    assert_eq!(
        spec.optional_close,
        Some(OptionalCloseRule::CloseTableRow),
        "tr must have CloseTableRow optional-close rule"
    );
}

#[test]
fn td_has_close_table_cell() {
    let spec = lookup(b"td").expect("td must be in table");
    assert_eq!(
        spec.optional_close,
        Some(OptionalCloseRule::CloseTableCell),
        "td must have CloseTableCell optional-close rule"
    );
}

#[test]
fn th_has_close_table_cell() {
    let spec = lookup(b"th").expect("th must be in table");
    assert_eq!(
        spec.optional_close,
        Some(OptionalCloseRule::CloseTableCell),
        "th must have CloseTableCell optional-close rule"
    );
}

#[test]
fn dt_has_close_sibling_dt_dd() {
    let spec = lookup(b"dt").expect("dt must be in table");
    assert_eq!(
        spec.optional_close,
        Some(OptionalCloseRule::CloseSiblingDtDd),
        "dt must have CloseSiblingDtDd optional-close rule"
    );
}

#[test]
fn dd_has_close_sibling_dt_dd() {
    let spec = lookup(b"dd").expect("dd must be in table");
    assert_eq!(
        spec.optional_close,
        Some(OptionalCloseRule::CloseSiblingDtDd),
        "dd must have CloseSiblingDtDd optional-close rule"
    );
}

#[test]
fn option_has_close_option() {
    let spec = lookup(b"option").expect("option must be in table");
    assert_eq!(
        spec.optional_close,
        Some(OptionalCloseRule::CloseOption),
        "option must have CloseOption optional-close rule"
    );
}

// ── Raw-text containers ────────────────────────────────────────────────────────

#[test]
fn script_is_rawtext_and_ignored() {
    let spec = lookup(b"script").expect("script must be in table");
    assert!(spec.is_rawtext, "script must have is_rawtext=true");
    assert_eq!(spec.kind, TagKind::Ignored, "script kind must be Ignored");
}

#[test]
fn style_is_rawtext_and_ignored() {
    let spec = lookup(b"style").expect("style must be in table");
    assert!(spec.is_rawtext, "style must have is_rawtext=true");
    assert_eq!(spec.kind, TagKind::Ignored, "style kind must be Ignored");
}

#[test]
fn textarea_is_rawtext() {
    let spec = lookup(b"textarea").expect("textarea must be in table");
    assert!(spec.is_rawtext, "textarea must have is_rawtext=true");
    assert_eq!(
        spec.kind,
        TagKind::RawText(RawKind::Textarea),
        "textarea must have RawText(Textarea) kind"
    );
}

#[test]
fn title_is_rawtext() {
    let spec = lookup(b"title").expect("title must be in table");
    assert!(spec.is_rawtext, "title must have is_rawtext=true");
    assert_eq!(
        spec.kind,
        TagKind::RawText(RawKind::Title),
        "title must have RawText(Title) kind"
    );
}

#[test]
fn xmp_is_rawtext() {
    let spec = lookup(b"xmp").expect("xmp must be in table");
    assert!(spec.is_rawtext, "xmp must have is_rawtext=true");
    assert_eq!(
        spec.kind,
        TagKind::RawText(RawKind::Xmp),
        "xmp must have RawText(Xmp) kind"
    );
}

// ── Unknown tags ──────────────────────────────────────────────────────────────

#[test]
fn unknown_tag_returns_none() {
    assert!(lookup(b"unknown-thing").is_none(), "unknown-thing must return None");
}

#[test]
fn custom_element_returns_none() {
    assert!(lookup(b"x-foo").is_none(), "x-foo must return None");
}

#[test]
fn completely_made_up_tag_returns_none() {
    assert!(
        lookup(b"kreuzberg-element").is_none(),
        "kreuzberg-element must return None"
    );
}

// ── Case-sensitivity contract ─────────────────────────────────────────────────
//
// The table keys are all ASCII-lowercase.  Callers MUST lowercase the tag
// name before calling `lookup()`; uppercase or mixed-case names will return
// `None` even if the lowercase equivalent is in the table.

#[test]
fn uppercase_tag_returns_none_caller_must_lowercase() {
    // The contract requires callers to lowercase first.  This test documents
    // that the table does NOT handle mixed-case on its own.
    assert!(
        lookup(b"DIV").is_none(),
        "DIV (uppercase) must return None — caller is responsible for lowercasing"
    );
    assert!(
        lookup(b"Div").is_none(),
        "Div (mixed-case) must return None — caller is responsible for lowercasing"
    );
}

#[test]
fn lowercase_div_found() {
    assert!(lookup(b"div").is_some(), "lowercase div must be found");
}

// ── TagKind details ────────────────────────────────────────────────────────────

#[test]
fn strong_kind() {
    assert_eq!(lookup(b"strong").expect("strong in table").kind, TagKind::Strong);
}

#[test]
fn b_is_also_strong() {
    assert_eq!(lookup(b"b").expect("b in table").kind, TagKind::Strong);
}

#[test]
fn em_is_emphasis() {
    assert_eq!(lookup(b"em").expect("em in table").kind, TagKind::Emphasis);
}

#[test]
fn i_is_also_emphasis() {
    assert_eq!(lookup(b"i").expect("i in table").kind, TagKind::Emphasis);
}

#[test]
fn code_kind() {
    assert_eq!(lookup(b"code").expect("code in table").kind, TagKind::Code);
}

#[test]
fn a_is_link() {
    assert_eq!(lookup(b"a").expect("a in table").kind, TagKind::Link);
    assert!(!lookup(b"a").unwrap().is_block, "a must not be block");
}

#[test]
fn img_is_image_and_void() {
    let spec = lookup(b"img").expect("img in table");
    assert_eq!(spec.kind, TagKind::Image);
    assert!(spec.is_void);
}

#[test]
fn br_is_line_break_and_void() {
    let spec = lookup(b"br").expect("br in table");
    assert_eq!(spec.kind, TagKind::LineBreak);
    assert!(spec.is_void);
}

#[test]
fn th_is_header_cell() {
    let spec = lookup(b"th").expect("th in table");
    assert_eq!(spec.kind, TagKind::TableCell { is_header: true });
}

#[test]
fn td_is_data_cell() {
    let spec = lookup(b"td").expect("td in table");
    assert_eq!(spec.kind, TagKind::TableCell { is_header: false });
}

#[test]
fn pre_is_pre_kind() {
    assert_eq!(lookup(b"pre").expect("pre in table").kind, TagKind::Pre);
    assert!(lookup(b"pre").unwrap().is_block);
}

#[test]
fn blockquote_kind() {
    assert_eq!(
        lookup(b"blockquote").expect("blockquote in table").kind,
        TagKind::Blockquote
    );
}

#[test]
fn hr_kind() {
    let spec = lookup(b"hr").expect("hr in table");
    assert_eq!(spec.kind, TagKind::Hr);
    assert!(spec.is_void);
    assert!(spec.is_block);
}

// ── Golden test: at least 50 standard HTML tags resolve successfully ──────────

#[test]
fn golden_at_least_50_tags_found() {
    let standard_tags: &[&[u8]] = &[
        b"html",
        b"head",
        b"body",
        b"meta",
        b"link",
        b"base",
        b"div",
        b"section",
        b"article",
        b"header",
        b"footer",
        b"aside",
        b"main",
        b"nav",
        b"p",
        b"h1",
        b"h2",
        b"h3",
        b"h4",
        b"h5",
        b"h6",
        b"blockquote",
        b"hr",
        b"pre",
        b"br",
        b"strong",
        b"b",
        b"em",
        b"i",
        b"s",
        b"del",
        b"u",
        b"ins",
        b"mark",
        b"small",
        b"sub",
        b"sup",
        b"code",
        b"span",
        b"a",
        b"img",
        b"ul",
        b"ol",
        b"dl",
        b"li",
        b"dt",
        b"dd",
        b"table",
        b"thead",
        b"tbody",
        b"tfoot",
        b"tr",
        b"th",
        b"td",
        b"caption",
        b"script",
        b"style",
        b"textarea",
        b"title",
        b"form",
        b"input",
        b"button",
        b"select",
        b"option",
        b"area",
        b"embed",
        b"source",
        b"track",
        b"wbr",
    ];

    let mut found = 0usize;
    let mut missing: Vec<&str> = Vec::new();

    for tag in standard_tags {
        if lookup(tag).is_some() {
            found += 1;
        } else {
            missing.push(std::str::from_utf8(tag).unwrap_or("<invalid utf8>"));
        }
    }

    assert!(
        found >= 50,
        "expected at least 50 standard tags to be found, got {found}; missing: {missing:?}"
    );
}

// ── Non-block inline elements do not carry is_block=true ─────────────────────

#[test]
fn inline_elements_are_not_block() {
    let inline_tags: &[&[u8]] = &[
        b"span", b"strong", b"em", b"code", b"a", b"b", b"i", b"small", b"sup", b"sub",
    ];
    for tag in inline_tags {
        let spec = lookup(tag).unwrap_or_else(|| panic!("{} must be in table", std::str::from_utf8(tag).unwrap()));
        assert!(
            !spec.is_block,
            "{} must not be block-level",
            std::str::from_utf8(tag).unwrap()
        );
    }
}

// ── Non-void elements do not carry is_void=true ───────────────────────────────

#[test]
fn non_void_elements_have_is_void_false() {
    let non_void: &[&[u8]] = &[
        b"div",
        b"p",
        b"h1",
        b"blockquote",
        b"pre",
        b"ul",
        b"ol",
        b"li",
        b"table",
        b"tr",
        b"td",
        b"th",
        b"script",
        b"style",
    ];
    for tag in non_void {
        let spec = lookup(tag).unwrap_or_else(|| panic!("{} must be in table", std::str::from_utf8(tag).unwrap()));
        assert!(!spec.is_void, "{} must not be void", std::str::from_utf8(tag).unwrap());
    }
}
