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

use std::{env, process};

use minigrep::Config;

fn main() {
    // Collect the arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Saving the arguments
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Run the program using the configuration
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
