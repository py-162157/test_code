type LinkNode<V> = Option<Box<Node<V>>>;
#[derive(Debug)]
struct Node<V> {
    value: Option<V>,
    next: LinkNode<V>,
}

struct LinkList<V> {
    head: Box<Node<V>>,
    rear: Box<Node<V>>,
}

/*trait LinkList<V> {
    fn new() -> Node<V>;
    fn head_insert(&self, value: V);
    fn rear_insert(&self, value: V);
    fn print_all(&self);
    fn head_delete(&self);
    fn erae_delete(&self);
}*/

impl <V: std::fmt::Display> Node<V> {
    fn new(value: Option<V>) -> Self {
        Node {
            value: value,
            next: None,
        }
    }

}

impl <V: std::fmt::Display> LinkList<V> {
    fn new() -> Self {
        let mut L = LinkList {
            head: Box::new(Node::new(None)),
            rear: Box::new(Node::new(None)),
        };
        L.rear = L.head;
        return L;
    }
}

fn main() {
    println!("hello world!");
}
