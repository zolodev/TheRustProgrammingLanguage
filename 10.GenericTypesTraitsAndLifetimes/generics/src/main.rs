/*****************************************************************************
 * Filename      : main.rs
 * Created       : Fri Jul 08 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 10-00 Generics
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U, // Will fix the problem using different types
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largets number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largets number is {}", result);

    let wont_work = Point { x: 5, y: 4.0 };

    println!("wont_work point {:?}", wont_work);
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
// for &item in list {
//     if item > largest {
//         largest = item
//     }
// }
//     largest
// }
