use std::rc::Rc;

enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(
        1,
        Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Cons(4, Rc::new(Nil))))))),
    ));

    let b = Rc::new(Cons(5, Rc::clone(&a)));
    let c = Rc::new(Cons(6, Rc::clone(&a)));

    let mut it = &a;
    while let Cons(value, next) = it.as_ref() {
        println!("value: {}", value);
        it = next;
    }

    let mut it = &b;
    while let Cons(value, next) = it.as_ref() {
        println!("value: {}", value);
        it = next;
    }

    let mut it = &c;
    while let Cons(value, next) = it.as_ref() {
        println!("value: {}", value);
        it = next;
    }
}
