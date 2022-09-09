" コメント


" 出力

echo "Hello,\tVim!\n"
" => Hello, Vim!


" 変数

let variable = "piyo\n"
echo variable
" => piyo


" 辞書型

let color = { 'red': 0xff, 'green': 0xfF, 'blue': 0xFF }
echo color.red
" => 255


" 四則演算

let result = 2 + 3 * 3 / 2.0 - 0
echo result
" => 6.5


" 条件分岐

if result < 0
elsif result == 1
else
    echo 'else'
endif


" 繰り返し

for i in [0, 1, 2, 4, 8]
    echo i * 2
endfor
" =>
" 0
" 2
" 4
" 8
" 16


" 関数

" `!` は同名の関数がある場合は上書き
" `abort` は関数内でエラーが発生した場合そこで終了
" 引数を利用する場合は `a:` を付ける

function! ReturnZero(number) abort
    return a:number * 0
endfunction

echo ReturnZero(1)
" => 0


" setline

call setline(1, ['first line', 'second line'])
" 一行目には `first line` が、
" 二行目には `second line` が挿入される


" getline

" 1 ~ 2 行目を出力
echo getline(1, 2)
" ['first line', 'second line']


" set
" 指定した項目の設定を行う

" 行番号の表示
set number
" `set nonumber` で非表示に戻せる


" syntax
" シンタックスハイライトの設定を行う

" シンタックスハイライトの有効化
syntax on
" `syntax off` で元に戻せる
