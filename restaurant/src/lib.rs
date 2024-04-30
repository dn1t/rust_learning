// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         // fn seat_at_table() {}
//     }

//     // mod serving {
//     //     fn take_order() {}
//     //     fn serve_order() {}
//     //     fn take_payment() {}
//     // }
// }

mod front_of_house;

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal._seasonal_fruit = String::from("blueberries");

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
}

fn _deliver_order() {}

mod back_of_house {
    fn _fix_incorrect_error() {
        _cook_order();
        super::_deliver_order();
    }
    fn _cook_order() {}
    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

mod customer {
    pub fn _eat_at_restaurant() {
        // hosting::add_to_waitlist();
    }
}

// use std::fmt;
// use std::io;

// fn _fn1() -> fmt::Result {}
// fn _fn2() -> io::Result<()> {}

// use std::fmt::Result;
// use std::io::Result as IoResult;

// pub use crate::front_of_house::hosting;

// use std::cmp::Ordering;
// use std::io;

// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;

// use std::io::{self, Write};

// use std::collections::*;
