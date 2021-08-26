# 関数
function sample_function (string) {
        print string
}

# 開始時の処理
BEGIN {
        # 標準出力
        print "標準出力:"
        print "  Sample"

        ## 改行したくない場合は printf を利用する
        printf "  Example"
        printf "\n\n"

        # 四則演算
        print "四則演算:"
        print "  3 + 3 = " 3 + 3 # => 6
        print "  3 - 3 = " 3 - 3 # => 0
        print "  3 * 3 = " 3 * 3 # => 9
        print "  3 / 3 = " 3 / 3 # => 1
        print "  2 + 3 * 3 / 2 - 0 = " 2 + 3 * 3 / 2 - 1 # => 5.5
        print ""

        # 関数の利用
        print "関数の利用:"
        sample_function("  sample_function!\n")

        # 条件分岐
        print "条件分岐:"
        if (2 == 2) {
                print "  2 == 2\n"
        } else {
                print "  !(2 == 2)\n"
        }
        # => 2 == 2

        # 変数の初期化
        count = 0

        # 繰り返し
        print "繰り返し:"
        for (i=0; i<3; i++) {
                print "  " i
        }
        print ""
        # =>
        #   0
        #   1
        #   2

        # システムのコマンドを実行
        print "システムのコマンド:"
        system("ls")
        print "\n"

        # 組込変数
        # 組込変数を利用することでセパレータへ渡す文字列が指定できる
        FS = ","
}

# 全ての行が対象
{
        print "field : (" $1 ", " $2 ")"
}

# /開始パターン/, /終了パターン/
# `a`を含む行から`d`を含む行までが対象
/a/, /d/ {
        # マッチした行を標準出力
        # NR にはそれまでに入力したレコード数が格納されている
        print "match " NR " : " $0

        # 変数のインクリメント
        ++count
}

# 終了時の処理
END {
        print "\ncount: " count "\n"
        print "finish!"
}
