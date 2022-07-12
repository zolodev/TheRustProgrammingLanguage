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
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}
