pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub location: String,
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} - {}", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) -> String {
    format!("Breaking news! {}", item.summarize())
}

// doesn't work ?
// pub fn returns_summarizable(t: bool) -> impl Summary {
//     if t {
//         return NewsArticle {
//             location: String::from("Washington DC"),
//             headline: String::from("Trump impeached!"),
//             author: String::from("see"),
//             content: String::from("SEEe"),
//         };
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }
