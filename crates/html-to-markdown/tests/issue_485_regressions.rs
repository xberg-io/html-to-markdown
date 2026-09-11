#![allow(missing_docs)]

use html_to_markdown_rs::{convert, options::ConversionOptions};

fn content(html: &str) -> String {
    convert(html, Some(ConversionOptions::default()))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

#[test]
fn should_map_c1_control_reference_to_windows_1252_replacement_when_preceded_by_a_comment() {
    assert_eq!(content("<!-- comment -->&#155;"), "\u{203A}\n");
}

#[test]
fn should_map_c1_control_reference_to_windows_1252_replacement_without_a_comment() {
    assert_eq!(content("&#155;"), "\u{203A}\n");
}

#[test]
fn should_map_c1_control_reference_to_windows_1252_replacement_in_hex_form() {
    assert_eq!(content("&#x9B;"), "\u{203A}\n");
    assert_eq!(content("&#X9B;"), "\u{203A}\n");
}

#[test]
fn should_map_every_windows_1252_table_entry_to_its_replacement_code_point() {
    let cases: &[(u32, char)] = &[
        (128, '\u{20AC}'), // EURO SIGN
        (130, '\u{201A}'), // SINGLE LOW-9 QUOTATION MARK
        (149, '\u{2022}'), // BULLET
        (159, '\u{0178}'), // LATIN CAPITAL LETTER Y WITH DIAERESIS
    ];
    for &(number, expected) in cases {
        let html = format!("<p>x&#{number};y</p>");
        assert_eq!(content(&html), format!("x{expected}y\n"), "input: {html}");
    }
}

#[test]
fn should_leave_del_control_character_unchanged_when_not_in_the_replacement_table() {
    assert_eq!(content("<p>x&#127;y</p>"), "x\u{007F}y\n");
}

#[test]
fn should_leave_non_breaking_space_unchanged_when_not_in_the_replacement_table() {
    // `<pre>` preserves whitespace verbatim, so the decoded code point survives the
    // later Markdown whitespace-normalization pass unmodified.
    assert_eq!(content("<pre>x&#160;y</pre>"), "```\nx\u{00A0}y\n```\n");
}

#[test]
fn should_replace_null_character_reference_with_replacement_character() {
    assert_eq!(content("<p>x&#0;y</p>"), "x\u{FFFD}y\n");
}

#[test]
fn should_replace_out_of_range_character_reference_with_replacement_character() {
    assert_eq!(content("<p>x&#1114112;y</p>"), "x\u{FFFD}y\n");
    // A digit run far too long to fit any integer type must still saturate to
    // "out of range" rather than silently wrapping around to an in-range value.
    assert_eq!(content("<p>x&#99999999999999999999999;y</p>"), "x\u{FFFD}y\n");
}

#[test]
fn should_replace_surrogate_character_reference_with_replacement_character() {
    assert_eq!(content("<p>x&#xD800;y</p>"), "x\u{FFFD}y\n");
    assert_eq!(content("<p>x&#55296;y</p>"), "x\u{FFFD}y\n");
}
