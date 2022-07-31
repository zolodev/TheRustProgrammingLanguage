/*****************************************************************************
 * Filename      : main.rs
 * Created       : Sun Jul 31 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 19
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

type Kilometers = i32;

// Alias Thunk for a long type
type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));

    let _s1: str = "Hello there!";
    let _s2: str = "How's it going?";
}

fn takes_long_type(f: Thunk) {
    todo!()
}

fn returns_long_type() -> Thunk {
    todo!()
}

fn generic<T>(t: T) {
    todo!()
}
