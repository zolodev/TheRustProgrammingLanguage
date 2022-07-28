/*****************************************************************************
 * Filename      : main.rs
 * Created       : Thu Jul 28 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 18
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using pruple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // Using let x = 5; as a pattern means the x is the PATTERN
    // means "bind what matches here to the variable x"
    // while x is the whole pattern it will bind everything to the variable x.
    // let PATTERN = EXPRESSION;

    //  |-- PATTERN
    //  |   |------- EXPRESSION
    //  v   v
    let x = 5;

    // Another example using a tuple
    // destructure a tuple creates three variables at once
    //
    //      |---- PATTERN (x, y, z)
    //      |           |------------- Matching Expression (1, 2, 3)
    //  vvvvvvvvv   vvvvvvvvv
    let (x, y, z) = (1, 2, 3); // Results x = 1, y = 2, z = 3
}
