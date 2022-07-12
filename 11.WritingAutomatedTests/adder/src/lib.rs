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
    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
