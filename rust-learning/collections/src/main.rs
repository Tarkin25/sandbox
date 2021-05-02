use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();

    // Insert value, potentially overriding existing an existing value for a given key
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Only insert a value for a given key if none exists yet
    scores.entry(String::from("Blue")).or_insert(50);

    // Word counter using or_insert
    let text = "hello world wonderful world";

    let mut word_counts = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }

    println!("word counts:\n{:?}", word_counts);
}
