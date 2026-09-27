//! Link destination and title helpers shared by the link, image and media handlers.
//!
//! The `<a>` handler itself is `converter::handlers::link`.

mod link_url;
pub use link_url::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::ConversionOptions;
    use crate::options::validation::UrlEscapeStyle;
    use std::borrow::Cow;

    fn opts_with_style(style: UrlEscapeStyle) -> ConversionOptions {
        ConversionOptions::builder().url_escape_style(style).build()
    }

    #[test]
    fn has_uri_scheme_accepts_http() {
        assert!(has_uri_scheme("http://example.com"));
        assert!(has_uri_scheme("https://example.com/path"));
    }

    #[test]
    fn has_uri_scheme_accepts_mailto() {
        assert!(has_uri_scheme("mailto:a@b.com"));
    }

    #[test]
    fn has_uri_scheme_accepts_uncommon_schemes() {
        assert!(has_uri_scheme("ftp://host"));
        assert!(has_uri_scheme("ssh://host"));
        assert!(has_uri_scheme("data:text/plain,foo"));
        assert!(has_uri_scheme("file:///etc/hosts"));
    }

    #[test]
    fn has_uri_scheme_rejects_bare_paths() {
        assert!(!has_uri_scheme("foobar.png"));
        assert!(!has_uri_scheme("/relative/path"));
        assert!(!has_uri_scheme("../up.html"));
        assert!(!has_uri_scheme("#fragment"));
    }

    #[test]
    fn has_uri_scheme_rejects_leading_digit_or_punct() {
        assert!(!has_uri_scheme("9scheme:foo"));
        assert!(!has_uri_scheme(":no-scheme"));
        assert!(!has_uri_scheme(""));
    }

    #[test]
    fn issue_397_filename_with_extension_is_not_autolinked() {
        assert!(!has_uri_scheme("foobar.png"));
    }

    #[test]
    fn percent_encode_url_leaves_unreserved_chars_unchanged() {
        let result = percent_encode_url("/path-to_file.html~");
        assert_eq!(result, "/path-to_file.html~");
    }

    #[test]
    fn percent_encode_url_encodes_spaces() {
        assert_eq!(percent_encode_url("/file (1).pdf"), "/file%20%281%29.pdf");
    }

    #[test]
    fn percent_encode_url_encodes_angle_brackets() {
        assert_eq!(percent_encode_url("/file <draft>.pdf"), "/file%20%3Cdraft%3E.pdf");
    }

    #[test]
    fn percent_encode_url_full_issue_example() {
        assert_eq!(
            percent_encode_url("/file (1) <draft>.pdf"),
            "/file%20%281%29%20%3Cdraft%3E.pdf"
        );
    }

    #[test]
    fn percent_encode_non_ascii_leaves_ascii_only_input_unchanged() {
        assert_eq!(
            percent_encode_non_ascii("/file (1) <draft>.pdf"),
            "/file (1) <draft>.pdf"
        );
    }

    #[test]
    fn percent_encode_non_ascii_encodes_only_non_ascii_bytes() {
        assert_eq!(percent_encode_non_ascii("/f\u{f6}\u{f6}.html"), "/f%C3%B6%C3%B6.html");
    }

    #[test]
    fn append_markdown_link_angle_plain_url_unchanged() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "text",
                href: "/file.pdf",
                title: None,
                raw_text: "text",
            },
            &options,
            None,
        );
        assert_eq!(out, "[text](/file.pdf)");
    }

    // ~keep Regression for the CommonMark spec fixpoint gap: a raw non-ASCII byte in a
    // ~keep destination is never valid URI syntax (RFC 3986), and every compliant HTML
    // ~keep renderer percent-encodes it on render (confirmed against the spec's own worked
    // ~keep example for entity references, `/f\u{f6}\u{f6}` -> `/f%C3%B6%C3%B6`), so the
    // ~keep default `Angle` style must emit it pre-encoded to reach a round-trip fixpoint.
    #[test]
    fn append_markdown_link_angle_percent_encodes_non_ascii() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "öö.html",
                href: "öö.html",
                title: None,
                raw_text: "öö.html",
            },
            &options,
            None,
        );
        assert_eq!(out, "[öö.html](%C3%B6%C3%B6.html)");
    }

    // ~keep Non-ASCII pre-encoding must not disturb ASCII reserved characters the `Angle`
    // ~keep style deliberately leaves alone (unlike `Percent`, which would also mangle this
    // ~keep query string): only bytes >= 0x80 are touched.
    #[test]
    fn append_markdown_link_angle_percent_encodes_non_ascii_but_not_ascii_reserved_chars() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "text",
                href: "https://example.com/söq?a=1&b=2",
                title: None,
                raw_text: "text",
            },
            &options,
            None,
        );
        assert_eq!(out, "[text](https://example.com/s%C3%B6q?a=1&b=2)");
    }

    // ~keep Regression: a same-document fragment must byte-match the target element's
    // ~keep `id`, which this converter does not control, and real-world generators
    // ~keep commonly leave non-ASCII characters raw there (verified against a
    // ~keep GitHub-rendered fixture, `gh-143-links-wordwrap.html`'s
    // ~keep `#walk-up-\u{fe0e}-and-walk-down-\u{fe0e}`) -- pre-encoding only the href while
    // ~keep the id it must match stays raw would break the anchor instead of normalizing it.
    #[test]
    fn append_markdown_link_angle_does_not_percent_encode_a_fragment_destination() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "text",
                href: "#walk-up-\u{fe0e}",
                title: None,
                raw_text: "text",
            },
            &options,
            None,
        );
        assert_eq!(out, "[text](#walk-up-\u{fe0e})");
    }

    #[test]
    fn append_markdown_link_angle_wraps_space_in_angle_brackets() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "file",
                href: "/file (1).pdf",
                title: None,
                raw_text: "file",
            },
            &options,
            None,
        );
        assert_eq!(out, "[file](</file (1).pdf>)");
    }

    #[test]
    fn append_markdown_link_percent_encodes_spaces() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Percent);
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "file",
                href: "/file (1).pdf",
                title: None,
                raw_text: "file",
            },
            &options,
            None,
        );
        assert_eq!(out, "[file](/file%20%281%29.pdf)");
    }

    #[test]
    fn append_markdown_link_percent_encodes_angle_brackets() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Percent);
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "file",
                href: "/file <draft>.pdf",
                title: None,
                raw_text: "file",
            },
            &options,
            None,
        );
        assert_eq!(out, "[file](/file%20%3Cdraft%3E.pdf)");
    }

    #[test]
    fn append_markdown_link_percent_full_issue_example() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Percent);
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "file",
                href: "/file (1) <draft>.pdf",
                title: None,
                raw_text: "file",
            },
            &options,
            None,
        );
        assert_eq!(out, "[file](/file%20%281%29%20%3Cdraft%3E.pdf)");
    }

    #[test]
    fn parens_are_balanced_accepts_nested_parens() {
        assert!(parens_are_balanced("wiki/Rust_(programming_language)"));
        assert!(parens_are_balanced("no/parens/here"));
    }

    #[test]
    fn parens_are_balanced_rejects_equal_counts_out_of_order() {
        // ~keep equal open/close counts are not sufficient for balance: a `)` before its `(`
        // ~keep is the exact naive-count bug this check replaces.
        assert!(!parens_are_balanced("a)b(c"));
    }

    #[test]
    fn parens_are_balanced_rejects_unmatched_open_or_close() {
        assert!(!parens_are_balanced("a(b"));
        assert!(!parens_are_balanced("a)b"));
    }

    #[test]
    fn append_markdown_link_angle_leaves_balanced_parens_unescaped_when_href_has_parens() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "Rust",
                href: "https://en.wikipedia.org/wiki/Rust_(programming_language)",
                title: None,
                raw_text: "Rust",
            },
            &options,
            None,
        );
        assert_eq!(out, "[Rust](https://en.wikipedia.org/wiki/Rust_(programming_language))");
    }

    #[test]
    fn append_markdown_link_angle_escapes_out_of_order_parens_when_href_has_parens() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "link",
                href: "http://example.com/a)(b",
                title: None,
                raw_text: "link",
            },
            &options,
            None,
        );
        assert_eq!(out, "[link](http://example.com/a\\)\\(b)");
    }

    #[test]
    fn append_markdown_link_angle_escapes_gt_inside_wrap_when_href_has_space_and_gt() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "text",
                href: "/my file >.pdf",
                title: None,
                raw_text: "text",
            },
            &options,
            None,
        );
        assert_eq!(out, "[text](</my file \\>.pdf>)");
    }

    #[test]
    fn append_markdown_link_angle_produces_empty_angle_brackets_when_href_is_empty() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "text",
                href: "",
                title: None,
                raw_text: "text",
            },
            &options,
            None,
        );
        assert_eq!(out, "[text](<>)");
    }

    #[test]
    fn append_markdown_link_percent_preserves_title() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Percent);
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "link",
                href: "/path with spaces",
                title: Some("My Title"),
                raw_text: "link",
            },
            &options,
            None,
        );
        assert_eq!(out, "[link](/path%20with%20spaces \"My Title\")");
    }

    // ~keep Issue #498: `decoded_attribute` already decoded `&amp;plus;` to `&plus;` before this
    // ~keep function ever sees it, so these unit tests exercise it with already-decoded input --
    // ~keep exactly what a destination/title looks like by the time it reaches `append_url_destination`
    // ~keep or `escape_markdown_title`.
    #[test]
    fn escape_entity_ampersands_escapes_a_named_reference() {
        assert_eq!(escape_entity_ampersands("?&plus;"), "?&amp;plus;");
    }

    #[test]
    fn escape_entity_ampersands_escapes_a_decimal_numeric_reference() {
        assert_eq!(escape_entity_ampersands("?&#43;"), "?&amp;#43;");
    }

    #[test]
    fn escape_entity_ampersands_escapes_a_hex_numeric_reference() {
        assert_eq!(escape_entity_ampersands("?&#x2B;"), "?&amp;#x2B;");
    }

    #[test]
    fn escape_entity_ampersands_leaves_a_bare_ampersand_unchanged() {
        // ~keep `&b` has no terminating `;`, so it is never a reference candidate -- pins the
        // ~keep existing `?a=1&b=2` behavior (link.rs's `append_markdown_link_angle_...` tests).
        match escape_entity_ampersands("?a&b") {
            Cow::Borrowed(unchanged) => assert_eq!(unchanged, "?a&b"),
            Cow::Owned(owned) => panic!("expected no allocation, got {owned:?}"),
        }
    }

    #[test]
    fn escape_entity_ampersands_leaves_an_unrecognized_named_reference_unchanged() {
        assert_eq!(escape_entity_ampersands("&foo;"), "&foo;");
    }

    #[test]
    fn escape_entity_ampersands_escapes_amp_itself_since_it_is_a_valid_reference() {
        assert_eq!(escape_entity_ampersands("&amp;"), "&amp;amp;");
    }

    #[test]
    fn escape_markdown_title_escapes_backslash_before_quote_so_the_closing_quote_is_not_swallowed() {
        // ~keep audit #24 finding 8: a title ending in a literal `\` must not let a following `\"`
        // (backslash escaping the delimiter's quote) read as an escaped quote instead of the
        // closing delimiter.
        assert_eq!(escape_markdown_title("foo\\"), "foo\\\\");
        assert_eq!(escape_markdown_title("say \"hi\"\\"), "say \\\"hi\\\"\\\\");
    }

    #[test]
    fn append_markdown_link_escapes_a_trailing_backslash_in_title_so_the_closing_quote_is_not_swallowed() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "text",
                href: "/url",
                title: Some("foo\\"),
                raw_text: "text",
            },
            &options,
            None,
        );
        assert_eq!(out, "[text](/url \"foo\\\\\")");
    }

    #[test]
    fn append_markdown_link_escapes_a_trailing_backslash_in_default_title_so_the_closing_quote_is_not_swallowed() {
        let mut out = String::new();
        let mut options = opts_with_style(UrlEscapeStyle::Angle);
        options.default_title = true;
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "text",
                href: "http://a\\",
                title: None,
                raw_text: "http://a\\",
            },
            &options,
            None,
        );
        assert_eq!(out, "[text](http://a\\ \"http://a\\\\\")");
    }

    #[test]
    fn append_markdown_link_escapes_a_backslash_inside_the_angle_bracket_wrap_so_it_cannot_unescape_a_delimiter() {
        // ~keep audit #24 finding 8: inside an angle-bracket-wrapped destination, an unescaped `\`
        // immediately before an escaped `<`/`>` merges with it into a single `\\` escape pair,
        // un-escaping the delimiter and terminating the destination early.
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "text",
                href: "/my file\\>.pdf",
                title: None,
                raw_text: "text",
            },
            &options,
            None,
        );
        assert_eq!(out, "[text](</my file\\\\\\>.pdf>)");
    }

    // ~keep Regression for CommonMark spec fixpoint example 631: a `\` immediately followed by
    // ~keep ASCII punctuation in a *balanced* (non-angle-bracket) destination must be doubled,
    // ~keep or a compliant reparse consumes it as a backslash escape and silently drops the
    // ~keep byte -- `href="\*"` emitted verbatim as `\*` reparses as destination `*`.
    #[test]
    fn append_markdown_link_escapes_a_backslash_before_punctuation_in_a_balanced_destination() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "text",
                href: "\\*",
                title: None,
                raw_text: "text",
            },
            &options,
            None,
        );
        assert_eq!(out, "[text](\\\\*)");
    }

    // ~keep A `\` that is the *last* character of a balanced destination is left unescaped:
    // ~keep this call cannot see what byte `output` gains next (title quote or closing `)`), and
    // ~keep escaping it unconditionally was the audit #24 regression pinned by
    // ~keep `append_markdown_link_escapes_a_trailing_backslash_in_default_title_...` above.
    // ~keep Regression: a trailing `\` in a balanced destination with no title next escapes
    // ~keep the closing `)` on reparse (`\)` is a CommonMark backslash escape) unless doubled.
    // ~keep Left unescaped, `href="x\"` with no title emitted `(x\)` reparses as `\[t\](x)` --
    // ~keep the destination never closes and the whole link degrades to literal bracket text,
    // ~keep which is worse than example 631's single dropped byte. `title_follows` is `false`
    // ~keep here because the very next byte this call emits is the closing `)`.
    #[test]
    fn append_markdown_link_escapes_a_trailing_backslash_in_a_balanced_destination_with_no_title() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "text",
                href: "/path\\",
                title: None,
                raw_text: "text",
            },
            &options,
            None,
        );
        assert_eq!(out, "[text](/path\\\\)");
    }

    // ~keep A trailing `\` in a balanced destination is left unescaped when a title follows:
    // ~keep the next byte this call emits is a literal space, and `\ ` is not a CommonMark
    // ~keep backslash escape (only ASCII punctuation may be escaped), so the `\` is already
    // ~keep safe. Escaping it here would be the audit #24 regression pinned by
    // ~keep `append_markdown_link_escapes_a_trailing_backslash_in_default_title_...` above.
    #[test]
    fn append_markdown_link_leaves_a_trailing_backslash_in_a_balanced_destination_unescaped_when_a_title_follows() {
        let mut out = String::new();
        let options = opts_with_style(UrlEscapeStyle::Angle);
        append_markdown_link(
            &mut out,
            &MarkdownLink {
                label: "text",
                href: "/path\\",
                title: Some("t"),
                raw_text: "text",
            },
            &options,
            None,
        );
        assert_eq!(out, "[text](/path\\ \"t\")");
    }
}
