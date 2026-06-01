//! HTML5 optional-close-tag transition rules consulted by the Tier-1 scanner
//! on every tag open.
//!
//! See <https://html.spec.whatwg.org/multipage/syntax.html#optional-tags>.

use crate::converter::tier1::tags::{OptionalCloseRule, TagKind, TagSpec};

/// Given the spec of a newly-opened tag and the spec at the top of the open-tag
/// stack, return `true` if the open tag should be implicitly closed before the
/// new tag is pushed.
///
/// The scanner consults this on every tag open. Returning `true` triggers the
/// scanner to:
///
/// 1. Emit the closing markdown for the open tag (e.g. newline for `<li>`)
/// 2. Pop the stack
/// 3. Re-test with the new top of the stack until `should_close_for_new_tag`
///    returns `false`
///
/// Then push the new tag.
pub fn should_close_for_new_tag(open: &TagSpec, new: &TagSpec) -> bool {
    match open.optional_close {
        None => false,

        // `<li>` closes any open `<li>` (same kind).
        Some(OptionalCloseRule::CloseSameKind) => {
            std::mem::discriminant(&open.kind) == std::mem::discriminant(&new.kind)
        }

        // `<p>` closes when a block-level element opens next.
        Some(OptionalCloseRule::CloseOnBlockChild) => new.is_block,

        // `<dt>`/`<dd>` close any open `<dt>`/`<dd>`.
        Some(OptionalCloseRule::CloseSiblingDtDd) => {
            matches!(new.kind, TagKind::DefinitionTerm | TagKind::DefinitionDescription)
        }

        // `<tr>` closes any open `<tr>`.
        Some(OptionalCloseRule::CloseTableRow) => matches!(new.kind, TagKind::TableRow),

        // `<td>`/`<th>` close any open `<td>`/`<th>`.
        Some(OptionalCloseRule::CloseTableCell) => matches!(new.kind, TagKind::TableCell { .. }),

        // `<option>` closes on another `<option>` or `<optgroup>`.
        // (Forms bail in M3c so this path is unreachable in practice.)
        Some(OptionalCloseRule::CloseOption) => std::mem::discriminant(&open.kind) == std::mem::discriminant(&new.kind),

        // `ImplicitTbody` is not a close rule — the scanner handles it
        // separately by synthesising an implicit open.
        Some(OptionalCloseRule::ImplicitTbody) => false,
    }
}

/// Given the new tag and the current top of the stack, return
/// `Some(implicit-open-tag-name-bytes)` if the scanner should synthesise an
/// implicit open before pushing the new tag.
///
/// Currently only the `<tr>`-inside-`<table>` without `<tbody>` case applies.
/// Returns `None` if no implicit open is needed.
///
/// Note: Tier-1 bails on tables (M3c), so this will not fire until tables are
/// unblocked in a future milestone.
pub fn implicit_open_before(top_of_stack: Option<&TagSpec>, new: &TagSpec) -> Option<&'static [u8]> {
    match (top_of_stack.map(|s| s.kind), new.kind) {
        (Some(TagKind::Table), TagKind::TableRow) => Some(b"tbody"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converter::tier1::tags;

    #[test]
    fn li_closes_li() {
        let li = tags::lookup(b"li").unwrap();
        assert!(should_close_for_new_tag(li, li));
    }

    #[test]
    fn li_does_not_close_on_text_only() {
        // Block (div) opening: li does not have CloseOnBlockChild — it only
        // closes on same kind.
        let li = tags::lookup(b"li").unwrap();
        let div = tags::lookup(b"div").unwrap();
        assert!(!should_close_for_new_tag(li, div));
    }

    #[test]
    fn p_closes_on_div() {
        let p = tags::lookup(b"p").unwrap();
        let div = tags::lookup(b"div").unwrap();
        assert!(should_close_for_new_tag(p, div));
    }

    #[test]
    fn p_does_not_close_on_inline_span() {
        let p = tags::lookup(b"p").unwrap();
        let span = tags::lookup(b"span").unwrap();
        assert!(!should_close_for_new_tag(p, span));
    }

    #[test]
    fn p_closes_on_another_p() {
        let p = tags::lookup(b"p").unwrap();
        // <p> is block-level so it triggers CloseOnBlockChild.
        assert!(should_close_for_new_tag(p, p));
    }

    #[test]
    fn dt_closes_on_dd() {
        let dt = tags::lookup(b"dt").unwrap();
        let dd = tags::lookup(b"dd").unwrap();
        assert!(should_close_for_new_tag(dt, dd));
    }

    #[test]
    fn dd_closes_on_dt() {
        let dd = tags::lookup(b"dd").unwrap();
        let dt = tags::lookup(b"dt").unwrap();
        assert!(should_close_for_new_tag(dd, dt));
    }

    #[test]
    fn div_has_no_optional_close() {
        let div = tags::lookup(b"div").unwrap();
        let p = tags::lookup(b"p").unwrap();
        assert!(!should_close_for_new_tag(div, p));
    }

    #[test]
    fn implicit_open_tr_inside_table() {
        let table = tags::lookup(b"table").unwrap();
        let tr = tags::lookup(b"tr").unwrap();
        assert_eq!(implicit_open_before(Some(table), tr), Some(b"tbody" as &[u8]));
    }

    #[test]
    fn no_implicit_open_for_div() {
        let div = tags::lookup(b"div").unwrap();
        let p = tags::lookup(b"p").unwrap();
        assert_eq!(implicit_open_before(Some(div), p), None);
    }

    #[test]
    fn no_implicit_open_when_stack_empty() {
        let tr = tags::lookup(b"tr").unwrap();
        assert_eq!(implicit_open_before(None, tr), None);
    }
}
