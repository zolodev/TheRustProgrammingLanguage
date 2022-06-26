fn main() {
    let words = String::from("Hello world!");

    let index = first_word(&words);

    println!("{}", index); // -> 5
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
