/*****************************************************************************
 * Filename      : main.rs
 * Created       : Thu Jul 07 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Workig through the Rust book chapter 09-02 Error Handling
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use std::fs::File;
// use std::io::ErrorKind;

fn main() {
    // let f: Result<File, std::io::Error> = File::open("hello.txt");

    // let _f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file {:?}", e),
    //         },
    //         _other_error => {
    //             panic!("Problem opening the file {:?}", _other_error)
    //         }
    //     },
    // };

    // // Alternative using closure
    // let _f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt")
    //             .unwrap_or_else(|error| panic!("Problem creating the file: {:?}", error))
    //     } else {
    //         panic!("Problem opening the file: {:?}", error)
    //     }
    // });

    let _f = File::open("hello.txt").expect("Failed to open hello.txt");
}
