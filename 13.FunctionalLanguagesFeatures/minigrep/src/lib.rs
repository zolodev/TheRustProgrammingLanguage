/*****************************************************************************
 * Filename      : lib.rs
 * Created       : Fri Jul 15 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Adding the lib.rs according to Rust idiom and best
 *                 practices. This file will contain the app logic and tests.
 *                 Optimizing according to chapter 13
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    /// Creates a new implementation of Config
    ///
    /// The config takes two args from cli.
    /// First is the `search query`.
    /// Seond is the filename (the file to seach in).
    ///
    /// # Errors
    ///
    /// `Didn't get a query string` if no search string was provided.
    /// `Didn't get a file name` if no filename was provided.
    ///
    /// # Example
    /// ```should_panic
    /// use std::{env, process};
    /// use minigrep::Config;
    ///
    /// let config = Config::new(env::args()).unwrap_or_else(|err| {
    ///     eprintln!("Problem parsing arguments: {}", err);
    ///     process::exit(1);
    /// });
    /// ```
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}

/// Run `minigrep` with the configuration
/// Depending on the configuration while running `minigrep`
/// it will use the function `search_sensitive` as default
/// or if `IGNORE_CASE` env variable is set, it will use `search_insensitive`.
///
/// # Errors
///
/// `Didn't get a query string` if no search string was provided.
/// `Didn't get a file name` if no filename was provided.
///
/// # Example
///
/// ```should_panic
/// use std::{env, process};
/// use minigrep::Config;
///
/// let config = Config::new(env::args()).unwrap_or_else(|err| {
///     eprintln!("Problem parsing arguments: {}", err);
///     process::exit(1);
/// });
///
/// if let Err(e) = minigrep::run(config) {
///     eprintln!("Application error: {}", e);
///     process::exit(1);
/// }
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.ignore_case {
        search_insensitive(&config.query, &contents)
    } else {
        search_sensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

fn search_sensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .map(str::trim)
        .collect()
}

fn search_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .map(str::trim)
        .collect()
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
            vec!["Rust:", "Trust me."],
            search_insensitive(query, contents)
        );
    }
}
