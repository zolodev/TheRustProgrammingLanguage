/**************************************************************&***************
 * Filename      : main.rs
 * Created       : Mon Jul 25 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 15
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // If a parent node is dropped, drop children
    children: RefCell<Vec<Rc<Node>>>, // if a child is dropped, parent remain
}
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!("branch parent = {:?}", branch.parent.borrow().upgrade());
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
