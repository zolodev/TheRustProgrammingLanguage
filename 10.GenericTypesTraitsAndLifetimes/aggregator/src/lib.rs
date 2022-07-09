/*****************************************************************************
 * Filename      : lib.rs
 * Created       : Sat Jul 09 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book 10-02 Traits
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
