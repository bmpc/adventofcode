const std = @import("std");
const utils = @import("utils.zig");

const assert = std.debug.assert;
const Tuple = std.meta.Tuple(&.{ []u8, u32 });

// 3-bit computer
// 3 registers: A, B, C
// 8 instructions
//
// Combo operands 0 through 3 represent literal values 0 through 3.
// Combo operand 4 represents the value of register A.
// Combo operand 5 represents the value of register B.
// Combo operand 6 represents the value of register C.
// Combo operand 7 is reserved and will not appear in valid programs.
//
// Instructions:
//   0-adv > division into A
//   1-bxl > bitwise OR
//   2-bst > modulo 8 into B
//   3-jnz > jump not zero
//   4-bxc > bitwise XOR into B
//   5-out > modulo 8 and outputs
//   6-bdv > division into B
//   7-cdv > division into C

const Opcode = enum { ADV, BXL, BST, JNZ, BXC, OUT, BDV, CDV };

const Computer = struct {
    const Self = @This();

    allocator: std.mem.Allocator = undefined,
    initialized: bool = false,
    ip: usize = 0,
    a: u32 = 0,
    b: u32 = 0,
    c: u32 = 0,
    input: []u3 = undefined,
    output: std.ArrayList(u3) = undefined,

    pub fn reset(self: *Self, allocator: std.mem.Allocator, a: u32, b: u32, c: u32, input: []u3) !void {
        self.input = input;
        self.a = a;
        self.b = b;
        self.c = c;
        self.ip = 0;

        self.allocator = allocator;

        if (self.initialized) {
            self.output.deinit();
        }

        self.initialized = true;
        self.output = std.ArrayList(u3).init(allocator);
    }

    pub fn run(self: *Self) !void {
        while (self.ip < self.input.len - 1) {
            const opcode: Opcode = @enumFromInt(self.input[self.ip]);
            const operand: u3 = self.input[self.ip + 1];
            switch (opcode) {
                Opcode.ADV => {
                    self.a = self.a / std.math.pow(u32, 2, try self.resolveCombo(operand));
                },
                Opcode.BXL => {
                    self.b = self.b ^ operand;
                },
                Opcode.BST => {
                    self.b = try self.resolveCombo(operand) % 8;
                },
                Opcode.JNZ => {
                    if (self.a > 0) {
                        self.ip = operand;
                        continue;
                    }
                },
                Opcode.BXC => {
                    self.b = self.b ^ self.c;
                },
                Opcode.OUT => {
                    const m = try self.resolveCombo(operand) % 8;
                    try self.output.append(@intCast(m));
                },
                Opcode.BDV => {
                    self.b = self.a / std.math.pow(u32, 2, try self.resolveCombo(operand));
                },
                Opcode.CDV => {
                    self.c = self.a / std.math.pow(u32, 2, try self.resolveCombo(operand));
                },
            }

            self.ip += 2;
        }
    }

    fn resolveCombo(self: *Self, combo: u3) !u32 {
        return switch (combo) {
            0...3 => @intCast(combo),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            else => unreachable,
        };
    }

    pub fn deinit(self: *Self) void {
        self.output.deinit();
        self.allocator.free(self.input);
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        _ = gpa.deinit();
    }
    const allocator = gpa.allocator();

    const result = try solution(allocator, "input/17_input.txt");
    defer allocator.free(result[0]);

    std.debug.print("Part 1: {s}\n", .{result[0]});
    std.debug.print("Part 2: {d}\n", .{result[1]});
}

fn solution(allocator: std.mem.Allocator, input: []const u8) !Tuple {
    const data = try utils.readFile(&allocator, input);
    defer allocator.free(data);

    var lines_it = std.mem.split(u8, data, "\n");
    var src_input: std.ArrayList(u3) = std.ArrayList(u3).init(allocator);
    defer src_input.deinit();

    var reg_a: u32 = 0;
    var reg_b: u32 = 0;
    var reg_c: u32 = 0;

    while (lines_it.next()) |line| {
        var it = std.mem.split(u8, line, ": ");

        if (it.peek() == null) continue;
        const name = it.next().?;

        if (it.peek() == null) continue;
        const value = it.next().?;

        if (std.mem.eql(u8, name, "Register A")) {
            reg_a = try std.fmt.parseInt(u32, value, 10);
        }

        if (std.mem.eql(u8, name, "Register B")) {
            reg_b = try std.fmt.parseInt(u32, value, 10);
        }

        if (std.mem.eql(u8, name, "Register C")) {
            reg_c = try std.fmt.parseInt(u32, value, 10);
        }

        if (std.mem.eql(u8, name, "Program")) {
            var iit = std.mem.split(u8, value, ",");

            while (iit.next()) |v| {
                const out = try std.fmt.parseInt(u3, v, 10);
                try src_input.append(out);
            }
        }
    }

    var computer: Computer = .{};
    try computer.reset(allocator, reg_a, reg_b, reg_c, try src_input.toOwnedSlice());

    try computer.run();

    const len = (computer.output.items.len * 2) - 1;
    const res1 = try allocator.alloc(u8, len);
    var idx: usize = 0;
    for (computer.output.items, 0..computer.output.items.len) |item, i| {
        res1[idx] = '0' + @as(u8, @intCast(item));
        if (i < computer.output.items.len - 1) {
            res1[idx + 1] = ',';
            idx += 2;
        }
    }

    defer computer.deinit();

    return .{ res1, 0 };
}

test "example 1" {
    const result = try solution(std.testing.allocator, "input/17_input_test.txt");
    defer std.testing.allocator.free(result[0]);

    try std.testing.expectEqualStrings("4,6,3,5,6,3,5,2,1,0", result[0]);
    //try std.testing.expectEqual(@as(u32, 0), result[1]);
}
