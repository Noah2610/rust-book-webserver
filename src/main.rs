#[macro_use]
mod macros;
mod http;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use http::prelude::*;

pub const PORT: usize = 7878;
pub const HTML_DIR: &str = "html/";

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

    let index_get = b"GET / HTTP/1.1\r\n";

    let response = if buffer.starts_with(index_get) {
        Response::File(HttpStatus::Ok, HtmlFile::new("index.html"))
    } else {
        Response::Text(HttpStatus::NotFound, String::from("404 Not Found"))
    };

    stream.write(response.raw().as_bytes()).unwrap();
    stream.flush().unwrap();
}
