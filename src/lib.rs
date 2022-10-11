// singly put above the unused struct and so on
#![allow(dead_code)]
// singly put above the unused variable
#![allow(unused_variables)]
// allow unused_imports
#![allow(unused_imports)]
use std::fmt::Debug;

mod front_of_house;
mod back_of_house {
    use super::front_of_house::hosting;
    pub fn test() {
        super::front_of_house::hosting::add_to_waitlist();
    }
}

use std::{fmt::Display, iter::Sum};

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

pub trait Summary {
    fn summarize_author(&self) -> &String;

    fn summarize(&self) -> String {
        format!("Read more from {} ...", self.summarize_author())
    }
    // fn summarize1(&self) -> String;
    // fn summarize2(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> &String {
        &self.author
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{} : {}", self.username, self.content)
    // }
    fn summarize_author(&self) -> &String {
        &self.username
    }
}
// any type that implement the specific trait
// pub fn notify(item: impl Summary + Display) {
//     println!("Breaking new! {}", item.summarize());
// }
// trait bound
pub fn notify<T: Summary + Display>(item: T) {
    println!("Breaking new! {}", item.summarize());
}
// trait bound with where
pub fn notify1<T, U>(item: T, event: U)
where
    T: Summary + Display,
    U: Clone + Debug,
{
    println!("Breaking new! {}", item.summarize());
}
// impl trait can only return data of the same type
