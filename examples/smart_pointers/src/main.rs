use std::fmt::Debug;
use std::mem::drop;
use std::ops::Deref;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn output(&self) -> String {
        match self {
            List::Cons(value, next) => format!("{value}, {}", next.output()),
            List::Nil => String::from("Nil"),
        }
    }
}

fn boxes() {
    let numbers = List::Cons(
        1,
        Box::new(List::Cons(
            2,
            Box::new(List::Cons(3, Box::new(List::Cons(4, Box::new(List::Nil))))),
        )),
    );
    println!("{}", numbers.output());
}

struct SmartPointer<T: Debug> {
    value: T,
}

impl<T: Debug> SmartPointer<T> {
    pub fn new(value: T) -> Self {
        SmartPointer { value }
    }
}

impl<T: Debug> Deref for SmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Debug> Drop for SmartPointer<T> {
    fn drop(&mut self) {
        eprintln!("{:?} just died", self.value);
    }
}

fn deref_drop() {
    let p = SmartPointer::new(13);
    {
        let q = SmartPointer::new(42);
        let r = SmartPointer::new(99);
        drop(r);
        println!("q={}", *q);
    }
    println!("p={}", *p);
}

fn main() {
    boxes();
    deref_drop();
}
