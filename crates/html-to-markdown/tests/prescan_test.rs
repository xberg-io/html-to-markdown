use html_to_markdown_rs::prescan::{self, PrescanReport};

fn report(html: &str) -> PrescanReport {
    prescan::run(html).1
}

fn cleaned(html: &str) -> String {
    prescan::run(html).0.into_owned()
}

// ── head_range ────────────────────────────────────────────────────────────────

#[test]
fn detects_head_range() {
    let html = "<html><head><title>x</title></head><body>y</body></html>";
    let r = report(html);
    let range = r.head_range.expect("should have head_range");
    let (cow, _) = prescan::run(html);
    let buf = cow.as_ref();
    // The range should slice to the head *contents* — between `<head>` and `</head>`.
    let head_contents = &buf[range];
    assert!(
        head_contents.contains("<title>x</title>"),
        "head contents should include title; got: {:?}",
        head_contents
    );
}

#[test]
fn missing_head_yields_none() {
    let r = report("<html><body><p>hello</p></body></html>");
    assert!(r.head_range.is_none(), "no <head> → head_range must be None");
}

#[test]
fn empty_head_yields_empty_range() {
    let html = "<head></head><body>z</body>";
    let (cow, r) = prescan::run(html);
    let range = r.head_range.expect("should detect empty head");
    // start == end: zero-length range
    assert_eq!(
        range.start,
        range.end,
        "empty <head></head> should produce start==end range; got {:?}",
        &cow.as_ref()[range.clone()]
    );
}

#[test]
fn unclosed_head_yields_range_to_eof() {
    // `</head>` is missing — range should extend to end of cleaned buffer.
    let html = "<head><meta charset=\"utf-8\">";
    let (cow, r) = prescan::run(html);
    let range = r.head_range.expect("should detect <head> without close");
    assert_eq!(range.end, cow.as_ref().len(), "unclosed head range end should be EOF");
}

// ── had_custom_elements ───────────────────────────────────────────────────────

#[test]
fn custom_elements_flagged() {
    let r = report("<my-thing attr=\"1\"></my-thing>");
    assert!(
        r.had_custom_elements,
        "tag name with hyphen should set had_custom_elements"
    );
}

#[test]
fn standard_tags_not_custom() {
    let r = report("<div><p class=\"x\"></p></div>");
    assert!(!r.had_custom_elements, "standard tags must not set had_custom_elements");
}

#[test]
fn x_foo_custom_element_flagged() {
    let r = report("<x-foo></x-foo>");
    assert!(r.had_custom_elements);
}

// ── had_cdata ─────────────────────────────────────────────────────────────────

#[test]
fn cdata_flagged() {
    let r = report("<svg><![CDATA[raw content]]></svg>");
    assert!(r.had_cdata, "CDATA section should set had_cdata");
}

#[test]
fn no_cdata_clean() {
    let r = report("<svg><path d=\"M0 0\"/></svg>");
    assert!(!r.had_cdata);
}

// ── had_unescaped_lt ──────────────────────────────────────────────────────────

#[test]
fn unescaped_lt_flagged() {
    // `< b` has a space after `<` — not a valid tag start.
    let r = report("<p>a < b</p>");
    assert!(
        r.had_unescaped_lt,
        "bare `<` not followed by valid tag char should set had_unescaped_lt"
    );
}

#[test]
fn unescaped_lt_clean() {
    let r = report("<p>a &lt; b</p>");
    assert!(
        !r.had_unescaped_lt,
        "pre-escaped &lt; should not trigger had_unescaped_lt"
    );
}

#[test]
fn unescaped_lt_at_eof() {
    // `<` at the very end of input with nothing after it.
    let r = report("<p>text<");
    assert!(r.had_unescaped_lt, "<p>text< should set had_unescaped_lt");
}

// ── had_optional_close_rule_trigger ──────────────────────────────────────────

#[test]
fn optional_close_li_triggers() {
    let r = report("<ul><li>a<li>b</ul>");
    assert!(
        r.had_optional_close_rule_trigger,
        "<li>…<li> without intervening </li> should trigger optional-close rule"
    );
}

#[test]
fn optional_close_p_triggers() {
    let r = report("<p>a<p>b");
    assert!(
        r.had_optional_close_rule_trigger,
        "<p>…<p> without </p> should trigger optional-close rule"
    );
}

