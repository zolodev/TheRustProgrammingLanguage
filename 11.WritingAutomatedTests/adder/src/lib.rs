/*****************************************************************************
 * Filename      : lib.rs
 * Created       : Tue Jul 12 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 11-01 writing tests
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}
