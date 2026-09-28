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

/// Check if a line is a thematic break: three or more `-`, `*` or `_`, spaces allowed between them.
pub fn is_thematic_break(trimmed: &str) -> bool {
    let mut marks = trimmed.chars().filter(|c| !matches!(c, ' ' | '\t'));
    let Some(first) = marks.next() else {
        return false;
    };
    let mut count = 1;
    for mark in marks {
        if mark != first {
            return false;
        }
        count += 1;
    }
    matches!(first, '-' | '*' | '_') && count >= 3
}

/// Check if a line is a setext heading underline: a run of `=` or of `-` with nothing else.
pub fn is_setext_underline(trimmed: &str) -> bool {
    let underline = trimmed.trim_end();
    !underline.is_empty() && (underline.bytes().all(|b| b == b'=') || underline.bytes().all(|b| b == b'-'))
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

/// Wrap a single line of text at the specified width.
///
/// This function wraps text without breaking long words or on hyphens,
/// similar to Python's `textwrap.fill()` with `break_long_words=False` and `break_on_hyphens=False`.
pub fn wrap_line(text: &str, width: usize) -> String {
    if text.len() <= width {
        return text.to_string();
    }

    let mut result = String::new();
    let mut current_line = String::new();
    let words: Vec<&str> = text.split_whitespace().collect();

    for word in words {
        if current_line.is_empty() {
            current_line.push_str(word);
        } else if current_line.len() + 1 + word.len() <= width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(&current_line);
            current_line.clear();
            current_line.push_str(word);
        }
    }

    if !current_line.is_empty() {
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(&current_line);
    }

    result
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

    let first_line_prefix_len = full_marker.len();
    let first_line_width = if width > first_line_prefix_len {
        width - first_line_prefix_len
    } else {
        width
    };

    let cont_line_prefix_len = continuation_indent.len();
    let cont_line_width = if width > cont_line_prefix_len {
        width - cont_line_prefix_len
    } else {
        width
    };

    let words: Vec<&str> = content.split_whitespace().collect();
    if words.is_empty() {
        return format!("{}\n", full_marker.trim_end());
    }

    let mut result = String::new();
    let mut current_line = String::new();
    let mut current_width = first_line_width;
    let mut is_first_line = true;

    for word in words {
        let word_len = word.len();
        let space_needed = usize::from(!current_line.is_empty());

        if !current_line.is_empty() && current_line.len() + space_needed + word_len > current_width {
            if is_first_line {
                result.push_str(&full_marker);
                is_first_line = false;
            } else {
                result.push_str(&continuation_indent);
            }
            result.push_str(&current_line);
            result.push('\n');
            current_line.clear();
            current_width = cont_line_width;
        }

        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }

    if !current_line.is_empty() {
        if is_first_line {
            result.push_str(&full_marker);
        } else {
            result.push_str(&continuation_indent);
        }
        result.push_str(&current_line);
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
}
