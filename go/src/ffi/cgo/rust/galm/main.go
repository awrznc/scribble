package main

/*
// macOSでクロスコンパイルする場合は -L./galm_wrapper/target/x86_64-unknown-linux-musl/release/ にする
#cgo linux LDFLAGS: -L./galm_wrapper/target/release/ -lgalm_wrapper -Wl,--no-as-needed -ldl
#cgo darwin LDFLAGS: -L./galm_wrapper/target/release/ -lgalm_wrapper
#include <stdlib.h>
#include "./include/galm.h"

// NOTE:
// Goの配列は静的な値で大きさを指定する必要があるため、余裕を持った定数値を設定する必要がある
// 今回はとりあえず255とする
#define MAX_SLICE_SIZE 0xff
*/
import "C"

import (
	"fmt"
	"unsafe"
)

func sort(sortTarget *[]string, sortKey string) ([]string, error) {

	// validate
	var result []string
	if len(*sortTarget) > C.MAX_SLICE_SIZE {
		return result, fmt.Errorf("sort: slice size is too large")
	}
	if len(*sortTarget) == 0 {
		return []string{}, nil
	}

	// target
	var cArray unsafe.Pointer = C.malloc( C.size_t(len(*sortTarget)) * C.size_t(unsafe.Sizeof(uintptr(0))) )
	var cSlice = (*[C.MAX_SLICE_SIZE]*C.char)(cArray)
	for index, substring:= range *sortTarget {
		cSlice[index] = C.CString(substring)
		defer C.free(unsafe.Pointer(cSlice[index]))
	}
	target := (**C.char)(cArray)
	defer C.free(unsafe.Pointer(cArray))

	// key
	key := C.CString(sortKey)
	defer C.free(unsafe.Pointer(key))

	// size
	size := C.size_t(len(*sortTarget))

	// sort
	C.galm_sort(target, size, key)

	// generate result
	for index := 0; index < len(*sortTarget); index++ {
		result = append(result, C.GoString(cSlice[index]))
	}

	return result, nil
}

func main() {
	sortKey := "王様"
	sortTarget := []string{"皇様", "玉様", "大様"}
	result, _ := sort(&sortTarget, sortKey)
	fmt.Println(result);
}
