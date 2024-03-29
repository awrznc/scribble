# coding: utf-8

# より短く配列を宣言
p %w[hoge fuga piyo]
# => ["hoge", "fuga", "piyo"]

# 実行されるファイルが存在するディレクトリの絶対パスを取得
p File.expand_path(File.dirname(__FILE__))
# => "/home/scribble/ruby/src/practice"

# 全ての組み合わせを取得(重複なし)
p ["Hello, ", "Hi, "].product(["World!", "World!", "Ruby!"]).uniq
# => [["Hello, ", "World!"], ["Hello, ", "Ruby!"], ["Hi, ", "World!"], ["Hi, ", "Ruby!"]]

# 自作ブロック付きメソッド
def twice(proc)
  yield proc * 2
end
twice(3) do |value|
  puts value
  # => 6
end

# Numbered parameters
['a', 'b', 'c'].each do
  p _1
end
# =>
# "a"
# "b"
# "c"

# 未定義時代入
# https://docs.ruby-lang.org/ja/latest/doc/symref.html#or
value ||= 1
p value
# => 1
value ||= 2
p value
# => 1
