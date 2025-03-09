use tr_lib::return_star;

#[test]
fn check() {
    let size = 8;
    let mut shorts = vec![0u16; size];
    unsafe {
        let len = return_star(shorts.as_mut_ptr(), size.try_into().unwrap());
        assert_eq!(8, len);
    }
    let result = String::from_utf16(&shorts).unwrap();
    let result = result.trim_end_matches(char::from(0));

    const EXPECT: &str = "🌟a🌟a🌟";
    assert_eq!(EXPECT, result);
}
