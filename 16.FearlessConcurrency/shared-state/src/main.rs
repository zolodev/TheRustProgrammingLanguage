/**************************************************************&***************
 * Filename      : main.rs
 * Created       : Tue Jul 26 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 16
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // Using Arc instead of Rc to make it atomic (thread safe)
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // using Arc instead of Rc
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            thread::sleep(Duration::from_secs(1));
        });
        handles.push(handle);
    }

    let mut count = 0;
    for handle in handles {
        handle.join().unwrap();
        count += 1;
        println!("joing thread {}", count);
    }

    println!("Result: {}", *counter.lock().unwrap());
}
