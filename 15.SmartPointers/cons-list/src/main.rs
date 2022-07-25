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
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = Box::new(x); // similar to &x, using Box<T> creating an instance

    assert_eq!(5, x);
    assert_eq!(5, *y); // dereference operator to follow the pointer

    // The difference between &x and Box::new(x) is that
    // we set y to be an instance of a box pointing to a copied value of x
    // rather than a reference pointing to the value of x.
}
