fn main() {
    let data = "initial contents";
    let s = data.to_string();

    // the method also works on a literal directly
    let s = "initial contents".to_string();
    let s = String::from("initial contents"); // Same as above
}
