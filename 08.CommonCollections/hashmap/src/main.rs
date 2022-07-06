use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue team"), 10);
    scores.insert(String::from("Red team"), 50);

    println!("{:?}", scores); // -> {"Blue team": 10, "Red team": 50}
}
