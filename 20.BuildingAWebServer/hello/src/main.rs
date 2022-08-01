/*****************************************************************************
 * Filename      : main.rs
 * Created       : Mon Aug 01 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 20 builden a web
 *                 server
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established!");
    }
}
