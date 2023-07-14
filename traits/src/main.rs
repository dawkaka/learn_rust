use traits::notify;
use traits::{Summary, Tweet};
fn main() {
    let tweet = Tweet {
        username: String::from("dawkaka"),
        content: String::from("shit post"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet {}", tweet.summarize());
    println!("{}", notify(&tweet));
}
