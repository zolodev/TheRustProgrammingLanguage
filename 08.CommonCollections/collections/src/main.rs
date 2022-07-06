/// ♥ To summarize, strings are complicated. Different programming
/// languages make different choices about how to present this
/// complexity to the programmer. Rust has chosen to make the
/// correct handling of String data the default behavior for all
/// Rust programs, which means programmers have to put more thought
/// into handling UTF-8 data upfront. This trade-off exposes more
/// of the complexity of strings than is apparent in other
/// programming languages, but it prevents you from having to
/// handle errors involving non-ASCII characters later in your
/// development life cycle. ♥
/// Source: https://doc.rust-lang.org/book/ch08-02-strings.html

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

    // adding a char to a string
    let mut s = String::from("lo");
    s.push('l');

    println!("{}", s); // -> lol

    // Concatinating + operator or format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note => s1 has been moved here and can no longer be used

    println!("{}", s3); // -> Hello, world!

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // One way to print tic-tac-toe
    println!("{}", s); // -> tic-tac-toe

    // Another way would be to use format! Macro
    let s1 = String::from("tic"); // Re-assigned after move

    // format! macro makes it easier to see what is going on
    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s); // -> tic-tac-toe

    // Not possible to index Strings or str
    // due to the support of UTF-8
    let s1 = String::from("hello");
    // let h = &s1[0]; // Error, `String` cannot be indexed by `{integer}`
    let h = &s1[..1]; // Slicing a String could be an alternative
    println!("{}", h);

    // Example string with special characters
    let hello = "नमस्ते"; // Hindi translates to Greetings!

    // Caution when slicing special chars.
    // special chars take up more bytes
    // let s = &hello[0..1]; // Error, thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'न' (bytes 0..3) of `नमस्ते`'
    let s = &hello[0..6];
    println!("{}", s); // -> नम
}
