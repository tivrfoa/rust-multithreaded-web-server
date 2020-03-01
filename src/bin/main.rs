use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

use hello::ThreadPool;

const STATUS_LINE_200: &str = "HTTP/1.1 200 OK\r\n\r\n";
const STATUS_LINE_404: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

const ROUTE_GET: &[u8] = b"GET / HTTP/1.1\r\n";
const ROUTE_GET_SLEEP: &[u8] = b"GET /sleep HTTP/1.1\r\n";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
    // for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        // println!("Connection established!");

        pool.execute(|| {
            handle_connection(stream); 
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let (status_line, filename) = if buffer.starts_with(ROUTE_GET) {
        (STATUS_LINE_200, "hello.html")
    } else if buffer.starts_with(ROUTE_GET_SLEEP) {
        thread::sleep(Duration::from_secs(5));
        (STATUS_LINE_200, "sleep.html")
    } else {
        (STATUS_LINE_404, "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
