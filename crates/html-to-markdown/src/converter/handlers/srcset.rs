//! Candidate splitting for `srcset`-shaped attribute values.

/// Split a `srcset`-style list into its candidates, each a URL and its descriptor.
///
/// ~keep Follows the HTML "parse a srcset attribute" steps: a candidate URL is a run of
/// ~keep non-whitespace (so a comma inside a URL, as in a `data:` URL, stays in it), trailing
/// ~keep commas on the URL end the candidate, and otherwise the descriptor runs to the next
/// ~keep comma outside parentheses, as the descriptor tokenizer's "in parens" state reads it.
/// ~keep Whitespace is the spec's five ASCII characters only, so a no-break space is text.
pub(super) fn srcset_candidates(list: &str) -> impl Iterator<Item = (&str, &str)> {
    let mut rest = list;
    std::iter::from_fn(move || {
        rest = rest.trim_start_matches(|c: char| c.is_ascii_whitespace() || c == ',');
        if rest.is_empty() {
            return None;
        }
        let url_end = rest.find(|c: char| c.is_ascii_whitespace()).unwrap_or(rest.len());
        let (candidate_url, after_url) = rest.split_at(url_end);
        if candidate_url.ends_with(',') {
            rest = after_url;
            return Some((candidate_url.trim_end_matches(','), ""));
        }
        let (descriptor, remainder) = split_descriptor(after_url);
        rest = remainder;
        Some((candidate_url, descriptor))
    })
}

/// Split the text after a candidate URL into its descriptor and the rest of the list, which
/// starts at the comma that ended the descriptor.
///
/// ~keep Parentheses do not nest: the tokenizer leaves its "in parens" state at the first `)`,
/// ~keep and a `(` still open at the end of the list keeps its trailing whitespace, which the
/// ~keep tokenizer reads as descriptor text there.
fn split_descriptor(after_url: &str) -> (&str, &str) {
    let is_space = |c: char| c.is_ascii_whitespace();
    let after_url = after_url.trim_start_matches(is_space);
    let mut in_parens = false;
    for (index, byte) in after_url.bytes().enumerate() {
        match byte {
            b'(' => in_parens = true,
            b')' => in_parens = false,
            b',' if !in_parens => return (after_url[..index].trim_end_matches(is_space), &after_url[index..]),
            _ => {}
        }
    }
    let descriptor = if in_parens {
        after_url
    } else {
        after_url.trim_end_matches(is_space)
    };
    (descriptor, "")
}

/// A candidate's descriptor as the spec's descriptor parser reads it, before any `sizes` value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum Descriptor {
    /// A `w` descriptor: the image's width in pixels.
    Width(f64),
    /// An `x` descriptor, or 1 for a candidate with no descriptor, as the spec normalises it.
    Density(f64),
}

/// Run the HTML "parse a srcset attribute" descriptor parser over one candidate's descriptor.
///
/// ~keep `None` is the spec's parse error, which drops the candidate: an unknown token, a number
/// ~keep outside the spec's grammar (`+2x`, `infx`, `1.5w`), a zero width or height, a negative
/// ~keep density, a second descriptor of a kind already set, a density next to a width or height,
/// ~keep or an `h` without a `w`. A valid `h` is otherwise ignored, as the spec ignores it.
pub(super) fn parse_descriptor(descriptor: &str) -> Option<Descriptor> {
    let mut width = None;
    let mut density = None;
    let mut has_height = false;
    for token in descriptor_tokens(descriptor) {
        if let Some(number) = token.strip_suffix('w') {
            if width.is_some() || density.is_some() {
                return None;
            }
            width = Some(positive_integer(number)?);
        } else if let Some(number) = token.strip_suffix('x') {
            if width.is_some() || density.is_some() || has_height {
                return None;
            }
            density = Some(non_negative_float(number)?);
        } else if let Some(number) = token.strip_suffix('h') {
            if has_height || density.is_some() {
                return None;
            }
            positive_integer(number)?;
            has_height = true;
        } else {
            return None;
        }
    }
    if has_height && width.is_none() {
        return None;
    }
    Some(match width {
        Some(width) => Descriptor::Width(width),
        None => Descriptor::Density(density.unwrap_or(1.0)),
    })
}

/// Split a descriptor into the spec tokenizer's tokens: ASCII whitespace outside parentheses
/// separates them, and a `(` still open at the end keeps the rest, trailing whitespace included.
fn descriptor_tokens(descriptor: &str) -> impl Iterator<Item = &str> {
    let mut rest = descriptor;
    std::iter::from_fn(move || {
        rest = rest.trim_start_matches(|c: char| c.is_ascii_whitespace());
        if rest.is_empty() {
            return None;
        }
        let mut in_parens = false;
        let end = rest
            .bytes()
            .position(|byte| match byte {
                b'(' => {
                    in_parens = true;
                    false
                }
                b')' => {
                    in_parens = false;
                    false
                }
                _ => !in_parens && byte.is_ascii_whitespace(),
            })
            .unwrap_or(rest.len());
        let (token, remainder) = rest.split_at(end);
        rest = remainder;
        Some(token)
    })
}

