use double_linked_list::Node;

fn main() {
    let mut head = Node::new(7);
    head = head.prepend(5);
    head = head.prepend(3);
    println!("{}", head.value());
}
