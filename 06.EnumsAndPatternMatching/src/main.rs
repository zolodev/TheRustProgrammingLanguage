fn main() {
    // Not a Random value,
    // because the focus of the example is about the match
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // example of how to ignore all other values
        _ => (), // '_' using a tuple type, will ignore all other values except 3 & 7 and do nothing
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
