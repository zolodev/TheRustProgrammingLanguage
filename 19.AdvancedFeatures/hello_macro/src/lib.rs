/*****************************************************************************
 * Filename      : lib.rs
 * Created       : Mon Aug 01 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 19
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

pub trait HelloMacro {
    fn hello_macro();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
