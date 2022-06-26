fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is '{}'", s2, len); // -> The length of 'hello' is '5'
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
