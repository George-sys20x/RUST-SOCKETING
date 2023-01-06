use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Instant;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Listening for connections on port 7878...");


    for stream in listener.incoming() {
        let start_time = Instant::now();
        let mut stream = stream.unwrap();

    
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

    
        let html_contents = fs::read_to_string("index.html").unwrap();

    
        let mut response = Vec::new();
        response.extend_from_slice(b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n");
        response.extend_from_slice(html_contents.as_bytes());

    
        stream.write(&response).unwrap();
        println!("Response sent: {:?}", response);
        println!("Connection took {:?}", start_time.elapsed());
    }
}
