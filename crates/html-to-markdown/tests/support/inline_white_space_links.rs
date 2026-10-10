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

/// A label of white space only has no ends, so no space goes outside the link. The full
/// converter writes the address as the label, the fast converter an empty label.
///
/// ~keep These rows pin the output of the base, not the output of a browser: a browser shows
/// ~keep the white space of such a link as a space between the two words.
#[test]
fn a_link_label_of_white_space_only_puts_no_space_outside_the_link() {
    let cases = [
        (
            "<p>Press<a href=\"/p\"> </a>now</p>",
            "Press[/p](/p)now\n",
            "Press[](/p)now\n",
        ),
        (
            "<p>Press <a href=\"/p\"> </a>.</p>",
            "Press [/p](/p).\n",
            "Press [](/p).\n",
        ),
        (
            "<p>Press<a href=\"/p\">&nbsp;</a>now</p>",
            "Press[/p](/p)now\n",
            "Press[](/p)now\n",
        ),
    ];
    for (html, full, fast) in cases {
        assert_eq!(tier2(html), full, "full converter: {html:?}");
        assert_eq!(convert_with(html, None), full, "default options: {html:?}");
        let written = tier1::run(html, &PrescanReport::default(), &tier_options());
        assert_eq!(written.as_deref().ok(), Some(fast), "fast converter: {html:?}");
    }
}

/// The white space at the start of a link label is no space at the start of the document, also
/// when the marks of another element stand before the link.
#[test]
fn white_space_at_the_start_of_a_link_label_is_no_space_at_the_start_of_the_document() {
    assert_on_both(&[
        (
            "<p><b><a href=\"/p\"> <img src=\"/i.png\" alt=\"Go\"> </a></b></p>",
            "**[![Go](/i.png)](/p)**\n",
        ),
        (
            "<p><em><a href=\"/p\"> <img src=\"/i.png\" alt=\"Go\"> </a></em></p>",
            "*[![Go](/i.png)](/p)*\n",
        ),
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
        (
            "<h2>Press <a href=\"/p\">Go<br></a> now</h2>",
            "## Press [Go](/p) now\n",
        ),
    ]);
}

/// A code span keeps the white space of its text. The white space at an end of a link label is
/// not written a second time outside the link there. The fast converter gives a link in a code
/// span to the full one.
#[test]
fn white_space_at_an_end_of_a_link_label_is_not_moved_in_a_code_span() {
    assert_on_full(&[
        (
            "<p><code>Press <a href=\"/p\"> Go </a> now</code></p>",
            "`Press [Go](/p) now`\n",
        ),
        (
            "<p><code>Press <a href=\"/p\">Go </a> now</code></p>",
            "`Press [Go](/p) now`\n",
        ),
        // ~keep This row pins the output of the base, not the output of a browser, which shows
        // ~keep `Press Go now`: a label space is not moved in a code span.
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
    ]);
}

/// In `<pre>`, `<code>`, `<kbd>` and `<samp>` no rule of running text runs. Each row pins the
/// output of the full converter before those rules, not the output of a browser. The fast
/// converter gives a link or a block in such an element to the full one.
#[test]
fn the_rules_of_running_text_do_not_run_in_code() {
    assert_on_full(&[
        (
            "<pre>Press<a href=\"/p\">Go </a>now</pre>",
            "```\nPress[Go](/p)now\n```\n",
        ),
        (
            "<pre>Press<a href=\"/p\"> Go </a>now</pre>",
            "```\nPress[Go](/p)now\n```\n",
        ),
        ("<pre>Press<q> Go </q>now</pre>", "```\nPress\"Go\"now\n```\n"),
        (
            "<pre>one\n<a href=\"/y\">t<div>y</div></a></pre>",
            "```\none\n[t y](/y)\n```\n",
        ),
        ("<p><kbd>Press<a name=\"n\">Go </a> now</kbd></p>", "`PressGo now`\n"),
        ("<pre>x<section> a</section></pre>", "```\nx\n\n a\n```\n"),
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
        ("<a href=\"/x\"><b>H</b>ello<div>x</div></a>", "[**H** ello x](/x)\n"),
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

/// In a code span the white space at the ends of a short quotation is not moved out of the marks:
/// the quotation is written as before that rule, as a link in a code span is. A browser shows no
/// space outside the marks that the source does not have: `Press" Go "now` and `Press " Go " now`.
#[test]
fn white_space_at_an_end_of_a_short_quotation_is_not_moved_in_a_code_span() {
    assert_on_full(&[
        ("<p><code><q> Go </q></code></p>", "`\"Go\"`\n"),
        ("<p><code>Press<q> Go </q>now</code></p>", "`Press\"Go\"now`\n"),
        ("<p><code>Press <q> Go </q> now</code></p>", "`Press \"Go\" now`\n"),
        ("<p><code>Press <q> Go </q>.</code></p>", "`Press \"Go\".`\n"),
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
