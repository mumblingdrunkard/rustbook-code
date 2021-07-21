use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(x, 5);
    assert_eq!(*y, x);

    let name = MyBox("mumblingdrunkard".to_string());
    hello(&name); // deref coercion

    // alternatively the explicit version
    hello(&(*name)[..]);
    // derefs the name to get a `String` which is then
    // converted to a slice with `&(String)[..]`
}
