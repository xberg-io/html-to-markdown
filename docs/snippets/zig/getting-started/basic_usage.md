```zig
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const html = "<h1>Hello</h1><p>This is <strong>fast</strong>!</p>";
    const result_json = try html_to_markdown.convert(html, null);
    defer std.heap.c_allocator.free(result_json);

    // result_json is the ConversionResult serialised as JSON; parse with
    // std.json or read the `content` field directly.
    std.debug.print("{s}\n", .{result_json});
}
```
