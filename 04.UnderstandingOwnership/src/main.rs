use std::io::Bytes;

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // Error, mutable borrow occurs here

    println!("{}", word); // -> hello
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
