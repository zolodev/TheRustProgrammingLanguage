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
    let contents = fs::read_to_string(config.filename)?;

    for line in search_sensitive(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

fn search_sensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // Iterate through each line of the contents.
    for line in contents.lines() {
        // Check whether the line contains our query string.
        if line.contains(query) {
            // If it does, add it to the list of values we’re returning.
            results.push(line.trim());
        }

        // If it doesn’t, do nothing.
    }

    // Return the list of results that match.
    results
}

fn search_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.";

        assert_eq!(
            vec!["Rust: ", "Trust me."],
            search_insensitive(query, contents)
        );
    }
}
