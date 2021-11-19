package main

import (
	"fmt"
)

func Piyo() int {
	return 0
}

type PiyoType interface {
	Foo() int
}

type PiyoStruct struct {}

func (piyo *PiyoStruct) Foo() int {
	return 1
}

func Bar(piyoType PiyoType) int {
	return piyoType.Foo()
}

func main() {
	fmt.Println(Piyo())

	piyoStruct := PiyoStruct{}
	fmt.Println(piyoStruct.Foo())
}
