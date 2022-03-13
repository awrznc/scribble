package main

import (
	"reflect"
	"testing"
)

func TestGalMSort(t *testing.T) {

	// ソート対象とソートキーに値が存在していた場合OK
	result, err := GalMSort(&[]string{"皇様", "玉様", "大様"}, "王様")
	if !reflect.DeepEqual(result, []string{"玉様", "大様", "皇様"}) || err != nil {
		t.Error("結果が正しくありません", result)
	}

	// ソート対象が空でもエラーにならない
	result, err = GalMSort(&[]string{}, "test")
	if !reflect.DeepEqual(result, []string{}) || err != nil {
		t.Error("結果が正しくありません", result)
	}

	// ソートキーが空でもエラーにならない
	result, err = GalMSort(&[]string{"test"}, "")
	if !reflect.DeepEqual(result, []string{"test"}) || err != nil {
		t.Error("結果が正しくありません", result)
	}
}
