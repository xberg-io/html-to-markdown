const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const test_step = b.step("test", "Run tests");
    const ffi_path = b.option([]const u8, "ffi_path", "Path to directory containing libhtml_to_markdown_ffi") orelse "../../target/debug";
    const ffi_include = b.option([]const u8, "ffi_include_path", "Path to directory containing FFI header") orelse "../../crates/html-to-markdown-ffi/include";

    const html_to_markdown_rs_module = b.addModule("html_to_markdown_rs", .{
        .root_source_file = b.path("../../packages/zig/src/html_to_markdown.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    html_to_markdown_rs_module.addLibraryPath(.{ .cwd_relative = ffi_path });
    html_to_markdown_rs_module.addIncludePath(.{ .cwd_relative = ffi_include });
    html_to_markdown_rs_module.linkSystemLibrary("html_to_markdown_ffi", .{});

    const conversion_module = b.createModule(.{
        .root_source_file = b.path("src/conversion_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    conversion_module.addImport("html_to_markdown_rs", html_to_markdown_rs_module);
    const conversion_tests = b.addTest(.{
        .name = "conversion_test",
        .root_module = conversion_module,
        .use_llvm = true,
    });
    const conversion_run = b.addRunArtifact(conversion_tests);
    test_step.dependOn(&conversion_run.step);

    const smoke_module = b.createModule(.{
        .root_source_file = b.path("src/smoke_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    smoke_module.addImport("html_to_markdown_rs", html_to_markdown_rs_module);
    const smoke_tests = b.addTest(.{
        .name = "smoke_test",
        .root_module = smoke_module,
        .use_llvm = true,
    });
    const smoke_run = b.addRunArtifact(smoke_tests);
    test_step.dependOn(&smoke_run.step);

}
