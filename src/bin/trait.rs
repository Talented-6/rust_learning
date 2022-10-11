// trait bounds

// You can't implement a trait defined outside this scope for a type defined outside this scope
use rust_learning::Summary;
use rust_learning::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probally already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}
