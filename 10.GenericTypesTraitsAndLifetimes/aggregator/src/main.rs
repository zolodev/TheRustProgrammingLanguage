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
    notify(&tweet);

    // Calling article is not possible because it does not have a bound
    // to the Display, this will throw an error:
    // -> required by this bound in `notify`
    // notify(&article);

    // Calling Tweet with Display
    println!("New article available! {}", &tweet);
    println!("New article available! {}", article.summarize());
}
