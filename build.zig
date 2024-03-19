const std = @import("std");

// pub fn addAllIncludePaths(b: *std.build.Builder, exe: *std.build.LibExeObjStep, basePath: []const u8) !void {
//     var dir = try std.fs.cwd().openDir(basePath, .{});
//     var iter = try dir.openIterableDir(basePath, .{});
//     defer dir.close();
//     defer iter.close();
//     var it = try iter.walk(b.allocator);
//     while (try it.next()) |entry| {
//         if (entry.kind == .directory) {
//             // Skip the base path itself
//             if (std.mem.eql(u8, entry.path, basePath)) continue;
//             exe.addIncludePath(std.build.LazyPath{ .path = entry.path });
//         }
//     }
// }

fn addDirectories(b: *std.Build, exe: *std.Build.Step.Compile, base: []const u8, directories: []const []const u8) !void {
    for (directories) |dir| {
        const fullPath = try std.fs.path.join(b.allocator, &[_][]const u8{ base, dir });
        defer b.allocator.free(fullPath);

        exe.addIncludePath(std.Build.LazyPath{ .path = fullPath });
        // Any other operation you need to perform with 'dir'
    }
}

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

    const lib = b.addSharedLibrary(.{
        .name = "summon",
        // In this case the main source file is merely a path, however, in more
        // complicated build scripts, this could be a generated file.
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    lib.linkLibC();
    lib.addIncludePath(std.Build.LazyPath{ .path = "./nginx-1.22.1/objs" });
    const directories = &[_][]const u8{
        "/mail",
        "/os/unix",
        "/os",
        "/event/modules",
        "/event",
        "/stream",
        "/http/v2",
        "/http/modules/perl",
        "/http/modules",
        "/http",
        "/core",
        "/misc",
    };
    addDirectories(b, lib, "./nginx-1.22.1/src", directories) catch {
        std.debug.print("errb", .{});
    };
    addDirectories(b, lib, "./nginx-1.22.1/objs/src", directories) catch {
        std.debug.print("erra", .{});
    };

    // This declares intent for the library to be installed into the standard
    // location when the user invokes the "install" step (the default step when
    // running `zig build`).
    b.installArtifact(lib);

    // Creates a step for unit testing. This only builds the test executable
    // but does not run it.
    const main_tests = b.addTest(.{
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    const run_main_tests = b.addRunArtifact(main_tests);

    // This creates a build step. It will be visible in the `zig build --help` menu,
    // and can be selected like this: `zig build test`
    // This will evaluate the `test` step rather than the default, which is "install".
    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&run_main_tests.step);
}
