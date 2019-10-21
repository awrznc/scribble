use std::io;
use std::io::Read;
use std::fs::File;

// ファイルを読み込むジェネリック関数
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut s = String::new();
//
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// 上記を糖衣構文へ置き換えたのがこちら
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}


fn main() {
    // error!
    let _f = read_username_from_file().unwrap();
}
