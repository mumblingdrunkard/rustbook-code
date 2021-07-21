use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
struct Node<T>
where
    T: std::fmt::Display,
{
    value: T,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Drop for Node<T>
where
    T: std::fmt::Display,
{
    fn drop(&mut self) {
        println!("Hey I was dropped! {}", self.value);
    }
}

fn main() {
    let leaf = Rc::new(Node {
        value: 7,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent: {:#?}", leaf.parent.borrow().upgrade());
    println!("references to leaf {}", Rc::strong_count(&leaf));

    let branch = Rc::new(Node {
        value: 6,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("references to leaf {}", Rc::strong_count(&leaf));
    println!("leaf parent: {:#?}", leaf.parent.borrow().upgrade());
}
