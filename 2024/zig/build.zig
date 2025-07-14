const std = @import("std");

// Although this function looks imperative, note that its job is to
// declaratively construct a build graph that will be executed by an external
// runner.
pub fn build(b: *std.Build) void {
    // Standard target options allows the person running `zig build` to choose
    // what target to build for. Here we do not override the defaults, which
    // means any target is allowed, and the default is native. Other options
    // for restricting supported target set are available.
    const target = b.standardTargetOptions(.{});

    // Standard optimization options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall. Here we do not
    // set a preferred release mode, allowing the user to decide how to optimize.
    const optimize = b.standardOptimizeOption(.{});

    buildExe(b, target, optimize, "01_historian_hysteria", "src/01_historian_hysteria.zig");
    buildExe(b, target, optimize, "02_rednosed_reports", "src/02_rednosed_reports.zig");
    buildExe(b, target, optimize, "03_mull_it_over", "src/03_mull_it_over.zig");
    buildExe(b, target, optimize, "04_ceres_search", "src/04_ceres_search.zig");
    buildExe(b, target, optimize, "05_print_queue", "src/05_print_queue.zig");
    buildExe(b, target, optimize, "06_guard_gallivant", "src/06_guard_gallivant.zig");
    buildExe(b, target, optimize, "07_bridge_repair", "src/07_bridge_repair.zig");
    buildExe(b, target, optimize, "08_resonant_collinearity", "src/08_resonant_collinearity.zig");
    buildExe(b, target, optimize, "09_disk_fragmenter", "src/09_disk_fragmenter.zig");
    buildExe(b, target, optimize, "10_hoof_it", "src/10_hoof_it.zig");
    buildExe(b, target, optimize, "11_plutonian_pebbles", "src/11_plutonian_pebbles.zig");
    buildExe(b, target, optimize, "12_garden_groups", "src/12_garden_groups.zig");
    buildExe(b, target, optimize, "13_claw_contraption", "src/13_claw_contraption.zig");
    buildExe(b, target, optimize, "14_restroom_redoubt", "src/14_restroom_redoubt.zig");
    buildExe(b, target, optimize, "15_warehouse_woes", "src/15_warehouse_woes.zig");
    buildExe(b, target, optimize, "16_reindeer_maze", "src/16_reindeer_maze.zig");
    buildExe(b, target, optimize, "17_chronospatial_computer", "src/17_chronospatial_computer.zig");
    buildExe(b, target, optimize, "18_ram_run", "src/18_ram_run.zig");
    buildExe(b, target, optimize, "19_linen_layout", "src/19_linen_layout.zig");
}

fn buildExe(b: *std.Build, target: std.Build.ResolvedTarget, optimize: std.builtin.OptimizeMode, comptime name: []const u8, comptime src_file_path: []const u8) void {
    const exe = b.addExecutable(.{
        .name = name,
        .root_source_file = b.path(src_file_path),
        .target = target,
        .optimize = optimize,
    });

    // This declares intent for the executable to be installed into the
    // standard location when the user invokes the "install" step (the default
    // step when running `zig build`).
    b.installArtifact(exe);

    // This *creates* a Run step in the build graph, to be executed when another
    // step is evaluated that depends on it. The next line below will establish
    // such a dependency.
    const run_cmd = b.addRunArtifact(exe);

    // By making the run step depend on the install step, it will be run from the
    // installation directory rather than directly from within the cache directory.
    // This is not necessary, however, if the application depends on other installed
    // files, this ensures they will be present and in the expected location.
    run_cmd.step.dependOn(b.getInstallStep());

    // This allows the user to pass arguments to the application in the build
    // command itself, like this: `zig build run -- arg1 arg2 etc`
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    // This creates a build step. It will be visible in the `zig build --help` menu,
    // and can be selected like this: `zig build run`
    // This will evaluate the `run` step rather than the default, which is "install".
    const run_step = b.step("run_" ++ name, "Run the app");
    run_step.dependOn(&run_cmd.step);

    const exe_unit_tests = b.addTest(.{
        .root_source_file = b.path(src_file_path),
        .target = target,
        .optimize = optimize,
    });

    const run_exe_unit_tests = b.addRunArtifact(exe_unit_tests);

    // Similar to creating the run step earlier, this exposes a `test` step to
    // the `zig build --help` menu, providing a way for the user to request
    // running the unit tests.
    const test_step = b.step("test_" ++ name, "Run unit tests");
    test_step.dependOn(&run_exe_unit_tests.step);
}
