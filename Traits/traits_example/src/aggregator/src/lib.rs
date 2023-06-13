pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
    
}
pub trait Operations {
    fn counter(&self) -> u32 {
        return 42;
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle{}
impl Operations for NewsArticle{}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }
}