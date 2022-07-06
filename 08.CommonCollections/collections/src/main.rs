fn main() {
    let v = vec![100, 32, 57];

    // Iterate over a immutable vector
    for i in &v {
        println!("{}", i);
    }
}
