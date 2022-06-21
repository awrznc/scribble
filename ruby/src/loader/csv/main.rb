require 'csv'

csv = CSV.read(

  # 読み込む対象
  "#{File.expand_path(File.dirname(__FILE__))}/example.csv",

  # データはダブルクォーテーションで囲む
  quote_char: '"',

  # カンマ区切り
  col_sep: ',',

  # ヘッダなし
  headers: false,

  # 可能な場合は数値に変換
  converters: :numeric,

  # 空行を処理しない
  skip_blanks: true,

  # マッチする行を処理しない
  #  `#` をコメントに見立てる例
  skip_lines: /^#.*$/
)

p csv
# => [[123, "abc"], [456, "def"]]
