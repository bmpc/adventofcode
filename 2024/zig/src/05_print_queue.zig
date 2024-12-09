const std = @import("std");
const utils = @import("utils.zig");

const assert = std.debug.assert;
const Tuple = std.meta.Tuple(&.{ u32, u32 });

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const result = try solution(allocator, "input/05_input.txt");

    std.debug.print("Part 1: {d}\n", .{result[0]});
    std.debug.print("Part 2: {d}\n", .{result[1]});
}

fn solution(allocator: std.mem.Allocator, input: []const u8) !Tuple {
    const data = try utils.readFile(&allocator, input);
    defer allocator.free(data);

    var lines = std.mem.split(u8, data, "\n");

    var page_ordering = std.StringHashMap(bool).init(allocator);
    defer page_ordering.deinit();

    var pages_updates = std.ArrayList([][]const u8).init(allocator);
    defer {
        for (pages_updates.items) |update| {
            allocator.free(update);
        }

        pages_updates.deinit();
    }

    // page ordering
    while (lines.next()) |rule| {
        if (rule.len == 0) break;

        try page_ordering.put(rule, true);
    }

    //page updates
    while (lines.next()) |update| {
        if (update.len == 0) break;

        var update_l = std.ArrayList([]const u8).init(allocator);

        var pages = std.mem.split(u8, update, ",");
        while (pages.next()) |page| {
            try update_l.append(page);
        }

        assert(update_l.items.len % 2 != 0); // odd number of pages

        try pages_updates.append(try update_l.toOwnedSlice());
    }

    var total1: u32 = 0;
    var total2: u32 = 0;

    for (pages_updates.items) |update| {
        var good: bool = false;
        var fixed: bool = false;
        while (!good) {
            good = try checkUpdate(update, page_ordering, allocator);
            if (!good) fixed = true;
        }

        if (fixed) {
            const mid = update[(update.len / 2)];
            total2 += try std.fmt.parseInt(u32, mid, 10);
        } else {
            const mid = update[(update.len / 2)];
            total1 += try std.fmt.parseInt(u32, mid, 10);
        }
    }

    return .{ total1, total2 };
}

fn checkUpdate(update: [][]const u8, page_ordering: std.StringHashMap(bool), allocator: std.mem.Allocator) !bool {
    var good: bool = true;
    var prev_page = update[0];

    for (update[1..], 1..) |page, i| {
        const key = try std.mem.concat(allocator, u8, &[_][]const u8{ prev_page, "|", page });
        defer allocator.free(key);

        if (!page_ordering.contains(key)) {
            good = false;

            // Fix update in place
            update[i - 1] = page;
            update[i] = prev_page;
        } else {
            prev_page = page;
        }
    }

    return good;
}

test "example" {
    const result = try solution(std.testing.allocator, "input/05_input_test.txt");

    try std.testing.expectEqual(@as(u32, 143), result[0]);
    try std.testing.expectEqual(@as(u32, 123), result[1]);
}
