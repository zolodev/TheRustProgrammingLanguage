use std::io;

fn main() {

    println!("Hello, world!");

    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);   // The value of x is: 5
}