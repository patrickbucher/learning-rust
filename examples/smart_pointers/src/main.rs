use std::fmt::Debug;
use std::mem::drop;
use std::ops::Deref;
use std::rc::Rc;

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

#[derive(Debug)]
enum SharedList {
    Cons(i32, Rc<SharedList>),
    Nil,
}

impl SharedList {
    fn output(&self) -> String {
        match self {
            SharedList::Cons(value, next) => format!("{value}, {}", next.output()),
            SharedList::Nil => String::from("Nil"),
        }
    }
}

fn reference_counter() {
    let tail = Box::new(List::Cons(2, Box::new(List::Cons(1, Box::new(List::Nil)))));
    println!("{}", tail.output());

    let head_increment = Box::new(List::Cons(3, tail));
    println!("{}", head_increment.output());

    // FIXME: value "tail" used here after move
    // let head_doubled = Box::new(List::Cons(4, tail));
    // println!("{}", head_doubled.output());

    let tail = Rc::new(SharedList::Cons(
        2,
        Rc::new(SharedList::Cons(1, Rc::new(SharedList::Nil))),
    ));
    println!("{} (count: {})", tail.output(), Rc::strong_count(&tail));

    {
        let head_increment = SharedList::Cons(3, Rc::clone(&tail));
        println!(
            "{}, (count: {})",
            head_increment.output(),
            Rc::strong_count(&tail)
        );
        {
            let head_doubled = SharedList::Cons(4, Rc::clone(&tail));
            println!(
                "{} (count: {})",
                head_doubled.output(),
                Rc::strong_count(&tail)
            );
        }
        println!("count: {}", Rc::strong_count(&tail));
    }
    println!("count: {}", Rc::strong_count(&tail));
}

fn main() {
    boxes();
    deref_drop();
    reference_counter();
}
