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
    #[should_panic]
    fn greater_then_100() {
        Guess::new(200);
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }
}
