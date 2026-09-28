//! Synchronous text wrapping for Markdown output.

use super::utils::{
    is_heading, is_list_like, is_numbered_list, is_setext_underline, is_thematic_break, parse_blockquote_line,
    parse_list_item, wrap_blockquote_paragraph, wrap_indented_line, wrap_list_item,
};
use crate::options::ConversionOptions;

/// Wrap text at specified width while preserving Markdown formatting.
///
/// This function wraps paragraphs of text at the specified width, but:
/// - Does not break long words
/// - Does not break on hyphens
/// - Preserves Markdown formatting (links, bold, etc.)
/// - Only wraps paragraph content, not headers, lists, code blocks, etc.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn wrap_markdown(markdown: &str, options: &ConversionOptions) -> String {
    if !options.wrap {
        return markdown.to_string();
    }

    let mut result = String::with_capacity(markdown.len());
    let mut in_code_block = false;
    let mut in_paragraph = false;
    let mut paragraph_buffer = String::new();
    let mut paragraph_indent = String::new();
    let mut in_blockquote_paragraph = false;
    let mut blockquote_prefix = String::new();
    let mut blockquote_indent = String::new();
    let mut blockquote_buffer = String::new();

    for line in markdown.lines() {
        let trimmed = line.trim_start();
        let is_code_fence = trimmed.starts_with("```");
        let is_indented_code = line.starts_with("    ")
            && !is_list_like(trimmed)
            && !is_numbered_list(trimmed)
            && !is_heading(trimmed)
            && !trimmed.starts_with('>')
            && !trimmed.starts_with('|');

        if is_code_fence || is_indented_code {
            if in_paragraph && !paragraph_buffer.is_empty() {
                result.push_str(&wrap_indented_line(
                    &paragraph_indent,
                    &paragraph_buffer,
                    options.wrap_width,
                ));
                result.push_str("\n\n");
                paragraph_buffer.clear();
                in_paragraph = false;
            }

            if is_code_fence {
                in_code_block = !in_code_block;
            }
            result.push_str(line);
            result.push('\n');
            continue;
        }

        if in_code_block {
            result.push_str(line);
            result.push('\n');
            continue;
        }

        if let Some((prefix, content)) = parse_blockquote_line(line) {
            // ~keep The indent after the quote prefix is a list item's content column inside
            // ~keep the quote; written without it, the paragraph would leave the item.
            let after_prefix = &line[prefix.len()..];
            let indent = &after_prefix[..after_prefix.len() - after_prefix.trim_start().len()];
            if in_paragraph && !paragraph_buffer.is_empty() {
                result.push_str(&wrap_indented_line(
                    &paragraph_indent,
                    &paragraph_buffer,
                    options.wrap_width,
                ));
                result.push_str("\n\n");
                paragraph_buffer.clear();
                in_paragraph = false;
            }

            let mut normalized_prefix = prefix;
            if !normalized_prefix.ends_with(' ') {
                normalized_prefix.push(' ');
            }

            if content.is_empty() {
                if in_blockquote_paragraph && !blockquote_buffer.is_empty() {
                    result.push_str(&wrap_blockquote_paragraph(
                        &format!("{blockquote_prefix}{blockquote_indent}"),
                        &blockquote_buffer,
                        options.wrap_width,
                    ));
                    result.push('\n');
                    blockquote_buffer.clear();
                    in_blockquote_paragraph = false;
                }
                result.push_str(normalized_prefix.trim_end());
                result.push('\n');
                continue;
            }

            if in_blockquote_paragraph && normalized_prefix != blockquote_prefix {
                result.push_str(&wrap_blockquote_paragraph(
                    &format!("{blockquote_prefix}{blockquote_indent}"),
                    &blockquote_buffer,
                    options.wrap_width,
                ));
                result.push('\n');
                blockquote_buffer.clear();
                in_blockquote_paragraph = false;
            }

            // ~keep Inside a quote a rule or a heading underline keeps its own line too (#607).
            let underline = in_blockquote_paragraph && is_setext_underline(content.trim_start());
            if underline || is_thematic_break(content.trim_start()) {
                if in_blockquote_paragraph && !blockquote_buffer.is_empty() {
                    if underline {
                        result.push_str(&blockquote_prefix);
                        result.push_str(&blockquote_indent);
                        result.push_str(&blockquote_buffer);
                    } else {
                        result.push_str(&wrap_blockquote_paragraph(
                            &format!("{blockquote_prefix}{blockquote_indent}"),
                            &blockquote_buffer,
                            options.wrap_width,
                        ));
                    }
                    result.push('\n');
                    blockquote_buffer.clear();
                    in_blockquote_paragraph = false;
                }
                result.push_str(line);
                result.push('\n');
                continue;
            }

            if in_blockquote_paragraph {
                blockquote_buffer.push(' ');
                blockquote_buffer.push_str(&content);
            } else {
                blockquote_prefix = normalized_prefix;
                blockquote_indent.clear();
                blockquote_indent.push_str(indent);
                blockquote_buffer.push_str(&content);
                in_blockquote_paragraph = true;
            }
            continue;
        } else if in_blockquote_paragraph && !blockquote_buffer.is_empty() {
            result.push_str(&wrap_blockquote_paragraph(
                &format!("{blockquote_prefix}{blockquote_indent}"),
                &blockquote_buffer,
                options.wrap_width,
            ));
            result.push('\n');
            blockquote_buffer.clear();
            in_blockquote_paragraph = false;
        }

        // ~keep An underline right under paragraph text makes that text its heading: the text ends
        // ~keep with one line break, and it is not reflowed, like an ATX heading. Folded into the
        // ~keep text, or cut off by a blank line, the underline is lost (#607).
        if in_paragraph && !paragraph_buffer.is_empty() && is_setext_underline(trimmed) {
            result.push_str(&paragraph_indent);
            result.push_str(&paragraph_buffer);
            result.push('\n');
            paragraph_buffer.clear();
            in_paragraph = false;
            result.push_str(line);
            result.push('\n');
            continue;
        }

        if let Some((indent, marker, content)) = parse_list_item(line) {
            if in_paragraph && !paragraph_buffer.is_empty() {
                result.push_str(&wrap_indented_line(
                    &paragraph_indent,
                    &paragraph_buffer,
                    options.wrap_width,
                ));
                result.push_str("\n\n");
                paragraph_buffer.clear();
                in_paragraph = false;
            }

            result.push_str(&wrap_list_item(&indent, &marker, &content, options.wrap_width));
            continue;
        }

        let is_structural = is_heading(trimmed)
            || is_thematic_break(trimmed)
            || trimmed.starts_with('>')
            || trimmed.starts_with('|')
            || trimmed.starts_with('=');

        if is_structural {
            if in_paragraph && !paragraph_buffer.is_empty() {
                result.push_str(&wrap_indented_line(
                    &paragraph_indent,
                    &paragraph_buffer,
                    options.wrap_width,
                ));
                result.push_str("\n\n");
                paragraph_buffer.clear();
                in_paragraph = false;
            }

            result.push_str(line);
            result.push('\n');
            continue;
        }

        if line.trim().is_empty() {
            if in_paragraph && !paragraph_buffer.is_empty() {
                result.push_str(&wrap_indented_line(
                    &paragraph_indent,
                    &paragraph_buffer,
                    options.wrap_width,
                ));
                result.push_str("\n\n");
                paragraph_buffer.clear();
                in_paragraph = false;
            } else if !in_paragraph {
                result.push('\n');
            }
            continue;
        }

        if in_paragraph {
            paragraph_buffer.push(' ');
        } else {
            paragraph_indent.clear();
            paragraph_indent.push_str(&line[..line.len() - trimmed.len()]);
        }
        paragraph_buffer.push_str(line.trim());
        in_paragraph = true;
    }

    if in_blockquote_paragraph && !blockquote_buffer.is_empty() {
        result.push_str(&wrap_blockquote_paragraph(
            &format!("{blockquote_prefix}{blockquote_indent}"),
            &blockquote_buffer,
            options.wrap_width,
        ));
        result.push('\n');
    }

    if in_paragraph && !paragraph_buffer.is_empty() {
        result.push_str(&wrap_indented_line(
            &paragraph_indent,
            &paragraph_buffer,
            options.wrap_width,
        ));
        result.push_str("\n\n");
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_markdown_disabled() {
        let markdown = "This is a very long line that would normally be wrapped at 40 characters";
        let options = ConversionOptions {
            wrap: false,
            ..Default::default()
        };
        let result = wrap_markdown(markdown, &options);
        assert_eq!(result, markdown);
    }

    fn wrap_at_20(markdown: &str) -> String {
        let options = ConversionOptions {
            wrap: true,
            wrap_width: 20,
            ..Default::default()
        };
        wrap_markdown(markdown, &options)
    }

    #[test]
    fn wrap_markdown_keeps_a_thematic_break_on_its_own_line() {
        assert_eq!(wrap_at_20("t\n\n---\nB\n"), "t\n\n---\nB\n\n");
        assert_eq!(wrap_at_20("***\nB\n"), "***\nB\n\n");
        assert_eq!(wrap_at_20("___\nB\n"), "___\nB\n\n");
    }

    #[test]
    fn wrap_markdown_keeps_a_setext_underline_under_its_heading_text() {
        assert_eq!(wrap_at_20("Heading\n-------\n\nx\n"), "Heading\n-------\n\nx\n\n");
        assert_eq!(wrap_at_20("Heading\n=======\n\nx\n"), "Heading\n=======\n\nx\n\n");
    }

    #[test]
    fn wrap_markdown_keeps_a_rule_and_an_underline_on_their_own_lines_in_a_quote() {
        assert_eq!(wrap_at_20("> t\n>\n> ---\n>  B\n"), "> t\n>\n> ---\n>  B\n");
        assert_eq!(wrap_at_20("> Heading\n> -------\n> x\n"), "> Heading\n> -------\n> x\n");
    }

    #[test]
    fn wrap_markdown_still_joins_ordinary_paragraph_lines() {
        assert_eq!(wrap_at_20("a\nb--\n"), "a b--\n\n");
    }

    #[test]
    fn test_wrap_markdown_paragraph() {
        let markdown = "This is a very long line that would normally be wrapped at 40 characters\n\n";
        let options = ConversionOptions {
            wrap: true,
            wrap_width: 40,
            ..Default::default()
        };
        let result = wrap_markdown(markdown, &options);
        assert!(result.lines().all(|line| line.len() <= 40 || line.trim().is_empty()));
    }

    #[test]
    fn test_wrap_markdown_blockquote_paragraph() {
        let markdown = "> This is a very long blockquote line that should wrap at 30 characters\n";
        let options = ConversionOptions {
            wrap: true,
            wrap_width: 30,
            ..Default::default()
        };
        let result = wrap_markdown(markdown, &options);
        assert!(
            result.lines().all(|line| line.len() <= 30 || line.trim().is_empty()),
            "Some lines exceed wrap width. Got: {result}"
        );
        assert!(
            result.contains("> This is a very"),
            "Missing expected wrapped content. Got: {result}"
        );
        assert!(
            result.lines().filter(|l| l.starts_with("> ")).count() >= 2,
            "Expected multiple wrapped blockquote lines. Got: {result}"
        );
    }

    #[test]
    fn test_wrap_markdown_preserves_code() {
        let markdown = "```\nThis is a very long line in a code block that should not be wrapped\n```\n";
        let options = ConversionOptions {
            wrap: true,
            wrap_width: 40,
            ..Default::default()
        };
        let result = wrap_markdown(markdown, &options);
        assert!(result.contains("This is a very long line in a code block that should not be wrapped"));
    }

    #[test]
    fn test_wrap_markdown_preserves_headings() {
        let markdown = "# This is a very long heading that should not be wrapped even if it exceeds the width\n\n";
        let options = ConversionOptions {
            wrap: true,
            wrap_width: 40,
            ..Default::default()
        };
        let result = wrap_markdown(markdown, &options);
        assert!(
            result.contains("# This is a very long heading that should not be wrapped even if it exceeds the width")
        );
    }

    #[test]
    fn wrap_markdown_wraps_long_list_items() {
        let markdown = "- This is a very long list item that should definitely be wrapped when it exceeds the specified wrap width\n- Short item\n";
        let options = ConversionOptions {
            wrap: true,
            wrap_width: 60,
            ..Default::default()
        };

        let result = wrap_markdown(markdown, &options);

        assert!(
            result.contains("- This is a very long list item that should definitely be\n  wrapped"),
            "First list item not properly wrapped. Got: {result}"
        );
        assert!(
            result.contains("- Short item"),
            "Short list item incorrectly modified. Got: {result}"
        );
    }

    #[test]
    fn wrap_markdown_wraps_ordered_lists() {
        let markdown = "1. This is a numbered list item with a very long text that should be wrapped at the specified width\n2. Short\n";
        let options = ConversionOptions {
            wrap: true,
            wrap_width: 60,
            ..Default::default()
        };

        let result = wrap_markdown(markdown, &options);

        assert!(
            result.lines().all(|line| line.len() <= 60 || line.trim().is_empty()),
            "Some lines exceed wrap width. Got: {result}"
        );
        assert!(result.contains("1."), "Lost ordered list marker. Got: {result}");
        assert!(result.contains("2."), "Lost second ordered list marker. Got: {result}");
    }

    #[test]
    fn wrap_markdown_preserves_nested_list_structure() {
        let markdown = "- Item one with some additional text that will need to be wrapped across multiple lines\n  - Nested item with long text that also needs wrapping at the specified width\n  - Short nested\n";
        let options = ConversionOptions {
            wrap: true,
            wrap_width: 50,
            ..Default::default()
        };

        let result = wrap_markdown(markdown, &options);

        assert!(result.contains("- Item"), "Lost top-level list marker. Got: {result}");
        assert!(
            result.contains("  - Nested"),
            "Lost nested list structure. Got: {result}"
        );
        assert!(
            result.lines().all(|line| line.len() <= 50 || line.trim().is_empty()),
            "Some lines exceed wrap width. Got: {result}"
        );
    }

    #[test]
    fn wrap_markdown_handles_list_with_links() {
        let markdown = "- [A](#a) with additional text that is long enough to require wrapping at the configured width\n  - [B](#b) also has more content that needs wrapping\n  - [C](#c)\n";
        let options = ConversionOptions {
            wrap: true,
            wrap_width: 50,
            ..Default::default()
        };

        let result = wrap_markdown(markdown, &options);

        assert!(result.contains("[A](#a)"), "Lost link in list. Got: {result}");
        assert!(result.contains("[B](#b)"), "Lost nested link. Got: {result}");
        assert!(result.contains("[C](#c)"), "Lost short nested link. Got: {result}");
        assert!(
            result.contains("- [A](#a)"),
            "Lost list structure with link. Got: {result}"
        );
        assert!(
            result.contains("  - [B](#b)"),
            "Lost nested list structure. Got: {result}"
        );
    }

    #[test]
    fn wrap_markdown_handles_empty_list_items() {
        let markdown = "- \n- Item with text\n- \n";
        let options = ConversionOptions {
            wrap: true,
            wrap_width: 40,
            ..Default::default()
        };

        let result = wrap_markdown(markdown, &options);

        assert!(result.contains("- "), "Lost list markers. Got: {result}");
        assert!(result.contains("Item with text"), "Lost item text. Got: {result}");
    }

    #[test]
    fn wrap_markdown_preserves_indented_lists_with_wrapping() {
        let markdown = "- [A](#a) with some additional text that makes this line very long and should be wrapped\n  - [B](#b)\n  - [C](#c) with more text that is also quite long and needs wrapping\n";
        let options = ConversionOptions {
            wrap: true,
            wrap_width: 50,
            ..Default::default()
        };

        let result = wrap_markdown(markdown, &options);

        assert!(result.contains("- [A](#a)"), "Lost top-level link. Got: {result}");
        assert!(result.contains("  - [B](#b)"), "Lost nested link B. Got: {result}");
        assert!(result.contains("  - [C](#c)"), "Lost nested link C. Got: {result}");
        assert!(
            result.lines().all(|line| line.len() <= 50),
            "Some lines exceed wrap width:\n{result}"
        );
    }

    #[test]
    fn wrap_markdown_does_not_wrap_link_only_items() {
        let markdown = "- [A very long link label that would exceed wrap width](#a-very-long-link-label)\n  - [Nested very long link label that would also exceed](#nested)\n";
        let options = ConversionOptions {
            wrap: true,
            wrap_width: 30,
            ..Default::default()
        };

        let result = wrap_markdown(markdown, &options);

        assert!(result.contains("- [A very long link label that would exceed wrap width](#a-very-long-link-label)"));
        assert!(result.contains("  - [Nested very long link label that would also exceed](#nested)"));
    }
}
