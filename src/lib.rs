// singly put above the unused struct and so on
#![allow(dead_code)]
// singly put above the unused variable
#![allow(unused_variables)]
// allow unused_imports
#![allow(unused_imports)]

mod front_of_house;
mod back_of_house {
    use super::front_of_house::hosting;
    pub fn test() {
        super::front_of_house::hosting::add_to_waitlist();
    }
}

// republicate the module
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // use import the namespace
    hosting::add_to_waitlist();
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();
}
