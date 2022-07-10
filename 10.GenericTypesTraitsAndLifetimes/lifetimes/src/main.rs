/*****************************************************************************
 * Filename      : main.rs
 * Created       : Sun Jul 10 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Validating Lifetimes in Rust book chapter 10-03
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
