/*****************************************************************************
 * Filename      : main.rs
 * Created       : Sun Jul 31 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 19
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use macros_example::myvec;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let v1 = myvec![1, 2, 3];
    println!("My vec macro: {:?}", v1);

    Pancakes::hello_macro();
}
