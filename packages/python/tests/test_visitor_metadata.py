from typing import Any

from html_to_markdown import ConversionOptions, convert

SAMPLE_HTML = """
<html>
  <head>
    <title>Sample Document</title>
    <meta property="og:title" content="Sample Document (OG)">
  </head>
  <body>
    <h1>Top Heading</h1>
    <p>Body text with a <a href="https://example.com">link</a>.</p>
    <img src="https://example.com/a.png" alt="">
    <img src="https://example.com/b.png" alt="described">
  </body>
</html>
"""


class ImageSkipVisitor:
    def visit_image(self, ctx: Any, src: str, alt: str, title: str) -> dict[str, str]:
        del ctx, src, title
        if not alt:
            return {"type": "skip"}
        return {"type": "continue"}


class ContinueAllVisitor:
    def visit_image(self, ctx: Any, src: str, alt: str, title: str) -> dict[str, str]:
        return {"type": "continue"}


def test_visitor_retains_metadata() -> None:
    opts = ConversionOptions(extract_metadata=True, extract_images=True)

    # Baseline: no visitor
    res_base = convert(SAMPLE_HTML, options=opts)
    assert res_base is not None
    assert isinstance(res_base, dict)

    base_md = res_base.get("metadata")
    assert base_md is not None, "Baseline metadata must not be empty"

    base_doc = base_md.get("document", {})
    assert base_doc is not None
    assert base_doc.get("title") == "Sample Document", "Baseline title must be present"
    assert len(base_md.get("headers", [])) == 1, "Baseline 1 header"
    assert len(base_md.get("links", [])) == 1, "Baseline 1 link"
    assert len(base_md.get("images", [])) == 2, "Baseline 2 images in metadata structure"

    # 1st Test: Skip specific image with visitor
    res_skip = convert(SAMPLE_HTML, options=opts, visitor=ImageSkipVisitor())
    skip_md = res_skip.get("metadata")

    assert skip_md is not None, "Metadata should still be populated with visitor"
    skip_doc = skip_md.get("document", {})
    assert skip_doc is not None
    assert skip_doc.get("title") == "Sample Document", "Metadata title should be present"

    # The header and link should still be there
    assert len(skip_md.get("headers", [])) == 1
    assert len(skip_md.get("links", [])) == 1

    # 2nd Test: Monotonic guard bounds tests
    res_continue = convert(SAMPLE_HTML, options=opts, visitor=ContinueAllVisitor())
    cont_md = res_continue.get("metadata")
    assert cont_md is not None

    # Guard: a continue-all visitor should yield strictly identical structure payload to baseline config.
    assert len(cont_md.get("images", [])) == 2, "Continue visitor should not drop metadata elements"
