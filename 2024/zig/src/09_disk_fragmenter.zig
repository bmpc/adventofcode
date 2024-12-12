const std = @import("std");
const utils = @import("utils.zig");

const assert = std.debug.assert;
const Tuple = std.meta.Tuple(&.{ u64, u64 });

const FileInfo = struct { id: u32, len: u32, idx: usize };

const FileIterator = struct {
    const Self = @This();

    data: []u32,
    cursor: usize = 0,

    pub fn init(data: []u32) Self {
        return Self{ .data = data, .cursor = data.len - 1 };
    }

    pub fn next(self: *Self) ?FileInfo {
        var curr_block: u32 = 0;
        var curr_block_len: u32 = 0;

        if (self.cursor == 0) return null;

        while (true) {
            const block = self.data[self.cursor];
            if (block != 0 or curr_block != 0) {
                if (curr_block == 0) {
                    curr_block = block;
                    curr_block_len = 1;
                } else {
                    if (curr_block == block) {
                        curr_block_len += 1;
                    } else {
                        // attempt to move file
                        return FileInfo{ .id = curr_block, .len = curr_block_len, .idx = self.cursor + 1 };
                    }
                }
            }

            if (self.cursor == 0) {
                break;
            }
            self.cursor -= 1;
        }

        if (curr_block_len > 0) {
            return FileInfo{ .id = curr_block, .len = curr_block_len, .idx = 0 };
        }

        return null;
    }
};

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const result = try solution(allocator, "input/09_input.txt");

    std.debug.print("Part 1: {d}\n", .{result[0]});
    std.debug.print("Part 2: {d}\n", .{result[1]});
}

fn solution(allocator: std.mem.Allocator, input: []const u8) !Tuple {
    const data = try utils.readFile(&allocator, input);
    defer allocator.free(data);

    const decoded = try decode(data, allocator);
    defer allocator.free(decoded);

    // part 1
    const frag = try allocator.alloc(u32, decoded.len);
    std.mem.copyForwards(u32, frag, decoded);
    defer allocator.free(frag);

    fragment(frag);
    const cs1 = checksum(frag);

    // part 2
    const defrag = try allocator.alloc(u32, decoded.len);
    std.mem.copyForwards(u32, defrag, decoded);
    defer allocator.free(defrag);

    defragment(defrag);
    const cs2 = checksum(defrag);

    return .{ cs1, cs2 };
}

fn decode(data: []u8, alloc: std.mem.Allocator) ![]u32 {
    var decoded = std.ArrayList(u32).init(alloc);

    var file_id: u32 = 1;
    for (data, 0..) |ch, i| {
        const block = @as(usize, ch - 48); // ascii to number
        if (i % 2 == 0) { // file
            try decoded.appendNTimes(file_id, block);
            file_id += 1;
        } else { // empty space
            if (block > 0) {
                try decoded.appendNTimes(0, block);
            }
        }
    }

    return decoded.toOwnedSlice();
}

fn fragment(data: []u32) void {
    var cursor: usize = data.len - 1;

    for (data, 0..) |block, i| {
        if (block == 0) {
            while (data[cursor] == 0 and cursor > i) {
                cursor -= 1;
            }
            data[i] = data[cursor];
            data[cursor] = 0;
            cursor -= 1;
        }

        if (i >= cursor) break;
    }
}

fn defragment(data: []u32) void {
    var file_it = FileIterator.init(data);

    while (file_it.next()) |fi| {
        var empty_len: u32 = 0;
        for (data[0..(fi.idx + 1)], 0..) |block, i| {
            if (block == 0) {
                empty_len += 1;
            } else if (empty_len > 0) {
                if (fi.len <= empty_len) {
                    for (0..fi.len) |j| {
                        data[i - empty_len + j] = fi.id; // copy
                        data[fi.idx + j] = 0; // clear
                    }
                    break;
                }
                empty_len = 0;
            }
        }
    }
}

fn checksum(data: []u32) u64 {
    var sum: u64 = 0;
    for (data, 0..) |block, i| {
        if (block == 0) continue;
        sum += @as(u64, block - 1) * @as(u64, @truncate(i));
    }
    return sum;
}

test "example1" {
    const result = try solution(std.testing.allocator, "input/09_input_test1.txt");

    try std.testing.expectEqual(@as(u64, 1928), result[0]);
    try std.testing.expectEqual(@as(u64, 2858), result[1]);
}

test "example2" {
    const result = try solution(std.testing.allocator, "input/09_input_test2.txt");

    try std.testing.expectEqual(@as(u64, 60), result[0]);
    try std.testing.expectEqual(@as(u64, 132), result[1]);
}
