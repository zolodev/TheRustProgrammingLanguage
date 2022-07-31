/**************************************************************&***************
 * Filename      : main.rs
 * Created       : Sun Jul 31 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 19
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use core::slice;

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

    // split_at_mut() runs unsafe code to perform this action
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // another custom implementation of split_at_mut
    let (left, right) = my_split_at_mut(r, 2);
    assert_eq!(left, &mut [1, 2]);
    assert_eq!(right, &mut [3, 4, 5, 6]);
}

unsafe fn dangerous() {}

fn my_split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
