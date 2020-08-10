// #[repr(C)]
// #[derive(std::fmt::Debug, Clone, Copy, Default)]
// struct Alpha {
//     code: [u8; 2]
// }

// #[repr(C)]
// #[derive(std::fmt::Debug, Clone)]
// struct Color {
//     code: [u8; 6],
//     alpha: *mut Alpha
// }

// #[link(name="color")]
// extern "C" {
//     fn new() -> Color;
//     fn set_code(color: *mut Color, code: *const std::os::raw::c_char) -> i32;
//     fn ppp(c: *const std::os::raw::c_char) -> i32;
// }

// #[repr(C)]
// struct Database {
//     d: i32
// }

// type PrivateDatabase = *mut Database;
type PublicDatabase = *mut std::ffi::c_void;

#[link(name="color")]
extern "C" {
    fn database_init() -> PublicDatabase;
    fn set_num(set_data: PublicDatabase, num: i32) -> i32;
    fn set_name(set_data: PublicDatabase, name: *const std::os::raw::c_char) -> i32;
    fn print_data(data: PublicDatabase) -> i32;
    fn get_num(data: PublicDatabase) -> i32;
    fn get_name(data: PublicDatabase, name: *mut std::os::raw::c_char) -> i32;
    fn database_delete(data: PublicDatabase) -> i32;
}

// use std::ffi::CString;
// use std::os::raw::c_char;
fn main() {
    // let color: Color;
    // let mut color = Box::new(Default::default());
    // let cptr: *const c_char = CString::new("hoge").unwrap().into_raw();
    // unsafe {
        // let color = new();
        // println!("{:?}", color);

        // let cptr = CString::new("hello")
        //     .expect("CString::new failed")
        //     .as_ptr();

        // ppp(cptr);
        // println!("通った");
        // set_code(&mut *color, cptr);
        // CString::from_raw(color as *mut c_char);
    // };
    // println!("通った");
    // println!("{:?}", color);

    // extern { fn puts(s: *const c_char); }

    // let to_print = CString::new("Hello!").expect("CString::new failed");
    // unsafe {
    //     puts(to_print.as_ptr());
    // }


    let value = unsafe {
        let data = database_init();
        set_num(data, 100);
        set_name(data, std::ffi::CString::new("Hello!").expect("CString::new failed").as_ptr());
        print_data(data);

        // use std::ffi::CString::from_raw;
        // println!("{:?}", std::ffi::CString::new("Hello!").unwrap().from_raw());
        // let num = get_num(data);
        // let mut ssss: [u8; 8] = [0; 8];
        // let mut name = ssss.as_mut_ptr();
        let mut ssss = std::ffi::CString::new("").expect("CString::new failed");
        let mut name = ssss.into_raw();
        get_name(data, name);
        database_delete(data);
        // *name.offset(0) as u8 as char // H
        std::ffi::CString::from_raw(name)
        // ssss.iter().map(|&s| s as char).collect::<String>()
    };

    println!("{:?}", value);
}