fn is_ascii_digits(text: &str) -> bool {
    !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit())
}

/// A "valid non-negative integer" that is not zero, the only kind a `w` or `h` accepts.
fn positive_integer(text: &str) -> Option<f64> {
    if !is_ascii_digits(text) {
        return None;
    }
    text.parse::<f64>().ok().filter(|value| *value > 0.0)
}

/// A "valid floating-point number" that is not negative: an optional `-`, digits with an
/// optional fraction (or a bare fraction), then an optional exponent. A value that rounds past
/// the largest finite double is an error, as the spec's parsing rules make it.
///
/// ~keep Rust's float grammar is the spec's from the exponent on, but before it Rust also accepts
/// ~keep a leading `+`, `inf`, `NaN` and `1.`, so only the part before the exponent is checked.
fn non_negative_float(text: &str) -> Option<f64> {
    let unsigned = text.strip_prefix('-').unwrap_or(text);
    let mantissa = unsigned
        .split_once(['e', 'E'])
        .map_or(unsigned, |(mantissa, _)| mantissa);
    let mantissa_is_valid = match mantissa.split_once('.') {
        Some((whole, fraction)) => (whole.is_empty() || is_ascii_digits(whole)) && is_ascii_digits(fraction),
        None => is_ascii_digits(mantissa),
    };
    if !mantissa_is_valid {
        return None;
    }
    text.parse::<f64>()
        .ok()
        .filter(|value| value.is_finite() && *value >= 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_spec_space(c: char) -> bool {
        matches!(c, '\t' | '\n' | '\x0C' | '\r' | ' ')
    }

    /// The HTML "parse a srcset attribute" steps up to the descriptor parser, written from the
    /// spec text one code point at a time: each candidate's URL and its descriptor tokens.
    fn spec_srcset(input: &str) -> Vec<(String, Vec<String>)> {
        let input: Vec<char> = input.chars().collect();
        let mut position = 0;
        let mut candidates = Vec::new();
        loop {
            while input.get(position).is_some_and(|&c| is_spec_space(c) || c == ',') {
                position += 1;
            }
            if position >= input.len() {
                return candidates;
            }
            let start = position;
            while input.get(position).is_some_and(|&c| !is_spec_space(c)) {
                position += 1;
            }
            let url: String = input[start..position].iter().collect();
            if url.ends_with(',') {
                candidates.push((url.trim_end_matches(',').to_owned(), Vec::new()));
                continue;
            }
            while input.get(position).is_some_and(|&c| is_spec_space(c)) {
                position += 1;
            }
            candidates.push((url, spec_descriptor_tokens(&input, &mut position)));
        }
    }

    #[derive(Clone, Copy)]
    enum State {
        InDescriptor,
        InParens,
        AfterDescriptor,
    }

    /// The spec's descriptor tokenizer from `position`, which it leaves after the comma that
    /// ended the candidate (or at the end of the input).
    fn spec_descriptor_tokens(input: &[char], position: &mut usize) -> Vec<String> {
        let mut descriptors = Vec::new();
        let mut current = String::new();
        let mut state = State::InDescriptor;
        loop {
            let c = input.get(*position).copied();
            match (state, c) {
                (State::InDescriptor, Some(c)) if is_spec_space(c) => {
                    if !current.is_empty() {
                        descriptors.push(std::mem::take(&mut current));
                        state = State::AfterDescriptor;
                    }
                }
                (State::InDescriptor, Some(',')) => {
                    *position += 1;
                    break;
                }
                (State::InDescriptor, Some('(')) => {
                    current.push('(');
                    state = State::InParens;
                }
                (State::InDescriptor, None) => break,
                (State::InParens, Some(')')) => {
                    current.push(')');
                    state = State::InDescriptor;
                }
                (State::InParens, None) => {
                    descriptors.push(std::mem::take(&mut current));
                    break;
                }
                (State::InDescriptor | State::InParens, Some(c)) => current.push(c),
                (State::AfterDescriptor, Some(c)) if is_spec_space(c) => {}
                (State::AfterDescriptor, None) => break,
                (State::AfterDescriptor, Some(_)) => {
                    state = State::InDescriptor;
                    continue;
                }
            }
            *position += 1;
        }
        if !current.is_empty() {
            descriptors.push(current);
        }
        descriptors
    }

    /// Where `srcset_candidates` and the spec steps disagree on `list`, as a message.
    fn disagreement(list: &str) -> Option<String> {
        let ours: Vec<(String, Vec<String>)> = srcset_candidates(list)
            .map(|(url, descriptor)| {
                let tokens = descriptor_tokens(descriptor).map(str::to_owned).collect();
                (url.to_owned(), tokens)
            })
            .collect();
        let spec = spec_srcset(list);
        (ours != spec).then(|| format!("{list:?}: ours {ours:?}, spec {spec:?}"))
    }

    #[test]
    fn the_descriptor_parser_follows_the_spec_steps() {
        assert_eq!(parse_descriptor(""), Some(Descriptor::Density(1.0)));
        assert_eq!(parse_descriptor("1.5x"), Some(Descriptor::Density(1.5)));
        assert_eq!(parse_descriptor("-0x"), Some(Descriptor::Density(0.0)));
        assert_eq!(parse_descriptor("-.5e-0x"), None);
        assert_eq!(parse_descriptor("1E+1x"), Some(Descriptor::Density(10.0)));
        assert_eq!(parse_descriptor("5e-1x"), Some(Descriptor::Density(0.5)));
        assert_eq!(parse_descriptor("800w"), Some(Descriptor::Width(800.0)));
        assert_eq!(parse_descriptor("007w"), Some(Descriptor::Width(7.0)));
        assert_eq!(parse_descriptor("800w 600h"), Some(Descriptor::Width(800.0)));
        for invalid in [
            "x",
            "w",
            "2",
            "2X",
            "2W",
            "1e",
            "1e+x",
            "1e1.5x",
            "1e+-1x",
            "1einfx",
            "e1x",
            "1.x",
            ".x",
            "--1x",
            "0x1x",
            "1_0w",
            "\u{661}x",
            "800w 900w",
            "600h 700h 800w",
            "600h",
            "1x 600h",
            "600h 1x",
            "(1x)",
            "1x (a)",
        ] {
            assert_eq!(parse_descriptor(invalid), None, "{invalid:?}");
        }
    }

    #[test]
    fn the_spec_reference_keeps_a_parenthesised_comma_in_one_candidate() {
        assert_eq!(
            spec_srcset("a.png 1x (x, y), b.png 2x"),
            vec![
                ("a.png".to_owned(), vec!["1x".to_owned(), "(x, y)".to_owned()]),
                ("b.png".to_owned(), vec!["2x".to_owned()]),
            ]
        );
    }

    #[test]
    fn a_parenthesised_comma_stays_in_the_descriptor() {
        let candidates: Vec<_> = srcset_candidates("a.png (x, b.png 3x ), c.png 2x").collect();
        assert_eq!(candidates, vec![("a.png", "(x, b.png 3x )"), ("c.png", "2x")]);
    }

    #[test]
    fn srcset_candidates_match_the_spec_steps_on_real_lists() {
        let corpus = [
            "a.png 1x (x, y), b.png 2x",
            "a.png (x, b.png 3x ), c.png 2x",
            "small.jpg 300w, large.jpg 800w",
            "data:image/png;base64,AA== 1x, b.png 2x",
            "a.png?x=1,2 2x, b.png 1x",
            "a.png,b.png",
            "a.png,,, b.png 2x,,",
            " , b.png",
            "a.png (x, y",
            "a.png 1x (x, y  ",
            "a.png x), b.png",
            "a.png ((x, y), z), b.png",
            "a.png 1x(x, y)z, b.png",
            "hero.jpg 100w (max-width: 600px, 50vw), big.jpg 2x",
            "a.png\t1x,\r\nb.png\x0C2x",
            "\u{e9}.png 1x (\u{e9}, \u{e9}), b.png",
            "a.png \u{a0}1x\u{a0}, b.png",
            "a\u{a0}b.png 2x, c.png\u{a0}1x",
            "",
            ",",
        ];
        let wrong: Vec<String> = corpus.into_iter().filter_map(disagreement).collect();
        assert!(wrong.is_empty(), "{wrong:#?}");
    }

    #[test]
    fn srcset_candidates_match_the_spec_steps_on_every_short_list() {
        const ALPHABET: [char; 8] = ['a', 'x', ',', ' ', '\t', '(', ')', '\u{e9}'];
        let mut wrong = Vec::new();
        let mut checked = 0usize;
        let mut list = String::new();
        for length in 0..=6u32 {
            for mut index in 0..ALPHABET.len().pow(length) {
                list.clear();
                for _ in 0..length {
                    list.push(ALPHABET[index % ALPHABET.len()]);
                    index /= ALPHABET.len();
                }
                checked += 1;
                wrong.extend(disagreement(&list));
            }
        }
        assert_eq!(checked, 299_593, "every list up to six characters");
        assert!(
            wrong.is_empty(),
            "{} disagreements, first: {:?}",
            wrong.len(),
            wrong.first()
        );
    }
}
