/*****************************************************************************
 * Filename      : main.rs
 * Created       : Sun Jul 10 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Validating Lifetimes in Rust book chapter 10-03
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

fn main() {
    // Preventing Dangling References with Lifetimes

    {
        let x = 5;
        let r = &x;
        println!("r: {}", r);
    }
}
