fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // Error, second mutable borrow occurs here

    println!("{}, {}", r1, r2);
}
