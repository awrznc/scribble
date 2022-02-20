{
    # 渡された文字列を"aaa"に置き換える
    s/.*/aaa/;

    # 最初にマッチした"aaa"という文字列を"bbb"という文字列に置き換える
    s/aaa/bbb/;

    # マッチした文字列を全て置き換える
    s/bbb/ccc/g;
    # 2回目以降にマッチした文字列のみを置き換える場合は"/2g"と記述する

    # 大文字小文字を区別せずに置き換える
    s/ccc/ddd/i;

    # マッチした文字列を2回繰り返した文字列に置き換える
    s/ddd/\0\0/;

    # "ddd"という文字列を2回繰り返す文字列を置き換える
    s/\(ddd\)\{2\}/eee/;
    # 2回以上の場合は"{2,}"と記述する
    # 2回以下の場合は"{,2}"と記述する
    # 2回以上3回以下の場合は"{2,3}"と記述する

    # マッチした文字列に対する処理を区切る
    /eee/ {
        s/eee/fff/;
    };

    # 1行目を置換対象の行とする
    1s/fff/abcd/;
    # 1行以上の場合は"1,s/"と記述する
    # 1行以下の場合は",1s/"と記述する
    # 1行以上2行以下の場合は"1,2s/"と記述する

    # 2行目を削除する
    # 行指定のフォーマットは"s"と同じ
    2d;

    # リストを利用した置換を行う
    y/dbac/hfeg/;

    # 置換した結果を出力する
    # sedを実行する際に`-n`オプションを付けて実行するとpのみの出力となる
    s/.*/\0/p;
    # 後ろに付けなくてもOK
    p;

    # 結果をShellScriptとして実行する
    s/.*/ls -al/e

    # 置換した結果の出力先を指定したファイルに変更する
    w result.txt
}