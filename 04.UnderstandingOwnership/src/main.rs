fn main() {
    let reference_to_nothing = dangle();
}

// Error, expected named lifetime parameter
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
