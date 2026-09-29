//! Utility functions for text wrapping.
//!
//! This module contains helper functions for parsing and wrapping Markdown elements.

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
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];

    if let Some(rest) = trimmed.strip_prefix('-') {
        if rest.starts_with(' ') || rest.is_empty() {
            return Some((indent.to_string(), "- ".to_string(), rest.trim_start().to_string()));
        }
    }
    if let Some(rest) = trimmed.strip_prefix('*') {
        if rest.starts_with(' ') || rest.is_empty() {
            return Some((indent.to_string(), "* ".to_string(), rest.trim_start().to_string()));
        }
    }
    if let Some(rest) = trimmed.strip_prefix('+') {
        if rest.starts_with(' ') || rest.is_empty() {
            return Some((indent.to_string(), "+ ".to_string(), rest.trim_start().to_string()));
        }
    }

    let first_token = trimmed.split_whitespace().next()?;
    if first_token.ends_with('.') || first_token.ends_with(')') {
        let digits = first_token.trim_end_matches(['.', ')']);
        if !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit()) {
            let marker_len = first_token.len();
            let rest = trimmed[marker_len..].trim_start();
            return Some((
                indent.to_string(),
                trimmed[..marker_len].to_string() + " ",
                rest.to_string(),
            ));
        }
    }

    None
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
/// with a hard line break.
///
/// ~keep A hard break is a line end the reflow never joins across, so `text` keeps it as a
/// ~keep newline and [`wrap_line`] wraps the text on each side of it on its own (#613).
pub fn push_paragraph_line(text: &mut String, line: &str) {
    if !text.is_empty() && !text.ends_with('\n') {
        text.push(' ');
    }
    text.push_str(line.trim());
    if let Some(spaces) = hard_break(line) {
        text.push_str(spaces);
        text.push('\n');
    }
}

/// Wrap a single line of text at the specified width.
///
/// This function wraps text without breaking long words or on hyphens,
/// similar to Python's `textwrap.fill()` with `break_long_words=False` and `break_on_hyphens=False`.
/// A newline in `text` is a hard line break: the text on each side is wrapped on its own, and
/// the spaces before the newline are kept.
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

/// Wrap the words of `text`, a text without newlines, at `width` onto `result`.
fn wrap_words(text: &str, width: usize, result: &mut String) {
    let mut lines: Vec<Vec<&str>> = vec![Vec::new()];
    let mut line_len = 0;
    for word in text.split_whitespace() {
        if line_len > 0 && line_len + 1 + word.len() > width {
            lines.push(Vec::new());
            line_len = 0;
        }
        line_len += usize::from(line_len > 0) + word.len();
        lines.last_mut().expect("lines starts with one line").push(word);
    }
    let lines: Vec<String> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.join(" "))
        .collect();
    result.push_str(&lines.join("\n"));
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

    if is_single_inline_link(content) {
        return format!("{}{}{}\n", indent, marker, content.trim());
    }

    let full_marker = format!("{indent}{marker}");
    let continuation_indent = format!("{}{}", indent, " ".repeat(marker.len()));
    let prefix_len = full_marker.len();
    let wrapped = wrap_line(content, if width > prefix_len { width - prefix_len } else { width });
    if wrapped.trim().is_empty() {
        return format!("{}\n", full_marker.trim_end());
    }

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
    fn wrap_line_drops_a_hard_break_at_the_end_of_the_text() {
        assert_eq!(wrap_line("one two  \n", 5), "one\ntwo");
    }
}
