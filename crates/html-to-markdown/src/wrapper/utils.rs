//! Utility functions for text wrapping.
//!
//! This module contains helper functions for parsing and wrapping Markdown elements.

use crate::converter::utility::escaping::{first_ordered_marker_len, opens_block};

/// Parse a blockquote line into its prefix and content.
///
/// Returns Some((prefix, content)) if the line is a blockquote, None otherwise.
pub fn parse_blockquote_line(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with('>') {
        return None;
    }

    let indent_len = line.len() - trimmed.len();
    let bytes = line.as_bytes();
    let mut i = indent_len;

    while i < bytes.len() {
        if bytes[i] != b'>' {
            break;
        }
        i += 1;
        if i < bytes.len() && bytes[i] == b' ' {
            i += 1;
        }
        while i + 1 < bytes.len() && bytes[i] == b' ' && bytes[i + 1] == b'>' {
            i += 1;
        }
    }

    let prefix = line[..i].to_string();
    let content = line[i..].trim().to_string();
    Some((prefix, content))
}

/// Wrap a blockquote paragraph while preserving its prefix.
///
/// # Arguments
/// - `prefix`: The blockquote prefix (e.g., "> " or "> > ")
/// - `content`: The text content to wrap
/// - `width`: The maximum line width
pub fn wrap_blockquote_paragraph(prefix: &str, content: &str, width: usize) -> String {
    let prefix_len = prefix.len();
    let inner_width = if width > prefix_len { width - prefix_len } else { 1 };

    let wrapped = wrap_line(content, inner_width);
    let mut out = String::new();
    for (idx, part) in wrapped.split('\n').enumerate() {
        if idx > 0 {
            out.push('\n');
        }
        out.push_str(prefix);
        out.push_str(part);
    }
    out
}

/// Check if a line looks like an unordered list item (-, *, or +).
pub fn is_list_like(trimmed: &str) -> bool {
    matches!(trimmed.chars().next(), Some('-' | '*' | '+'))
}

/// Check if a line is a numbered list item.
pub fn is_numbered_list(trimmed: &str) -> bool {
    let token = trimmed.split_whitespace().next().unwrap_or("");
    if token.is_empty() || !(token.ends_with('.') || token.ends_with(')')) {
        return false;
    }

    let digits = token.trim_end_matches(['.', ')']);
    !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit())
}

/// Check if a line is a Markdown heading.
pub fn is_heading(trimmed: &str) -> bool {
    trimmed.starts_with('#')
}

/// Parse a list item into its components: (indent, marker, content)
///
/// Returns Some((indent, marker, content)) if the line is a valid list item,
/// None otherwise.
///
/// Examples:
/// - "- text" -> ("", "- ", "text")
/// - "  - text" -> ("  ", "- ", "text")
/// - "1. text" -> ("", "1. ", "text")
/// - "  42) text" -> ("  ", "42) ", "text")
pub fn parse_list_item(line: &str) -> Option<(String, String, String)> {
    let trimmed = line.trim_ascii_start();
    let indent = &line[..line.len() - trimmed.len()];
    let bytes = trimmed.as_bytes();
    // ~keep Only a space or a tab ends a list marker; a non-breaking space after `1.` is text.
    let marker_len = if let Some(b'-' | b'*' | b'+') = bytes.first() {
        1
    } else {
        let digits = bytes.iter().take_while(|byte| byte.is_ascii_digit()).count();
        if digits == 0 || !matches!(bytes.get(digits), Some(b'.' | b')')) {
            return None;
        }
        digits + 1
    };
    if !matches!(bytes.get(marker_len), None | Some(b' ' | b'\t')) {
        return None;
    }
    Some((
        indent.to_string(),
        format!("{} ", &trimmed[..marker_len]),
        trimmed[marker_len..].trim_ascii_start().to_string(),
    ))
}

/// Check if content is a single inline link (e.g., "[text](#anchor)").
pub fn is_single_inline_link(content: &str) -> bool {
    let trimmed = content.trim();
    if !(trimmed.starts_with('[') && trimmed.ends_with(')')) {
        return false;
    }

    let Some(mid) = trimmed.find("](") else {
        return false;
    };

    let url_part = &trimmed[mid + 2..trimmed.len() - 1];
    if url_part.chars().any(char::is_whitespace) {
        return false;
    }

    !trimmed[mid + 2..].contains("](")
}

