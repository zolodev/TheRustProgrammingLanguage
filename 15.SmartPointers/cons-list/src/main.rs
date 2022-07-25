/*****************************************************************************
 * Filename      : main.rs
 * Created       : Thu Jul 21 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 15
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use std::{cell::RefCell, ops::Deref, rc::Rc};

use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
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
    let value = Rc::new(RefCell::new(5));

    let _list = Cons(
        Rc::new(RefCell::new(1)),
        Rc::new(Cons(
            Rc::new(RefCell::new(2)),
            Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::new(Nil))),
        )),
    );

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

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
