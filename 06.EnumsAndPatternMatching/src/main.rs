#[derive(Debug)]
enum UsState {
    Alabama,
    // --snip--
}
enum Coin {
    Quarter(UsState),
}
fn main() {
    let coin = Coin::Quarter(UsState::Alabama);

    let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}", state), // -> State quarter from Alabama
    //     _ => count += 1,
    // }

    // Same as the match, but with if-let
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state); // -> State quarter from Alabama
    } else {
        count += 1;
    }

    println!("Counted: {}", count); // -> Counted: 0
}
