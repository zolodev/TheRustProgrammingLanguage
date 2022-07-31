/**************************************************************&***************
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

    let _address = 0x012345usize;
    let _r3 = _address as *const i32;
    println!("unknown value ptr: {:?}", _r3);

    unsafe {
        println!("_r1 is {}", *_r1); // _r1 is 5
        println!("_r2 is {}", *_r2); // _r2 is 5

        // println!("_r3 is {}", *_r3); // Error, segmentation fault
    }

    unsafe {
        // Can only call dangours (unsafe) function from within
        // either another unsafe function or an unsafe block like this.
        dangerous();
    }
}

unsafe fn dangerous() {}
