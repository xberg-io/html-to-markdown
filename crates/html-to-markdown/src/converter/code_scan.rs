//! The code blocks of finished Markdown.
//!
//! The passes that clean the finished document, and the containers that trim the Markdown they hold,
//! ask this module which lines are code. A line of code keeps its white space.

use crate::options::CodeBlockStyle;

/// Follows the code blocks of finished Markdown, one line at a time.
pub enum CodeScan {
    /// The style writes fenced code blocks.
    Fenced(FenceScan),
    /// The style writes indented code blocks: whether each line is code, in the order of the lines.
    Indented(std::vec::IntoIter<bool>),
}

impl CodeScan {
    /// A scan of `output` for the code blocks that `style` writes.
    ///
    /// ~keep Only the configured style opens a block. A line of indented code that looks like a
    /// ~keep fence opens nothing, and neither does a line of tildes in a document whose fences are
    /// ~keep backticks.
    pub fn new(style: CodeBlockStyle, output: &str) -> Self {
        match style {
            CodeBlockStyle::Indented => Self::Indented(indented_code_lines(output).into_iter()),
            CodeBlockStyle::Backticks => Self::Fenced(FenceScan {
                fence: b'`',
                open: None,
            }),
            CodeBlockStyle::Tildes => Self::Fenced(FenceScan {
                fence: b'~',
                open: None,
            }),
        }
    }

    /// Whether `line`, the next line of the output, is a line of code.
    pub fn is_code(&mut self, line: &str) -> bool {
        match self {
            Self::Fenced(fences) => fences.is_code(line),
            Self::Indented(lines) => lines.next().unwrap_or(false),
        }
    }
}

/// Follows the fenced code blocks of finished Markdown, one line at a time.
pub struct FenceScan {
    /// The fence character of the configured style.
    fence: u8,
    open: Option<(u8, usize)>,
}

impl FenceScan {
    /// Whether `line` is a line of code: a line between the two fences of a code block.
    ///
    /// ~keep A fence is longer than every run of its character in the code, so no line of code
    /// ~keep closes it. A block quote and a list item put their prefix before every line.
    fn is_code(&mut self, line: &str) -> bool {
        use crate::converter::utility::escaping::code_fence;

        let Some((marker, length)) = self.open else {
            self.open = code_fence(after_container_markers(line)).filter(|(marker, _)| *marker == self.fence);
            return false;
        };
        let rest = line.trim_start_matches([' ', '\t', '>']);
        let closes = code_fence(rest).is_some_and(|(fence, run)| fence == marker && run >= length)
            && rest.trim_start_matches(char::from(marker)).trim().is_empty();
        if closes {
            self.open = None;
        }
        !closes
    }
}

/// `line` without the block quote and list item markers that can precede an opening fence.
fn after_container_markers(mut line: &str) -> &str {
    loop {
        line = line.trim_start_matches([' ', '\t', '>']);
        let Some(marker) = list_marker_length(line) else {
            return line;
        };
        line = &line[marker..];
    }
}

/// The length of the list item marker that `rest` starts with: a bullet, or a number of at most
/// nine digits with its delimiter, before a space, a tab or the end of the line.
fn list_marker_length(rest: &str) -> Option<usize> {
    let bytes = rest.as_bytes();
    let digits = bytes.iter().take_while(|byte| byte.is_ascii_digit()).count();
    let length = match bytes.first()? {
        b'-' | b'*' | b'+' => 1,
        _ if (1..=9).contains(&digits) && matches!(bytes.get(digits), Some(b'.' | b')')) => digits + 1,
        _ => return None,
    };
    matches!(bytes.get(length), None | Some(b' ' | b'\t')).then_some(length)
}

/// A block whose marker precedes its lines: a block quote, or a list item with the number of
/// columns its content is indented by.
enum Container {
    Quote,
    Item(usize),
}

/// A position in a line that counts columns: a tab reaches the next multiple of four.
struct LineCursor<'a> {
    rest: &'a str,
    column: usize,
    /// Columns of a tab that `skip` passed but did not use.
    spare: usize,
}