#[test]
fn optional_close_li_with_close_no_trigger() {
    let r = report("<ul><li>a</li><li>b</li></ul>");
    assert!(
        !r.had_optional_close_rule_trigger,
        "properly closed <li> should not trigger optional-close rule"
    );
}

// ── has_no_tbody_table ────────────────────────────────────────────────────────

#[test]
fn no_tbody_table_flagged() {
    let r = report("<table><tr><td>a</td></tr></table>");
    assert!(
        r.has_no_tbody_table,
        "<table><tr>… without <tbody> should set has_no_tbody_table"
    );
}

#[test]
fn tbody_table_clean() {
    let r = report("<table><tbody><tr><td>a</td></tr></tbody></table>");
    assert!(
        !r.has_no_tbody_table,
        "<table><tbody>… should not set has_no_tbody_table"
    );
}

// ── has_script_or_style ───────────────────────────────────────────────────────

#[test]
fn script_flag() {
    let r = report("<script>alert(1)</script>");
    assert!(r.has_script_or_style);
}

#[test]
fn style_flag() {
    let r = report("<style>body { color: red; }</style>");
    assert!(r.has_script_or_style);
}

#[test]
fn no_script_or_style_clean() {
    let r = report("<div><p>text</p></div>");
    assert!(!r.has_script_or_style);
}

// ── has_svg ───────────────────────────────────────────────────────────────────

#[test]
fn svg_flag() {
    let r = report("<svg viewBox=\"0 0 10 10\"></svg>");
    assert!(r.has_svg, "<svg> should set has_svg");
}

#[test]
fn no_svg_clean() {
    let r = report("<div><p>text</p></div>");
    assert!(!r.has_svg);
}

// ── total_tags ────────────────────────────────────────────────────────────────

#[test]
fn total_tags_counts() {
    let r = report("<p><a></a></p>");
    // <p>, <a>, </a>, </p> = 4
    assert_eq!(r.total_tags, 4, "expected 4 tags; got {}", r.total_tags);
}

#[test]
fn total_tags_empty() {
    let r = report("just text no tags");
    assert_eq!(r.total_tags, 0);
}

#[test]
fn total_tags_self_closing_counted() {
    // <br/> is normalised to <br> but still counted as 1 tag.
    let r = report("<br/>");
    assert_eq!(r.total_tags, 1);
}

// ── max_estimated_depth ───────────────────────────────────────────────────────

#[test]
fn max_depth_estimate() {
    let r = report("<div><div><div></div></div></div>");
    assert!(
        r.max_estimated_depth >= 3,
        "three nested divs should yield depth >= 3; got {}",
        r.max_estimated_depth
    );
}

#[test]
fn max_depth_flat() {
    // Sibling elements: depth never exceeds 1.
    let r = report("<p>a</p><p>b</p><p>c</p>");
    assert_eq!(
        r.max_estimated_depth, 1,
        "flat siblings should have depth 1; got {}",
        r.max_estimated_depth
    );
}

// ── cleaning behaviour (regression guard) ────────────────────────────────────

#[test]
fn empty_comment_normalised() {
    let out = cleaned("<!---->");
    assert_eq!(out, "<!-- -->");
}

#[test]
fn self_closing_br_normalised() {
    let out = cleaned("<br/>");
    assert_eq!(out, "<br>");
}

#[test]
fn script_content_stripped() {
    let out = cleaned("<p>before</p><script>alert(1)</script><p>after</p>");
    assert!(out.contains("<p>before</p>"));
    assert!(out.contains("<p>after</p>"));
    assert!(!out.contains("alert(1)"), "script body should be stripped");
}

#[test]
fn doctype_removed() {
    let out = cleaned("<!DOCTYPE html><html><body>hi</body></html>");
    assert!(
        !out.to_lowercase().contains("doctype"),
        "DOCTYPE should be stripped; got: {:?}",
        out
    );
}

#[test]
fn cow_borrowed_when_no_change() {
    // Input that requires zero transformations should return Borrowed.
    let html = "<p>hello world</p>";
    let (cow, _) = prescan::run(html);
    // Verify the pointer identity — Borrowed means the same underlying string data.
    assert!(
        matches!(cow, std::borrow::Cow::Borrowed(_)),
        "no-op input should return Cow::Borrowed"
    );
}