/// The hard line break that ends `line`: its trailing spaces, or `""` for a backslash break.
pub fn hard_break(line: &str) -> Option<&str> {
    let text = line.trim_end_matches(' ');
    let spaces = &line[text.len()..];
    if text.trim_matches([' ', '\t']).is_empty() {
        return None;
    }
    if spaces.len() >= 2 {
        return Some(spaces);
    }
    let backslashes = text.len() - text.trim_end_matches('\\').len();
    (spaces.is_empty() && backslashes % 2 == 1).then_some("")
}

/// Append `line` to paragraph `text`: after a space, or on a new line when the text so far ends
/// with a hard line break or when the joined text would open a block.
///
/// ~keep A hard break is a line end the reflow never joins across, so `text` keeps it as a
/// ~keep newline and [`wrap_line`] wraps the text on each side of it on its own (#613). A line
/// ~keep that starts with a bare marker (`*`, `1.`) or a short run of rule characters opens a
/// ~keep block once the next line joins it, so that line end is kept as well (#614).
pub fn push_paragraph_line(text: &mut String, line: &str) {
    let line_text = line.trim_matches([' ', '\t']);
    if !text.is_empty() && !text.ends_with('\n') {
        text.push(if joins_into_a_block(text, line_text) { '\n' } else { ' ' });
    }
    text.push_str(line_text);
    if let Some(spaces) = hard_break(line) {
        text.push_str(spaces);
        text.push('\n');
    }
}

/// Whether the last line of `text`, which opens no block, opens one once `line` joins it.
///
/// ~keep Only the end of `text` is read, so joining many lines stays linear.
pub fn joins_into_a_block(text: &str, line: &str) -> bool {
    let bytes = text.as_bytes();
    (bytes.len().saturating_sub(BARE_MARKER_LEN)..bytes.len())
        .rev()
        .find(|&start| start == 0 || bytes[start - 1] == b'\n')
        .is_some_and(|start| is_bare_marker(&text[start..]) && opens_block(&format!("{} {line}", &text[start..])))
}

/// The longest line [`is_bare_marker`] accepts: an ordered marker of nine digits.
const BARE_MARKER_LEN: usize = 10;

/// The longest line of bullet and rule characters [`is_bare_marker`] accepts.
const BARE_RUN_LEN: usize = 3;

/// Whether `line`, which opens no block, can open one once a word joins its end.
///
/// ~keep Only a bare marker (`*`, `+`, `1.`, `01)`) turns into a list item, and only a run of
/// ~keep fewer than three `*` or `_` into a rule (`**` + `*`); any other line keeps opening no
/// ~keep block.
fn is_bare_marker(line: &str) -> bool {
    (line.len() <= BARE_RUN_LEN
        && line
            .bytes()
            .all(|byte| matches!(byte, b'*' | b'+' | b'-' | b'_' | b'1' | b'.' | b')' | b' ')))
        || first_ordered_marker_len(line) == Some(line.len())
}

/// [`is_bare_marker`] for a line of `words`.
fn words_are_bare_marker(words: &[&str]) -> bool {
    words.len() <= 2
        && words.iter().map(|word| word.len() + 1).sum::<usize>() <= BARE_MARKER_LEN + 1
        && is_bare_marker(&words.join(" "))
}

/// Wrap a single line of text at the specified width.
///
/// This function wraps text without breaking long words or on hyphens,
/// similar to Python's `textwrap.fill()` with `break_long_words=False` and `break_on_hyphens=False`.
/// A newline in `text` is a line end the reflow keeps: the text on each side is wrapped on its
/// own, and the spaces of a hard break before the newline are kept.
pub fn wrap_line(text: &str, width: usize) -> String {
    let text = text.trim_end_matches(['\n', ' ']);
    if text.len() <= width {
        return text.to_string();
    }

    let mut result = String::new();
    for (index, segment) in text.split('\n').enumerate() {
        if index > 0 {
            result.push('\n');
        }
        let words = segment.trim_end_matches(' ');
        wrap_words(words, width, &mut result);
        result.push_str(&segment[words.len()..]);
    }
    result
}

