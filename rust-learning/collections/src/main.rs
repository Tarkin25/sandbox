fn main() {

    let words = "first apple";

    println!("\"{}\" in pig latin is \"{}\"", words, pig_latin(words));

}

fn pig_latin(s: &str) -> String {
    let mut result = String::new();

    for word in s.split_whitespace() {
        if let Some(first) = word.chars().nth(0) {
            if is_vowel(&first) {
                result.push_str(word);
                result.push_str("-hay");
            } else {
                result.push_str(&word[1..]);
                result.push('-');
                result.push(first);
                result.push_str("ay");
            }

            result.push(' ');
        }
    }

    result.truncate(result.len() - 1);

    result
}

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn is_vowel(c: &char) -> bool {
    VOWELS.contains(c)
}
