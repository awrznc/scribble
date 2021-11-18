package main

import (
	"testing"
)

func TestPiyo(tester *testing.T) {
	if Piyo() != 0 {
		tester.Error("数値が一致しません")
	}
}
