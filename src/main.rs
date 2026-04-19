use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request received: {:#?}", request);

    let response = handle_response(request);
    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_response(request: Vec<String>) -> &'static str {
    match request[0].as_str() {
        "GET / HTTP/1.1" => "HTTP/1.1 200 OK\r\n\r\nOla",
        "GET /about HTTP/1.1" => "HTTP/1.1 200 OK\r\n\r\nWill be added soon",
        "GET /admin HTTP/1.1" => "HTTP/1.1 403 FORBIDDEN\r\n\r\nForbidden",
        _ => "HTTP/1.1 404 NOT FOUND\r\n\r\n Route not found",
    }
}
