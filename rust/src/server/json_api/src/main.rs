use std::net::{TcpListener, TcpStream};
use std::writeln;
use std::thread;
use std::io::{Write, BufRead, BufReader, copy};
use std::io::prelude::*;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod response;

fn operation(stream: &mut TcpStream) {

    // convert bytes => str
    // let mut file: &[u8] = "hello".as_bytes();
    let aaaa = response::get_response().unwrap();
    let mut file: &[u8] = &aaaa.as_bytes();

    // let mut result: String = "".to_string();
    // response::get_response(&result);
    // println!("{:?}", response::get_response()?);

    // write
    writeln!(stream, "HTTP/1.1 200 OK").unwrap();
    writeln!(stream, "Content-Type: text/json; charset=UTF-8").unwrap();
    // writeln!(stream, "Content-Length: {}", file.len()).unwrap();
    writeln!(stream).unwrap();

    copy(&mut file, stream).unwrap();
}

fn handle_client(stream: TcpStream) {
    let mut stream = BufReader::new(stream);

    // メソッド、パス、規格を取得
    // ex. "GET /xxx?aaa=bbb HTTP/1.1\r\n"
    let mut first_line = String::new();

    // 受信内容を読み込む
    stream.read_line(&mut first_line).unwrap_or_else( | error |
        panic!("error during receive a line: {}", error)
    );

    // 受信内容の表示
    print!("{}", first_line);
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[..]));


    // 受信内容からレスポンス内容を作成
    let mut params = first_line.split_whitespace();
    let method = params.next();
    match method {
        Some("GET")  => operation(stream.get_mut()),
        Some("POST") => operation(stream.get_mut()),
        _ => panic!("failed to parse"),
    }
}


fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                println!("failed: {}", e)
            } Ok(stream) => {
                thread::spawn(move || { handle_client(stream) });
            }
        }
    }
}
