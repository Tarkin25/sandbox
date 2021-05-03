use traits::{NewsArticle, Tweet, Summary, EnhancedCollection};

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    let tweet = Tweet {
        username: String::from("Tarkin25"),
        content: String::from("hello there fellow birds"),
        reply: false,
        retweet: false
    };

    println!("News article: {}", article.summarize());

    println!("Tweet: {}", tweet.summarize());

    let numbers = vec![1,2,3,4,5];
    println!("{:?} doubled is {:?}", numbers, numbers.map(|i| *i * 2));
}
