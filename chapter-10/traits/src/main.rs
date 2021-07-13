use std::fmt::Display;

pub trait Summary {
    fn author(&self) -> String;

    // Default implementation
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn author(&self) -> String {
        format!("{}", self.author)
    }
}

pub enum TweetType {
    Original,
    Retweet,
    Reply,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub post_type: TweetType,
}

impl Summary for Tweet {
    fn author(&self) -> String {
        format!("{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Works for any combination of types as long as both implement Summary
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: "username".to_string(),
        content: "content".to_string(),
        post_type: TweetType::Retweet,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

// Implemented for all types of pair
impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Implemented only for types that implement Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: "mumblingdrunkard".to_string(),
        content: "This is my first tweet".to_string(),
        post_type: TweetType::Original,
    };

    println!("{}", tweet.summarize());

    let article = NewsArticle {
        headline: "mumblingdrunkard starts twitter".to_string(),
        location: "Home".to_string(),
        author: "mumblingdrunkard".to_string(),
        content: "The online persona mumblingdrunkard has just started tweeting".to_string(),
    };

    println!("{}", article.summarize());

    println!("");
    notify(&tweet, &tweet);
    println!("");
    notify(&tweet, &article);
    println!("");
    notify(&article, &article);

    println!("");
    println!("{}", returns_summarizable().summarize());

    let pair = Pair::new(32, 44);
    pair.cmp_display();
}
