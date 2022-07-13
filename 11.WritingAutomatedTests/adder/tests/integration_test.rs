use adder;

// Include the module common for shared test code
mod common;

#[test]
fn it_add_two() {
    // Calling shared test code
    common::setup();

    assert_eq!(4, adder::add_two(2))
}
