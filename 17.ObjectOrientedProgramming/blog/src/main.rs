/*****************************************************************************
 * Filename      : main.rs
 * Created       : Thu Jul 28 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 17
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
