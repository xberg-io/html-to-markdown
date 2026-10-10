//! Indented code line classification for final Markdown cleanup. ~keep

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

enum Container {
    Quote,
    Item(usize),
}

struct LineCursor<'a> {
    rest: &'a str,
    column: usize,
    spare: usize,
}

impl LineCursor<'_> {
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

    fn advance(&mut self, bytes: usize) {
        self.rest = &self.rest[bytes..];
        self.column += bytes;
    }

    fn skip_quote_marker(&mut self) {
        self.advance(1);
        if self.indent() > 0 {
            self.skip(1);
        }
    }

    fn is_blank(&self) -> bool {
        self.rest.trim_matches([' ', '\t']).is_empty()
    }

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

/// ~keep A line is code when it is indented by four columns or more past the block quotes and the
/// ~keep list items it is in, and it does not continue a paragraph. The blank lines between two
/// ~keep lines of one code block belong to the block. The indentation alone does not say this:
/// ~keep the third level of a list is indented by four columns too.
pub(super) fn indented_code_lines(output: &str) -> Vec<bool> {
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

#[cfg(test)]
mod tests {
    use super::indented_code_lines;

    #[test]
    fn should_preserve_only_blank_lines_inside_indented_code() {
        for (markdown, expected) in [
            ("- t\n\n<!-- -->\n\n    a\n\n\n    b\n", "000011110"),
            ("- t\n\n    a\n\n\n    b\n", "0000000"),
            ("- t\n\n      a\n\n\n      b\n", "0011110"),
            (">     a\n>\n>\n>     b\n", "11110"),
            ("paragraph\n    continuation", "00"),
            ("7  x\n\n     c", "001"),
        ] {
            let actual: String = indented_code_lines(markdown)
                .into_iter()
                .map(|is_code| if is_code { '1' } else { '0' })
                .collect();
            assert_eq!(actual, expected, "{markdown:?}");
        }
    }
}
