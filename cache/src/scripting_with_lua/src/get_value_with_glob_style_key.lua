

-- キーの取得
local keys = {}
do
    local offset = 0
    local limit = '1'

    -- 繰り返しGLOBスタイルにマッチするキーを取得する
    repeat
        -- {カーソル (次に参照する位置), SCAN結果 (文字列のテーブル)}
        local scan_result = redis.call('SCAN', tostring(offset), 'MATCH', KEYS[1], 'COUNT', limit)

        -- SCAN結果の取得
        if not(scan_result[2] == nil) then
            for index = 1, #scan_result[2] do
                table.insert(keys, scan_result[2][index])
            end
        end

        -- 次の参照場所の決定
        offset = tonumber(scan_result[1])

        -- 次の参照場所が 0 の場合は探索の終了を表す
    until offset == 0
end

-- 結果の取得
local result = {}
for index = 1, #keys do
    local a = redis.call('GET', keys[index])
    table.insert(result, a)
end

return result
