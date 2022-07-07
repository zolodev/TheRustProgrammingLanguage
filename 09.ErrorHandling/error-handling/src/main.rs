/*****************************************************************************
 * Filename      : main.rs
 * Created       : Thu Jul 07 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Workig through the Rust book chapter 09-02 Error Handling
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use std::fs::File;

fn main() {
    let f: Result<File, std::io::Error> = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the fil {:?}", error),
    };
}
