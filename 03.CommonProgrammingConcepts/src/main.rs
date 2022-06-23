fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    // x = 6; // <-- Error, cannot assign twice to immutable variable, x need mut
    println!("The value of x is: {}", x)
}