/// The words of `text`, split at spaces and tabs.
///
/// ~keep Only a space or a tab separates words: a non-breaking space is part of its word (#614).
/// ~keep A link destination in angle brackets cannot hold a line end, so it is one word even
/// ~keep when it holds spaces.
fn words(text: &str) -> Vec<&str> {
    let bytes = text.as_bytes();
    let mut words = Vec::new();
    let mut start = None;
    let mut in_destination = false;
    for (index, &byte) in bytes.iter().enumerate() {
        if in_destination {
            in_destination = !(byte == b'>' && bytes[index - 1] != b'\\');
        } else if byte == b'<' && bytes[..index].ends_with(b"](") {
            in_destination = true;
        } else if matches!(byte, b' ' | b'\t') {
            if let Some(word_start) = start.take() {
                words.push(&text[word_start..index]);
            }
            continue;
        }
        start.get_or_insert(index);
    }
    if let Some(word_start) = start {
        words.push(&text[word_start..]);
    }
    words
}

/// Wrap the words of `text`, a text without newlines, at `width` onto `result`.
///
/// ~keep A wrapped line never starts with a word that would open a block there (a list marker,
/// ~keep a `#`, a `>`, a rule); that word moves to the end of the line before it (#614). An
/// ~keep escape would not do: inside a code span the backslash is literal text. A line left
/// ~keep empty is dropped. Only a bare marker or rule characters open a block by taking a word
/// ~keep (`*` + `-` opens a list); such a line that then opens one joins the line before it, so
/// ~keep a run of `*` lines is not moved one word at a time. A first line that opens a block
/// ~keep (`---` of `--- x`) takes words from the lines after it, twice as many each time; `text`
/// ~keep itself opens no block, so that ends. Every step moves a line start right or drops a
/// ~keep line, so the loop ends. A line that takes words can run past `width`.
fn wrap_words(text: &str, width: usize, result: &mut String) {
    let words = words(text);
    let mut greedy_ends = Vec::new();
    let mut line_len = 0;
    for (index, word) in words.iter().enumerate() {
        if line_len > 0 && line_len + 1 + word.len() > width {
            greedy_ends.push(index);
            line_len = 0;
        }
        line_len += usize::from(line_len > 0) + word.len();
    }
    greedy_ends.push(words.len());

    let opens = |start: usize, end: usize| opens_block(&words[start..end].join(" "));
    // ~keep Line `k` is `words[starts[k]..starts[k + 1]]`, the last one ends at `end`.
    let mut starts: Vec<usize> = Vec::new();
    let mut end = 0;
    for greedy_end in greedy_ends {
        if greedy_end <= end {
            continue;
        }
        starts.push(end);
        end = greedy_end;
        let mut index = starts.len() - 1;
        while index < starts.len() {
            let line_end = starts.get(index + 1).copied().unwrap_or(end);
            if !opens(starts[index], line_end) {
                index += 1;
            } else if index == 0 {
                let mut step = 1;
                let mut first_end = line_end;
                while first_end < words.len() && opens(0, first_end) {
                    first_end = (first_end + step).min(words.len());
                    step *= 2;
                    while starts.len() > 1 && starts.get(2).copied().unwrap_or(end) <= first_end {
                        starts.remove(1);
                    }
                    if starts.len() > 1 {
                        starts[1] = starts[1].max(first_end);
                    } else {
                        end = end.max(first_end);
                    }
                }
                index = 1;
            } else {
                let receiver = index - 1;
                let receiver_may_open = words_are_bare_marker(&words[starts[receiver]..starts[index]]);
                starts[index] += 1;
                if starts[index] == line_end {
                    starts.remove(index);
                }
                if receiver_may_open && receiver == 0 {
                    index = 0;
                } else if receiver_may_open && opens(starts[receiver], starts.get(index).copied().unwrap_or(end)) {
                    // ~keep Merged into the line before it, with every bare marker line right
                    // ~keep before it, a line cannot open a block unless it is the first line.
                    let mut merged = receiver;
                    while merged > 0 {
                        let previous_bare = words_are_bare_marker(&words[starts[merged - 1]..starts[merged]]);
                        starts.remove(merged);
                        index -= 1;
                        merged -= 1;
                        if !previous_bare {
                            break;
                        }
                        if merged == 0 {
                            index = 0;
                        }
                    }
                }
            }
        }
    }

    for (index, &start) in starts.iter().enumerate() {
        if index > 0 {
            result.push('\n');
        }
        result.push_str(&words[start..starts.get(index + 1).copied().unwrap_or(end)].join(" "));
    }
}

