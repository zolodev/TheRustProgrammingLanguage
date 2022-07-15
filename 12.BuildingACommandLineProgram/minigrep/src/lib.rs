/*****************************************************************************
 * Filename      : lib.rs
 * Created       : Fri Jul 15 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Adding the lib.rs according to Rust idiom and best
 *                 practices. This file will contain the app logic and tests.
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    fs::read_to_string(config.filename)?;

    Ok(())
}

fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    // TODO:
    // Iterate through each line of the contents.
    // Check whether the line contains our query string.
    // If it does, add it to the list of values we’re returning.
    // If it doesn’t, do nothing.
    // Return the list of results that match.

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
