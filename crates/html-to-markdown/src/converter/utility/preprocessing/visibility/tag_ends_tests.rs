use super::{HiddenContent, TagEnds, find_tag_end, unwrap_kept_inert_elements};

#[test]
fn should_step_over_a_run_of_raw_text_tag_starts_with_no_end_in_linear_time() {
    // ~keep The quote before the last `>` has no partner, so no tag of the run ends. The scan
    // ~keep for the end of a raw-text element reads the rest of the input for each start.
    let timed = |name: &str| {
        let page = format!("{}\">", format!("<{name} ").repeat(1_600_000 / (name.len() + 2)));
        let started = std::time::Instant::now();
        let unwrapped = unwrap_kept_inert_elements(&page, HiddenContent::All, &[]);
        assert_eq!(unwrapped, page, "no template and no noscript: nothing is removed");
        started.elapsed()
    };
    let plain = timed("a");
    for name in ["script", "style"] {
        let raw_text = timed(name);
        // ~keep A quadratic scan reads 160 GB here. The bound is a multiple of a page of the
        // ~keep same size with an ordinary tag, so a loaded host moves both sides.
        assert!(
            raw_text < plain * 50 + std::time::Duration::from_secs(5),
            "{name} took {raw_text:?}, an ordinary tag took {plain:?}"
        );
    }
}

/// The pieces of the generated pages: the text, and for a tag of the two elements its
/// element and whether it is a start tag.
type Piece = (&'static str, Option<(usize, bool)>);
const PIECES: [Piece; 10] = [
    ("<head>", None),
    ("</head>", None),
    ("<title>", None),
    ("</title>", None),
    ("<p>", None),
    ("<template>", Some((0, true))),
    (r#"<TEMPLATE shadowrootmode="open">"#, Some((0, true))),
    ("</template>", Some((0, false))),
    (r#"<noscript class="a">"#, Some((1, true))),
    ("</NOSCRIPT>", Some((1, false))),
];

/// Which pieces of `page` the scan removed. A comment with the position follows each piece.
///
/// ~keep The scan writes `</body>` for the removed tag that started the body: once, and for
/// ~keep the first tag that it removes.
fn removed_pieces(pieces: &[Piece], page: &str, choice: HiddenContent) -> Vec<bool> {
    let unwrapped = unwrap_kept_inert_elements(page, choice, &[]);
    if let Some(at) = unwrapped.find("</body>") {
        assert!(page.starts_with(&unwrapped[..at]), "{page} {choice:?}");
        assert_eq!(unwrapped.matches("</body>").count(), 1, "{page} {choice:?}");
    }
    let mut rest: &str = &unwrapped;
    let removed = pieces
        .iter()
        .enumerate()
        .map(|(position, (text, _))| {
            let kept = rest.strip_prefix(text);
            rest = kept
                .or_else(|| rest.strip_prefix("</body>"))
                .unwrap_or(rest)
                .strip_prefix(&format!("<!--{position}-->"))
                .unwrap_or_else(|| panic!("{page} {choice:?}: {unwrapped} is not the page without some tags"));
            kept.is_none()
        })
        .collect();
    assert_eq!(rest, "", "{page} {choice:?}");
    removed
}

/// Check one page under one choice, and give the count of its elements with two tags and
/// the count of those whose tags the scan removed.
///
/// ~keep The pairs come from a count of start and end tags for each element, which knows
/// ~keep nothing of the head. A page with a tag of the head or with a `<title>` can have a
/// ~keep head, with or without the `<head>` tag.
fn check_the_pairs(pieces: &[Piece], page: &str, choice: HiddenContent) -> (usize, usize) {
    let has_head = pieces
        .iter()
        .any(|(text, _)| matches!(*text, "<head>" | "</head>" | "<title>"));
    let removed = removed_pieces(pieces, page, choice);
    let mut open: [Vec<usize>; 2] = [Vec::new(), Vec::new()];
    let (mut pairs, mut removed_pairs) = (0, 0);
    for (position, (text, tag)) in pieces.iter().enumerate() {
        let Some((element, is_start)) = *tag else {
            assert!(!removed[position], "{page} {choice:?}: {text} is removed");
            continue;
        };
        if is_start {
            open[element].push(position);
            let kept_by_choice = choice == HiddenContent::All || text.contains("shadowrootmode");
            assert!(kept_by_choice || !removed[position], "{page} {choice:?}: {text}");
            assert!(
                has_head || removed[position] == kept_by_choice,
                "{page} {choice:?}: {text}"
            );
        } else if let Some(start) = open[element].pop() {
            pairs += 1;
            removed_pairs += usize::from(removed[position]);
            assert_eq!(
                removed[start], removed[position],
                "{page} {choice:?}: the tags at {start} and {position} are one element"
            );
        } else {
            assert!(!removed[position], "{page} {choice:?}: {text} closes nothing");
        }
    }
    (pairs, removed_pairs)
}

#[test]
fn should_remove_the_two_tags_of_an_element_together_or_not_at_all() {
    // ~keep Every page of up to five pieces.
    let (mut pairs, mut removed_pairs) = (0, 0);
    for len in 0..=5u32 {
        for mut code in 0..PIECES.len().pow(len) {
            let mut pieces = Vec::new();
            let mut page = String::new();
            for position in 0..len {
                let piece = PIECES[code % PIECES.len()];
                code /= PIECES.len();
                pieces.push(piece);
                page.extend([piece.0, "<!--", &position.to_string(), "-->"]);
            }
            assert_eq!(unwrap_kept_inert_elements(&page, HiddenContent::Drop, &[]), page);
            for choice in [HiddenContent::Reachable, HiddenContent::All] {
                let (in_page, removed_in_page) = check_the_pairs(&pieces, &page, choice);
                pairs += in_page;
                removed_pairs += removed_in_page;
            }
        }
    }
    assert!(
        pairs > 20_000 && removed_pairs > 5_000,
        "{pairs} pairs, {removed_pairs} removed"
    );
}

#[test]
fn should_give_the_tag_end_that_a_scan_from_each_byte_gives() {
    // ~keep Every page of up to eight bytes over the bytes that the scan tells apart.
    let alphabet = [b'"', b'\'', b'>', b'a'];
    let mut compared = 0;
    for len in 0..=8u32 {
        for mut code in 0..4usize.pow(len) {
            let page: Vec<u8> = (0..len)
                .map(|_| {
                    let byte = alphabet[code % 4];
                    code /= 4;
                    byte
                })
                .collect();
            let table = TagEnds::new(&page);
            for from in 0..=page.len() + 1 {
                assert_eq!(table.tag_end(from), find_tag_end(&page, from), "{page:?} from {from}");
                compared += 1;
            }
        }
    }
    assert!(compared > 500_000, "compared only {compared} scans");
}
