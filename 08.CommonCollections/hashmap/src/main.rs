/****************************************************************************
 * Filename      : main.rs
 * Created       : Wed Jul 06 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 08-03 HashMaps
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // Word counter
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map); // -> {"hello": 1, "world": 2, "wonderful": 1}
}
