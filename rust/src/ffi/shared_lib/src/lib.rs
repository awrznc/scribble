#[no_mangle]
pub extern fn hello() -> u8 {
    println!("Hello");
    return 0;
}
