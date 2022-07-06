fn main() {
    let mut v = vec![100, 32, 57];

    // updating values in vector
    for i in &mut v {
        *i += 50
    }

    // Iterate over a mutable vector after it has changed
    for i in &v {
        println!("{}", i);
    }
}
