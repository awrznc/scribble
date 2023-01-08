-- コメント

--[[
    ブロックコメント
]]


-- 出力
print("Hello, Lua!")
-- => Hello, Lua!


-- 変数
do
    -- グローバル変数
    a = 42

    print("a:", a)
    -- => a:      42

    print( string.format("a: %d", a) )
    -- => a: 42

    -- ローカル変数
    local string = [[
        夏は、夜。
        月のころは、さらなり。
    ]]
    print( string )
    -- =>
    --         夏は、夜。
    --         月のころは、さらなり。
end


-- テーブル
do
    -- リスト
    table = { 3, 2, 1 }

    -- 大きさは#で取得できる
    print( "table length: ", #table )
    -- => table length:   3

    -- Luaは1始まり
    print( "table value: ", table[1] )
    -- => table value:    3

    -- 範囲外はnilになる
    print( "table value: ", table[0] )
    print( "table value: ", table[4] )
    -- => table value:    nil

    -- 要素追加
    table[ #table + 1 ] = 42
    print( "table value: ", table[4] )
    -- => table value:    42

    -- 連想配列
    table_with_key = { a = 1 }
    print( "table value: ", table_with_key["a"] )
    print( "table value: ", table_with_key.a )
    -- => table value:    1
end


-- 四則演算
calc_result = 2 + 3 * 3 / 2.0 - 0
print("calc_result:", calc_result)
-- => calc_result:    6.5


-- 条件分岐
if 0 == 1 then
    print("0 == 1")
elseif 1 == 2 then
    print("1 == 2")
else
    print("else")
end
-- => else


-- 繰り返し
t = { "b", "a", "c" }
for i = 1, #t do
    print( "t:", t[i] )
end
-- =>
-- t:      b
-- t:      a
-- t:      c


-- 関数
function piyo(a)
    return a
end
print("return value: ", piyo(33))
-- => return value:   33


-- コルーチン
do
    -- 作成
    ct = coroutine.create(
        function (value)
            print(coroutine.yield(value))
        end
    )

    -- 状態確認
    print(coroutine.status(ct))
    -- => suspended

    -- 実行
    a, b = coroutine.resume(ct, "piyo")

    print(a)
    -- => true

    print(b)
    -- => piyo
end


-- ファイル分割
do
    dofile("./src/hello_world/main.lua")
    -- => Hello, World!
end
