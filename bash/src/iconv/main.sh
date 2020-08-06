#!/bin/bash

# sample.txt の中身を表示する
# Shift JIS なので文字化けする
cat sample.txt

# SJIS を UTF-8 に変換して表示する
# 文字化けしない
iconv --from-code sjis --to-code UTF-8 sample.txt
