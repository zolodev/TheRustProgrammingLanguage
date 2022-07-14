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
use std::process;

fn main() {
    // Collect the arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Saving the arguments
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

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

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
