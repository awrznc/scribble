# coding: utf-8

# より短く配列を宣言
p %w[hoge fuga piyo]
# => ["hoge", "fuga", "piyo"]

# 実行されるファイルが存在するディレクトリの絶対パスを取得
p File.expand_path(File.dirname(__FILE__))
# => "/home/scribble/ruby/src/practice"
