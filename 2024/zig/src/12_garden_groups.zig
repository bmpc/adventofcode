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

const BorderPlot = struct { pos: Position, dir: Direction };

const Region = struct {
    const Self = @This();

    positions: []Position,
    plant_type: u8,

    pub fn perimeter(self: *const Self, width: usize, height: usize, allocator: std.mem.Allocator) !std.ArrayList(BorderPlot) {
        var border: std.ArrayList(BorderPlot) = std.ArrayList(BorderPlot).init(allocator);

        for (self.positions) |pos| {
            for ([_]Direction{ Direction.UP, Direction.RIGHT, Direction.DOWN, Direction.LEFT }) |dir| {
                const neighbour = switch (dir) {
                    Direction.UP => if (pos.row > 0) Position{ .row = pos.row - 1, .col = pos.col } else null,
                    Direction.RIGHT => if (pos.col < width - 1) Position{ .row = pos.row, .col = pos.col + 1 } else null,
                    Direction.DOWN => if (pos.row < height - 1) Position{ .row = pos.row + 1, .col = pos.col } else null,
                    Direction.LEFT => if (pos.col > 0) Position{ .row = pos.row, .col = pos.col - 1 } else null,
                };

                if (neighbour == null or !utils.inSlice(Position, self.positions, neighbour.?)) {
                    const bl = BorderPlot{ .pos = pos, .dir = dir };
                    try border.append(bl);
                }
            }
        }

        return border;
    }
};

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const result = try solution(allocator, "input/12_input.txt");

    std.debug.print("Part 1: {d}\n", .{result[0]});
    std.debug.print("Part 2: {d}\n", .{result[1]});
}

fn solution(allocator: std.mem.Allocator, input: []const u8) !Tuple {
    const content = try utils.readFile(&allocator, input);
    defer allocator.free(content);

    var matrix = try utils.Matrix(u8).initFromSequence(content, "\n", allocator);
    defer matrix.deinit();

    var visited = std.AutoHashMap(Position, bool).init(allocator);
    defer visited.deinit();

    var regions = std.ArrayList(Region).init(allocator);
    defer {
        for (regions.items) |region| {
            allocator.free(region.positions);
        }
        regions.deinit();
    }

    // determine regions
    for (0..matrix.height) |row| {
        for (0..matrix.width) |col| {
            const pos = Position{ .row = row, .col = col };
            if (visited.contains(pos)) {
                continue;
            }

            var positions = std.ArrayList(Position).init(allocator);
            try positions.append(pos);

            try determineRegionGardenPlots(pos, &matrix, &positions);

            // add all positions to visited
            for (positions.items) |position| {
                try visited.put(position, true);
            }

            const p = try matrix.get(pos.row, pos.col);
            try regions.append(Region{ .plant_type = p, .positions = try positions.toOwnedSlice() });
        }
    }

    // get regions perimeter perimiter
    var total1: u32 = 0;
    var total2: u32 = 0;
    for (regions.items) |region| {
        const borderPlots = try region.perimeter(matrix.width, matrix.height, allocator);
        defer borderPlots.deinit();

        const size: u32 = @intCast(region.positions.len);
        const per: u32 = @intCast(borderPlots.items.len);
        const _sides: u32 = sides(&borderPlots);
        total1 += per * size;
        total2 += _sides * size;
    }

    return .{ total1, total2 };
}

fn determineRegionGardenPlots(pos: Position, matrix: *utils.Matrix(u8), positions: *std.ArrayList(Position)) !void {
    for ([_]Direction{ Direction.UP, Direction.RIGHT, Direction.DOWN, Direction.LEFT }) |dir| {
        const new_pos = try next(matrix, pos, dir);
        if (new_pos != null) {
            if (!utils.inSlice(Position, positions.items, new_pos.?)) {
                try positions.append(new_pos.?);
                try determineRegionGardenPlots(new_pos.?, matrix, positions);
            }
        }
    }
}

fn next(matrix: *utils.Matrix(u8), pos: Position, dir: Direction) !?Position {
    const p = try matrix.get(pos.row, pos.col);

    const new_pos = switch (dir) {
        Direction.UP => if (pos.row > 0) Position{ .row = pos.row - 1, .col = pos.col } else null,
        Direction.RIGHT => if (pos.col < matrix.width - 1) Position{ .row = pos.row, .col = pos.col + 1 } else null,
        Direction.DOWN => if (pos.row < matrix.height - 1) Position{ .row = pos.row + 1, .col = pos.col } else null,
        Direction.LEFT => if (pos.col > 0) Position{ .row = pos.row, .col = pos.col - 1 } else null,
    };
    if (new_pos != null) {
        const np = matrix.get(new_pos.?.row, new_pos.?.col) catch unreachable;
        if (np == p) {
            return new_pos;
        }
    }

    return null;
}

// Sort the border garden plots according to (DIR, ROW, COL).
// After the sort, we just need to skip the count on consequetive nodes for the same dir
fn sides(perimiter: *const std.ArrayList(BorderPlot)) u32 {
    // first sort the border garden plots
    std.mem.sort(BorderPlot, perimiter.items, {}, borderPlotAsc);

    var count: u32 = 1;
    var prev_bp: BorderPlot = perimiter.items[0];

    for (perimiter.items[1..]) |bp| {
        const pr: i32 = @intCast(prev_bp.pos.row);
        const pc: i32 = @intCast(prev_bp.pos.col);
        const cr: i32 = @intCast(bp.pos.row);
        const cc: i32 = @intCast(bp.pos.col);
        if (prev_bp.dir == bp.dir and (@abs(pr - cr) + @abs(pc - cc)) == 1) {
            prev_bp = bp;
            continue;
        }

        count += 1;
        prev_bp = bp;
    }

    return count;
}

fn borderPlotAsc(_: void, a: BorderPlot, b: BorderPlot) bool {
    if (a.dir == b.dir) {
        if (a.dir == Direction.DOWN or a.dir == Direction.UP) {
            if (a.pos.row == b.pos.row) {
                return a.pos.col < b.pos.col;
            } else if (a.pos.col == b.pos.col) {
                return a.pos.row < b.pos.row;
            }

            return a.pos.row < b.pos.row;
        } else {
            if (a.pos.col == b.pos.col) {
                return a.pos.row < b.pos.row;
            } else if (a.pos.row == b.pos.row) {
                return a.pos.col < b.pos.col;
            }

            return a.pos.col < b.pos.col;
        }
    } else {
        return @intFromEnum(a.dir) < @intFromEnum(b.dir);
    }
}

test "example 1" {
    const result = try solution(std.testing.allocator, "input/12_input_test1.txt");

    try std.testing.expectEqual(@as(u32, 140), result[0]);
    try std.testing.expectEqual(@as(u32, 80), result[1]);
}

test "example 2" {
    const result = try solution(std.testing.allocator, "input/12_input_test2.txt");

    try std.testing.expectEqual(@as(u32, 1930), result[0]);
    try std.testing.expectEqual(@as(u32, 1206), result[1]);
}
