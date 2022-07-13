/*****************************************************************************
 * Filename      : lib.rs
 * Created       : Tue Jul 12 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 11-01 writing tests
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2))
    }
}
