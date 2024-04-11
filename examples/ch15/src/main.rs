fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let list = List::from(numbers);
    println!("{:?}", list);
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
}
