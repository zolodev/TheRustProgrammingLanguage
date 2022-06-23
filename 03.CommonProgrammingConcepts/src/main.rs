use core::num;

fn main() {

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    for element in a.iter() {
        println!("the value is: {}", element)
    }

    // the value is: 10
    // the value is: 20
    // the value is: 30
    // the value is: 40
    // the value is: 50
}

