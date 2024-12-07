const std = @import("std");
const utils = @import("utils.zig");

const assert = std.debug.assert;
const Tuple = std.meta.Tuple(&.{ u32, u32 });

const HorizontalDir = enum { NONE, RIGHT, LEFT };
const VerticalDir = enum { NONE, UP, DOWN };

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const result = try solution(allocator, "input/04_input.txt");

    std.debug.print("Part 1: {d}\n", .{result[0]});
    std.debug.print("Part 2: {d}\n", .{result[1]});
}

fn solution(allocator: std.mem.Allocator, input: []const u8) !Tuple {
    const content = try utils.readFile(&allocator, input);
    defer allocator.free(content);

    var matrix = try utils.Matrix(u8).initFromSequence(content, "\n", allocator);
    defer matrix.deinit();

    //matrix.print();

    // part 1
    var total1: u32 = 0;
    for (0..matrix.height) |row| {
        for (0..matrix.width) |col| {
            const ch = try matrix.get(row, col);
            if (ch == 'X') {
                total1 += try checkXMAS(row, col, &matrix);
            }
        }
    }

    // part 2
    var total2: u32 = 0;
    for (0..matrix.height) |row| {
        for (0..matrix.width) |col| {
            const ch = try matrix.get(row, col);
            if (ch == 'A') {
                total2 += if (try checkX_MAS(row, col, &matrix)) 1 else 0;
            }
        }
    }

    return .{ total1, total2 };
}

fn checkXMAS(row: usize, col: usize, matrix: *utils.Matrix(u8)) !u32 {
    var count: u32 = 0;
    const right = try checkDir(HorizontalDir.RIGHT, VerticalDir.NONE, row, col, matrix);
    if (right) count += 1;
    const left = try checkDir(HorizontalDir.LEFT, VerticalDir.NONE, row, col, matrix);
    if (left) count += 1;
    const up = try checkDir(HorizontalDir.NONE, VerticalDir.UP, row, col, matrix);
    if (up) count += 1;
    const down = try checkDir(HorizontalDir.NONE, VerticalDir.DOWN, row, col, matrix);
    if (down) count += 1;
    const up_right = try checkDir(HorizontalDir.RIGHT, VerticalDir.UP, row, col, matrix);
    if (up_right) count += 1;
    const up_left = try checkDir(HorizontalDir.LEFT, VerticalDir.UP, row, col, matrix);
    if (up_left) count += 1;
    const down_right = try checkDir(HorizontalDir.RIGHT, VerticalDir.DOWN, row, col, matrix);
    if (down_right) count += 1;
    const down_left = try checkDir(HorizontalDir.LEFT, VerticalDir.DOWN, row, col, matrix);
    if (down_left) count += 1;

    return count;
}

fn checkDir(horizontal_dir: HorizontalDir, vertical_dir: VerticalDir, row: usize, col: usize, matrix: *utils.Matrix(u8)) !bool {
    var rowDir: i8 = 0;
    var colDir: i8 = 0;

    const i_row: i32 = @as(i32, @intCast(row));
    const i_col: i32 = @as(i32, @intCast(col));

    switch (horizontal_dir) {
        HorizontalDir.RIGHT => {
            if (col + 3 > matrix.width - 1) return false;
            colDir = 1;
        },
        HorizontalDir.LEFT => {
            if (col < 3) return false;
            colDir = -1;
        },
        HorizontalDir.NONE => {},
    }

    switch (vertical_dir) {
        VerticalDir.UP => {
            if (row < 3) return false;
            rowDir = -1;
        },
        VerticalDir.DOWN => {
            if (row > matrix.height - 3 - 1) return false;
            rowDir = 1;
        },
        VerticalDir.NONE => {},
    }

    const x = try matrix.get(row, col);
    const m = try matrix.get(@intCast(i_row + 1 * rowDir), @intCast(i_col + 1 * colDir));
    const a = try matrix.get(@intCast(i_row + 2 * rowDir), @intCast(i_col + 2 * colDir));
    const s = try matrix.get(@intCast(i_row + 3 * rowDir), @intCast(i_col + 3 * colDir));

    const slice = [_]u8{ x, m, a, s };

    return std.mem.eql(u8, &slice, "XMAS");
}

fn checkX_MAS(row: usize, col: usize, matrix: *utils.Matrix(u8)) !bool {
    if (col + 1 > matrix.width - 1) return false; // right
    if (col < 1) return false; // left
    if (row < 1) return false; // up
    if (row > matrix.height - 1 - 1) return false; // down

    const up_left = try matrix.get(row - 1, col - 1);
    const up_right = try matrix.get(row - 1, col + 1);
    const down_left = try matrix.get(row + 1, col - 1);
    const down_right = try matrix.get(row + 1, col + 1);

    const slice1 = [_]u8{ up_left, 'A', down_right };
    const slice2 = [_]u8{ down_left, 'A', up_right };

    return (std.mem.eql(u8, &slice1, "MAS") or std.mem.eql(u8, &slice1, "SAM")) and (std.mem.eql(u8, &slice2, "MAS") or std.mem.eql(u8, &slice2, "SAM"));
}

test "example" {
    const result = try solution(std.testing.allocator, "input/04_input_test.txt");

    try std.testing.expectEqual(@as(u32, 18), result[0]);
    try std.testing.expectEqual(@as(u32, 9), result[1]);
}
