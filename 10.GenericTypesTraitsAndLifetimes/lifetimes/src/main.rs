/*****************************************************************************
 * Filename      : main.rs
 * Created       : Sun Jul 10 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Validating Lifetimes in Rust book chapter 10-03
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

/// Rust lifetime rules
/// 1. Each elided lifetime in input position becomes a distinct
/// lifetime parameter.
/// ´´´
/// pub fn name<'a: 'b, 'b>(in: &'a str, other: &'b str) -> &'a str {
///     in
/// }
/// ´´´  
/// 2. If there is exactly one input lifetime position (elided or not),
/// that lifetime is assigned to all elided output lifetimes.
///
/// ´´´
/// pub fn first(input: &str) -> (&str, &str) {
///     (input, input)
/// }
/// ´´´
/// 3. If there are multiple input lifetime positions, but one of them is
/// &self or &mut self, the lifetime of self is assigned to all elided
/// output lifetimes.
/// ´´´
/// struct Config {
///     name: String
/// }
///
/// impl Config {
///     fn name(&self, other_useless_val: &str) -> &str {
///         &self.name // <- same lifetime as &self
///     }
/// }
/// ´´´
/// 4. Otherwise, it is an error to elide an output lifetime.
/// Source: https://togglebit.io/posts/lifetime-rules-of-rust/

// Struct Lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    //  first elision rule, because &self is the only parameter
    fn level(&self) -> i32 {
        3
    }

    // first lifetime elision rule applies, because first parameter is a &self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first };

    println!("Part: {}", i.part);

    i.announce_and_return_part("announcement");
    println!("Level: {}", i.level());

    // Static lifetime
    let s: &'static str = "I have a static lifetime.";
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
