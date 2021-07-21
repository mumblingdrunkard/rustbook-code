use std::cell::RefCell;
use std::rc::Rc;

use cycle::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation {}", Rc::strong_count(&a));
    println!("b initial rc count {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    println!("b rc count after changing a = {}", Rc::strong_count(&b));

    // Uncomment to see we have a cycle, it will overflow the stack as
    // the list never ends
    // println!("a next item = {:?}", a.tail());
}
