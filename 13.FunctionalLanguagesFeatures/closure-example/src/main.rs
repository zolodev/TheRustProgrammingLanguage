/*****************************************************************************
 * Filename      : main.rs
 * Created       : Tue Jul 19 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working throught the Rust book chapter 13
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use std::{thread, time::Duration};

fn main() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    println!("{}", expensive_closure(2));
}
