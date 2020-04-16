#!/bin/bash

# echo した結果を変数に入れ込み次のコマンドに組み込む
# curl で受け取った結果をパースして、ワンライナーで別のリクエストを行いたい場合などに利用すると綺麗に書けそう
echo Sample | { read query && echo Read $query; }
# => Read Sample
