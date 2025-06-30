const std = @import("std");
const utils = @import("utils.zig");

const assert = std.debug.assert;
const Tuple = std.meta.Tuple(&.{ u32, u32 });

const Vec2 = struct {
    x: usize,
    y: usize,

    fn step(self: *@This(), dir: Direction) void {
        switch (dir) {
            Direction.UP => self.y = self.y - 1,
            Direction.DOWN => self.y = self.y + 1,
            Direction.LEFT => self.x = self.x - 1,
            Direction.RIGHT => self.x = self.x + 1,
        }
    }
};

const Direction = enum(u8) { UP = '^', DOWN = 'v', LEFT = '<', RIGHT = '>' };

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const result = try solution(allocator, "input/15_input.txt");

    std.debug.print("Part 1: {d}\n", .{result[0]});
    std.debug.print("Part 2: {d}\n", .{result[1]});
}

fn solution(allocator: std.mem.Allocator, input: []const u8) !Tuple {
    const data = try utils.readFile(&allocator, input);
    defer allocator.free(data);

    var parts_it = std.mem.splitSequence(u8, data, "\n\n");

    var map = try utils.Matrix(u8).initFromSequence(parts_it.next().?, "\n", allocator);
    defer map.deinit();
    var map2 = try map.clone(allocator);
    defer map2.deinit();

    var directions = std.ArrayList(Direction).init(allocator);
    defer directions.deinit();

    const dirs = parts_it.next().?;
    var lines_it = std.mem.splitSequence(u8, dirs, "\n");

    var l: usize = 0;
    while (lines_it.next()) |line| {
        for (line) |ch| {
            try directions.append(@enumFromInt(ch));
        }
        l += 1;
    }

    // part 1
    var curr = try find_start(&map);
    for (directions.items) |dir| {
        curr = try move(&map, curr, dir);
    }
    const total1 = try get_boxes_coordinates_sum(&map);

    // part 2
    var expanded_map = try expand_map(&map2, allocator);
    defer expanded_map.deinit();

    curr = try find_start(&expanded_map);

    for (directions.items) |dir| {
        curr = try move_expanded(&expanded_map, curr, dir);
    }
    const total2 = try get_boxes_coordinates_sum(&expanded_map);

    return .{ total1, total2 };
}

fn move(map: *utils.Matrix(u8), pos: Vec2, dir: Direction) !Vec2 {
    var np = pos;
    np.step(dir);

    var nnp = np;

    var boxes: u32 = 0;
    while (try map.get(nnp.y, nnp.x) == 'O') {
        boxes += 1;
        nnp.step(dir);
    }

    const v = try map.get(nnp.y, nnp.x);

    if (v == '.') {
        try map.set(pos.y, pos.x, '.');
        try map.set(np.y, np.x, '@');

        if (boxes > 0) {
            try map.set(nnp.y, nnp.x, 'O');
        }
        return np;
    }

    return pos;
}

fn move_expanded(map: *utils.Matrix(u8), pos: Vec2, dir: Direction) !Vec2 {
    if (try can_move(map, pos, dir)) {
        var np = pos;
        np.step(dir);
        try do_move_expanded(map, np, '@', dir);
        try map.set(pos.y, pos.x, '.');
        return np;
    } else {
        return pos;
    }
}

fn can_move(map: *utils.Matrix(u8), pos: Vec2, dir: Direction) !bool {
    var np = pos;
    np.step(dir);

    const v = try map.get(np.y, np.x);
    return switch (v) {
        '#' => false,
        '.' => true,
        '[' => {
            if (dir == Direction.DOWN or dir == Direction.UP) {
                return try can_move(map, np, dir) and try can_move(map, Vec2{ .x = np.x + 1, .y = np.y }, dir);
            } else {
                return try can_move(map, np, dir);
            }
        },
        ']' => {
            if (dir == Direction.DOWN or dir == Direction.UP) {
                return try can_move(map, np, dir) and try can_move(map, Vec2{ .x = np.x - 1, .y = np.y }, dir);
            } else {
                return try can_move(map, np, dir);
            }
        },
        else => unreachable,
    };
}

fn do_move_expanded(map: *utils.Matrix(u8), pos: Vec2, vp: u8, dir: Direction) !void {
    const v = try map.get(pos.y, pos.x);
    try map.set(pos.y, pos.x, vp);

    var np = pos;
    np.step(dir);

    switch (v) {
        '[' => {
            try do_move_expanded(map, np, v, dir);
            if (dir == Direction.DOWN or dir == Direction.UP) {
                try map.set(pos.y, pos.x + 1, '.');
                try do_move_expanded(map, Vec2{ .x = np.x + 1, .y = np.y }, ']', dir);
            }
        },
        ']' => {
            try do_move_expanded(map, np, v, dir);
            if (dir == Direction.DOWN or dir == Direction.UP) {
                try map.set(pos.y, pos.x - 1, '.');
                try do_move_expanded(map, Vec2{ .x = np.x - 1, .y = np.y }, '[', dir);
            }
        },
        else => {},
    }
}

fn get_boxes_coordinates_sum(map: *utils.Matrix(u8)) !u32 {
    var sum: usize = 0;

    for (0..map.height) |row| {
        for (0..map.width) |col| {
            const v = try map.get(row, col);
            if (v == 'O' or v == '[') {
                sum += 100 * row + col;
            }
        }
    }

    return @intCast(sum);
}

fn find_start(map: *utils.Matrix(u8)) !Vec2 {
    var start = std.mem.zeroes(Vec2);

    for (0..map.height) |row| {
        for (0..map.width) |col| {
            if (try map.get(row, col) == '@') {
                start = .{ .x = col, .y = row };
            }
        }
    }

    return start;
}

fn expand_map(map: *utils.Matrix(u8), allocator: std.mem.Allocator) !utils.Matrix(u8) {
    var new_map = utils.Matrix(u8).init(@intCast(map.width * 2), @intCast(map.height), allocator);

    var ncol: usize = 0;
    for (0..map.height) |row| {
        for (0..map.width) |col| {
            const v = try map.get(row, col);
            switch (v) {
                '@' => {
                    try new_map.set(row, ncol, v);
                    try new_map.set(row, ncol + 1, '.');
                },
                'O' => {
                    try new_map.set(row, ncol, '[');
                    try new_map.set(row, ncol + 1, ']');
                },
                '.', '#' => {
                    try new_map.set(row, ncol, v);
                    try new_map.set(row, ncol + 1, v);
                },
                else => {},
            }
            ncol += 2;
        }
        ncol = 0;
    }

    return new_map;
}

test "example 1" {
    const result = try solution(std.testing.allocator, "input/15_input_test1.txt");

    try std.testing.expectEqual(@as(u32, 2028), result[0]);
}

test "example 2" {
    const result = try solution(std.testing.allocator, "input/15_input_test2.txt");

    try std.testing.expectEqual(@as(u32, 10092), result[0]);
    try std.testing.expectEqual(@as(u32, 9021), result[1]);
}

test "example 3" {
    const result = try solution(std.testing.allocator, "input/15_input_test3.txt");

    try std.testing.expectEqual(@as(u32, 618), result[1]);
}
