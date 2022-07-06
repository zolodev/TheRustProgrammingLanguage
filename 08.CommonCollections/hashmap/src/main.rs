use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue team"), 10);
    scores.insert(String::from("Red team"), 50);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    println!("{:?}", map); // -> {"Favorite color": "Blue"}
    println!("{:?}", map[&field_name]); // -> "Blue"

    let team_name = String::from("Blue team");
    let score = scores.get(&team_name);
    println!("{}", score.unwrap()); // -> 10

    for (key, value) in &scores {
        println!("{}: {}", key, value);
        // Output:
        // -> Blue team: 10
        // -> Red team: 50
    }
}
