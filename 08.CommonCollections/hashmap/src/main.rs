/****************************************************************************
 * Filename     : main.rs
 * Created      : Wed Jul 06 2022
 * Author       : Zolo
 * Description  : Working through the Rust book chapter 08-03 HashMaps
*****************************************************************************/

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // Insert first value of 10
    scores.insert(String::from("Blue team"), 10);

    // Overwriting the value with a new value of 25
    scores.insert(String::from("Blue team"), 25);

    println!("{:?}", scores); // -> {"Blue team": 25}
}
