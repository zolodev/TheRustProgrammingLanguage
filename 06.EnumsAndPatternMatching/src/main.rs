enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn main() {
    println!("How many cents? {}", value_in_cents(Coin::Penny)); // -> How many cents? 1
    println!("How many cents? {}", value_in_cents(Coin::Nickel)); // -> How many cents? 5
    println!("How many cents? {}", value_in_cents(Coin::Dime)); // -> How many cents? 10
    println!("How many cents? {}", value_in_cents(Coin::Quarter)); // -> How many cents? 25
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!"); // -> Lucky penny!
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
