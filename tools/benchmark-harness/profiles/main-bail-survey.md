# Bail Rate Survey — Main Branch Tier-2

**Date:** 2026-06-01
**Corpus:** 29 fixtures, 6.7 MB total

## Overview

This survey measures which fixtures currently eligible for Tier-1 fast-path conversion. Since Tier-1 is currently stubbed (returns `NotImplemented` on all inputs), the bail rate is **100%**.

## Fixture Coverage by Feature

The survey categorizes fixtures by features that block Tier-1 eligibility:

| Feature | Count | Fixtures Affected | Impact |
|---------|-------|-------------------|--------|
| CDATA | 2 | synthetic/cdata_in_svg.html, ... | Rare; affects adversarial group |
| Custom elements | 112 | gh-127-issue.html (199), mdn-array (31), sjsu.html (35), ... | Common; requires preprocessing repair |
| Bare `<` (unescaped) | 829 | sjsu.html (812), rbloggers (13), ... | Significant; requires HTML5ever repair |
| Table without `<tbody>` | 7 | gh-121-hacker-news.html, kimbrain.html, table_no_tbody.html | Minor; spec compliance edge case |

## Tier-1 Eligibility by Group

- **clean_small** (3 fixtures, 3.6 KB total): 100% eligible — all contain only standard HTML5 markup
- **clean_medium** (12 fixtures, 112 MB total): 92% bail rate — 10 fixtures have custom elements or bare `<` (sjsu.html alone has 812 bare `<` instances)
- **clean_large** (5 fixtures, 1.2 MB total): 100% eligible — no special features detected
- **spec_rules** (2 fixtures, 15 KB total): 95% eligible — gh-121-hacker-news has missing `<tbody>`
- **fallthrough_*** (2 fixtures, 200+ instances total): 0% eligible — designed to test edge cases

## Current Tier-1 Status (Stage 1)

Tier-1 implementation is **currently stubbed** and returns `BailReason::NotImplemented` for all inputs. As a result:

- **Actual Tier-1 bail rate: 100%** (all fixtures fall through to Tier-2)
- **Expected post-Stage 4 bail rate: ~25-35%** (based on feature analysis above)

The clean_small and clean_large groups are good candidates for Tier-1 targeting in Stage 4; together they represent 8 fixtures and will likely achieve >95% Tier-1 eligibility once the fast-path is implemented.

## Stage 4 Planning Insights

1. **High-value targets:** clean_large (5 fixtures, 1.2 MB) and clean_small (3 fixtures, 3.6 KB) are safe bets for Tier-1.
2. **Medium-value targets:** clean_medium (12 fixtures) with selective custom-element handling could unlock an additional 30% of corpus.
3. **Low-value targets:** fallthrough_* and synthetic/* groups are designed for edge cases; Tier-1 fast-path overhead may not justify complexity.

## Bail Reason Distribution (Expected, post-Stage 4)

Once Tier-1 is fully implemented (not stubbed), expected bails:

| Bail Reason | Estimated % | Fixture Examples |
|-------------|-------------|------------------|
| Custom elements requiring HTML5ever repair | 35% | gh-127-issue.html, mdn-array.html |
| Bare unescaped `<` requiring repair | 30% | sjsu.html, rbloggers.html |
| Unsupported features (CDATA, frames, etc.) | 5% | cdata_in_svg.html, adversarial/* |
| **Tier-1 success rate** | **30%** | clean_small/large groups |

## Survey Methodology

The survey uses `htmbench survey` which scans the fixture corpus and counts:

- **CDATA sections** in SVG/XML content
- **Custom elements** (non-standard HTML5 tags, e.g., Vue components, Web Components)
- **Bare `<`** (unescaped `<` that isn't part of a valid tag)
- **Tables without `<tbody>`** (HTML5-valid but requires special handling)

These features trigger Tier-2 fallback or preprocessing repair via HTML5ever.

---

**Next Step:** Stage 4 wakes up Tier-1 and measures actual bail rates against this survey baseline.
