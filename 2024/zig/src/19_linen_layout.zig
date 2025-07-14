const std = @import("std");
const utils = @import("utils.zig");

const assert = std.debug.assert;
const Tuple = std.meta.Tuple(&.{ u32, u64 });

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        _ = gpa.deinit();
    }
    const allocator = gpa.allocator();

    const result = try solution(allocator, "input/19_input.txt");

    std.debug.print("Part 1: {d}\n", .{result[0]});
    std.debug.print("Part 2: {d}\n", .{result[1]});
}

fn solution(allocator: std.mem.Allocator, input: []const u8) !Tuple {
    const data = try utils.readFile(&allocator, input);
    defer allocator.free(data);

    var towels = std.StringHashMap(bool).init(allocator);
    defer towels.deinit();

    var designs = std.ArrayList([]const u8).init(allocator);
    defer designs.deinit();

    var lines_it = std.mem.splitScalar(u8, data, '\n');

    // parse towels line
    const towels_line = lines_it.next().?;

    var max_towel: usize = 0;

    var it = std.mem.splitScalar(u8, towels_line, ',');
    while (it.next()) |towel| {
        const tt = std.mem.trim(u8, towel, " ");
        if (tt.len > max_towel) {
            max_towel = tt.len;
        }
        try towels.put(tt, true);
    }

    const sep = lines_it.next().?; // empty line separator
    assert(std.mem.eql(u8, sep, ""));

    // parse designs
    while (lines_it.next()) |design| {
        try designs.append(design);
    }

    // solution
    var count: u32 = 0;
    var total_count: u64 = 0;

    var cache = std.StringHashMap(u64).init(allocator);
    defer cache.deinit();

    for (designs.items) |design| {
        const dc = try count_design_solutions(towels, max_towel, design, &cache);
        total_count += dc;
        if (dc > 0) {
            count += 1;
        }
        //std.debug.print("Design {s} = {}\n", .{ design, dc });
    }

    return .{ count, total_count };
}

fn count_design_solutions(towels: std.StringHashMap(bool), max_towel: usize, design: []const u8, cache: *std.StringHashMap(u64)) !u64 {
    var count: u64 = 0;
    var wd: []const u8 = design;

    if (cache.contains(design)) {
        return cache.get(design).?;
    }

    if (design.len > max_towel) {
        wd = design[0..max_towel];
    }

    while (wd.len > 0) {
        if (towels.contains(wd)) {
            if (design.len > wd.len) {
                count += try count_design_solutions(towels, max_towel, design[wd.len..], cache);
            } else {
                count += 1; // found last fragment
                try cache.put(design, count);
            }
        }

        if (wd.len > 1) {
            wd = wd[0..(wd.len - 1)];
        } else {
            try cache.put(design, count);
            break;
        }
    }

    return count;
}

test "example 1" {
    const result = try solution(std.testing.allocator, "input/19_input_test1.txt");

    try std.testing.expectEqual(@as(u32, 6), result[0]);
    try std.testing.expectEqual(@as(u32, 16), result[1]);
}

test "example 2" {
    const result = try solution(std.testing.allocator, "input/19_input_test2.txt");

    try std.testing.expectEqual(@as(u32, 7), result[0]);
}

test "example 3" {
    const result = try solution(std.testing.allocator, "input/19_input_test3.txt");

    try std.testing.expectEqual(@as(u32, 1), result[0]);
}
