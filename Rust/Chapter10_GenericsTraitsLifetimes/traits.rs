pub trait Summary {
    fn summarize(&self) -> String {
        //default behavior, which implementation overwrites
        String::from("(Read more ...)")
    }
}
// function which accepts any type which implements Summary
pub fn notify(item: &impl summary) {
    println!("breaking news! {}", item.summarize());
}
/*
 * same thing as above ^^^
pub fn notify<T: Summary>(item: &T) {
    println!("breaking news! {}", item.summarize());
*/
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//implement the summary interface(trait) for NewsArticle
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//same thing for the tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet1 = Tweet {
        username: String::from("dom"),
        content: String::from("Rust is cool!"),
        reply: true,
        retweet: false,
    };
    println!("{}", tweet1.summarize());
    notify(&tweet1);
}
