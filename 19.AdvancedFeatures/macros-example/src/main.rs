/*****************************************************************************
 * Filename      : main.rs
 * Created       : Sun Jul 31 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 19
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use macros_example::myvec;

fn main() {
    let v1 = myvec![1, 2, 3];
    println!("My vec macro: {:?}", v1);
}
