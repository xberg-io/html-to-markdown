```zig
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");
const c = html_to_markdown.c;

// Visitor callbacks return an int32 status code:
//   0 (HTM_VISIT_CONTINUE)      use default conversion
//   1 (HTM_VISIT_SKIP)          drop the element
//   2 (HTM_VISIT_PRESERVE_HTML) emit the raw HTML
//   3 (HTM_VISIT_CUSTOM)        replace with the string written via out_custom/out_len
//   4 (HTM_VISIT_ERROR)         abort conversion with the error in out_custom
fn visit_heading(
    _ctx: [*c]const c.HTMHtmNodeContext,
    _user_data: ?*anyopaque,
    _level: u32,
    _text: [*c]const u8,
    _id: [*c]const u8,
    out_custom: [*c][*c]u8,
    out_len: [*c]usize,
) callconv(.c) i32 {
    _ = _ctx;
    _ = _user_data;
    _ = _id;
    const text = std.mem.span(_text);
    const buf = std.fmt.allocPrintSentinel(
        std.heap.c_allocator,
        "<<H{d}: {s}>>",
        .{ _level, text },
        0,
    ) catch return 0;
    if (out_custom != null) out_custom.* = buf.ptr;
    if (out_len != null) out_len.* = buf.len;
    return 3; // HTM_VISIT_CUSTOM
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var callbacks: c.HTMHtmVisitorCallbacks = std.mem.zeroes(c.HTMHtmVisitorCallbacks);
    callbacks.visit_heading = &visit_heading;

    const visitor = c.htm_visitor_create(&callbacks);
    defer c.htm_visitor_free(visitor);

    const options_z = try std.heap.c_allocator.dupeZ(u8, "{}");
    defer std.heap.c_allocator.free(options_z);
    const options = c.htm_conversion_options_from_json(options_z.ptr);
    defer c.htm_conversion_options_free(options);
    c.htm_options_set_visitor_handle(options, visitor);

    const html_z = try std.heap.c_allocator.dupeZ(u8, "<h1>Title</h1><p>Body.</p>");
    defer std.heap.c_allocator.free(html_z);

    const result = c.htm_convert(html_z.ptr, options) orelse return error.ConvertFailed;
    defer c.htm_conversion_result_free(result);

    const json_ptr = c.htm_conversion_result_to_json(result);
    defer c.htm_free_string(json_ptr);
    const json = std.mem.sliceTo(json_ptr, 0);

    var parsed = try std.json.parseFromSlice(std.json.Value, allocator, json, .{});
    defer parsed.deinit();
    std.debug.print("{s}\n", .{parsed.value.object.get("content").?.string});
}
```
