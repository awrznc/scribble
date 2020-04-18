use std::fs;
use std::io::Write;

fn main() {
    let json = include_str!("./assets/data.json");
    let data_string = ["json_parse!(", json,")"].concat();
    let data_bytes = data_string.as_bytes();
    let mut file = fs::File::create("./target/converted_data.rs").unwrap();
    file.write(data_bytes).unwrap();
}
