#![allow(unused)]
use std::fmt::{Debug, Display};

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
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

// * Traits裡面不用實作功能
// ! 重點就是, 如果有traits有default implementation的話, impl裡面就可不需要實作
pub trait Summary {
    // * 這個是不實作的
    fn summarize(&self) -> String;

    // * 但也可以實作, 當作default
    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn some_function<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}

fn main() {
    let news_article = NewsArticle {
        author: String::from("Jason Wang"),
        headline: String::from("FIFA 2022"),
        content: String::from("HHHH"),
    };

    let tweet = Tweet {
        username: String::from("Jason Wang"),
        content: String::from("IDK"),
        reply: true,
        retweet: false,
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", news_article.summarize());

    println!(
        "Article default summary: {}",
        news_article.summarize_default()
    );

    notify(&news_article);
}
