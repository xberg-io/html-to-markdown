//! Regression tests for issue #772: an element with no end tag ends where the HTML standard says.
//!
//! Each expected output matches the tree that Google Chrome 155 builds for the input.

#![cfg(feature = "testkit")]

use std::fmt::Write;

use html_to_markdown_rs::{ConversionOptions, ConversionResult, TierStrategy, WarningKind, convert};

fn convert_with(html: &str, tier_strategy: TierStrategy) -> ConversionResult {
    let options = ConversionOptions {
        tier_strategy,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(options)).expect("conversion must succeed")
}

fn assert_all_routes(html: &str, expected: &str) {
    let default = convert(html, None).expect("conversion must succeed");
    assert_eq!(default.content.as_deref(), Some(expected), "default options: {html:?}");
    for tier_strategy in [TierStrategy::Tier2, TierStrategy::Tier1] {
        let result = convert_with(html, tier_strategy);
        assert_eq!(result.content.as_deref(), Some(expected), "{tier_strategy:?}: {html:?}");
    }
}

fn issue_page(blocks: usize, paragraph_end_tag: &str) -> String {
    let mut notes = String::new();
    for number in 1..=blocks {
        write!(notes, "<div class='note'><p>Note {number}.{paragraph_end_tag}</div>").expect("write to a string");
    }
    format!("<!doctype html><html><body><main>{notes}</main><footer><p>Built with care.</p></footer></body></html>")
}

fn issue_markdown(blocks: usize) -> String {
    let mut expected = String::new();
    for number in 1..=blocks {
        write!(expected, "Note {number}.\n\n").expect("write to a string");
    }
    expected.push_str("Built with care.\n");
    expected
}

#[test]
fn eighty_paragraphs_with_no_end_tag_keep_every_note_and_the_footer() {
    let html = issue_page(80, "");
    let expected = issue_markdown(80);
    assert_all_routes(&html, &expected);
    let result = convert(&html, None).expect("conversion must succeed");
    assert!(result.warnings.is_empty(), "warnings: {:?}", result.warnings);
}

#[test]
fn a_paragraph_with_no_end_tag_gives_the_output_of_a_closed_paragraph() {
    for blocks in [1, 29, 30, 31] {
        let closed = convert(&issue_page(blocks, "</p>"), None).expect("conversion must succeed");
        let open = convert(&issue_page(blocks, ""), None).expect("conversion must succeed");
        assert_eq!(closed.content.as_deref(), Some(issue_markdown(blocks).as_str()));
        assert_eq!(open.content, closed.content, "{blocks} blocks");
    }
}

#[test]
fn the_end_tag_of_the_parent_ends_an_open_paragraph() {
    assert_all_routes("<!doctype html><div><p>one</div>tail", "one\n\ntail\n");
    assert_all_routes(
        "<!doctype html><section><p>one</section><span>tail</span>",
        "one\n\ntail\n",
    );
    assert_all_routes(
        "<!doctype html><div><p>one</div><h2>Title</h2><ul><li>item</li></ul>",
        "one\n\n## Title\n\n- item\n",
    );
    assert_all_routes(
        "<!doctype html><blockquote><p>quoted</blockquote><p>after</p>",
        "> quoted\n\nafter\n",
    );
    assert_all_routes(
        "<!doctype html><table><tr><th>h</th></tr><tr><td><p>cell</td></tr></table><p>after</p>",
        "| h    |\n| ---- |\n| cell |\n\nafter\n",
    );
    assert_all_routes(
        "<!doctype html><details><summary>S</summary><p>one</details><p>two</p>",
        "**S**\n\none\n\ntwo\n",
    );
    assert_all_routes(
        "<!doctype html><div><p>one<!-- </div><div> --></div>tail",
        "one\n\ntail\n",
    );
}

#[test]
fn a_block_start_tag_ends_an_open_paragraph() {
    assert_all_routes("<!doctype html><p>one<div>two</div>three", "one\n\ntwo\n\nthree\n");
    assert_all_routes(
        "<!doctype html><p>one<ul><li>two</li></ul>three",
        "one\n\n- two\n\nthree\n",
    );
    assert_all_routes(
        "<!doctype html><p>one<table><tr><th>h</th></tr><tr><td>two</td></tr></table>three",
        "one\n\n| h   |\n| --- |\n| two |\n\nthree\n",
    );
    assert_all_routes("<!doctype html><p>one<h2>two</h2>three", "one\n\n## two\n\nthree\n");
    assert_all_routes(
        "<!doctype html><p>one<pre>two</pre>three",
        "one\n\n```\ntwo\n```\n\nthree\n",
    );
    assert_all_routes("<!doctype html><p>one<p>two<p>three", "one\n\ntwo\n\nthree\n");
}

