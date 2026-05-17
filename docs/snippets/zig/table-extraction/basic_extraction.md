```zig
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const html =
        \\<table>
        \\  <tr><th>Name</th><th>Age</th></tr>
        \\  <tr><td>Alice</td><td>30</td></tr>
        \\  <tr><td>Bob</td><td>25</td></tr>
        \\</table>
    ;

    // Tables are always extracted into the `tables` array of ConversionResult.
    const result_json = try html_to_markdown.convert(html, null);
    defer std.heap.c_allocator.free(result_json);

    var parsed = try std.json.parseFromSlice(std.json.Value, allocator, result_json, .{});
    defer parsed.deinit();

    const tables = parsed.value.object.get("tables").?.array;
    std.debug.print("Extracted {d} table(s)\n", .{tables.items.len});

    for (tables.items, 0..) |table, i| {
        const grid = table.object.get("grid").?.object;
        const rows = grid.get("rows").?.integer;
        const cols = grid.get("cols").?.integer;
        const markdown = table.object.get("markdown").?.string;
        std.debug.print("Table {d}: {d}x{d}\n{s}\n", .{ i, rows, cols, markdown });

        for (grid.get("cells").?.array.items) |cell| {
            const content = cell.object.get("content").?.string;
            const row = cell.object.get("row").?.integer;
            const col = cell.object.get("col").?.integer;
            const is_header = cell.object.get("is_header").?.bool;
            std.debug.print("  [{d},{d}] header={} '{s}'\n", .{ row, col, is_header, content });
        }
    }
}
```
