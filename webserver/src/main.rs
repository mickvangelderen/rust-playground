use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let mut args = std::env::args();

    // Ignore command.
    args.next();

    let port = args.next().unwrap_or(String::from("8080"));

    let address = format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

use std::path::Path;
use std::fs::File;
use std::io;
fn read_file_to_string<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    Ok(contents)
}

fn handle_connection(mut stream: TcpStream) {
    let request_line = b"GET / HTTP/1.1\r\n";

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    println!("{}", buffer.lines().next().unwrap().unwrap());

    if buffer.starts_with(request_line) {
        let document = read_file_to_string("index.html").unwrap();
        stream.write(
            format!("HTTP/1.1 200 OK\r\n\r\n{}", document).as_bytes()
        ).unwrap();
    } else {
        let document = read_file_to_string("404.html").unwrap();
        stream.write(
            format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", document).as_bytes()
        ).unwrap();
    }

    stream.flush().unwrap();
}
