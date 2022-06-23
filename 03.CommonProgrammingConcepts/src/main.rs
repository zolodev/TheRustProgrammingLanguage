use core::num;

fn main() {

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        
        index = index + 1;
    }

    // the value is: 10
    // the value is: 20
    // the value is: 30
    // the value is: 40
    // the value is: 50
}