impl LineCursor<'_> {
    /// The columns of white space at the cursor.
    fn indent(&self) -> usize {
        let mut column = self.column;
        for byte in self.rest.bytes() {
            match byte {
                b' ' => column += 1,
                b'\t' => column += 4 - column % 4,
                _ => break,
            }
        }
        self.spare + column - self.column
    }

    /// Move past `columns` columns of white space.
    fn skip(&mut self, mut columns: usize) {
        let used = columns.min(self.spare);
        self.spare -= used;
        columns -= used;
        while columns > 0 {
            let width = match self.rest.as_bytes().first() {
                Some(b' ') => 1,
                Some(b'\t') => 4 - self.column % 4,
                _ => return,
            };
            self.advance(1);
            self.column += width - 1;
            if width > columns {
                self.spare = width - columns;
                return;
            }
            columns -= width;
        }
    }

    /// Move past `bytes` bytes that are one column each.
    fn advance(&mut self, bytes: usize) {
        self.rest = &self.rest[bytes..];
        self.column += bytes;
    }

    /// Move past the `>` of a block quote and the one space that can follow it.
    fn skip_quote_marker(&mut self) {
        self.advance(1);
        if self.indent() > 0 {
            self.skip(1);
        }
    }

    fn is_blank(&self) -> bool {
        self.rest.trim_matches([' ', '\t']).is_empty()
    }

    /// Move past the markers of the `containers` that the line continues, the outer ones first.
    /// Gives the number of containers that the line continues.
    fn enter(&mut self, containers: &[Container]) -> usize {
        let mut depth = 0;
        for container in containers {
            match *container {
                Container::Quote => {
                    let indent = self.indent();
                    if indent > 3 || !self.rest.trim_start_matches([' ', '\t']).starts_with('>') {
                        break;
                    }
                    self.skip(indent);
                    self.skip_quote_marker();
                }
                Container::Item(content) => {
                    if self.indent() >= content {
                        self.skip(content);
                    } else if !self.is_blank() {
                        break;
                    }
                }
            }
            depth += 1;
        }
        depth
    }
}

/// Whether each line of `output` is a line of an indented code block (CommonMark 4.4).
///
/// ~keep A line is code when it is indented by four columns or more past the block quotes and the
/// ~keep list items it is in, and it does not continue a paragraph. The blank lines between two
/// ~keep lines of one code block belong to the block. The indentation alone does not say this:
/// ~keep the third level of a list is indented by four columns too.
fn indented_code_lines(output: &str) -> Vec<bool> {
    let mut code = Vec::new();
    let mut containers: Vec<Container> = Vec::new();
    let mut in_paragraph = false;
    let mut last_code: Option<usize> = None;
    for (index, line) in output.split('\n').enumerate() {
        code.push(false);
        let mut cursor = LineCursor {
            rest: line,
            column: 0,
            spare: 0,
        };
        let depth = cursor.enter(&containers);
        if cursor.is_blank() {
            in_paragraph = false;
            continue;
        }
        if depth < containers.len() {
            containers.truncate(depth);
            last_code = None;
        }
        let is_code = loop {
            let indent = cursor.indent();
            if indent >= 4 {
                break !in_paragraph;
            }
            cursor.skip(indent);
            if cursor.rest.starts_with('>') {
                cursor.skip_quote_marker();
                containers.push(Container::Quote);
            } else if let Some(marker) = list_marker_length(cursor.rest) {
                cursor.advance(marker);
                // ~keep The content of an item starts after one to four spaces. With more, or with
                // ~keep nothing after the marker, it starts after one.
                let padding = match cursor.indent() {
                    spaces @ 1..=4 if !cursor.is_blank() => spaces,
                    _ => 1,
                };
                cursor.skip(padding);
                containers.push(Container::Item(indent + marker + padding));
            } else {
                in_paragraph = true;
                break false;
            }
            in_paragraph = false;
            last_code = None;
            if cursor.is_blank() {
                break false;
            }
        };
        if is_code {
            code[index] = true;
            if let Some(last) = last_code {
                code[last + 1..index].fill(true);
            }
            last_code = Some(index);
        } else {
            last_code = None;
        }
    }
    code
}

