################################################################################
# @brief 再帰的に数値、文字列、ハッシュを配列に変換する
# @param  [Integer, String, Hash] origin_hash 配列に変換したい数値、文字列、ハッシュ
# @return [Array]                 hash_array  ハッシュの配列
################################################################################
def insert_array(origin_hash)
  # 配列かどうか判定し、配列でなかったら配列にする
  hash_array = origin_hash.kind_of?(Array) ? origin_hash : [origin_hash]

  # 文字列か数値の場合はそのまま返す
  if origin_hash.kind_of?(String) || origin_hash.kind_of?(Integer)
    return hash_array

  # それ以外は再帰的にハッシュに変換する
  else
    return hash_array.each do |hash|
      hash.keys.each do |key|
        hash[key] = insert_array(hash[key])
      end
    end
  end
end


hash = {
  hoge: {
    huga: 'piyo'
  }
}

p insert_array(hash)
# => [{:hoge=>[{:huga=>["piyo"]}]}]
