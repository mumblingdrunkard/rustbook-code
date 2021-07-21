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

    println!("count after creating a: `{}`", Rc::strong_count(&a));

    #[allow(unused)]
    let b = Rc::new(Cons(5, Rc::clone(&a)));
    println!("count after creating b: `{}`", Rc::strong_count(&a));

    {
        #[allow(unused)]
        let c = Rc::new(Cons(6, Rc::clone(&a)));
        println!("count after creating c: `{}`", Rc::strong_count(&a));
    }

    println!(
        "count after c goes out of scope: `{}`",
        Rc::strong_count(&a)
    );
}
