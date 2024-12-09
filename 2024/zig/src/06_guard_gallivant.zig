const std = @import("std");
const utils = @import("utils.zig");

const assert = std.debug.assert;
const Tuple = std.meta.Tuple(&.{ u32, u32 });

const Position = struct { row: usize, col: usize };

const Step = std.meta.Tuple(&.{ Position, Direction });

const Direction = enum {
    UP,
    RIGHT,
    DOWN,
    LEFT,
    pub fn getNextDir(self: Direction) Direction {
        return switch (self) {
            Direction.UP => Direction.RIGHT,
            Direction.RIGHT => Direction.DOWN,
            Direction.DOWN => Direction.LEFT,
            Direction.LEFT => Direction.UP,
        };
    }
    pub fn getDirSymbol(self: Direction) u8 {
        return switch (self) {
            Direction.UP, Direction.DOWN => '|',
            Direction.RIGHT, Direction.LEFT => '-',
        };
    }
};

pub const Error = error{InvalidPosition};

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const result = try solution(allocator, "input/06_input.txt");

    std.debug.print("Part 1: {d}\n", .{result[0]});
    std.debug.print("Part 2: {d}\n", .{result[1]});
}

fn solution(allocator: std.mem.Allocator, input: []const u8) !Tuple {
    const content = try utils.readFile(&allocator, input);
    defer allocator.free(content);

    var matrix = try utils.Matrix(u8).initFromSequence(content, "\n", allocator);
    defer matrix.deinit();

    const start_pos: Position = try findGuard(&matrix);

    // part 1
    _ = try followGuard(start_pos, &matrix, allocator);
    const total1: u32 = getGuardPathSteps(&matrix);

    //matrix.print();

    // part 2
    // for each position in the original guard path, we need to put an obstruction
    // and check if this results in a loop
    // NOTE: this solution takes around 30s to compute

    var total2: u32 = 0;
    const positions = try getGuardPath(&matrix, allocator);
    defer positions.deinit();

    for (positions.items) |pos| {
        if (std.meta.eql(pos, start_pos)) continue;
        try resetMap(start_pos, pos, &matrix);
        const loop = try followGuard(start_pos, &matrix, allocator);
        if (loop) {
            total2 += 1;
        }
    }

    return .{ total1, total2 };
}

fn findGuard(matrix: *utils.Matrix(u8)) !Position {
    for (0..matrix.height) |row| {
        for (0..matrix.width) |col| {
            const ch = try matrix.get(row, col);
            if (ch == '^') {
                return Position{ .row = row, .col = col };
            }
        }
    }

    return error.InvalidPosition;
}

fn resetMap(start: Position, obstruction: Position, matrix: *utils.Matrix(u8)) !void {
    // set start position
    try matrix.set(start.row, start.col, '^');
    // reset matrix
    for (0..matrix.height) |row| {
        for (0..matrix.width) |col| {
            const ch = try matrix.get(row, col);
            var new_ch: u8 = ch;
            if (obstruction.row == row and obstruction.col == col) {
                new_ch = 'O';
            } else if (ch == '|' or ch == '-' or ch == '+' or ch == 'O') {
                new_ch = '.';
            }
            try matrix.set(row, col, new_ch);
        }
    }
}

fn getGuardPath(matrix: *utils.Matrix(u8), allocator: std.mem.Allocator) !std.ArrayList(Position) {
    var positions = std.ArrayList(Position).init(allocator);

    for (0..matrix.height) |row| {
        for (0..matrix.width) |col| {
            const ch = try matrix.get(row, col);
            if (ch == '|' or ch == '-' or ch == '+') {
                try positions.append(Position{ .row = row, .col = col });
            }
        }
    }

    return positions;
}

fn followGuard(start_pos: Position, matrix: *utils.Matrix(u8), allocator: std.mem.Allocator) !bool {
    var pos: Position = start_pos;
    var dir: Direction = Direction.UP;

    var visited = std.AutoHashMap(Step, bool).init(allocator);
    defer visited.deinit();

    try matrix.set(pos.row, pos.col, '|');
    try visited.put(Step{ pos, Direction.UP }, true);

    var next_pos: ?Position = null;
    while (true) {
        next_pos = try getNextPosition(pos, dir, matrix);
        if (next_pos == null) {
            return false; // guard exited area
        }

        if (visited.contains(Step{ next_pos.?, dir })) { // loop
            return true;
        }

        const val = try matrix.get(next_pos.?.row, next_pos.?.col);
        if (val == '#' or val == 'O') {
            dir = Direction.getNextDir(dir);
        } else {
            var path_symbol: u8 = dir.getDirSymbol();
            if (val != '.' and val != path_symbol) {
                path_symbol = '+';
            }

            try matrix.set(next_pos.?.row, next_pos.?.col, path_symbol);
            try visited.put(Step{ next_pos.?, dir }, true);
            pos = next_pos.?;
        }
    }

    return false;
}

fn getNextPosition(pos: Position, dir: Direction, matrix: *utils.Matrix(u8)) !?Position {
    var next_pos: Position = pos;
    switch (dir) {
        Direction.UP => {
            if (pos.row == 0) {
                return null;
            }
            next_pos.row -= 1;
        },
        Direction.DOWN => {
            if (pos.row == matrix.height - 1) {
                return null;
            }
            next_pos.row += 1;
        },
        Direction.LEFT => {
            if (pos.col == 0) {
                return null;
            }
            next_pos.col -= 1;
        },
        Direction.RIGHT => {
            if (pos.col == matrix.width - 1) {
                return null;
            }
            next_pos.col += 1;
        },
    }

    return next_pos;
}

fn getGuardPathSteps(matrix: *utils.Matrix(u8)) u32 {
    var count: u32 = 0;
    for (0..matrix.height) |row| {
        for (0..matrix.width) |col| {
            const ch = matrix.get(row, col) catch unreachable;
            if (ch == '|' or ch == '-' or ch == '+') {
                count += 1;
            }
        }
    }

    return count;
}

test "example" {
    const result = try solution(std.testing.allocator, "input/06_input_test.txt");

    try std.testing.expectEqual(@as(u32, 41), result[0]);
    try std.testing.expectEqual(@as(u32, 6), result[1]);
}
