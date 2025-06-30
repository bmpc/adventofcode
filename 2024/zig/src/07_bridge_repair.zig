const std = @import("std");
const utils = @import("utils.zig");

const assert = std.debug.assert;
const Tuple = std.meta.Tuple(&.{ u64, u64 });

const Operator = enum { ADD, MULT, CONCAT };

const Equation = struct {
    result: u64,
    operands: []u64,
};

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const result = try solution(allocator, "input/07_input.txt");

    std.debug.print("Part 1: {d}\n", .{result[0]});
    std.debug.print("Part 2: {d}\n", .{result[1]});
}

fn solution(allocator: std.mem.Allocator, input: []const u8) !Tuple {
    const content = try utils.readFile(&allocator, input);
    defer allocator.free(content);

    var lines = std.mem.splitSequence(u8, content, "\n");

    var equations = std.ArrayList(Equation).init(allocator);
    defer {
        for (equations.items) |eq| {
            allocator.free(eq.operands);
        }
        equations.deinit();
    }

    while (lines.next()) |line| {
        var parts = std.mem.splitSequence(u8, line, " ");
        const result_str = parts.next().?;

        const res_str = result_str[0 .. result_str.len - 1]; // removing the colon from the result
        const result = try std.fmt.parseInt(u64, res_str, 10);

        var operands = std.ArrayList(u64).init(allocator);
        while (parts.next()) |operand_str| {
            const operand = try std.fmt.parseInt(u64, operand_str, 10);
            try operands.append(operand);
        }

        try equations.append(Equation{ .result = result, .operands = try operands.toOwnedSlice() });
    }

    // part 1
    var total1: u64 = 0;
    for (equations.items) |equation| {
        if (isEquationTrue(equation, &[_]Operator{ Operator.ADD, Operator.MULT })) {
            total1 += equation.result;
        }
    }

    // part 2
    var total2: u64 = 0;
    for (equations.items) |equation| {
        if (isEquationTrue(equation, &[_]Operator{ Operator.ADD, Operator.MULT, Operator.CONCAT })) {
            total2 += equation.result;
        }
    }

    return .{ total1, total2 };
}

fn isEquationTrue(equation: Equation, operators: []const Operator) bool {
    return getBranchResult(0, 0, null, operators, equation);
}

fn getBranchResult(operand_idx: usize, carry: u64, operator: ?Operator, operators: []const Operator, equation: Equation) bool {
    var _carry: u64 = carry;

    if (operator == null) {
        _carry = equation.operands[operand_idx];
    } else {
        switch (operator.?) {
            Operator.ADD => {
                _carry += equation.operands[operand_idx];
            },
            Operator.MULT => {
                _carry *= equation.operands[operand_idx];
            },
            Operator.CONCAT => {
                _carry = utils.concatNumbersStatic(_carry, equation.operands[operand_idx]) catch unreachable;
            },
        }
    }

    if (_carry > equation.result) {
        return false;
    }

    if (operand_idx < equation.operands.len - 1) {
        for (operators) |op| {
            const res = getBranchResult(operand_idx + 1, _carry, op, operators, equation);
            if (res) return true;
        }

        return false;
    } else { // last operand (operand_idx == equation.operands.len - 1)
        return (equation.result == _carry);
    }
}

test "example" {
    const result = try solution(std.testing.allocator, "input/07_input_test.txt");

    try std.testing.expectEqual(@as(u64, 3749), result[0]);
    try std.testing.expectEqual(@as(u64, 11387), result[1]);
}
