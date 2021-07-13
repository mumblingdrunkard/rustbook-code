fn main() {
    // utf-8 string
    let message = String::from("こんにちは");
    println!("Message: {}", message);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is: '{}'", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);

    println!("s2 is '{}'", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // s1 has been moved, s3 now has ownership of it

    // The below line will fail to compile if uncommented
    // println!("s1 is '{}'", s1);
    println!("s2 is '{}'", s2);
    println!("s3 is '{}'", s3);

    // Indexing into strings
    let _s = String::from("hello");
    // Error, type `String` cannot be indexed by `{integer}`
    // let h = s[0];

    // Strings in rust are utf-8 and code points vary in length
    let len = String::from("Здравствуйте").len();
    println!("len: {}", len); // 24

    let message = "Здравствуйте";
    for c in message.chars() {
        println!("{}", c);
    }

    for b in message.bytes() {
        println!("{}", b);
    }
}
