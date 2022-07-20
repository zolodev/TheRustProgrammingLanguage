/*****************************************************************************
 * Filename      : main.rs
 * Created       : Wed Jul 20 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 14
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use mycrate::{
    add_one,
    Art::{mix, PrimaryColor},
};

fn main() {
    let arg = 5;
    let answer = add_one(arg);

    println!("{answer}"); // -> 6

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;

    mix(red, yellow);
}
