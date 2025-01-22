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

    pub fn eql(self: @This(), other: @This()) bool {
        return self.x == other.x and self.y == other.y;
    }
};

const Node = struct {
    tile: u8,
    pos: Vec2,
    dir: Direction,
    w: u32,
    parent: ?*Node,

    fn print(self: Node) void {
        std.debug.print("({},{}) = {}\n", .{ self.pos.y, self.pos.x, self.w });
    }
};

const PriorityQueue = std.PriorityQueue(*Node, void, compare);

fn compare(_: void, a: *Node, b: *Node) std.math.Order {
    return std.math.order(a.w, b.w);
}

const Direction = enum(u8) { UP, DOWN, LEFT, RIGHT };

fn getDirsFrom(dir: Direction) [3]Direction {
    return switch (dir) {
        Direction.LEFT => .{ Direction.LEFT, Direction.UP, Direction.DOWN },
        Direction.RIGHT => .{ Direction.RIGHT, Direction.UP, Direction.DOWN },
        Direction.UP => .{ Direction.RIGHT, Direction.UP, Direction.LEFT },
        Direction.DOWN => .{ Direction.RIGHT, Direction.LEFT, Direction.DOWN },
    };
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        _ = gpa.deinit();
    }
    const allocator = gpa.allocator();

    const result = try solution(allocator, "input/16_input.txt");

    std.debug.print("Part 1: {d}\n", .{result[0]});
    std.debug.print("Part 2: {d}\n", .{result[1]});
}

fn solution(allocator: std.mem.Allocator, input: []const u8) !Tuple {
    const data = try utils.readFile(&allocator, input);
    defer allocator.free(data);

    var map = try utils.Matrix(u8).initFromSequence(data, "\n", allocator);
    defer map.deinit();

    const pos = try findStart(&map);
    const result = (try lowestScore(&map, pos, Direction.RIGHT, allocator));

    return result;
}

// using Dijkstra's
fn lowestScore(map: *utils.Matrix(u8), pos: Vec2, dir: Direction, allocator: std.mem.Allocator) !Tuple {
    var pq = PriorityQueue.init(allocator, {});
    defer pq.deinit();

    var visited = std.AutoHashMap(Vec2, *Node).init(allocator);
    defer visited.deinit();

    var node_arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer node_arena.deinit();
    const arena_allocator = node_arena.allocator();

    const sn = try arena_allocator.create(Node);
    sn.* = .{ .pos = pos, .w = 0, .dir = dir, .tile = 'S', .parent = null };
    try pq.add(sn);

    var found_e = false;
    var w: u32 = 0;

    var node_map = std.AutoHashMap(Vec2, bool).init(allocator);
    defer node_map.deinit();

    while (pq.removeOrNull()) |n| {
        if (found_e and n.w > w) continue;

        const tile = try map.get(n.pos.y, n.pos.x);
        if (tile == 'E') {
            found_e = true;
            w = n.w;

            var nn = n;
            try node_map.put(nn.pos, true);
            while (nn.parent) |pn| {
                try node_map.put(pn.pos, true);
                nn = pn;
            }
        }

        try visited.put(n.pos, n);
        const nodes = try getNeighbors(map, n, arena_allocator);
        for (nodes) |nn| {
            if (nn != null) {
                if (visited.get(nn.?.pos)) |vn| {
                    if (vn.parent != null and nn.?.parent != null) {
                        if (vn.parent.?.pos.eql(nn.?.parent.?.pos)) {
                            continue;
                        }
                    }
                }
                try pq.add(nn.?);
            }
        }
    }

    return .{ w, node_map.count() };
}

fn getNeighbors(map: *utils.Matrix(u8), node: *Node, allocator: std.mem.Allocator) ![3]?*Node {
    var neighbors = std.mem.zeroes([3]?*Node);
    var idx: usize = 0;

    const all_dirs = getDirsFrom(node.dir);
    for (all_dirs) |d| {
        var w = node.w;
        var np = node.pos;
        np.step(d);
        const tile = try map.get(np.y, np.x);
        if (tile != '#') {
            if (node.dir == d) {
                w += 1;
            } else {
                w += 1001;
            }

            const an = try allocator.create(Node);
            an.* = Node{ .tile = tile, .pos = np, .dir = d, .w = w, .parent = node };

            neighbors[idx] = an;
            idx += 1;
        }
    }

    return neighbors;
}

fn findStart(map: *utils.Matrix(u8)) !Vec2 {
    var start = std.mem.zeroes(Vec2);

    for (0..map.height) |row| {
        for (0..map.width) |col| {
            if (try map.get(row, col) == 'S') {
                start = .{ .x = col, .y = row };
            }
        }
    }

    return start;
}

test "example 1" {
    const result = try solution(std.testing.allocator, "input/16_input_test1.txt");

    try std.testing.expectEqual(@as(u32, 7036), result[0]);
    try std.testing.expectEqual(@as(u32, 45), result[1]);
}

test "example 2" {
    const result = try solution(std.testing.allocator, "input/16_input_test2.txt");

    try std.testing.expectEqual(@as(u32, 11048), result[0]);
    try std.testing.expectEqual(@as(u32, 64), result[1]);
}
