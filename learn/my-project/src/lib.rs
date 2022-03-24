mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {
            add_to_waitlist();
            super::serving::serve_order();
        }
    }

    mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    
    // front_of_house::hosting::seat_at_table(); // Error(private function)
}


// Rexport module by using pub use
pub use std::collections::HashMap;
//pub use crate::front_of_house::hosting;

// Bring hosting into scope
use crate::front_of_house::hosting;

pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
}

// Bring add_to_waitlist to scope
use crate::front_of_house::hosting::add_to_waitlist;
pub fn  eat_at_restaurant3() {
    add_to_waitlist();
}

// Importing multiple submodules
use std::{collections, cmp::Ordering};
use std::io::{self, Write};
use std::cmp::*; // Glob operator to bring all public submodules and members
