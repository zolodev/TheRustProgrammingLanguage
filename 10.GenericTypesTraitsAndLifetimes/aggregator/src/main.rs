/*****************************************************************************
 * Filename      : main.rs
 * Created       : Sat Jul 09 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 10-02
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use aggregator::{Summary, Tweet};
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people
        ",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize())
}
