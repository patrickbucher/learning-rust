use double_linked_list::Node;

fn main() {
    let mut head = Node::new(7);
    head = head.prepend(5);
    head = head.prepend(3);
    head = head.prepend(1);
    println!("head = {}", head.value());
    println!("values = {:?}", head.values());
}
