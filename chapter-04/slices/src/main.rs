fn main() {
    let message = String::from("Hello, World!");
    let word = first_word(&message[..]);

    println!("First word is: '{}'", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("a is: {:?}", a);
    println!("&a[1..3] is: {:?}", slice);
}

fn first_word(s: &str) -> &str {
    for (i, &character) in s.as_bytes().iter().enumerate() {
        if character == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
