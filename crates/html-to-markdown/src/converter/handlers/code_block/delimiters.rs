/// Compute the length of the longest consecutive run of `marker` in `content`.
pub(super) fn longest_consecutive_run(content: &str, marker: char) -> usize {
    content
        .chars()
        .fold((0usize, 0usize), |(max, current), c| {
            if c == marker {
                let next = current + 1;
                (max.max(next), next)
            } else {
                (max, 0)
            }
        })
        .0
}

/// Smallest backtick-run length (starting at 1) that does not occur as a run inside `content`.
///
/// CommonMark closes an inline code span at the next backtick string of the *same* length as
/// the opening delimiter (6.1) — a longer or shorter run never matches. So the delimiter only
/// needs to avoid colliding with a run length that actually appears in `content`; it does not
/// need to exceed the longest run (unlike a fenced block, whose closing rule matches on *any*
/// run at least as long as the fence). Picking `longest_run + 1` unconditionally over-escapes:
/// content `` `` `` (a single length-2 run, no length-1 run) is valid with a single backtick.
pub(super) fn min_safe_code_span_delimiter_length(content: &str) -> usize {
    let mut run_lengths = std::collections::HashSet::new();
    let mut current = 0usize;
    for c in content.chars() {
        if c == '`' {
            current += 1;
        } else {
            if current > 0 {
                run_lengths.insert(current);
            }
            current = 0;
        }
    }
    if current > 0 {
        run_lengths.insert(current);
    }

    let mut candidate = 1usize;
    while run_lengths.contains(&candidate) {
        candidate += 1;
    }
    candidate
}
