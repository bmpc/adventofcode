const std = @import("std");
const utils = @import("utils.zig");

const assert = std.debug.assert;
const Tuple = std.meta.Tuple(&.{ u32, u32 });

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const result = try solution(allocator, "input/01_input.txt");

    std.debug.print("Part 1: {d}\n", .{result[0]});
    std.debug.print("Part 2: {d}\n", .{result[1]});
}

fn solution(allocator: std.mem.Allocator, input: []const u8) !Tuple {
    const content = try utils.readFile(&allocator, input);
    defer allocator.free(content);

    var lines = std.mem.splitSequence(u8, content, "\n");

    var list_a = std.ArrayList(i32).init(allocator);
    defer list_a.deinit();

    var list_b = std.ArrayList(i32).init(allocator);
    defer list_b.deinit();

    while (lines.next()) |line| {
        var pair = std.mem.splitSequence(u8, line, "   ");
        const str_a = pair.next();
        assert(str_a != null);
        const str_b = pair.next();
        assert(str_b != null);

        const a = try std.fmt.parseInt(i32, str_a.?, 10);
        const b = try std.fmt.parseInt(i32, str_b.?, 10);

        try list_a.append(a);
        try list_b.append(b);
    }

    std.mem.sort(i32, list_a.items, {}, std.sort.asc(i32));
    std.mem.sort(i32, list_b.items, {}, std.sort.asc(i32));

    const count = list_a.items.len;

    var total1: u32 = 0;
    for (0..count) |idx| {
        total1 += @abs(list_a.items[idx] - list_b.items[idx]);
    }

    var total2: u32 = 0;
    for (list_a.items) |a| {
        var v_count: i32 = 0;
        for (list_b.items) |b| {
            if (a == b) {
                v_count += 1;
            } else if (v_count > 0) {
                break;
            }
        }
        total2 += @intCast(a * v_count);
        v_count = 0;
    }

    return .{ total1, total2 };
}

test "example" {
    const result = try solution(std.testing.allocator, "input/01_input_test.txt");

    try std.testing.expectEqual(@as(u32, 11), result[0]);
    try std.testing.expectEqual(@as(u32, 31), result[1]);
}
