const std = @import("std");
const utils = @import("utils.zig");

const assert = std.debug.assert;
const Tuple = std.meta.Tuple(&.{ u32, u32 });

const TokenType = enum { Symbol, Name, Number, LParen, RParen, Comma, Eof };

const Token = struct { token_type: TokenType = TokenType.Eof, content: []const u8 = &[_]u8{} };

const InstructionType = enum { mult, toggle };

const BinOp = struct { l_op: u32, r_op: u32 };

const Instruction = union(InstructionType) { mult: BinOp, toggle: bool };

pub const Error = error{UnexpectedToken};

fn print_token(token: Token) void {
    std.debug.print("[{}] = {s}\n", .{ token.token_type, token.content });
}

const Tokenizer = struct {
    const Self = @This();

    data: []const u8,
    cursor: usize = 0,

    pub fn init(self: *Self, data: []const u8) Self {
        return self{ .data = data };
    }

    pub fn getNextToken(self: *Self) Token {
        if (self.cursor >= self.data.len) {
            return Token{ .token_type = TokenType.Eof };
        }

        var ch = self.data[self.cursor];
        var token: Token = Token{};

        // skip whitespace
        while (isSpace(ch)) {
            self.cursor += 1;
            ch = self.data[self.cursor];
        }

        switch (ch) {
            '0'...'9' => { // digit
                var digits_count: u32 = 1;
                for (self.data[(self.cursor + 1)..]) |c| {
                    if (c < '0' or c > '9') break;
                    digits_count += 1;
                }

                token = Token{ .token_type = TokenType.Number, .content = self.data[self.cursor..(self.cursor + digits_count)] };
            },
            'A'...'Z', 'a'...'z', '\'' => { // letter
                var letters_count: u32 = 1;
                for (self.data[(self.cursor + 1)..]) |c| {
                    if ((c < 'A' or c > 'Z') and (c < 'a' or c > 'z') and c != '\'') break;
                    letters_count += 1;
                }

                token = Token{ .token_type = TokenType.Name, .content = self.data[self.cursor..(self.cursor + letters_count)] };
            },
            '(' => token = Token{ .token_type = TokenType.LParen, .content = &[_]u8{ch} },
            ')' => token = Token{ .token_type = TokenType.RParen, .content = &[_]u8{ch} },
            ',' => token = Token{ .token_type = TokenType.Comma, .content = &[_]u8{ch} },
            else => token = Token{ .token_type = TokenType.Symbol, .content = &[_]u8{ch} },
        }

        // advance cursor
        self.cursor += token.content.len;

        //print_token(token);

        return token;
    }
};

fn isSpace(ch: u8) bool {
    return utils.inSlice(u8, &[_]u8{ ' ', '\x09', '\n', '\r' }, ch);
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const result = try solution(allocator, "input/03_input.txt");

    std.debug.print("Part 1: {d}\n", .{result[0]});
    std.debug.print("Part 2: {d}\n", .{result[1]});
}

fn solution(allocator: std.mem.Allocator, input: []const u8) !Tuple {
    const content = try utils.readFile(&allocator, input);
    defer allocator.free(content);

    var tokenizer: Tokenizer = .{ .data = content };

    const instructions = try parse(&tokenizer, allocator);
    defer instructions.deinit();

    // part 1
    var sum1: u32 = 0;
    for (instructions.items) |instr| {
        if (instr == .mult) {
            sum1 += instr.mult.l_op * instr.mult.r_op;
        }
    }

    // part 2
    var sum2: u32 = 0;
    var do: bool = true;
    for (instructions.items) |instr| {
        switch (instr) {
            .mult => {
                if (do) {
                    sum2 += instr.mult.l_op * instr.mult.r_op;
                }
            },
            .toggle => {
                do = instr.toggle;
            },
        }
    }

    return .{ sum1, sum2 };
}

fn parse(tokenizer: *Tokenizer, allocator: std.mem.Allocator) !std.ArrayList(Instruction) {
    var instructions = try std.ArrayList(Instruction).initCapacity(allocator, 100);

    while (true) {
        const token = tokenizer.getNextToken();
        if (token.token_type == TokenType.Eof) break;

        if (token.token_type == TokenType.Name) {
            if (std.mem.endsWith(u8, token.content, "mul")) {
                const inst = parseMult(tokenizer) catch continue;
                try instructions.append(inst);
            } else if (std.mem.endsWith(u8, token.content, "do")) {
                parseEmptyExpr(tokenizer) catch continue;
                try instructions.append(Instruction{ .toggle = true });
            } else if (std.mem.endsWith(u8, token.content, "don't")) {
                parseEmptyExpr(tokenizer) catch continue;
                try instructions.append(Instruction{ .toggle = false });
            }
        }
    }

    return instructions;
}

fn parseMult(tokenizer: *Tokenizer) !Instruction {
    _ = try expectToken(TokenType.LParen, tokenizer);
    const n1 = try expectToken(TokenType.Number, tokenizer);
    if (n1.len > 3) return Error.UnexpectedToken;
    _ = try expectToken(TokenType.Comma, tokenizer);
    const n2 = try expectToken(TokenType.Number, tokenizer);
    if (n2.len > 3) return Error.UnexpectedToken;
    _ = try expectToken(TokenType.RParen, tokenizer);

    const a = try std.fmt.parseInt(u32, n1, 10);
    const b = try std.fmt.parseInt(u32, n2, 10);

    return Instruction{ .mult = BinOp{ .l_op = a, .r_op = b } };
}

fn parseEmptyExpr(tokenizer: *Tokenizer) !void {
    _ = try expectToken(TokenType.LParen, tokenizer);
    _ = try expectToken(TokenType.RParen, tokenizer);
}

fn expectToken(token_type: TokenType, tokenizer: *Tokenizer) Error![]const u8 {
    const tk = tokenizer.getNextToken();
    if (tk.token_type != token_type) {
        return Error.UnexpectedToken;
    }

    return tk.content;
}

test "example1" {
    const result = try solution(std.testing.allocator, "input/03_input_test_1.txt");

    try std.testing.expectEqual(@as(u32, 161), result[0]);
}

test "example2" {
    const result = try solution(std.testing.allocator, "input/03_input_test_2.txt");

    try std.testing.expectEqual(@as(u32, 48), result[1]);
}
