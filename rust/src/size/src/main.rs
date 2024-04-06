fn main() {
    struct EmptyStruct {}
    let size = std::mem::size_of::<EmptyStruct>();
    assert_eq!(size, 0);

    #[allow(dead_code)]
    enum IncludeEmptyStruct {
        A(EmptyStruct),
        B(EmptyStruct),
        C(EmptyStruct),
        D(EmptyStruct),
    }
    let size = std::mem::size_of::<IncludeEmptyStruct>();
    assert_eq!(size, 1); // 1 Byte
}
