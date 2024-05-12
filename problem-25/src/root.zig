const std = @import("std");
const BigInt = std.math.big.int;

pub fn search_fiborank(n_digits: usize, allocator: std.mem.Allocator) !usize {
    if (n_digits == 1) return 0;
    var fiboseq = std.ArrayList(BigInt.Managed).init(allocator);
    {
        try fiboseq.append(try BigInt.Managed.initSet(allocator, 0));
        try fiboseq.append(try BigInt.Managed.initSet(allocator, 1));
    }

    var curr_rank: usize = 2;
    while (try compute_bignum_len(fiboseq.getLast(), allocator) < n_digits) : (curr_rank += 1) {
        var sum = try BigInt.Managed.init(allocator);
        try sum.add(&fiboseq.items[curr_rank - 1], &fiboseq.items[curr_rank - 2]);
        try fiboseq.append(sum);
    }
    return fiboseq.items.len - 1;
}

pub fn fiboseqbig(n: usize, allocator: std.mem.Allocator) !std.ArrayList(BigInt.Managed) {
    var fiboseq = try std.ArrayList(BigInt.Managed).initCapacity(allocator, n + 1);
    {
        try fiboseq.append(try BigInt.Managed.initSet(allocator, 0));
        try fiboseq.append(try BigInt.Managed.initSet(allocator, 1));
    }
    if (n <= 2) return fiboseq;

    for (2..(n + 1)) |i| {
        var sum = try BigInt.Managed.init(allocator);
        try sum.add(&fiboseq.items[i - 1], &fiboseq.items[i - 2]);
        try fiboseq.append(sum);
    }
    return fiboseq;
}

pub fn compute_bignum_len(bn: BigInt.Managed, allocator: std.mem.Allocator) !usize {
    const bn_str = try bn.toString(allocator, 10, std.fmt.Case.lower);
    return bn_str.len;
}

pub fn print_big_numbers(bns: std.ArrayList(BigInt.Managed), allocator: std.mem.Allocator) void {
    for (bns.items, 0..) |bignum, xi|
        std.debug.print("{d} {!s}\n", .{ xi, bignum.toString(allocator, 10, std.fmt.Case.lower) });
}

const LinearRegressionResult = struct { slope: f64, origin: f64 };

fn u2i(u: usize) isize {
    return @as(isize, @intCast(u));
}

fn u2f(u: usize) f64 {
    return @as(f64, @floatFromInt(u));
}

pub fn fib_linear_regression(y: std.ArrayList(usize)) LinearRegressionResult {
    const n = y.items.len;

    var sum_xiyi: usize = 0;
    var sum_yi: usize = 0;

    for (y.items, 0..) |yi, xi| {
        sum_yi += yi;
        sum_xiyi += xi * yi;
    }

    const sum_xi = n * (n + 1) / 2;
    const sum_xi_squared = sum_xi * sum_xi;
    const sum_xi_square = n * (n + 1) * (2 * n + 1) / 6;

    var result = LinearRegressionResult{ .slope = undefined, .origin = undefined };

    {
        const numerator = u2i(sum_xiyi) - u2i(sum_xi * sum_yi);
        const denominator = u2i(sum_xi_square) - u2i(sum_xi_squared);

        const slope = @as(f64, @floatFromInt(numerator)) / @as(f64, @floatFromInt(denominator));
        result.slope = slope;
    }

    {
        const numerator = u2f(sum_yi) - result.slope * u2f(sum_xi);
        const denominator = u2f(n);

        const origin = numerator / denominator;
        result.origin = origin;
    }

    return result;
}
