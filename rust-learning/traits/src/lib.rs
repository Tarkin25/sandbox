pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

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

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait EnhancedCollection<T> {
    fn transform(&mut self, transformer: fn(&T) -> T);

    fn map(&self, mapper: fn(&T) -> T) -> Self;
}

impl<T> EnhancedCollection<T> for Vec<T> {
    fn transform(&mut self, transformer: fn(&T) -> T) {
        for element in self.iter_mut() {
            *element = transformer(element);
        }
    }

    fn map(&self, mapper: fn(&T) -> T) -> Self {
        let mut new = Vec::with_capacity(self.len());

        for element in self {
            new.push(mapper(element));
        }

        new
    }
}