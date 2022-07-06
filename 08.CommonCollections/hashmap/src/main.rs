use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue team"), String::from("Red team")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", scores); // -> {"Red team": 50, "Blue team": 10}
}
