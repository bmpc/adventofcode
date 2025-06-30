const std = @import("std");
const utils = @import("utils.zig");

const assert = std.debug.assert;
fn Tuple(comptime T: type) type {
    return std.meta.Tuple(&.{ T, T });
}

const PRIZE_FIX_PART2 = 10000000000000;

const Vec2 = struct { x: i64, y: i64 };

const Machine = struct { a: Vec2, b: Vec2, prize: Vec2 };

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const result = try solution(allocator, "input/13_input.txt");

    std.debug.print("Part 1: {d}\n", .{result[0]});
    std.debug.print("Part 2: {d}\n", .{result[1]});
}

fn solution(allocator: std.mem.Allocator, input: []const u8) !Tuple(u64) {
    const data = try utils.readFile(&allocator, input);
    defer allocator.free(data);

    var lines_it = std.mem.splitSequence(u8, data, "\n");

    var machines: std.ArrayList(Machine) = std.ArrayList(Machine).init(allocator);
    defer machines.deinit();

    var ax: i64 = 0;
    var ay: i64 = 0;
    var bx: i64 = 0;
    var by: i64 = 0;
    while (lines_it.next()) |line| {
        if (std.mem.startsWith(u8, line, "Button A:")) {
            ax = try std.fmt.parseInt(i64, line[12..14], 10);
            ay = try std.fmt.parseInt(i64, line[18..20], 10);
        } else if (std.mem.startsWith(u8, line, "Button B:")) {
            bx = try std.fmt.parseInt(i64, line[12..14], 10);
            by = try std.fmt.parseInt(i64, line[18..20], 10);
        } else if (std.mem.startsWith(u8, line, "Prize: ")) {
            var it = std.mem.splitSequence(u8, line[7..], ", ");
            const px = try std.fmt.parseInt(i64, (it.next().?)[2..], 10);
            const py = try std.fmt.parseInt(i64, (it.next().?)[2..], 10);

            const m = Machine{ .a = Vec2{ .x = ax, .y = ay }, .b = Vec2{ .x = bx, .y = by }, .prize = Vec2{ .x = px, .y = py } };

            try machines.append(m);
        }
    }

    var total1: u64 = 0;
    for (machines.items) |m| {
        const res = try solve_min_tokens(m, false);

        if (res != null) {
            total1 += res.?[0] * 3 + res.?[1];
        }
    }

    var total2: u64 = 0;
    for (machines.items) |m| {
        const res = try solve_min_tokens(m, true);

        if (res != null) {
            total2 += res.?[0] * 3 + res.?[1];
        }
    }

    return .{ total1, total2 };
}

fn solve_min_tokens(machine: Machine, prize_fix: bool) !?Tuple(u64) {
    var p = machine.prize;
    const a = machine.a;
    const b = machine.b;

    if (prize_fix) {
        p.x += PRIZE_FIX_PART2;
        p.y += PRIZE_FIX_PART2;
    }

    const m = @as(f64, @floatFromInt(a.x * p.y - a.y * p.x)) / @as(f64, @floatFromInt(a.x * b.y - a.y * b.x));
    if (m - @floor(m) > 0.0) return null;

    const n = @as(f64, @floatFromInt(p.x - b.x * @as(i64, @intFromFloat(m)))) / @as(f64, @floatFromInt(a.x));
    if (n - @floor(n) > 0.0) return null;

    return .{ @intFromFloat(n), @intFromFloat(m) };
}

test "example" {
    const result = try solution(std.testing.allocator, "input/13_input_test.txt");

    try std.testing.expectEqual(@as(u64, 480), result[0]);
    try std.testing.expectEqual(@as(u64, 875318608908), result[1]);
}
