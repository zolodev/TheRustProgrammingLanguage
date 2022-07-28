/*****************************************************************************
 * Filename      : lib.rs
 * Created       : Thu Jul 28 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 17
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

trait State {}

struct Draft {}

impl State for Draft {}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(&self) {
        todo!()
    }

    pub fn approve(&self) {
        todo!()
    }

    pub fn content(&self) -> &str {
        ""
    }
}
