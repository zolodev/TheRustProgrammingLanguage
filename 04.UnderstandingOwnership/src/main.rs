fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world"); // Error, `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}
