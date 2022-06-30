fn main() {
    let config_max = Some(3u8);

    // match will only execute when config_max is Some()
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }

    // same code as above, with if-let and shorter
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
