const std = @import("std");
const root = @import("root.zig");
const BigInt = std.math.big.int;

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const fibrank = try root.search_fiborank(1000, allocator);
    std.debug.print("rank: {d}", .{fibrank});

    // const n = 200;
    // var fibolen = try std.ArrayList(usize).initCapacity(allocator, n + 1);
    // for (0..(n + 1)) |i| try fibolen.append(try root.compute_bignum_len(fibseq.items[i], allocator));

    // for (fibolen.items, 0..) |flen, xi|
    //     std.debug.print("{d} {d}\n", .{ xi, flen });

    // const regression_result = root.fib_linear_regression(fibolen);
    // std.debug.print("slope={}, origin={}", .{ regression_result.slope, regression_result.origin });
}