#[test]
fn list_items_and_definitions_with_no_end_tag_stay_siblings() {
    assert_all_routes("<!doctype html><ul><li>one<li>two</ul>after", "- one\n- two\n\nafter\n");
    assert_all_routes("<!doctype html><ol><li>one</ol><p>after</p>", "1. one\n\nafter\n");
    assert_all_routes(
        "<!doctype html><ul><li><p>one<li><p>two</ul><p>after</p>",
        "- one\n\n- two\n\nafter\n",
    );
    assert_all_routes(
        "<!doctype html><dl><dt>term<dd>desc<dt>term2<dd>desc2</dl>after",
        "term\ndesc\n\nterm2\ndesc2\n\nafter\n",
    );
}

#[test]
fn table_parts_with_no_end_tag_keep_every_row() {
    let expected = "| a | b |\n| --- | --- |\n| c | d |\n\nafter\n";
    assert_all_routes(
        "<!doctype html><table><tr><th>a<th>b<tr><td>c<td>d</table>after",
        expected,
    );
    assert_all_routes(
        "<!doctype html><table><thead><tr><th>a<th>b<tbody><tr><td>c<td>d</table>after",
        expected,
    );
}

#[test]
fn a_document_with_no_end_tag_for_head_and_body_keeps_its_body() {
    let html = "<!doctype html><html><head><title>T</title><body><p>one<p>two";
    for tier_strategy in [TierStrategy::Tier2, TierStrategy::Tier1] {
        let result = convert_with(html, tier_strategy);
        assert_eq!(result.content.as_deref(), Some("one\n\ntwo\n"), "{tier_strategy:?}");
    }
    let default = convert(html, None).expect("conversion must succeed");
    assert_eq!(default.content.as_deref(), Some("---\ntitle: T\n---\n\none\n\ntwo\n"));
}

#[test]
fn an_end_tag_with_no_start_tag_changes_nothing() {
    assert_all_routes("<!doctype html><p>one</div>two</p><p>three</p>", "onetwo\n\nthree\n");
    assert_all_routes("<!doctype html>one</div>two</span>three", "onetwothree\n");
}

/// For an input where the fast converter writes other Markdown for the closed form too.
fn assert_default_and_tier2(html: &str, expected: &str) {
    let default = convert(html, None).expect("conversion must succeed");
    assert_eq!(default.content.as_deref(), Some(expected), "default options: {html:?}");
    let tier2 = convert_with(html, TierStrategy::Tier2);
    assert_eq!(tier2.content.as_deref(), Some(expected), "Tier2: {html:?}");
}

#[test]
fn a_block_in_an_open_list_item_or_section_ends_with_its_parent() {
    assert_all_routes(
        "<!doctype html><ul><li><div>one</div><li><div>two</ul>after",
        "- one\n\n- two\n\nafter\n",
    );
    assert_all_routes(
        "<!doctype html><ol><li>one<p>two<li>three</ol>after",
        "1. one\n\n   two\n\n2. three\n\nafter\n",
    );
    assert_all_routes(
        "<!doctype html><article><header><p>h</header><p>one</article><footer><p>f</footer>tail",
        "h\n\none\n\nf\n\ntail\n",
    );
    assert_all_routes("<!doctype html><dl><dt>a<dt>b<dd>c</dl>after", "a\nb\nc\n\nafter\n");
    assert_default_and_tier2(
        "<!doctype html><dl><dt>t<dd><p>d1<p>d2</dl>after",
        "t\nd1\n\nd2\n\nafter\n",
    );
}

#[test]
fn options_with_no_end_tag_give_the_output_of_closed_options() {
    assert_all_routes(
        "<!doctype html><select><option>one<option>two</select>after",
        "onetwoafter\n",
    );
    assert_all_routes(
        "<!doctype html><p>before <select><option>one<option>two</select> after<p>next",
        "before onetwo after\n\nnext\n",
    );
    let closed = "<!doctype html><select><optgroup label=a><option>x</option></optgroup>\
                  <optgroup label=b><option>y</option></optgroup></select>after";
    let open = "<!doctype html><select><optgroup label=a><option>x<optgroup label=b><option>y</select>after";
    for tier_strategy in [TierStrategy::Tier2, TierStrategy::Tier1] {
        assert_eq!(
            convert_with(open, tier_strategy).content,
            convert_with(closed, tier_strategy).content,
            "{tier_strategy:?}"
        );
    }
    assert_eq!(
        convert(open, None).expect("conversion must succeed").content,
        convert(closed, None).expect("conversion must succeed").content
    );
}

#[test]
fn ruby_text_with_no_end_tag_ends_at_the_next_ruby_part() {
    assert_all_routes(
        "<!doctype html><p><ruby>base<rt>top</ruby> after</p>",
        "base(top) after\n",
    );
    assert_all_routes("<!doctype html><p><ruby>a<rt>b<rt>c</ruby> after", "a(b)(c) after\n");
    assert_all_routes(
        "<!doctype html><p><ruby>kan<rp>(<rt>ji<rp>)</ruby> after<p>next",
        "kan(ji) after\n\nnext\n",
    );
}

