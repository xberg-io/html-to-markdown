const std = @import("std");
const builtin = @import("builtin");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const test_step = b.step("test", "Run tests");
    const ffi_path = b.option([]const u8, "ffi_path", "Path to directory containing libhtml_to_markdown_ffi") orelse "../../target/release";
    const ffi_include = b.option([]const u8, "ffi_include_path", "Path to directory containing FFI header") orelse "../../crates/html-to-markdown-ffi/include";
    const ffi_path_abs = b.pathFromRoot(ffi_path);

    const html_to_markdown_rs_module = b.addModule("html_to_markdown_rs", .{
        .root_source_file = b.path("../../packages/zig/src/html_to_markdown_rs.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    html_to_markdown_rs_module.addLibraryPath(.{ .cwd_relative = ffi_path });
    html_to_markdown_rs_module.addIncludePath(.{ .cwd_relative = ffi_include });
    html_to_markdown_rs_module.linkSystemLibrary("html_to_markdown_ffi", .{});
    html_to_markdown_rs_module.addRPath(.{ .cwd_relative = ffi_path_abs });

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
    conversion_tests.root_module.addRPath(.{ .cwd_relative = ffi_path_abs });
    const conversion_run = b.addRunArtifact(conversion_tests);
    test_step.dependOn(&conversion_run.step);

    const edge_cases_module = b.createModule(.{
        .root_source_file = b.path("src/edge_cases_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    edge_cases_module.addImport("html_to_markdown_rs", html_to_markdown_rs_module);
    const edge_cases_tests = b.addTest(.{
        .name = "edge_cases_test",
        .root_module = edge_cases_module,
        .use_llvm = true,
    });
    edge_cases_tests.root_module.addRPath(.{ .cwd_relative = ffi_path_abs });
    const edge_cases_run = b.addRunArtifact(edge_cases_tests);
    test_step.dependOn(&edge_cases_run.step);

    const metadata_module = b.createModule(.{
        .root_source_file = b.path("src/metadata_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    metadata_module.addImport("html_to_markdown_rs", html_to_markdown_rs_module);
    const metadata_tests = b.addTest(.{
        .name = "metadata_test",
        .root_module = metadata_module,
        .use_llvm = true,
    });
    metadata_tests.root_module.addRPath(.{ .cwd_relative = ffi_path_abs });
    const metadata_run = b.addRunArtifact(metadata_tests);
    test_step.dependOn(&metadata_run.step);

    const options_module = b.createModule(.{
        .root_source_file = b.path("src/options_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    options_module.addImport("html_to_markdown_rs", html_to_markdown_rs_module);
    const options_tests = b.addTest(.{
        .name = "options_test",
        .root_module = options_module,
        .use_llvm = true,
    });
    options_tests.root_module.addRPath(.{ .cwd_relative = ffi_path_abs });
    const options_run = b.addRunArtifact(options_tests);
    test_step.dependOn(&options_run.step);

    const real_world_module = b.createModule(.{
        .root_source_file = b.path("src/real_world_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    real_world_module.addImport("html_to_markdown_rs", html_to_markdown_rs_module);
    const real_world_tests = b.addTest(.{
        .name = "real_world_test",
        .root_module = real_world_module,
        .use_llvm = true,
    });
    real_world_tests.root_module.addRPath(.{ .cwd_relative = ffi_path_abs });
    const real_world_run = b.addRunArtifact(real_world_tests);
    test_step.dependOn(&real_world_run.step);

    const result_module = b.createModule(.{
        .root_source_file = b.path("src/result_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    result_module.addImport("html_to_markdown_rs", html_to_markdown_rs_module);
    const result_tests = b.addTest(.{
        .name = "result_test",
        .root_module = result_module,
        .use_llvm = true,
    });
    result_tests.root_module.addRPath(.{ .cwd_relative = ffi_path_abs });
    const result_run = b.addRunArtifact(result_tests);
    test_step.dependOn(&result_run.step);

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
    smoke_tests.root_module.addRPath(.{ .cwd_relative = ffi_path_abs });
    const smoke_run = b.addRunArtifact(smoke_tests);
    test_step.dependOn(&smoke_run.step);

    const structure_module = b.createModule(.{
        .root_source_file = b.path("src/structure_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    structure_module.addImport("html_to_markdown_rs", html_to_markdown_rs_module);
    const structure_tests = b.addTest(.{
        .name = "structure_test",
        .root_module = structure_module,
        .use_llvm = true,
    });
    structure_tests.root_module.addRPath(.{ .cwd_relative = ffi_path_abs });
    const structure_run = b.addRunArtifact(structure_tests);
    test_step.dependOn(&structure_run.step);

    const visitor_module = b.createModule(.{
        .root_source_file = b.path("src/visitor_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    visitor_module.addImport("html_to_markdown_rs", html_to_markdown_rs_module);
    const visitor_tests = b.addTest(.{
        .name = "visitor_test",
        .root_module = visitor_module,
        .use_llvm = true,
    });
    visitor_tests.root_module.addRPath(.{ .cwd_relative = ffi_path_abs });
    const visitor_run = b.addRunArtifact(visitor_tests);
    test_step.dependOn(&visitor_run.step);

}
