use core::usize;
use std::{
    io::{prelude::*, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use backend_server::ThreadPool;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&stream);
    let mut headers = String::new();

    let mut content_length = 0;
    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line).unwrap();
        
        if line == "\r\n" {
            break;
        }

        if let Some(len) = line.strip_prefix("Content-Length: ") {
            content_length = len.trim().parse::<usize>().unwrap();
        }
        headers.push_str(&line);
    }

    println!("Headers: {headers}");

    let mut body = vec![0; content_length];
    buf_reader.read_exact(&mut body).unwrap();
    let body_str = String::from_utf8(body).unwrap();

    println!("Body: {body_str}");

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
