const std = @import("std");
const utils = @import("utils.zig");

const assert = std.debug.assert;
const Tuple = std.meta.Tuple(&.{ u32, u32 });

const Order = enum { nil, asc, desc };

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const result = try solution(allocator, "input/02_input.txt");

    std.debug.print("Part 1: {d}\n", .{result[0]});
    std.debug.print("Part 2: {d}\n", .{result[1]});
}

fn solution(allocator: std.mem.Allocator, input: []const u8) !Tuple {
    const content = try utils.readFile(&allocator, input);
    defer allocator.free(content);

    var lines = std.mem.split(u8, content, "\n");

    var reports = std.ArrayList(std.ArrayList(u8)).init(allocator);
    defer reports.deinit();
    defer for (reports.items) |item| {
        item.deinit();
    };

    while (lines.next()) |line| {
        var report_str = std.mem.split(u8, line, " ");

        var levels = std.ArrayList(u8).init(allocator);
        while (report_str.next()) |level_str| {
            const level = try std.fmt.parseInt(u8, level_str, 10);
            try levels.append(level);
        }
        try reports.append(levels);
    }

    var safe_count: u32 = 0;
    var safe_damp_count: u32 = 0;
    for (reports.items) |rep| {
        if (isReportSafe(rep, null)) {
            safe_count += 1;
        } else {
            for (0..rep.items.len) |i| {
                if (isReportSafe(rep, i)) {
                    safe_damp_count += 1;
                    break;
                }
            }
        }
    }

    return .{ safe_count, safe_count + safe_damp_count };
}

fn isReportSafe(report: std.ArrayList(u8), skipLevel: ?usize) bool {
    const idx: u8 = if (skipLevel != null and skipLevel == 0) 1 else 0;

    var curr_level: u8 = report.items[idx];
    var inc: ?bool = null;

    for (report.items[(idx + 1)..], (idx + 1)..) |level, i| {
        if (skipLevel != null and idx + i == skipLevel.?) continue;
        if (inc == null) {
            if (level == curr_level) {
                return false;
            }
            inc = level > curr_level;
        }

        if (inc.?) {
            if (level <= curr_level) {
                return false;
            } else if (level - curr_level > 3) {
                return false;
            }
        } else {
            if (level >= curr_level) {
                return false;
            } else if (curr_level - level > 3) {
                return false;
            }
        }

        curr_level = level;
    }

    return true;
}

fn printReport(report: std.ArrayList(u8), safe: bool) void {
    for (report.items) |level| {
        std.debug.print("{} ", .{level});
    }
    std.debug.print(" = {}\n", .{safe});
}

test "example" {
    const result = try solution(std.testing.allocator, "input/02_input_test.txt");

    try std.testing.expectEqual(@as(u32, 2), result[0]);
    try std.testing.expectEqual(@as(u32, 5), result[1]);
}
