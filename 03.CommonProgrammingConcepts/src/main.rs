fn main() {
    let x = 5;
    
    // Shadowing variable x
    let x = x + 1;
    
    let x = x * 2;

    println!("The value of x is: {}", x);
}
