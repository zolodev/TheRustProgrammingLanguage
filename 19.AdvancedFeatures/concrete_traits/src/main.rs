/*****************************************************************************
 * Filename      : main.rs
 * Created       &: Sun Jul 31 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 19
*****************************************************************************/

//! In this example we will overload the '+' operator

#![warn(clippy::all, clippy::pedantic)]

use std::{fmt, ops::Add};

#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self::Output {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

/// Implementing the Add trait to overload the `+`operator for Point
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is you captain speaking...");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}
struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let m1 = Millimeters(15);
    let m2 = Meters(1);
    let m3 = m1 + m2;

    println!("m3: {:?}", m3);

    // Fully qualified Syntax for methods with the same name
    let person = Human;
    person.fly();
    // let wizard = Wizard;
    // wizard.fly() //  Does not work.
    Wizard::fly(&person);
    Pilot::fly(&person);

    println!("A baby dog is called a {}", Dog::baby_name());

    // Does not compile, Due to Error, cannot infer type
    // Because Rust can not know which implementation we want
    // println!("A baby dog is called a {}", Animal::baby_name());

    // Instead we need to disambiguate and be more explicit
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let p1 = Point { x: 3, y: 1 };
    p1.outline_print();
    println!("p1 withouth outlining: {}", p1);
}
