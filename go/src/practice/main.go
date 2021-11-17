package main

import (
	"fmt"
	"os"
	"bufio"
	"strconv"
	"reflect"
)


// 先頭が小文字の関数はprivateとして扱われる
// 先頭が大文字の関数はpublicとして扱われる
func variable() {

	// 変数の宣言と代入
	// 代入を行わない場合自動的に0が代入される
	var number int = 1

	// 宣言代入の糖衣構文
	number_two := 2

	// 変数の出力
	fmt.Println(number, number_two)
	// => 1 2
}


func pointer() {
	var number int = 1
	var numberPointer *int = &number
	fmt.Println(numberPointer, *numberPointer)
	// => 0x(numberのアドレス) 1
}


func calculation() {
	fmt.Println(2 + 3 * 3 / 2 - 0)
	// => 6
}


func variadicArguments(a int, b ...int) {
	fmt.Println(a)
	// => (aの値)

	fmt.Println(b)
	// => [(bの値その1) (bの値その2)]
}


func array() {

	// 固定長配列
	var numbers = [3] int { 0, 2, 0 }
	fmt.Println(numbers)
	// => [0 2 0]

	// スライス（動的配列）
	var strings = [] string { "go", "lang" }
	fmt.Println(strings)
	// => [go lang]

	// 長さ
	fmt.Println(len(strings))
	// => 2

	// 容量
	fmt.Println(cap(strings))
	// => 2

	// 要素の参照
	accessPsition := 1
	fmt.Println(strings[accessPsition])		// => [lang]
	fmt.Println(strings[:accessPsition])	// => [go]
	fmt.Println(strings[accessPsition:])	// => [lang]

	// 配列の一部からスライスを生成
	// 0番目から2番目のひとつ前の要素までを対象とする
	fmt.Println(numbers[0:2])
	// => [0 2]

	// 要素の追加
	strings = append(strings, "!", "?")
	fmt.Println(strings)
	// => [go lang ! ?]

	// 要素の削除
	// 指定した添字のひとつ前までの値と、
	// 指定した添字のひとつ後から先の値を展開し、
	// append関数で結合することで削除する
	removePosition := 1
	fmt.Println(append(strings[:removePosition], strings[removePosition+1:]...))
	// => [go ! ?]
}


func mapExample() {
	var color = map[string] int {
		"red":   0xff0000,
		"green": 0x00ff00,
		"blue":  0x0000ff,
	}
	fmt.Println(color)
	// => map[blue:255 green:65280 red:16711680]
}


func conditionalBranch() {

	// if
	if 1 == 1 {
		fmt.Println("if 1 == 1 =>", 1 == 1)
		// => if 1 == 1 => true
	} else if 2 == 2 {
		fmt.Println("else if")
	} else {
		fmt.Println("else")
	}

	// switch
	number := 1
	switch number {
		case 1: fmt.Println("number == 1")
		default: fmt.Println("number != 1")
	}

	// fallthrough
	// マッチした条件のひとつ下の条件の処理も実行させる
	switch number {
		case 1:
			fmt.Println("number == 1")
			fallthrough

		// case 1 が実行された後に実行される
		default: fmt.Println("default")
	}
}


func loop() {
	for i := 0; i < 10; i++ {
		fmt.Print(i, " ")
	}
	fmt.Println()
	// => 0 1 2 3 4 5 6 7 8 9

	// スライス
	numbers := []string { "a", "b", "c" }
	for index, value := range numbers {
		fmt.Print("(", index, ", ", value, ")", " ")
	}
	fmt.Println()
	// => (0, a) (1, b) (2, c)
}


func input() {

	// Scanを利用する方法もある
	var scanner *bufio.Scanner = bufio.NewScanner(os.Stdin)
	fmt.Print("input: ")
	scanner.Scan()
	inputString := scanner.Text()
	fmt.Println(inputString)
}


func convert() {
	// 文字列から数値へ変換する
	num, err := strconv.Atoi("12345")
	if err == nil {
		fmt.Println("Success:", num)
		// => Success: 12345
	}

	// 数値から文字列へ変換する
	str := strconv.Itoa(num)
	fmt.Println("String: ", str)
	// => String:  12345
}


type colorType struct {
	red int
	green int
	blue int
}

func (color *colorType) print() {
	fmt.Println(color)
}

func structExample() {
	var color colorType
	color.red = 0
	fmt.Println(color)
	// => {0 0 0}

	color = colorType{255, 0, 0}
	color.print()
	// => &{255 0 0}
}


type Point struct {
	X int `label:"x" read:"えっくす"`
	Y int `label:"y" read:"わい"`
}

func tag() {
	point := Point{ 2, 4 }
	pointType := reflect.TypeOf(point)
	for i := 0; i < pointType.NumField(); i++ {
		field := pointType.Field(i)
		fmt.Println(field.Name, field.Tag, field.Tag.Get("read"))
	}
	// =>
	// X label:"x" read:"えっくす" えっくす
	// Y label:"y" read:"わい" わい
}


type Foo struct {
	str string
}

type Piyo interface {
	print()
}

func (foo Foo) print() {
	fmt.Println(foo)
}

func interfaceExample() {
	var foo = interface{}(Foo{"example"})
	piyo, boolean := foo.(Piyo)

	if boolean {
		fmt.Println("foo:", piyo, foo, boolean)
		// => foo: {example} true

		piyo.print()
		// => {example}
	}
}


func main() {

	// 標準出力
	fmt.Print("Golang\n")

	// 変数
	variable()

	// ポインタ
	pointer()

	// 四則演算
	calculation()

	// 可変長引数
	variadicArguments(1, 2, 3)

	// 配列・スライス
	array()

	// マップ
	mapExample()

	// 条件分岐
	conditionalBranch()

	// 繰り返し
	loop()

	// 標準入力
	// input()

	// 型変換
	convert()

	// 無名関数
	closure := func() int {
		return 0
	}
	fmt.Println(closure())
	// => 0

	// 名前付き戻り値
	named := func()(result int) {
		result = 2
		return
	}
	fmt.Println(named())
	// => 2

	// 構造体
	structExample()

	// タグ
	tag()

	// インターフェース
	interfaceExample()
}
