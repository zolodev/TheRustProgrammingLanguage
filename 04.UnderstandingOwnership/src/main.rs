use std::io::Bytes;

fn main() {
    let s = "Hello world!";

    let word = first_word(&s);

    println!("{}", word); // -> Hello
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
