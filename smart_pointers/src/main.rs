use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;
fn main() {
    let b = Box::new(5);
    println!("{}", b);
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // let m = MyBox::new(String::from("Rust"));
    // hello(&m);
    let c = CustomeSmartPointer {
        data: "First".to_string(),
    };
    let d = CustomeSmartPointer {
        data: "Nice".to_string(),
    };
    drop(c);
    println!("Smart pointers created");
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

enum List {
    Cons(i32, Rc<List>),
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

struct CustomeSmartPointer {
    data: String,
}

impl Drop for CustomeSmartPointer {
    fn drop(&mut self) {
        println!("Smart pointer dropped: {}", self.data);
    }
}
