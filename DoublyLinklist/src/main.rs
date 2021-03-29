use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node<T> {
    value: Option<T>,
    pre: Option<RefCell<Weak<Node<T>>>>,
    next: Option<RefCell<Rc<Node<T>>>>,
}

impl<T: std::fmt::Display> Node<T> {
    fn new(key: T) -> Node<T> {
        Node {
            value: Some(key),
            pre: None,
            next: None,
        }
    }
}

struct DoublyLinklist<T> {
    head: Rc<Node<T>>,
    //tail: Rc<Node<T>>,
}

impl<T: std::fmt::Display + std::fmt::Debug> DoublyLinklist<T> {
    fn new() -> DoublyLinklist<T> {
        /*let mut tail = Rc::new( Node {
            value: None,
            pre: Some(RefCell::new(Weak::new())),
            next: None,
        });*/

        let mut head = Rc::new(Node {
            value: None,
            pre: None,
            next: None,
        });

        //*tail.pre.as_ref().unwrap().borrow_mut() = Rc::downgrade(&head);
        
        DoublyLinklist {
            head: head,
        }
    }

    fn head_insert(&mut self, key: T) {
        let NewNode = Node {
            value: Some(T),

            
        }
    }

    /*fn head_insert(&mut self, key: T) {

        let head_next = &*self.head.next.as_ref().unwrap().borrow();
        //println!("head = {:?}", head_next);
        let mut NewNode = Node {
            value: Some(key),
            pre: Some(RefCell::new(Weak::new())),
            //next: Some(RefCell::new(Rc::new(*self.head.next.unwrap().borrow())),
            next: Some(RefCell::new(Rc::clone(head_next))),
        };
        println!("newnode's next = {:?}", NewNode.next);
    }*/
}
fn main() {
    let mut List = DoublyLinklist::<i32>::new();
    //println!("head = {:?}", List.head);
    //println!("tail = {:?}", List.tail);
}

//value: Node { value: None, pre: Some(RefCell { value: (Weak) }), next: None }