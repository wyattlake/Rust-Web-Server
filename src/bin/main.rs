use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use hello::ThreadPool;

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
    let mut buffer = [0; 1204];

    stream.read(&mut buffer).unwrap();

    let buffer_str = String::from_utf8_lossy(&buffer[..]);

    let pathname = get_page(&buffer_str);

    let (status_line, filename) = if !has_html(&pathname) {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    } else {
        ("HTTP/1.1 200 OK\r\n\r\n", {&*pathname})
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_page(buffer: &str) -> String {
    let words_buffer: Vec<&str> = buffer.split(' ').collect();
    let page = format!("./{}{}", words_buffer[1].replace("/", ""), ".html");
    page
}

fn has_html(filename: &str) -> bool {
    let files = fs::read_dir("./").unwrap();
    for file in files {
        let file_str = file.unwrap().path().display().to_string();
        if &filename == &file_str {
            return true
        }
    }
    false
}