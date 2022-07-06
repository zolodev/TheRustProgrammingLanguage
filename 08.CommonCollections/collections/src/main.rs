fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");

    println!("{}", s); // -> foobar

    let mut s1 = String::from("foo");
    let s2 = "bar";

    // push_str can also take a variable like
    // a str literal and append it to the String
    s1.push_str(s2);

    println!("{}", s1); // -> foobar
}
