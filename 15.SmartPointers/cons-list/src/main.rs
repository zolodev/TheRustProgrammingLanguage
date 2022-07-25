/*****************************************************************************
 * Filename      : main.rs
 * Created       : Thu Jul 21 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 15
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use std::ops::Deref;

use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Necessary in order for the assert_eq!(5, *y) to compile.
// Missing Deref due to *y.
// Similar to how Box<T> from the standard library works.
impl<T> Deref for MyBox<T> {
    // syntax defines an associated type for Deref
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Returns a reference to the value we want to access with the *
        // and returns the first value in a tuple by calling .0 on &self
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = MyBox::new(x); // similar to &x, using Box<T> creating an instance

    assert_eq!(5, x);
    assert_eq!(5, *y); // dereference operator to follow the pointer

    // The difference between &x and Box::new(x) is that
    // we set y to be an instance of a box pointing to a copied value of x
    // rather than a reference pointing to the value of x.

    // *y is the same as *(y.deref()) behind the scene
    assert_eq!(5, *(y.deref()));

    let m = MyBox::new(String::from("Rust"));

    // This works because of the deref coercion
    hello(&m);

    // If we would not had the deref coercion the code would look like:
    hello(&(*m));

    // or ...
    hello(&(*m)[..]);

    // There is no runtime penalty for taking advantages of deref coercion!
}
