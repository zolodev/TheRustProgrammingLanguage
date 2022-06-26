fn main() {
    // Values copied to stack (not heap)
    let x = 5; // assign x with the value of 5
    let y = x; // copy the value from x to y, now y also have a value of 5

    println!("x:{}, y:{}", x, y); // -> x:5, y:5

    // No let's look at the String version
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Clone() is similare to deep copy

    // No error s2 is now a copy of s1.
    println!("s1:{}, s2:{}", s1, s2); // -> s1:hello, s2:hello
}
