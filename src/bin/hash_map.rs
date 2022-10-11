#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use std::{collections::HashMap, hash::Hash};

fn main() {
    // let mut scores: HashMap<String, i32> = HashMap::new();
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 16);
    println!("{:?}", scores);

    // second init method
    let teams = vec!["Blue", "Yellow"];
    let initial_scores = vec![10, 50];
    // Question: About the internal reference process
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
    // get the value
    let score = scores.get(&&"Yellow").expect("this key doesn't exist");
    // print the HashMap
    for (k, v) in &scores {
        println!("k is {}, v is {}.", k, v);
    }
    // update the HashMap
    // 1.replace
    // scores.insert();
    // 2.if k doesn't exist, insert. Else, return the value's mutable reference
    // scores.entry(&&"Red").or_insert(&99);
    // 3.
    let text = "hellow world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}
