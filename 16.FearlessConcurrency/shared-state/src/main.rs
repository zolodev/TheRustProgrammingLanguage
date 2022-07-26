/*****************************************************************************
 * Filename      : main.rs
 * Created       : Tue Jul 26 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 16
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
