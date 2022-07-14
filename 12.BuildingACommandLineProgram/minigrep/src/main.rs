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
use std::fs;

fn main() {
    // Collect the arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Saving the arguments
    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("in file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
