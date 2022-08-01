/*****************************************************************************
 * Filename      : lib.rs
 * Created       : Mon Aug 01 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 19
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

// More resources about Macros can be found at the following links:
// https://doc.rust-lang.org/reference/macros-by-example.html
// https://veykril.github.io/tlborm/
#[macro_export]
macro_rules! myvec {
    ( $( $x:expr ), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
