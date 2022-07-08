/*****************************************************************************
 * Filename      : main.rs
 * Created       : Fri Jul 08 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 10-00 Generics
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largets number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largets number is {}", result);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &number in list {
        if number > largest {
            largest = number
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &number in list {
        if number > largest {
            largest = number
        }
    }
    largest
}
