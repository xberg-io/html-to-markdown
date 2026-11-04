from typing import cast

from html_to_markdown import NodeContext, convert
from html_to_markdown.api import HtmlVisitor


class LinkAndImageVisitor:
    """Exercise documented visitor return actions."""

    def visit_link(self, _context: object, href: str, text: str, _title: str | None) -> dict[str, str]:
        """Replace a link with custom output."""
        rewritten_href = href.replace("https://old-cdn.example", "https://new-cdn.example")
        return {"type": "custom", "output": f"[{text}]({rewritten_href})"}

    def visit_image(self, _context: object, src: str, _alt: str, _title: str | None) -> dict[str, str]:
        """Skip tracking images and continue normal images."""
        return {"type": "skip"} if "tracking" in src else {"type": "continue"}


def test_visitor_honors_tagged_link_and_image_actions() -> None:
    html = (
        '<a href="https://old-cdn.example/file.pdf">Download</a>'
        '<img src="https://assets.example/tracking.gif" alt="Tracking">'
        ' <img src="https://assets.example/photo.jpg" alt="Photo">'
    )

    result = convert(html, visitor=cast("HtmlVisitor", LinkAndImageVisitor()))

    assert result.content == "[Download](https://new-cdn.example/file.pdf) ![Photo](https://assets.example/photo.jpg)\n"


class AttributeVisitor:
    """Record the map associated with each visited tag."""

    def __init__(self) -> None:
        self.attributes: dict[str, dict[str, str]] = {}

    def visit_element_start(self, context: NodeContext) -> dict[str, str]:
        """Read the attributes while the element context is active."""
        self.attributes[context.tag_name] = context.attributes()
        return {"type": "continue"}


def test_visitor_reads_each_elements_attributes() -> None:
    visitor = AttributeVisitor()
    html = '<div class="banner"><a href="/a" class="c">a</a><span>b</span></div>'

    result = convert(html, visitor=cast("HtmlVisitor", visitor))

    assert result.metadata.links[0].attributes["class"] == "c"
    assert visitor.attributes == {"div": {"class": "banner"}, "a": {"href": "/a", "class": "c"}, "span": {}}


class NestedAttributeVisitor:
    """Read the inner and outer contexts during reentrant conversion."""

    def __init__(self) -> None:
        self.outer: NodeContext | None = None
        self.reads: list[dict[str, str]] = []

    def visit_element_start(self, context: NodeContext) -> dict[str, str]:
        """Enter a separate conversion from the first callback."""
        if self.outer is None:
            self.outer = context
            convert('<em data-level="inner">inner</em>', visitor=cast("HtmlVisitor", self))
            self.reads.append(context.attributes())
        else:
            self.reads.extend([self.outer.attributes(), context.attributes()])
        return {"type": "continue"}


def test_visitor_keeps_live_nested_contexts_separate() -> None:
    visitor = NestedAttributeVisitor()

    convert('<p data-level="outer">outer</p>', visitor=cast("HtmlVisitor", visitor))

    assert visitor.reads == [{"data-level": "outer"}, {"data-level": "inner"}, {"data-level": "outer"}]
    assert visitor.outer is not None
    assert visitor.outer.attributes() == {}
