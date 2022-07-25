/*****************************************************************************
 * Filename      : main.rs
 * Created       : Thu Jul 21 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 15
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use crate::List::{Cons, Nil};

enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
