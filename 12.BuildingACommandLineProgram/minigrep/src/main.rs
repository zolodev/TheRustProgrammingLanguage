/*****************************************************************************
 * Filename      : main.rs
 * Created       : Wed Jul 13 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Building a Command Line Program minigrep, a small program
 *                 to search files for a specific string similar to the
 *                 program `grep`
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use std::env;
fn main() {
    // Collect the arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Saving the arguments
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("in file {}", filename);
}
