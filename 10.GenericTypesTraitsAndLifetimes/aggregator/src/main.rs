/*****************************************************************************
 * Filename      : main.rs
 * Created       : Sat Jul 09 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 10-02
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use aggregator::{notify, NewsArticle, Summary, Tweet};
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

    // println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanly Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };

    // Calling notify and passing triat as an paramenter
    println!("New article available! {:?}", notify(&tweet));
    println!("New article available! {}", article.summarize());
}
