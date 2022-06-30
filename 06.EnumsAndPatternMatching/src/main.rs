fn main() {
    let config_max = Some(3u8);

    // match will only execute when config_max is Some()
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}
