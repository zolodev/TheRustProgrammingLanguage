mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}

// Create a shortcut to hosting
use crate::front_of_house::hosting;

// Adding an unidomatic path
use crate::front_of_house::hosting::add_to_waitlist;
mod customer {

    pub fn eat_at_resturant() {
        // All enums options are available when public
        let order1 = super::back_of_house::Appetizer::Soup;
        let order2 = super::back_of_house::Appetizer::Salad;

        // Order a breakfast in the summer with Rye toast
        let mut meal = super::back_of_house::Breakfast::summer("Rye");

        // Change our mind about twhat bread we'd like
        meal.toast = String::from("Wheat");

        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");

        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        // Relative path
        super::front_of_house::hosting::add_to_waitlist();

        // Shortcut variant
        super::hosting::add_to_waitlist();

        // unidiomatic path
        super::add_to_waitlist();
    }
}

// Sometimes the same names exists
// This can cause confusions
// Below is an example where we use fmt::Result and io::Result<()>
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}