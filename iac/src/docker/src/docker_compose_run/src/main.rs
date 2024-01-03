use std::io::Write;
use std::net::ToSocketAddrs;

pub fn http_request(address_port: &str) -> std::io::Result<String> {
    dbg!(&address_port);

    let mut socket_addr = address_port.to_socket_addrs().unwrap();
    let ip_v4 = socket_addr.find(|v| v.is_ipv4()).unwrap();
    dbg!(&ip_v4);

    let duration = std::time::Duration::from_secs(1);
    let mut stream = std::net::TcpStream::connect_timeout(&ip_v4, duration).unwrap();
    stream.set_read_timeout(Some(duration)).unwrap();
    stream.set_write_timeout(Some(duration)).unwrap();

    let input = String::from("GET / HTTP/1.0\r\n")
        + "Host: "
        + address_port
        + "\r\n"
        + "Connection: close\r\n\r\n";
    println!("{input}");

    stream.write_all(input.as_bytes()).unwrap();
    stream.flush().unwrap();

    use std::io::Read;
    let mut result = String::new();
    stream.read_to_string(&mut result).unwrap();
    dbg!(&result);
    Ok(result)
}

fn main() {
    let address_port = std::env::var("ADDRESS_PORT").unwrap();
    http_request(&address_port).unwrap();
}

#[cfg(test)]
mod test {
    #[test]
    fn request() {
        let address_port = std::env::var("ADDRESS_PORT").unwrap();
        crate::http_request(&address_port).unwrap();
    }
}
