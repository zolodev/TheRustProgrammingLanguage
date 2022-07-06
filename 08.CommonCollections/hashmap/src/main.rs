/****************************************************************************
 * Filename     : main.rs
 * Created      : Wed Jul 06 2022
 * Author       : Zolo
 * Description  : Working through the Rust book chapter 08-03 HashMaps
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // Key Yellow will be created with the value of 50
    scores.entry(String::from("Yellow")).or_insert(50);

    // Value will not be replaced due to key already exists
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores); // -> {"Yellow": 50, "Blue": 10}
}
