fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    match x.len() > y.len() {
        true => x,
        false => y,
    }
}

// No lifetimes needed as lifetimes for inputs are valid
// for the entire function body and there are no output lifetimes
fn longest_cmp(x: &str, y: &str) -> bool {
    x.len() > y.len()
}

fn longest_with_announcement<'a, T: std::fmt::Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("Announcement! {}", ann);
    longest(x, y)
}

fn main() {
    let r; // declare outside
    {
        let x = 5;
        r = &x; // initialize inside
        println!("r: {}", r);
    }

    // r lives longer than x, but it's fine because we don't use r

    // The below line will fail to compile if uncommented
    // println!("r: {}", r);

    let s1 = "abcd".to_string();
    let s2 = "xyz";

    let result = longest(&s1, s2);
    println!("The longest string is: '{}'", result);

    println!(
        "The longest string is: '{}'",
        match longest_cmp(&s1, s2) {
            true => &s1,
            false => s2,
        }
    );

    // This is ok because literals have static lifetimes
    let result = longest("testtest123", "testtest1234");
    println!("The longest string is: '{}'", result);

    // This is not ok as the lifetime of the returned value
    // will be the shortest of the input lifetimes
    // let result = longest(&"abcd".to_string(), "testtest1234");

    println!(
        "Longest with announcement: {}",
        longest_with_announcement(&s1, s2, "Hello, World!")
    );
}
