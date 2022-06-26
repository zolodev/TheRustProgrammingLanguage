fn main() {
    let s = String::from("hello world");

    // let len = s.len();

    // let slice = &s[0..2];
    // let slice = &s[..2]; // the same without the zero

    // let slice = &s[3..len];
    // let slice = &s[3..]; // the same without len

    println!("{} {}", &s[..5], &s[6..]); // -> hello world
}
