fn main() {
    let text = "Hello, World! You are wonderful!";

    piglatin_text(&text);

    println!("{}", piglatin_text("Piglatin is very ugly"));
}

fn piglatin_text(text: &str) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut result = String::new();

    for word in &words {
        result.push_str(&piglatin_word(word));
        result.push_str(" ");
    }

    result
}

fn piglatin_word(word: &str) -> String {
    let mut word = String::from(word);

    let vowels: Vec<char> = "aeiou".chars().collect();

    let letters: Vec<char> = word.chars().collect();
    if letters.len() < 3 {
        return String::from(word);
    }

    let uppercase = letters[0].is_uppercase();
    let is_vowel = vowels.contains(&letters[0]);

    match is_vowel {
        true => {
            word.push_str("-hay");
        },

        false => {
            let first_letter = letters[0];
            word.remove(0);
            word.push_str(&format!("-{}ay", first_letter.to_lowercase()));
        },
    }

    if let true = uppercase {
        let mut c = word.chars();
        word = match c.next() {
            Some(letter) => letter.to_uppercase().collect::<String>() + c.as_str(),
            None => String::new(),
        }
    }

    word
}
