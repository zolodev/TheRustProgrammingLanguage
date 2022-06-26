fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM // -> Error, mutable borrow occurs here

    println!("{}, {}, {}", r1, r2, r3);
}
