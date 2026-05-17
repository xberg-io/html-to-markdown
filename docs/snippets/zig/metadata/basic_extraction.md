```zig
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const html =
        \\<html><head>
        \\<title>Example Page</title>
        \\<meta name="description" content="A short description.">
        \\<meta name="author" content="Jane Doe">
        \\<link rel="canonical" href="https://example.com/page">
        \\</head><body><h1>Hello</h1></body></html>
    ;

    // `convert` returns the ConversionResult as a JSON string. Metadata
    // extraction is controlled by the `extract_metadata` option (default true).
    const result_json = try html_to_markdown.convert(html, "{\"extract_metadata\":true}");
    defer std.heap.c_allocator.free(result_json);

    var parsed = try std.json.parseFromSlice(std.json.Value, allocator, result_json, .{});
    defer parsed.deinit();
    const document = parsed.value.object.get("metadata").?.object.get("document").?.object;

    if (document.get("title")) |v| {
        if (v == .string) std.debug.print("Title: {s}\n", .{v.string});
    }
    if (document.get("description")) |v| {
        if (v == .string) std.debug.print("Description: {s}\n", .{v.string});
    }
    if (document.get("author")) |v| {
        if (v == .string) std.debug.print("Author: {s}\n", .{v.string});
    }
    if (document.get("canonical_url")) |v| {
        if (v == .string) std.debug.print("Canonical: {s}\n", .{v.string});
    }
}
```
