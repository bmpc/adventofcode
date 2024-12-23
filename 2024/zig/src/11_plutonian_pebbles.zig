const std = @import("std");
const utils = @import("utils.zig");

const assert = std.debug.assert;
fn Tuple(comptime T: type) type {
    return std.meta.Tuple(&.{ T, T });
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const result1 = try solution(allocator, "input/11_input.txt", 25);
    const result2 = try solution(allocator, "input/11_input.txt", 75);

    std.debug.print("Part 1: {d}\n", .{result1});
    std.debug.print("Part 2: {d}\n", .{result2});
}

fn solution(allocator: std.mem.Allocator, input: []const u8, iterations: usize) !u128 {
    const data = try utils.readFile(&allocator, input);
    defer allocator.free(data);

    var stones_it = std.mem.split(u8, data, " ");

    var stones = try std.ArrayList(u128).initCapacity(allocator, 100000);
    defer stones.deinit();

    while (stones_it.next()) |stone| {
        const id = try std.fmt.parseInt(u128, stone, 10);

        try stones.append(id);
    }

    var visited = std.AutoHashMap(Tuple(u128), u128).init(allocator);
    defer visited.deinit();

    var total: u128 = 0;
    for (stones.items) |stone| {
        total += try countStoneSteps(stone, iterations, &visited);
    }

    return total;
}

// !!! Very Slow !!!
// This solution replaces the values inline in the array
fn naiveSolution(stones: *std.ArrayList(u128), iterations: usize) !u128 {
    var list = try stones.clone();
    defer list.deinit();

    for (0..iterations) |_| {
        var i: usize = 0;
        while (i < list.items.len) {
            const stone = list.items[i];
            if (stone == 0) {
                // 1. If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
                list.items[i] = 1;
            } else {
                const np = numPlaces(stone);
                if (np % 2 == 0) {
                    // 2. If the stone is engraved with a number that has an even number of digits, it is replaced by two list.
                    // The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved
                    // on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
                    const split = numSplit(stone, np);
                    list.items[i] = split[0];
                    try list.insert(i + 1, split[1]);
                    i += 1;
                } else {
                    // 3. If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied
                    // by 2024 is engraved on the new stone.

                    // std.debug.print("Stone = {}\n", .{stone});
                    list.items[i] = stone * 2024;
                }
            }
            i += 1;
        }
    }

    return list.items.len;
}

fn countStoneSteps(stone: u128, iterations: usize, visited: *std.AutoHashMap(Tuple(u128), u128)) !u128 {
    const v = visited.get(.{ stone, iterations });
    if (v != null) {
        return v.?;
    }

    if (iterations == 0) {
        //try visited.put(.{ stone, iterations }, 1);
        return 1;
    }

    if (stone == 0) {
        // 1. If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
        const count = try countStoneSteps(1, iterations - 1, visited);
        try visited.put(.{ stone, iterations }, count);
        return count;
    } else {
        const np = numPlaces(stone);
        if (np % 2 == 0) {
            // 2. If the stone is engraved with a number that has an even number of digits, it is replaced by two stones.
            // The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved
            // on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
            const split = numSplit(stone, np);
            const left_it = try countStoneSteps(split[0], iterations - 1, visited);
            const right_it = try countStoneSteps(split[1], iterations - 1, visited);
            try visited.put(.{ stone, iterations }, left_it + right_it);
            return left_it + right_it;
        } else {
            // 3. If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied
            // by 2024 is engraved on the new stone.
            const count = try countStoneSteps(stone * 2024, iterations - 1, visited);
            try visited.put(.{ stone, iterations }, count);
            return count;
        }
    }
}

fn numPlaces(n: u128) u128 {
    if (n < 10) return 1;
    return 1 + numPlaces(n / 10);
}

fn exp(e: usize) u128 {
    var v: u128 = 1;
    for (0..e) |_| {
        v *= 10;
    }
    return v;
}

fn numSplit(n: u128, places: u128) Tuple(u128) {
    const fact = exp(@intCast(places / 2));
    const l = n / fact;
    const r = n % fact;

    return .{ l, r };
}

test "example" {
    const result = try solution(std.testing.allocator, "input/11_input_test.txt", 25);

    try std.testing.expectEqual(@as(u32, 55312), result);
}
