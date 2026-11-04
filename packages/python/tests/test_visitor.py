from typing import cast

from html_to_markdown import ConversionOptions, NodeContext, convert
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


def test_visitor_decodes_href_and_iframe_src_once() -> None:
    visitor = AttributeVisitor()
    html = '<a href="/x?a=1&amp;b=2" title="A &quot;q&quot; &lt;" data-nested="&amp;lt;">t</a>'
    html += '<iframe src="/y?a=1&amp;b=2"></iframe>'

    convert(html, visitor=cast("HtmlVisitor", visitor))

    assert visitor.attributes["a"] == {"href": "/x?a=1&b=2", "title": 'A "q" <', "data-nested": "&lt;"}
    assert visitor.attributes["iframe"] == {"src": "/y?a=1&b=2"}


class MediaChildVisitor(AttributeVisitor):
    """Record media child end callbacks as well as attributes."""

    def __init__(self) -> None:
        super().__init__()
        self.ends: list[str] = []

    def visit_element_end(self, context: NodeContext, _output: str) -> dict[str, str]:
        """Record the closed element."""
        self.ends.append(context.tag_name)
        return {"type": "continue"}


def test_visitor_observes_lazy_media_source_and_track() -> None:
    visitor = MediaChildVisitor()
    html = '<div><video controls><source data-src="/x.mp4?a=1&amp;b=2"><track src="/captions.vtt"></video></div>'

    result = convert(html, visitor=cast("HtmlVisitor", visitor))

    assert visitor.attributes["source"] == {"data-src": "/x.mp4?a=1&b=2"}
    assert visitor.attributes["track"] == {"src": "/captions.vtt"}
    assert visitor.ends == ["source", "track", "video", "div"]
    assert result.content == convert(html).content


def test_preserve_tags_keeps_media_source_visitor_output_equal_to_default() -> None:
    html = '<video><source src="/media.mp4"></video><source src="/outside.mp4">'
    options = ConversionOptions(preserve_tags=["source"])
    visitor = MediaChildVisitor()

    observed = convert(html, options, visitor=cast("HtmlVisitor", visitor))

    assert observed.content == convert(html, options).content
    assert observed.content is not None
    assert '<source src="/media.mp4">' not in observed.content
    assert '<source src="/outside.mp4">' in observed.content
    assert visitor.ends == ["source", "video"]
