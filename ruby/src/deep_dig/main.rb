################################################################################
# @brief  Hash形式のデータを深く掘る
# @param  [Hash]      data    掘る対象のデータ
# @param  [String]    key     掘りたいキー
# @param  [anything]  defalut キーがない場合の返り値
# @return [Array]             末端まで掘った値（複数）
################################################################################
def deep_dig(data, key, default = {})
  # キーが文字列がシンボルでない場合はdefaultを返す
  return default unless key.is_a?(String) || key.is_a?(Symbol)

  values = []

  # 配列じゃない場合は配列に変換する
  array = data.is_a?(Array) ? data : [data]
  array.each do |hash|
    # ハッシュでない場合は次のループへ
    next unless hash.is_a?(Hash)

    # キーをシンボルに統一
    hash = hash.each_with_object({}) do |(k, v), elem|
      elem[k.to_sym] = v
    end

    # ハッシュ内に目的のキーが存在する場合はキーの値を返す
    values << hash[key.to_sym] if hash.keys.include?(key.to_sym)

    # 再帰的にキーを探索する
    hash.keys.each do |hash_key|
      value = deep_dig(hash[hash_key], key, default)

      # defaultでない値が帰ってきた場合は目的のキーの値を発見したとみなし返り値を返す
      values << value unless default == value
    end
  end

  # キーが見つけられなかった場合はdefaultを返す
  values.empty? ? default : values.flatten
end
