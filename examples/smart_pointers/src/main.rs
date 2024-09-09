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

fn main() {
    boxes();
}
