package main
/*
#cgo LDFLAGS: -ldl
#include <dlfcn.h>
void call_func(void* p) {
    void (*func)() = p;
    func();
}
*/
import "C"
func main() {
    handle := C.dlopen(C.CString("./target/release/libfoo.so"), C.RTLD_LAZY)
    if handle == nil {
        panic("Failed to open shared object.")
    }
    defer C.dlclose(handle)
    hello := C.dlsym(handle, C.CString("hello"))
    if hello == nil {
        panic("Could not found function pointer.")
    }
    C.call_func(hello)
}
// CGOはGO言語標準。
