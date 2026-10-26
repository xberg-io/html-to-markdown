use super::content::{DomContext, get_text_content};
use crate::options::HiddenContent;
use crate::text;
use tl::{Node, NodeHandle, Parser};

enum Step {
    Visit(NodeHandle, bool),
    Mark(&'static str),
}

pub(super) fn text_content(
    node_handle: &NodeHandle,
    parser: &Parser,
    dom_ctx: &DomContext,
    hidden_content: HiddenContent,
) -> String {
    let mut output = String::new();
    let mut stack = vec![Step::Visit(*node_handle, true)];
    while let Some(step) = stack.pop() {
        let (handle, in_math) = match step {
            Step::Mark(mark) => {
                output.push_str(mark);
                continue;
            }
            Step::Visit(handle, in_math) => (handle, in_math),
        };
        match handle.get(parser) {
            Some(Node::Raw(bytes)) => {
                let raw = bytes.as_utf8_str();
                let decoded = text::decode_html_entities_cow(raw.as_ref());
                output.push_str(&decoded.split_ascii_whitespace().collect::<Vec<_>>().join(" "));
            }
            Some(Node::Tag(tag)) => {
                let name = tag.name().as_utf8_str();
                let in_math = in_math || name.eq_ignore_ascii_case("math");
                if in_math && is_token(name.as_ref()) {
                    let token = get_text_content(&handle, parser, dom_ctx);
                    output.push_str(&token.split_ascii_whitespace().collect::<Vec<_>>().join(" "));
                    continue;
                }
                push_children(&mut stack, handle, tag, parser, dom_ctx, hidden_content, in_math);
            }
            _ => {}
        }
    }
    output
}

fn is_token(name: &str) -> bool {
    matches!(name, "mi" | "mo" | "mn" | "mtext" | "ms")
}

fn push_children(
    stack: &mut Vec<Step>,
    handle: NodeHandle,
    tag: &tl::HTMLTag,
    parser: &Parser,
    dom_ctx: &DomContext,
    hidden_content: HiddenContent,
    in_math: bool,
) {
    let children = dom_ctx
        .children_of(handle.get_inner())
        .cloned()
        .unwrap_or_else(|| tag.children().top().to_vec());
    let name = tag.name().as_utf8_str();
    if in_math && matches!(name.as_ref(), "mfrac" | "msup") && !has_raw_operand_text(&children, parser) {
        let operands: Vec<_> = children
            .iter()
            .filter(|child| matches!(child.get(parser), Some(Node::Tag(_))))
            .collect();
        if let [left, right] = operands.as_slice() {
            push_operand(stack, **right, parser, dom_ctx);
            stack.push(Step::Mark(if name == "mfrac" { "/" } else { "^" }));
            push_operand(stack, **left, parser, dom_ctx);
            return;
        }
    }
    if in_math && name == "semantics" && hidden_content != HiddenContent::All {
        // ~keep CSS first-child uses original sibling identity, even when that child is hidden.
        if let Some(child) = tag
            .children()
            .top()
            .iter()
            .find(|child| matches!(child.get(parser), Some(Node::Tag(_))))
        {
            if children.contains(child) {
                stack.push(Step::Visit(*child, in_math));
            }
        }
        return;
    }
    let child_math = in_math && !is_html_integration(tag);
    stack.extend(children.iter().rev().map(|child| Step::Visit(*child, child_math)));
}

fn has_raw_operand_text(children: &[NodeHandle], parser: &Parser) -> bool {
    children.iter().any(|child| {
        matches!(child.get(parser), Some(Node::Raw(bytes)) if !text::decode_html_entities_cow(bytes.as_utf8_str().as_ref()).trim().is_empty())
    })
}

fn is_atomic(handle: NodeHandle, parser: &Parser, dom_ctx: &DomContext) -> bool {
    if !matches!(handle.get(parser), Some(Node::Tag(tag)) if is_token(tag.name().as_utf8_str().as_ref())) {
        return false;
    }
    let token = get_text_content(&handle, parser, dom_ctx);
    let token = token.trim();
    (token.chars().count() == 1 && token.chars().all(char::is_alphanumeric))
        || (!token.is_empty()
            && token.chars().any(|c| c.is_ascii_digit())
            && token.chars().all(|c| c.is_ascii_digit() || c == '.')
            && token.chars().filter(|c| *c == '.').count() <= 1)
}

fn push_operand(stack: &mut Vec<Step>, handle: NodeHandle, parser: &Parser, dom_ctx: &DomContext) {
    let atomic = is_atomic(handle, parser, dom_ctx);
    // ~keep Group compound operands so a/(b/c) and (a+b)^n keep their MathML meaning (#940).
    if !atomic {
        stack.push(Step::Mark(")"));
    }
    stack.push(Step::Visit(handle, true));
    if !atomic {
        stack.push(Step::Mark("("));
    }
}

pub(super) fn semantics_slots(input: &str) -> std::collections::BTreeSet<usize> {
    let mut slots = std::collections::BTreeSet::new();
    if !input
        .as_bytes()
        .windows(b"semantics".len())
        .any(|name| name.eq_ignore_ascii_case(b"semantics"))
    {
        return slots;
    }
    let Ok(dom) = tl::parse(input, tl::ParserOptions::default()) else {
        return slots;
    };
    let parser = dom.parser();
    let mut stack: Vec<_> = dom.children().iter().map(|child| (*child, false)).collect();
    while let Some((handle, in_math)) = stack.pop() {
        let Some(Node::Tag(tag)) = handle.get(parser) else {
            continue;
        };
        let name = tag.name().as_utf8_str();
        let in_math = in_math || name.eq_ignore_ascii_case("math");
        if in_math && name.eq_ignore_ascii_case("semantics") {
            if let Some(first) = tag.children().top().iter().find_map(|child| match child.get(parser) {
                Some(Node::Tag(tag)) => Some(tag.boundaries(parser).0),
                _ => None,
            }) {
                slots.insert(first);
            }
        }
        let child_math = in_math && !is_html_integration(tag);
        stack.extend(tag.children().top().iter().map(|child| (*child, child_math)));
    }
    slots
}

fn is_html_integration(tag: &tl::HTMLTag) -> bool {
    let name = tag.name().as_utf8_str();
    if is_token(name.as_ref()) {
        return true;
    }
    name.eq_ignore_ascii_case("annotation-xml")
        && tag.attributes().get("encoding").flatten().is_some_and(|encoding| {
            encoding.as_utf8_str().eq_ignore_ascii_case("text/html")
                || encoding.as_utf8_str().eq_ignore_ascii_case("application/xhtml+xml")
        })
}
