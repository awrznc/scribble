const std = @import("std");

// コメント
// //! や /// でドキュメンテーションコメントが可能

pub fn main() void {

    // 出力
    std.debug.print("Hello, Zig!\n", .{});

    // 変数
    var two: i32 = 2;
    std.debug.print("{}\n", .{two});
    // => 2

    // 定数
    const one: i64 = 1;
    std.debug.print("{}\n", .{one});
    // => 1

    // リスト
    const items = [_]i32 { 0, 1, 2, 3, 4, 5 };

    // 四則演算
    const calc_result: f32 = 2.0 + 3.0 * 3.0 / 2.0 - 0.0;
    std.debug.print("{}\n", .{calc_result});
    // => 6.5e+00

    // 条件分岐
    if (1 == 2) {
        std.debug.print("1 == 2\n", .{});
    } else {
        std.debug.print("1 != 2\n", .{});
    }
    // => 1 != 2

    // 繰り返し
    for (items) |item| {
        if (item != 5) {
            continue;
        }
        std.debug.print("{}\n", .{item});
    }
    // => 5
}
