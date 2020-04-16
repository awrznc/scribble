#!/bin/bash

# スクリプトのディレクトリ
SCRIPT_DIR=$(cd $(dirname $0); pwd)


# 文字列の表示

## 基本編
echo -e "\n=== echo === === === ==="
echo "Sample"

## 応用編
echo -e "\n=== cat === === === ==="
cat $SCRIPT_DIR/../assets/index.html

## 実践編
echo -e "\n=== curl === === === ==="
curl https://example.com/


# grep
# 引数に指定した文字列の行のみを表示する。
echo -e "\n=== grep === === === ==="
cat $SCRIPT_DIR/../assets/index.html | grep div
# =>     <div>Sample</div>


# sed (macの場合はgsed。sedもあるけど挙動が少し違うみたい)
# 正規表現でマッチしたものを置換する（grepの後にやることが多い）
echo -e "\n=== sed === === === ==="
cat $SCRIPT_DIR/../assets/index.html | \
    sed -E "s/^.*(<div>)(.*)(<\/div>).*$/\2/"
# => "    <div>Sample</div>" が "Sample" に置き換わる


# tr
# 文字を置換・削除する。sed の親戚。
echo -e "\n=== tr === === === ==="
cat $SCRIPT_DIR/../assets/index.html | \
    tr '\r' '&'
# => 行末に & が付く

# awk
# 文字を抜き出す。文法がわかりやすい気がする。
echo -e "\n=== awk === === === ==="
ls -al | awk -F ' ' '{print $8 " ... " $9}'
# => 半角スペースで分割して、8つ目と9つ目を表示する
