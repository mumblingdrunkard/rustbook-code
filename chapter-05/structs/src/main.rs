#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let user1 = build_user("someuser@example.com".to_string(), "someuser123".to_string());

    let user2 = User {
        username: String::from("somenewuser123"),
        email: String::from("somenewuser@example.com"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    println!("user2: {:?}\n", user2);

    let user3 = User {
        username: String::from("somethirduser"),
        email: String::from("somethirduser@example.com"),
        ..user1 // Struct update syntax
    };

    println!("user3: {:?}\n", user3);

    let black = Color(0, 0, 0);
    println!("Black is: {:?}\n", black);

    let origin = Point(0, 0, 0);
    println!("Origin is: {:?}\n", origin);
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
