/*****************************************************************************
 * Filename      : main.rs
 * Created       : Sun Jul 31 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 19
*****************************************************************************/

//! In this example we will overload the '+' operator

#![warn(clippy::all, clippy::pedantic)]

use std::ops::Add;

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

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
