fn main() {
    let v = vec![1, 2, 3, 4];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100];   // Error, thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 100', src\main.rs:12:27
    let does_not_exist = v.get(100);

    println!("does it exist? {:?}", does_not_exist); // -> does it exist? None

    // This would be the better alternative to raw unwrap
    if let Some(item) = does_not_exist {
        println!("does it exist? {}", item); // Will never return while index is out of bounds
    }

    // Unwrapping can be dangerous, use with caution
    // println!("does it exist? {}", does_not_exist.unwrap()); // Error, thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src\main.rs:22:50
}
