# Main Tier-2 Baseline Profile

**Fixture:** mdream/wikipedia-small.html (166 KB input)
**Date:** 2026-06-01
**Iterations:** 50
**Profile Method:** Code inspection + benchmark timing

## Overview

The Tier-2 (mdream/wikipedia-small.html) baseline runs at ~9.7ms for 166 KB input (16.3 MB/s), which is significantly slower than the Stage-1 baseline (4.2 ms, 37.7 MB/s). This stage captures the main profile for future optimization work.

## Hot Call Stack (from code inspection)

The primary hot path through conversion is:

```
convert_html_impl()
  → preprocess_html() [multiple passes for normalization]
    - strip_script_and_style_tags()
    - strip_hidden_elements()
    - normalize_bogus_comment_endings()
    - normalize_split_closing_tags()
    - normalize_unclosed_list_items()
  → tl::parse() [HTML5 parser]
  → walk_node() [recursive DOM walker] ← PRIMARY HOT LOOP
    - For each tag in document:
      - tag_name resolution & context lookup
      - visitor hooks (if enabled)
      - handler dispatch (block/inline/special)
      - recursive descent on children
  → post-processing (collapse whitespace, etc.)
```

## Top 3 Suspected Hot Functions

Based on code structure and call frequency:

| Rank | Function | Location | Estimated % | Reason |
|------|----------|----------|-------------|--------|
| 1 | `walk_node()` | converter/main.rs:330 | ~35% | Recursive tree walk called for every DOM node; ~1000+ calls on wikipedia-small |
| 2 | `process_text_node()` | converter/text_node.rs | ~20% | Text normalization & escape handling, called frequently for text content |
| 3 | `handle_*` (dispatch) | converter/block/*.rs, inline/*.rs | ~15% | Tag-specific handlers for h1-h6, p, lists, tables, links, images, etc. |

## Preprocessing Breakdown

The preprocessing phase before parsing is substantial:

- **strip_script_and_style_tags()**: Regex-based removal of script/style content (3 passes total on processed HTML)
- **normalize_*** functions: Multiple string replacements for edge cases (bogus comments, JSX-style close tags, unclosed list items)
- **tl::parse()**: HTML5 parsing is delegated to `tl` crate; likely significant but externally optimized

**Estimate:** Preprocessing + parsing: ~30-40% of total time.

## Known Slowness Patterns

1. **Heavy visitor hook overhead** (if enabled): Every element start/end triggers callbacks; no fast path when disabled
2. **No DOM node ID caching** for common lookups (e.g., tag_info, dom_ctx lookups)
3. **String allocations in walk_node()** for tag_name Cow and visitor context
4. **Metadata/structure collection** adds per-node overhead even when unused
5. **No parallel preprocessing**: All string passes are sequential

## Comparison to Stage 1 Baseline

Stage 1 (Tier-1 overlay) achieves 4.2 ms on the same fixture.  The ~2.3x slowdown (9.7 vs 4.2 ms) suggests:

- Tier-1 fast-path (direct buffer walk) is ~2-3x faster than full DOM tree walk
- Tier-2 preprocessing+parsing overhead is substantial
- Tier-1 avoids visitor/metadata/structure collection overhead

## Next Optimization Opportunities

(For Stage 7 profile-driven iteration):

1. **Reduce preprocessing passes**: Combine normalizations into single-pass regex or streaming parser
2. **Cache tag metadata**: Avoid repeated lookups in dom_ctx for the same nodes
3. **Lazy visitor invocation**: Only call visitor hooks if actually subscribed
4. **Zero-copy text node processing**: Avoid UTF-8 validation re-work
5. **Parallel fixture benchmarking**: Use rayon to benchmark multiple fixtures concurrently

## Profile Artifacts

- **Sample format**: Code inspection (samply/flamegraph had symbolication issues on this macOS setup)
- **Profile data**: /tmp/h2m-profile.json (96 KB, not symbolicated)
- **Flamegraph status**: Failed to collapse (XML format issue in time-profiler trace)
