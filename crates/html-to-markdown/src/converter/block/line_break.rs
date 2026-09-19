//! Handler for line break elements (br).
//!
//! Converts HTML line break tags to Markdown line breaks using the configured
//! newline style (spaces, backslash, or plain newline).

use crate::converter::main_helpers::{emit_table_cell_break, hard_break_marker, trim_trailing_whitespace};
use crate::options::ConversionOptions;
#[cfg(feature = "visitor")]
use std::borrow::Cow;
use tl::{NodeHandle, Parser};

type Context = crate::converter::Context;
type DomContext = crate::converter::DomContext;

/// Handle line break elements (br).
///
/// Converts to appropriate Markdown line break syntax based on the configured
/// newline style and current context (e.g., in headings).
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle(
    node_handle: &NodeHandle,
    parser: &Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    #[cfg(feature = "visitor")]
    if let Some(ref visitor_handle) = ctx.visitor {
        use crate::visitor::EMPTY_ATTRS;
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);
        let node_ctx = if let Some(tl::Node::Tag(t)) = node_handle.get(parser) {
            NodeContext::with_lazy_attributes(
                NodeType::Br,
                Cow::Borrowed("br"),
                t,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                true,
            )
        } else {
            NodeContext::with_borrowed_attributes(
                NodeType::Br,
                Cow::Borrowed("br"),
                &EMPTY_ATTRS,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                true,
            )
        };
        let visit_result = {
            let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
            visitor.visit_line_break(&node_ctx)
        };
        match visit_result {
            VisitResult::Continue => {}
            VisitResult::Skip => return,
            VisitResult::Custom(custom) => {
                output.push_str(&custom);
                return;
            }
            VisitResult::PreserveHtml => {
                use crate::converter::utility::serialization::serialize_node;
                output.push_str(&serialize_node(node_handle, parser));
                return;
            }
            VisitResult::Error(err) => {
                if ctx.visitor_error.borrow().is_none() {
                    *ctx.visitor_error.borrow_mut() = Some(err);
                }
                return;
            }
        }
    }

    if ctx.in_heading {
        // ~keep A single-line ATX heading cannot carry a hard break at all, so any marker
        // ~keep here is inherently lossy. A single space is the only choice that is
        // ~keep round-trip stable: a renderer's own HTML whitespace collapsing already
        // ~keep reduces a run of literal spaces to one on the next parse, so a
        // ~keep two-space marker here only survives the FIRST conversion before
        // ~keep collapsing on the second, and never reaches a fixpoint. Matches
        // ~keep Tier-1: `tier1/scanner.rs`'s `TagKind::LineBreak` handling emits the same
        // ~keep "  \n" marker regardless of heading context, but `close_heading` then
        // ~keep folds every whitespace run (including that marker) in the finished
        // ~keep heading body down to one space.
        trim_trailing_whitespace(output);
        output.push(' ');
    } else if ctx.in_table_cell && ctx.in_code && !ctx.in_code_block {
        // ~keep Neither a code SPAN nor a table cell can carry a hard break on its own
        // ~keep (see the two ~keep blocks this combines, immediately below and at the
        // ~keep plain `in_table_cell` arm): fold straight to a single space rather than
        // ~keep letting the split path below run, which would break the span in two and
        // ~keep leave the second half on its own physical line -- corrupting the cell's
        // ~keep pipe-row syntax exactly as a raw newline would (issue #455's rule, now
        // ~keep also covering the `<br>`-driven case rather than only a literal source
        // ~keep newline). Not `emit_table_cell_break`: a literal `<br>` in HTML is not
        // ~keep valid content inside a code span regardless of `br_in_tables`, so that
        // ~keep option is never consulted here either.
        trim_trailing_whitespace(output);
        output.push(' ');
    } else if ctx.in_code_block {
        // ~keep A `<pre>` code BLOCK reproduces its content literally, line structure
        // ~keep included: a `<br>` here is real content, so the byte pushed is a genuine
        // ~keep `\n`, not a marker -- `newline_style` is never consulted (issue #487).
        output.push('\n');
    } else if ctx.in_code {
        // ~keep A code SPAN's content is otherwise reproduced literally too, but unlike a
        // ~keep block it has no interior line structure of its own to preserve: `<br>` is
        // ~keep a DOM-level split point, not span content. Push a plain '\n' as an
        // ~keep internal-only split marker rather than syntax -- `format_inline_code`
        // ~keep (`handlers/code_block.rs`) and `handle_kbd_samp` (`inline/code.rs`) later
        // ~keep split on it, rendering each half as its own backtick span joined by the
        // ~keep configured `newline_style` marker OUTSIDE the backticks, where it is
        // ~keep syntax (issue #487). This byte can only have come from a real `<br>`: a
        // ~keep source text node's own literal line ending is folded to a space before
        // ~keep it ever reaches this buffer (`text_node.rs`'s `in_code && !in_code_block`
        // ~keep branch), so nothing else can leave a bare '\n' here for this split to
        // ~keep misfire on.
        output.push('\n');
    } else if ctx.in_table_cell || ctx.in_layout_cell {
        // ~keep Shared with div/p continuations inside a cell (issue #453, #454): a cell
        // ~keep cannot contain a hard line break, so newline_style is never consulted and
        // ~keep source whitespace before the <br> is trimmed rather than leaked. A layout
        // ~keep cell is one list item's line and already sends its div/p continuations
        // ~keep through this rule (issue #470); a hard-break marker there ended the item.
        // ~keep An empty buffer here is an inline wrapper's scratch buffer, not the cell's
        // ~keep own: `<td>A<em><br></em>B</td>` saw the continuation rule suppress its space
        // ~keep against `<em>`'s fresh `String`, and the words joined (issue #504). The one
        // ~keep space the `<br>` stands for is pushed there; a cell's own buffer is trimmed
        // ~keep by `render_cell_text`/`append_layout_cell_text`, so a genuinely leading
        // ~keep `<br>` still contributes nothing. A literal `<br>` under `br_in_tables` is
        // ~keep unchanged.
        if output.is_empty() && !options.br_in_tables {
            output.push(' ');
        } else {
            emit_table_cell_break(output, options.br_in_tables);
        }
    } else if ctx.in_link {
        // ~keep #497: inside a link label a `<br>` with nothing before it is still real
        // ~keep content -- `B<a href="H"><br>A</a>` renders as B, a line break, then A, and
        // ~keep `B[  \n A](H)` (without the space) re-parses to exactly that. The
        // ~keep `block_content_start` test below cannot see this case: a label is built into a
        // ~keep fresh local `String` whose length starts at 0 while `block_content_start` still
        // ~keep refers to the ENCLOSING block's buffer, so "nothing on this line yet" comes out
        // ~keep true or false by coincidence. Whether a break at either edge of the label
        // ~keep survives is `normalize_link_label`'s decision, not this one's.
        output.push_str(hard_break_marker(options.newline_style));
    } else if output.len() == ctx.block_content_start {
        // ~keep A <br> with nothing before it on the current line has no prior line to
        // ~keep break: emitting a style marker here would leave a leading artifact instead
        // ~keep of being invisible. A leading run of <br> therefore collapses to no output
        // ~keep at all (issue #464), rather than the previous "swallow every break after the
        // ~keep first" check (`output.ends_with('\n')`), which also matched — and silently
        // ~keep collapsed — a run of *consecutive* breaks with real content before them.
        // ~keep Unguarded by `ctx.in_paragraph` (unlike `text_node.rs`'s identical-looking
        // ~keep check): a bare top-level <br> with no enclosing paragraph/div must also
        // ~keep no-op here, matching Tier-1's explicit "bare <br> at top level emits
        // ~keep nothing" contract (`tier1/scanner.rs`'s `TagKind::LineBreak` arm) — the
        // ~keep default `block_content_start: 0` from a fresh `Context` still equals
        // ~keep `output.len()` at true document start, so this stays correct there.
        //
        // ~keep The bare `\n` (rather than no output at all) is load-bearing and predates
        // ~keep #464: `integration_test.rs::test_breaks_and_newlines_issue_112` pins that a
        // ~keep leading top-level `<br>` still opens a line. Only the CONDITION changed for
        // ~keep #464 -- the old `output.ends_with('\n')` also matched a break that followed
        // ~keep another break's marker, which is what swallowed consecutive runs.
        output.push('\n');
    } else {
        output.push_str(hard_break_marker(options.newline_style));
    }
}
