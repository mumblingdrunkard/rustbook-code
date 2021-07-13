fn main() {
    let mut s1 = String::from("Hello");
    let len = change(&mut s1);
    println!("The length of '{}' is {}", s1, len);

    let s2 = returns_string();
    println!("{}", s2);
}

fn change(s: &mut String) -> usize {
    s.push_str(", world!");
    s.len()
}

fn returns_string() -> String {
    String::from("Hello")
}
