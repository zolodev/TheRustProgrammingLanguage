/*****************************************************************************
 * Filename      : main.rs
 * Created       : Wed Jul 20 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 13
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}
