/*****************************************************************************
 * Filename      : main.rs
 * Created       : Sun Jul 31 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 19
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

fn main() {
    let mut num = 5;

    let _r1 = &num as *const i32; // Immutable raw pointer to num
    let _r2 = &mut num as *mut i32; // Mutable raw pointer to num

    println!("immutable ptr: {:?}, mutable ptr: {:?}", _r1, _r2);
}
