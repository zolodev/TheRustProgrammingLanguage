fn main() {

    // variable declaration
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);  // The value of y is: 4
}