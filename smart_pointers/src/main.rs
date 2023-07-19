use crate::List::{Cons, Nil};
use std::ops::Deref;
fn main() {
    let b = Box::new(5);
    println!("{}", b);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn hello(name: &str) {
    println!("Hello {name}");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(val: T) -> MyBox<T> {
        MyBox(val)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
