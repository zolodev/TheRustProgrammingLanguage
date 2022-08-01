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

    let _f: Thunk = Box::new(|| println!("hi"));

    let _a = takes_long_type(Box::new(foo));
    let _b = returns_long_type();
    let _c = generic("hi");
}
fn foo() {
    todo!()
}

fn takes_long_type(_f: Thunk) {
    todo!()
}

fn returns_long_type() -> Thunk {
    todo!()
}

// fn generic<T>(t: T) {
// is the same as
// fn generic<T: Sized>(t: T) {
// A more relaxed way to do it
// fn generic<T: ?Sized>(t: &T) {
// ?Sized means “T may or may not be Sized”
fn generic<T: ?Sized>(_t: &T) {
    todo!()
}