/// The part of `content`, the Markdown inside a block quote, without the white space around it.
///
/// ~keep With the indented style the indentation of a first line of code stays, and so does the
/// ~keep line end of a last line of code: without its four columns the line is running text.
pub fn quote_content_range(content: &str, style: CodeBlockStyle) -> std::ops::Range<usize> {
    let start = content.len() - content.trim_start().len();
    let end = content.trim_end().len();
    if style != CodeBlockStyle::Indented || start >= end {
        return start..end.max(start);
    }
    let first_line = content[..start].rfind('\n').map_or(0, |index| index + 1);
    let last_line = content[end..].find('\n').map_or(content.len(), |index| end + index);
    let code = indented_code_lines(&content[first_line..last_line]);
    let start = if code.first() == Some(&true) { first_line } else { start };
    let end = if code.last() == Some(&true) { last_line } else { end };
    start..end
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One character for each line of `markdown`: `1` for a line of indented code, `0` for another.
    fn code(markdown: &str) -> String {
        indented_code_lines(markdown)
            .into_iter()
            .map(|is_code| if is_code { '1' } else { '0' })
            .collect()
    }

    #[test]
    fn should_measure_indented_code_from_the_content_column_of_a_list_item() {
        let cases = [
            // A number without a delimiter is no item marker: the last line is code at the top level.
            ("7  x\n\n     c", "001"),
            // An item whose marker has three spaces after it has its content at column four.
            ("-   a\n\n      b", "000"),
            // A marker with only spaces after it has its content one column after the marker.
            ("-  \n      b", "01"),
            // The columns before a marker belong to the content column of its item.
            (" - a\n\n      b", "000"),
            // A line at the left margin closes the item, so five columns after it are code.
            ("- a\n\nb\n\n     c", "00001"),
        ];
        for (markdown, expected) in cases {
            assert_eq!(code(markdown), expected, "{markdown:?}");
        }
    }

    #[test]
    fn should_measure_indented_code_after_the_marker_of_a_block_quote() {
        let cases = [
            // One space after the marker belongs to the marker.
            (">    a", "0"),
            // A marker after three columns continues the quote and its code block.
            (">     a\n>\n   >     b", "111"),
            // A tab after the marker gives one column to the marker and two to the content
            // (CommonMark example 6).
            (">\t foo", "0"),
            (">\t\tfoo", "1"),
        ];
        for (markdown, expected) in cases {
            assert_eq!(code(markdown), expected, "{markdown:?}");
        }
    }

    #[test]
    fn should_count_a_tab_to_the_next_multiple_of_four_columns() {
        // The tab after two spaces is two columns wide, less than the content column of the item.
        assert_eq!(code("1234. a\n\n  \tb"), "001");

        let mut cursor = LineCursor {
            rest: "  \t\tx",
            column: 0,
            spare: 0,
        };
        assert_eq!(cursor.indent(), 8);
        cursor.skip(2);
        cursor.skip(1);
        // One column of the first tab is left, and the second tab is four columns wide.
        assert_eq!(cursor.indent(), 5);
        cursor.skip(1);
        assert_eq!(cursor.indent(), 4);
    }

    #[test]
    fn should_keep_the_white_space_of_code_at_the_edges_of_the_content_of_a_container() {
        let indented = CodeBlockStyle::Indented;
        assert_eq!(quote_content_range("\n    a  \n", indented), 1..8);
        assert_eq!(quote_content_range("\n  a  \n", indented), 3..4);
        assert_eq!(quote_content_range("  \n", indented), 3..3);
        assert_eq!(quote_content_range("    a\n", CodeBlockStyle::Backticks), 4..5);
    }
}
