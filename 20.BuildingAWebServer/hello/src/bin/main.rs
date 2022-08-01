/*****************************************************************************
 * Filename      : main.rs
 * Created       : Mon Aug 01 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 20 building a web
 *                 server
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

use hello::ThreadPool;

fn main() {
    let localhost = "127.0.0.1:7878";
    let listener = TcpListener::bind(localhost).unwrap();
    let pool = ThreadPool::new(4);

    println!("Web server started at {}", localhost);
    println!("To shut down the server, press CTRL+C");

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(_stream);
        });
    }

    println!("Shutting down...");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
