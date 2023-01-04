use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    let b = Rc::new(Cons(4, Rc::clone(&a)));
    let c = Rc::new(Cons(5, Rc::clone(&a)));

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}

use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
