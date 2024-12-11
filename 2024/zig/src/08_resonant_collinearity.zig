const std = @import("std");
const utils = @import("utils.zig");

const assert = std.debug.assert;
const Tuple = std.meta.Tuple(&.{ u32, u32 });

const Position = struct {
    row: isize,
    col: isize,

    pub fn minus(self: @This(), pos: Position) Position {
        return Position{ .row = self.row - pos.row, .col = self.col - pos.col };
    }

    pub fn plus(self: @This(), pos: Position) Position {
        return Position{ .row = self.row + pos.row, .col = self.col + pos.col };
    }

    pub fn eql(self: @This(), pos: Position) bool {
        return std.meta.eql(self, pos);
    }
};

pub const Error = error{InvalidPosition};

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const result = try solution(allocator, "input/08_input.txt");

    std.debug.print("Part 1: {d}\n", .{result[0]});
    std.debug.print("Part 2: {d}\n", .{result[1]});
}

fn solution(allocator: std.mem.Allocator, input: []const u8) !Tuple {
    const content = try utils.readFile(&allocator, input);
    defer allocator.free(content);

    var map = try utils.Matrix(u8).initFromSequence(content, "\n", allocator);
    defer map.deinit();

    var antennas = std.AutoHashMap(u8, std.ArrayList(Position)).init(allocator);
    defer {
        var it = antennas.valueIterator();
        while (it.next()) |value_ptr| {
            value_ptr.deinit();
        }

        antennas.deinit();
    }

    for (0..map.height) |row| {
        for (0..map.width) |col| {
            const ch = try map.get(row, col);
            if (ch != '.') {
                const pos = Position{ .row = @intCast(row), .col = @intCast(col) };
                var positions = antennas.get(ch);
                if (positions == null) {
                    positions = std.ArrayList(Position).init(allocator);
                }

                try positions.?.append(pos);
                try antennas.put(ch, positions.?);
            }
        }
    }

    var antinodes1 = std.AutoHashMap(Position, bool).init(allocator);
    defer antinodes1.deinit();

    var antinodes2 = std.AutoHashMap(Position, bool).init(allocator);
    defer antinodes2.deinit();

    var it = antennas.iterator();
    while (it.next()) |entry| {
        const positions = entry.value_ptr;
        if (positions.items.len < 2) {
            continue;
        }

        // part 1
        const new_antinodes1 = try checkForAntinodes(positions, false, &map, allocator);
        for (new_antinodes1) |an| {
            if (!antinodes1.contains(an)) {
                try antinodes1.put(an, true);
            }
        }

        // part 2
        const new_antinodes2 = try checkForAntinodes(positions, true, &map, allocator);
        for (new_antinodes2) |an| {
            if (!antinodes2.contains(an)) {
                try antinodes2.put(an, true);
            }
        }

        allocator.free(new_antinodes1);
        allocator.free(new_antinodes2);
    }

    const total1 = antinodes1.count();
    const total2 = antinodes2.count();

    return .{ total1, total2 };
}

fn checkForAntinodes(positions: *std.ArrayList(Position), extend: bool, map: *utils.Matrix(u8), allocator: std.mem.Allocator) ![]Position {
    var antinodes = std.ArrayList(Position).init(allocator);

    for (positions.items) |anchor_pos| {
        for (positions.items) |pos| {
            if (pos.eql(anchor_pos)) continue;
            if (extend) {
                try antinodes.append(anchor_pos);
            }

            const dist = anchor_pos.minus(pos);

            var curr_pos = anchor_pos;
            var new_pos = anchor_pos.plus(dist);
            while (new_pos.row >= 0 and new_pos.col >= 0 and new_pos.row < map.height and new_pos.col < map.width) {
                try antinodes.append(new_pos);
                if (!extend) break;
                new_pos = curr_pos.plus(dist);
                curr_pos = new_pos;
            }
        }
    }

    return antinodes.toOwnedSlice();
}

test "example" {
    const result = try solution(std.testing.allocator, "input/08_input_test.txt");

    try std.testing.expectEqual(@as(u32, 14), result[0]);
    try std.testing.expectEqual(@as(u32, 34), result[1]);
}
