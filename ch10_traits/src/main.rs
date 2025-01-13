pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

// replace abover impl block with the following to demonstrate default summarize implementation if there are defaults setup...if any of the impl functions are not setup, then they have to be setup where implemented.
//impl Summary for NewsArticle {}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,   // is tweet a reply
    pub retweet: bool, // is tweet a retweet
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

/*
we want shared behavior (methods) between different things (structs in this case)

so we define a public trait, Summary.
we say that summarize is a shared method...

we could only specify the method signature if we don't want to dictate implementaiton. we just want to say "for every type that implements this trait, it should have this summarize method that will return a String"

OR

we couple provide a default implementation for the trait and any custom implementation will simply override it.
*/

pub trait Summary {
    fn summarize(&self) -> String {
        format!("read more from {}...", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

// anything that implments the Summary trait, can be passed to notify. essentially we are passing Traits as parameters.
pub fn notify(item: &impl Summary) {
    println!("breaking news!! {}", item.summarize());
}

// we can use trait bounds to get more advanced with how we pass traits as parameters.
// we can specify which traits exactly can be passed in as which parrameters...

pub fn notify_two<T: Summary, N: Summary>(item1: &T, item2: &N) {
    println!(
        "double notify: {}, {}",
        item1.summarize(),
        item2.summarize()
    );
}

// the above and below are the same

pub fn notify_two_too<T, N>(item1: &T, item2: &N)
where
    T: Summary,
    N: Summary,
{
    println!(
        "double notify with where: {}. {}",
        item1.summarize(),
        item2.summarize(),
    );
}

// we can return a value of some type that implements a trait...
// let's return a tweet since that implemenets Summary

fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("twitter username"),
        content: String::from("tweet is 128 chars of noise"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("@username"),
        content: String::from("hello tweet"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("me"),
        headline: String::from("the headline"),
        content: String::from("the big story"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    notify(&tweet);
    notify(&article);

    notify_two(&tweet, &article);
    notify_two_too(&tweet, &article);

    let returned_tweet = return_summarizable();

    println!(
        "{}",
        format!("returned tweet: {}", returned_tweet.summarize())
    );
}
