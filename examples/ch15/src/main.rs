use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // cons list
    let numbers = vec![1, 2, 3, 4, 5];
    let list = List::from(numbers);
    println!("{:?}", list);
    println!("{:?}", list.get_values());

    // single linked list
    let mut head = SingleNode {
        value: 10,
        next: None,
    };
    head = prepend(head, 7);
    head = prepend(head, 4);
    head = prepend(head, 1);
    println!("values: {:?}", get_values(&head));

    // double linked list
    let a = DoubleNode::Some(Node {
        value: 3,
        prev: Rc::new(RefCell::new(DoubleNode::None)),
        next: Rc::new(RefCell::new(DoubleNode::None)),
    });
    let _b = DoubleNode::Some(Node {
        value: 5,
        prev: Rc::new(RefCell::new(DoubleNode::None)),
        next: Rc::new(RefCell::new(DoubleNode::None)),
    });
    let _c = DoubleNode::Some(Node {
        value: 8,
        prev: Rc::new(RefCell::new(DoubleNode::None)),
        next: Rc::new(RefCell::new(DoubleNode::None)),
    });
    if let DoubleNode::Some(a) = a {
        println!("{}", a.value);
    }
}

struct Node {
    value: i32,
    prev: Rc<RefCell<DoubleNode>>,
    next: Rc<RefCell<DoubleNode>>,
}

impl Node {
    fn prev(&self) -> Rc<RefCell<DoubleNode>> {
        self.prev.clone()
    }
    fn next(&self) -> Rc<RefCell<DoubleNode>> {
        self.next.clone()
    }
}

enum DoubleNode {
    Some(Node),
    None,
}

fn prepend(head: SingleNode, value: i32) -> SingleNode {
    let new_head = SingleNode {
        value,
        next: Some(Box::new(head)),
    };
    new_head
}

fn get_values(head: &SingleNode) -> Vec<i32> {
    let mut values: Vec<i32> = Vec::new();
    let mut current = head;
    while let Some(node_box) = &current.next {
        values.push(current.value);
        current = node_box;
    }
    values.push(current.value);
    values
}

struct SingleNode {
    value: i32,
    next: Option<Box<SingleNode>>,
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn from(mut vec: Vec<i32>) -> List {
        let mut head: List = List::Nil;
        vec.reverse();
        for v in vec.iter() {
            let node = List::Cons(*v, Box::new(head));
            head = node;
        }
        head
    }

    fn get_values(&self) -> Vec<i32> {
        let mut values: Vec<i32> = Vec::new();
        let mut node = self;
        while let List::Cons(v, b) = node {
            values.push(*v);
            node = b;
        }
        values
    }
}
