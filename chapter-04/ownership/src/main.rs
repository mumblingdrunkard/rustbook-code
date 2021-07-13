fn main() {
    let s1 = String::from("Hello");

    takes_ownership(s1);

    let x = 5;

    makes_copy(x);

    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = String::from("Hello!");

    let s3 = takes_and_gives_back(s2);

    println!("{}", s3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    String::from("Hello!")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
