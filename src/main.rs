use std::io::prelude::*;
use std::fs;
use std::path::Path;
use std::net::TcpListener;
use std::net::TcpStream;

macro_rules! path {
    ( $( $x:expr ),* ) => {
        Path::new(".")
            $(
                .join($x.to_string())
            )*
    }
}

const PORT: usize = 7878;
const HTML_DIR: &str = "html/";

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

    let response;
    let index_get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(index_get) {
        let contents = fs::read_to_string(path!(HTML_DIR, "index.html")).unwrap();
        response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    } else {
        response = String::from("HTTP/1.1 404 Not Found\r\n\r\n");
    }

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
