use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

const STATUS_LINE_200: &str = "HTTP/1.1 200 OK\r\n\r\n";
const STATUS_LINE_404: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        println!("Connection established!");

        handle_connection(stream);
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 512];

        stream.read(&mut buffer).unwrap();

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        // byte string
        let get = b"GET / HTTP/1.1\r\n";

        let (status_line, filename) = if buffer.starts_with(get) {
            (STATUS_LINE_200, "hello.html")
        } else {
            (STATUS_LINE_404, "404.html")
        };

        let contents = fs::read_to_string(filename).unwrap();
        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
