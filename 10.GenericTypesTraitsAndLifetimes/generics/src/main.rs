/*****************************************************************************
 * Filename      : main.rs
 * Created       : Fri Jul 08 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 10-00 Generics
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largets number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largets number is {}", result);

    // This can also be used with two of the same type
    let p: Point<f32> = Point { x: 5.0, y: 1.0 };

    println!("p.x = {}", p.distance_from_origin());
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
