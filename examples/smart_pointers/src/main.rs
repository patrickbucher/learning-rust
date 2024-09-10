use std::cell::RefCell;
use std::fmt::Debug;
use std::mem::drop;
use std::ops::Deref;
use std::rc::{Rc, Weak};

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

fn interior_mutability() {
    let mut x = 13;
    let y = &mut x;
    println!("y={y}");
    *y = 42;
    println!("y={y}");

    let x = String::from("Joe");
    let y: RefCell<String> = RefCell::new(x.clone());
    let mut z = y.borrow_mut();

    // NOTE: this compiles, but panics at runtime
    // let mut q = y.borrow_mut();

    z.push_str("y");
    println!("x={x}, z={z}");
}

#[derive(Debug)]
enum MultiList {
    Cons(Rc<RefCell<i32>>, Rc<MultiList>),
    Nil,
}

impl MultiList {
    fn output(&self) -> String {
        match self {
            MultiList::Cons(value, next) => format!("{}, {}", value.borrow(), next.output()),
            MultiList::Nil => String::from("Nil"),
        }
    }
}

fn multiple_mutable_owners() {
    let last_value = Rc::new(RefCell::new(1));
    let tail = MultiList::Cons(
        Rc::new(RefCell::new(2)),
        Rc::new(MultiList::Cons(
            Rc::clone(&last_value),
            Rc::new(MultiList::Nil),
        )),
    );
    println!("{}", tail.output());

    let tail_rc = Rc::new(tail);

    let head_increment = MultiList::Cons(Rc::new(RefCell::new(3)), Rc::clone(&tail_rc));
    println!("{}", head_increment.output());

    let head_double = MultiList::Cons(Rc::new(RefCell::new(4)), Rc::clone(&tail_rc));
    println!("{}", head_double.output());

    // NOTE: sneaky edit
    *last_value.borrow_mut() *= 10;
    println!("{}", head_increment.output());
    println!("{}", head_double.output());
}

#[derive(Debug)]
enum CycleList {
    Cons(i32, RefCell<Rc<CycleList>>),
    Nil,
}

impl CycleList {
    fn tail(&self) -> Option<&RefCell<Rc<CycleList>>> {
        match self {
            CycleList::Cons(_, item) => Some(item),
            CycleList::Nil => None,
        }
    }
}

fn reference_cycles() {
    let a = Rc::new(CycleList::Cons(5, RefCell::new(Rc::new(CycleList::Nil))));
    let b = Rc::new(CycleList::Cons(10, RefCell::new(Rc::clone(&a))));
    println!(
        "strong count before block: a={}, b={}",
        Rc::strong_count(&a),
        Rc::strong_count(&b)
    );
    {
        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b); // NOTE: this creates a cycle
        }
        println!(
            "strong count in block: a={}, b={}",
            Rc::strong_count(&a),
            Rc::strong_count(&b)
        );
    }
    println!(
        "strong count after block: a={}, b={}",
        Rc::strong_count(&a),
        Rc::strong_count(&b)
    );
    // NOTE: this causes a stack overflow
    // println!("tail of a: {:?}", a.tail());
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn trees() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf: strong={}, weak={}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf: strong={}, weak={}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    println!(
        "branch: strong={}, weak={}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)
    );
}

fn main() {
    boxes();
    deref_drop();
    reference_counter();
    interior_mutability();
    multiple_mutable_owners();
    reference_cycles();
    trees();
}
