// A trait is a collection of functions and properties that can be used 
// to implement a specific behavior.
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")}
}

pub struct NewsArticle {
    pub headline: String,
    pub content: String,
    pub location: String,
    pub author: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Traits as parameters
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize()); 
    println!("Breaking news! {}", item2.summarize()); 
}

// implementation by default
impl Summary for NewsArticle {}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as I predicted"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship"),
        content: String::from("The Stanley Cup Final was won by the Penguins, who beat the Maple Leafs 4-2"),
        location: String::from("Pittsburgh, Pennsylvania"),
        author: String::from("Tom Smith"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", article.summarize());
    println!("Retweet staus: {}", tweet.retweet);

    notify(&article, &article);
    notify(&tweet, &tweet);
}


