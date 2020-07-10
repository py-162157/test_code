struct List<T> {
    len: usize,
    next: Option<Box<Node<T>>>,
}

#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Display> Node<T> {
    fn new(elem:T) -> Self {
        Node {
            value: elem,
            next: None,
        }
    }
    
    fn set_next(&mut self, elem: T) {
        self.next = Some(Box::new(Node::new(elem)));
    }

    fn get_last(&mut self) -> &mut Self {
        if let Some(ref mut x) = self.next {
            return x.get_last();
        }
        self
    }

    fn print_all_element(&mut self) {
        if let Some(ref mut x) = self.next {
            println!("{} ", x.value);
            x.print_all_element();
        } else {
            //println!("{} ", self.value);
        }
    }
}

impl<T: std::fmt::Display> List<T> {
    fn new() ->Self {
        List {
            len: 0,
            next: None,
        }
    }

    fn set_next(&mut self, elem: T) {
        self.next = Some(Box::new(Node::new(elem)));
    }

    fn get_next(&mut self) -> Option<&mut Node<T>> {
        if let Some(ref mut x) = self.next {
            Some(x)
        } else {
            None
        }
    }

    fn get_last(&mut self) -> Option<&mut Node<T>> {
        if let Some(ref mut x) = self.next {
            Some(x.get_last())
        } else {
            None
        }
    }

    fn rear_insert(&mut self, elem: T) {
        if self.len != 0 {
            if let Some(ref mut last_node) = self.get_last() {
                last_node.set_next(elem);
                self.len += 1;
            } else {
                println!("Error: Faild to get the last node when it's not empty!");
            }
        } else {
            self.set_next(elem);
            self.len += 1;
        }
    }

    /*fn insert_in_next(node: Option<Box<List>>, value: T) {
        if let Some(ref mut x) = node.next {
            let mut temp = None;
            let mut now_node = node;
            temp = 
        } else {

        }
    }*/

    fn print_all_element(&mut self) {
        if self.len == 0 {
            println!("The linklist is empty!");
        } else {
            if let Some(ref mut x) = self.next {
                println!("{} ", x.value);
                x.print_all_element();
            } else {
                println!("travel finished")
            }
        }
    }
}

fn main() {
     let mut L = List::new();
     //test
     L.rear_insert(1);
     L.rear_insert(2);
     L.rear_insert(3);
     L.print_all_element();
}
