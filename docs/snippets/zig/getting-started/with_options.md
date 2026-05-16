```zig
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const html = "<h1>Hello</h1><p>This is <strong>formatted</strong> content.</p>";
    const options_json =
        \\{"heading_style":"atx","list_indent_width":2,"wrap":true}
    ;

    const result_json = try html_to_markdown.convert(html, options_json);
    defer std.heap.c_allocator.free(result_json);

    std.debug.print("{s}\n", .{result_json});
}
```
