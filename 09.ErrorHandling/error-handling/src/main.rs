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
    // Error, when using '?' operator
    let f = File::open("hello.txt")?;
    // -> - this function should return `Result` or `Option` to accept `?`
}
