/*****************************************************************************
 * Filename      : main.rs
 * Created       : Mon Aug 01 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 20 builden a web
 *                 server
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
