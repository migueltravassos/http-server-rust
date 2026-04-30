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
    let mut request = Vec::new();

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        };

        if line.is_empty() {
            break;
        }

        request.push(line);
    }

    let Some(first_line) = request.get(0) else {
        return;
    };

    let response = match first_line.as_str() {
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
