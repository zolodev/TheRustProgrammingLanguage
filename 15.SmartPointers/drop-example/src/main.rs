/*****************************************************************************
 * Filename      : main.rs
 * Created       : Mon Jul 25 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working throught the Rust book chapter 15
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer width data `{}`!", self.data)
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointer created.")
}
