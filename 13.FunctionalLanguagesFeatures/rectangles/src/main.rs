/*****************************************************************************
 * Filename      : main.rs
 * Created       : Wed Jul 20 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 13
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