/// Wrap a paragraph whose first line starts with `indent`, and start every wrapped line with it.
///
/// ~keep An indented paragraph is a list item's continuation paragraph; written at the start of
/// ~keep the line it would leave the item.
pub fn wrap_indented_line(indent: &str, text: &str, width: usize) -> String {
    let wrapped = wrap_line(text, width.saturating_sub(indent.len()).max(1));
    if indent.is_empty() {
        return wrapped;
    }
    let mut result = String::with_capacity(wrapped.len() + indent.len() * 2);
    for (index, line) in wrapped.split('\n').enumerate() {
        if index > 0 {
            result.push('\n');
        }
        result.push_str(indent);
        result.push_str(line);
    }
    result
}

/// Wrap a list item while preserving its structure.
///
/// The first line of output will be: `<indent><marker><content_start>`
/// Continuation lines will be: `<indent><spaces_matching_marker><content_continued>`
///
/// # Arguments
/// - `indent`: The leading whitespace (for nested lists)
/// - `marker`: The list marker (e.g., "- ", "1. ")
/// - `content`: The text content after the marker
/// - `width`: The maximum line width
pub fn wrap_list_item(indent: &str, marker: &str, content: &str, width: usize) -> String {
    if content.is_empty() {
        return format!("{}{}\n", indent, marker.trim_end());
    }

    let full_marker = format!("{indent}{marker}");
    let continuation_indent = format!("{}{}", indent, " ".repeat(marker.len()));
    let prefix_len = full_marker.len();
    // ~keep A link-only item is not reflowed, but a line after a hard break in its label still
    // ~keep needs the item's indent, or it leaves the item and the link breaks (issue #678).
    let wrapped = if is_single_inline_link(content) {
        content.trim().to_string()
    } else {
        wrap_line(content, if width > prefix_len { width - prefix_len } else { width })
    };

    let mut result = String::with_capacity(wrapped.len() + prefix_len * 2);
    for (index, line) in wrapped.split('\n').enumerate() {
        result.push_str(if index == 0 { &full_marker } else { &continuation_indent });
        result.push_str(line);
        result.push('\n');
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_line_short() {
        let text = "Short text";
        let wrapped = wrap_line(text, 80);
        assert_eq!(wrapped, "Short text");
    }

    #[test]
    fn test_wrap_line_long() {
        let text = "123456789 123456789";
        let wrapped = wrap_line(text, 10);
        assert_eq!(wrapped, "123456789\n123456789");
    }

    #[test]
    fn test_wrap_line_no_break_long_words() {
        let text = "12345678901 12345";
        let wrapped = wrap_line(text, 10);
        assert_eq!(wrapped, "12345678901\n12345");
    }

    #[test]
    fn hard_break_reads_two_spaces_or_an_odd_run_of_backslashes() {
        assert_eq!(hard_break("a  "), Some("  "));
        assert_eq!(hard_break("a\\"), Some(""));
        assert_eq!(hard_break("a\\\\"), None);
        assert_eq!(hard_break("a\\\\\\"), Some(""));
        assert_eq!(hard_break("a "), None);
        assert_eq!(hard_break("a\\ "), None);
        assert_eq!(hard_break("   "), None);
        assert_eq!(hard_break("\u{a0}  "), Some("  "));
    }

    #[test]
    fn wrap_line_keeps_a_link_destination_in_angle_brackets_whole() {
        assert_eq!(
            wrap_line("see [x](<a b c d e f>) and more", 10),
            "see\n[x](<a b c d e f>)\nand more"
        );
    }

    #[test]
    fn wrap_line_checks_a_line_again_when_a_word_moves_into_it() {
        assert_eq!(wrap_line("a - - x", 1), "a - -\nx");
        assert_eq!(wrap_line("a * - x", 1), "a * -\nx");
        assert_eq!(wrap_line("--- x y", 1), "--- x\ny");
        for text in [
            "xx --- --- --- --- --- --- --- --- yy zz",
            "intro text * * * * * * * * * * * * outro",
            "some words here === === === === === === === more text",
            "a - b - c - d - e - f",
            "a * - x * * - - + y",
        ] {
            for width in 1..=40 {
                let wrapped = wrap_line(text, width);
                assert_eq!(
                    wrapped.split_whitespace().collect::<Vec<_>>(),
                    words(text),
                    "{text:?} at {width}"
                );
                for line in wrapped.lines() {
                    assert!(
                        !opens_block(line),
                        "{text:?} at {width}: {line:?} opens a block in {wrapped:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn wrap_line_starts_no_line_with_a_block_in_any_short_run_of_markers() {
        let vocabulary = [
            "*", "**", "_", "__", "-", "+", "1.", "01.", "001)", "x", "* *", "***", "_ _",
        ];
        let mut texts: Vec<Vec<&str>> = vec![Vec::new()];
        for _ in 0..4 {
            texts = texts
                .iter()
                .flat_map(|text| {
                    vocabulary.iter().map(move |word| {
                        let mut longer = text.clone();
                        longer.push(*word);
                        longer
                    })
                })
                .collect();
            for text in &texts {
                let text = text.join(" ");
                if opens_block(&text) {
                    continue;
                }
                for width in 1..=6 {
                    let wrapped = wrap_line(&text, width);
                    assert!(!wrapped.lines().any(opens_block), "{text:?} at {width}: {wrapped:?}");
                }
            }
        }
    }

    #[test]
    fn is_bare_marker_holds_every_short_line_that_a_word_turns_into_a_block() {
        let alphabet: Vec<char> = "*+-_01.)#>=`~< a2".chars().collect();
        let words = [
            "x", "*", "-", "_", "+", "1.", "01.", "**", "__", "`", "---", "===", "!--",
        ];
        let mut lines = vec![String::new()];
        for _ in 0..4 {
            lines = lines
                .iter()
                .flat_map(|line| alphabet.iter().map(move |c| format!("{line}{c}")))
                .collect();
            for line in &lines {
                if line.starts_with(' ') || line.ends_with(' ') || line.contains("  ") || opens_block(line) {
                    continue;
                }
                for word in words {
                    assert!(
                        is_bare_marker(line) || !opens_block(&format!("{line} {word}")),
                        "{line:?} + {word:?} opens a block"
                    );
                }
            }
        }
    }

    #[test]
    fn push_paragraph_line_keeps_the_line_end_after_a_bare_marker() {
        for (first, second) in [
            ("*", "b"),
            ("1.", "b c"),
            ("01.", "b c"),
            ("000000001)", "b"),
            ("+", "b"),
            ("**", "*"),
            ("_ _", "_"),
        ] {
            let mut text = String::from("a  \n");
            push_paragraph_line(&mut text, first);
            push_paragraph_line(&mut text, second);
            assert_eq!(text, format!("a  \n{first}\n{second}"));
        }
        let mut text = String::from("x");
        push_paragraph_line(&mut text, "*");
        push_paragraph_line(&mut text, "b");
        assert_eq!(text, "x * b");
    }

    #[test]
    fn wrap_line_drops_a_hard_break_at_the_end_of_the_text() {
        assert_eq!(wrap_line("one two  \n", 5), "one\ntwo");
    }

    #[test]
    fn push_paragraph_line_keeps_non_breaking_spaces_at_the_line_ends() {
        let mut text = String::new();
        push_paragraph_line(&mut text, "\u{a0}x\u{a0}");
        assert_eq!(text, "\u{a0}x\u{a0}");
    }

    #[test]
    fn wrap_line_keeps_an_escaped_bracket_inside_a_link_destination() {
        assert_eq!(
            wrap_line("see [x](<a\\> b c d e>) and more", 10),
            "see\n[x](<a\\> b c d e>)\nand more"
        );
    }
}
