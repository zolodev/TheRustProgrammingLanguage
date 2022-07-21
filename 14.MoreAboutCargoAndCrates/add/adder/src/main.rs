/*****************************************************************************
 * Filename      : main.rs
 * Created       : Thu Jul 21 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 14
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]



fn main() {
    let num = 10;

    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
}
