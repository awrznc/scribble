# 標準出力
echo "hello, world"

# 変数

## 出力
var n = 1
echo "n=" & $n

var members = @["Ruby", "Garnet"]
add(members, "Amethyst")
echo "members=" & $members
echo "members=" & repr(members)

## イミュータブル変数
let nn = 1
echo "nn=" & $nn
# n = 2 # compile error

## ミュータブル変数
var nnn = 1
nnn = 2

## 定数
const const_n = 1
# const_n = 2 #error

## 型推論
var type_n: int = 1

## resultなるものもあるらしい

# 制御構文
## if
let if_n = 10
if if_n == 0:
  echo "zero"
elif if_n mod 2 == 0:
  echo "even"
else:
  echo "odd"

## case
let case_n = -2
let case_r = case case_n:
  of low(int).. -1:
    "negative"
  of 0:
    "zero"
  of 2..high(int):
    "positive to 2 over"
  else:
    "impossible"

echo case_r

## for
for i in 0..10:
  echo i

## table
import tables
for id, name in { 1: "Ruby", 2: "Garnet" }.toTable:
  echo $id & ":" & $name

## while
var i: int = 0;
while i < 5:
  echo i
  i.inc

# プロージャ
proc hogeHoge(n: int): string =
  "hogehoge" & $n

echo hogehoge(10)
echo 10.hogehoge

# オペレータ
proc `->`[A, B](a: A, b: B): tuple[fst: A, snd: B] =
  (a, b)

echo "hello" -> "world"

# 第一級関数
import sequtils
let nums = @[1, 2, 3, 4, 5]
echo filter(nums, proc (n: int): bool = n mod 2 == 0)
echo filter(nums) do (n: int) -> bool: n mod 2 == 0

## 引数にプロシージャを渡す
proc even(n: int): bool = n mod 2 == 0
echo filter(nums, even)
