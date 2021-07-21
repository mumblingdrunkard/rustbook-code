use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List<T> {
    Cons(RefCell<T>, Rc<List<T>>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(RefCell::new(5), Rc::new(Nil)));
    let b = Rc::new(Cons(RefCell::new(6), Rc::clone(&a)));
    let c = Rc::new(Cons(RefCell::new(7), Rc::clone(&a)));

    if let Cons(value, _next) = &*a {
        *value.borrow_mut() += 10;
    }

    // println!("References to `value`: {}", Rc::strong_count(&value));
    println!("References to `a`: {}", Rc::strong_count(&a));

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
