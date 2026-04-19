use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let reader = BufReader::new(&mut stream);

        let request: Vec<_> = reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request received: {:#?}", request);
        let response = "HTTP/1.1 200 OK\r\n\r\nOlá Arquiteto";
        stream.write_all(response.as_bytes()).unwrap();
    }
}
