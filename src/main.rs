use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Couldnt bind");
    for stream in listener.incoming() {
        match stream {
            Ok(t) => handle_connection(t),
            Err(e) => eprintln!("Failed to establish connection {}.", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let request: Vec<_> = reader
        .lines()
        .filter_map(|result| match result {
            Ok(r) => Some(r),
            Err(e) => {
                eprintln!("Error: {}", e);
                None
            }
        })
        .take_while(|line| !line.is_empty())
        .collect();

    if request.is_empty() {
        return;
    }

    let response = match request[0].as_str() {
        "GET / HTTP/1.1" => "HTTP/1.1 200 OK\r\n\r\nHello",
        "GET /about HTTP/1.1" => "HTTP/1.1 200 OK\r\n\r\nSoon",
        _ => "HTTP/1.1 404 NOT FOUND\r\n\r\nNOT FOUND",
    };

    match stream.write_all(response.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
