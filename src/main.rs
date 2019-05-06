#[macro_use]
mod macros;
mod http;
mod thread_pool;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

use http::prelude::*;
use thread_pool::ThreadPool;

pub const PORT: usize = 7878;
pub const HTML_DIR: &str = "html/";

fn main() {
    let listener = TcpListener::bind(&format!("0.0.0.0:{}", PORT)).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        eprintln!("New incoming request");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let path_index = b"GET / HTTP/1.1\r\n";
    let path_sleep = b"GET /sleep HTTP/1.1\r\n";

    let response = if buffer.starts_with(path_index) {
        Response::File(HttpStatus::Ok, HtmlFile::new("index.html"))
    } else if buffer.starts_with(path_sleep) {
        use std::time::Duration;

        thread::sleep(Duration::from_secs(1));
        Response::Text(HttpStatus::Ok, String::from("slept"))
    } else {
        Response::Text(HttpStatus::NotFound, String::from("404 Not Found"))
    };

    stream.write(response.raw().as_bytes()).unwrap();
    stream.flush().unwrap();
}
