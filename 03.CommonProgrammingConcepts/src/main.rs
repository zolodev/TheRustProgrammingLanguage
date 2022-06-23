fn main() {

    let x = plus_one(5);
    println!("The value of x is: {}", x);  // The value of x is: 6
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
