/*****************************************************************************
 * Filename      : lib.rs
 * Created       : Wed Jul 20 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 14
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

//! # My Crate
//! `mycrate` is a collection of utilities to make performing certain
//! calculations more convenient.

//! # Art
//!
//! A library for modeling articstic concepts.

pub mod Art {
    pub use self::kinds::PrimaryColor;
    pub use self::kinds::SecondaryColor;
    pub use self::utils::mix;

    pub mod kinds {
        /// The primary colors according to the RYB color model.
        pub enum PrimaryColor {
            Red,
            Yellow,
            Blue,
        }

        /// The secondary colors according to the RYB color model.
        pub enum SecondaryColor {
            Orange,
            Green,
            Purple,
        }
    }

    pub mod utils {
        use super::kinds::*;

        /// Combines two primary colors in equal amounts to create
        /// a secondary color.
        pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
            todo!()
        }
    }
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = mycrate::add_one(arg);
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(tests)]
mod tests {
    fn add_one() {
        let arg = 2;
        let result = add_one(arg);

        assert_eq!(3, result);
    }
}
