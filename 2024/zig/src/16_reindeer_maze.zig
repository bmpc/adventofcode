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

const Node = struct { tile: u8, pos: Vec2, dir: Direction, w: u32 };

const PriorityQueue = std.PriorityQueue(Node, void, compare);

fn compare(_: void, a: Node, b: Node) std.math.Order {
    return std.math.order(a.w, b.w);
}

const Direction = enum(u8) { UP, DOWN, LEFT, RIGHT };

const Move = std.meta.Tuple(&.{ Direction, Vec2 });
const MoveDirections = struct {
    left: ?Vec2,
    right: ?Vec2,
    up: ?Vec2,
    down: ?Vec2,

    fn set(self: *MoveDirections, dir: Direction, pos: Vec2) void {
        switch (dir) {
            Direction.LEFT => {
                self.left = pos;
            },
            Direction.RIGHT => {
                self.right = pos;
            },
            Direction.UP => {
                self.up = pos;
            },
            Direction.DOWN => {
                self.down = pos;
            },
        }
    }

    fn getSingle(self: MoveDirections) ?Move {
        if (self.left) |left| {
            return .{ Direction.LEFT, left };
        }
        if (self.right) |right| {
            return .{ Direction.RIGHT, right };
        }
        if (self.up) |up| {
            return .{ Direction.UP, up };
        }
        if (self.down) |down| {
            return .{ Direction.DOWN, down };
        }

        return null;
    }

    fn count(self: MoveDirections) u32 {
        var c: u32 = 0;
        if (self.left != null) {
            c += 1;
        }
        if (self.right != null) {
            c += 1;
        }
        if (self.up != null) {
            c += 1;
        }
        if (self.down != null) {
            c += 1;
        }
        return c;
    }
};

fn getDirsFrom(dir: Direction) [3]Direction {
    return switch (dir) {
        Direction.LEFT => .{ Direction.LEFT, Direction.UP, Direction.DOWN },
        Direction.RIGHT => .{ Direction.RIGHT, Direction.UP, Direction.DOWN },
        Direction.UP => .{ Direction.RIGHT, Direction.UP, Direction.LEFT },
        Direction.DOWN => .{ Direction.RIGHT, Direction.LEFT, Direction.DOWN },
    };
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const result = try solution(allocator, "input/16_input.txt");

    std.debug.print("Part 1: {d}\n", .{result[0]});
    std.debug.print("Part 2: {d}\n", .{result[1]});
}

fn solution(allocator: std.mem.Allocator, input: []const u8) !Tuple {
    const data = try utils.readFile(&allocator, input);
    defer allocator.free(data);

    var map = try utils.Matrix(u8).initFromSequence(data, "\n", allocator);
    defer map.deinit();

    // part 1
    const pos = try findStart(&map);
    const total1 = (try lowestScore(&map, pos, Direction.RIGHT, allocator)).?;

    // TODO: part 2

    return .{ total1, 0 };
}

// using Dijkstra's
fn lowestScore(map: *utils.Matrix(u8), pos: Vec2, dir: Direction, allocator: std.mem.Allocator) !?u32 {
    var pq = PriorityQueue.init(allocator, {});
    defer pq.deinit();

    var visited = std.AutoHashMap(Vec2, bool).init(allocator);
    defer visited.deinit();

    try pq.add(.{ .pos = pos, .w = 0, .dir = dir, .tile = 'S' });

    while (pq.removeOrNull()) |n| {
        try visited.put(n.pos, true);

        const tile = try map.get(n.pos.y, n.pos.x);
        if (tile == 'E') {
            return n.w;
        }
        const nodes = try getNeighbors(map, n);
        if (nodes != null) {
            for (nodes.?) |nn| {
                if (nn.pos.x != 0 and nn.pos.y != 0 and !visited.contains(nn.pos)) {
                    try pq.add(nn);
                }
            }
        }
    }

    return null;
}

fn getNeighbors(map: *utils.Matrix(u8), node: Node) !?[3]Node {
    var neighbors = std.mem.zeroes([3]Node);
    var idx: usize = 0;

    var w = node.w;
    var dir = node.dir;
    var dirs = try getPossibleDirs(map, node.pos, node.dir);
    while (dirs.count() == 1) {
        const m = dirs.getSingle().?;
        if (dir == m[0]) {
            w += 1;
        } else {
            w += 1001;
            dir = m[0];
        }

        const tile = try map.get(m[1].y, m[1].x);

        if (tile == 'E') {
            neighbors[idx] = Node{ .tile = tile, .pos = m[1], .dir = m[0], .w = w };
            return neighbors;
        }
        dirs = try getPossibleDirs(map, m[1], m[0]);
    }

    if (dirs.count() > 1) {
        // found a branch

        if (dirs.left) |p| {
            const tile = try map.get(p.y, p.x);
            var nw = w;
            if (dir == Direction.LEFT) nw += 1 else nw += 1001;

            neighbors[idx] = .{ .tile = tile, .pos = p, .dir = Direction.LEFT, .w = nw };
            idx += 1;
        }
        if (dirs.right) |p| {
            const tile = try map.get(p.y, p.x);
            var nw = w;
            if (dir == Direction.RIGHT) nw += 1 else nw += 1001;

            neighbors[idx] = .{ .tile = tile, .pos = p, .dir = Direction.RIGHT, .w = nw };
            idx += 1;
        }
        if (dirs.up) |p| {
            const tile = try map.get(p.y, p.x);
            var nw = w;
            if (dir == Direction.UP) nw += 1 else nw += 1001;

            neighbors[idx] = .{ .tile = tile, .pos = p, .dir = Direction.UP, .w = nw };
            idx += 1;
        }
        if (dirs.down) |p| {
            const tile = try map.get(p.y, p.x);
            var nw = w;
            if (dir == Direction.DOWN) nw += 1 else nw += 1001;

            neighbors[idx] = .{ .tile = tile, .pos = p, .dir = Direction.DOWN, .w = nw };
            idx += 1;
        }

        return neighbors;
    } else {
        // dead end
        return null;
    }
}

fn getPossibleDirs(map: *utils.Matrix(u8), pos: Vec2, dir: Direction) !MoveDirections {
    var res = std.mem.zeroes(MoveDirections);

    const all_dirs = getDirsFrom(dir);
    for (all_dirs) |d| {
        var np = pos;
        np.step(d);
        const tile = try map.get(np.y, np.x);
        if (tile != '#') {
            res.set(d, np);
        }
    }

    return res;
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
}

test "example 2" {
    const result = try solution(std.testing.allocator, "input/16_input_test2.txt");

    try std.testing.expectEqual(@as(u32, 11048), result[0]);
}
