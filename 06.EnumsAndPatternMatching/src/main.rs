fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y; // Error, no implementation for `i8 + Option<i8>`
}
