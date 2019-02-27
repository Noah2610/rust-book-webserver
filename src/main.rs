use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

const PORT: usize = 7878;

fn main() {
    let listener = TcpListener::bind(&format!("0.0.0.0:{}", PORT)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
