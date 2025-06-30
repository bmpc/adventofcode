const std = @import("std");
const utils = @import("utils.zig");

const assert = std.debug.assert;
const Tuple = std.meta.Tuple(&.{ u32, Vec2 });

const Vec2 = struct {
    x: usize,
    y: usize,

    pub fn eql(self: @This(), other: @This()) bool {
        return self.x == other.x and self.y == other.y;
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        _ = gpa.deinit();
    }
    const allocator = gpa.allocator();

    const result = try solution(allocator, "input/18_input.txt", 71, 1024);

    std.debug.print("Part 1: {d}\n", .{result[0]});
    std.debug.print("Part 2: {},{}\n", .{ result[1].x, result[1].y });
}

fn solution(allocator: std.mem.Allocator, input: []const u8, gridSize: u32, n: u32) !Tuple {
    const data = try utils.readFile(&allocator, input);
    defer allocator.free(data);

    var positions = std.ArrayList(Vec2).init(allocator);
    defer positions.deinit();

    var lines_it = std.mem.splitScalar(u8, data, '\n');
    while (lines_it.next()) |line| {
        var it = std.mem.splitScalar(u8, line, ',');
        const sx = it.next().?;
        const sy = it.next().?;

        const px = try std.fmt.parseInt(usize, sx, 10);
        const py = try std.fmt.parseInt(usize, sy, 10);

        try positions.append(Vec2{ .x = px, .y = py });
    }

    var map = utils.Matrix(u8).initWithElem(gridSize, gridSize, '.', allocator);
    defer map.deinit();

    // part 1
    const min = try check(n, &map, positions, Vec2{ .x = 0, .y = 0 }, allocator);

    // part 2 / apply binary search to improve speed
    var total: u32 = @intCast(positions.items.len);
    var blocked: Vec2 = Vec2{ .x = 0, .y = 0 };

    var needle: u32 = total / 2;
    while (total > 0) {
        const res = try check(needle + 1, &map, positions, Vec2{ .x = 0, .y = 0 }, allocator);

        if (res == 0) {
            if (positions.items.len % 2 == 0) {
                blocked = positions.items[needle - 1];
            } else {
                blocked = positions.items[needle];
            }
            needle = needle - (total / 2);
        } else {
            needle = needle + (total / 2);
        }
        total /= 2;
    }

    return .{ min, blocked };
}

fn check(n: u32, map: *utils.Matrix(u8), positions: std.ArrayList(Vec2), start: Vec2, allocator: std.mem.Allocator) !u32 {
    var tmap = try map.clone(allocator);
    defer tmap.deinit();

    for (positions.items, 0..) |pos, i| {
        try tmap.set(pos.y, pos.x, '#');
        if (i == n - 1) break;
    }

    return try bfs(&tmap, start, allocator);
}

// Breadth-first search algorithm
fn bfs(map: *utils.Matrix(u8), start: Vec2, allocator: std.mem.Allocator) !u32 {
    const Node = struct { pos: Vec2, count: u32 };

    var queue = std.ArrayList(Node).init(allocator);
    defer queue.deinit();
    var visited = std.AutoHashMap(Vec2, void).init(allocator);
    defer visited.deinit();

    // start position
    try queue.append(Node{ .pos = start, .count = 0 });
    try visited.put(start, {});

    while (queue.items.len > 0) {
        const nn = queue.orderedRemove(0);
        if (std.meta.eql(nn.pos, Vec2{ .x = map.width - 1, .y = map.height - 1 })) {
            return nn.count;
        }

        const neighbors = try getNeighbors(map, nn.pos, allocator);
        defer neighbors.deinit();

        for (neighbors.items) |neighbor| {
            if (!visited.contains(neighbor)) {
                try visited.put(neighbor, {});
                try queue.append(Node{ .pos = neighbor, .count = nn.count + 1 });
            }
        }
    }
    return 0;
}

fn getNeighbors(map: *utils.Matrix(u8), pos: Vec2, allocator: std.mem.Allocator) !std.ArrayList(Vec2) {
    var neighbors = try std.ArrayList(Vec2).initCapacity(allocator, 4);

    // left
    if (pos.x > 0) {
        const v = try map.get(pos.y, pos.x - 1);
        if (v != '#') {
            try neighbors.append(Vec2{ .y = pos.y, .x = pos.x - 1 });
        }
    }
    // right
    if (pos.x < map.width - 1) {
        const v = try map.get(pos.y, pos.x + 1);
        if (v != '#') {
            try neighbors.append(Vec2{ .y = pos.y, .x = pos.x + 1 });
        }
    }

    // up
    if (pos.y > 0) {
        const v = try map.get(pos.y - 1, pos.x);
        if (v != '#') {
            try neighbors.append(Vec2{ .y = pos.y - 1, .x = pos.x });
        }
    }

    //down
    if (pos.y < map.height - 1) {
        const v = try map.get(pos.y + 1, pos.x);
        if (v != '#') {
            try neighbors.append(Vec2{ .y = pos.y + 1, .x = pos.x });
        }
    }

    return neighbors;
}

test "example 1" {
    const result = try solution(std.testing.allocator, "input/18_input_test.txt", 7, 12);

    try std.testing.expectEqual(@as(u32, 22), result[0]);
    try std.testing.expectEqual(@as(Vec2, Vec2{ .x = 6, .y = 1 }), result[1]);
}
