fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    // println!("{}", s); // Error, value borrowed here after move

    let x = 5;
    makes_copy(x); // -> 5

    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string); // -> hello
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer); // -> 5
}
