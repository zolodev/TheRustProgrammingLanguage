/*****************************************************************************
 * Filename      : main.rs
 * Created       : Sun Jul 10 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Validating Lifetimes in Rust book chapter 10-03
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

fn main() {
    // Preventing Dangling References with Lifetimes
    {
        let r;
        {
            let x = 5;
            r = &x; // Error, borrowed value does not live long enough
        }

        // r has lost scope from above, therfore we get the error
        println!("r: {}", r);
    }
}
