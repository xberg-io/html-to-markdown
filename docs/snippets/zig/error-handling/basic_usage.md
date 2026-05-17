```zig
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const result_json = html_to_markdown.convert("<h1>Hello</h1>", null) catch |err| switch (err) {
        html_to_markdown.ConversionError.ParseError => {
            std.debug.print("Parse failed\n", .{});
            return;
        },
        else => return err,
    };
    defer std.heap.c_allocator.free(result_json);

    std.debug.print("{s}\n", .{result_json});
}
```
