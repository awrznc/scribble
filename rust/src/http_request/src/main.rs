// use std::net::{TcpStream, ToSocketAddrs};
// use std::io::{BufReader, BufWriter, Write};

// fn main() {

//     let mut addrs = "localhost:8080".to_socket_addrs().unwrap();
//     if let Some(addr) = addrs.find(|x| (*x).is_ipv4()) {
//         match TcpStream::connect(addr) {
//             Err(_) => {
//                 println!("Failed.");
//             }
//             Ok(stream) => {
//                 println!("Success.");
//                 let mut reader = BufReader::new(&stream);
//                 let mut writer = BufWriter::new(&stream);

//                 writer.write("GET / HTTP/1.1\nHost: localhost:8080".as_bytes()).unwrap();
//                 reader.get_mut();
//                 println!("{:?}", reader);
//             }
//         }
//     } else {
//         eprintln!("Invalid Host:Port Number");
//     }
// }


// use std::net::{TcpStream};
// use std::io::{Read, Write};
// use std::str::from_utf8;

// fn main() {
//     match TcpStream::connect("localhost:8080") {
//         Ok(mut stream) => {
//             println!("Successfully connected to server in port 8080");

//             let msg = b"GET / HTTP/1.1";

//             stream.write(msg).unwrap();
//             println!("Sent Hello, awaiting reply...");

//             let mut data = [0 as u8; 14];
//             match stream.read_exact(&mut data) {
//                 Ok(_) => {
//                     if &data == msg {
//                         println!("Reply is ok!");
//                     } else {
//                         let text = from_utf8(&data).unwrap();
//                         println!("Unexpected reply: {}", text);
//                     }
//                 },
//                 Err(e) => {
//                     println!("Failed to receive data: {}", e);
//                 }
//             }
//         },
//         Err(e) => {
//             println!("Failed to connect: {}", e);
//         }
//     }
//     println!("Terminated.");
// }


use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}
