const std = @import("std");
const Builder = std.Build;
const Mode = std.builtin.Mode;

pub fn build(b: *Builder) void {
    const target = b.standardTargetOptions(.{});

    const lib = b.addStaticLibrary(.{
        .name = "utils_zig",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = Mode.ReleaseSmall,
    });

    b.installArtifact(lib);
}
