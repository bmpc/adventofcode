const std = @import("std");

const assert = std.debug.assert;

pub const Error = error{ OutOfBounds, Unsupported };

pub fn intToString(int: u32, buf: []u8) ![]const u8 {
    return try std.fmt.bufPrint(buf, "{}", .{int});
}

pub fn concatNumbers(int1: anytype, int2: anytype, buf: []u8) !@TypeOf(int1) {
    const str = try std.fmt.bufPrint(buf, "{}{}", .{ int1, int2 });
    return try std.fmt.parseInt(@TypeOf(int1), str, 10);
}

pub fn concatNumbersStatic(int1: anytype, int2: anytype) Error!@TypeOf(int1) {
    var factor: @TypeOf(int1) = 0;
    switch (int2) {
        0...9 => factor = 10,
        10...99 => factor = 100,
        100...999 => factor = 1000,
        1000...9999 => factor = 10000,
        else => return Error.Unsupported,
    }

    return (int1 * factor) + int2;
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

        pub fn initWithElem(width: u32, height: u32, base: T, allocator: std.mem.Allocator) Self {
            const data = allocator.alloc(T, width * height) catch unreachable;

            for (0..width * height) |i| {
                data[i] = base;
            }

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

        pub fn clone(self: *Self, allocator: std.mem.Allocator) !Self {
            var dest = try std.ArrayList(T).initCapacity(allocator, self.data.len);
            try dest.appendSlice(self.data);
            std.mem.copyForwards(T, dest.items, self.data);
            return .{ .width = self.width, .height = self.height, .data = try dest.toOwnedSlice(), .allocator = allocator };
        }

        pub fn deinit(self: *Self) void {
            self.allocator.free(self.data);
        }

        pub fn set(self: *Self, row: usize, col: usize, value: T) Error!void {
            if (row > self.height - 1 or col > self.width - 1) {
                return Error.OutOfBounds;
            }
            self.data[row * self.width + col] = value;
        }

        pub fn get(self: *Self, row: usize, col: usize) Error!T {
            if (row > self.height - 1 or col > self.width - 1) {
                return Error.OutOfBounds;
            }
            return self.data[row * self.width + col];
        }

        pub fn getSlice(self: *Self, row: usize, col: usize, size: usize) Error![]const T {
            if (row > self.height - 1 or col > self.width - 1 or (row * self.width + col + size > self.data.len)) {
                return Error.OutOfBounds;
            }
            const idx = row * self.width + col;
            return self.data[idx..(idx + size)];
        }

        pub fn print(self: *Self) void {
            for (0..self.height) |row| {
                for (0..self.width) |col| {
                    const ch = self.get(row, col) catch unreachable;
                    std.debug.print("{c}", .{ch});
                }
                std.debug.print("\n", .{});
            }
        }

        pub fn clear(self: *Self, base: T) void {
            for (0..self.width * self.height) |i| {
                self.data[i] = base;
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
        if (std.meta.eql(thing, needle)) {
            return true;
        }
    }
    return false;
}
