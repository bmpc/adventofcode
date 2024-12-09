const std = @import("std");

const assert = std.debug.assert;

pub const BoundsError = error{OutOfBounds};

pub fn intToString(int: u32, buf: []u8) ![]const u8 {
    return try std.fmt.bufPrint(buf, "{}", .{int});
}

pub fn Matrix(comptime T: type) type {
    return struct {
        data: []T = undefined,
        width: usize,
        height: usize,
        allocator: std.mem.Allocator,

        const Self = @This();
        pub fn init(width: u32, height: u32, allocator: std.mem.Allocator) Self {
            const data = allocator.alloc(T, width * height) catch unreachable;

            return .{ .width = width, .height = height, .data = data, .allocator = allocator };
        }

        pub fn initFromSequence(seq: []const T, delimiter: []const u8, allocator: std.mem.Allocator) !Self {
            var rows = std.mem.split(T, seq, delimiter);
            var list = std.ArrayList(T).init(allocator);

            var width: usize = 0;
            var height: usize = 0;

            while (rows.next()) |row| {
                if (width == 0) {
                    width = row.len;
                }
                assert(width == row.len);
                try list.appendSlice(row);
                height += 1;
            }

            const data = try list.toOwnedSlice();

            return .{ .width = width, .height = height, .data = data, .allocator = allocator };
        }

        pub fn deinit(self: *Self) void {
            self.allocator.free(self.data);
        }

        pub fn set(self: *Self, row: usize, col: usize, value: T) BoundsError!void {
            if (row > self.height - 1 or col > self.width - 1) {
                return BoundsError.OutOfBounds;
            }
            self.data[row * self.width + col] = value;
        }

        pub fn get(self: *Self, row: usize, col: usize) BoundsError!T {
            if (row > self.height - 1 or col > self.width - 1) {
                return BoundsError.OutOfBounds;
            }
            return self.data[row * self.width + col];
        }

        pub fn getSlice(self: *Self, row: usize, col: usize, size: usize) BoundsError![]const T {
            if (row > self.height - 1 or col > self.width - 1 or (row * self.width + col + size > self.data.len)) {
                return BoundsError.OutOfBounds;
            }
            const idx = row * self.width + col;
            return self.data[idx..(idx + size)];
        }

        pub fn print(self: *Self) void {
            for (0..self.height) |row| {
                for (0..self.width) |col| {
                    const ch = self.get(row, col) catch unreachable;
                    std.debug.print("{c} ", .{ch});
                }
                std.debug.print("\n", .{});
            }
        }
    };
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
