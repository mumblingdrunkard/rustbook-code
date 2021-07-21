use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum List<T> {
    Cons(T, RefCell<Rc<List<T>>>),
    Nil,
}

use crate::List::{Cons, Nil};

impl<T> List<T> {
    pub fn tail(&self) -> Option<&RefCell<Rc<List<T>>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[cfg(test)]
mod tests {}
