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

    let mut largets = number_list[0];

    for number in number_list {
        if number > largets {
            largets = number
        }
    }

    println!("The largets number is {}", largets);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largets = number_list[0];

    for number in number_list {
        if number > largets {
            largets = number
        }
    }

    println!("The largets number is {}", largets);
}
