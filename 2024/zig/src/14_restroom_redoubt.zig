const std = @import("std");
const utils = @import("utils.zig");

const assert = std.debug.assert;
const Tuple = std.meta.Tuple(&.{ u32, u32 });

const Vec2 = struct { x: i32, y: i32 };

const Robot = struct { p: Vec2, v: Vec2 };

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const result = try solution(allocator, "input/14_input.txt", Vec2{ .x = 101, .y = 103 });

    std.debug.print("Part 1: {d}\n", .{result[0]});
    std.debug.print("Part 2: {d}\n", .{result[1]});
}

fn solution(allocator: std.mem.Allocator, input: []const u8, size: Vec2) !Tuple {
    const data = try utils.readFile(&allocator, input);
    defer allocator.free(data);

    var lines_it = std.mem.splitSequence(u8, data, "\n");

    var robots: std.ArrayList(Robot) = std.ArrayList(Robot).init(allocator);
    defer robots.deinit();

    while (lines_it.next()) |line| {
        var it = std.mem.splitSequence(u8, line, " ");
        var p = it.next().?;
        var v = it.next().?;

        it = std.mem.splitSequence(u8, p[2..], ",");
        const px = try std.fmt.parseInt(i32, it.next().?, 10);
        const py = try std.fmt.parseInt(i32, it.next().?, 10);
        it = std.mem.splitSequence(u8, v[2..], ",");
        const vx = try std.fmt.parseInt(i32, it.next().?, 10);
        const vy = try std.fmt.parseInt(i32, it.next().?, 10);

        try robots.append(Robot{ .p = Vec2{ .x = px, .y = py }, .v = Vec2{ .x = vx, .y = vy } });
    }

    var robots2 = try robots.clone();
    defer robots2.deinit();

    // part 1
    for (0..100) |_| {
        for (robots.items) |*robot| {
            move_robot(robot, size);
        }
    }

    const lx = @divFloor(size.x, @as(i32, 2));
    const ly = @divFloor(size.y, @as(i32, 2));

    // q1 | q2
    // -------
    // q3 | q4
    var q1: u32 = 0;
    var q2: u32 = 0;
    var q3: u32 = 0;
    var q4: u32 = 0;

    for (robots.items) |robot| {
        if (robot.p.x < lx) {
            if (robot.p.y < ly) {
                q1 += 1;
            } else if (robot.p.y > ly) {
                q3 += 1;
            }
        } else if (robot.p.x > lx) {
            if (robot.p.y < ly) {
                q2 += 1;
            } else if (robot.p.y > ly) {
                q4 += 1;
            }
        }
    }

    // part 2
    var map = utils.Matrix(u8).init(@intCast(size.x), @intCast(size.y), allocator);
    defer map.deinit();

    var easter_egg_it: u32 = 0;

    for (0..10000) |iteration| {
        map.clear('.');
        for (robots2.items) |*robot| {
            move_robot(robot, size);
            try map.set(@intCast(robot.p.y), @intCast(robot.p.x), '#');
        }
        if (try print_christmas_tree_candidate(&map)) {
            easter_egg_it = @intCast(iteration);
        }
    }

    return .{ q1 * q2 * q3 * q4, easter_egg_it + 1 };
}

fn move_robot(robot: *Robot, size: Vec2) void {
    var px = robot.p.x + robot.v.x;
    var py = robot.p.y + robot.v.y;

    if (px < 0) {
        px = size.x - @as(i32, @intCast(@abs(px)));
    }

    if (py < 0) {
        py = size.y - @as(i32, @intCast(@abs(py)));
    }

    if (px > size.x - 1) {
        px = px - size.x;
    }

    if (py > size.y - 1) {
        py = py - size.y;
    }

    robot.p.x = px;
    robot.p.y = py;
}

fn print_christmas_tree_candidate(map: *utils.Matrix(u8)) !bool {
    // detect christmas tree
    var acc: u32 = 0;
    for (0..map.height) |row| {
        for (0..map.width) |col| {
            const ch = map.get(row, col) catch unreachable;
            if (ch == '.') {
                acc = 0;
            } else {
                acc += 1;
            }

            if (acc > 10) {
                break;
            }
        }

        if (acc > 10) {
            break;
        }
        acc = 0;
    }

    if (acc > 10) {
        map.print();
        return true;
    }

    return false;
}

test "example" {
    const result = try solution(std.testing.allocator, "input/14_input_test.txt", Vec2{ .x = 11, .y = 7 });

    try std.testing.expectEqual(@as(u32, 12), result[0]);
}
