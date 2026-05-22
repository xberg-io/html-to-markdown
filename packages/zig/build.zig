const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Default library/include search paths follow the conventional Cargo workspace
    // layout (`<workspace>/target/{profile}` and the FFI crate's `include/` dir).
    // Override with `-Dffi_path=...` and `-Dffi_include_path=...` if your layout differs.
    const ffi_path = b.option([]const u8, "ffi_path", "Path to directory containing libhtml_to_markdown_ffi.{dylib,so,dll,a}") orelse "../../target/release";
    const ffi_include = b.option([]const u8, "ffi_include_path", "Path to directory containing the FFI C header") orelse "../../crates/html-to-markdown-rs-ffi/include";

    const module = b.addModule("html_to_markdown_rs", .{
        .root_source_file = b.path("src/html_to_markdown_rs.zig"),
        .target = target,
        .optimize = optimize,
    });
    module.addLibraryPath(.{ .cwd_relative = ffi_path });
    module.addIncludePath(.{ .cwd_relative = ffi_include });
    module.linkSystemLibrary("html_to_markdown_ffi", .{});

    const test_module = b.createModule(.{
        .root_source_file = b.path("src/html_to_markdown_rs.zig"),
        .target = target,
        .optimize = optimize,
    });
    test_module.addLibraryPath(.{ .cwd_relative = ffi_path });
    test_module.addIncludePath(.{ .cwd_relative = ffi_include });
    test_module.linkSystemLibrary("html_to_markdown_ffi", .{});

    const tests = b.addTest(.{
        .root_module = test_module,
    });

    const run_tests = b.addRunArtifact(tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_tests.step);
}
