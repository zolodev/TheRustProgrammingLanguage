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
}
