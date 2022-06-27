struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    ); // -> The area of the rectangle is 1500 square pixels.
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
