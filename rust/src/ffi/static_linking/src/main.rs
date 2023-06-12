#[cfg_attr(target_os = "windows", link(name = "ret", kind = "static"))]
extern "C" {
    fn return_42() -> std::ffi::c_char;
}

fn main() {
    assert_eq!(unsafe { return_42() }, 42);
}
