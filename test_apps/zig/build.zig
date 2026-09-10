const std = @import("std");
const builtin = @import("builtin");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const test_step = b.step("test", "Run tests");
    const smoke_step = b.step("smoke", "Run smoke tests only");

    // Fetch the published Zig package from the registry.
    const html_to_markdown_dep = b.dependency("html_to_markdown", .{
        .target = target,
        .optimize = optimize,
        .ffi_path = b.pathFromRoot("../c/ffi/lib"),
        .ffi_include_path = b.pathFromRoot("../c/ffi/include"),
    });
    const html_to_markdown_rs_module = html_to_markdown_dep.module("html_to_markdown_rs");

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
    smoke_run.step.dependOn(&conversion_run.step);
    test_step.dependOn(&smoke_run.step);

    const smoke_smoke_run = b.addRunArtifact(smoke_tests);
    smoke_step.dependOn(&smoke_smoke_run.step);

}
