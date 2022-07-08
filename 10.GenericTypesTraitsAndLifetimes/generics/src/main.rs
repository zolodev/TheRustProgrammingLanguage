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

    let result = largest(&number_list);
    println!("The largets number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largets number is {}", result);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &number in list {
        if number > largest {
            largest = number
        }
    }
    largest
}