#[test]
fn each_table_part_with_no_end_tag_keeps_every_row() {
    let two_columns = "| a | b |\n| --- | --- |\n| c | d |\n\nafter\n";
    assert_all_routes(
        "<!doctype html><table><tr><th>a</th><th>b</th><tr><td>c</td><td>d</td></table>after",
        two_columns,
    );
    assert_all_routes(
        "<!doctype html><table><tr><th>a<th>b</tr><tr><td>c</td><td>d</td></tr></table>after",
        two_columns,
    );
    assert_all_routes(
        "<!doctype html><table><tr><th>a</th><th>b</th></tr><tr><td>c<td>d</tr></table>after",
        two_columns,
    );
    assert_all_routes(
        "<!doctype html><table><thead><tr><th>a</th></tr><tbody><tr><td>b</td></tr></tbody></table>after",
        "| a |\n| --- |\n| b |\n\nafter\n",
    );
    assert_all_routes(
        "<!doctype html><table><thead><tr><th>h</th></tr></thead><tbody><tr><td>a</td></tr>\
         <tbody><tr><td>b</td></tr><tfoot><tr><td>f</td></tr></table>after",
        "| h |\n| --- |\n| a |\n| b |\n| f |\n\nafter\n",
    );
}

#[test]
fn a_head_with_no_end_tag_ends_at_the_first_body_content() {
    for (html, body) in [
        (
            "<!doctype html><html><head><title>T</title><body><p>one</p></body></html>",
            "one\n",
        ),
        (
            "<!doctype html><html><head><title>T</title><meta charset=utf-8><p>one<p>two",
            "one\n\ntwo\n",
        ),
        (
            "<!doctype html><html><head><title>T</title><div><p>one</div>tail",
            "one\n\ntail\n",
        ),
    ] {
        for tier_strategy in [TierStrategy::Tier2, TierStrategy::Tier1] {
            let result = convert_with(html, tier_strategy);
            assert_eq!(result.content.as_deref(), Some(body), "{tier_strategy:?}: {html:?}");
        }
        let default = convert(html, None).expect("conversion must succeed");
        let expected = format!("---\ntitle: T\n---\n\n{body}");
        assert_eq!(default.content.as_deref(), Some(expected.as_str()), "{html:?}");
    }
    assert_all_routes("<!doctype html><body><p>one</body><p>two", "one\n\ntwo\n");
}

#[test]
fn an_end_tag_with_no_start_tag_ends_no_other_element() {
    assert_all_routes(
        "<!doctype html><ul><li>one</li></li><li>two</li></ul></ul>after",
        "- one\n- two\n\nafter\n",
    );
    assert_all_routes(
        "<!doctype html><table><tr><th>a</th></th><th>b</th></tr></tr><tr><td>c</td><td>d</td></tr></table></table>after",
        "| a | b |\n| --- | --- |\n| c | d |\n\nafter\n",
    );
    assert_all_routes("<!doctype html></div><p>one</p></section>", "one\n");
    assert_all_routes(
        "<!doctype html><p>one</b> two</em> three</p><p>four</p>",
        "one two three\n\nfour\n",
    );
    assert_all_routes("<!doctype html><p>one</p></body></html>", "one\n");
}

#[test]
fn an_end_tag_in_another_spelling_ends_its_element() {
    assert_all_routes("<!doctype html><DIV><p>one</p></div><p>two</p>", "one\n\ntwo\n");
    assert_all_routes("<!doctype html><div><p>one</p></div ><p>two</p>", "one\n\ntwo\n");
    assert_all_routes(
        "<!doctype html><DIV><P>one</DIV><DIV><P>two</DIV>tail",
        "one\n\ntwo\n\ntail\n",
    );
    // ~keep The end tag of any heading ends an open heading.
    assert_default_and_tier2("<!doctype html><h2>one</h3>two", "## one\n\ntwo\n");
}

#[test]
fn a_page_that_nests_past_the_limit_is_still_cut_and_reports_it() {
    let html = format!(
        "<!doctype html>{}<p>Deep text.</p>{}<p>Shallow text.</p>",
        "<div>".repeat(70),
        "</div>".repeat(70)
    );
    let default = convert(&html, None).expect("conversion must succeed");
    let routes = [
        default,
        convert_with(&html, TierStrategy::Tier2),
        convert_with(&html, TierStrategy::Tier1),
    ];
    for result in routes {
        assert_eq!(result.content.as_deref(), Some("Shallow text.\n"));
        assert_eq!(result.warnings.len(), 1, "warnings: {:?}", result.warnings);
        assert!(matches!(result.warnings[0].kind, WarningKind::DepthLimitExceeded));
    }
}
