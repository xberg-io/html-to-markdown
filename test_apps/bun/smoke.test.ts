import { test, expect, describe } from "bun:test";
import { convert } from "@kreuzberg/html-to-markdown";

describe("html-to-markdown Bun smoke tests", () => {
  describe("Basic conversion", () => {
    test("converts simple HTML to Markdown", () => {
      const html = "<h1>Hello World</h1>";
      const result = convert(html);
      expect(result.content).toContain("# Hello World");
      expect(typeof result.content).toBe("string");
    });

    test("converts paragraph to text", () => {
      const html = "<p>This is a test paragraph.</p>";
      const result = convert(html);
      expect(result.content).toContain("This is a test paragraph.");
    });

    test("handles empty input", () => {
      const result = convert("");
      expect(result.content).toBe("");
    });

    test("handles malformed HTML gracefully", () => {
      const html = "<p>Unclosed paragraph";
      expect(() => convert(html)).not.toThrow();
      const result = convert(html);
      expect(typeof result.content).toBe("string");
    });
  });

  describe("Result structure", () => {
    test("returns result with content field", () => {
      const html = "<h1>Test</h1><p>Content</p>";
      const result = convert(html);
      expect(result).toBeDefined();
      expect(result.content).toBeTruthy();
    });

    test("returns result with metadata field", () => {
      const html = "<h1>Title</h1><h2>Subtitle</h2>";
      const result = convert(html);
      expect(result).toBeDefined();
      if (result.metadata) {
        expect(typeof result.metadata).toBe("object");
      }
    });

    test("returns result with warnings field", () => {
      const html = "<p>Test</p>";
      const result = convert(html);
      expect(result.warnings).toBeDefined();
    });
  });

  describe("Options handling", () => {
    test("accepts conversion options", () => {
      const html = "<p>Test</p>";
      const options = { hardBreaks: true };
      expect(() => convert(html, options)).not.toThrow();
      const result = convert(html, options);
      expect(typeof result.content).toBe("string");
    });

    test("handles null options", () => {
      const html = "<p>Test</p>";
      expect(() => convert(html, null)).not.toThrow();
    });
  });

  describe("Complex HTML", () => {
    test("handles lists", () => {
      const html = "<ul><li>Item 1</li><li>Item 2</li></ul>";
      const result = convert(html);
      expect(result.content).toContain("Item 1");
      expect(result.content).toContain("Item 2");
    });

    test("handles links", () => {
      const html = '<a href="https://example.com">Link Text</a>';
      const result = convert(html);
      expect(result.content).toContain("Link Text");
      expect(result.content).toContain("https://example.com");
    });

    test("handles images", () => {
      const html = '<img src="image.jpg" alt="Test Image">';
      const result = convert(html);
      expect(result.content).toContain("Test Image");
      expect(result.content).toContain("image.jpg");
    });
  });
});
