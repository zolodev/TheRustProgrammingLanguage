/*****************************************************************************
 * Filename      : main.rs
 * Created       : Sun Jul 10 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Validating Lifetimes in Rust book chapter 10-03
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

// Struct Lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Part: {}", i.part);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
