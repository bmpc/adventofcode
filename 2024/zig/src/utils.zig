const std = @import("std");

pub fn concatAndReturnBuffer(allocator: *std.mem.Allocator, one: []const u8, two: []const u8) !std.Buffer {
    var b = try std.Buffer.init(allocator, one);
    try b.append(two);
    return b;
}

pub fn readFile(allocator: *const std.mem.Allocator, file_path: []const u8) ![]u8 {
    var file = try std.fs.cwd().openFile(file_path, .{});
    defer file.close();

    const stat = try file.stat();
    const buff = try file.readToEndAlloc(allocator.*, stat.size);
    return buff;
}

pub fn inSlice(comptime T: type, haystack: []const T, needle: T) bool {
    for (haystack) |thing| {
        if (thing == needle) {
            return true;
        }
    }
    return false;
}
