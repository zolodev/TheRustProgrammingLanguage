/*****************************************************************************
 * Filename      : main.rs
 * Created       : Sun Jul 31 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 19
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

//! Foreign Function Interface (FFI)
//!
//! This is a demonstration of how to interact with an external C module.

// Accessing C library and the function abs()
extern "C" {
    fn abs(input: i32) -> i32;
}

// Creating an interface that can be called from C
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
