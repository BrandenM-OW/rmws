use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs,
    thread,
    time::Duration,
};

use rmws::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4200").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buffer = BufReader::new(&mut stream);
    let http_request = capture_request(buffer);

    if http_request[0].contains("GET / HTTP/1.1") {
        respond_with_index(&mut stream);
    } else if http_request[0].contains("GET /sleep HTTP/1.1") {
        thread::sleep(Duration::from_secs(5));
        respond_with_index(&mut stream);
    } else {
        respond_with_404(&mut stream);
    }
}

fn capture_request(buffer: BufReader<&mut TcpStream>) -> Vec<String> {
    let mut http_request: Vec<String> = Vec::new();
    for line in buffer.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            break;
        }
        http_request.push(line);
    }
    http_request
}

fn respond_with_index(stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let content_type = "Content-Type: text/html; charset=utf-8";
    let content = fs::read_to_string("html/index.html").unwrap();
    let content_length = format!("Content-Length: {}", content.len());
    let response = format!(
        "{}\r\n{}\r\n{}\r\n\r\n{}",
        status_line, content_type, content_length, content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn respond_with_404(stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let content_type = "Content-Type: text/html; charset=utf-8";
    let content = fs::read_to_string("html/404.html").unwrap();
    let content_length = format!("Content-Length: {}", content.len());
    let response = format!(
        "{}\r\n{}\r\n{}\r\n\r\n{}",
        status_line, content_type, content_length, content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
