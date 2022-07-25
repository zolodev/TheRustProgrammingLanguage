/*****************************************************************************
 * Filename      : main.rs
 * Created       : Thu Jul 21 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 15
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

fn main() {
    let b = Box::new(5); // Using Box stores the value 5 on the heap
    println!("b = {b}");
}
