#![allow(missing_docs)]
#![cfg(feature = "testkit")]

//! White space between inline content (issues #751, #762, #778, #795, #800, #803): white space in
//! the source is one space in the output, no white space is no space, and a block boundary inside
//! a link label or a heading separates words. Every input runs on both converters.

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::{
    CodeBlockStyle, ConversionOptions, HighlightStyle, TierStrategy, WhitespaceMode, convert, tier1,
};

fn tier_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        ..ConversionOptions::default()
    }
}

fn convert_with(html: &str, options: Option<ConversionOptions>) -> String {
    convert(html, options)
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

fn tier2(html: &str) -> String {
    convert_with(
        html,
        Some(ConversionOptions {
            tier_strategy: TierStrategy::Tier2,
            ..tier_options()
        }),
    )
}

/// What the two converters and the default options write for `html`, or the reason they differ.
/// `fast_converter_bails` says whether the fast converter gives this input to the other one.
fn convert_everywhere(html: &str, fast_converter_bails: bool) -> Result<String, String> {
    let full = tier2(html);
    let default = convert_with(html, None);
    if default != full {
        return Err(format!("default options give {default:?}, the full converter {full:?}"));
    }
    match (
        tier1::run(html, &PrescanReport::default(), &tier_options()),
        fast_converter_bails,
    ) {
        (Ok(fast), false) if fast == full => Ok(full),
        (Ok(fast), false) => Err(format!("fast converter gives {fast:?}, the full converter {full:?}")),
        (Ok(fast), true) => Err(format!("fast converter no longer bails: it gives {fast:?}")),
        (Err(_), true) => Ok(full),
        (Err(reason), false) => Err(format!("fast converter bails: {reason}")),
    }
}

fn assert_cases(cases: &[(&str, &str)], fast_converter_bails: bool) {
    let wrong: Vec<String> = cases
        .iter()
        .filter_map(
            |(html, expected)| match convert_everywhere(html, fast_converter_bails) {
                Ok(actual) if actual == *expected => None,
                Ok(actual) => Some(format!("{html:?}\n  expected {expected:?}\n  actual   {actual:?}")),
                Err(reason) => Some(format!("{html:?}\n  {reason}")),
            },
        )
        .collect();
    assert!(wrong.is_empty(), "{} wrong:\n{}", wrong.len(), wrong.join("\n"));
}

/// Cases that both converters convert.
fn assert_on_both(cases: &[(&str, &str)]) {
    assert_cases(cases, false);
}

/// Cases that the fast converter gives to the full one.
fn assert_on_full(cases: &[(&str, &str)]) {
    assert_cases(cases, true);
}

#[test]
fn a_line_break_before_an_inline_element_is_a_space() {
    assert_on_both(&[
        (
            "<p>hard to read\n<span id=\"index-0\"></span>docstrings. Compare</p>",
            "hard to read docstrings. Compare\n",
        ),
        (
            "<p>hard to read\n<span class=\"target\" id=\"index-0\"></span><a href=\"/pep\"><strong>docstrings</strong></a>. Compare</p>",
            "hard to read [**docstrings**](/pep). Compare\n",
        ),
        ("<p>one\n<span>two</span> three</p>", "one two three\n"),
        ("<p>one\n<span>two</span>\nthree</p>", "one two three\n"),
        (
            "<p>one\n<span id=\"a\"></span><span id=\"b\"></span>two</p>",
            "one two\n",
        ),
        (
            "<p>one\n<span id=\"a\"></span>two\n<span id=\"b\"></span>three</p>",
            "one two three\n",
        ),
        (
            "<p>first</p><p>one\n<span id=\"a\"></span>two</p>",
            "first\n\none two\n",
        ),
        ("<p><em>one\n<span id=\"a\"></span>two</em></p>", "*one two*\n"),
        (
            "<p><a href=\"/x\">one\n<span id=\"a\"></span>two</a></p>",
            "[one two](/x)\n",
        ),
        (
            "<p>zero\n<span class=\"ocrx_word\">one</span>\n<span class=\"ocrx_word\">two</span></p>",
            "zero one two\n",
        ),
    ]);
}

/// The container does not change the rule: where a page shows a space, the output has a space,
/// not a line end.
#[test]
fn a_line_break_before_an_inline_element_is_a_space_in_each_container() {
    assert_on_both(&[
        ("<p>one\n<span id=\"a\"></span>two</p>", "one two\n"),
        ("<div>one\n<span id=\"a\"></span>two</div>", "one two\n"),
        ("<ul><li>one\n<span id=\"a\"></span>two</li></ul>", "- one two\n"),
        (
            "<table><tr><th>h</th></tr><tr><td>one\n<span id=\"a\"></span>two</td></tr></table>",
            "| h       |\n| ------- |\n| one two |\n",
        ),
        ("<blockquote>one\n<span id=\"a\"></span>two</blockquote>", "> one two\n"),
        ("<h2>one\n<span id=\"a\"></span>two</h2>", "## one two\n"),
        ("one\n<span id=\"a\"></span>two", "one two\n"),
        (
            "<dl><dt>t</dt><dd>one\n<span id=\"a\"></span>two</dd></dl>",
            "t\none two\n",
        ),
    ]);
}

/// Before an element that the converter does not know as inline (a custom element), a line end
/// is a space in running text: a paragraph, a heading, an inline element. In a `<div>` and a list
/// item it stays a line end, as it was before.
#[test]
fn a_line_break_before_a_custom_element_is_a_space_in_running_text_only() {
    assert_on_both(&[
        ("<p>one\n<x-y>two</x-y></p>", "one two\n"),
        ("<div><b>one\n<x-y>two</x-y></b></div>", "**one two**\n"),
        ("<h2>one\n<x-y>two</x-y></h2>", "## one two\n"),
        ("<div>one\n<x-y>two</x-y></div>", "one\ntwo\n"),
        ("<ul><li>one\n<x-y>two</x-y></li></ul>", "- one\n  two\n"),
    ]);
}

/// A span that holds only white space after a line end is the same run of white space.
#[test]
fn a_line_break_before_white_space_in_an_element_is_one_space_in_each_container() {
    assert_on_both(&[
        ("<p>one\n<span> </span>two</p>", "one two\n"),
        ("<div>one\n<span> </span>two</div>", "one two\n"),
        ("<ul><li>one\n<span> </span>two</li></ul>", "- one two\n"),
        (
            "<table><tr><th>h</th></tr><tr><td>one\n<span> </span>two</td></tr></table>",
            "| h       |\n| ------- |\n| one two |\n",
        ),
        ("<blockquote>one\n<span> </span>two</blockquote>", "> one two\n"),
        ("<h2>one\n<span> </span>two</h2>", "## one two\n"),
        ("one\n<span> </span>two", "one two\n"),
        ("<div>one\n<span> </span> two</div>", "one two\n"),
        ("<ul><li>one\n<span> </span>\ntwo</li></ul>", "- one two\n"),
        ("<ul><li>one\n<span>&nbsp;</span>two</li></ul>", "- one \u{a0}two\n"),
        ("one\n<span>&nbsp;</span>two", "one \u{a0}two\n"),
    ]);
}

/// The element that follows the line end holds text: the same space in each container.
#[test]
fn a_line_break_before_a_filled_inline_element_is_a_space_in_each_container() {
    assert_on_both(&[
        ("<p>one\n<b>two</b></p>", "one **two**\n"),
        ("<div>one\n<b>two</b></div>", "one **two**\n"),
        ("<ul><li>one\n<b>two</b></li></ul>", "- one **two**\n"),
        (
            "<table><tr><th>h</th></tr><tr><td>one\n<b>two</b></td></tr></table>",
            "| h           |\n| ----------- |\n| one **two** |\n",
        ),
        ("<blockquote>one\n<b>two</b></blockquote>", "> one **two**\n"),
        ("<h2>one\n<b>two</b></h2>", "## one **two**\n"),
        ("one\n<b>two</b>", "one **two**\n"),
        ("<div>one\n<span>two</span> three</div>", "one two three\n"),
        (
            "<div>see\n<a href=\"/x\">two</a>\n<code>three</code></div>",
            "see [two](/x) `three`\n",
        ),
        ("<div>one\n<br>two</div>", "one  \ntwo\n"),
        // ~keep A comment before the element changes nothing.
        ("<div>one\n<!-- c --><b>two</b></div>", "one **two**\n"),
        ("<div>one\n<!-- c --> <!-- d --><b>two</b></div>", "one **two**\n"),
        ("<div>one\n<!-- c --><br>two</div>", "one  \ntwo\n"),
    ]);
}

/// An inline element whose content starts with a block starts a line of its own in a browser:
/// the line end before it stays a line end, and a heading in a list item stays a heading.
#[test]
fn a_line_break_before_an_inline_element_that_starts_with_a_block_stays_a_line_break() {
    assert_on_both(&[
        ("<div>one\n<b><p>y</p></b></div>", "one\n**y**\n"),
        ("one\n<b><p>y</p></b>", "one\n**y**\n"),
        ("<blockquote>one\n<b><p>y</p></b></blockquote>", "> one\n> **y**\n"),
        ("<div>one\n<b> <!-- c --> <p>y</p></b></div>", "one\n**y**\n"),
        ("<div>one\n<!-- c --><b><p>y</p></b></div>", "one\n**y**\n"),
        ("<div>one\n<b><i><p>y</p></i></b></div>", "one\n***y***\n"),
        (
            "<blockquote>one\n<span><h3>y</h3></span></blockquote>",
            "> one\n>\n> ### y\n",
        ),
        (
            "<details><summary>one\n<span><div>y</div></span></summary>body</details>",
            "**one**\n\n**y**\n\nbody\n",
        ),
        // ~keep A `<span>` writes no marks: the block in it breaks the line as it does alone.
        (
            "<blockquote>one\n<span><div>y</div></span></blockquote>",
            "> one\n>\n> y\n",
        ),
        (
            "<blockquote>one\n<span><ul><li>y</li></ul></span>two</blockquote>",
            "> one\n>\n> - y\n>\n> two\n",
        ),
        ("<div>one\n<span><b><p>y</p></b></span></div>", "one\n**y**\n"),
        // ~keep Text before the block, or no block: the element is inline content.
        ("<div>one\n<b>x<p>y</p></b></div>", "one **x**\n\n**y**\n"),
        ("<div>one\n<b><i>y</i></b></div>", "one ***y***\n"),
        ("<div>one\n<b>y</b></div>", "one **y**\n"),
    ]);
    assert_on_full(&[
        ("<div>one\n<a href=\"/y\"><div>y</div></a></div>", "one\n[y](/y)\n"),
        (
            "<ul><li>one\n<a href=\"/y\"><div>y</div></a></li></ul>",
            "- one\n[y](/y)\n",
        ),
        (
            "<ul><li>one\n<a href=\"/y\"><h3>y</h3></a></li></ul>",
            "- one\n    ### [y](/y)\n",
        ),
        (
            "<ol><li>one\n<a href=\"/y\"><h2>Title</h2></a></li></ol>",
            "1. one\n    ## [Title](/y)\n",
        ),
        ("<ul><li>one\n<b><p>y</p></b></li></ul>", "- one\n**y**\n"),
    ]);
}

/// White space before an element and white space at its start are one run: one space, and it
/// goes outside the marks of the element.
#[test]
fn white_space_on_both_sides_of_an_element_start_is_one_space() {
    assert_on_both(&[
        ("<p>one <b> y</b>two</p>", "one **y**two\n"),
        ("<p>one <b> y</b> two</p>", "one **y** two\n"),
        ("<p>one <b> y </b>two</p>", "one **y** two\n"),
        ("<p>one <b> y </b> two</p>", "one **y** two\n"),
        ("<p>one <b>\ny</b>two</p>", "one **y**two\n"),
        ("<p>one\n<b> y</b>two</p>", "one **y**two\n"),
        ("<p>one\t<b>\ty</b>two</p>", "one **y**two\n"),
        ("<p><b>one </b><i> two</i></p>", "**one** *two*\n"),
        ("<p>one <em> y</em>two</p>", "one *y*two\n"),
        ("<p>one <del> y</del>two</p>", "one ~~y~~two\n"),
        ("<p>one <b> y</b></p>", "one **y**\n"),
        ("<ul><li><b> y</b>one</li></ul>", "- **y**one\n"),
        ("<ul><li>one <b> y</b>two</li></ul>", "- one **y**two\n"),
        ("<div>one <b> y</b>two</div>", "one **y**two\n"),
        ("<blockquote>one <b> y</b>two</blockquote>", "> one **y**two\n"),
        ("<h2>one <b> y</b>two</h2>", "## one **y**two\n"),
        ("one <b> y</b>two", "one **y**two\n"),
        (
            "<table><tr><th>h</th></tr><tr><td>one <b> y</b>two</td></tr></table>",
            "| h            |\n| ------------ |\n| one **y**two |\n",
        ),
    ]);
}

/// The white space at the start of an element is the only white space there: it is kept.
#[test]
fn white_space_at_an_element_start_alone_is_one_space() {
    assert_on_both(&[
        ("<p>one<b> y</b>two</p>", "one **y**two\n"),
        ("<p>one<b> y </b>two</p>", "one **y** two\n"),
        ("<p>one<em>\ny</em>two</p>", "one *y*two\n"),
        ("<p>one <b>y</b>two</p>", "one **y**two\n"),
        ("<p>one<b>y</b>two</p>", "one**y**two\n"),
    ]);
}

/// The other inline elements that write their own marks follow the same rule.
#[test]
fn white_space_on_both_sides_of_the_start_of_any_marked_element_is_one_space() {
    assert_on_both(&[
        ("<p>one <kbd> y</kbd>two</p>", "one `y`two\n"),
        ("<p>one <abbr> y</abbr>two</p>", "one ytwo\n"),
        ("<p>one <dfn> y</dfn>two</p>", "one *y*two\n"),
        ("<p>one <ins> y</ins>two</p>", "one ==y==two\n"),
        ("<p>one <b><i> y</i></b>two</p>", "one ***y***two\n"),
    ]);
}

/// A zero-width space is a place where a line can break, not a space. A browser drops the line
/// end beside it, so the two parts stay one word.
#[test]
fn a_line_end_before_a_zero_width_space_is_no_space() {
    assert_on_both(&[
        ("<p>one\n<span></span>\u{200b}two</p>", "one\u{200b}two\n"),
        ("<p>one\n<span id=\"a\"></span>&#8203;two</p>", "one\u{200b}two\n"),
        ("<p>one\n<span>\u{200b}</span>two</p>", "one\u{200b}two\n"),
        ("<p>one\n<span>\u{200b}</span> two</p>", "one\u{200b} two\n"),
        ("<p>one\n<b>\u{200b}two</b></p>", "one**\u{200b}two**\n"),
        (
            "<p>one\n<span><i></i></span><!-- c -->\u{200b}two</p>",
            "one\u{200b}two\n",
        ),
        ("<p>one \n <span></span>\u{200b}two</p>", "one\u{200b}two\n"),
        ("<p>one\n<wbr>\u{200b}two</p>", "one\u{200b}two\n"),
        ("<ul><li>one\n<span></span>\u{200b}two</li></ul>", "- one\u{200b}two\n"),
        ("<h2>one\n<span></span>\u{200b}two</h2>", "## one\u{200b}two\n"),
        (
            "<p><a href=\"/x\">one\n<span></span>\u{200b}two</a></p>",
            "[one\u{200b}two](/x)\n",
        ),
        ("<div>one\n<span></span>\u{200b}two</div>", "one\u{200b}two\n"),
        (
            "<blockquote>one\n<span></span>\u{200b}two</blockquote>",
            "> one\u{200b}two\n",
        ),
        ("one\n<span></span>\u{200b}two", "one\u{200b}two\n"),
        // ~keep The line end is the last thing in an element and the zero-width space follows
        // ~keep that element.
        ("<p><span>one\n</span>\u{200b}two</p>", "one\u{200b}two\n"),
        ("<p><b>one\n</b>\u{200b}two</p>", "**one**\u{200b}two\n"),
        ("<p><b><i>one\n</i></b>\u{200b}two</p>", "***one***\u{200b}two\n"),
        ("<p><b>one\n</b><i>\u{200b}two</i></p>", "**one***\u{200b}two*\n"),
        ("<ul><li><span>one\n</span>\u{200b}two</li></ul>", "- one\u{200b}two\n"),
        ("<p><b>one\n</b><span></span>\u{200b}two</p>", "**one**\u{200b}two\n"),
        (
            "<p><b>x</b> <a href=\"/x\">one\n</a>\u{200b}two</p>",
            "**x** [one](/x)\u{200b}two\n",
        ),
        (
            "<table><tr><td>one\n<span></span>\u{200b}two</td></tr></table>",
            "| one\u{200b}two |\n| ------- |\n",
        ),
        (
            "<table><tr><td>one\n<!-- c -->\u{200b}two</td></tr></table>",
            "| one\u{200b}two |\n| ------- |\n",
        ),
    ]);
}

/// A space that is not a line end stays beside a zero-width space, and so does a line end
/// that content of its own separates from that character.
#[test]
fn a_space_beside_a_zero_width_space_is_kept() {
    assert_on_both(&[
        ("<p>one <span></span>\u{200b}two</p>", "one \u{200b}two\n"),
        ("<p>one\n<span></span>two</p>", "one two\n"),
        ("<p>one\n<wbr>two</p>", "one two\n"),
        (
            "<p>one\n<img src=\"/i.png\" alt=\"i\">\u{200b}two</p>",
            "one ![i](/i.png)\u{200b}two\n",
        ),
        ("<p>one\n<span>x</span>\u{200b}two</p>", "one x\u{200b}two\n"),
        ("<p><span>one </span>\u{200b}two</p>", "one \u{200b}two\n"),
        // ~keep The line end is the last thing in an element with a style, or in a block.
        (
            "<p><span style=\"display:inline-block\">one\n</span>\u{200b}two</p>",
            "one \u{200b}two\n",
        ),
        ("<div><span>one\n</span></div>\u{200b}two", "one\n\n\u{200b}two\n"),
        // ~keep An element with a style can be a box of its own, and a browser keeps the
        // ~keep line end before such a box.
        (
            "<p>one\n<span style=\"display:inline-block\"></span>\u{200b}two</p>",
            "one \u{200b}two\n",
        ),
        (
            "<table><tr><td>one \n <span style=\"display:inline-block\"></span>\u{200b}two</td></tr></table>",
            "| one \u{200b}two |\n| -------- |\n",
        ),
        (
            "<pre>one\n<span></span>\u{200b}two</pre>",
            "```\none\n\u{200b}two\n```\n",
        ),
    ]);
}

/// The name of the `style` attribute has no letter case: both converters read `STYLE` and
/// `Style` as `style`.
#[test]
fn the_style_attribute_name_is_read_in_any_letter_case() {
    assert_on_both(&[
        (
            "<p>one\n<span STYLE=\"color:red\">&#8203;two</span></p>",
            "one \u{200b}two\n",
        ),
        (
            "<p>one\n<span Style=\"display:inline-block\"></span>&#8203;two</p>",
            "one \u{200b}two\n",
        ),
        (
            "<p><span STYLE=\"display:inline-block\">one\n</span>\u{200b}two</p>",
            "one \u{200b}two\n",
        ),
        (
            "<table><tr><td>one\n<span STYLE=\"color:red\">&#8203;two</span></td></tr></table>",
            "| one \u{200b}two |\n| -------- |\n",
        ),
    ]);
}

/// A link that has no text still writes its marks: the space after it is kept at the start of
/// a document, on both converters.
#[test]
fn the_space_after_a_link_without_text_is_kept() {
    assert_on_both(&[
        ("<p><a href=\"/y\"></a> one</p>", "[](/y) one\n"),
        ("<div><a href=\"/y\"></a> one</div>", "[](/y) one\n"),
        ("<a href=\"/y\"></a> one", "[](/y) one\n"),
        ("<p> <a href=\"/y\"></a>\none </p>", "[](/y) one\n"),
        ("<p><a id=\"n\"></a> one</p>", "one\n"),
    ]);
}

/// A link label is written without the white space at its two ends. That white space is still
/// white space between two words: it is one space outside the link (issue #800).
#[test]
fn white_space_at_an_end_of_a_link_label_is_one_space_outside_the_link() {
    assert_on_both(&[
        ("<p>Press <a href=\"/p\">Go </a>now.</p>", "Press [Go](/p) now.\n"),
        ("<p>Press<a href=\"/p\"> Go</a> now.</p>", "Press [Go](/p) now.\n"),
        ("<p>Press<a href=\"/p\"> Go </a>now.</p>", "Press [Go](/p) now.\n"),
        ("<p>Press <a href=\"/p\">Go\n</a>now.</p>", "Press [Go](/p) now.\n"),
        ("<p>Press<a href=\"/p\">\nGo</a> now.</p>", "Press [Go](/p) now.\n"),
        ("<p>Press <a href=\"/p\">Go\t</a>now.</p>", "Press [Go](/p) now.\n"),
        ("<p>Press <a href=\"/p\">Go </a> now.</p>", "Press [Go](/p) now.\n"),
        ("<p>Press <a href=\"/p\"> Go</a> now.</p>", "Press [Go](/p) now.\n"),
        (
            "<p>Press <a href=\"/p\" title=\"t\">Go </a>now.</p>",
            "Press [Go](/p \"t\") now.\n",
        ),
        ("Press <a href=\"/p\">Go </a>now.", "Press [Go](/p) now.\n"),
        ("Press<a href=\"/p\"> Go</a> now.", "Press [Go](/p) now.\n"),
        ("<div>Press <a href=\"/p\">Go </a>now.</div>", "Press [Go](/p) now.\n"),
        (
            "<ul><li>Press <a href=\"/p\">Go </a>now.</li></ul>",
            "- Press [Go](/p) now.\n",
        ),
        (
            "<ul><li>Press<a href=\"/p\"> Go</a> now.</li></ul>",
            "- Press [Go](/p) now.\n",
        ),
        ("<h2>Press <a href=\"/p\">Go </a>now.</h2>", "## Press [Go](/p) now.\n"),
        (
            "<blockquote><p>Press <a href=\"/p\">Go </a>now.</p></blockquote>",
            "> Press [Go](/p) now.\n",
        ),
        (
            "<table><tr><th>h</th></tr><tr><td>Press <a href=\"/p\">Go </a>now.</td></tr></table>",
            "| h                   |\n| ------------------- |\n| Press [Go](/p) now. |\n",
        ),
        (
            "<table><tr><th>h</th></tr><tr><td>Press<a href=\"/p\"> Go</a> now.</td></tr></table>",
            "| h                   |\n| ------------------- |\n| Press [Go](/p) now. |\n",
        ),
        (
            "<p><a href=\"/a\">one </a><a href=\"/b\">two</a></p>",
            "[one](/a) [two](/b)\n",
        ),
        (
            "<p><a href=\"/a\">one</a><a href=\"/b\"> two</a></p>",
            "[one](/a) [two](/b)\n",
        ),
        ("<p><a href=\"/a\">one </a><b>two</b></p>", "[one](/a) **two**\n"),
        (
            "<p>Press <a href=\"/p\"><b>Go </b></a>now.</p>",
            "Press [**Go**](/p) now.\n",
        ),
        (
            "<p>Press<a href=\"/p\"><b> Go</b></a> now.</p>",
            "Press [**Go**](/p) now.\n",
        ),
        (
            "<p>Press <a href=\"/p\"><span>Go </span></a>now.</p>",
            "Press [Go](/p) now.\n",
        ),
        (
            "<p>Press <a href=\"https://e.org/\">https://e.org/ </a>now.</p>",
            "Press <https://e.org/> now.\n",
        ),
        (
            "<p>Press <a href=\"/p\"><img src=\"/i.png\" alt=\"Go\"> </a>now.</p>",
            "Press [![Go](/i.png)](/p) now.\n",
        ),
        (
            "<p>Press<a href=\"/p\"> <img src=\"/i.png\" alt=\"Go\"></a> now.</p>",
            "Press [![Go](/i.png)](/p) now.\n",
        ),
        ("<p>Press <a>Go </a>now.</p>", "Press Go now.\n"),
        ("<p>Press<a> Go</a> now.</p>", "Press Go now.\n"),
        ("<p>Press<a> Go </a>now.</p>", "Press Go now.\n"),
        ("<p>Press <a href=\"/p\">Go&nbsp;</a>now.</p>", "Press [Go](/p) now.\n"),
        (
            "<p>Press<a href=\"https://e.org/\"> https://e.org/</a> now.</p>",
            "Press <https://e.org/> now.\n",
        ),
        (
            "<p>Press<a href=\"/p\">\n<img src=\"/i.png\" alt=\"Go\"></a> now.</p>",
            "Press [![Go](/i.png)](/p) now.\n",
        ),
        (
            "<p>Press<a href=\"/p\"><span> Go</span></a> now.</p>",
            "Press [Go](/p) now.\n",
        ),
        (
            "<p>Press <b><a href=\"/p\">Go </a></b>now.</p>",
            "Press **[Go](/p)** now.\n",
        ),
    ]);
}

/// The white space at an end of a label is no space at the start or the end of a line, and a
/// label with no white space at its ends gets no space.
#[test]
fn a_link_label_gets_no_space_that_the_source_does_not_have() {
    assert_on_both(&[
        ("<p>Press<a href=\"/p\">Go</a>now.</p>", "Press[Go](/p)now.\n"),
        ("<p>Press <a href=\"/p\">Go </a></p>", "Press [Go](/p)\n"),
        ("<p><a href=\"/p\"> Go</a> now.</p>", "[Go](/p) now.\n"),
        ("<a href=\"/p\"> Go</a> now.", "[Go](/p) now.\n"),
        ("<ul><li><a href=\"/p\"> Go </a></li></ul>", "- [Go](/p)\n"),
        (
            "<p>Press<a href=\"/p\"><img src=\"/i.png\" alt=\"Go\"></a>now.</p>",
            "Press[![Go](/i.png)](/p)now.\n",
        ),
        ("<p>Press<a>Go</a>now.</p>", "PressGonow.\n"),
        ("<p>x</p><p><a href=\"/p\"> Go</a> now.</p>", "x\n\n[Go](/p) now.\n"),
        ("<h2><a href=\"/p\"> Go </a></h2>", "## [Go](/p)\n"),
        (
            "<table><tr><th>h</th></tr><tr><td><a href=\"/p\"> Go </a>now.</td></tr></table>",
            "| h             |\n| ------------- |\n| [Go](/p) now. |\n",
        ),
        (
            "<p><a href=\"https://e.org/\"> https://e.org/</a> now.</p>",
            "<https://e.org/> now.\n",
        ),
        (
            "<div><a href=\"https://e.org/\"> https://e.org/</a> now.</div>",
            "<https://e.org/> now.\n",
        ),
        // ~keep A line break at an end of the label is no white space.
        ("<p>Press<a href=\"/p\"><br>Go</a> now.</p>", "Press[  \nGo](/p) now.\n"),
        ("<p>Press <a href=\"/p\">Go<br></a>now.</p>", "Press [Go  \n](/p)now.\n"),
    ]);
}

/// The white space at the end of a label is one space before a period too: a browser shows
/// `Press Go .` for the first input and `Press Go.` for the second.
#[test]
fn white_space_at_the_end_of_a_link_label_is_one_space_before_a_period() {
    assert_on_both(&[
        ("<p>Press <a href=\"/p\">Go </a>.</p>", "Press [Go](/p) .\n"),
        ("<p>Press <a href=\"/p\">Go</a>.</p>", "Press [Go](/p).\n"),
    ]);
}

/// A heading and a table cell cannot hold a line break, so a `<br>` is a space there. At an end
/// of a link label that space is white space of the label: one space outside the link.
#[test]
fn a_line_break_at_an_end_of_a_link_label_is_one_space_outside_the_link_in_a_heading_or_a_cell() {
    assert_on_both(&[
        ("<h2>Press<a href=\"/p\">Go<br></a>now</h2>", "## Press[Go](/p) now\n"),
        ("<h2>Press <a href=\"/p\">Go<br></a>.</h2>", "## Press [Go](/p) .\n"),
        ("<h2>Press<a href=\"/p\"><br>Go</a>now</h2>", "## Press [Go](/p)now\n"),
        (
            "<table><tr><td>Press<a href=\"/p\">Go<br></a>now</td></tr></table>",
            "| Press[Go](/p) now |\n| ----------------- |\n",
        ),
        (
            "<table><tr><td>Press <a href=\"/p\">Go<br></a>.</td></tr></table>",
            "| Press [Go](/p) . |\n| ---------------- |\n",
        ),
        (
            "<table><tr><th>Press<a href=\"/p\">Go<br></a>now</th></tr><tr><td>c</td></tr></table>",
            "| Press[Go](/p) now |\n| ----------------- |\n| c                 |\n",
        ),
        // ~keep Nothing is owed at the end of the heading or the cell, or where a space is written.
        ("<h2><a href=\"/p\">Go<br></a></h2>", "## [Go](/p)\n"),
        ("<h2>Press <a href=\"/p\">Go<br></a> now</h2>", "## Press [Go](/p) now\n"),
    ]);
}

/// A code span keeps the white space of its text. The white space at an end of a link label is
/// not written a second time outside the link there. A `<pre>` block keeps it, as a browser does.
#[test]
fn white_space_at_an_end_of_a_link_label_is_not_moved_in_a_code_span() {
    assert_on_both(&[
        (
            "<p><code>Press <a href=\"/p\"> Go </a> now</code></p>",
            "`Press [Go](/p) now`\n",
        ),
        (
            "<p><code>Press <a href=\"/p\">Go </a> now</code></p>",
            "`Press [Go](/p) now`\n",
        ),
        (
            "<p><code>Press<a href=\"/p\"> Go </a>now</code></p>",
            "`Press[Go](/p)now`\n",
        ),
        ("<p><code><a href=\"/p\"> Go </a></code></p>", "`[Go](/p)`\n"),
        (
            "<p><code><a href=\"/p\"> <img src=\"/i.png\" alt=\"Go\"> </a></code></p>",
            "`[![Go](/i.png)](/p)`\n",
        ),
        (
            "<p><kbd>Press <a href=\"/p\"> Go </a> now</kbd></p>",
            "`Press [Go](/p) now`\n",
        ),
        // ~keep A link with no address is running text of the span, with its white space.
        ("<p><code>Press<a>Go </a>now</code></p>", "`PressGo now`\n"),
        // ~keep A `<pre>` block shows the label as it is written.
        (
            "<pre>Press<a href=\"/p\"> Go </a>now</pre>",
            "```\nPress [Go](/p) now\n```\n",
        ),
    ]);
}

/// A block in marks in a heading is a word of its own: one space after the closing marks. The
/// fast converter gives this input to the full one.
#[test]
fn a_block_in_marks_in_a_heading_is_a_word_of_its_own() {
    assert_on_full(&[
        ("<h2>one\n<b><p>y</p></b>two</h2>", "## one **y** two\n"),
        ("<h2>one <i><div>y</div></i>two</h2>", "## one *y* two\n"),
    ]);
    assert_on_both(&[("<h2>one <b>y</b>two</h2>", "## one **y**two\n")]);
}

/// A sectioning element (`<footer>`, `<section>`, `<article>`, `<aside>`, `<header>`, `<main>`)
/// is a block. The white space that a link label starts with is no space at the start of that
/// block, whatever lies between the element and the link.
#[test]
fn white_space_at_the_start_of_a_link_label_is_no_space_at_the_start_of_a_sectioning_element() {
    assert_on_both(&[
        ("x<footer><a href=\"/x\">\nLogo</a></footer>", "x\n\n[Logo](/x)\n"),
        (
            "x<footer><a href=\"/x\"> Logo</a> now</footer>",
            "x\n\n[Logo](/x) now\n",
        ),
        ("x<footer>\n<a href=\"/x\">\nLogo</a>\n</footer>", "x\n\n[Logo](/x)\n"),
        (
            "x<footer><div><a href=\"/x\">\nLogo</a></div></footer>",
            "x\n\n[Logo](/x)\n",
        ),
        (
            "<p>x</p><footer><p><a href=\"/x\">\nLogo</a></p></footer>",
            "x\n\n[Logo](/x)\n",
        ),
        (
            "x<section><div><a href=\"/x\">\nLogo</a></div></section>",
            "x\n\n[Logo](/x)\n",
        ),
        (
            "x<article><div><a href=\"/x\">\nLogo</a></div></article>",
            "x\n\n[Logo](/x)\n",
        ),
        (
            "x<aside><div><a href=\"/x\">\nLogo</a></div></aside>",
            "x\n\n[Logo](/x)\n",
        ),
        (
            "x<header><div><a href=\"/x\">\nLogo</a></div></header>",
            "x\n\n[Logo](/x)\n",
        ),
        (
            "x<main><article><section><div><a href=\"/x\">\nLogo</a></div></section></article></main>",
            "x\n\n[Logo](/x)\n",
        ),
        (
            "<p>x</p><footer><div>\n<a href=\"/\">\n<img src=\"/l.png\" alt=\"Logo\">\n</a>\n</div></footer>",
            "x\n\n[![Logo](/l.png)](/)\n",
        ),
        (
            "x<footer><div><a href=\"https://e.org/\"> https://e.org/</a></div></footer>",
            "x\n\n<https://e.org/>\n",
        ),
        (
            "x<footer><span><a href=\"/x\">\nLogo</a></span></footer>",
            "x\n\n[Logo](/x)\n",
        ),
        (
            "x<footer><div><b><a href=\"/x\">\nLogo</a></b></div></footer>",
            "x\n\n**[Logo](/x)**\n",
        ),
        (
            "x<footer><a href=\"/x\"><b> Logo</b></a></footer>",
            "x\n\n[**Logo**](/x)\n",
        ),
        (
            "x<footer><a href=\"/x\">\nLogo</a></footer><footer><a href=\"/y\">\nTwo</a></footer>",
            "x\n\n[Logo](/x)\n\n[Two](/y)\n",
        ),
        (
            "<div>x<footer><a href=\"/x\">\nLogo</a></footer></div>",
            "x\n\n[Logo](/x)\n",
        ),
        (
            "<blockquote>x<footer><a href=\"/x\">\nLogo</a></footer></blockquote>",
            "> x\n>\n> [Logo](/x)\n",
        ),
    ]);
    assert_on_full(&[
        (
            "<ul><li>x<footer><a href=\"/x\">\nLogo</a></footer></li></ul>",
            "- x\n\n  [Logo](/x)\n",
        ),
        (
            "<ul><li><footer><a href=\"/x\">\nLogo</a></footer></li></ul>",
            "- [Logo](/x)\n",
        ),
        ("x<footer><div><q> Logo</q></div></footer>", "x\n\n\"Logo\"\n"),
    ]);
}

/// The space is owed where the link is not the start of the sectioning element, and the white
/// space at the end of a label is no space at the end of the element.
#[test]
fn a_link_label_in_a_sectioning_element_keeps_the_space_between_two_words() {
    assert_on_both(&[
        (
            "x<footer><div>one<a href=\"/x\">\nLogo</a></div></footer>",
            "x\n\none [Logo](/x)\n",
        ),
        (
            "x<section><a href=\"/x\">one </a><a href=\"/y\"> two </a></section><p>y</p>",
            "x\n\n[one](/x) [two](/y)\n\ny\n",
        ),
        (
            "x<footer><a href=\"/x\">Logo </a></footer><p>y</p>",
            "x\n\n[Logo](/x)\n\ny\n",
        ),
        (
            "x<footer><a href=\"/x\">Logo </a><div>y</div></footer>",
            "x\n\n[Logo](/x)\n\ny\n",
        ),
    ]);
}

/// The other content that a sectioning element can start with follows the same rule: text, and
/// an inline element that writes marks.
#[test]
fn a_sectioning_element_does_not_start_with_a_space() {
    assert_on_both(&[
        ("x<footer> Logo</footer>", "x\n\nLogo\n"),
        ("x<footer><b> Logo</b></footer>", "x\n\n**Logo**\n"),
        ("x<footer> <b>Logo</b></footer>", "x\n\n**Logo**\n"),
        ("x<footer><div><i> Logo</i></div></footer>", "x\n\n*Logo*\n"),
        ("x<footer><div><a> Logo</a></div></footer>", "x\n\nLogo\n"),
    ]);
}

/// Only the one space of collapsed white space is removed at the start of a sectioning
/// element: the indent of a code block stays, and so does white space in the strict mode.
#[test]
fn a_sectioning_element_keeps_an_indent_and_strict_white_space() {
    let indented = convert_with(
        "x<footer><pre><code>a</code></pre></footer>",
        Some(ConversionOptions {
            code_block_style: CodeBlockStyle::Indented,
            tier_strategy: TierStrategy::Tier2,
            ..tier_options()
        }),
    );
    assert_eq!(indented, "x\n\n    a\n");
    let strict = convert_with(
        "x<footer> Logo</footer>",
        Some(ConversionOptions {
            whitespace_mode: WhitespaceMode::Strict,
            tier_strategy: TierStrategy::Tier2,
            ..tier_options()
        }),
    );
    assert_eq!(strict, "x\n\n Logo\n");
}

/// The strict white space mode keeps the white space of the source. No rule of running text
/// applies there: each row is the output that the strict mode had before those rules. The strict
/// mode always runs on the full converter.
#[test]
fn the_strict_mode_writes_what_it_wrote_before_the_rules_of_running_text() {
    let cases = [
        // ~keep The white space at an end of a link label writes no space outside the link.
        ("<p>Press <a href=\"/p\">Go </a> now</p>", "Press [Go](/p) now\n"),
        ("<p><a href=\"/p\"> Go </a>now</p>", "[Go](/p)now\n"),
        // ~keep A marked element writes the white space it starts with, also after a space.
        ("<p>one <b> y</b>two</p>", "one  **y**two\n"),
        // ~keep A short quotation writes nothing outside its marks.
        ("<p>Press <q> Go </q> now</p>", "Press \"Go\" now\n"),
        ("<p>Press<q>Go<br></q>now</p>", "Press\"Go\"now\n"),
        // ~keep A block boundary in a heading writes no space of its own.
        ("<h2>x<footer> Logo</footer></h2>", "## x Logo\n"),
        // ~keep Two children of a link label that touch get one space when a block is among them.
        (
            "<a href=\"/x\"><b>H</b>ello<div>x</div></a>",
            "[**H** ello x](/x)\n",
        ),
    ];
    let wrong: Vec<String> = cases
        .iter()
        .flat_map(|(html, expected)| {
            [TierStrategy::Tier2, TierStrategy::default()].map(|tier_strategy| {
                let actual = convert_with(
                    html,
                    Some(ConversionOptions {
                        whitespace_mode: WhitespaceMode::Strict,
                        tier_strategy,
                        ..tier_options()
                    }),
                );
                (actual != *expected).then(|| format!("{html:?}\n  expected {expected:?}\n  actual   {actual:?}"))
            })
        })
        .flatten()
        .collect();
    assert!(wrong.is_empty(), "{} wrong:\n{}", wrong.len(), wrong.join("\n"));
}

/// A short quotation is written without the white space at its two ends too, and that white
/// space is one space outside the quotation marks. The fast converter does not write these marks.
#[test]
fn white_space_at_an_end_of_a_short_quotation_is_one_space_outside_the_marks() {
    assert_on_full(&[
        ("<p>Press <q>Go </q>now.</p>", "Press \"Go\" now.\n"),
        ("<p>Press<q> Go</q> now.</p>", "Press \"Go\" now.\n"),
        ("<p>Press<q> Go </q>now.</p>", "Press \"Go\" now.\n"),
        ("<p>Press <q>Go </q> now.</p>", "Press \"Go\" now.\n"),
        ("<p>Press <q> Go</q>now.</p>", "Press \"Go\"now.\n"),
        ("<p>Press<q>Go</q>now.</p>", "Press\"Go\"now.\n"),
        ("<p>Press <q>Go </q></p>", "Press \"Go\"\n"),
        ("<ul><li>Press <q>Go </q>now.</li></ul>", "- Press \"Go\" now.\n"),
        ("<p>Press <b><q>Go </q></b>now.</p>", "Press **\"Go\"** now.\n"),
    ]);
}

/// A line break at the end of a short quotation is kept: a browser shows `now.` on a new line.
#[test]
fn a_line_break_at_the_end_of_a_short_quotation_is_kept() {
    assert_on_full(&[("<p>Press <q>Go<br></q>now.</p>", "Press \"Go\"  \nnow.\n")]);
}

/// A browser writes the closing mark of a short quotation after the line end, so the line end is
/// a space before a zero-width space: it shows `one`, a space, then the zero-width space.
#[test]
fn the_space_after_a_short_quotation_is_kept_before_a_zero_width_space() {
    assert_on_full(&[("<p><q>one\n</q>\u{200b}two</p>", "\"one\" \u{200b}two\n")]);
}

/// A no-break space beside a space in a link label is one space, on both converters.
#[test]
fn a_no_break_space_beside_a_space_in_a_link_label_is_one_space() {
    assert_on_both(&[
        (
            "<p><a href=\"/x\">one<span>&nbsp;</span> two</a></p>",
            "[one two](/x)\n",
        ),
        ("<p><a href=\"/x\"><span>&nbsp;</span> one</a></p>", "[one](/x)\n"),
        ("<p><a href=\"/x\">one&nbsp;two</a></p>", "[one two](/x)\n"),
    ]);
}

#[test]
fn an_empty_inline_element_of_any_kind_keeps_the_space_before_it() {
    assert_on_both(&[
        ("<p>one\n<b></b>two</p>", "one two\n"),
        ("<p>one\n<i></i>two</p>", "one two\n"),
        ("<p>one\n<code></code>two</p>", "one two\n"),
        ("<p>one\n<a></a>two</p>", "one two\n"),
        ("<p>one\n<a id=\"target\"></a>two</p>", "one two\n"),
        ("<p>one\n<i class=\"fa fa-x\"></i>two</p>", "one two\n"),
        ("<p>one <span id=\"a\"></span>two</p>", "one two\n"),
        ("<p>one\t<span id=\"a\"></span>two</p>", "one two\n"),
        ("<p>one<span id=\"a\"></span>\ntwo</p>", "one two\n"),
    ]);
}

/// White space on the two sides of an element that writes nothing is one run of white space.
#[test]
fn white_space_on_both_sides_of_an_empty_inline_element_is_one_space() {
    assert_on_both(&[
        ("<p>one\n<span id=\"a\"></span>\ntwo</p>", "one two\n"),
        ("<p>one <span id=\"a\"></span> two</p>", "one two\n"),
        ("<p>one\t<a id=\"t\"></a>\ttwo</p>", "one two\n"),
        (
            "<p>one\n<span id=\"a\"></span>\n<span id=\"b\"></span>\ntwo</p>",
            "one two\n",
        ),
        ("<p>one <b></b> two</p>", "one two\n"),
        ("<p>one <span> </span> two</p>", "one two\n"),
        ("<p><em>one\n<span id=\"a\"></span>\ntwo</em></p>", "*one two*\n"),
        ("<h2>one\n<span id=\"a\"></span>\ntwo</h2>", "## one two\n"),
        ("<ul><li>one <span id=\"a\"></span> two</li></ul>", "- one two\n"),
        ("<div>one <span id=\"a\"></span> two</div>", "one two\n"),
        ("<blockquote>one <span id=\"a\"></span> two</blockquote>", "> one two\n"),
        (
            "<p><a href=\"/x\">one <span id=\"a\"></span> two</a></p>",
            "[one two](/x)\n",
        ),
        ("<p><span>one </span> two</p>", "one two\n"),
        (
            "<table><tr><th>h</th></tr><tr><td>one <span id=\"a\"></span> two</td></tr></table>",
            "| h       |\n| ------- |\n| one two |\n",
        ),
        (
            "<table><tr><th>h</th></tr><tr><td>one\n<span id=\"a\"></span>\ntwo</td></tr></table>",
            "| h       |\n| ------- |\n| one two |\n",
        ),
    ]);
}

/// A no-break space in a node of its own is kept as it is, also beside a space. In running text
/// the default white space mode writes it as one space, at the edge of the text as in the middle.
#[test]
fn a_no_break_space_does_not_collapse() {
    assert_on_both(&[
        ("<p>one\n<span>&nbsp;</span>two</p>", "one \u{a0}two\n"),
        ("<p>one <span>&nbsp;</span> two</p>", "one \u{a0} two\n"),
        ("<p>one <b>x</b> &nbsp;<i>two</i></p>", "one **x** \u{a0}*two*\n"),
        ("<p><b>one</b>&nbsp;<i>two</i></p>", "**one**\u{a0}*two*\n"),
        ("<p><b>one</b>&nbsp;&nbsp;<i>two</i></p>", "**one**\u{a0}\u{a0}*two*\n"),
        ("<p>one&nbsp;two</p>", "one two\n"),
        ("<p>one&nbsp;<b>two</b></p>", "one **two**\n"),
        ("<p><b>one</b>&nbsp;two</p>", "**one** two\n"),
        ("<p>one&#160;<b>two</b>&#xa0;three</p>", "one **two** three\n"),
        ("<p>one<span id=\"a\"></span>&nbsp;two</p>", "one two\n"),
        ("<p>one&nbsp;<span id=\"a\"></span>two</p>", "one two\n"),
        ("<p><img src=\"/i.png\" alt=\"i\">&nbsp;text</p>", "![i](/i.png) text\n"),
        ("<p>see&nbsp;<a href=\"/x\">two</a>&nbsp;now</p>", "see [two](/x) now\n"),
        ("<ul><li>one&nbsp;<b>two</b></li></ul>", "- one **two**\n"),
        ("<h2>one&nbsp;<b>two</b></h2>", "## one **two**\n"),
        ("<pre>one&nbsp;two</pre>", "```\none\u{a0}two\n```\n"),
    ]);
}

#[test]
fn no_white_space_in_the_source_is_no_space() {
    assert_on_both(&[
        ("<p>one<span id=\"a\"></span>two</p>", "onetwo\n"),
        ("<p>one<b></b>two</p>", "onetwo\n"),
        ("<p><b>H</b>ello</p>", "**H**ello\n"),
        ("<p>foo<span>bar</span>baz</p>", "foobarbaz\n"),
        ("<p><a href=\"/x\">link</a>.</p>", "[link](/x).\n"),
        ("<p>see <em>this</em>, then</p>", "see *this*, then\n"),
        ("<p>one\n<span id=\"a\"></span>, two</p>", "one , two\n"),
        ("<p><img src=\"/i.png\" alt=\"alt\">text</p>", "![alt](/i.png)text\n"),
        ("<p>one<img src=\"/i.png\" alt=\"i\">two</p>", "one![i](/i.png)two\n"),
        ("<p>see <img src=\"/i.png\" alt=\"i\">.</p>", "see ![i](/i.png).\n"),
    ]);
}

#[test]
fn an_empty_inline_element_is_not_the_start_or_the_end_of_a_line() {
    assert_on_both(&[
        ("<p><span id=\"a\"></span> one two</p>", "one two\n"),
        ("<p>\n<span id=\"a\"></span>\none two</p>", "one two\n"),
        ("<p>one two\n<span id=\"a\"></span></p>", "one two\n"),
        (
            "<p>one two\n<span id=\"a\"></span></p><p>three</p>",
            "one two\n\nthree\n",
        ),
        ("<p>one\n<span id=\"a\"></span><br>two</p>", "one  \ntwo\n"),
        ("<p>one\n<br>two</p>", "one  \ntwo\n"),
    ]);
}

#[test]
fn the_space_after_an_image_is_kept() {
    assert_on_both(&[
        ("<p><img src=\"/i.png\" alt=\"alt\"> text</p>", "![alt](/i.png) text\n"),
        ("<p><img src=\"/i.png\" alt=\"alt\">\ntext</p>", "![alt](/i.png) text\n"),
        (
            "<p><a href=\"/x\"><img src=\"/i.png\" alt=\"alt\"> text</a></p>",
            "[![alt](/i.png) text](/x)\n",
        ),
        (
            "<p><a href=\"/x\">text <img src=\"/i.png\" alt=\"alt\"></a></p>",
            "[text ![alt](/i.png)](/x)\n",
        ),
        (
            "<p><a href=\"/x\"><img src=\"/i.png\" alt=\"i\"></a> text</p>",
            "[![i](/i.png)](/x) text\n",
        ),
        ("<img src=\"/i.png\" alt=\"alt\"> text", "![alt](/i.png) text\n"),
        (
            "<div><img src=\"/i.png\" alt=\"alt\"> text</div>",
            "![alt](/i.png) text\n",
        ),
        (
            "<blockquote><img src=\"/i.png\" alt=\"i\"> text</blockquote>",
            "> ![i](/i.png) text\n",
        ),
        (
            "<p><b><img src=\"/i.png\" alt=\"i\"> text</b></p>",
            "**![i](/i.png) text**\n",
        ),
        (
            "<p><img src=\"/i.png\" alt=\"i\"> <b>bold</b></p>",
            "![i](/i.png) **bold**\n",
        ),
        (
            "<p><img src=\"/a.png\" alt=\"a\"> <img src=\"/b.png\" alt=\"b\"> text</p>",
            "![a](/a.png) ![b](/b.png) text\n",
        ),
        (
            "<p>first</p><p><img src=\"/i.png\" alt=\"alt\"> text</p>",
            "first\n\n![alt](/i.png) text\n",
        ),
        (
            "<p>one <img src=\"/i.png\" alt=\"i\"> two</p>",
            "one ![i](/i.png) two\n",
        ),
        (
            "<p>one\n<img src=\"/i.png\" alt=\"i\">\ntwo</p>",
            "one ![i](/i.png) two\n",
        ),
        ("<p>one <img src=\"/i.png\" alt=\"i\"> </p>", "one ![i](/i.png)\n"),
        ("<p> <img src=\"/i.png\" alt=\"i\"> text</p>", "![i](/i.png) text\n"),
        (
            "<ul><li><img src=\"/i.png\" alt=\"i\"> text</li></ul>",
            "- ![i](/i.png) text\n",
        ),
        ("<h2><img src=\"/i.png\" alt=\"i\"> text</h2>", "## i text\n"),
        (
            "<p><img src=\"/i.png\" alt=\"i\"><br> text</p>",
            "![i](/i.png)  \ntext\n",
        ),
    ]);
}

#[test]
fn the_space_after_an_inline_graphic_is_kept() {
    const GRAPHIC: &str = "<svg viewBox=\"0 0 1 1\"><title>Logo</title><path d=\"M0 0\"/></svg>";
    for (html, start, end) in [
        (
            format!("<p>{GRAPHIC} text</p>"),
            "![Logo](data:image/svg+xml;base64,",
            ") text\n",
        ),
        (
            format!("<p>{GRAPHIC}text</p>"),
            "![Logo](data:image/svg+xml;base64,",
            ")text\n",
        ),
        (
            format!("<a href=\"/x\">{GRAPHIC} text</a>"),
            "[![Logo](data:image/svg+xml;base64,",
            ") text](/x)\n",
        ),
        (
            format!("<p>one {GRAPHIC} two</p>"),
            "one ![Logo](data:image/svg+xml;base64,",
            ") two\n",
        ),
        (
            format!("<p>one {GRAPHIC}</p>"),
            "one ![Logo](data:image/svg+xml;base64,",
            ")\n",
        ),
    ] {
        let actual = convert_everywhere(&html, false).unwrap_or_else(|reason| panic!("{html:?}: {reason}"));
        assert!(
            actual.starts_with(start) && actual.ends_with(end),
            "{html:?} gives {actual:?}"
        );
    }
}

#[test]
fn the_space_after_other_replaced_content_is_kept() {
    assert_on_full(&[
        ("<p><video src=\"/v.mp4\"></video> text</p>", "[/v.mp4](/v.mp4) text\n"),
        ("<p><input type=\"checkbox\"> text</p>", "[ ] text\n"),
    ]);
}

#[test]
fn white_space_of_any_kind_is_one_space() {
    assert_on_both(&[
        ("<p>one\n<b>two</b>\nthree</p>", "one **two** three\n"),
        ("<p>one <b>two</b> three</p>", "one **two** three\n"),
        ("<p>one\t<b>two</b>\tthree</p>", "one **two** three\n"),
        ("<p>one \n\t <b>two</b> \n\t three</p>", "one **two** three\n"),
        ("<p>one\r\n<b>two</b>\r\nthree</p>", "one **two** three\n"),
        ("<p>one<span> </span>two</p>", "one two\n"),
        ("<p>one<span>\n</span>two</p>", "one two\n"),
        ("<p>one<span>\t</span>two</p>", "one two\n"),
        ("<p>one<span><span>\n</span></span>two</p>", "one two\n"),
        ("<p><span> </span>one</p>", "one\n"),
        ("<p>one<span> </span></p>", "one\n"),
        ("<p>one<span>&nbsp;</span>two</p>", "one\u{a0}two\n"),
    ]);
}

#[test]
fn nothing_collapses_in_preformatted_text() {
    assert_on_both(&[(
        "<pre>one\n<span id=\"a\"></span>two  three\n\tfour</pre>",
        "```\none\ntwo  three\n\tfour\n```\n",
    )]);
}

#[test]
fn blocks_inside_a_link_are_separate_words_at_every_depth() {
    assert_on_full(&[
        (
            "<a href=\"/x\"><div>Next</div><div>Cross-references</div></a>",
            "[Next Cross-references](/x)\n",
        ),
        (
            "<a href=\"/x\"><div><div>Next</div><div>Cross-references</div></div></a>",
            "[Next Cross-references](/x)\n",
        ),
        (
            "<a href=\"/x\"><div><div><div>Next</div><div>Cross-references</div></div></div></a>",
            "[Next Cross-references](/x)\n",
        ),
        (
            "<a href=\"/x\"><div><div><span>Next</span></div><div>Cross-references</div></div></a>",
            "[Next Cross-references](/x)\n",
        ),
        (
            "<a href=\"/x\">\n  <div>\n    <div>Next</div>\n    <div>Cross-references</div>\n  </div>\n</a>",
            "[Next Cross-references](/x)\n",
        ),
        (
            "<a href=\"/x\"><div><p>Next</p><p>Cross-references</p></div></a>",
            "[Next Cross-references](/x)\n",
        ),
        (
            "<a href=\"/x\"><div>Next<div>Cross-references</div></div></a>",
            "[Next Cross-references](/x)\n",
        ),
        (
            "<a href=\"/x\"><div><div>Next</div>Cross-references</div></a>",
            "[Next Cross-references](/x)\n",
        ),
        (
            "<a href=\"/x\"><div><span>Next</span><div>Cross-references</div></div></a>",
            "[Next Cross-references](/x)\n",
        ),
        (
            "<a href=\"/x\"><div><div>a</div><div>b</div><div>c</div></div></a>",
            "[a b c](/x)\n",
        ),
        (
            "<a href=\"/x\">Next<b><div>Cross</div></b></a>",
            "[Next **Cross**](/x)\n",
        ),
        (
            "<a href=\"/x\"><b><div>Next</div></b>Cross</a>",
            "[**Next** Cross](/x)\n",
        ),
        (
            "<a href=\"/x\"><div><div></div><div>Next</div><div></div><div>Cross</div></div></a>",
            "[Next Cross](/x)\n",
        ),
        (
            "<p>See <a href=\"/x\"><div><div>Next</div><div>Cross-references</div></div></a> now</p>",
            "See [Next Cross-references](/x) now\n",
        ),
        (
            "<ul><li><a href=\"/x\"><div><div>Next</div><div>Cross</div></div></a></li></ul>",
            "- [Next Cross](/x)\n",
        ),
        (
            "<a class=\"right-next\" href=\"cross.html\" title=\"next page\">\n<div class=\"prev-next-info\">\n<p class=\"prev-next-subtitle\">next</p>\n<p class=\"prev-next-title\">Cross-references</p>\n</div>\n<i class=\"fa-solid fa-angle-right\"></i>\n</a>",
            "[next Cross-references](cross.html \"next page\")\n",
        ),
    ]);
}

#[test]
fn inline_content_beside_a_block_in_a_link_gets_no_space_of_its_own() {
    assert_on_full(&[
        ("<a href=\"/x\"><b>H</b>ello<div>x</div></a>", "[**H**ello x](/x)\n"),
        (
            "<a href=\"/x\"><span>a</span><span>b</span><div>c</div></a>",
            "[ab c](/x)\n",
        ),
        (
            "<a href=\"/x\"><span>a</span> <span>b</span><div>c</div></a>",
            "[a b c](/x)\n",
        ),
        (
            "<a href=\"/x\">Next<span>Cross<div>x</div></span></a>",
            "[NextCross x](/x)\n",
        ),
        (
            "<a href=\"/x\">Next<span><!-- c -->Cross<div>x</div></span></a>",
            "[NextCross x](/x)\n",
        ),
        (
            "<a href=\"/x\">Next<span>\n<div>Cross</div></span></a>",
            "[Next Cross](/x)\n",
        ),
        ("<a href=\"/x\"><div>x</div>text</a>", "[x text](/x)\n"),
        ("<a href=\"/x\">text<div>x</div></a>", "[text x](/x)\n"),
    ]);
}

/// A label that holds a block is written without Markdown images: an image is its alternative
/// text there, and a reader sees the image and the text beside it as two things.
#[test]
fn the_text_of_an_image_beside_a_block_in_a_link_is_a_word_of_its_own() {
    assert_on_full(&[
        (
            "<a href=\"/x\"><img src=\"/i.png\" alt=\"Logo\"><span>Docs</span><p>desc</p></a>",
            "[Logo Docs desc](/x)\n",
        ),
        (
            "<a href=\"/x\"><span>Docs</span><img src=\"/i.png\" alt=\"Logo\"><p>desc</p></a>",
            "[Docs Logo desc](/x)\n",
        ),
        (
            "<a href=\"/x\"><img src=\"/i.png\" alt=\"Logo\">Docs<div>desc</div></a>",
            "[Logo Docs desc](/x)\n",
        ),
        (
            "<a href=\"/x\"><img src=\"/a.png\" alt=\"A\"><img src=\"/b.png\" alt=\"B\"><div>desc</div></a>",
            "[A B desc](/x)\n",
        ),
        (
            "<a href=\"/x\"><img src=\"/i.png\" alt=\"Logo\"><b>Docs</b><p>desc</p></a>",
            "[Logo **Docs** desc](/x)\n",
        ),
        (
            "<a href=\"/x\"><img src=\"/i.png\" alt=\"Logo\"> <span>Docs</span><p>desc</p></a>",
            "[Logo Docs desc](/x)\n",
        ),
        (
            "<a href=\"/x\"><img src=\"/i.png\" alt=\"\"><span>Docs</span><p>desc</p></a>",
            "[Docs desc](/x)\n",
        ),
        (
            "<a href=\"/x\"><span>Docs</span><img src=\"/i.png\" alt=\"\"><span>New</span><p>desc</p></a>",
            "[DocsNew desc](/x)\n",
        ),
        (
            "<a href=\"/x\"><img src=\"/i.png\" alt=\"Logo\"><span id=\"a\"></span><span>Docs</span><p>desc</p></a>",
            "[Logo Docs desc](/x)\n",
        ),
        (
            "<a href=\"/x\"><svg viewBox=\"0 0 1 1\"><title>Logo</title><path d=\"M0 0\"/></svg><span>Docs</span><p>desc</p></a>",
            "[Logo Docs desc](/x)\n",
        ),
        (
            "<a href=\"/x\"><picture><source srcset=\"/a.webp\"><img src=\"/i.png\" alt=\"Logo\"></picture><span>Docs</span><p>desc</p></a>",
            "[Logo Docs desc](/x)\n",
        ),
        (
            "<a href=\"/x\"><img src=\"/i.png\" alt=\"Logo\">.<p>desc</p></a>",
            "[Logo. desc](/x)\n",
        ),
        (
            "<a href=\"/x\">(<img src=\"/i.png\" alt=\"Logo\">)<p>desc</p></a>",
            "[(Logo) desc](/x)\n",
        ),
        (
            "<a href=\"/x\"><span>Docs</span><span>New</span><p>desc</p></a>",
            "[DocsNew desc](/x)\n",
        ),
    ]);
}

#[test]
fn blocks_inside_a_heading_or_a_cell_are_separate_words() {
    assert_on_both(&[
        ("<h2><p>a</p><p>b</p></h2>", "## a b\n"),
        ("<h2><div><div>a</div><div>b</div></div></h2>", "## a b\n"),
        (
            "<table><tr><th>h</th></tr><tr><td><div><div>a</div><div>b</div></div></td></tr></table>",
            "| h   |\n| --- |\n| a b |\n",
        ),
    ]);
}
