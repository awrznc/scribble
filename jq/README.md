jq
===

コマンドラインJSONプロセッサ。

JSON文字列の加工を容易に行うことができる。


## Run

```bash
$ jq --version
jq-1.6

$ jq -n --from-file --raw-output ./src/hello_world/main.jq
Hello, World!

$ jq --from-file ./src/practice/main.jq ./src/practice/color.json
"3,4,5"

$ jq -n \
    --arg x 1 \
    --arg y 2 \
    --arg z 3 \
    '[ $x, $y, $z ]'
[
  "1",
  "2",
  "3"
]
```
