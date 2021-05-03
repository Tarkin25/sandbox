struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {

    let a = String::from("abcd");
    let b = "xyz";

    let result = longest(a.as_str(), b);

    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let excerpt = ImportantExcerpt {
        part: first_sentence
    };

    println!("first sentence: {}", first_sentence);

}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
