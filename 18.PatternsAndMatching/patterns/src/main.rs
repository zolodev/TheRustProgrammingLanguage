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

    //   |-- PATTERN
    //   |   |------- EXPRESSION
    //   v   v
    let _x = 5;

    // Another example using a tuple
    // destructure a tuple creates three variables at once
    //
    //        |---- PATTERN (x, y, z)
    //        |            |------------- Matching Expression (1, 2, 3)
    //  vvvvvvvvvvvv   vvvvvvvvv
    let (_x, _y, _z) = (1, 2, 3); // Results _x = 1, _y = 2, _z = 3

    let point = (3, 5);
    print_coordinates(&point);

    // Working with Refutable and irrefutable patterns
    // a refutable pattern would be

    let some_option_value: Option<i32> = None;
    // let Some(x) = some_option_value; // Won't compile!
    // Error, pattern `None` not covered
    // Solution recomendation from the Rust compiler:
    // let x = if let Some(x) = some_option_value { x } else { todo!() };
    // Or ...

    // Instead if we do not need to cover all cases except one...
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    // This will yield a warning
    // irrefutable `if let` pattern
    if let x = 5 {
        println!("{}", x);
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 1;

    match x {
        // 1 | 2, the pipe '|' = 'or' => match on either 1 or 2
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        // 1..=5, any number between 1 and 5 will match
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Destructing values
    let p = Point { x: 0, y: 7 };

    // Destructing a struct, creates two variables 'a' and 'b' with the values
    // from p.x and p.y
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Creates new variables 'x' and 'y' with the values from p.x and p.y
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // Matching and destructuring Enums
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => println!("The Quit varianthas no data to destruct"),
        Message::Move { x, y } => {
            println!("Move to cordinates x: {} and y: {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("New color red {}, green {}, blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("New color hue {}, saturation {}, value {}", h, s, v)
        }
    }

    // Nesting, mixing and matching like crazy
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("Feet: {}, Inches: {}", feet, inches);
    println!("Point.x: {}, Point.y: {}", x, y);

    // Will not use the '3' due to the functions signature bar(_, y)
    bar(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            // Will match the pattern (Some(x), Some(y)) without using the
            // 'x' and 'y' variables the compiler will warn us of unused
            // variables.
            // if we use the '_' the compiler will not warn us about any
            // unused variables.
            println!("Can't overwrite an exsiting customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    // Avoid compiler warnings of 'Unused variable' by prefix it with '_'
    let _x = 5;
    let y = 10;
    println!("y: {}", y);

    let s = Some(String::from("Hello!"));

    // if we use Some(_s) we need to set s.clone()
    // if we use Some(_) it wil not be bound and we do not need to clone
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    let origin = Point { x: 0, y: 0 };

    match origin {
        // Ignore the y point
        Point { x, .. } => println!("x is {}", x),
    }

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn bar(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

// x is another pattern in a function like the one below.
//
//     |- PATTERN
//     v
fn foo(x: i32) {
    todo!()
}

// Using pattern to get a tuple into a function
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y)
}
