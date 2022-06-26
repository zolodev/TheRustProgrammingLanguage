fn main() {
    let reference_to_nothing = dangle();
}

// Error, expected named lifetime parameter
// dangle returns a reference to a String
fn dangle() -> &String {
    let s = String::from("hello"); // s is anew String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
