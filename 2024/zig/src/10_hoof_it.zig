const std = @import("std");
const utils = @import("utils.zig");

const assert = std.debug.assert;
const Tuple = std.meta.Tuple(&.{ u32, u32 });

const Position = struct {
    row: usize,
    col: usize,

    pub fn eql(self: @This(), pos: Position) bool {
        return std.meta.eql(self, pos);
    }
};
const Direction = enum { UP, RIGHT, DOWN, LEFT };

const Move = struct { pos: Position, dir: Direction };

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const result = try solution(allocator, "input/10_input.txt");

    std.debug.print("Part 1: {d}\n", .{result[0]});
    std.debug.print("Part 2: {d}\n", .{result[1]});
}

fn solution(allocator: std.mem.Allocator, input: []const u8) !Tuple {
    const content = try utils.readFile(&allocator, input);
    defer allocator.free(content);

    var matrix = try utils.Matrix(u8).initFromSequence(content, "\n", allocator);
    defer matrix.deinit();

    // convert matrix values to ints
    for (0..matrix.height) |row| {
        for (0..matrix.width) |col| {
            const h = try matrix.get(row, col);
            const v = try std.fmt.parseInt(u8, &[_]u8{h}, 10);
            try matrix.set(row, col, v);
        }
    }

    // start by finding the trail start points
    var total1: u32 = 0;
    var total2: u32 = 0;

    for (0..matrix.height) |row| {
        for (0..matrix.width) |col| {
            const h = try matrix.get(row, col);
            if (h == 0) {
                var final_positions: std.AutoHashMap(Position, bool) = std.AutoHashMap(Position, bool).init(allocator);
                defer final_positions.deinit();
                total2 += try trailHeadScore(&matrix, Position{ .row = row, .col = col }, null, &final_positions);

                total1 += final_positions.count();
            }
        }
    }

    return .{ total1, total2 };
}

fn trailHeadScore(matrix: *utils.Matrix(u8), pos: Position, old_pos: ?Position, final_positions: *std.AutoHashMap(Position, bool)) !u32 {
    var score: u32 = 0;

    for ([_]Direction{ Direction.UP, Direction.RIGHT, Direction.DOWN, Direction.LEFT }) |dir| {
        var new_pos = move(matrix, pos, dir);
        if (new_pos != null) {
            if (old_pos != null and new_pos.?.eql(old_pos.?)) { // do not turn back
                continue;
            }
            const v = matrix.get(new_pos.?.row, new_pos.?.col) catch unreachable;
            if (v == 9) {
                score += 1;
                try final_positions.put(new_pos.?, true);
                continue;
            }

            score += try trailHeadScore(matrix, new_pos.?, pos, final_positions);
        }
    }

    return score;
}

fn move(matrix: *utils.Matrix(u8), pos: Position, dir: Direction) ?Position {
    const new_pos = switch (dir) {
        Direction.UP => if (pos.row > 0) Position{ .row = pos.row - 1, .col = pos.col } else null,
        Direction.RIGHT => if (pos.col < matrix.width - 1) Position{ .row = pos.row, .col = pos.col + 1 } else null,
        Direction.DOWN => if (pos.row < matrix.height - 1) Position{ .row = pos.row + 1, .col = pos.col } else null,
        Direction.LEFT => if (pos.col > 0) Position{ .row = pos.row, .col = pos.col - 1 } else null,
    };
    if (new_pos != null) {
        const v1 = matrix.get(pos.row, pos.col) catch unreachable;
        const v2 = matrix.get(new_pos.?.row, new_pos.?.col) catch unreachable;
        if (v2 == v1 + 1) {
            return new_pos;
        }
    }

    return null;
}

test "example1 part 1" {
    const result = try solution(std.testing.allocator, "input/10_input_test1.txt");

    try std.testing.expectEqual(@as(u32, 36), result[0]);
}

test "example2 part 1" {
    const result = try solution(std.testing.allocator, "input/10_input_test2.txt");

    try std.testing.expectEqual(@as(u32, 2), result[0]);
}

test "example3 part 1" {
    const result = try solution(std.testing.allocator, "input/10_input_test3.txt");

    try std.testing.expectEqual(@as(u32, 3), result[0]);
}

test "example4 part 2" {
    const result = try solution(std.testing.allocator, "input/10_input_test4.txt");

    try std.testing.expectEqual(@as(u32, 3), result[1]);
}

test "example5 part 2" {
    const result = try solution(std.testing.allocator, "input/10_input_test5.txt");

    try std.testing.expectEqual(@as(u32, 81), result[1]);
}
