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
